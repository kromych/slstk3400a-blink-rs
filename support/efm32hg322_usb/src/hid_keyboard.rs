//! USB HID keyboard class driver.
//!
//! Enumerates as a boot-protocol keyboard. The demo sends 8-byte HID reports
//! via [`UsbBus::ep_write`] on EP1 IN.

use crate::{usb_string, EpConfig, EpType, SetupPacket, SetupResult, UsbBus, UsbClass, UsbConfig};
use portable_atomic::{AtomicBool, Ordering};

/// Set once the host completes SET_CONFIGURATION.
pub static CONFIGURED: AtomicBool = AtomicBool::new(false);

// ---------------------------------------------------------------------------
// Descriptors
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
    0x04, 0x00, // idProduct 0x0004
    0x00, 0x01, // bcdDevice 1.00
    1,          // iManufacturer
    2,          // iProduct
    0,          // iSerialNumber
    1,          // bNumConfigurations
];

/// HID Report Descriptor - standard keyboard (no LED output).
/// 8-byte reports: [modifiers, reserved, key1..key6]
#[rustfmt::skip]
static REPORT_DESC: [u8; 45] = [
    0x05, 0x01,       // Usage Page (Generic Desktop)
    0x09, 0x06,       // Usage (Keyboard)
    0xA1, 0x01,       // Collection (Application)

    // Modifier keys (1 byte, 8 bits)
    0x05, 0x07,       //   Usage Page (Key Codes)
    0x19, 0xE0,       //   Usage Minimum (Left Control)
    0x29, 0xE7,       //   Usage Maximum (Right GUI)
    0x15, 0x00,       //   Logical Minimum (0)
    0x25, 0x01,       //   Logical Maximum (1)
    0x75, 0x01,       //   Report Size (1)
    0x95, 0x08,       //   Report Count (8)
    0x81, 0x02,       //   Input (Data, Variable, Absolute)

    // Reserved byte
    0x95, 0x01,       //   Report Count (1)
    0x75, 0x08,       //   Report Size (8)
    0x81, 0x01,       //   Input (Constant)

    // Key array (6 keys)
    0x95, 0x06,       //   Report Count (6)
    0x75, 0x08,       //   Report Size (8)
    0x15, 0x00,       //   Logical Minimum (0)
    0x25, 0x65,       //   Logical Maximum (101)
    0x05, 0x07,       //   Usage Page (Key Codes)
    0x19, 0x00,       //   Usage Minimum (0)
    0x29, 0x65,       //   Usage Maximum (101)
    0x81, 0x00,       //   Input (Data, Array)

    0xC0,             // End Collection
];

const REPORT_DESC_LEN: u16 = REPORT_DESC.len() as u16;

// Config: Configuration(9) + Interface(9) + HID(9) + Endpoint(7) = 34
const CONFIG_TOTAL_LEN: u16 = 34;

#[rustfmt::skip]
static CONFIG_DESC: [u8; CONFIG_TOTAL_LEN as usize] = [
    // ---- Configuration Descriptor ----
    9, 0x02,
    (CONFIG_TOTAL_LEN & 0xFF) as u8, (CONFIG_TOTAL_LEN >> 8) as u8,
    1,              // bNumInterfaces
    1,              // bConfigurationValue
    0,              // iConfiguration
    0x80,           // bmAttributes (bus-powered)
    50,             // bMaxPower (100 mA)

    // ---- Interface 0: HID Keyboard ----
    9, 0x04,
    0,              // bInterfaceNumber
    0,              // bAlternateSetting
    1,              // bNumEndpoints
    0x03,           // bInterfaceClass (HID)
    0x01,           // bInterfaceSubClass (Boot Interface)
    0x01,           // bInterfaceProtocol (Keyboard)
    0,              // iInterface

    // ---- HID Descriptor ----
    9, 0x21,        // bLength, bDescriptorType (HID)
    0x11, 0x01,     // bcdHID 1.11
    0x00,           // bCountryCode (not localized)
    1,              // bNumDescriptors
    0x22,           // bDescriptorType (Report)
    (REPORT_DESC_LEN & 0xFF) as u8, (REPORT_DESC_LEN >> 8) as u8,

    // ---- Endpoint 1 IN – Interrupt (keyboard reports) ----
    7, 0x05,
    0x81,           // bEndpointAddress (EP1 IN)
    0x03,           // bmAttributes (Interrupt)
    0x08, 0x00,     // wMaxPacketSize (8)
    10,             // bInterval (10 ms)
];

static STRING1: [u8; 2 + 10 * 2] = usb_string!("SLSTK3400A");
static STRING2: [u8; 2 + 18 * 2] = usb_string!("EFM32 USB Keyboard");

// ---------------------------------------------------------------------------
// HID key codes and helpers
// ---------------------------------------------------------------------------

