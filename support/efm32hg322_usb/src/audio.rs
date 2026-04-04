//! USB Audio Class 1.0 driver (source / virtual synthesizer).
//!
//! Streams 8 kHz 16-bit mono PCM to the host via an isochronous IN endpoint.
//! The application provides samples through the [`AudioHandler`] trait.

use crate::{usb_string, EpConfig, EpType, SetupPacket, SetupResult, UsbBus, UsbClass, UsbConfig};
use portable_atomic::{AtomicBool, Ordering};

/// Set once the host completes SET_CONFIGURATION.
pub static CONFIGURED: AtomicBool = AtomicBool::new(false);

/// PCM sample rate in Hz.
pub const SAMPLE_RATE: u32 = 8000;
/// Samples per USB frame (1 ms at full-speed).
pub const SAMPLES_PER_FRAME: usize = (SAMPLE_RATE / 1000) as usize; // 8
/// Bytes per USB frame (16-bit mono).
const FRAME_BYTES: usize = SAMPLES_PER_FRAME * 2; // 16

// ---------------------------------------------------------------------------
// Descriptors (USB Audio Class 1.0)
// ---------------------------------------------------------------------------

#[rustfmt::skip]
static DEVICE_DESC: [u8; 18] = [
    18,         // bLength
    0x01,       // bDescriptorType (Device)
    0x00, 0x02, // bcdUSB 2.00
    0x00,       // bDeviceClass    (defined at interface level)
    0x00,       // bDeviceSubClass
    0x00,       // bDeviceProtocol
    64,         // bMaxPacketSize0
    0xFE, 0xCA, // idVendor  0xCAFE (test VID)
    0x05, 0x00, // idProduct 0x0005
    0x00, 0x01, // bcdDevice 1.00
    1,          // iManufacturer
    2,          // iProduct
    0,          // iSerialNumber
    1,          // bNumConfigurations
];

// Config total = 9 + 9 + 9 + 12 + 9 + 9 + 9 + 7 + 11 + 9 + 7 = 100
const CONFIG_TOTAL_LEN: u16 = 100;
const _: () = assert!(CONFIG_TOTAL_LEN == 100);

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

    // ---- Interface 0: Audio Control (9) ----
    9, 0x04,
    0,              // bInterfaceNumber
    0,              // bAlternateSetting
    0,              // bNumEndpoints
    0x01,           // bInterfaceClass (Audio)
    0x01,           // bInterfaceSubClass (Audio Control)
    0x00,           // bInterfaceProtocol
    0,              // iInterface

    // AC Interface Header (9)
    9, 0x24, 0x01,  // CS_INTERFACE, HEADER
    0x00, 0x01,     // bcdADC 1.00
    30, 0x00,       // wTotalLength (9 + 12 + 9 = 30)
    1,              // bInCollection
    1,              // baInterfaceNr(1) - streaming interface

    // Input Terminal (12)
    12, 0x24, 0x02, // CS_INTERFACE, INPUT_TERMINAL
    1,              // bTerminalID
    0x03, 0x06,     // wTerminalType (0x0603 = Synthesizer)
    0,              // bAssocTerminal
    1,              // bNrChannels
    0x00, 0x00,     // wChannelConfig (mono, no spatial)
    0,              // iChannelNames
    0,              // iTerminal

    // Output Terminal (9)
    9, 0x24, 0x03,  // CS_INTERFACE, OUTPUT_TERMINAL
    2,              // bTerminalID
    0x01, 0x01,     // wTerminalType (0x0101 = USB Streaming)
    0,              // bAssocTerminal
    1,              // bSourceID (Input Terminal)
    0,              // iTerminal

    // ---- Interface 1, Alt 0: Audio Streaming (zero-bandwidth) (9) ----
    9, 0x04,
    1,              // bInterfaceNumber
    0,              // bAlternateSetting
    0,              // bNumEndpoints
    0x01,           // bInterfaceClass (Audio)
    0x02,           // bInterfaceSubClass (Audio Streaming)
    0x00,           // bInterfaceProtocol
    0,              // iInterface

    // ---- Interface 1, Alt 1: Audio Streaming (active) (9) ----
    9, 0x04,
    1,              // bInterfaceNumber
    1,              // bAlternateSetting
    1,              // bNumEndpoints
    0x01,           // bInterfaceClass (Audio)
    0x02,           // bInterfaceSubClass (Audio Streaming)
    0x00,           // bInterfaceProtocol
    0,              // iInterface

    // AS General (7)
    7, 0x24, 0x01,  // CS_INTERFACE, AS_GENERAL
    2,              // bTerminalLink (Output Terminal)
    1,              // bDelay (1 frame)
    0x01, 0x00,     // wFormatTag (PCM)

    // Format Type I (11)
    11, 0x24, 0x02, // CS_INTERFACE, FORMAT_TYPE
    0x01,           // bFormatType (TYPE_I)
    1,              // bNrChannels
    2,              // bSubFrameSize (2 bytes)
    16,             // bBitResolution
    1,              // bSamFreqType (1 discrete rate)
    0x40, 0x1F, 0x00, // tSamFreq = 8000 Hz (LE 3 bytes)

    // ---- Endpoint 1 IN - Isochronous (9) ----
    9, 0x05,
    0x81,           // bEndpointAddress (EP1 IN)
    0x05,           // bmAttributes (isochronous, asynchronous)
    FRAME_BYTES as u8, 0x00, // wMaxPacketSize (16)
    1,              // bInterval (every frame)
    0,              // bRefresh
    0,              // bSynchAddress

    // CS Audio Data Endpoint (7)
    7, 0x25, 0x01,  // CS_ENDPOINT, EP_GENERAL
    0x00,           // bmAttributes (no controls)
    0x00,           // bLockDelayUnits
    0x00, 0x00,     // wLockDelay
];

