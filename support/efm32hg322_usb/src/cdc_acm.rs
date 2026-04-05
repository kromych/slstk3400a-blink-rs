//! USB CDC ACM (virtual serial port) class driver.
//!
//! The library handles the CDC protocol (SET_LINE_CODING, GET_LINE_CODING,
//! SET_CONTROL_LINE_STATE). Application behaviour is provided through the
//! [`CdcAcmHandler`] trait.

use crate::{usb_string, EpConfig, EpType, SetupPacket, SetupResult, UsbBus, UsbClass, UsbConfig};
use portable_atomic::{AtomicBool, Ordering};

/// Set once the host completes SET_CONFIGURATION.
pub static CONFIGURED: AtomicBool = AtomicBool::new(false);

/// Non-blocking check.
pub fn is_configured() -> bool {
    CONFIGURED.load(Ordering::Acquire)
}

const EP1_MPS: u16 = 64;
const EP2_MPS: u16 = 8;

// ---------------------------------------------------------------------------
// Handler trait
// ---------------------------------------------------------------------------

/// Application-specific behaviour for a CDC ACM device.
pub trait CdcAcmHandler {
    /// Called when data is received from the host on the bulk OUT endpoint.
    fn data_received(&mut self, data: &[u8], usb: &UsbBus);

    /// Called when a bulk IN transfer completes (e.g. to re-arm the OUT endpoint).
    fn tx_complete(&mut self, _usb: &UsbBus) {}

    /// Called when the host asserts or deasserts DTR via SET_CONTROL_LINE_STATE.
    fn dtr_changed(&mut self, _dtr: bool) {}
}

// ---------------------------------------------------------------------------
// Descriptors
// ---------------------------------------------------------------------------

#[rustfmt::skip]
static DEVICE_DESC: [u8; 18] = [
    18,         // bLength
    0x01,       // bDescriptorType (Device)
    0x00, 0x02, // bcdUSB 2.00
    0x02,       // bDeviceClass    (Communications – CDC)
    0x00,       // bDeviceSubClass
    0x00,       // bDeviceProtocol
    64,         // bMaxPacketSize0
    0xFE, 0xCA, // idVendor  0xCAFE (test VID)
    0x02, 0x00, // idProduct 0x0002
    0x00, 0x01, // bcdDevice 1.00
    1,          // iManufacturer
    2,          // iProduct
    0,          // iSerialNumber
    1,          // bNumConfigurations
];

const CONFIG_TOTAL_LEN: u16 = 9 + 9 + 5 + 5 + 4 + 5 + 7 + 9 + 7 + 7; // = 67

#[rustfmt::skip]
static CONFIG_DESC: [u8; CONFIG_TOTAL_LEN as usize] = [
    // ---- Configuration Descriptor ----
    9, 0x02,
    (CONFIG_TOTAL_LEN & 0xFF) as u8, (CONFIG_TOTAL_LEN >> 8) as u8,
    2,              // bNumInterfaces
    1,              // bConfigurationValue
    0,              // iConfiguration
    0x80,           // bmAttributes (bus-powered)
    50,             // bMaxPower (100 mA)

    // ---- Interface 0: CDC Communication ----
    9, 0x04,
    0,              // bInterfaceNumber
    0,              // bAlternateSetting
    1,              // bNumEndpoints (interrupt IN)
    0x02,           // bInterfaceClass    (Communications)
    0x02,           // bInterfaceSubClass (Abstract Control Model)
    0x00,           // bInterfaceProtocol (none – no AT commands)
    0,              // iInterface

    // CDC Header Functional Descriptor
    5, 0x24, 0x00,
    0x20, 0x01,     // bcdCDC 1.20

    // CDC Call Management Functional Descriptor
    5, 0x24, 0x01,
    0x00,           // bmCapabilities (no call management)
    1,              // bDataInterface

    // CDC Abstract Control Management Functional Descriptor
    4, 0x24, 0x02,
    0x02,           // bmCapabilities (line coding + serial state)

    // CDC Union Functional Descriptor
    5, 0x24, 0x06,
    0,              // bControlInterface
    1,              // bSubordinateInterface0

    // Endpoint 2 IN – Interrupt (notifications)
    7, 0x05,
    0x82,           // bEndpointAddress (EP2 IN)
    0x03,           // bmAttributes     (Interrupt)
    (EP2_MPS & 0xFF) as u8, (EP2_MPS >> 8) as u8,
    255,            // bInterval (ms)

    // ---- Interface 1: CDC Data ----
    9, 0x04,
    1,              // bInterfaceNumber
    0,              // bAlternateSetting
    2,              // bNumEndpoints (bulk IN + bulk OUT)
    0x0A,           // bInterfaceClass    (CDC Data)
    0x00,           // bInterfaceSubClass
    0x00,           // bInterfaceProtocol
    0,              // iInterface

    // Endpoint 1 IN – Bulk (device → host)
    7, 0x05,
    0x81,           // bEndpointAddress (EP1 IN)
    0x02,           // bmAttributes     (Bulk)
    (EP1_MPS & 0xFF) as u8, (EP1_MPS >> 8) as u8,
    0,              // bInterval

    // Endpoint 1 OUT – Bulk (host → device)
    7, 0x05,
    0x01,           // bEndpointAddress (EP1 OUT)
    0x02,           // bmAttributes     (Bulk)
    (EP1_MPS & 0xFF) as u8, (EP1_MPS >> 8) as u8,
    0,              // bInterval
];

