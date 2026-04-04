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
use hal::clocks::enable_gpio_clock;
use hal::gpio::GPIOExt;
use hal::watchdog::WatchdogExt;
use portable_atomic::{AtomicBool, Ordering};
use slstk3400a::SlStk3400a;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const USB_BASE: u32 = 0x400C_4000;

/// DWC2 DFIFO addresses – one per endpoint, 0x1000 apart from OTG base.
const EP0_FIFO: u32 = USB_BASE + 0x3D000;
const EP1_FIFO: u32 = USB_BASE + 0x3E000;
#[allow(dead_code)]
const EP2_FIFO: u32 = USB_BASE + 0x3F000;

const EP1_MPS: u16 = 64; // Bulk max packet size (full speed)
const EP2_MPS: u16 = 8; // Interrupt notification endpoint

// ---------------------------------------------------------------------------
// Volatile FIFO helpers
// ---------------------------------------------------------------------------

/// Read one 32-bit word from a FIFO address using a volatile read.
#[inline]
fn fifo_read(addr: u32) -> u32 {
    // SAFETY: FIFO addresses are valid MMIO registers within the USB peripheral.
    unsafe { core::ptr::read_volatile(addr as *const u32) }
}

/// Write one 32-bit word to a FIFO address using a volatile write.
#[inline]
fn fifo_write(addr: u32, value: u32) {
    // SAFETY: FIFO addresses are valid MMIO registers within the USB peripheral.
    unsafe { core::ptr::write_volatile(addr as *mut u32, value) }
}

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

/// String descriptor 0 – language ID (English US).
#[rustfmt::skip]
static STRING0: [u8; 4] = [4, 0x03, 0x09, 0x04];

/// Encode a short ASCII string as a UTF-16LE USB string descriptor at compile
/// time.  Maximum 31 characters (descriptor length fits in u8).
macro_rules! usb_string {
    ($s:expr) => {{
        const N: usize = $s.len();
        const LEN: usize = 2 + N * 2;
        const fn make() -> [u8; LEN] {
            let mut buf = [0u8; LEN];
            buf[0] = LEN as u8;
            buf[1] = 0x03; // bDescriptorType = String
            let bytes = $s.as_bytes();
            let mut i = 0;
            while i < N {
                buf[2 + i * 2] = bytes[i];
                buf[2 + i * 2 + 1] = 0;
                i += 1;
            }
            buf
        }
        make()
    }};
}

static STRING1: [u8; 2 + 10 * 2] = usb_string!("SLSTK3400A");
static STRING2: [u8; 2 + 16 * 2] = usb_string!("EFM32 USB Serial");

// ---- CDC Line Coding (7 bytes, stored in RAM so the host can SET it) ----
// Default: 115200 baud, 1 stop bit, no parity, 8 data bits.
static LINE_CODING: Mutex<RefCell<[u8; 7]>> = Mutex::new(RefCell::new([
    0x00, 0xC2, 0x01, 0x00, // dwDTERate = 115200 (little-endian)
    0x00, // bCharFormat = 1 stop bit
    0x00, // bParityType = none
    0x08, // bDataBits   = 8
]));

// ---------------------------------------------------------------------------
// FIFO helpers
// ---------------------------------------------------------------------------

/// Write `data` (up to `max_len` bytes) into EP0 IN FIFO and arm the transfer.
fn ep0_write(usb: &pac::usb::RegisterBlock, data: &[u8], max_len: usize) {
    let len = data.len().min(max_len);
    usb.diep0tsiz()
        .write(|w: &mut pac::usb::diep0tsiz::W| unsafe {
            w.xfersize().bits(len as u8).pktcnt().bits(1)
        });
    usb.diep0ctl()
        .modify(|_, w: &mut pac::usb::diep0ctl::W| w.epena().set_bit().cnak().set_bit());
    write_fifo(EP0_FIFO, data, len);
}

/// Write `data` into EP1 IN FIFO (bulk data, device→host).
fn ep1_write(usb: &pac::usb::RegisterBlock, data: &[u8]) {
    let len = data.len().min(EP1_MPS as usize);
    usb.diep0_tsiz()
        .write(|w: &mut pac::usb::diep0_tsiz::W| unsafe {
            w.xfersize().bits(len as u32).pktcnt().bits(1)
        });
    usb.diep0_ctl()
        .modify(|_, w: &mut pac::usb::diep0_ctl::W| w.cnak().set_bit().epena().set_bit());
    write_fifo(EP1_FIFO, data, len);
}

