//! DWC2 USB device driver for EFM32HG322.
//!
//! Provides a generic `UsbDevice<C>` that handles standard USB requests
//! (enumeration, descriptors, address, configuration) and delegates
//! class-specific behaviour to a [`UsbClass`] implementation.

#![no_std]

pub mod bus;
pub mod cdc_acm;
pub mod hid_keyboard;
pub mod midi;
pub mod msc;

pub use bus::UsbBus;

use efm32hg322_pac as pac;
use pac::interrupt;
use portable_atomic::{AtomicUsize, Ordering};

// ---------------------------------------------------------------------------
// Utilities
// ---------------------------------------------------------------------------

/// Busy-wait delay assuming the 14 MHz HFRCO default.
pub fn delay_ms(ms: u32) {
    cortex_m::asm::delay(ms * 14_000);
}

/// Sleep forever, handling all USB activity in interrupt callbacks.
pub fn idle() -> ! {
    loop {
        cortex_m::asm::wfi();
    }
}

// ---------------------------------------------------------------------------
// USB interrupt handler (lives in the library, dispatches via function pointer)
// ---------------------------------------------------------------------------

static POLL_FN: AtomicUsize = AtomicUsize::new(0);

#[interrupt]
fn USB() {
    let f = POLL_FN.load(Ordering::Relaxed);
    if f != 0 {
        let poll: fn() = unsafe { core::mem::transmute(f) };
        poll();
    }
}

// ---------------------------------------------------------------------------
// Macro support
// ---------------------------------------------------------------------------

/// Re-exports used by [`usb_device!`]. Not part of the public API.
#[doc(hidden)]
pub mod __private {
    pub use critical_section::{with, Mutex};

    pub fn register_poll(f: fn()) {
        super::POLL_FN.store(f as usize, super::Ordering::Release);
    }

    pub fn unpend_and_unmask() {
        use efm32hg322_pac as pac;
        pac::NVIC::unpend(pac::Interrupt::USB);
        unsafe { pac::NVIC::unmask(pac::Interrupt::USB) };
    }
}

