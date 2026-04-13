//! Driver for the Sharp Memory LCD (LS013B7DH03) on the SLSTK3400A board.
//!
//! The display is 128x128 monochrome, driven via USART0 in SPI mode.
//!
//! Pin mapping (SLSTK3400A, from SiLabs displayls013b7dh03config.h):
//!   PE10 - USART0_TX  (SPI MOSI / SI),  location 0
//!   PE12 - USART0_CLK (SPI SCK / SCLK), location 0
//!   PA10 - LCD chip-select (SCS, active HIGH)
//!   PA8  - LCD display-select (DISP_SEL)
//!   PF3  - EXTCOMIN (VCOM inversion toggle)
//!
//! The framebuffer-less API writes individual text lines directly, avoiding a
//! 2 KiB RAM allocation.

use crate::font8x8::{reverse_bits, FONT};
use efm32hg322_pac as pac;

// ---------- display geometry ----------

/// Display width in pixels.
pub const WIDTH: u8 = 128;
/// Display height in pixels.
pub const HEIGHT: u8 = 128;
/// Bytes per pixel row (128 / 8).
pub const BYTES_PER_ROW: usize = 16;
/// Number of 8-pixel-tall text rows (128 / 8).
pub const TEXT_ROWS: u8 = 16;
/// Number of 8-pixel-wide text columns (128 / 8).
pub const TEXT_COLS: u8 = 16;

// ---------- GPIO helpers (raw PAC) ----------

/// GPIO port A DOUT bit positions used by the LCD.
const PA_CS: u32 = 1 << 10;
const PA_DISP_SEL: u32 = 1 << 8;

/// GPIO port F DOUT bit position for EXTCOMIN.
const PF_EXTCOMIN: u32 = 1 << 3;

fn gpio() -> &'static pac::gpio::RegisterBlock {
    unsafe { &*pac::Gpio::ptr() }
}

fn usart0() -> &'static pac::usart0::RegisterBlock {
    unsafe { &*pac::Usart0::ptr() }
}

#[inline]
fn cs_high() {
    gpio().pa_doutset().write(|w| unsafe { w.bits(PA_CS) });
}
#[inline]
fn cs_low() {
    gpio().pa_doutclr().write(|w| unsafe { w.bits(PA_CS) });
}

// ---------- SPI helpers ----------

#[inline]
fn spi_write(byte: u8) {
    let u = usart0();
    while u.status().read().txbl().bit_is_clear() {}
    u.txdata().write(|w| unsafe { w.bits(byte as u32) });
}

fn spi_wait_done() {
    while usart0().status().read().txc().bit_is_clear() {}
}

/// Delay for at least `us` microseconds at 14 MHz HFRCO.
#[inline]
fn delay_us(us: u32) {
    cortex_m::asm::delay(us * 14);
}

/// SCS setup time: 6 us min (LS013B7DH03 datasheet).
#[inline]
fn scs_setup_delay() {
    delay_us(6);
}

/// SCS hold time: 2 us min (LS013B7DH03 datasheet).
#[inline]
fn scs_hold_delay() {
    delay_us(2);
}

// ---------- uDMA for SPI (feature = "dma") ----------
//
// The EFM32HG PL230 uDMA controller can feed bytes from RAM to USART0_TXDATA
// autonomously, freeing the CPU while the LCD SPI transaction is in flight.
//
// Descriptor table layout (PL230):
//   6 primary descriptors (96 B) + 6 alternate descriptors (96 B) = 192 B
//   Base address must be aligned to the next power-of-2 >= 192, i.e. 256.
//
// Each descriptor: src_end_ptr, dst_end_ptr, ctrl, _pad  (4 words = 16 bytes).

/// USART0 TXDATA register address (base 0x4000_C000, offset 0x34).
#[cfg(feature = "dma")]
const USART0_TXDATA_ADDR: u32 = 0x4000_C034;
/// DMA channel used for SPI display transfers.
#[cfg(feature = "dma")]
const DMA_CH: usize = 0;
/// Max bytes per single basic-cycle DMA transfer (n_minus_1 is 10-bit).
#[cfg(feature = "dma")]
const DMA_MAX_XFER: usize = 1024;

#[cfg(feature = "dma")]
/// Single PL230 DMA descriptor (16 bytes).
#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct DmaDesc {
    src_end: u32,
    dst_end: u32,
    ctrl: u32,
    _pad: u32,
}

#[cfg(feature = "dma")]
impl DmaDesc {
    const ZERO: Self = Self {
        src_end: 0,
        dst_end: 0,
        ctrl: 0,
        _pad: 0,
    };
}

#[cfg(feature = "dma")]
/// Full descriptor table: 6 primary + 6 alternate, 256-byte aligned.
#[repr(C, align(256))]
struct DmaDescTable {
    desc: [DmaDesc; 12],
}

