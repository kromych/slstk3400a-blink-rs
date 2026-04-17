use efm32hg322_pac as pac;

const USB_BASE: u32 = 0x400C_4000;

// ---------------------------------------------------------------------------
// DMA buffers (feature = "dma")
// ---------------------------------------------------------------------------

/// Word-aligned buffer wrapper for DMA transfers.
#[cfg(feature = "dma")]
#[repr(C, align(4))]
pub struct AlignedBuf<const N: usize> {
    pub data: [u8; N],
}

/// EP0 IN DMA buffer (one packet at a time, 64-byte MPS).
#[cfg(feature = "dma")]
static mut EP0_IN_DMA: AlignedBuf<64> = AlignedBuf { data: [0; 64] };
/// EP0 OUT DMA buffer (receives SETUP packets and DATA OUT payloads).
#[cfg(feature = "dma")]
static mut EP0_OUT_DMA: AlignedBuf<64> = AlignedBuf { data: [0; 64] };
/// EP1 IN DMA buffer (512 bytes for isochronous).
#[cfg(feature = "dma")]
static mut EP1_IN_DMA: AlignedBuf<512> = AlignedBuf { data: [0; 512] };
/// EP1 OUT DMA buffer.
#[cfg(feature = "dma")]
static mut EP1_OUT_DMA: AlignedBuf<64> = AlignedBuf { data: [0; 64] };
/// EP2 IN DMA buffer.
#[cfg(feature = "dma")]
static mut EP2_IN_DMA: AlignedBuf<512> = AlignedBuf { data: [0; 512] };
/// EP2 OUT DMA buffer.
#[cfg(feature = "dma")]
static mut EP2_OUT_DMA: AlignedBuf<64> = AlignedBuf { data: [0; 64] };

// Raw pointer accessors (only accessed from ISR / critical-section context).
#[cfg(feature = "dma")]
fn ep0_in_dma() -> *mut AlignedBuf<64> {
    core::ptr::addr_of_mut!(EP0_IN_DMA)
}
#[cfg(feature = "dma")]
fn ep0_out_dma() -> *mut AlignedBuf<64> {
    core::ptr::addr_of_mut!(EP0_OUT_DMA)
}
#[cfg(feature = "dma")]
fn ep1_in_dma() -> *mut AlignedBuf<512> {
    core::ptr::addr_of_mut!(EP1_IN_DMA)
}
#[cfg(feature = "dma")]
fn ep1_out_dma() -> *mut AlignedBuf<64> {
    core::ptr::addr_of_mut!(EP1_OUT_DMA)
}
#[cfg(feature = "dma")]
fn ep2_in_dma() -> *mut AlignedBuf<512> {
    core::ptr::addr_of_mut!(EP2_IN_DMA)
}
#[cfg(feature = "dma")]
fn ep2_out_dma() -> *mut AlignedBuf<64> {
    core::ptr::addr_of_mut!(EP2_OUT_DMA)
}

/// DWC2 FIFO base address for endpoint `ep`.
#[inline]
const fn fifo_addr(ep: u8) -> u32 {
    USB_BASE + 0x3D000 + (ep as u32) * 0x1000
}

#[inline]
fn fifo_read(addr: u32) -> u32 {
    unsafe { core::ptr::read_volatile(addr as *const u32) }
}

#[cfg(not(feature = "dma"))]
#[inline]
fn fifo_write(addr: u32, value: u32) {
    unsafe { core::ptr::write_volatile(addr as *mut u32, value) }
}