static STRING1: [u8; 2 + 10 * 2] = usb_string!("SLSTK3400A");
static STRING2: [u8; 2 + 16 * 2] = usb_string!("EFM32 USB Serial");

// ---------------------------------------------------------------------------
// Class driver
// ---------------------------------------------------------------------------

pub struct CdcAcmClass<H: CdcAcmHandler> {
    line_coding: [u8; 7],
    handler: H,
}

impl<H: CdcAcmHandler> CdcAcmClass<H> {
    pub fn new(handler: H) -> Self {
        Self {
            // Default: 115200 baud, 1 stop bit, no parity, 8 data bits.
            line_coding: [0x00, 0xC2, 0x01, 0x00, 0x00, 0x00, 0x08],
            handler,
        }
    }
}

/// Recommended USB peripheral / FIFO configuration for CDC ACM.
pub fn usb_config() -> UsbConfig {
    UsbConfig {
        rx_fifo_words: 64,
        tx0_fifo_words: 24,
        tx1_fifo_words: 64,
        tx2_fifo_words: 16,
        ep1: Some(EpConfig {
            ep_type: EpType::Bulk,
            mps: EP1_MPS,
            has_in: true,
            has_out: true,
        }),
        ep2: Some(EpConfig {
            ep_type: EpType::Interrupt,
            mps: EP2_MPS,
            has_in: true,
            has_out: false,
        }),
    }
}

impl<H: CdcAcmHandler> UsbClass for CdcAcmClass<H> {
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
        const SET_LINE_CODING: u8 = 0x20;
        const GET_LINE_CODING: u8 = 0x21;
        const SET_CONTROL_LINE_STATE: u8 = 0x22;

        match (setup.bm_request_type, setup.b_request) {
            (0x21, SET_LINE_CODING) => {
                defmt::info!("SET_LINE_CODING (waiting for data)");
                SetupResult::DataOut
            }

            (0xA1, GET_LINE_CODING) => {
                defmt::info!("GET_LINE_CODING");
                let len = (setup.w_length as usize).min(self.line_coding.len());
                usb.ep0_write_packet(&self.line_coding[..len]);
                SetupResult::DataIn
            }

            (0x21, SET_CONTROL_LINE_STATE) => {
                let dtr = setup.w_value & 0x01 != 0;
                defmt::info!("SET_CONTROL_LINE_STATE DTR={}", dtr);
                self.handler.dtr_changed(dtr);
                SetupResult::Handled
            }

            _ => SetupResult::Unhandled,
        }
    }

    fn ep0_data_out(&mut self, data: &[u8], _usb: &UsbBus) {
        if data.len() >= 7 {
            self.line_coding.copy_from_slice(&data[..7]);
            let baud = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
            defmt::info!(
                "Line coding: baud={} stop={} par={} bits={}",
                baud,
                data[4],
                data[5],
                data[6]
            );
        }
    }

    fn data_out(&mut self, ep: u8, data: &[u8], usb: &UsbBus) {
        if ep == 1 && !data.is_empty() {
            self.handler.data_received(data, usb);
        }
    }

    fn in_complete(&mut self, ep: u8, usb: &UsbBus) {
        if ep == 1 {
            self.handler.tx_complete(usb);
        }
    }

    fn configured(&mut self, _usb: &UsbBus) {
        CONFIGURED.store(true, Ordering::Release);
        defmt::info!("CDC ACM configured");
    }

    fn reset(&mut self) {
        CONFIGURED.store(false, Ordering::Release);
    }

    fn suspended(&mut self) {
        CONFIGURED.store(false, Ordering::Release);
    }
}
