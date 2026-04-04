//! USB MIDI class driver.
//!
//! Enumerates as a USB MIDI device (Audio Control + MIDI Streaming).
//! The demo sends MIDI event packets via [`UsbBus::ep_write`] on EP1 IN.

use crate::{usb_string, EpConfig, EpType, SetupPacket, SetupResult, UsbBus, UsbClass, UsbConfig};
use portable_atomic::{AtomicBool, Ordering};

/// Set once the host completes SET_CONFIGURATION.
pub static CONFIGURED: AtomicBool = AtomicBool::new(false);

// ---------------------------------------------------------------------------
// Descriptors (USB MIDI 1.0, Appendix B)
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
    1,              // baInterfaceNr(1) - MIDI Streaming interface number

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

    // MIDI IN Jack (Embedded) - receives from host OUT endpoint
    6, 0x24, 0x02,  // CS_INTERFACE, MIDI_IN_JACK
    0x01,           // bJackType (Embedded)
    1,              // bJackID
    0,              // iJack

    // MIDI IN Jack (External) - represents physical MIDI IN
    6, 0x24, 0x02,
    0x02,           // bJackType (External)
    2,              // bJackID
    0,              // iJack

    // MIDI OUT Jack (Embedded) - sends to host IN endpoint
    9, 0x24, 0x03,  // CS_INTERFACE, MIDI_OUT_JACK
    0x01,           // bJackType (Embedded)
    3,              // bJackID
    1,              // bNrInputPins
    2,              // baSourceID(1) - connected to External IN Jack
    1,              // baSourcePin(1)
    0,              // iJack

    // MIDI OUT Jack (External) - represents physical MIDI OUT
    9, 0x24, 0x03,
    0x02,           // bJackType (External)
    4,              // bJackID
    1,              // bNrInputPins
    1,              // baSourceID(1) - connected to Embedded IN Jack
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
    1,              // baAssocJackID(1) - Embedded IN Jack

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
    3,              // baAssocJackID(1) - Embedded OUT Jack
];

static STRING1: [u8; 2 + 10 * 2] = usb_string!("SLSTK3400A");
static STRING2: [u8; 2 + 14 * 2] = usb_string!("EFM32 USB MIDI");

// ---------------------------------------------------------------------------
// MIDI packet helpers
// ---------------------------------------------------------------------------

/// Build a 4-byte USB-MIDI Note On event packet (cable 0).
pub fn note_on_packet(channel: u8, note: u8, velocity: u8) -> [u8; 4] {
    [0x09, 0x90 | (channel & 0x0F), note, velocity]
}

/// Build a 4-byte USB-MIDI Note Off event packet (cable 0).
pub fn note_off_packet(channel: u8, note: u8) -> [u8; 4] {
    [0x08, 0x80 | (channel & 0x0F), note, 0]
}

// ---------------------------------------------------------------------------
// Handler trait
// ---------------------------------------------------------------------------

/// Application-provided tune for the MIDI device.
pub trait MidiHandler {
    /// Sequence of `(midi_note, duration_ms)` pairs to play in a loop.
    fn tune(&self) -> &[(u8, u32)];
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

/// Playback loop: wait for configuration, play the handler's tune, repeat.
///
/// `send` is called with each 4-byte USB-MIDI event packet.  A typical
/// implementation:
/// ```ignore
/// midi::run(&handler, |pkt| usb_with_bus(|bus| bus.ep_write(1, pkt)));
/// ```
pub fn run(handler: &impl MidiHandler, mut send: impl FnMut(&[u8; 4])) -> ! {
    loop {
        wait_configured();
        crate::delay_ms(500);
        for &(note, dur) in handler.tune() {
            if !is_configured() {
                break;
            }
            send(&note_on_packet(0, note, 64));
            crate::delay_ms(dur);
            send(&note_off_packet(0, note));
            crate::delay_ms(50);
        }
        crate::delay_ms(1000);
    }
}

// ---------------------------------------------------------------------------
// Class driver
// ---------------------------------------------------------------------------

pub struct MidiClass;

/// Recommended USB peripheral / FIFO configuration for MIDI.
pub fn usb_config() -> UsbConfig {
    UsbConfig {
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
    }
}

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
