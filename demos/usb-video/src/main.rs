//! USB Video device demo for the SLSTK3400A.
//!
//! Streams a Star Wars-style opening text crawl as 160x120 YUY2 video
//! at 5 fps via isochronous IN transfers.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use efm32hg322_pac as pac;
use efm32hg322_usb::video::{
    self, VideoClass, VideoHandler, BYTES_PER_LINE, HEIGHT, WIDTH,
};
use efm32hg322_usb::UsbDevice;

// ---------------------------------------------------------------------------
// 5x7 bitmap font (printable ASCII 0x20..0x7E, 5 bytes per glyph, column-major)
//
// Each byte is one column, bit 0 = top row, bit 6 = bottom row.
// Character cell: 6 wide (5 + 1 space) x 8 tall (7 + 1 space).
// ---------------------------------------------------------------------------

#[rustfmt::skip]
static FONT_5X7: [[u8; 5]; 95] = [
    [0x00,0x00,0x00,0x00,0x00], // 32 ' '
    [0x00,0x00,0x5F,0x00,0x00], // 33 '!'
    [0x00,0x07,0x00,0x07,0x00], // 34 '"'
    [0x14,0x7F,0x14,0x7F,0x14], // 35 '#'
    [0x24,0x2A,0x7F,0x2A,0x12], // 36 '$'
    [0x23,0x13,0x08,0x64,0x62], // 37 '%'
    [0x36,0x49,0x55,0x22,0x50], // 38 '&'
    [0x00,0x05,0x03,0x00,0x00], // 39 '''
    [0x00,0x1C,0x22,0x41,0x00], // 40 '('
    [0x00,0x41,0x22,0x1C,0x00], // 41 ')'
    [0x08,0x2A,0x1C,0x2A,0x08], // 42 '*'
    [0x08,0x08,0x3E,0x08,0x08], // 43 '+'
    [0x00,0x50,0x30,0x00,0x00], // 44 ','
    [0x08,0x08,0x08,0x08,0x08], // 45 '-'
    [0x00,0x60,0x60,0x00,0x00], // 46 '.'
    [0x20,0x10,0x08,0x04,0x02], // 47 '/'
    [0x3E,0x51,0x49,0x45,0x3E], // 48 '0'
    [0x00,0x42,0x7F,0x40,0x00], // 49 '1'
    [0x42,0x61,0x51,0x49,0x46], // 50 '2'
    [0x21,0x41,0x45,0x4B,0x31], // 51 '3'
    [0x18,0x14,0x12,0x7F,0x10], // 52 '4'
    [0x27,0x45,0x45,0x45,0x39], // 53 '5'
    [0x3C,0x4A,0x49,0x49,0x30], // 54 '6'
    [0x01,0x71,0x09,0x05,0x03], // 55 '7'
    [0x36,0x49,0x49,0x49,0x36], // 56 '8'
    [0x06,0x49,0x49,0x29,0x1E], // 57 '9'
    [0x00,0x36,0x36,0x00,0x00], // 58 ':'
    [0x00,0x56,0x36,0x00,0x00], // 59 ';'
    [0x00,0x08,0x14,0x22,0x41], // 60 '<'
    [0x14,0x14,0x14,0x14,0x14], // 61 '='
    [0x41,0x22,0x14,0x08,0x00], // 62 '>'
    [0x02,0x01,0x51,0x09,0x06], // 63 '?'
    [0x32,0x49,0x79,0x41,0x3E], // 64 '@'
    [0x7E,0x11,0x11,0x11,0x7E], // 65 'A'
    [0x7F,0x49,0x49,0x49,0x36], // 66 'B'
    [0x3E,0x41,0x41,0x41,0x22], // 67 'C'
    [0x7F,0x41,0x41,0x22,0x1C], // 68 'D'
    [0x7F,0x49,0x49,0x49,0x41], // 69 'E'
    [0x7F,0x09,0x09,0x01,0x01], // 70 'F'
    [0x3E,0x41,0x41,0x51,0x32], // 71 'G'
    [0x7F,0x08,0x08,0x08,0x7F], // 72 'H'
    [0x00,0x41,0x7F,0x41,0x00], // 73 'I'
    [0x20,0x40,0x41,0x3F,0x01], // 74 'J'
    [0x7F,0x08,0x14,0x22,0x41], // 75 'K'
    [0x7F,0x40,0x40,0x40,0x40], // 76 'L'
    [0x7F,0x02,0x04,0x02,0x7F], // 77 'M'
    [0x7F,0x04,0x08,0x10,0x7F], // 78 'N'
    [0x3E,0x41,0x41,0x41,0x3E], // 79 'O'
    [0x7F,0x09,0x09,0x09,0x06], // 80 'P'
    [0x3E,0x41,0x51,0x21,0x5E], // 81 'Q'
    [0x7F,0x09,0x19,0x29,0x46], // 82 'R'
    [0x46,0x49,0x49,0x49,0x31], // 83 'S'
    [0x01,0x01,0x7F,0x01,0x01], // 84 'T'
    [0x3F,0x40,0x40,0x40,0x3F], // 85 'U'
    [0x1F,0x20,0x40,0x20,0x1F], // 86 'V'
    [0x7F,0x20,0x18,0x20,0x7F], // 87 'W'
    [0x63,0x14,0x08,0x14,0x63], // 88 'X'
    [0x03,0x04,0x78,0x04,0x03], // 89 'Y'
    [0x61,0x51,0x49,0x45,0x43], // 90 'Z'
    [0x00,0x00,0x7F,0x41,0x41], // 91 '['
    [0x02,0x04,0x08,0x10,0x20], // 92 '\'
    [0x41,0x41,0x7F,0x00,0x00], // 93 ']'
    [0x04,0x02,0x01,0x02,0x04], // 94 '^'
    [0x40,0x40,0x40,0x40,0x40], // 95 '_'
    [0x00,0x01,0x02,0x04,0x00], // 96 '`'
    [0x20,0x54,0x54,0x54,0x78], // 97 'a'
    [0x7F,0x48,0x44,0x44,0x38], // 98 'b'
    [0x38,0x44,0x44,0x44,0x20], // 99 'c'
    [0x38,0x44,0x44,0x48,0x7F], // 100 'd'
    [0x38,0x54,0x54,0x54,0x18], // 101 'e'
    [0x08,0x7E,0x09,0x01,0x02], // 102 'f'
    [0x08,0x14,0x54,0x54,0x3C], // 103 'g'
    [0x7F,0x08,0x04,0x04,0x78], // 104 'h'
    [0x00,0x44,0x7D,0x40,0x00], // 105 'i'
    [0x20,0x40,0x44,0x3D,0x00], // 106 'j'
    [0x00,0x7F,0x10,0x28,0x44], // 107 'k'
    [0x00,0x41,0x7F,0x40,0x00], // 108 'l'
    [0x7C,0x04,0x18,0x04,0x78], // 109 'm'
    [0x7C,0x08,0x04,0x04,0x78], // 110 'n'
    [0x38,0x44,0x44,0x44,0x38], // 111 'o'
    [0x7C,0x14,0x14,0x14,0x08], // 112 'p'
    [0x08,0x14,0x14,0x18,0x7C], // 113 'q'
    [0x7C,0x08,0x04,0x04,0x08], // 114 'r'
    [0x48,0x54,0x54,0x54,0x20], // 115 's'
    [0x04,0x3F,0x44,0x40,0x20], // 116 't'
    [0x3C,0x40,0x40,0x20,0x7C], // 117 'u'
    [0x1C,0x20,0x40,0x20,0x1C], // 118 'v'
    [0x3C,0x40,0x30,0x40,0x3C], // 119 'w'
    [0x44,0x28,0x10,0x28,0x44], // 120 'x'
    [0x0C,0x50,0x50,0x50,0x3C], // 121 'y'
    [0x44,0x64,0x54,0x4C,0x44], // 122 'z'
    [0x00,0x08,0x36,0x41,0x00], // 123 '{'
    [0x00,0x00,0x7F,0x00,0x00], // 124 '|'
    [0x00,0x41,0x36,0x08,0x00], // 125 '}'
    [0x08,0x08,0x2A,0x1C,0x08], // 126 '~'
];

