//! Simple 16-bit LFSR pseudo-random number generator.

pub struct Rng {
    state: u16,
}

impl Rng {
    pub const fn new(seed: u16) -> Self {
        // Avoid zero state (LFSR would get stuck).
        let state = if seed == 0 { 0xACE1 } else { seed };
        Self { state }
    }

    /// Return the next pseudo-random u16.
    pub fn next(&mut self) -> u16 {
        // Galois LFSR with taps at bits 16, 14, 13, 11 (maximal period 65535).
        let lsb = self.state & 1;
        self.state >>= 1;
        if lsb != 0 {
            self.state ^= 0xB400;
        }
        self.state
    }

    /// Return a random value in `0..limit` (exclusive).
    pub fn next_bounded(&mut self, limit: u16) -> u16 {
        self.next() % limit
    }
}
