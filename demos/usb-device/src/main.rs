//! USB CDC ACM (virtual serial port) demo for the SLSTK3400A.
//!
//! The EFM32HG322 enumerates as a USB serial port (CDC ACM class). Data
//! received from the host is echoed back, and LED 0 toggles on every received
//! chunk. LED 1 lights once the host opens the port (SET_CONTROL_LINE_STATE
//! with DTR asserted).
//!
//! Endpoint mapping (DWC2 with 3 IN + 3 OUT endpoints):
//!   EP0  – Control (SETUP / status)
//!   EP1  – CDC Data Bulk IN + OUT  (serial data)
//!   EP2  – CDC Communication Interrupt IN (notifications, unused but required)

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use crate::pac::interrupt;
use core::cell::RefCell;
use core::ops::DerefMut;
use critical_section::Mutex;
use efm32hg322_hal as hal;
use efm32hg322_pac as pac;
use efm32hg322_usb::{
    usb_string, EpConfig, EpType, SetupPacket, SetupResult, UsbBus, UsbClass, UsbConfig,
    UsbDevice,
};
use hal::clocks::enable_gpio_clock;
use hal::gpio::GPIOExt;
use hal::watchdog::WatchdogExt;
use portable_atomic::{AtomicBool, Ordering};
use slstk3400a::SlStk3400a;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const EP1_MPS: u16 = 64; // Bulk max packet size (full speed)
const EP2_MPS: u16 = 8; // Interrupt notification endpoint

// ---------------------------------------------------------------------------
// USB descriptors
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

/// Total length of the configuration descriptor bundle.
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

// ---- String descriptors ----

static STRING1: [u8; 2 + 10 * 2] = usb_string!("SLSTK3400A");
static STRING2: [u8; 2 + 16 * 2] = usb_string!("EFM32 USB Serial");

// ---------------------------------------------------------------------------
// CDC ACM class driver
// ---------------------------------------------------------------------------

struct CdcAcmClass {
    line_coding: [u8; 7],
}

impl CdcAcmClass {
    fn new() -> Self {
        Self {
            // Default: 115200 baud, 1 stop bit, no parity, 8 data bits.
            line_coding: [0x00, 0xC2, 0x01, 0x00, 0x00, 0x00, 0x08],
        }
    }
}

impl UsbClass for CdcAcmClass {
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
            // SET_LINE_CODING – host sends 7-byte payload in DATA stage.
            (0x21, SET_LINE_CODING) => {
                defmt::info!("SET_LINE_CODING (waiting for data)");
                SetupResult::DataOut
            }

            // GET_LINE_CODING – return current line coding.
            (0xA1, GET_LINE_CODING) => {
                defmt::info!("GET_LINE_CODING");
                usb.ep0_write(&self.line_coding, setup.w_length as usize);
                SetupResult::DataIn
            }

            // SET_CONTROL_LINE_STATE – DTR / RTS signals from host.
            (0x21, SET_CONTROL_LINE_STATE) => {
                let dtr = setup.w_value & 0x01 != 0;
                defmt::info!("SET_CONTROL_LINE_STATE DTR={}", dtr);
                critical_section::with(|lock| {
                    if let Some(board) = BOARD.borrow(lock).borrow_mut().deref_mut() {
                        if dtr {
                            board.leds_mut()[1].on();
                        } else {
                            board.leds_mut()[1].off();
                        }
                    }
                });
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
            defmt::debug!("RX {} bytes", data.len());

            // Toggle LED 0 on received data.
            critical_section::with(|lock| {
                if let Some(board) = BOARD.borrow(lock).borrow_mut().deref_mut() {
                    let t = RX_TOGGLE.fetch_xor(true, Ordering::Relaxed);
                    if t {
                        board.leds_mut()[0].on();
                    } else {
                        board.leds_mut()[0].off();
                    }
                }
            });

            // Echo back.
            usb.ep_write(1, data);
        }
    }

    fn in_complete(&mut self, ep: u8, usb: &UsbBus) {
        if ep == 1 {
            // Re-arm EP1 OUT for next host data after echo completes.
            usb.ep_prepare_out(1, EP1_MPS);
        }
    }

    fn reset(&mut self) {
        critical_section::with(|lock| {
            if let Some(board) = BOARD.borrow(lock).borrow_mut().deref_mut() {
                board.leds_mut()[0].off();
                board.leds_mut()[1].off();
            }
        });
    }

    fn suspended(&mut self) {
        critical_section::with(|lock| {
            if let Some(board) = BOARD.borrow(lock).borrow_mut().deref_mut() {
                board.leds_mut()[0].off();
                board.leds_mut()[1].off();
            }
        });
    }
}

// ---------------------------------------------------------------------------
// Global state
// ---------------------------------------------------------------------------

static BOARD: Mutex<RefCell<Option<SlStk3400a>>> = Mutex::new(RefCell::new(None));
static USB_DEV: Mutex<RefCell<Option<UsbDevice<CdcAcmClass>>>> = Mutex::new(RefCell::new(None));

static RX_TOGGLE: AtomicBool = AtomicBool::new(false);

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
// Main
// ---------------------------------------------------------------------------

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    p.wdog.constrain().disable();

    enable_gpio_clock();
    let gpio = p.gpio.constrain().split();
    let board = SlStk3400a::new(gpio).unwrap();
    critical_section::with(|lock| {
        BOARD.borrow(lock).replace(Some(board));
    });

    let dev = UsbDevice::init(
        &p.cmu,
        &p.usb,
        CdcAcmClass::new(),
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
        },
    );

    critical_section::with(|lock| {
        USB_DEV.borrow(lock).replace(Some(dev));
    });

    pac::NVIC::unpend(pac::Interrupt::USB);
    unsafe { pac::NVIC::unmask(pac::Interrupt::USB) };

    defmt::info!("USB CDC ACM serial port ready — connect cable now");

    loop {
        cortex_m::asm::wfi();
    }
}
