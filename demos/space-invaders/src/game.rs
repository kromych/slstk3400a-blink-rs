//! Game state, update logic, and collision detection for Space Invaders.

use crate::render;
use crate::rng::Rng;
use crate::sprites;

// --- Layout constants ---

/// Top of the playfield (below the HUD row).
const PLAY_Y: u8 = 10;
/// Player Y position (bottom area).
const PLAYER_Y: u8 = 116;
/// Player ship width in pixels.
const PLAYER_W: u8 = 16;
/// Movement speed in pixels per tick.
const PLAYER_SPEED: u8 = 2;

/// Number of invader columns.
const INV_COLS: usize = 8;
/// Number of invader rows.
const INV_ROWS: usize = 5;
/// Pixel pitch between invaders (sprite 8px + 2px gap).
const INV_PITCH: u8 = 10;
/// Invader grid starting X.
const INV_START_X: u8 = 24;
/// Invader grid starting Y.
const INV_START_Y: u8 = PLAY_Y + 4;
/// How many pixels invaders descend per edge hit.
const INV_DROP: u8 = 6;

/// Maximum active player bullets.
const MAX_P_BULLETS: usize = 3;
/// Maximum active invader bullets.
const MAX_I_BULLETS: usize = 3;
/// Bullet speed in pixels per tick.
const BULLET_SPEED: u8 = 3;
/// Auto-fire interval in ticks (~1.3s at 15 Hz).
const AUTO_FIRE_INTERVAL: u8 = 20;
/// Frames to show explosion.
const EXPLOSION_FRAMES: u8 = 4;
/// Invader fire probability: 1/N chance per tick when eligible.
const INVADER_FIRE_CHANCE: u16 = 30;

/// Player starting lives.
const STARTING_LIVES: u8 = 3;
/// Invincibility frames after getting hit.
const HIT_INVINCIBILITY: u8 = 30;

#[derive(Clone, Copy)]
pub struct Bullet {
    pub x: u8,
    pub y: u8,
    pub active: bool,
}

impl Bullet {
    const fn inactive() -> Self {
        Self {
            x: 0,
            y: 0,
            active: false,
        }
    }
}

#[derive(Clone, Copy)]
struct Explosion {
    x: u8,
    y: u8,
    timer: u8,
}

pub struct GameState {
    pub player_x: u8,
    pub fire_cooldown: u8,
    pub invincible: u8,

    /// Bitmask per row: bit N = column N alive.
    pub inv_alive: [u8; INV_ROWS],
    pub inv_x: u8,
    pub inv_y: u8,
    pub inv_dx: i8,
    pub inv_phase: u8,
    pub inv_move_counter: u8,
    pub inv_move_speed: u8,
    pub inv_alive_count: u8,

    pub p_bullets: [Bullet; MAX_P_BULLETS],
    pub i_bullets: [Bullet; MAX_I_BULLETS],

    pub score: u16,
    pub lives: u8,
    pub game_over: bool,
    pub wave: u8,

    pub rng: Rng,
    pub frame: u16,

    explosions: [Explosion; 4],
}

impl GameState {
    pub fn new(seed: u16) -> Self {
        Self {
            player_x: 56,
            fire_cooldown: 0,
            invincible: 0,

            inv_alive: [0xFF; INV_ROWS], // all 8 columns alive in each row
            inv_x: INV_START_X,
            inv_y: INV_START_Y,
            inv_dx: 1,
            inv_phase: 0,
            inv_move_counter: 0,
            inv_move_speed: 10,
            inv_alive_count: (INV_ROWS * INV_COLS) as u8,

            p_bullets: [Bullet::inactive(); MAX_P_BULLETS],
            i_bullets: [Bullet::inactive(); MAX_I_BULLETS],

            score: 0,
            lives: STARTING_LIVES,
            game_over: false,
            wave: 1,

            rng: Rng::new(seed),
            frame: 0,

            explosions: [Explosion {
                x: 0,
                y: 0,
                timer: 0,
            }; 4],
        }
    }

