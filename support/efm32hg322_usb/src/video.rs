//! USB Video Class 1.0 driver (virtual camera / pattern generator).
//!
//! Streams uncompressed YUY2 video to the host via an isochronous IN endpoint.
//! The application provides pixel data through the [`VideoHandler`] trait.

use crate::{usb_string, EpConfig, EpType, SetupPacket, SetupResult, UsbBus, UsbClass, UsbConfig};
use portable_atomic::{AtomicBool, Ordering};

/// Set once the host completes SET_CONFIGURATION.
pub static CONFIGURED: AtomicBool = AtomicBool::new(false);

/// Frame width in pixels.
pub const WIDTH: usize = 160;
/// Frame height in pixels.
pub const HEIGHT: usize = 120;
/// Bytes per scanline (YUY2 = 2 bytes/pixel).
pub const BYTES_PER_LINE: usize = WIDTH * 2;
/// Total bytes per video frame.
pub const FRAME_SIZE: usize = WIDTH * HEIGHT * 2;
/// Target frame rate.
pub const FRAME_RATE: u32 = 5;
/// Milliseconds between frames.
const FRAME_INTERVAL: u16 = (1000 / FRAME_RATE) as u16;
/// Max isochronous packet size.
const MAX_PACKET: usize = 512;
/// UVC payload header size.
const HEADER_LEN: usize = 2;
/// Max video data bytes per packet.
const MAX_PAYLOAD: usize = MAX_PACKET - HEADER_LEN;

// ---------------------------------------------------------------------------
// Descriptors (USB Video Class 1.0, uncompressed YUY2)
// ---------------------------------------------------------------------------

#[rustfmt::skip]
static DEVICE_DESC: [u8; 18] = [
    18,         // bLength
    0x01,       // bDescriptorType (Device)
    0x00, 0x02, // bcdUSB 2.00
    0xEF,       // bDeviceClass (Miscellaneous)
    0x02,       // bDeviceSubClass (Common Class)
    0x01,       // bDeviceProtocol (IAD)
    64,         // bMaxPacketSize0
    0xFE, 0xCA, // idVendor  0xCAFE (test VID)
    0x06, 0x00, // idProduct 0x0006
    0x00, 0x01, // bcdDevice 1.00
    1,          // iManufacturer
    2,          // iProduct
    0,          // iSerialNumber
    1,          // bNumConfigurations
];

// VC class-specific total: 13 + 17 + 9 = 39
const VC_TOTAL: u16 = 39;
// VS class-specific total: 14 + 27 + 30 + 6 = 77
const VS_TOTAL: u16 = 77;
// Config total: 9+8+9+13+17+9+9+9+14+27+30+6+7 = 167
const CONFIG_TOTAL_LEN: u16 = 167;
const _: () = assert!(CONFIG_TOTAL_LEN == 167);

// Frame timing: 200 ms = 2,000,000 * 100 ns units = 0x001E8480
// Bit rate: 160*120*16*5 = 1,536,000 = 0x00177000
// Frame buffer: 160*120*2 = 38,400 = 0x00009600

