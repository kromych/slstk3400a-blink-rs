//! Framebuffer management and sprite rendering for the 128x128 monochrome LCD.
//!
//! LCD convention: bit=1 = white, bit=0 = black.  LSB = leftmost pixel in byte.
//! Sprites use MSB = leftmost pixel convention and are converted during blit.
//!
//! Safety: all framebuffer access is single-threaded (main loop only, never from ISRs).

use slstk3400a::display;
use slstk3400a::font8x8::FONT;

const FB_SIZE: usize = 128 * display::BYTES_PER_ROW;

/// 256-byte lookup table for bit reversal (MSB<->LSB).
/// Single indexed load replaces 6 shifts + 6 masks + 3 ORs per byte.
static REVERSE_LUT: [u8; 256] = {
    let mut lut = [0u8; 256];
    let mut i = 0u16;
    while i < 256 {
        let b = i as u8;
        let b = (b & 0xF0) >> 4 | (b & 0x0F) << 4;
        let b = (b & 0xCC) >> 2 | (b & 0x33) << 2;
        let b = (b & 0xAA) >> 1 | (b & 0x55) << 1;
        lut[i as usize] = b;
        i += 1;
    }
    lut
};

#[inline(always)]
fn reverse_bits(b: u8) -> u8 {
    REVERSE_LUT[b as usize]
}

/// The framebuffer: 128 rows x 16 bytes = 2048 bytes.
/// 1 = white, 0 = black (matches LCD).
static mut FB: [u8; FB_SIZE] = [0xFF; FB_SIZE];

/// Dirty-row bitmask: one bit per row (128 bits = 16 bytes).
/// Bit set = row has been modified since last flush.
static mut DIRTY: [u8; 16] = [0; 16];

// Raw pointer accessors to avoid `static_mut_refs` warnings.
// SAFETY: only accessed from main thread (never from ISRs).

#[inline(always)]
fn fb() -> *mut [u8; FB_SIZE] {
    core::ptr::addr_of_mut!(FB)
}

#[inline(always)]
fn dirty() -> *mut [u8; 16] {
    core::ptr::addr_of_mut!(DIRTY)
}

#[inline(always)]
fn fb_byte(idx: usize) -> u8 {
    unsafe { (*fb())[idx] }
}

#[inline(always)]
fn set_fb_byte(idx: usize, val: u8) {
    unsafe { (*fb())[idx] = val }
}

#[inline]
fn mark_dirty(row: u8) {
    let byte = (row / 8) as usize;
    let bit = row % 8;
    unsafe { (*dirty())[byte] |= 1 << bit }
}

/// Clear the framebuffer to white, only touching rows that have non-white pixels.
///
/// Rows that are already all-white are skipped entirely (no write, no dirty mark),
/// which dramatically reduces the number of rows flushed to the LCD each frame.
pub fn fb_clear() {
    for row in 0..128usize {
        let base = row * display::BYTES_PER_ROW;
        let mut needs_clear = false;
        for i in 0..display::BYTES_PER_ROW {
            if fb_byte(base + i) != 0xFF {
                needs_clear = true;
                break;
            }
        }
        if needs_clear {
            for i in 0..display::BYTES_PER_ROW {
                set_fb_byte(base + i, 0xFF);
            }
            mark_dirty(row as u8);
        }
    }
}

/// Blit an 8-pixel-wide sprite (MSB-left format) into the framebuffer.
/// Black pixels (1 in sprite) become black (0) in the framebuffer.
pub fn fb_blit_8(x: u8, y: u8, sprite: &[u8]) {
    for (sy, &sprite_byte) in sprite.iter().enumerate() {
        let row = y as usize + sy;
        if row >= 128 {
            break;
        }
        if sprite_byte == 0 {
            continue;
        }
        let base = row * display::BYTES_PER_ROW;
        let lcd_bits = reverse_bits(sprite_byte);
        let shift = x % 8;
        let byte_col = (x / 8) as usize;

        if shift == 0 {
            if byte_col < display::BYTES_PER_ROW {
                set_fb_byte(base + byte_col, fb_byte(base + byte_col) & !lcd_bits);
            }
        } else {
            if byte_col < display::BYTES_PER_ROW {
                let idx = base + byte_col;
                set_fb_byte(idx, fb_byte(idx) & !(lcd_bits << shift));
            }
            if byte_col + 1 < display::BYTES_PER_ROW {
                let idx = base + byte_col + 1;
                set_fb_byte(idx, fb_byte(idx) & !(lcd_bits >> (8 - shift)));
            }
        }
        mark_dirty(row as u8);
    }
}