#[cfg(feature = "dma")]
/// Max SPI packet: cmd + 128 * (addr + 16 data + trailer) + final_trailer = 2306.
const DMA_TX_BUF_SIZE: usize = 2308;

#[cfg(feature = "dma")]
static mut DMA_DESC: DmaDescTable = DmaDescTable {
    desc: [DmaDesc::ZERO; 12],
};
#[cfg(feature = "dma")]
static mut DMA_TX_BUF: [u8; DMA_TX_BUF_SIZE] = [0; DMA_TX_BUF_SIZE];

#[cfg(feature = "dma")]
fn dma_desc() -> *mut DmaDescTable {
    core::ptr::addr_of_mut!(DMA_DESC)
}

#[cfg(feature = "dma")]
fn dma_tx_buf() -> *mut [u8; DMA_TX_BUF_SIZE] {
    core::ptr::addr_of_mut!(DMA_TX_BUF)
}

#[cfg(feature = "dma")]
fn dma() -> &'static pac::dma::RegisterBlock {
    unsafe { &*pac::Dma::ptr() }
}

#[cfg(feature = "dma")]
/// Build a PL230 control word for a byte-to-peripheral basic transfer.
///   src: byte, incrementing
///   dst: byte, no increment (fixed TXDATA)
///   R_power = 0 (re-arbitrate after every transfer -- peripheral paced)
///   cycle_ctrl = 1 (basic)
#[inline]
const fn dma_ctrl_word(n_minus_1: u32) -> u32 {
    // dst_inc=no_increment(11), dst_size=byte, src_inc=byte, src_size=byte,
    // R_power=0 (re-arb every xfer), cycle_ctrl=basic(1)
    (0b11 << 30) | (n_minus_1 << 4) | 1
}

#[cfg(feature = "dma")]
/// Kick off a DMA-driven transfer of `data` to USART0_TXDATA.
/// Blocks until all bytes have been pushed into the USART TX buffer.
fn dma_spi_send(data: &[u8]) {
    let dma = dma();
    let mut offset = 0usize;

    while offset < data.len() {
        let chunk = (data.len() - offset).min(DMA_MAX_XFER);
        let n_minus_1 = (chunk - 1) as u32;

        // Programme the primary descriptor for channel 0.
        let desc = unsafe { &mut (*dma_desc()).desc[DMA_CH] };
        desc.src_end = unsafe { data.as_ptr().add(offset + chunk - 1) } as u32;
        desc.dst_end = USART0_TXDATA_ADDR;
        desc.ctrl = dma_ctrl_word(n_minus_1);

        // Clear any stale done flag, then enable the channel.
        dma.ifc().write(|w| w.ch0done().set_bit());
        dma.chens().write(|w| w.ch0ens().set_bit());

        // Spin until the DMA channel completes.
        while dma.if_().read().ch0done().bit_is_clear() {}
        dma.ifc().write(|w| w.ch0done().set_bit());

        offset += chunk;
    }
}

// ---------- public API ----------