/// Push `len` bytes of `data` into a DWC2 FIFO at `addr` (32-bit writes).
fn write_fifo(addr: u32, data: &[u8], len: usize) {
    let mut i = 0;
    while i < len {
        let mut word = 0u32;
        for b in 0..4 {
            if i + b < len {
                word |= (data[i + b] as u32) << (b * 8);
            }
        }
        fifo_write(addr, word);
        i += 4;
    }
}

/// Prepare EP0 OUT to receive the next SETUP / data packet.
fn ep0_prepare_out(usb: &pac::usb::RegisterBlock) {
    usb.doep0tsiz()
        .write(|w: &mut pac::usb::doep0tsiz::W| unsafe {
            w.supcnt().bits(3).pktcnt().set_bit().xfersize().bits(64)
        });
    usb.doep0ctl()
        .modify(|_, w: &mut pac::usb::doep0ctl::W| w.epena().set_bit().cnak().set_bit());
}

/// Arm EP1 OUT to receive up to EP1_MPS bytes from the host.
fn ep1_prepare_out(usb: &pac::usb::RegisterBlock) {
    usb.doep0_tsiz()
        .write(|w: &mut pac::usb::doep0_tsiz::W| unsafe {
            w.xfersize().bits(EP1_MPS as u32).pktcnt().bits(1)
        });
    usb.doep0_ctl()
        .modify(|_, w: &mut pac::usb::doep0_ctl::W| w.epena().set_bit().cnak().set_bit());
}

// ---------------------------------------------------------------------------
// Global state
// ---------------------------------------------------------------------------

static BOARD: Mutex<RefCell<Option<SlStk3400a>>> = Mutex::new(RefCell::new(None));
static PENDING_ADDRESS: Mutex<RefCell<u8>> = Mutex::new(RefCell::new(0));

/// Small buffer used to bounce EP0 OUT data (e.g. SET_LINE_CODING payload).
static EP0_OUT_BUF: Mutex<RefCell<[u8; 64]>> = Mutex::new(RefCell::new([0u8; 64]));
static EP0_OUT_LEN: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));

/// Flag: the last class-specific OUT on EP0 was SET_LINE_CODING.
static EXPECT_LINE_CODING: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

/// LED0 toggle state for received data.
static RX_TOGGLE: AtomicBool = AtomicBool::new(false);

// ---------------------------------------------------------------------------
// USB interrupt handler
// ---------------------------------------------------------------------------

#[interrupt]
fn USB() {
    let usb = unsafe { &*pac::Usb::ptr() };
    let gintsts = usb.gintsts().read();

    // ---- USB Bus Reset ----
    if gintsts.usbrst().bit_is_set() {
        usb.gintsts()
            .write(|w: &mut pac::usb::gintsts::W| w.usbrst().set_bit());
        handle_usb_reset(usb);
    }

    // ---- Enumeration Done ----
    if gintsts.enumdone().bit_is_set() {
        usb.gintsts()
            .write(|w: &mut pac::usb::gintsts::W| w.enumdone().set_bit());
        usb.gusbcfg()
            .modify(|_, w: &mut pac::usb::gusbcfg::W| unsafe { w.usbtrdtim().bits(5) });
        defmt::info!("Enumeration done");
    }

    // ---- RX FIFO Non-Empty ----
    if gintsts.rxflvl().bit_is_set() {
        handle_rxflvl(usb);
    }

    // ---- IN Endpoint Interrupts ----
    if gintsts.iepint().bit_is_set() {
        handle_iepint(usb);
    }

    // ---- OUT Endpoint Interrupts ----
    if gintsts.oepint().bit_is_set() {
        handle_oepint(usb);
    }
}

// ---------------------------------------------------------------------------
// Interrupt sub-handlers
// ---------------------------------------------------------------------------