const CHAR_W: usize = 6; // 5 glyph + 1 space
const CHAR_H: usize = 8; // 7 glyph + 1 space

// ---------------------------------------------------------------------------
// Crawl text
// ---------------------------------------------------------------------------

// Each line is padded / centered by the renderer based on the visible width.
static CRAWL: &[&[u8]] = &[
    b"",
    b"",
    b"",
    b"A long time ago",
    b"in a galaxy",
    b"far, far away....",
    b"",
    b"",
    b"",
    b"",
    b"",
    b"",
    b"S T A R",
    b"W A R S",
    b"",
    b"",
    b"Episode IV",
    b"",
    b"A NEW HOPE",
    b"",
    b"",
    b"It is a period of",
    b"civil war. Rebel",
    b"spaceships, striking",
    b"from a hidden base,",
    b"have won their first",
    b"victory against the",
    b"evil Galactic Empire.",
    b"",
    b"During the battle,",
    b"Rebel spies managed",
    b"to steal secret",
    b"plans to the",
    b"Empire's ultimate",
    b"weapon, the DEATH",
    b"STAR, an armored",
    b"space station with",
    b"enough power to",
    b"destroy an entire",
    b"planet.",
    b"",
    b"Pursued by the",
    b"Empire's sinister",
    b"agents, Princess",
    b"Leia races home",
    b"aboard her starship,",
    b"custodian of the",
    b"stolen plans that",
    b"can save her people",
    b"and restore freedom",
    b"to the galaxy....",
    b"",
    b"",
    b"",
    b"",
    b"",
    b"",
    b"",
    b"",
    b"",
    b"",
];