/// Blit a 16-pixel-wide sprite (2 bytes per row, MSB-left format).
pub fn fb_blit_16(x: u8, y: u8, sprite: &[[u8; 2]]) {
    for (sy, row_data) in sprite.iter().enumerate() {
        let row = y as usize + sy;
        if row >= 128 {
            break;
        }
        let base = row * display::BYTES_PER_ROW;
        let shift = x % 8;
        let byte_col = (x / 8) as usize;

        let left = reverse_bits(row_data[0]);
        let right = reverse_bits(row_data[1]);

        if shift == 0 {
            if byte_col < display::BYTES_PER_ROW {
                set_fb_byte(base + byte_col, fb_byte(base + byte_col) & !left);
            }
            if byte_col + 1 < display::BYTES_PER_ROW {
                let idx = base + byte_col + 1;
                set_fb_byte(idx, fb_byte(idx) & !right);
            }
        } else {
            let combined: u32 = ((left as u32) | ((right as u32) << 8)) << shift;
            for off in 0..3usize {
                if byte_col + off < display::BYTES_PER_ROW {
                    let idx = base + byte_col + off;
                    let mask = (combined >> (off * 8)) as u8;
                    set_fb_byte(idx, fb_byte(idx) & !mask);
                }
            }
        }
        mark_dirty(row as u8);
    }
}

/// Render a text string into the framebuffer at a text-row/column position.
/// Characters are 8x8 pixels. Background is white, text is black.
pub fn fb_draw_text(text_row: u8, text_col: u8, text: &str) {
    let y = text_row as usize * 8;
    if y >= 128 {
        return;
    }

    for (ci, ch) in text.bytes().enumerate() {
        let cx = text_col as usize + ci;
        if cx >= 16 {
            break;
        }
        let glyph_idx = if (32..=126).contains(&ch) {
            (ch - 32) as usize
        } else {
            0
        };
        let glyph = &FONT[glyph_idx];
        for (py, &glyph_row) in glyph.iter().enumerate().take(8) {
            let row = y + py;
            if row >= 128 {
                break;
            }
            let base = row * display::BYTES_PER_ROW;
            let lcd_byte = !reverse_bits(glyph_row);
            set_fb_byte(base + cx, lcd_byte);
            mark_dirty(row as u8);
        }
    }
}

/// Flush dirty rows to the LCD, batching consecutive dirty rows into single
/// SPI transactions via `write_rows()` to reduce CS toggle overhead.
pub fn fb_flush(vcom: &mut bool) {
    let mut row: u8 = 0;
    while row < 128 {
        let byte_idx = (row / 8) as usize;
        let bit = row % 8;
        if unsafe { (*dirty())[byte_idx] } & (1 << bit) == 0 {
            row += 1;
            continue;
        }

        // Start of a dirty run.
        let start = row;
        row += 1;
        while row < 128 {
            let byte_idx = (row / 8) as usize;
            let bit = row % 8;
            if unsafe { (*dirty())[byte_idx] } & (1 << bit) == 0 {
                break;
            }
            row += 1;
        }

        // Send the entire consecutive run in one SPI transaction.
        let count = (row - start) as usize;
        let base = start as usize * display::BYTES_PER_ROW;
        let rows_data: &[[u8; display::BYTES_PER_ROW]] = unsafe {
            core::slice::from_raw_parts(
                (*fb()).as_ptr().add(base).cast(),
                count,
            )
        };
        #[cfg(feature = "dma")]
        display::write_rows_dma(start, rows_data, vcom);
        #[cfg(not(feature = "dma"))]
        display::write_rows(start, rows_data, vcom);
    }

    // Clear all dirty flags.
    for i in 0..16 {
        unsafe { (*dirty())[i] = 0 }
    }
}