fn handle_usb_reset(usb: &pac::usb::RegisterBlock) {
    defmt::info!("USB reset");

    usb.dcfg()
        .modify(|_, w: &mut pac::usb::dcfg::W| unsafe { w.devaddr().bits(0) });

    // Flush FIFOs.
    usb.grstctl().write(|w: &mut pac::usb::grstctl::W| {
        w.txfflsh().set_bit().txfnum().fall().rxfflsh().set_bit()
    });
    while usb.grstctl().read().txfflsh().bit_is_set() || usb.grstctl().read().rxfflsh().bit_is_set()
    {
    }

    // Configure EP0 (64-byte MPS).
    usb.diep0ctl()
        .write(|w: &mut pac::usb::diep0ctl::W| w.mps()._64b().snak().set_bit());
    usb.doep0ctl()
        .write(|w: &mut pac::usb::doep0ctl::W| w.snak().set_bit());

    // Activate EP1 IN (bulk, 64-byte MPS, TXFIFO 1).
    usb.diep0_ctl()
        .write(|w: &mut pac::usb::diep0_ctl::W| unsafe {
            w.mps()
                .bits(EP1_MPS)
                .eptype()
                .bulk()
                .usbactep()
                .set_bit()
                .txfnum()
                .bits(1)
                .snak()
                .set_bit()
        });
    // Activate EP1 OUT (bulk, 64-byte MPS).
    usb.doep0_ctl()
        .write(|w: &mut pac::usb::doep0_ctl::W| unsafe {
            w.mps()
                .bits(EP1_MPS)
                .eptype()
                .bulk()
                .usbactep()
                .set_bit()
                .snak()
                .set_bit()
        });

    // Activate EP2 IN (interrupt, 8-byte MPS, TXFIFO 2).
    usb.diep1_ctl()
        .write(|w: &mut pac::usb::diep1_ctl::W| unsafe {
            w.mps()
                .bits(EP2_MPS)
                .eptype()
                .int()
                .usbactep()
                .set_bit()
                .txfnum()
                .bits(2)
                .snak()
                .set_bit()
        });

    // Unmask EP0-2 IN + EP0-1 OUT.
    // DAINTMSK: bits 0-2 = IN EPs, bits 16-17 = OUT EPs.
    usb.daintmsk()
        .write(|w: &mut pac::usb::daintmsk::W| unsafe { w.bits(0x0003_0007) });
    usb.diepmsk()
        .write(|w: &mut pac::usb::diepmsk::W| w.xfercomplmsk().set_bit());
    usb.doepmsk()
        .write(|w: &mut pac::usb::doepmsk::W| w.xfercomplmsk().set_bit().setupmsk().set_bit());

    // Arm EP0 OUT for SETUP.
    ep0_prepare_out(usb);

    critical_section::with(|lock| {
        if let Some(board) = BOARD.borrow(lock).borrow_mut().deref_mut() {
            board.leds_mut()[0].off();
            board.leds_mut()[1].off();
        }
    });
}

fn handle_rxflvl(usb: &pac::usb::RegisterBlock) {
    let grxstsp = usb.grxstsp().read().bits();
    let epnum = (grxstsp & 0xF) as u8;
    let bcnt = ((grxstsp >> 4) & 0x7FF) as usize;
    let pktsts = (grxstsp >> 17) & 0xF;

    match (epnum, pktsts) {
        // ---- EP0 SETUP data received ----
        (0, 0x6) => {
            let w0 = fifo_read(EP0_FIFO);
            let w1 = fifo_read(EP0_FIFO);

            let bm_request_type = (w0 & 0xFF) as u8;
            let b_request = ((w0 >> 8) & 0xFF) as u8;
            let w_value = ((w0 >> 16) & 0xFFFF) as u16;
            let w_index = (w1 & 0xFFFF) as u16;
            let w_length = ((w1 >> 16) & 0xFFFF) as u16;

            defmt::debug!(
                "SETUP: type={:02x} req={:02x} val={:04x} idx={:04x} len={}",
                bm_request_type,
                b_request,
                w_value,
                w_index,
                w_length,
            );

            handle_setup(usb, bm_request_type, b_request, w_value, w_index, w_length);
        }
        // EP0 SETUP complete – re-arm.
        (0, 0x4) => ep0_prepare_out(usb),

        // ---- EP0 OUT data packet (e.g. SET_LINE_CODING payload) ----
        (0, 0x2) => {
            critical_section::with(|lock| {
                let mut buf = EP0_OUT_BUF.borrow(lock).borrow_mut();
                let mut off = 0usize;
                for _ in 0..bcnt.div_ceil(4) {
                    let w = fifo_read(EP0_FIFO);
                    for &b in w.to_le_bytes().iter() {
                        if off < bcnt && off < buf.len() {
                            buf[off] = b;
                            off += 1;
                        }
                    }
                }
                *EP0_OUT_LEN.borrow(lock).borrow_mut() = bcnt;
            });
        }
        // EP0 OUT transfer complete.
        (0, 0x3) => {}

        // ---- EP1 OUT data packet (bulk serial data from host) ----
        (1, 0x2) => {
            let mut buf = [0u8; 64];
            let mut off = 0usize;
            for _ in 0..bcnt.div_ceil(4) {
                let w = fifo_read(EP0_FIFO); // RX FIFO read port is shared
                for &b in w.to_le_bytes().iter() {
                    if off < bcnt {
                        buf[off] = b;
                        off += 1;
                    }
                }
            }

            defmt::debug!("RX {} bytes", bcnt);

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

            // Echo received data back to the host.
            if bcnt > 0 {
                ep1_write(usb, &buf[..bcnt]);
            }
        }
        // EP1 OUT transfer complete.
        (1, 0x3) => {}

        // Drain unexpected data.
        (_, 0x2) => {
            for _ in 0..bcnt.div_ceil(4) {
                fifo_read(EP0_FIFO);
            }
        }
        _ => {}
    }
}

