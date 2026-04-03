//! Driver for the Sharp Memory LCD (LS013B7DH03) on the SLSTK3400A board.
//!
//! The display is 128x128 monochrome, driven via USART1 in SPI mode.
//!
//! Pin mapping (SLSTK3400A):
//!   PD7  – USART1_TX  (SPI MOSI), location 1
//!   PC15 – USART1_CLK (SPI SCK),  location 1
//!   PA10 – LCD chip-select (active HIGH)
//!   PA8  – LCD display-enable
//!   PA9  – EXTCOMIN (VCOM inversion toggle)
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
const PA_EN: u32 = 1 << 8;
const PA_EXTCOMIN: u32 = 1 << 9;

fn gpio() -> &'static pac::gpio::RegisterBlock {
    unsafe { &*pac::GPIO::ptr() }
}

fn usart1() -> &'static pac::usart1::RegisterBlock {
    unsafe { &*pac::USART1::ptr() }
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
    let u = usart1();
    while u.status().read().txbl().bit_is_clear() {}
    u.txdata().write(|w| unsafe { w.bits(byte as u32) });
}

fn spi_wait_done() {
    while usart1().status().read().txc().bit_is_clear() {}
}

// ---------- public API ----------

/// One-time initialisation: clocks, USART1 SPI mode, GPIO pins, display power.
///
/// Call this once after disabling the watchdog and enabling the GPIO clock.
/// The GPIO `Pins` struct must already have been split (or not yet consumed)
/// so that the raw GPIO registers are accessible.
pub fn init() {
    let cmu = unsafe { &*pac::CMU::ptr() };
    let u = usart1();
    let gpio = gpio();

    // Enable USART1 peripheral clock.
    cmu.hfperclken0().modify(|_, w| w.usart1().set_bit());

    // ---- Configure GPIO pins ----
    // PA8 (DISP_EN), PA9 (EXTCOMIN), PA10 (CS) → push-pull (mode 4).
    // PA_MODEH covers pins 8-15, 4 bits each.
    gpio.pa_modeh().modify(|r, w| unsafe {
        let mut v = r.bits();
        v &= !(0xFFF); // clear bits 0-11 (pins 8,9,10)
        v |= 0x444; // mode 4 = push-pull for each
        w.bits(v)
    });

    // PD7 → push-pull (USART1_TX).  PD_MODEL covers pins 0-7; pin 7 = bits 28-31.
    gpio.pd_model().modify(|r, w| unsafe {
        let mut v = r.bits();
        v &= !(0xF << 28);
        v |= 4 << 28; // push-pull
        w.bits(v)
    });

    // PC15 → push-pull (USART1_CLK). PC_MODEH pin 15 = bits 28-31.
    gpio.pc_modeh().modify(|r, w| unsafe {
        let mut v = r.bits();
        v &= !(0xF << 28);
        v |= 4 << 28; // push-pull
        w.bits(v)
    });

    // CS low initially.
    gpio.pa_doutclr().write(|w| unsafe { w.bits(PA_CS) });

    // ---- Configure USART1 as SPI master ----
    // Synchronous mode, LSB first (MSBF=0), CPOL=0, CPHA=0.
    u.ctrl().write(|w| w.sync().set_bit());
    u.frame().write(|w| w.databits().eight());

    // Clock divider: ~1 MHz SPI clock at 14 MHz HFPERCLK.
    // DIV = 256 * (f_HFPER / (2 * f_SPI) - 1) = 256 * 6 = 1536
    u.clkdiv().write(|w| unsafe { w.bits(1536) });

    // Route TX and CLK to Location 1 (PD7 / PC15).
    u.route()
        .write(|w| w.txpen().set_bit().clkpen().set_bit().location().loc1());

    // Enable master mode and TX.
    u.cmd()
        .write(|w| w.masteren().set_bit().txen().set_bit().rxen().set_bit());

    // ---- Power on the display ----
    gpio.pa_doutset().write(|w| unsafe { w.bits(PA_EN) });

    // Small delay for display power-up.
    cortex_m::asm::delay(100_000);
}

/// Clear the entire display to white.
pub fn clear(vcom: &mut bool) {
    let cmd = 0x04 | if *vcom { 0x02 } else { 0x00 };
    *vcom = !*vcom;

    cs_high();
    spi_write(cmd);
    spi_write(0x00);
    spi_wait_done();
    cs_low();
}

/// Toggle EXTCOMIN pin (call at ~1 Hz to prevent DC bias damage).
pub fn toggle_vcom() {
    let g = gpio();
    g.pa_douttgl().write(|w| unsafe { w.bits(PA_EXTCOMIN) });
}

/// Write a single pixel row (0-127) with 16 bytes of data.
///
/// `vcom` is toggled on each call to embed the VCOM bit in the command.
pub fn write_row(row: u8, data: &[u8; BYTES_PER_ROW], vcom: &mut bool) {
    let cmd = 0x01 | if *vcom { 0x02 } else { 0x00 };
    *vcom = !*vcom;

    cs_high();
    spi_write(cmd);
    spi_write(row + 1); // 1-indexed line address
    for &b in data.iter() {
        spi_write(b);
    }
    spi_write(0x00); // line trailer
    spi_write(0x00); // command trailer
    spi_wait_done();
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
    cs_low();
}

/// Render a text string at the given text-row and text-column position.
///
/// Characters are 8x8 pixels. The display has 16 columns × 16 rows of text.
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