static STRING1: [u8; 2 + 10 * 2] = usb_string!("SLSTK3400A");
static STRING2: [u8; 2 + 16 * 2] = usb_string!("EFM32 USB Synth.");

// ---------------------------------------------------------------------------
// Handler trait
// ---------------------------------------------------------------------------

/// Application-provided PCM sample source.
pub trait AudioHandler {
    /// Fill `buf` with 16-bit signed PCM samples for one 1 ms frame.
    /// Called from ISR context on each isochronous transfer completion.
    fn fill_frame(&mut self, buf: &mut [i16; SAMPLES_PER_FRAME]);
}

// ---------------------------------------------------------------------------
// Class driver
// ---------------------------------------------------------------------------

pub struct AudioClass<H: AudioHandler> {
    handler: H,
    streaming: bool,
    alt_setting: u8,
}

impl<H: AudioHandler> AudioClass<H> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            streaming: false,
            alt_setting: 0,
        }
    }

    fn send_frame(&mut self, usb: &UsbBus) {
        let mut samples = [0i16; SAMPLES_PER_FRAME];
        self.handler.fill_frame(&mut samples);
        let mut buf = [0u8; FRAME_BYTES];
        for (i, &s) in samples.iter().enumerate() {
            let le = s.to_le_bytes();
            buf[i * 2] = le[0];
            buf[i * 2 + 1] = le[1];
        }
        usb.ep_write(1, &buf);
    }
}

/// Recommended USB peripheral / FIFO configuration for audio.
pub fn usb_config() -> UsbConfig {
    UsbConfig {
        rx_fifo_words: 64,
        tx0_fifo_words: 28,
        tx1_fifo_words: 16,
        tx2_fifo_words: 0,
        ep1: Some(EpConfig {
            ep_type: EpType::Isochronous,
            mps: FRAME_BYTES as u16,
            has_in: true,
            has_out: false,
        }),
        ep2: None,
    }
}

impl<H: AudioHandler> UsbClass for AudioClass<H> {
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

    fn handle_setup(&mut self, _setup: &SetupPacket, _usb: &UsbBus) -> SetupResult {
        SetupResult::Unhandled
    }

    fn set_interface(&mut self, interface: u8, alt_setting: u8, usb: &UsbBus) {
        if interface == 1 {
            self.alt_setting = alt_setting;
            if alt_setting == 1 {
                defmt::info!("Audio streaming started");
                self.streaming = true;
                self.send_frame(usb);
            } else {
                defmt::info!("Audio streaming stopped");
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
            self.send_frame(usb);
        }
    }

    fn configured(&mut self, _usb: &UsbBus) {
        CONFIGURED.store(true, Ordering::Release);
        defmt::info!("Audio device configured");
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