#[rustfmt::skip]
static CONFIG_DESC: [u8; CONFIG_TOTAL_LEN as usize] = [
    // ---- Configuration Descriptor (9) ----
    9, 0x02,
    (CONFIG_TOTAL_LEN & 0xFF) as u8, (CONFIG_TOTAL_LEN >> 8) as u8,
    2,              // bNumInterfaces
    1,              // bConfigurationValue
    0,              // iConfiguration
    0x80,           // bmAttributes (bus-powered)
    50,             // bMaxPower (100 mA)

    // ---- Interface Association Descriptor (8) ----
    8, 0x0B,
    0,              // bFirstInterface
    2,              // bInterfaceCount
    0x0E,           // bFunctionClass (Video)
    0x03,           // bFunctionSubClass (Video Interface Collection)
    0x00,           // bFunctionProtocol
    0,              // iFunction

    // ---- Interface 0: Video Control (9) ----
    9, 0x04,
    0, 0, 0,        // bInterfaceNumber, bAlternateSetting, bNumEndpoints
    0x0E, 0x01, 0x00, // Video, Video Control
    0,              // iInterface

    // VC Header (13)
    13, 0x24, 0x01, // CS_INTERFACE, VC_HEADER
    0x00, 0x01,     // bcdUVC 1.00
    (VC_TOTAL & 0xFF) as u8, (VC_TOTAL >> 8) as u8,
    0x00, 0x6C, 0xDC, 0x02, // dwClockFrequency = 48 MHz (LE)
    1,              // bInCollection
    1,              // baInterfaceNr(1) = VS interface

    // Camera Terminal (17)
    17, 0x24, 0x02, // CS_INTERFACE, INPUT_TERMINAL
    1,              // bTerminalID
    0x01, 0x02,     // wTerminalType = ITT_CAMERA (0x0201)
    0,              // bAssocTerminal
    0,              // iTerminal
    0x00, 0x00,     // wObjectiveFocalLengthMin
    0x00, 0x00,     // wObjectiveFocalLengthMax
    0x00, 0x00,     // wOcularFocalLength
    2,              // bControlSize
    0x00, 0x00,     // bmControls (none)

    // Output Terminal (9)
    9, 0x24, 0x03,  // CS_INTERFACE, OUTPUT_TERMINAL
    2,              // bTerminalID
    0x01, 0x01,     // wTerminalType = TT_STREAMING (0x0101)
    0,              // bAssocTerminal
    1,              // bSourceID
    0,              // iTerminal

    // ---- Interface 1, Alt 0: Video Streaming (zero-bandwidth) (9) ----
    9, 0x04,
    1, 0, 0,        // bInterfaceNumber, bAlternateSetting, bNumEndpoints
    0x0E, 0x02, 0x00, // Video, Video Streaming
    0,

    // VS Input Header (14) — class-specific VS descriptors go after Alt 0
    14, 0x24, 0x01, // CS_INTERFACE, VS_INPUT_HEADER
    1,              // bNumFormats
    (VS_TOTAL & 0xFF) as u8, (VS_TOTAL >> 8) as u8,
    0x81,           // bEndpointAddress
    0x00,           // bmInfo
    2,              // bTerminalLink (OT)
    0,              // bStillCaptureMethod
    0,              // bTriggerSupport
    0,              // bTriggerUsage
    1,              // bControlSize
    0x00,           // bmaControls

    // VS Uncompressed Format (27)
    27, 0x24, 0x04, // CS_INTERFACE, VS_FORMAT_UNCOMPRESSED
    1,              // bFormatIndex
    1,              // bNumFrameDescriptors
    // guidFormat = YUY2 {32595559-0000-0010-8000-00AA00389B71}
    0x59, 0x55, 0x59, 0x32,
    0x00, 0x00,
    0x10, 0x00,
    0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71,
    16,             // bBitsPerPixel
    1,              // bDefaultFrameIndex
    0, 0,           // bAspectRatioX, Y
    0x00,           // bmInterlaceFlags
    0,              // bCopyProtect

    // VS Uncompressed Frame (30)
    30, 0x24, 0x05, // CS_INTERFACE, VS_FRAME_UNCOMPRESSED
    1,              // bFrameIndex
    0x00,           // bmCapabilities
    (WIDTH as u16 & 0xFF) as u8, (WIDTH as u16 >> 8) as u8,
    (HEIGHT as u16 & 0xFF) as u8, (HEIGHT as u16 >> 8) as u8,
    0x00, 0x70, 0x17, 0x00, // dwMinBitRate  = 1,536,000
    0x00, 0x70, 0x17, 0x00, // dwMaxBitRate  = 1,536,000
    0x00, 0x96, 0x00, 0x00, // dwMaxVideoFrameBufferSize = 38,400
    0x80, 0x84, 0x1E, 0x00, // dwDefaultFrameInterval = 2,000,000 (200 ms)
    1,              // bFrameIntervalType (1 discrete)
    0x80, 0x84, 0x1E, 0x00, // dwFrameInterval = 2,000,000

    // VS Color Matching (6)
    6, 0x24, 0x0D,
    1,              // bColorPrimaries (BT.709)
    1,              // bTransferCharacteristics (BT.709)
    4,              // bMatrixCoefficients (SMPTE 170M)

    // ---- Interface 1, Alt 1: Video Streaming (active) (9) ----
    9, 0x04,
    1, 1, 1,        // bInterfaceNumber, bAlternateSetting, bNumEndpoints
    0x0E, 0x02, 0x00, // Video, Video Streaming
    0,

    // ---- Endpoint 1 IN - Isochronous (7) ----
    7, 0x05,
    0x81,           // bEndpointAddress (EP1 IN)
    0x05,           // bmAttributes (isochronous, asynchronous)
    (MAX_PACKET & 0xFF) as u8, (MAX_PACKET >> 8) as u8,
    1,              // bInterval (every frame)
];