    /// Process one game tick. Returns true if the game is still active.
    pub fn update(&mut self, btn_left: bool, btn_right: bool, btn_fire: bool) -> bool {
        if self.game_over {
            return false;
        }
        self.frame = self.frame.wrapping_add(1);

        // --- Player movement ---
        if btn_left && self.player_x >= PLAYER_SPEED {
            self.player_x -= PLAYER_SPEED;
        }
        if btn_right && self.player_x + PLAYER_W + PLAYER_SPEED <= 128 {
            self.player_x += PLAYER_SPEED;
        }

        // --- Player firing ---
        if self.fire_cooldown > 0 {
            self.fire_cooldown -= 1;
        }
        let should_fire = btn_fire || self.fire_cooldown == 0;
        if should_fire && self.fire_cooldown == 0 {
            self.fire_player_bullet();
            self.fire_cooldown = AUTO_FIRE_INTERVAL;
        }

        // --- Move player bullets ---
        for b in self.p_bullets.iter_mut() {
            if b.active {
                if b.y < BULLET_SPEED + PLAY_Y {
                    b.active = false;
                } else {
                    b.y -= BULLET_SPEED;
                }
            }
        }

        // --- Move invader bullets ---
        for b in self.i_bullets.iter_mut() {
            if b.active {
                let new_y = b.y as u16 + BULLET_SPEED as u16;
                if new_y >= 128 {
                    b.active = false;
                } else {
                    b.y = new_y as u8;
                }
            }
        }

        // --- Move invaders ---
        self.inv_move_counter += 1;
        if self.inv_move_counter >= self.inv_move_speed {
            self.inv_move_counter = 0;
            self.inv_phase ^= 1;
            self.step_invaders();
        }

        // --- Invader firing ---
        if self.rng.next_bounded(INVADER_FIRE_CHANCE) == 0 {
            self.fire_invader_bullet();
        }

        // --- Collision: player bullets vs invaders ---
        for i in 0..MAX_P_BULLETS {
            if !self.p_bullets[i].active {
                continue;
            }
            let bx = self.p_bullets[i].x;
            let by = self.p_bullets[i].y;
            if let Some((row, col)) = self.hit_invader(bx, by) {
                self.inv_alive[row] &= !(1 << col);
                self.inv_alive_count -= 1;
                self.p_bullets[i].active = false;

                // Score: top rows are worth more.
                self.score += match row {
                    0 => 30,
                    1 => 20,
                    _ => 10,
                };

                // Spawn explosion.
                self.add_explosion(
                    self.inv_x + col as u8 * INV_PITCH,
                    self.inv_y + row as u8 * INV_PITCH,
                );

                // Speed up invaders as they die.
                self.inv_move_speed = self.calc_invader_speed();
            }
        }

        // --- Collision: invader bullets vs player ---
        if self.invincible > 0 {
            self.invincible -= 1;
        } else {
            for i in 0..MAX_I_BULLETS {
                if !self.i_bullets[i].active {
                    continue;
                }
                // Player hitbox: (player_x, PLAYER_Y) size (PLAYER_W, 8).
                let bx = self.i_bullets[i].x;
                let by = self.i_bullets[i].y;
                if bx >= self.player_x
                    && bx < self.player_x + PLAYER_W
                    && by + 4 > PLAYER_Y
                    && by < PLAYER_Y + 8
                {
                    self.i_bullets[i].active = false;
                    self.lives -= 1;
                    self.invincible = HIT_INVINCIBILITY;
                    if self.lives == 0 {
                        self.game_over = true;
                        return false;
                    }
                }
            }
        }

        // --- Check if invaders reached player ---
        let inv_bottom = self.inv_y as u16 + self.bottom_alive_row() as u16 * INV_PITCH as u16 + 8;
        if inv_bottom >= PLAYER_Y as u16 {
            self.game_over = true;
            return false;
        }

        // --- Check wave cleared ---
        if self.inv_alive_count == 0 {
            self.next_wave();
        }

        // --- Tick explosions ---
        for exp in self.explosions.iter_mut() {
            if exp.timer > 0 {
                exp.timer -= 1;
            }
        }

        true
    }

