//! Display a greeting and simple geometric patterns on the Sharp Memory LCD.

#![no_main]
#![no_std]

use cortex_m as _;
use cortex_m_rt as _;
use defmt_rtt as _;
use panic_halt as _;

use efm32hg322_hal as hal;
use efm32hg322_pac as pac;
use hal::clocks::enable_gpio_clock;
use hal::watchdog::WatchdogExt;
use slstk3400a::display;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    p.wdog.constrain().disable();
    enable_gpio_clock();

    // Initialise the Sharp Memory LCD (USART1 SPI + GPIO).
    display::init();

    let mut vcom = false;

    // Clear to white.
    display::clear(&mut vcom);

    // Draw a title.
    display::draw_text(1, 1, "Hello, World!", &mut vcom);
    display::draw_text(2, 1, "EFM32 Happy", &mut vcom);
    display::draw_text(3, 1, "Gecko SLSTK3400A", &mut vcom);

    // Draw a horizontal rule.
    display::fill_rect(8, 36, 112, 2, true, &mut vcom);

    // Draw a checkerboard pattern in the lower half.
    let mut rows = [[0xFFu8; display::BYTES_PER_ROW]; 8];
    for block_row in 0..5 {
        for row in &mut rows {
            for (bx, pixel) in row.iter_mut().enumerate() {
                // 8x8 checkerboard: alternate black/white per 8-pixel block.
                let checker = ((block_row + bx) & 1) != 0;
                *pixel = if checker { 0x00 } else { 0xFF };
            }
        }
        let start = 48 + block_row as u8 * 8;
        display::write_rows(start, &rows, &mut vcom);
    }

    // Draw a border rectangle outline (1-pixel lines).
    for x in 0..128u8 {
        // top
        set_pixel_row(x, 96, true, &mut vcom);
        // bottom
        set_pixel_row(x, 123, true, &mut vcom);
    }
    for y in 96..124u8 {
        // left
        set_pixel_row(0, y, true, &mut vcom);
        // right
        set_pixel_row(127, y, true, &mut vcom);
    }

    // Label inside the border.
    display::draw_text(13, 3, "Sharp LCD", &mut vcom);
    display::draw_text(14, 3, "128 x 128", &mut vcom);

    defmt::info!("LCD hello demo running");

    // Main loop - toggle VCOM at ~1 Hz to keep the display healthy.
    loop {
        cortex_m::asm::delay(14_000_000); // ~1 s at 14 MHz
        display::toggle_vcom();
    }
}

/// Helper: write a single black or white pixel by updating its entire row.
fn set_pixel_row(x: u8, y: u8, black: bool, vcom: &mut bool) {
    // Read-modify-write is not possible without a framebuffer, so we
    // write a full row with just the one pixel set.  For border lines
    // this is called once per row so it works fine.
    let mut row = [0xFFu8; display::BYTES_PER_ROW]; // white
    if black {
        row[(x / 8) as usize] &= !(1 << (x % 8));
    }
    display::write_row(y, &row, vcom);
}