/// Define the USB device global and helper functions.
///
/// Expands to:
/// - `static USB_DEV` - global device instance behind a `Mutex<RefCell<…>>`
/// - `fn usb_start(dev)` - stores the device and enables the USB interrupt
/// - `fn usb_with_bus(f)` - runs a closure with `&UsbBus` in a critical section
#[macro_export]
macro_rules! usb_device {
    ($class:ty) => {
        static USB_DEV: $crate::__private::Mutex<
            core::cell::RefCell<Option<$crate::UsbDevice<$class>>>,
        > = $crate::__private::Mutex::new(core::cell::RefCell::new(None));

        fn usb_poll() {
            $crate::__private::with(|lock| {
                use core::ops::DerefMut;
                if let Some(dev) = USB_DEV.borrow(lock).borrow_mut().deref_mut() {
                    dev.poll();
                }
            });
        }

        fn usb_start(dev: $crate::UsbDevice<$class>) {
            $crate::__private::with(|lock| {
                USB_DEV.borrow(lock).replace(Some(dev));
            });
            $crate::__private::register_poll(usb_poll);
            $crate::__private::unpend_and_unmask();
        }

        #[allow(dead_code)]
        fn usb_with_bus(f: impl FnOnce(&$crate::UsbBus)) {
            $crate::__private::with(|lock| {
                use core::ops::DerefMut;
                if let Some(dev) = USB_DEV.borrow(lock).borrow_mut().deref_mut() {
                    f(dev.bus());
                }
            });
        }
    };
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// 8-byte USB SETUP packet parsed from the RX FIFO.
#[derive(Clone, Copy, Debug, defmt::Format)]
pub struct SetupPacket {
    pub bm_request_type: u8,
    pub b_request: u8,
    pub w_value: u16,
    pub w_index: u16,
    pub w_length: u16,
}

/// Result returned by [`UsbClass::handle_setup`].
pub enum SetupResult {
    /// Request fully handled; library sends a ZLP status stage.
    Handled,
    /// Class already wrote a response via [`UsbBus::ep0_write`].
    DataIn,
    /// Class expects a DATA OUT stage; library buffers it and calls
    /// [`UsbClass::ep0_data_out`] when complete.
    DataOut,
    /// Request not recognized - library STALLs EP0.
    Unhandled,
}

/// Endpoint transfer type.
#[derive(Clone, Copy)]
pub enum EpType {
    Bulk,
    Interrupt,
}

/// Configuration for a non-EP0 endpoint index (1 or 2).
#[derive(Clone, Copy)]
pub struct EpConfig {
    pub ep_type: EpType,
    pub mps: u16,
    pub has_in: bool,
    pub has_out: bool,
}

/// USB peripheral and FIFO configuration.
pub struct UsbConfig {
    /// RX FIFO depth in 32-bit words.
    pub rx_fifo_words: u16,
    /// TX FIFO 0 (EP0 IN) depth in 32-bit words.
    pub tx0_fifo_words: u16,
    /// TX FIFO 1 (EP1 IN) depth in 32-bit words.
    pub tx1_fifo_words: u16,
    /// TX FIFO 2 (EP2 IN) depth in 32-bit words.
    pub tx2_fifo_words: u16,
    /// EP1 configuration (`None` = endpoint not used).
    pub ep1: Option<EpConfig>,
    /// EP2 configuration (`None` = endpoint not used).
    pub ep2: Option<EpConfig>,
}

// ---------------------------------------------------------------------------
// UsbClass trait
// ---------------------------------------------------------------------------

/// Trait implemented by USB class drivers (CDC ACM, MIDI, HID, …).
pub trait UsbClass {
    /// Return the device descriptor (18 bytes).
    fn device_descriptor(&self) -> &[u8];
    /// Return the full configuration descriptor bundle.
    fn config_descriptor(&self) -> &[u8];
    /// Return a USB string descriptor for `index` (1-based). Return `None` to STALL.
    fn string_descriptor(&self, index: u8) -> Option<&[u8]>;

    /// Handle a class-specific or vendor SETUP request.
    fn handle_setup(&mut self, setup: &SetupPacket, usb: &UsbBus) -> SetupResult;

    /// Called when the EP0 DATA OUT stage completes (e.g. SET_LINE_CODING payload).
    fn ep0_data_out(&mut self, _data: &[u8], _usb: &UsbBus) {}
    /// Called when data is received on a non-EP0 OUT endpoint.
    fn data_out(&mut self, _ep: u8, _data: &[u8], _usb: &UsbBus) {}
    /// Called when an IN transfer completes on a non-EP0 endpoint.
    fn in_complete(&mut self, _ep: u8, _usb: &UsbBus) {}
    /// Called after SET_CONFIGURATION succeeds.
    fn configured(&mut self, _usb: &UsbBus) {}
    /// Called on bus reset.
    fn reset(&mut self) {}
    /// Called on bus suspend.
    fn suspended(&mut self) {}
}

// ---------------------------------------------------------------------------
// usb_string! macro
// ---------------------------------------------------------------------------

/// Encode a short ASCII string as a UTF-16LE USB string descriptor at compile time.
/// Maximum 31 characters (descriptor length fits in `u8`).
#[macro_export]
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

/// String descriptor 0 - language ID (English US).
static STRING0: [u8; 4] = [4, 0x03, 0x09, 0x04];

// ---------------------------------------------------------------------------
// UsbDevice
// ---------------------------------------------------------------------------

/// Core USB device state machine, generic over a [`UsbClass`] implementation.
pub struct UsbDevice<C: UsbClass> {
    bus: UsbBus,
    /// The class driver instance. Public so demos can inspect class state.
    pub class: C,
    config: UsbConfig,
    ep0_out_buf: [u8; 64],
    ep0_out_len: usize,
    pending_data_out: bool,
}

impl<C: UsbClass> UsbDevice<C> {
    /// Initialize USB clocks, the DWC2 peripheral, and connect to the bus.
    ///
    /// Call this from `main()` after disabling the watchdog and setting up
    /// GPIOs. The returned `UsbDevice` should be stored in a
    /// `Mutex<RefCell<Option<…>>>` and polled from the USB ISR via [`poll`].
    pub fn init(
        cmu: &pac::cmu::RegisterBlock,
        usb: &pac::usb::RegisterBlock,
        class: C,
        config: UsbConfig,
    ) -> Self {
        // ---- Clock setup (USHFRCO 48 MHz + USB clock recovery) ----
        cmu.hfcoreclken0()
            .modify(|_, w| w.usbc().set_bit().usb().set_bit().le().set_bit());
        cmu.lfclksel().modify(|_, w| w.lfc().lfrco());
        cmu.lfcclken0().modify(|_, w| w.usble().set_bit());
        cmu.ushfrcoconf().write(|w| w.band()._48mhz());
        cmu.usbcrctrl().modify(|_, w| w.en().set_bit());
        cmu.oscencmd().write(|w| w.ushfrcoen().set_bit());
        while cmu.status().read().ushfrcordy().bit_is_clear() {}
        cmu.cmd().write(|w| w.usbcclksel().ushfrco());
        while cmu.status().read().usbcushfrcosel().bit_is_clear() {}

        // ---- USB peripheral init ----

        // Disable LEM oscillator control during init.
        usb.ctrl().modify(|_, w| w.lemoscctrl().none());
        // PHY pins + D+ pull-up (signals device presence to host).
        usb.route()
            .write(|w| w.phypen().set_bit().dmpupen().set_bit());
        // Power/clock gating off.
        usb.pcgcctl().write(|w| unsafe { w.bits(0) });

        // Soft-reset.
        while usb.grstctl().read().ahbidle().bit_is_clear() {}
        usb.grstctl()
            .modify(|_, w| w.csftrst().set_bit());
        while usb.grstctl().read().csftrst().bit_is_set() {}
        while usb.grstctl().read().ahbidle().bit_is_clear() {}

        usb.gahbcfg().write(|w| w.glblintrmsk().set_bit());
        usb.dctl()
            .modify(|_, w| w.sftdiscon().set_bit());
        usb.dcfg()
            .write(|w| unsafe { w.devspd().fs().devaddr().bits(0) });

        // Endpoint interrupt masks.
        usb.diepmsk()
            .write(|w| w.xfercomplmsk().set_bit());
        usb.doepmsk()
            .write(|w| w.xfercomplmsk().set_bit().setupmsk().set_bit());

        // ---- FIFO allocation ----
        usb.grxfsiz()
            .write(|w| unsafe { w.rxfdep().bits(config.rx_fifo_words) });
        let tx0_start = config.rx_fifo_words;
        usb.gnptxfsiz().write(|w| unsafe {
            w.nptxfstaddr()
                .bits(tx0_start)
                .nptxfineptxf0dep()
                .bits(config.tx0_fifo_words)
        });
        let tx1_start = tx0_start + config.tx0_fifo_words;
        usb.dieptxf1().write(|w| unsafe {
            w.inepntxfstaddr()
                .bits(tx1_start)
                .inepntxfdep()
                .bits(config.tx1_fifo_words)
        });
        let tx2_start = tx1_start + config.tx1_fifo_words;
        usb.dieptxf2().write(|w| unsafe {
            w.inepntxfstaddr()
                .bits(tx2_start)
                .inepntxfdep()
                .bits(config.tx2_fifo_words)
        });

        // ---- Global interrupt mask ----
        usb.gintmsk().write(|w| {
            w.usbrstmsk()
                .set_bit()
                .enumdonemsk()
                .set_bit()
                .usbsuspmsk()
                .set_bit()
                .wkupintmsk()
                .set_bit()
                .iepintmsk()
                .set_bit()
                .oepintmsk()
                .set_bit()
                .rxflvlmsk()
                .set_bit()
        });

        // Clear all pending interrupts.
        usb.gintsts()
            .write(|w| unsafe { w.bits(0xFFFF_FFFF) });

        // Power-on programming done handshake.
        usb.dctl()
            .modify(|_, w| w.pwronprgdone().set_bit());
        cortex_m::asm::delay(800); // ~10 µs at 48 MHz
        usb.dctl()
            .modify(|_, w| w.pwronprgdone().clear_bit());

        // Connect (clear soft-disconnect).
        usb.dctl()
            .modify(|_, w| w.sftdiscon().clear_bit());

        Self {
            bus: UsbBus::new(),
            class,
            config,
            ep0_out_buf: [0u8; 64],
            ep0_out_len: 0,
            pending_data_out: false,
        }
    }

    /// Get a reference to the underlying [`UsbBus`] for direct endpoint
    /// operations (e.g. sending data from the main loop).
    pub fn bus(&self) -> &UsbBus {
        &self.bus
    }

    /// Poll for and handle USB interrupts. Call this from the `USB` ISR
    /// inside a critical section.
    pub fn poll(&mut self) {
        let gintsts = self.bus.regs().gintsts().read();

        if gintsts.usbrst().bit_is_set() {
            self.bus.regs().gintsts().write(|w| w.usbrst().set_bit());
            self.handle_usb_reset();
        }

        if gintsts.enumdone().bit_is_set() {
            let usb = self.bus.regs();
            usb.gintsts().write(|w| w.enumdone().set_bit());
            usb.gusbcfg()
                .modify(|_, w| unsafe { w.usbtrdtim().bits(5) });
            usb.diep0ctl().modify(|_, w| w.mps()._64b());
            // Clear Global Non-Periodic IN NAK set during bus reset.
            usb.dctl().modify(|_, w| w.cgnpinnak().set_bit());
            self.bus.ep0_prepare_out();
            defmt::info!("Speed negotiation complete");
        }

        if gintsts.usbsusp().bit_is_set() {
            self.bus.regs().gintsts().write(|w| w.usbsusp().set_bit());
            defmt::info!("USB suspended");
            self.class.suspended();
        }

        if gintsts.wkupint().bit_is_set() {
            self.bus.regs().gintsts().write(|w| w.wkupint().set_bit());
            defmt::info!("USB wakeup");
        }

        if gintsts.rxflvl().bit_is_set() {
            self.handle_rxflvl();
        }

        if gintsts.iepint().bit_is_set() {
            self.handle_iepint();
        }

        if gintsts.oepint().bit_is_set() {
            self.handle_oepint();
        }
    }

    // -----------------------------------------------------------------------
    // Internal handlers
    // -----------------------------------------------------------------------

    fn handle_usb_reset(&mut self) {
        defmt::info!("USB reset");
        let usb = self.bus.regs();

        usb.dcfg()
            .modify(|_, w| unsafe { w.devaddr().bits(0) });

        // Flush all FIFOs.
        usb.grstctl()
            .write(|w| w.txfflsh().set_bit().txfnum().fall().rxfflsh().set_bit());
        while usb.grstctl().read().txfflsh().bit_is_set()
            || usb.grstctl().read().rxfflsh().bit_is_set()
        {}

        // Configure EP0.
        usb.diep0ctl()
            .write(|w| w.mps()._64b().snak().set_bit());
        usb.doep0ctl().write(|w| w.snak().set_bit());

        // Activate class endpoints and set DAINTMSK.
        self.activate_endpoints();

        // EP0 OUT STUPCNT.
        usb.doep0tsiz()
            .write(|w| unsafe { w.supcnt().bits(3) });

        self.pending_data_out = false;
        self.class.reset();
    }

    fn activate_endpoints(&self) {
        let usb = self.bus.regs();
        // EP0 IN + OUT always enabled.
        let mut daintmsk: u32 = 0x0001_0001;

        if let Some(ref ep) = self.config.ep1 {
            if ep.has_in {
                daintmsk |= 1 << 1;
                usb.diep0_ctl().write(|w| unsafe {
                    let w = w
                        .mps()
                        .bits(ep.mps)
                        .usbactep()
                        .set_bit()
                        .txfnum()
                        .bits(1)
                        .snak()
                        .set_bit();
                    match ep.ep_type {
                        EpType::Bulk => w.eptype().bulk(),
                        EpType::Interrupt => w.eptype().int(),
                    }
                });
            }
            if ep.has_out {
                daintmsk |= 1 << 17;
                usb.doep0_ctl().write(|w| unsafe {
                    let w = w
                        .mps()
                        .bits(ep.mps)
                        .usbactep()
                        .set_bit()
                        .snak()
                        .set_bit();
                    match ep.ep_type {
                        EpType::Bulk => w.eptype().bulk(),
                        EpType::Interrupt => w.eptype().int(),
                    }
                });
            }
        }

        if let Some(ref ep) = self.config.ep2 {
            if ep.has_in {
                daintmsk |= 1 << 2;
                usb.diep1_ctl().write(|w| unsafe {
                    let w = w
                        .mps()
                        .bits(ep.mps)
                        .usbactep()
                        .set_bit()
                        .txfnum()
                        .bits(2)
                        .snak()
                        .set_bit();
                    match ep.ep_type {
                        EpType::Bulk => w.eptype().bulk(),
                        EpType::Interrupt => w.eptype().int(),
                    }
                });
            }
            if ep.has_out {
                daintmsk |= 1 << 18;
                usb.doep1_ctl().write(|w| unsafe {
                    let w = w
                        .mps()
                        .bits(ep.mps)
                        .usbactep()
                        .set_bit()
                        .snak()
                        .set_bit();
                    match ep.ep_type {
                        EpType::Bulk => w.eptype().bulk(),
                        EpType::Interrupt => w.eptype().int(),
                    }
                });
            }
        }

        usb.daintmsk()
            .write(|w| unsafe { w.bits(daintmsk) });
        usb.diepmsk()
            .write(|w| w.xfercomplmsk().set_bit());
        usb.doepmsk()
            .write(|w| w.xfercomplmsk().set_bit().setupmsk().set_bit());
    }

    fn handle_rxflvl(&mut self) {
        loop {
            if !self.bus.regs().gintsts().read().rxflvl().bit_is_set() {
                break;
            }
            let grxstsp = self.bus.regs().grxstsp().read().bits();
            let epnum = (grxstsp & 0xF) as u8;
            let bcnt = ((grxstsp >> 4) & 0x7FF) as usize;
            let pktsts = (grxstsp >> 17) & 0xF;

            match (epnum, pktsts) {
                // EP0 SETUP data received.
                (0, 0x6) => {
                    self.bus.flush_ep0_tx_if_pending();
                    self.bus.clear_ep0_setup_int();

                    let w0 = bus::read_rx_word();
                    let w1 = bus::read_rx_word();

                    let setup = SetupPacket {
                        bm_request_type: (w0 & 0xFF) as u8,
                        b_request: ((w0 >> 8) & 0xFF) as u8,
                        w_value: ((w0 >> 16) & 0xFFFF) as u16,
                        w_index: (w1 & 0xFFFF) as u16,
                        w_length: ((w1 >> 16) & 0xFFFF) as u16,
                    };

                    defmt::debug!(
                        "SETUP: type={:02x} req={:02x} val={:04x} idx={:04x} len={}",
                        setup.bm_request_type,
                        setup.b_request,
                        setup.w_value,
                        setup.w_index,
                        setup.w_length,
                    );

                    self.handle_setup(setup);
                }
                // EP0 SETUP complete - re-arm.
                (0, 0x4) => self.bus.ep0_prepare_out(),

                // EP0 OUT data (e.g. SET_LINE_CODING payload).
                (0, 0x2) => {
                    self.ep0_out_len = bus::read_rx_fifo(&mut self.ep0_out_buf, bcnt);
                }
                // EP0 OUT transfer complete.
                (0, 0x3) => {}

                // EPn OUT data.
                (ep, 0x2) if ep > 0 => {
                    let mut buf = [0u8; 64];
                    let len = bus::read_rx_fifo(&mut buf, bcnt);
                    self.class.data_out(ep, &buf[..len], &self.bus);
                }
                // EPn OUT transfer complete.
                (_, 0x3) => {}

                // Drain unexpected data.
                (_, 0x2) => {
                    bus::drain_rx_fifo(bcnt);
                }
                _ => {}
            }
        }
    }

    fn handle_iepint(&mut self) {
        let usb = self.bus.regs();

        // EP0 IN - read and clear all pending bits.
        let diep0int = usb.diep0int().read();
        usb.diep0int()
            .write(|w| unsafe { w.bits(diep0int.bits()) });
        if diep0int.xfercompl().bit_is_set() {
            self.bus.ep0_prepare_out();
        }

        // EP1 IN.
        if self.config.ep1.as_ref().is_some_and(|e| e.has_in) {
            let int = usb.diep0_int().read();
            usb.diep0_int()
                .write(|w| unsafe { w.bits(int.bits()) });
            if int.xfercompl().bit_is_set() {
                self.class.in_complete(1, &self.bus);
            }
        }

        // EP2 IN.
        if self.config.ep2.as_ref().is_some_and(|e| e.has_in) {
            let int = usb.diep1_int().read();
            usb.diep1_int()
                .write(|w| unsafe { w.bits(int.bits()) });
            if int.xfercompl().bit_is_set() {
                self.class.in_complete(2, &self.bus);
            }
        }
    }

    fn handle_oepint(&mut self) {
        let usb = self.bus.regs();

        // EP0 OUT - read and clear all pending bits.
        let doep0int = usb.doep0int().read();
        usb.doep0int()
            .write(|w| unsafe { w.bits(doep0int.bits()) });
        if doep0int.xfercompl().bit_is_set() {
            if self.pending_data_out {
                self.pending_data_out = false;
                let len = self.ep0_out_len;
                let mut buf = [0u8; 64];
                buf[..len].copy_from_slice(&self.ep0_out_buf[..len]);
                self.class.ep0_data_out(&buf[..len], &self.bus);
                self.bus.ep0_write(&[], 0);
            } else {
                self.bus.ep0_prepare_out();
            }
        }

        // EP1 OUT.
        if self.config.ep1.as_ref().is_some_and(|e| e.has_out) {
            let int = usb.doep0_int().read();
            usb.doep0_int()
                .write(|w| unsafe { w.bits(int.bits()) });
            if int.xfercompl().bit_is_set() {
                if let Some(ref ep) = self.config.ep1 {
                    self.bus.ep_prepare_out(1, ep.mps);
                }
            }
        }

        // EP2 OUT.
        if self.config.ep2.as_ref().is_some_and(|e| e.has_out) {
            let int = usb.doep1_int().read();
            usb.doep1_int()
                .write(|w| unsafe { w.bits(int.bits()) });
            if int.xfercompl().bit_is_set() {
                if let Some(ref ep) = self.config.ep2 {
                    self.bus.ep_prepare_out(2, ep.mps);
                }
            }
        }
    }

    fn handle_setup(&mut self, setup: SetupPacket) {
        const GET_STATUS: u8 = 0x00;
        const SET_ADDRESS: u8 = 0x05;
        const GET_DESCRIPTOR: u8 = 0x06;
        const SET_CONFIGURATION: u8 = 0x09;

        const DESC_DEVICE: u8 = 0x01;
        const DESC_CONFIGURATION: u8 = 0x02;
        const DESC_STRING: u8 = 0x03;

        match (setup.bm_request_type, setup.b_request) {
            // ---- Standard device requests ----

            // GET_STATUS (device).
            (0x80, GET_STATUS) => {
                self.bus
                    .ep0_write(&[0x00, 0x00], setup.w_length as usize);
            }

            // GET_DESCRIPTOR.
            (0x80, GET_DESCRIPTOR) => {
                let desc_type = (setup.w_value >> 8) as u8;
                let desc_index = (setup.w_value & 0xFF) as u8;
                match desc_type {
                    DESC_DEVICE => {
                        defmt::info!("GET_DESCRIPTOR Device");
                        self.bus.ep0_write(
                            self.class.device_descriptor(),
                            setup.w_length as usize,
                        );
                    }
                    DESC_CONFIGURATION => {
                        defmt::info!("GET_DESCRIPTOR Configuration");
                        self.bus.ep0_write(
                            self.class.config_descriptor(),
                            setup.w_length as usize,
                        );
                    }
                    DESC_STRING => {
                        if desc_index == 0 {
                            self.bus.ep0_write(&STRING0, setup.w_length as usize);
                        } else if let Some(desc) =
                            self.class.string_descriptor(desc_index)
                        {
                            self.bus.ep0_write(desc, setup.w_length as usize);
                        } else {
                            self.bus.stall_ep0();
                        }
                    }
                    _ => {
                        defmt::warn!("Unsupported descriptor type {}", desc_type);
                        self.bus.stall_ep0();
                    }
                }
            }

            // SET_ADDRESS.
            (0x00, SET_ADDRESS) => {
                let addr = (setup.w_value & 0x7F) as u8;
                defmt::info!("SET_ADDRESS {}", addr);
                self.bus
                    .regs()
                    .dcfg()
                    .modify(|_, w| unsafe { w.devaddr().bits(addr) });
                self.bus.ep0_write(&[], 0);
            }

            // SET_CONFIGURATION.
            (0x00, SET_CONFIGURATION) => {
                defmt::info!("SET_CONFIGURATION {}", setup.w_value);
                // Arm configured OUT endpoints.
                if let Some(ref ep) = self.config.ep1 {
                    if ep.has_out {
                        self.bus.ep_prepare_out(1, ep.mps);
                    }
                }
                if let Some(ref ep) = self.config.ep2 {
                    if ep.has_out {
                        self.bus.ep_prepare_out(2, ep.mps);
                    }
                }
                // Enable Low Energy Mode.
                self.bus.regs().ctrl().modify(|_, w| {
                    w.lemoscctrl()
                        .gate()
                        .lemidleen()
                        .set_bit()
                        .lemphyctrl()
                        .set_bit()
                        .lemnaken()
                        .set_bit()
                        .lemaddrmen()
                        .set_bit()
                });
                self.bus.ep0_write(&[], 0);
                self.class.configured(&self.bus);
            }

            // GET_STATUS (interface / endpoint).
            (0x81, GET_STATUS) | (0x82, GET_STATUS) => {
                self.bus
                    .ep0_write(&[0x00, 0x00], setup.w_length as usize);
            }

            // ---- Delegate to class ----
            _ => match self.class.handle_setup(&setup, &self.bus) {
                SetupResult::Handled => {
                    self.bus.ep0_write(&[], 0);
                }
                SetupResult::DataIn => { /* class already sent response */ }
                SetupResult::DataOut => {
                    self.pending_data_out = true;
                }
                SetupResult::Unhandled => {
                    defmt::warn!(
                        "Unhandled SETUP: type={:02x} req={:02x} val={:04x} idx={:04x}",
                        setup.bm_request_type,
                        setup.b_request,
                        setup.w_value,
                        setup.w_index,
                    );
                    self.bus.stall_ep0();
                }
            },
        }
    }
}
