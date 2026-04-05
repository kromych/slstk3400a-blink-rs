//! Framebuffer management and sprite rendering for the 128x128 monochrome LCD.
//!
//! LCD convention: bit=1 → white, bit=0 → black.  LSB = leftmost pixel in byte.
//! Sprites use MSB = leftmost pixel convention and are converted during blit.
//!
//! Safety: all framebuffer access is single-threaded (main loop only, never from ISRs).

use slstk3400a::display;
use slstk3400a::font8x8::{reverse_bits, FONT};

const FB_SIZE: usize = 128 * display::BYTES_PER_ROW;

/// The framebuffer: 128 rows × 16 bytes = 2048 bytes.
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

/// Clear entire framebuffer to white and mark all rows dirty.
pub fn fb_clear() {
    for i in 0..FB_SIZE {
        set_fb_byte(i, 0xFF);
    }
    for i in 0..16 {
        unsafe { (*dirty())[i] = 0xFF }
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

/// Flush all dirty rows to the LCD.
pub fn fb_flush(vcom: &mut bool) {
    for byte_idx in 0..16usize {
        let dirty_byte = unsafe { (*dirty())[byte_idx] };
        if dirty_byte == 0 {
            continue;
        }
        for bit in 0..8u8 {
            if dirty_byte & (1 << bit) != 0 {
                let row = byte_idx as u8 * 8 + bit;
                if row < 128 {
                    let base = row as usize * display::BYTES_PER_ROW;
                    let row_data: &[u8; display::BYTES_PER_ROW] = unsafe {
                        &*((&(*fb()))[base..base + display::BYTES_PER_ROW]
                            .as_ptr()
                            .cast())
                    };
                    display::write_row(row, row_data, vcom);
                }
            }
        }
    }
    // Clear all dirty flags.
    for i in 0..16 {
        unsafe { (*dirty())[i] = 0 }
    }
}