    /// Draw the current game state into the framebuffer.
    pub fn draw(&self, vcom: &mut bool) {
        render::fb_clear();

        // HUD.
        self.draw_hud();

        // Invaders.
        self.draw_invaders();

        // Player (blink during invincibility).
        if self.invincible == 0 || self.frame % 4 < 2 {
            render::fb_blit_16(self.player_x, PLAYER_Y, &sprites::PLAYER);
        }

        // Player bullets.
        for b in &self.p_bullets {
            if b.active {
                render::fb_blit_8(b.x, b.y, &sprites::BULLET_PLAYER);
            }
        }

        // Invader bullets.
        for b in &self.i_bullets {
            if b.active {
                render::fb_blit_8(b.x, b.y, &sprites::BULLET_INVADER);
            }
        }

        // Explosions.
        for exp in &self.explosions {
            if exp.timer > 0 {
                render::fb_blit_8(exp.x, exp.y, &sprites::EXPLOSION);
            }
        }

        // Game over overlay.
        if self.game_over {
            render::fb_draw_text(7, 3, "GAME  OVER");
        }

        render::fb_flush(vcom);
    }

    // --- Private helpers ---

    fn draw_hud(&self) {
        // "Snnn  Lx  Wn"
        let mut buf = [b' '; 16];
        buf[0] = b'S';

        // Score (up to 5 digits).
        let mut score_buf = [0u8; 10];
        let digits = slstk3400a::display::format_u32(self.score as u32, &mut score_buf);
        let start = 5 - digits.len().min(5);
        for (i, &d) in digits.iter().enumerate() {
            if 1 + start + i < 7 {
                buf[1 + start + i] = d;
            }
        }
        // Zero-pad.
        for i in 1..1 + start {
            buf[i] = b'0';
        }

        buf[8] = b'L';
        buf[9] = b'0' + self.lives.min(9);

        buf[12] = b'W';
        buf[13] = b'0' + self.wave.min(9);

        // SAFETY: buf is valid ASCII.
        let s = core::str::from_utf8(&buf).unwrap_or("");
        render::fb_draw_text(0, 0, s);
    }

    fn draw_invaders(&self) {
        for row in 0..INV_ROWS {
            let row_bits = self.inv_alive[row];
            if row_bits == 0 {
                continue;
            }
            let iy = self.inv_y + row as u8 * INV_PITCH;
            let (f1, f2) = match row {
                0 | 1 => (&sprites::INVADER_A1, &sprites::INVADER_A2),
                2 | 3 => (&sprites::INVADER_B1, &sprites::INVADER_B2),
                _ => (&sprites::INVADER_C1, &sprites::INVADER_C2),
            };
            let sprite = if self.inv_phase == 0 { f1 } else { f2 };
            for col in 0..INV_COLS {
                if row_bits & (1 << col) != 0 {
                    let ix = self.inv_x + col as u8 * INV_PITCH;
                    render::fb_blit_8(ix, iy, sprite);
                }
            }
        }
    }

    fn step_invaders(&mut self) {
        // Find leftmost and rightmost alive columns.
        let (left_col, right_col) = self.alive_column_range();
        let left_px = self.inv_x + left_col as u8 * INV_PITCH;
        let right_px = self.inv_x + right_col as u8 * INV_PITCH + 8;

        let next_x = (self.inv_x as i16) + self.inv_dx as i16;

        if (self.inv_dx > 0 && right_px >= 126) || (self.inv_dx < 0 && left_px <= 2) {
            // Reverse direction and drop.
            self.inv_dx = -self.inv_dx;
            self.inv_y += INV_DROP;
        } else {
            self.inv_x = next_x as u8;
        }
    }