// Max chars per line at full width: 160 / 6 = 26
const CHARS_PER_LINE: usize = WIDTH / CHAR_W;

// ---------------------------------------------------------------------------
// Perspective map: display row -> virtual text row offset
//
// Built so the bottom of the screen shows nearby text (1:1) and the top
// compresses many virtual rows into fewer display rows.
// ---------------------------------------------------------------------------

/// Precompute cumulative virtual-row offsets (8.8 fixed point internally,
/// stored as integer virtual rows). Index 0 = top of screen, 119 = bottom.
/// Value = how many virtual rows above the "bottom" this display row shows.
const fn build_perspective() -> [u16; HEIGHT] {
    let mut map = [0u16; HEIGHT];
    // Accumulate from bottom (y=119) to top (y=0).
    // The "speed" at which virtual rows accumulate increases towards the top.
    // scale(y) = (y + 16) / (HEIGHT + 16) ; dv/dy ~ 1/scale(y)
    let mut accum: u32 = 0;
    let mut y: i32 = (HEIGHT as i32) - 2;
    while y >= 0 {
        // Inverse scale * 256 for fixed-point accumulation.
        let inv_scale_256 = ((HEIGHT as u32 + 16) * 256) / (y as u32 + 16);
        accum += inv_scale_256;
        map[y as usize] = (accum >> 8) as u16;
        y -= 1;
    }
    map
}

static PERSPECTIVE: [u16; HEIGHT] = build_perspective();

// ---------------------------------------------------------------------------
// Star Wars crawl renderer
// ---------------------------------------------------------------------------

struct StarWarsCrawl {
    scroll: u16, // virtual row at the bottom of the screen
}

impl StarWarsCrawl {
    fn new() -> Self {
        Self { scroll: 0 }
    }
}

