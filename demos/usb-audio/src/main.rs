//! USB Audio device demo for the SLSTK3400A.
//!
//! Streams "Twinkle Twinkle Little Star" as 8 kHz 16-bit mono PCM
//! via isochronous IN transfers. Connect to any host and open an
//! audio capture application to hear the melody.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use efm32hg322_pac as pac;
use efm32hg322_usb::audio::{self, AudioClass, AudioHandler, SAMPLES_PER_FRAME, SAMPLE_RATE};
use efm32hg322_usb::UsbDevice;

// ---------------------------------------------------------------------------
// Sine wave table (64 entries, full-scale i16)
// ---------------------------------------------------------------------------

#[rustfmt::skip]
const SINE: [i16; 64] = [
        0,  3212,  6393,  9512, 12540, 15447, 18205, 20788,
    23170, 25330, 27246, 28899, 30274, 31357, 32138, 32610,
    32767, 32610, 32138, 31357, 30274, 28899, 27246, 25330,
    23170, 20788, 18205, 15447, 12540,  9512,  6393,  3212,
        0, -3212, -6393, -9512,-12540,-15447,-18205,-20788,
   -23170,-25330,-27246,-28899,-30274,-31357,-32138,-32610,
   -32767,-32610,-32138,-31357,-30274,-28899,-27246,-25330,
   -23170,-20788,-18205,-15447,-12540, -9512, -6393, -3212,
];

// ---------------------------------------------------------------------------
// Phase increment lookup (16.16 fixed-point, 64-entry table, 8 kHz)
//
// increment = freq_hz * 64 * 65536 / 8000
// ---------------------------------------------------------------------------

const fn phase_inc(freq_hz: u32) -> u32 {
    (freq_hz as u64 * 64 * 65536 / 8000) as u32
}

const fn note_inc(midi_note: u8) -> u32 {
    match midi_note {
        60 => phase_inc(262), // C4
        62 => phase_inc(294), // D4
        64 => phase_inc(330), // E4
        65 => phase_inc(349), // F4
        67 => phase_inc(392), // G4
        69 => phase_inc(440), // A4
        _ => 0,
    }
}

// ---------------------------------------------------------------------------
// Melody (same as MIDI demo)
// ---------------------------------------------------------------------------

#[rustfmt::skip]
static TUNE: [(u8, u32); 42] = [
    (60, 400), (60, 400), (67, 400), (67, 400),
    (69, 400), (69, 400), (67, 800),
    (65, 400), (65, 400), (64, 400), (64, 400),
    (62, 400), (62, 400), (60, 800),
    (67, 400), (67, 400), (65, 400), (65, 400),
    (64, 400), (64, 400), (62, 800),
    (67, 400), (67, 400), (65, 400), (65, 400),
    (64, 400), (64, 400), (62, 800),
    (60, 400), (60, 400), (67, 400), (67, 400),
    (69, 400), (69, 400), (67, 800),
    (65, 400), (65, 400), (64, 400), (64, 400),
    (62, 400), (62, 400), (60, 800),
];

// ---------------------------------------------------------------------------
// Synthesizer
// ---------------------------------------------------------------------------

struct TwinkleSynth {
    tune_pos: usize,
    samples_in_note: u32,
    phase: u32, // 16.16 fixed-point
}

impl TwinkleSynth {
    fn new() -> Self {
        Self {
            tune_pos: 0,
            samples_in_note: 0,
            phase: 0,
        }
    }
}

impl AudioHandler for TwinkleSynth {
    fn fill_frame(&mut self, buf: &mut [i16; SAMPLES_PER_FRAME]) {
        for sample in buf.iter_mut() {
            let (note, dur_ms) = TUNE[self.tune_pos];
            let dur_samples = dur_ms * (SAMPLE_RATE / 1000);
            let inc = note_inc(note);

            *sample = if inc > 0 {
                // Attenuate to ~50 % to avoid clipping distortion.
                SINE[((self.phase >> 16) & 63) as usize] / 2
            } else {
                0
            };

            self.phase = self.phase.wrapping_add(inc);
            self.samples_in_note += 1;

            if self.samples_in_note >= dur_samples {
                self.samples_in_note = 0;
                self.tune_pos = (self.tune_pos + 1) % TUNE.len();
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

efm32hg322_usb::usb_device!(AudioClass<TwinkleSynth>);

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    p.wdog.ctrl().write(|w| w.en().clear_bit());

    let class = AudioClass::new(TwinkleSynth::new());
    let dev = UsbDevice::init(&p.cmu, &p.usb, class, audio::usb_config());

    defmt::info!("USB Audio synth ready - connect cable now");
    usb_start(dev);
    efm32hg322_usb::idle();
}