static STRING1: [u8; 2 + 10 * 2] = usb_string!("SLSTK3400A");
static STRING2: [u8; 2 + 15 * 2] = usb_string!("EFM32 USB Video");

// ---------------------------------------------------------------------------
// VS Probe/Commit control (UVC negotiation)
// ---------------------------------------------------------------------------

/// 26-byte Video Probe/Commit Control data (UVC 1.0).
#[rustfmt::skip]
static PROBE_COMMIT: [u8; 26] = [
    0x00, 0x00,     // bmHint
    0x01,           // bFormatIndex
    0x01,           // bFrameIndex
    0x80, 0x84, 0x1E, 0x00, // dwFrameInterval = 2,000,000
    0x00, 0x00,     // wKeyFrameRate
    0x00, 0x00,     // wPFrameRate
    0x00, 0x00,     // wCompQuality
    0x00, 0x00,     // wCompWindowSize
    0x00, 0x00,     // wDelay
    0x00, 0x96, 0x00, 0x00, // dwMaxVideoFrameSize = 38,400
    0x00, 0x02, 0x00, 0x00, // dwMaxPayloadTransferSize = 512
];

// ---------------------------------------------------------------------------
// Handler trait
// ---------------------------------------------------------------------------

/// Application-provided video frame renderer.
pub trait VideoHandler {
    /// Fill `buf` with YUY2 pixel data starting at byte `offset` within the frame.
    /// Returns the number of bytes written (up to `buf.len()`).
    /// Called from ISR context, multiple times per video frame.
    fn render(&mut self, buf: &mut [u8], offset: usize) -> usize;

    /// Called once per video frame to advance animation state.
    fn advance_frame(&mut self);
}

// ---------------------------------------------------------------------------
// Class driver
// ---------------------------------------------------------------------------

pub struct VideoClass<H: VideoHandler> {
    handler: H,
    streaming: bool,
    alt_setting: u8,
    frame_id: bool,
    frame_timer: u16,
    sending: bool,
    byte_offset: usize,
}

impl<H: VideoHandler> VideoClass<H> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            streaming: false,
            alt_setting: 0,
            frame_id: false,
            frame_timer: FRAME_INTERVAL, // trigger immediately
            sending: false,
            byte_offset: 0,
        }
    }

    fn send_packet(&mut self, usb: &UsbBus) {
        let mut buf = [0u8; MAX_PACKET];

        if !self.sending {
            if self.frame_timer >= FRAME_INTERVAL {
                self.frame_timer = 0;
                self.sending = true;
                self.byte_offset = 0;
                self.handler.advance_frame();
            } else {
                // Idle packet: header only.
                buf[0] = HEADER_LEN as u8;
                buf[1] = if self.frame_id { 1 } else { 0 };
                usb.ep_write(1, &buf[..HEADER_LEN]);
                return;
            }
        }

        let remaining = FRAME_SIZE - self.byte_offset;
        let data_len = MAX_PAYLOAD.min(remaining);
        let is_last = self.byte_offset + data_len >= FRAME_SIZE;

        // UVC payload header.
        buf[0] = HEADER_LEN as u8;
        buf[1] = if self.frame_id { 1 } else { 0 } | if is_last { 2 } else { 0 };

        let rendered = self.handler.render(
            &mut buf[HEADER_LEN..HEADER_LEN + data_len],
            self.byte_offset,
        );
        self.byte_offset += rendered;

        usb.ep_write(1, &buf[..HEADER_LEN + rendered]);

        if is_last {
            self.sending = false;
            self.frame_id = !self.frame_id;
        }
    }
}

