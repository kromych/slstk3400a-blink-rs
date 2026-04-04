//! USB MIDI device demo for the SLSTK3400A.
//!
//! The EFM32HG322 enumerates as a USB MIDI device and plays "Twinkle Twinkle
//! Little Star" in a loop once a host MIDI application opens the device.
//!
//! Endpoint mapping:
//!   EP0  – Control (SETUP / status)
//!   EP1  – MIDI Streaming Bulk IN + OUT

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
// USB descriptors (USB MIDI 1.0, Appendix B)
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
    0x03, 0x00, // idProduct 0x0003
    0x00, 0x01, // bcdDevice 1.00
    1,          // iManufacturer
    2,          // iProduct
    0,          // iSerialNumber
    1,          // bNumConfigurations
];

/// Configuration descriptor: Audio Control + MIDI Streaming interfaces.
const CONFIG_TOTAL_LEN: u16 = 9 + 9 + 9 + 9 + 7 + 6 + 6 + 9 + 9 + 9 + 5 + 9 + 5; // = 101
const _: () = assert!(CONFIG_TOTAL_LEN == 101);

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

    // ---- Interface 0: Audio Control ----
    9, 0x04,
    0,              // bInterfaceNumber
    0,              // bAlternateSetting
    0,              // bNumEndpoints
    0x01,           // bInterfaceClass (Audio)
    0x01,           // bInterfaceSubClass (Audio Control)
    0x00,           // bInterfaceProtocol
    0,              // iInterface

    // Audio Control Interface Header Descriptor
    9, 0x24, 0x01,  // bDescriptorType = CS_INTERFACE, bDescriptorSubtype = HEADER
    0x00, 0x01,     // bcdADC 1.00
    0x09, 0x00,     // wTotalLength (9 bytes, just this header)
    1,              // bInCollection (1 streaming interface)
    1,              // baInterfaceNr(1) — MIDI Streaming interface number

    // ---- Interface 1: MIDI Streaming ----
    9, 0x04,
    1,              // bInterfaceNumber
    0,              // bAlternateSetting
    2,              // bNumEndpoints (bulk IN + bulk OUT)
    0x01,           // bInterfaceClass (Audio)
    0x03,           // bInterfaceSubClass (MIDI Streaming)
    0x00,           // bInterfaceProtocol
    0,              // iInterface

    // MIDI Streaming Interface Header Descriptor
    7, 0x24, 0x01,  // CS_INTERFACE, MS_HEADER
    0x00, 0x01,     // bcdMSC 1.00
    0x41, 0x00,     // wTotalLength (65 bytes: header + 4 jacks + 2 CS EP)

    // MIDI IN Jack (Embedded) — receives from host OUT endpoint
    6, 0x24, 0x02,  // CS_INTERFACE, MIDI_IN_JACK
    0x01,           // bJackType (Embedded)
    1,              // bJackID
    0,              // iJack

    // MIDI IN Jack (External) — represents physical MIDI IN
    6, 0x24, 0x02,
    0x02,           // bJackType (External)
    2,              // bJackID
    0,              // iJack

    // MIDI OUT Jack (Embedded) — sends to host IN endpoint
    9, 0x24, 0x03,  // CS_INTERFACE, MIDI_OUT_JACK
    0x01,           // bJackType (Embedded)
    3,              // bJackID
    1,              // bNrInputPins
    2,              // baSourceID(1) — connected to External IN Jack
    1,              // baSourcePin(1)
    0,              // iJack

    // MIDI OUT Jack (External) — represents physical MIDI OUT
    9, 0x24, 0x03,
    0x02,           // bJackType (External)
    4,              // bJackID
    1,              // bNrInputPins
    1,              // baSourceID(1) — connected to Embedded IN Jack
    1,              // baSourcePin(1)
    0,              // iJack

    // ---- Endpoint 1 OUT – Bulk (host → device) ----
    9, 0x05,
    0x01,           // bEndpointAddress (EP1 OUT)
    0x02,           // bmAttributes (Bulk)
    0x40, 0x00,     // wMaxPacketSize (64)
    0,              // bInterval
    0,              // bRefresh
    0,              // bSynchAddress

    // Class-specific MS Bulk Data Endpoint Descriptor (OUT)
    5, 0x25, 0x01,  // CS_ENDPOINT, MS_GENERAL
    1,              // bNumEmbMIDIJack
    1,              // baAssocJackID(1) — Embedded IN Jack

    // ---- Endpoint 1 IN – Bulk (device → host) ----
    9, 0x05,
    0x81,           // bEndpointAddress (EP1 IN)
    0x02,           // bmAttributes (Bulk)
    0x40, 0x00,     // wMaxPacketSize (64)
    0,              // bInterval
    0,              // bRefresh
    0,              // bSynchAddress

    // Class-specific MS Bulk Data Endpoint Descriptor (IN)
    5, 0x25, 0x01,  // CS_ENDPOINT, MS_GENERAL
    1,              // bNumEmbMIDIJack
    3,              // baAssocJackID(1) — Embedded OUT Jack
];

