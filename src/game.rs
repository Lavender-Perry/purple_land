use crate::wasm4;
use rand::prelude::*;
use rand_pcg::Pcg64;

mod basics;
mod projectile;
use self::basics::{Point, Square};
use self::projectile::Projectile;

const PLAYER_SIZE: u32 = 16;

pub struct Game {
    player: Square,
    score: u8,
    high_score: u8,
    projectiles: Vec<Projectile>,
    frame_count: u64,
    rng: Pcg64,
    pre_game: bool,
    over: bool,
    btn_pressed: bool,
}

impl Game {
    pub fn new() -> Self {
        let high_score = unsafe {
            let mut buf = [0u8; 1];
            let read_amt = wasm4::diskr(buf.as_mut_ptr(), 1);
            if read_amt < 1 {
                0
            } else {
                u8::from_le_bytes(buf)
            }
        };

        Self {
            player: Square {
                pos: Point { x: 69, y: 69 },
                size: PLAYER_SIZE,
            },
            score: 0,
            high_score,
            projectiles: Vec::new(),
            frame_count: 0,
            rng: Pcg64::seed_from_u64(0), // Temporary value
            pre_game: true,
            over: false,
            btn_pressed: false,
        }
    }

    /// Updates game state, draws required items.
    pub fn update(&mut self) {
        if self.over {
            // Do game over screen
            if self.score > self.high_score {
                // Update high score
                self.high_score = self.score;
                unsafe {
                    wasm4::diskw(self.score.to_le_bytes().as_ptr(), 1);
                }
            }

            // Not using format!() here to reduce cart size
            wasm4::text(
                "Your score:\nHigh score:\nPress a button\nto restart",
                20,
                20,
            );
            wasm4::text(&self.score.to_string(), 110, 20);
            wasm4::text(&self.high_score.to_string(), 110, 28);

            if unsafe { *wasm4::GAMEPAD1 } == 0 {
                if !self.btn_pressed {
                    self.btn_pressed = true;
                }
            } else if self.btn_pressed {
                *self = Game::new();
            }

            return;
        }

        self.frame_count += 1;

        if self.pre_game {
            wasm4::text("Press any button\nto start", 20, 20);
            if unsafe { *wasm4::GAMEPAD1 } == 0 {
                if !self.btn_pressed {
                    self.btn_pressed = true;
                }
            } else if self.btn_pressed {
                self.pre_game = false;
                self.rng = Pcg64::seed_from_u64(self.frame_count);
                self.btn_pressed = false;
            }
            return;
        }

        self.handle_input();

        if self.frame_count % 120 == 0 {
            self.projectiles.push(Projectile::new(&mut self.rng));
            self.score += 1;
        }

        let mut i = 0;
        while i < self.projectiles.len() {
            if self.projectiles[i].mv() {
                // Game over
                self.over = true;
                return;
            }
            self.projectiles[i].update(&self.player);
            i += 1;
        }

        let prev_draw_colors = unsafe { *wasm4::DRAW_COLORS };
        unsafe {
            *wasm4::DRAW_COLORS = 0x42;
        }

        // Draw score
        wasm4::text(&self.score.to_string(), 0, 0);
        // Draw player
        self.player.draw();

        unsafe {
            *wasm4::DRAW_COLORS = prev_draw_colors;
        }
    }

    /// Takes required actions depending on state of gamepad
    fn handle_input(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };

        if gamepad & wasm4::BUTTON_UP != 0 {
            self.player.move_y(-2);
        }
        if gamepad & wasm4::BUTTON_DOWN != 0 {
            self.player.move_y(2);
        }
        if gamepad & wasm4::BUTTON_LEFT != 0 {
            self.player.move_x(-2);
        }
        if gamepad & wasm4::BUTTON_RIGHT != 0 {
            self.player.move_x(2);
        }
    }
}
