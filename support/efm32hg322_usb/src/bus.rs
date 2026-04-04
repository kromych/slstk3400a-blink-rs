use efm32hg322_pac as pac;

const USB_BASE: u32 = 0x400C_4000;

/// DWC2 FIFO base address for endpoint `ep`.
#[inline]
const fn fifo_addr(ep: u8) -> u32 {
    USB_BASE + 0x3D000 + (ep as u32) * 0x1000
}

#[inline]
fn fifo_read(addr: u32) -> u32 {
    unsafe { core::ptr::read_volatile(addr as *const u32) }
}

#[inline]
fn fifo_write(addr: u32, value: u32) {
    unsafe { core::ptr::write_volatile(addr as *mut u32, value) }
}

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

/// Read `bcnt` bytes from the shared RX FIFO into `buf`. Returns bytes read.
pub fn read_rx_fifo(buf: &mut [u8], bcnt: usize) -> usize {
    let addr = fifo_addr(0); // RX FIFO read port is always EP0 address
    let len = bcnt.min(buf.len());
    let mut off = 0usize;
    for _ in 0..bcnt.div_ceil(4) {
        let w = fifo_read(addr);
        for &b in w.to_le_bytes().iter() {
            if off < len {
                buf[off] = b;
            }
            off += 1;
        }
    }
    len
}

/// Drain `bcnt` bytes from the RX FIFO without storing.
pub fn drain_rx_fifo(bcnt: usize) {
    let addr = fifo_addr(0);
    for _ in 0..bcnt.div_ceil(4) {
        fifo_read(addr);
    }
}

/// Read a raw 32-bit word from the RX FIFO.
pub fn read_rx_word() -> u32 {
    fifo_read(fifo_addr(0))
}

/// Thin wrapper around `&pac::usb::RegisterBlock` providing endpoint operations.
pub struct UsbBus {
    usb: &'static pac::usb::RegisterBlock,
}

// SAFETY: UsbBus holds a reference to a fixed MMIO register block.
// Access is mediated by critical sections in the ISR / main-loop pattern.
unsafe impl Send for UsbBus {}
unsafe impl Sync for UsbBus {}

impl Default for UsbBus {
    fn default() -> Self {
        Self::new()
    }
}

impl UsbBus {
    pub fn new() -> Self {
        Self {
            usb: unsafe { &*pac::Usb::ptr() },
        }
    }

    #[inline]
    pub fn regs(&self) -> &pac::usb::RegisterBlock {
        self.usb
    }

    /// Write data to EP0 IN (control responses).
    pub fn ep0_write(&self, data: &[u8], max_len: usize) {
        let len = data.len().min(max_len);
        let pktcnt = if len == 0 { 1 } else { len.div_ceil(64) as u8 };
        self.usb.diep0tsiz().write(|w| unsafe {
            w.xfersize().bits(len as u8).pktcnt().bits(pktcnt)
        });
        self.usb
            .diep0ctl()
            .modify(|_, w| w.epena().set_bit().cnak().set_bit());
        write_fifo(fifo_addr(0), data, len);
    }

    /// Prepare EP0 OUT to receive SETUP or data.
    pub fn ep0_prepare_out(&self) {
        self.usb.doep0tsiz().write(|w| unsafe {
            w.supcnt().bits(3).pktcnt().set_bit().xfersize().bits(64)
        });
        self.usb
            .doep0ctl()
            .modify(|_, w| w.epena().set_bit().cnak().set_bit());
    }

    /// STALL EP0 (both directions).
    pub fn stall_ep0(&self) {
        self.usb
            .diep0ctl()
            .modify(|_, w| w.stall().set_bit());
        self.usb
            .doep0ctl()
            .modify(|_, w| w.stall().set_bit());
    }

    /// Write data to a bulk/interrupt IN endpoint (1 or 2).
    pub fn ep_write(&self, ep: u8, data: &[u8]) {
        match ep {
            1 => {
                let len = data.len().min(64);
                self.usb.diep0_tsiz().write(|w| unsafe {
                    w.xfersize().bits(len as u32).pktcnt().bits(1)
                });
                let is_iso = self.usb.diep0_ctl().read().eptype().is_iso();
                self.usb.diep0_ctl().modify(|_, w| {
                    let w = w.cnak().set_bit().epena().set_bit();
                    if is_iso {
                        // Schedule for the next frame (opposite parity).
                        let even_now =
                            self.usb.dsts().read().soffn().bits() & 1 == 0;
                        if even_now {
                            w.setd1pidof().set_bit()
                        } else {
                            w.setd0pidef().set_bit()
                        }
                    } else {
                        w
                    }
                });
                write_fifo(fifo_addr(1), data, len);
            }
            2 => {
                let len = data.len().min(64);
                self.usb.diep1_tsiz().write(|w| unsafe {
                    w.xfersize().bits(len as u32).pktcnt().bits(1)
                });
                let is_iso = self.usb.diep1_ctl().read().eptype().is_iso();
                self.usb.diep1_ctl().modify(|_, w| {
                    let w = w.cnak().set_bit().epena().set_bit();
                    if is_iso {
                        let even_now =
                            self.usb.dsts().read().soffn().bits() & 1 == 0;
                        if even_now {
                            w.setd1pidof().set_bit()
                        } else {
                            w.setd0pidef().set_bit()
                        }
                    } else {
                        w
                    }
                });
                write_fifo(fifo_addr(2), data, len);
            }
            _ => {}
        }
    }

    /// Prepare a bulk/interrupt OUT endpoint (1 or 2) to receive data.
    pub fn ep_prepare_out(&self, ep: u8, mps: u16) {
        match ep {
            1 => {
                self.usb.doep0_tsiz().write(|w| unsafe {
                    w.xfersize().bits(mps as u32).pktcnt().bits(1)
                });
                self.usb
                    .doep0_ctl()
                    .modify(|_, w| w.epena().set_bit().cnak().set_bit());
            }
            2 => {
                self.usb.doep1_tsiz().write(|w| unsafe {
                    w.xfersize().bits(mps as u32).pktcnt().bits(1)
                });
                self.usb
                    .doep1_ctl()
                    .modify(|_, w| w.epena().set_bit().cnak().set_bit());
            }
            _ => {}
        }
    }

    /// Flush EP0 IN TX FIFO if a transfer is pending.
    pub fn flush_ep0_tx_if_pending(&self) {
        if self.usb.diep0tsiz().read().pktcnt().bits() != 0 {
            self.usb.grstctl().write(|w| w.txfflsh().set_bit().txfnum().f0());
            while self.usb.grstctl().read().txfflsh().bit_is_set() {}
        }
    }

    /// Clear SETUP-phase bits in DOEP0INT (prevents race with data-stage XFERCOMPL).
    pub fn clear_ep0_setup_int(&self) {
        self.usb.doep0int().write(|w| w.setup().set_bit().xfercompl().set_bit());
    }
}