// ---- String descriptors ----

static STRING1: [u8; 2 + 10 * 2] = usb_string!("SLSTK3400A");
static STRING2: [u8; 2 + 14 * 2] = usb_string!("EFM32 USB MIDI");

// ---------------------------------------------------------------------------
// MIDI class driver
// ---------------------------------------------------------------------------

static CONFIGURED: AtomicBool = AtomicBool::new(false);

struct MidiClass;

impl UsbClass for MidiClass {
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
        // MIDI class has no class-specific control requests.
        SetupResult::Unhandled
    }

    fn configured(&mut self, _usb: &UsbBus) {
        CONFIGURED.store(true, Ordering::Release);
        defmt::info!("MIDI device configured");
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

static USB_DEV: Mutex<RefCell<Option<UsbDevice<MidiClass>>>> = Mutex::new(RefCell::new(None));

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
// MIDI helpers
// ---------------------------------------------------------------------------

/// Send a 4-byte USB-MIDI event packet on EP1 IN.
fn send_midi_packet(packet: &[u8; 4]) {
    critical_section::with(|lock| {
        if let Some(dev) = USB_DEV.borrow(lock).borrow_mut().deref_mut() {
            dev.bus().ep_write(1, packet);
        }
    });
}

fn send_note_on(note: u8, velocity: u8) {
    // Cable 0, CIN 0x9 (Note On), channel 0
    send_midi_packet(&[0x09, 0x90, note, velocity]);
}

fn send_note_off(note: u8) {
    // Cable 0, CIN 0x8 (Note Off), channel 0
    send_midi_packet(&[0x08, 0x80, note, 0]);
}

/// Busy-wait delay. HFRCO runs at 14 MHz by default.
fn delay_ms(ms: u32) {
    cortex_m::asm::delay(ms * 14_000);
}

// ---------------------------------------------------------------------------
// Tune data: "Twinkle Twinkle Little Star"
// (MIDI note, duration in ms)
// ---------------------------------------------------------------------------

#[rustfmt::skip]
static TUNE: &[(u8, u32)] = &[
    // "Twinkle, twinkle, little star"
    (60, 400), (60, 400), (67, 400), (67, 400),
    (69, 400), (69, 400), (67, 800),
    // "How I wonder what you are"
    (65, 400), (65, 400), (64, 400), (64, 400),
    (62, 400), (62, 400), (60, 800),
    // "Up above the world so high"
    (67, 400), (67, 400), (65, 400), (65, 400),
    (64, 400), (64, 400), (62, 800),
    // "Like a diamond in the sky"
    (67, 400), (67, 400), (65, 400), (65, 400),
    (64, 400), (64, 400), (62, 800),
    // "Twinkle, twinkle, little star"
    (60, 400), (60, 400), (67, 400), (67, 400),
    (69, 400), (69, 400), (67, 800),
    // "How I wonder what you are"
    (65, 400), (65, 400), (64, 400), (64, 400),
    (62, 400), (62, 400), (60, 800),
];

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    // Disable watchdog (no HAL/board crate needed for this minimal demo).
    p.wdog.ctrl().write(|w| w.en().clear_bit());

    let dev = UsbDevice::init(
        &p.cmu,
        &p.usb,
        MidiClass,
        UsbConfig {
            // MIDI doesn't use EP2, so give more room to TX0 for the 101-byte
            // config descriptor.
            rx_fifo_words: 64,
            tx0_fifo_words: 28, // 112 bytes, fits 101-byte config desc
            tx1_fifo_words: 64,
            tx2_fifo_words: 0,
            ep1: Some(EpConfig {
                ep_type: EpType::Bulk,
                mps: 64,
                has_in: true,
                has_out: true,
            }),
            ep2: None,
        },
    );

    critical_section::with(|lock| {
        USB_DEV.borrow(lock).replace(Some(dev));
    });

    pac::NVIC::unpend(pac::Interrupt::USB);
    unsafe { pac::NVIC::unmask(pac::Interrupt::USB) };

    defmt::info!("USB MIDI device ready — connect cable now");

    loop {
        if !CONFIGURED.load(Ordering::Acquire) {
            cortex_m::asm::wfi();
            continue;
        }

        // Small delay after configuration to let the host finish setup.
        delay_ms(500);

        for &(note, dur) in TUNE.iter() {
            if !CONFIGURED.load(Ordering::Acquire) {
                break;
            }
            send_note_on(note, 64);
            delay_ms(dur);
            send_note_off(note);
            delay_ms(50); // inter-note gap
        }

        // Pause between repeats.
        delay_ms(1000);
    }
}