fn handle_iepint(usb: &pac::usb::RegisterBlock) {
    // EP0 IN complete.
    let diep0int = usb.diep0int().read();
    if diep0int.xfercompl().bit_is_set() {
        usb.diep0int()
            .write(|w: &mut pac::usb::diep0int::W| w.xfercompl().set_bit());

        // Apply deferred SET_ADDRESS.
        let addr = critical_section::with(|lock| {
            let mut a = PENDING_ADDRESS.borrow(lock).borrow_mut();
            let v = *a;
            *a = 0;
            v
        });
        if addr != 0 {
            usb.dcfg()
                .modify(|_, w: &mut pac::usb::dcfg::W| unsafe { w.devaddr().bits(addr & 0x7F) });
            defmt::info!("Address set to {}", addr);
        }

        ep0_prepare_out(usb);
    }

    // EP1 IN complete (bulk TX done) – re-arm EP1 OUT for next host data.
    let diep1int = usb.diep0_int().read();
    if diep1int.xfercompl().bit_is_set() {
        usb.diep0_int()
            .write(|w: &mut pac::usb::diep0_int::W| w.xfercompl().set_bit());
        ep1_prepare_out(usb);
    }

    // EP2 IN complete – nothing to do.
    let diep2int = usb.diep1_int().read();
    if diep2int.xfercompl().bit_is_set() {
        usb.diep1_int()
            .write(|w: &mut pac::usb::diep1_int::W| w.xfercompl().set_bit());
    }
}

fn handle_oepint(usb: &pac::usb::RegisterBlock) {
    // EP0 OUT complete.
    let doep0int = usb.doep0int().read();
    if doep0int.xfercompl().bit_is_set() {
        usb.doep0int()
            .write(|w: &mut pac::usb::doep0int::W| w.xfercompl().set_bit());

        // Check if this was a SET_LINE_CODING data stage.
        let is_line_coding = critical_section::with(|lock| {
            let mut flag = EXPECT_LINE_CODING.borrow(lock).borrow_mut();
            let v = *flag;
            *flag = false;
            v
        });

        if is_line_coding {
            critical_section::with(|lock| {
                let out_buf = EP0_OUT_BUF.borrow(lock).borrow();
                let out_len = *EP0_OUT_LEN.borrow(lock).borrow();
                if out_len >= 7 {
                    let mut lc = LINE_CODING.borrow(lock).borrow_mut();
                    lc.copy_from_slice(&out_buf[..7]);
                    defmt::info!("Line coding updated");
                }
            });
            // Acknowledge with zero-length IN.
            ep0_write(usb, &[], 0);
        } else {
            ep0_prepare_out(usb);
        }
    }

    // EP1 OUT complete.
    let doep1int = usb.doep0_int().read();
    if doep1int.xfercompl().bit_is_set() {
        usb.doep0_int()
            .write(|w: &mut pac::usb::doep0_int::W| w.xfercompl().set_bit());
        // Re-arm for next bulk OUT packet.
        ep1_prepare_out(usb);
    }
}

// ---------------------------------------------------------------------------
// SETUP request handling
// ---------------------------------------------------------------------------