#[cfg(not(feature = "dma"))]
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

    /// Write up to 64 bytes (one MPS) to EP0 IN and start the transfer.
    ///
    /// For responses longer than 64 bytes, the caller must track the
    /// remaining data and call this again from the EP0 IN XFERCOMPL handler.
    pub fn ep0_write_packet(&self, data: &[u8]) {
        defmt::assert!(
            data.len() <= 64,
            "EP0 IN: len exceeds 64-byte MPS (DIEP0TSIZ.xfersize is 7 bits)"
        );

        #[cfg(feature = "dma")]
        return self.ep0_write_packet_dma(data);

        #[cfg(not(feature = "dma"))]
        {
            let len = data.len();
            self.usb
                .diep0tsiz()
                .write(|w| unsafe { w.xfersize().bits(len as u8).pktcnt().bits(1) });
            self.usb
                .diep0ctl()
                .modify(|_, w| w.epena().set_bit().cnak().set_bit());
            write_fifo(fifo_addr(0), data, len);
        }
    }

    /// Prepare EP0 OUT to receive SETUP or data.
    pub fn ep0_prepare_out(&self) {
        #[cfg(feature = "dma")]
        return self.ep0_prepare_out_dma();

        #[cfg(not(feature = "dma"))]
        {
            self.usb
                .doep0tsiz()
                .write(|w| unsafe { w.supcnt().bits(3).pktcnt().set_bit().xfersize().bits(64) });
            self.usb
                .doep0ctl()
                .modify(|_, w| w.epena().set_bit().cnak().set_bit());
        }
    }

    /// STALL EP0 (both directions).
    pub fn stall_ep0(&self) {
        self.usb.diep0ctl().modify(|_, w| w.stall().set_bit());
        self.usb.doep0ctl().modify(|_, w| w.stall().set_bit());
    }

    /// Write data to a non-EP0 IN endpoint (1 or 2).
    pub fn ep_write(&self, ep: u8, data: &[u8]) {
        #[cfg(feature = "dma")]
        return self.ep_write_dma(ep, data);

        #[cfg(not(feature = "dma"))]
        match ep {
            1 => {
                let len = data.len();
                self.usb
                    .diep0_tsiz()
                    .write(|w| unsafe { w.xfersize().bits(len as u32).pktcnt().bits(1) });
                let is_iso = self.usb.diep0_ctl().read().eptype().is_iso();
                self.usb.diep0_ctl().modify(|_, w| {
                    let w = w.usbactep().set_bit().cnak().set_bit().epena().set_bit();
                    if is_iso {
                        // Schedule for the next frame (opposite parity).
                        let even_now = self.usb.dsts().read().soffn().bits() & 1 == 0;
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
                let len = data.len();
                self.usb
                    .diep1_tsiz()
                    .write(|w| unsafe { w.xfersize().bits(len as u32).pktcnt().bits(1) });
                let is_iso = self.usb.diep1_ctl().read().eptype().is_iso();
                self.usb.diep1_ctl().modify(|_, w| {
                    let w = w.usbactep().set_bit().cnak().set_bit().epena().set_bit();
                    if is_iso {
                        let even_now = self.usb.dsts().read().soffn().bits() & 1 == 0;
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
        #[cfg(feature = "dma")]
        return self.ep_prepare_out_dma(ep, mps);

        #[cfg(not(feature = "dma"))]
        match ep {
            1 => {
                self.usb
                    .doep0_tsiz()
                    .write(|w| unsafe { w.xfersize().bits(mps as u32).pktcnt().bits(1) });
                self.usb
                    .doep0_ctl()
                    .modify(|_, w| w.epena().set_bit().cnak().set_bit());
            }
            2 => {
                self.usb
                    .doep1_tsiz()
                    .write(|w| unsafe { w.xfersize().bits(mps as u32).pktcnt().bits(1) });
                self.usb
                    .doep1_ctl()
                    .modify(|_, w| w.epena().set_bit().cnak().set_bit());
            }
            _ => {}
        }
    }

    /// Flush EP0 IN TX FIFO if a transfer is pending.
    ///
    /// The DWC2 spec requires Global IN NAK to be set before flushing a
    /// TX FIFO so the core does not transmit a partial packet while the
    /// FIFO is being drained.
    pub fn flush_ep0_tx_if_pending(&self) {
        if self.usb.diep0tsiz().read().pktcnt().bits() != 0 {
            self.usb
                .dctl()
                .modify(|_, w| w.sgnpinnak().set_bit());
            while !self.usb.dctl().read().gnpinnaksts().bit_is_set() {}
            self.usb
                .grstctl()
                .write(|w| w.txfflsh().set_bit().txfnum().f0());
            while self.usb.grstctl().read().txfflsh().bit_is_set() {}
            self.usb
                .dctl()
                .modify(|_, w| w.cgnpinnak().set_bit());
        }
    }

    /// Clear SETUP-phase bits in DOEP0INT (prevents race with data-stage XFERCOMPL).
    pub fn clear_ep0_setup_int(&self) {
        self.usb
            .doep0int()
            .write(|w| w.setup().set_bit().xfercompl().set_bit());
    }

    // -------------------------------------------------------------------
    // DMA-mode endpoint operations
    // -------------------------------------------------------------------

    /// Write a single EP0 IN packet via DMA (max 64 bytes).
    ///
    /// DIEP0TSIZ.xfersize is only 7 bits wide, so callers must chunk
    /// transfers larger than 64 bytes themselves (see `ep0_start_in` /
    /// `ep0_continue_in`).
    #[cfg(feature = "dma")]
    pub fn ep0_write_packet_dma(&self, data: &[u8]) {
        defmt::assert!(
            data.len() <= 64,
            "EP0 IN DMA: len exceeds 64-byte MPS (DIEP0TSIZ.xfersize is 7 bits)"
        );
        let len = data.len();
        let buf = unsafe { &mut (*ep0_in_dma()).data };
        buf[..len].copy_from_slice(&data[..len]);
        cortex_m::asm::dsb();

        self.usb
            .diep0tsiz()
            .write(|w| unsafe { w.xfersize().bits(len as u8).pktcnt().bits(1) });
        self.usb
            .diep0dmaaddr()
            .write(|w| unsafe { w.diep0dmaaddr().bits(buf.as_ptr() as u32) });
        self.usb
            .diep0ctl()
            .modify(|_, w| w.epena().set_bit().cnak().set_bit());
    }

    /// Prepare EP0 OUT for DMA reception (SETUP or data).
    ///
    /// Arms the endpoint with NAK set so that only SETUP packets (which
    /// bypass NAK) can be received.  This prevents a DATA OUT from racing
    /// ahead and overwriting the SETUP data in the shared DMA buffer
    /// before the ISR can read it.  Call [`ep0_clear_out_nak`] when the
    /// firmware is ready to accept a DATA OUT or status-stage ZLP.
    #[cfg(feature = "dma")]
    pub fn ep0_prepare_out_dma(&self) {
        let buf = unsafe { &(*ep0_out_dma()).data };
        self.usb
            .doep0tsiz()
            .write(|w| unsafe { w.supcnt().bits(3).pktcnt().set_bit().xfersize().bits(64) });
        self.usb
            .doep0dmaaddr()
            .write(|w| unsafe { w.doep0dmaaddr().bits(buf.as_ptr() as u32) });
        self.usb
            .doep0ctl()
            .modify(|_, w| w.epena().set_bit().snak().set_bit());
    }

    /// Clear NAK on EP0 OUT so the next DATA OUT or status ZLP can be
    /// received.  Only meaningful in DMA mode where [`ep0_prepare_out_dma`]
    /// arms with SNAK.
    ///
    /// Must also re-assert EPENA because the DWC2 core clears it on SETUP
    /// reception — if a SETUP arrived between the SNAK and CNAK writes,
    /// a bare CNAK `modify` would read back EPENA=0 and leave the endpoint
    /// disabled.
    #[cfg(feature = "dma")]
    pub fn ep0_clear_out_nak(&self) {
        self.usb
            .doep0ctl()
            .modify(|_, w| w.epena().set_bit().cnak().set_bit());
    }

    /// Read the SETUP packet (8 bytes) from the EP0 OUT DMA buffer.
    #[cfg(feature = "dma")]
    pub fn read_setup_dma(&self) -> [u8; 8] {
        cortex_m::asm::dsb();
        let buf = unsafe { &(*ep0_out_dma()).data };
        let mut setup = [0u8; 8];
        setup.copy_from_slice(&buf[..8]);
        setup
    }

    /// Read EP0 OUT data from the DMA buffer. Returns bytes actually received.
    #[cfg(feature = "dma")]
    pub fn read_ep0_data_dma(&self, out: &mut [u8], max: usize) -> usize {
        cortex_m::asm::dsb();
        let remaining = self.usb.doep0tsiz().read().xfersize().bits() as usize;
        let received = max.saturating_sub(remaining);
        let n = received.min(out.len()).min(64);
        let buf = unsafe { &(*ep0_out_dma()).data };
        out[..n].copy_from_slice(&buf[..n]);
        n
    }

    /// Write data to EP1 or EP2 IN via DMA.
    #[cfg(feature = "dma")]
    pub fn ep_write_dma(&self, ep: u8, data: &[u8]) {
        match ep {
            1 => {
                let len = data.len().min(512);
                let buf = unsafe { &mut (*ep1_in_dma()).data };
                buf[..len].copy_from_slice(&data[..len]);
                cortex_m::asm::dsb();

                self.usb
                    .diep0_tsiz()
                    .write(|w| unsafe { w.xfersize().bits(len as u32).pktcnt().bits(1) });
                self.usb
                    .diep0_dmaaddr()
                    .write(|w| unsafe { w.dmaaddr().bits(buf.as_ptr() as u32) });
                let is_iso = self.usb.diep0_ctl().read().eptype().is_iso();
                self.usb.diep0_ctl().modify(|_, w| {
                    let w = w.usbactep().set_bit().cnak().set_bit().epena().set_bit();
                    if is_iso {
                        let even_now = self.usb.dsts().read().soffn().bits() & 1 == 0;
                        if even_now {
                            w.setd1pidof().set_bit()
                        } else {
                            w.setd0pidef().set_bit()
                        }
                    } else {
                        w
                    }
                });
            }
            2 => {
                let len = data.len().min(512);
                let buf = unsafe { &mut (*ep2_in_dma()).data };
                buf[..len].copy_from_slice(&data[..len]);
                cortex_m::asm::dsb();

                self.usb
                    .diep1_tsiz()
                    .write(|w| unsafe { w.xfersize().bits(len as u32).pktcnt().bits(1) });
                self.usb
                    .diep1_dmaaddr()
                    .write(|w| unsafe { w.dmaaddr().bits(buf.as_ptr() as u32) });
                let is_iso = self.usb.diep1_ctl().read().eptype().is_iso();
                self.usb.diep1_ctl().modify(|_, w| {
                    let w = w.usbactep().set_bit().cnak().set_bit().epena().set_bit();
                    if is_iso {
                        let even_now = self.usb.dsts().read().soffn().bits() & 1 == 0;
                        if even_now {
                            w.setd1pidof().set_bit()
                        } else {
                            w.setd0pidef().set_bit()
                        }
                    } else {
                        w
                    }
                });
            }
            _ => {}
        }
    }

    /// Prepare EP1 or EP2 OUT for DMA reception.
    #[cfg(feature = "dma")]
    pub fn ep_prepare_out_dma(&self, ep: u8, mps: u16) {
        match ep {
            1 => {
                let buf = unsafe { &(*ep1_out_dma()).data };
                self.usb
                    .doep0_tsiz()
                    .write(|w| unsafe { w.xfersize().bits(mps as u32).pktcnt().bits(1) });
                self.usb
                    .doep0_dmaaddr()
                    .write(|w| unsafe { w.dmaaddr().bits(buf.as_ptr() as u32) });
                self.usb
                    .doep0_ctl()
                    .modify(|_, w| w.epena().set_bit().cnak().set_bit());
            }
            2 => {
                let buf = unsafe { &(*ep2_out_dma()).data };
                self.usb
                    .doep1_tsiz()
                    .write(|w| unsafe { w.xfersize().bits(mps as u32).pktcnt().bits(1) });
                self.usb
                    .doep1_dmaaddr()
                    .write(|w| unsafe { w.dmaaddr().bits(buf.as_ptr() as u32) });
                self.usb
                    .doep1_ctl()
                    .modify(|_, w| w.epena().set_bit().cnak().set_bit());
            }
            _ => {}
        }
    }

    /// Read data from an EPn OUT DMA buffer.
    #[cfg(feature = "dma")]
    pub fn read_ep_data_dma(&self, ep: u8, out: &mut [u8], max: usize) -> usize {
        cortex_m::asm::dsb();
        match ep {
            1 => {
                let remaining = self.usb.doep0_tsiz().read().xfersize().bits() as usize;
                let received = max.saturating_sub(remaining);
                let n = received.min(out.len()).min(64);
                let buf = unsafe { &(*ep1_out_dma()).data };
                out[..n].copy_from_slice(&buf[..n]);
                n
            }
            2 => {
                let remaining = self.usb.doep1_tsiz().read().xfersize().bits() as usize;
                let received = max.saturating_sub(remaining);
                let n = received.min(out.len()).min(64);
                let buf = unsafe { &(*ep2_out_dma()).data };
                out[..n].copy_from_slice(&buf[..n]);
                n
            }
            _ => 0,
        }
    }
}