/// One-time initialisation: clocks, USART0 SPI mode, GPIO pins, display power.
///
/// Call this once after disabling the watchdog and enabling the GPIO clock.
/// The GPIO `Pins` struct must already have been split (or not yet consumed)
/// so that the raw GPIO registers are accessible.
pub fn init() {
    let cmu = unsafe { &*pac::Cmu::ptr() };
    let u = usart0();
    let gpio = gpio();

    // Enable USART0 peripheral clock.
    cmu.hfperclken0()
        .modify(|_, w: &mut pac::cmu::hfperclken0::W| w.usart0().set_bit());

    // ---- Configure GPIO pins ----
    // PA8 (DISP_SEL), PA10 (CS) -> push-pull (mode 4).
    // PA_MODEH covers pins 8-15, 4 bits each.
    // Pin 8: bits 0-3, Pin 10: bits 8-11.
    gpio.pa_modeh().modify(|r, w| unsafe {
        let mut v = r.bits();
        v &= !0xF; // clear pin 8 (DISP_SEL)
        v |= 4; // push-pull
        v &= !(0xF << 8); // clear pin 10 (CS)
        v |= 4 << 8; // push-pull
        w.bits(v)
    });

    // PE10 -> push-pull (USART0_TX / MOSI).
    // PE_MODEH covers pins 8-15; pin 10 = bits 8-11.
    gpio.pe_modeh().modify(|r, w| unsafe {
        let mut v = r.bits();
        v &= !(0xF << 8); // clear pin 10
        v |= 4 << 8; // push-pull
        w.bits(v)
    });

    // PE12 -> push-pull (USART0_CLK / SCK).
    // PE_MODEH pin 12 = bits 16-19.
    gpio.pe_modeh().modify(|r, w| unsafe {
        let mut v = r.bits();
        v &= !(0xF << 16); // clear pin 12
        v |= 4 << 16; // push-pull
        w.bits(v)
    });

    // PF3 -> push-pull (EXTCOMIN).
    // PF_MODEL covers pins 0-7; pin 3 = bits 12-15.
    gpio.pf_model().modify(|r, w| unsafe {
        let mut v = r.bits();
        v &= !(0xF << 12); // clear pin 3
        v |= 4 << 12; // push-pull
        w.bits(v)
    });

    // CS low initially.
    gpio.pa_doutclr().write(|w| unsafe { w.bits(PA_CS) });

    // ---- Configure USART0 as SPI master ----
    // Synchronous mode, LSB first (MSBF=0), CPOL=0, CPHA=0.
    u.ctrl().write(|w| w.sync().set_bit());
    u.frame().write(|w| w.databits().eight());

    // Clock divider: ~1 MHz SPI clock at 14 MHz HFPERCLK.
    // DIV = 256 * (f_HFPER / (2 * f_SPI) - 1) = 256 * 6 = 1536
    u.clkdiv().write(|w| unsafe { w.bits(1536) });

    // Route TX and CLK to Location 0 (PE10 / PE12).
    u.route()
        .write(|w| w.txpen().set_bit().clkpen().set_bit().location().loc0());

    // Enable master mode and TX.
    u.cmd()
        .write(|w| w.masteren().set_bit().txen().set_bit().rxen().set_bit());

    // ---- Power on the display ----
    gpio.pa_doutset().write(|w| unsafe { w.bits(PA_DISP_SEL) });

    // Small delay for display power-up.
    cortex_m::asm::delay(100_000);
}

/// Clear the entire display to white.
pub fn clear(vcom: &mut bool) {
    let cmd = 0x04 | if *vcom { 0x02 } else { 0x00 };
    *vcom = !*vcom;

    cs_high();
    scs_setup_delay();
    spi_write(cmd);
    spi_write(0x00);
    spi_wait_done();
    scs_hold_delay();
    cs_low();
}

/// Toggle EXTCOMIN pin (call at ~1 Hz to prevent DC bias damage).
pub fn toggle_vcom() {
    let g = gpio();
    g.pf_douttgl().write(|w| unsafe { w.bits(PF_EXTCOMIN) });
}

/// Write a single pixel row (0-127) with 16 bytes of data.
///
/// `vcom` is toggled on each call to embed the VCOM bit in the command.
pub fn write_row(row: u8, data: &[u8; BYTES_PER_ROW], vcom: &mut bool) {
    let cmd = 0x01 | if *vcom { 0x02 } else { 0x00 };
    *vcom = !*vcom;

    cs_high();
    scs_setup_delay();
    spi_write(cmd);
    spi_write(row + 1); // 1-indexed line address
    for &b in data.iter() {
        spi_write(b);
    }
    spi_write(0x00); // line trailer
    spi_write(0x00); // command trailer
    spi_wait_done();
    scs_hold_delay();
    cs_low();
}

/// Write multiple consecutive pixel rows starting at `start_row`.
pub fn write_rows(start_row: u8, rows: &[[u8; BYTES_PER_ROW]], vcom: &mut bool) {
    if rows.is_empty() {
        return;
    }
    let cmd = 0x01 | if *vcom { 0x02 } else { 0x00 };
    *vcom = !*vcom;

    cs_high();
    scs_setup_delay();
    spi_write(cmd);
    for (i, row_data) in rows.iter().enumerate() {
        spi_write(start_row + i as u8 + 1);
        for &b in row_data.iter() {
            spi_write(b);
        }
        spi_write(0x00); // line trailer
    }
    spi_write(0x00); // final trailer
    spi_wait_done();
    scs_hold_delay();
    cs_low();
}

#[cfg(feature = "dma")]
/// One-time DMA initialisation for SPI display transfers.
///
/// Enables the DMA clock, sets the descriptor table base pointer, configures
/// channel 0 for USART0 TXBL requests, and enables the controller.
/// After this call, [`write_rows_dma`] can be used instead of [`write_rows`].
pub fn dma_init() {
    let cmu = unsafe { &*pac::Cmu::ptr() };
    let dma = dma();

    // Enable DMA clock.
    cmu.hfcoreclken0().modify(|_, w| w.dma().set_bit());

    // Point controller at descriptor table.
    dma.ctrlbase()
        .write(|w| unsafe { w.ctrlbase().bits(dma_desc() as u32) });

    // Enable controller.
    dma.config().write(|w| w.en().set_bit());

    // Channel 0: USART0, signal 1 = TXBL.
    dma.ch0_ctrl()
        .write(|w| unsafe { w.sourcesel().usart0().sigsel().bits(1) });
}