impl VideoHandler for StarWarsCrawl {
    fn advance_frame(&mut self) {
        // Scroll up by 6 virtual rows per frame (= 30 rows/s at 5 fps).
        self.scroll = self.scroll.wrapping_add(6);
    }

    fn render(&mut self, buf: &mut [u8], offset: usize) -> usize {
        let mut written = 0;
        let end = offset + buf.len();

        // Process scanline by scanline to avoid per-byte division.
        let mut display_y = offset / BYTES_PER_LINE;
        let mut line_start = display_y * BYTES_PER_LINE;

        while display_y < HEIGHT && line_start < end {
            let vrow_offset = PERSPECTIVE[display_y];
            let vy = self.scroll.wrapping_sub(vrow_offset);

            // Margins and brightness (computed once per scanline).
            let margin = ((HEIGHT - 1 - display_y) as u32 * 50 / HEIGHT as u32) as usize;
            let margin_bytes = margin * 2;
            let visible_w = WIDTH - 2 * margin;
            let brightness = if vrow_offset < 200 {
                255 - (vrow_offset as u32 * 200 / 200) as u8
            } else {
                55
            };

            // Precompute text line info for this scanline.
            let text_line = (vy as usize) / CHAR_H;
            let char_row = (vy as usize) % CHAR_H;
            let (line, pad_px) = if text_line < CRAWL.len() {
                let l = CRAWL[text_line];
                let lc = l.len();
                let pp = if CHARS_PER_LINE > lc {
                    (CHARS_PER_LINE - lc) / 2 * CHAR_W
                } else {
                    0
                };
                (l, pp)
            } else {
                (b"" as &[u8], 0)
            };
            let line_px = line.len() * CHAR_W;

            // Iterate over bytes in this scanline that overlap [offset, end).
            let row_start = if line_start >= offset { 0 } else { offset - line_start };
            let row_end = BYTES_PER_LINE.min(end - line_start);

            let mut lb = row_start;
            while lb < row_end {
                let val = if lb < margin_bytes || lb >= BYTES_PER_LINE - margin_bytes {
                    if lb & 1 == 0 { 16 } else { 128 }
                } else if lb & 1 != 0 {
                    128 // chrominance = neutral
                } else {
                    // Y sample in visible area.
                    let visible_byte = lb - margin_bytes;
                    let px = visible_byte / 2;
                    let vx = if visible_w > 0 {
                        px * WIDTH / visible_w
                    } else {
                        0
                    };
                    // Inline text_pixel logic to avoid function call overhead.
                    if line_px == 0 || vx < pad_px || vx >= pad_px + line_px {
                        16
                    } else {
                        let tx = vx - pad_px;
                        let ci = tx / CHAR_W;
                        let cc = tx % CHAR_W;
                        if ci < line.len() && cc < 5 && char_row < 7 {
                            let ch = line[ci];
                            let idx = if (0x20..=0x7E).contains(&ch) {
                                (ch - 0x20) as usize
                            } else {
                                0
                            };
                            if (FONT_5X7[idx][cc] >> char_row) & 1 != 0 {
                                16 + ((219u16 * brightness as u16) >> 8) as u8
                            } else {
                                16
                            }
                        } else {
                            16
                        }
                    }
                };
                buf[written] = val;
                written += 1;
                lb += 1;
            }

            display_y += 1;
            line_start += BYTES_PER_LINE;
        }

        written
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

efm32hg322_usb::usb_device!(VideoClass<StarWarsCrawl>);

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    p.wdog.ctrl().write(|w| w.en().clear_bit());

    let class = VideoClass::new(StarWarsCrawl::new());
    let dev = UsbDevice::init(&p.cmu, &p.usb, class, video::usb_config());

    defmt::info!("USB Video device ready - connect cable now");
    usb_start(dev);
    efm32hg322_usb::idle();
}