    fn alive_column_range(&self) -> (usize, usize) {
        let mut combined = 0u8;
        for row in 0..INV_ROWS {
            combined |= self.inv_alive[row];
        }
        if combined == 0 {
            return (0, 0);
        }
        let mut left = 0;
        while left < INV_COLS && (combined & (1 << left)) == 0 {
            left += 1;
        }
        let mut right = INV_COLS - 1;
        while right > left && (combined & (1 << right)) == 0 {
            right -= 1;
        }
        (left, right)
    }

    fn bottom_alive_row(&self) -> usize {
        for row in (0..INV_ROWS).rev() {
            if self.inv_alive[row] != 0 {
                return row;
            }
        }
        0
    }

    fn hit_invader(&self, bx: u8, by: u8) -> Option<(usize, usize)> {
        // Check if bullet position overlaps any alive invader.
        if by < self.inv_y || bx < self.inv_x {
            return None;
        }
        let rel_y = by - self.inv_y;
        let rel_x = bx - self.inv_x;

        let row = (rel_y / INV_PITCH) as usize;
        let col = (rel_x / INV_PITCH) as usize;

        if row >= INV_ROWS || col >= INV_COLS {
            return None;
        }

        // Check pixel-level overlap (within the 8x8 sprite area).
        let in_sprite_y = rel_y % INV_PITCH;
        let in_sprite_x = rel_x % INV_PITCH;
        if in_sprite_y >= 8 || in_sprite_x >= 8 {
            return None;
        }

        if self.inv_alive[row] & (1 << col) != 0 {
            Some((row, col))
        } else {
            None
        }
    }

    fn fire_player_bullet(&mut self) {
        for b in self.p_bullets.iter_mut() {
            if !b.active {
                b.active = true;
                b.x = self.player_x + PLAYER_W / 2;
                b.y = PLAYER_Y - 4;
                return;
            }
        }
    }

    fn fire_invader_bullet(&mut self) {
        if self.inv_alive_count == 0 {
            return;
        }
        // Find a free bullet slot.
        let slot = self.i_bullets.iter_mut().find(|b| !b.active);
        let slot = match slot {
            Some(s) => s,
            None => return,
        };

        // Pick a random alive invader from the bottom row of each column.
        let target = self.rng.next_bounded(self.inv_alive_count as u16) as u8;
        let mut count = 0u8;
        for col in 0..INV_COLS {
            // Find bottommost alive invader in this column.
            for row in (0..INV_ROWS).rev() {
                if self.inv_alive[row] & (1 << col) != 0 {
                    if count == target {
                        slot.active = true;
                        slot.x = self.inv_x + col as u8 * INV_PITCH + 3;
                        slot.y = self.inv_y + row as u8 * INV_PITCH + 8;
                        return;
                    }
                    count += 1;
                    break; // Only count bottommost per column.
                }
            }
        }
    }

    fn calc_invader_speed(&self) -> u8 {
        // Fewer invaders → faster movement.
        if self.inv_alive_count <= 1 {
            1
        } else if self.inv_alive_count <= 5 {
            2
        } else if self.inv_alive_count <= 15 {
            4
        } else if self.inv_alive_count <= 25 {
            6
        } else {
            8
        }
    }

    fn next_wave(&mut self) {
        self.wave += 1;
        self.inv_alive = [0xFF; INV_ROWS];
        self.inv_alive_count = (INV_ROWS * INV_COLS) as u8;
        self.inv_x = INV_START_X;
        self.inv_y = INV_START_Y;
        self.inv_dx = 1;
        self.inv_move_counter = 0;
        // Each wave is slightly faster.
        self.inv_move_speed = 10u8.saturating_sub(self.wave.min(6));

        // Clear bullets.
        self.p_bullets = [Bullet::inactive(); MAX_P_BULLETS];
        self.i_bullets = [Bullet::inactive(); MAX_I_BULLETS];
    }

    fn add_explosion(&mut self, x: u8, y: u8) {
        for exp in self.explosions.iter_mut() {
            if exp.timer == 0 {
                exp.x = x;
                exp.y = y;
                exp.timer = EXPLOSION_FRAMES;
                return;
            }
        }
    }
}