#[cfg(feature = "dma")]
/// Write multiple consecutive pixel rows via DMA.
///
/// Same interface as [`write_rows`] but the SPI packet is assembled into a
/// static buffer and transferred by the uDMA engine, freeing the CPU while
/// bytes are clocked out.  Call [`dma_init`] once before first use.
pub fn write_rows_dma(start_row: u8, rows: &[[u8; BYTES_PER_ROW]], vcom: &mut bool) {
    if rows.is_empty() {
        return;
    }
    let cmd = 0x01 | if *vcom { 0x02 } else { 0x00 };
    *vcom = !*vcom;

    // Assemble the full SPI packet into the DMA TX buffer.
    let buf = unsafe { &mut *dma_tx_buf() };
    let mut pos = 0usize;
    buf[pos] = cmd;
    pos += 1;
    for (i, row_data) in rows.iter().enumerate() {
        buf[pos] = start_row + i as u8 + 1; // 1-indexed line address
        pos += 1;
        buf[pos..pos + BYTES_PER_ROW].copy_from_slice(row_data);
        pos += BYTES_PER_ROW;
        buf[pos] = 0x00; // line trailer
        pos += 1;
    }
    buf[pos] = 0x00; // final trailer
    pos += 1;

    cs_high();
    scs_setup_delay();

    dma_spi_send(&buf[..pos]);

    // Wait for the last byte to finish clocking out of the shift register.
    spi_wait_done();
    scs_hold_delay();
    cs_low();
}

/// Render a text string at the given text-row and text-column position.
///
/// Characters are 8x8 pixels. The display has 16 columns x 16 rows of text.
/// Background is white (1), text is black (0).
pub fn draw_text(row: u8, col: u8, text: &str, vcom: &mut bool) {
    if row >= TEXT_ROWS {
        return;
    }

    // Build 8 pixel rows for this text row.
    let mut pixel_rows = [[0xFFu8; BYTES_PER_ROW]; 8];

    for (ci, ch) in text.bytes().enumerate() {
        let cx = col as usize + ci;
        if cx >= TEXT_COLS as usize {
            break;
        }
        let glyph_idx = if (32..=126).contains(&ch) {
            (ch - 32) as usize
        } else {
            0 // space for unprintable
        };
        let glyph = &FONT[glyph_idx];
        for py in 0..8 {
            // Font: MSB = leftmost pixel.  LCD: LSB = leftmost pixel.
            // Invert because 1=white, 0=black on the LCD (black text on white).
            pixel_rows[py][cx] = !reverse_bits(glyph[py]);
        }
    }

    let start = row * 8;
    write_rows(start, &pixel_rows, vcom);
}

/// Fill a rectangular region (in pixel coordinates) with black or white.
///
/// Coordinates are clipped to the 128x128 display.
pub fn fill_rect(x: u8, y: u8, w: u8, h: u8, black: bool, vcom: &mut bool) {
    let x0 = x.min(WIDTH);
    let y0 = y.min(HEIGHT);
    let x1 = (x as u16 + w as u16).min(WIDTH as u16) as u8;
    let y1 = (y as u16 + h as u16).min(HEIGHT as u16) as u8;

    for row in y0..y1 {
        let mut data = [0xFFu8; BYTES_PER_ROW]; // white background
        for px in x0..x1 {
            let byte_idx = (px / 8) as usize;
            let bit_idx = px % 8;
            if black {
                data[byte_idx] &= !(1 << bit_idx); // clear bit = black
            }
            // white: already 0xFF
        }
        write_row(row, &data, vcom);
    }
}

/// Format a `u32` as decimal into a caller-provided buffer.
/// Returns the slice of `buf` that contains the formatted digits.
pub fn format_u32(value: u32, buf: &mut [u8; 10]) -> &[u8] {
    if value == 0 {
        buf[0] = b'0';
        return &buf[..1];
    }
    let mut v = value;
    let mut i = 10;
    while v > 0 && i > 0 {
        i -= 1;
        buf[i] = b'0' + (v % 10) as u8;
        v /= 10;
    }
    &buf[i..]
}

/// Format an `i32` as decimal into a caller-provided buffer.
/// Returns the number of bytes written.
pub fn format_i32(value: i32, buf: &mut [u8; 12]) -> usize {
    let negative = value < 0;
    let abs = if negative {
        (value as i64).unsigned_abs() as u32
    } else {
        value as u32
    };

    let mut tmp = [0u8; 10];
    let digits = format_u32(abs, &mut tmp);
    let mut pos = 0;
    if negative {
        buf[0] = b'-';
        pos = 1;
    }
    buf[pos..pos + digits.len()].copy_from_slice(digits);
    pos + digits.len()
}