pub const MOD_LSHIFT: u8 = 0x02;
pub const KEY_A: u8 = 0x04;
pub const KEY_ENTER: u8 = 0x28;
pub const KEY_SPACE: u8 = 0x2C;
pub const KEY_1: u8 = 0x1E;

/// Map an ASCII character to `(modifier, keycode)`.
pub const fn ascii_to_hid(ch: u8) -> (u8, u8) {
    match ch {
        b'a'..=b'z' => (0, KEY_A + (ch - b'a')),
        b'A'..=b'Z' => (MOD_LSHIFT, KEY_A + (ch - b'A')),
        b'1'..=b'9' => (0, KEY_1 + (ch - b'1')),
        b'0' => (0, 0x27),
        b' ' => (0, KEY_SPACE),
        b'!' => (MOD_LSHIFT, KEY_1),
        b'\n' => (0, KEY_ENTER),
        _ => (0, 0),
    }
}

// ---------------------------------------------------------------------------
// Handler trait
// ---------------------------------------------------------------------------

/// Application-provided content for the HID keyboard.
pub trait HidKeyboardHandler {
    /// The message to type each cycle.
    fn message(&self) -> &[u8];
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Block until the host completes SET_CONFIGURATION.
pub fn wait_configured() {
    while !CONFIGURED.load(Ordering::Acquire) {
        cortex_m::asm::wfi();
    }
}

/// Non-blocking check.
pub fn is_configured() -> bool {
    CONFIGURED.load(Ordering::Acquire)
}

/// Typing loop: wait for configuration, type the handler's message, repeat.
///
/// `send` is called with each 8-byte HID report.  A typical implementation:
/// ```ignore
/// hid_keyboard::run(&handler, |report| usb_with_bus(|bus| bus.ep_write(1, report)));
/// ```
pub fn run(handler: &impl HidKeyboardHandler, mut send: impl FnMut(&[u8; 8])) -> ! {
    loop {
        wait_configured();
        crate::delay_ms(2000);
        if is_configured() {
            for &ch in handler.message() {
                let (m, k) = ascii_to_hid(ch);
                if k != 0 {
                    send(&[m, 0x00, k, 0, 0, 0, 0, 0]);
                    crate::delay_ms(50);
                    send(&[0, 0, 0, 0, 0, 0, 0, 0]);
                    crate::delay_ms(30);
                }
            }
        }
        crate::delay_ms(5000);
    }
}

// ---------------------------------------------------------------------------
// Class driver
// ---------------------------------------------------------------------------

pub struct HidKeyboardClass;

/// Recommended USB peripheral / FIFO configuration for HID keyboard.
pub fn usb_config() -> UsbConfig {
    UsbConfig {
        rx_fifo_words: 64,
        tx0_fifo_words: 24,
        tx1_fifo_words: 16, // 8-byte keyboard reports
        tx2_fifo_words: 0,
        ep1: Some(EpConfig {
            ep_type: EpType::Interrupt,
            mps: 8,
            has_in: true,
            has_out: false,
        }),
        ep2: None,
    }
}

impl UsbClass for HidKeyboardClass {
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
        const GET_DESCRIPTOR: u8 = 0x06;
        const SET_IDLE: u8 = 0x0A;
        const SET_PROTOCOL: u8 = 0x0B;
        const DESC_HID_REPORT: u8 = 0x22;

        match (setup.bm_request_type, setup.b_request) {
            // GET_DESCRIPTOR (interface recipient) - HID Report Descriptor.
            (0x81, GET_DESCRIPTOR) => {
                let desc_type = (setup.w_value >> 8) as u8;
                if desc_type == DESC_HID_REPORT {
                    defmt::info!("GET_DESCRIPTOR HID Report");
                    let len = (setup.w_length as usize).min(REPORT_DESC.len());
                    usb.ep0_write_packet(&REPORT_DESC[..len]);
                    SetupResult::DataIn
                } else {
                    SetupResult::Unhandled
                }
            }

            // SET_IDLE - host sets idle rate; just ACK.
            (0x21, SET_IDLE) => {
                defmt::info!("SET_IDLE rate={}", (setup.w_value >> 8) as u8);
                SetupResult::Handled
            }

            // SET_PROTOCOL - boot/report protocol; just ACK.
            (0x21, SET_PROTOCOL) => {
                defmt::info!("SET_PROTOCOL {}", setup.w_value);
                SetupResult::Handled
            }

            _ => SetupResult::Unhandled,
        }
    }

    fn configured(&mut self, _usb: &UsbBus) {
        CONFIGURED.store(true, Ordering::Release);
        defmt::info!("HID keyboard configured");
    }

    fn reset(&mut self) {
        CONFIGURED.store(false, Ordering::Release);
    }

    fn suspended(&mut self) {
        CONFIGURED.store(false, Ordering::Release);
    }
}