fn handle_setup(
    usb: &pac::usb::RegisterBlock,
    bm_request_type: u8,
    b_request: u8,
    w_value: u16,
    w_index: u16,
    w_length: u16,
) {
    const GET_STATUS: u8 = 0x00;
    const SET_ADDRESS: u8 = 0x05;
    const GET_DESCRIPTOR: u8 = 0x06;
    const SET_CONFIGURATION: u8 = 0x09;

    const SET_LINE_CODING: u8 = 0x20;
    const GET_LINE_CODING: u8 = 0x21;
    const SET_CONTROL_LINE_STATE: u8 = 0x22;

    const DESC_DEVICE: u8 = 0x01;
    const DESC_CONFIGURATION: u8 = 0x02;
    const DESC_STRING: u8 = 0x03;

    match (bm_request_type, b_request) {
        // ---- Standard device requests ----

        // GET_STATUS (device).
        (0x80, GET_STATUS) => {
            let status: [u8; 2] = [0x00, 0x00]; // bus-powered, no remote wakeup
            ep0_write(usb, &status, w_length as usize);
        }

        // GET_DESCRIPTOR.
        (0x80, GET_DESCRIPTOR) => {
            let desc_type = (w_value >> 8) as u8;
            let desc_index = (w_value & 0xFF) as u8;
            match desc_type {
                DESC_DEVICE => {
                    defmt::info!("GET_DESCRIPTOR Device");
                    ep0_write(usb, &DEVICE_DESC, w_length as usize);
                }
                DESC_CONFIGURATION => {
                    defmt::info!("GET_DESCRIPTOR Configuration");
                    ep0_write(usb, &CONFIG_DESC, w_length as usize);
                }
                DESC_STRING => {
                    let desc: &[u8] = match desc_index {
                        0 => &STRING0,
                        1 => &STRING1,
                        2 => &STRING2,
                        _ => {
                            stall_ep0(usb);
                            return;
                        }
                    };
                    ep0_write(usb, desc, w_length as usize);
                }
                _ => {
                    defmt::warn!("Unsupported descriptor type {}", desc_type);
                    stall_ep0(usb);
                }
            }
        }

        // SET_ADDRESS.
        (0x00, SET_ADDRESS) => {
            let addr = (w_value & 0x7F) as u8;
            defmt::info!("SET_ADDRESS {}", addr);
            critical_section::with(|lock| {
                *PENDING_ADDRESS.borrow(lock).borrow_mut() = addr;
            });
            ep0_write(usb, &[], 0);
        }

        // SET_CONFIGURATION.
        (0x00, SET_CONFIGURATION) => {
            defmt::info!("SET_CONFIGURATION {}", w_value);
            // Activate data endpoints.
            ep1_prepare_out(usb);
            ep0_write(usb, &[], 0);
        }

        // GET_STATUS (interface / endpoint).
        (0x81, GET_STATUS) | (0x82, GET_STATUS) => {
            let status: [u8; 2] = [0x00, 0x00];
            ep0_write(usb, &status, w_length as usize);
        }

        // ---- CDC class requests (host→device, class, interface) ----

        // SET_LINE_CODING – host sends 7-byte payload in the DATA stage.
        (0x21, SET_LINE_CODING) => {
            defmt::info!("SET_LINE_CODING (waiting for data)");
            critical_section::with(|lock| {
                *EXPECT_LINE_CODING.borrow(lock).borrow_mut() = true;
            });
            // The data arrives in the next OUT transaction; status IN is sent
            // after we process it in handle_oepint.
        }

        // GET_LINE_CODING – return current line coding.
        (0xA1, GET_LINE_CODING) => {
            defmt::info!("GET_LINE_CODING");
            critical_section::with(|lock| {
                let lc = LINE_CODING.borrow(lock).borrow();
                ep0_write(usb, &*lc, w_length as usize);
            });
        }

        // SET_CONTROL_LINE_STATE – DTR / RTS signals from host.
        (0x21, SET_CONTROL_LINE_STATE) => {
            let dtr = w_value & 0x01 != 0;
            defmt::info!("SET_CONTROL_LINE_STATE DTR={}", dtr);
            // Light LED 1 when the host opens the serial port (DTR asserted).
            critical_section::with(|lock| {
                if let Some(board) = BOARD.borrow(lock).borrow_mut().deref_mut() {
                    if dtr {
                        board.leds_mut()[1].on();
                    } else {
                        board.leds_mut()[1].off();
                    }
                }
            });
            ep0_write(usb, &[], 0);
        }

        _ => {
            defmt::warn!(
                "Unhandled SETUP: type={:02x} req={:02x} val={:04x} idx={:04x}",
                bm_request_type,
                b_request,
                w_value,
                w_index,
            );
            stall_ep0(usb);
        }
    }
}

