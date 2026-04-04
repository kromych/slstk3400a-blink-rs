//! USB HID keyboard demo for the SLSTK3400A.
//!
//! The EFM32HG322 enumerates as a USB keyboard and types "Hello from EFM32!\n"
//! every 5 seconds.
//!
//! Endpoint mapping:
//!   EP0  – Control (SETUP / status)
//!   EP1  – HID Interrupt IN (keyboard reports)

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use core::cell::RefCell;
use core::ops::DerefMut;
use critical_section::Mutex;
use efm32hg322_pac as pac;
use efm32hg322_usb::{
    usb_string, EpConfig, EpType, SetupPacket, SetupResult, UsbBus, UsbClass, UsbConfig,
    UsbDevice,
};
use pac::interrupt;
use portable_atomic::{AtomicBool, Ordering};

// ---------------------------------------------------------------------------
// USB descriptors
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

/// HID Report Descriptor — standard keyboard (no LED output).
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

// ---- String descriptors ----

static STRING1: [u8; 2 + 10 * 2] = usb_string!("SLSTK3400A");
static STRING2: [u8; 2 + 18 * 2] = usb_string!("EFM32 USB Keyboard");

// ---------------------------------------------------------------------------
// HID class driver
// ---------------------------------------------------------------------------

static CONFIGURED: AtomicBool = AtomicBool::new(false);

struct HidKeyboardClass;

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
            // GET_DESCRIPTOR (interface recipient) — HID Report Descriptor.
            (0x81, GET_DESCRIPTOR) => {
                let desc_type = (setup.w_value >> 8) as u8;
                if desc_type == DESC_HID_REPORT {
                    defmt::info!("GET_DESCRIPTOR HID Report");
                    usb.ep0_write(&REPORT_DESC, setup.w_length as usize);
                    SetupResult::DataIn
                } else {
                    SetupResult::Unhandled
                }
            }

            // SET_IDLE — host sets idle rate; just ACK.
            (0x21, SET_IDLE) => {
                defmt::info!("SET_IDLE rate={}", (setup.w_value >> 8) as u8);
                SetupResult::Handled
            }

            // SET_PROTOCOL — boot/report protocol; just ACK.
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

// ---------------------------------------------------------------------------
// Global state
// ---------------------------------------------------------------------------

static USB_DEV: Mutex<RefCell<Option<UsbDevice<HidKeyboardClass>>>> =
    Mutex::new(RefCell::new(None));

// ---------------------------------------------------------------------------
// USB interrupt handler
// ---------------------------------------------------------------------------

#[interrupt]
fn USB() {
    critical_section::with(|lock| {
        if let Some(dev) = USB_DEV.borrow(lock).borrow_mut().deref_mut() {
            dev.poll();
        }
    });
}

// ---------------------------------------------------------------------------
// Keyboard helpers
// ---------------------------------------------------------------------------

/// Send an 8-byte HID keyboard report on EP1 IN.
fn send_report(modifier: u8, keycode: u8) {
    let report = [modifier, 0x00, keycode, 0, 0, 0, 0, 0];
    critical_section::with(|lock| {
        if let Some(dev) = USB_DEV.borrow(lock).borrow_mut().deref_mut() {
            dev.bus().ep_write(1, &report);
        }
    });
}

/// Send key-down then key-up for a single character.
fn type_key(modifier: u8, keycode: u8) {
    send_report(modifier, keycode);
    delay_ms(50);
    send_report(0, 0); // key release
    delay_ms(30);
}

/// Busy-wait delay. HFRCO runs at 14 MHz by default.
fn delay_ms(ms: u32) {
    cortex_m::asm::delay(ms * 14_000);
}

// HID key codes
const MOD_LSHIFT: u8 = 0x02;
const KEY_A: u8 = 0x04;
const KEY_ENTER: u8 = 0x28;
const KEY_SPACE: u8 = 0x2C;
const KEY_1: u8 = 0x1E;

/// Map an ASCII character to (modifier, keycode).
const fn ascii_to_hid(ch: u8) -> (u8, u8) {
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

/// Type a string one character at a time.
fn type_string(s: &[u8]) {
    for &ch in s {
        let (modifier, keycode) = ascii_to_hid(ch);
        if keycode != 0 {
            type_key(modifier, keycode);
        }
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    p.wdog.ctrl().write(|w| w.en().clear_bit());

    let dev = UsbDevice::init(
        &p.cmu,
        &p.usb,
        HidKeyboardClass,
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
        },
    );

    critical_section::with(|lock| {
        USB_DEV.borrow(lock).replace(Some(dev));
    });

    pac::NVIC::unpend(pac::Interrupt::USB);
    unsafe { pac::NVIC::unmask(pac::Interrupt::USB) };

    defmt::info!("USB HID keyboard ready — connect cable now");

    loop {
        if !CONFIGURED.load(Ordering::Acquire) {
            cortex_m::asm::wfi();
            continue;
        }

        delay_ms(2000);

        if CONFIGURED.load(Ordering::Acquire) {
            type_string(b"Hello from EFM32!\n");
        }

        delay_ms(5000);
    }
}