/// Recommended USB peripheral / FIFO configuration for video.
pub fn usb_config() -> UsbConfig {
    UsbConfig {
        rx_fifo_words: 64,
        tx0_fifo_words: 48,  // 192 bytes - fits 167-byte config descriptor
        tx1_fifo_words: 144, // 576 bytes - fits 512-byte isochronous packets
        tx2_fifo_words: 0,
        ep1: Some(EpConfig {
            ep_type: EpType::Isochronous,
            mps: MAX_PACKET as u16,
            has_in: true,
            has_out: false,
        }),
        ep2: None,
    }
}

impl<H: VideoHandler> UsbClass for VideoClass<H> {
    fn device_descriptor(&self) -> &[u8] {
        &DEVICE_DESC
    }
    fn config_descriptor(&self) -> &[u8] {
        &CONFIG_DESC
    }
    fn string_descriptor(&self, index: u8) -> Option<&[u8]> {
        match index {
            1 => Some(&STRING1),
            2 => Some(&STRING2),
            _ => None,
        }
    }

    fn handle_setup(&mut self, setup: &SetupPacket, usb: &UsbBus) -> SetupResult {
        // VS_PROBE_CONTROL and VS_COMMIT_CONTROL
        const SET_CUR: u8 = 0x01;
        const GET_CUR: u8 = 0x81;
        const GET_MIN: u8 = 0x82;
        const GET_MAX: u8 = 0x83;
        const GET_DEF: u8 = 0x87;

        const VS_PROBE: u16 = 0x0100;
        const VS_COMMIT: u16 = 0x0200;

        match (setup.bm_request_type, setup.b_request) {
            // GET on VS interface (class, interface recipient, device-to-host)
            (0xA1, GET_CUR | GET_MIN | GET_MAX | GET_DEF)
                if setup.w_value == VS_PROBE || setup.w_value == VS_COMMIT =>
            {
                defmt::info!("VS Probe/Commit GET");
                let len = (setup.w_length as usize).min(PROBE_COMMIT.len());
                usb.ep0_write_packet(&PROBE_COMMIT[..len]);
                SetupResult::DataIn
            }
            // SET_CUR on VS interface (class, interface recipient, host-to-device)
            (0x21, SET_CUR) if setup.w_value == VS_PROBE || setup.w_value == VS_COMMIT => {
                defmt::info!("VS Probe/Commit SET");
                // Accept and discard the host's probe/commit data.
                SetupResult::DataOut
            }
            _ => SetupResult::Unhandled,
        }
    }

    fn ep0_data_out(&mut self, _data: &[u8], _usb: &UsbBus) {
        // Discard VS_PROBE/VS_COMMIT SET_CUR data - we only support one format.
    }

    fn set_interface(&mut self, interface: u8, alt_setting: u8, usb: &UsbBus) {
        if interface == 1 {
            self.alt_setting = alt_setting;
            if alt_setting == 1 {
                defmt::info!("Video streaming started");
                self.streaming = true;
                self.frame_timer = FRAME_INTERVAL;
                self.sending = false;
                // Kick off the isochronous chain with an initial packet.
                self.send_packet(usb);
            } else {
                defmt::info!("Video streaming stopped");
                self.streaming = false;
            }
        }
    }

    fn get_interface(&self, interface: u8) -> u8 {
        if interface == 1 {
            self.alt_setting
        } else {
            0
        }
    }

    fn in_complete(&mut self, ep: u8, usb: &UsbBus) {
        if ep == 1 && self.streaming {
            self.frame_timer += 1;
            self.send_packet(usb);
        }
    }

    fn configured(&mut self, _usb: &UsbBus) {
        CONFIGURED.store(true, Ordering::Release);
        defmt::info!("Video device configured");
    }

    fn reset(&mut self) {
        CONFIGURED.store(false, Ordering::Release);
        self.streaming = false;
        self.alt_setting = 0;
    }

    fn suspended(&mut self) {
        CONFIGURED.store(false, Ordering::Release);
        self.streaming = false;
    }
}