fn stall_ep0(usb: &pac::usb::RegisterBlock) {
    usb.diep0ctl()
        .modify(|_, w: &mut pac::usb::diep0ctl::W| w.stall().set_bit());
    usb.doep0ctl()
        .modify(|_, w: &mut pac::usb::doep0ctl::W| w.stall().set_bit());
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

    // ---- Enable USHFRCO at 48 MHz for USB PHY ----
    p.cmu
        .ushfrcoconf()
        .write(|w: &mut pac::cmu::ushfrcoconf::W| w.band()._48mhz());
    p.cmu
        .oscencmd()
        .write(|w: &mut pac::cmu::oscencmd::W| w.ushfrcoen().set_bit());
    while p.cmu.status().read().ushfrcordy().bit_is_clear() {}

    p.cmu
        .cmd()
        .write(|w: &mut pac::cmu::cmd::W| w.usbcclksel().ushfrco());
    p.cmu
        .hfcoreclken0()
        .modify(|_, w: &mut pac::cmu::hfcoreclken0::W| w.usbc().set_bit().usb().set_bit());

    let usb = &p.usb;

    // ---- Core initialisation ----
    usb.route()
        .write(|w: &mut pac::usb::route::W| w.phypen().set_bit());
    usb.gahbcfg()
        .write(|w: &mut pac::usb::gahbcfg::W| w.glblintrmsk().set_bit());
    usb.gusbcfg()
        .write(|w: &mut pac::usb::gusbcfg::W| unsafe { w.usbtrdtim().bits(5) });

    // Soft reset.
    usb.grstctl()
        .write(|w: &mut pac::usb::grstctl::W| w.csftrst().set_bit());
    while usb.grstctl().read().csftrst().bit_is_set() {}

    usb.gahbcfg()
        .write(|w: &mut pac::usb::gahbcfg::W| w.glblintrmsk().set_bit());
    usb.dcfg()
        .write(|w: &mut pac::usb::dcfg::W| unsafe { w.devspd().fs().devaddr().bits(0) });

    // ---- FIFO allocation (in 32-bit words) ----
    //   RX FIFO          : 64 words  @  0
    //   TX FIFO 0 (EP0)  : 16 words  @ 64
    //   TX FIFO 1 (EP1)  : 64 words  @ 80
    //   TX FIFO 2 (EP2)  : 16 words  @144
    usb.grxfsiz()
        .write(|w: &mut pac::usb::grxfsiz::W| unsafe { w.rxfdep().bits(64) });
    usb.gnptxfsiz()
        .write(|w: &mut pac::usb::gnptxfsiz::W| unsafe {
            w.nptxfstaddr().bits(64).nptxfineptxf0dep().bits(16)
        });
    usb.dieptxf1()
        .write(|w: &mut pac::usb::dieptxf1::W| unsafe {
            w.inepntxfstaddr().bits(80).inepntxfdep().bits(64)
        });
    usb.dieptxf2()
        .write(|w: &mut pac::usb::dieptxf2::W| unsafe {
            w.inepntxfstaddr().bits(144).inepntxfdep().bits(16)
        });

    // ---- Unmask interrupts ----
    usb.gintmsk().write(|w: &mut pac::usb::gintmsk::W| {
        w.usbrstmsk()
            .set_bit()
            .enumdonemsk()
            .set_bit()
            .iepintmsk()
            .set_bit()
            .oepintmsk()
            .set_bit()
            .rxflvlmsk()
            .set_bit()
    });

    usb.gintsts()
        .write(|w: &mut pac::usb::gintsts::W| unsafe { w.bits(0xFFFF_FFFF) });

    // Connect.
    usb.dctl()
        .modify(|_, w: &mut pac::usb::dctl::W| w.sftdiscon().clear_bit());
    usb.dctl()
        .modify(|_, w: &mut pac::usb::dctl::W| w.pwronprgdone().set_bit());

    pac::NVIC::unpend(pac::Interrupt::USB);
    unsafe { pac::NVIC::unmask(pac::Interrupt::USB) };

    defmt::info!("USB CDC ACM serial port ready");

    loop {
        cortex_m::asm::wfi();
    }
}
