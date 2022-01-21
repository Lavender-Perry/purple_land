use crate::wasm4;
use rand::prelude::*;
use rand_pcg::Pcg64;

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Area {
    pos: Point,
    width: u32,
    height: u32,
}

impl Area {
    /// Moves the object on the x-axis by amt (amt < 0 is left, amt > 0 is right).
    /// Returns true if the object was moved, false if it is collided with something.
    fn move_x(&mut self, amt: i32) -> bool {
        self.pos.x += amt;
        if self.pos.x < 0 {
            self.pos.x = 0;
            return false;
        }
        let upper_limit = 160 - self.width as i32;
        if self.pos.x > upper_limit {
            self.pos.x = upper_limit;
            return false;
        }
        return true;
    }

    /// Moves the object on the y-axis by amt (amt < 0 is up, amt > 0 is down)
    /// Returns true if the object was moved, false if it is collided with something.
    fn move_y(&mut self, amt: i32) -> bool {
        self.pos.y += amt;
        if self.pos.y < 0 {
            self.pos.y = 0;
            return false;
        }
        let upper_limit = 160 - self.height as i32;
        if self.pos.y > upper_limit {
            self.pos.y = upper_limit;
            return false;
        }
        return true;
    }

    /// Returns if self overlaps sr (another Area)
    fn overlaps(&self, sr: &Area) -> bool {
        let x_diff = self.pos.x - sr.pos.x;
        let x_overlaps = if x_diff >= 0 {
            x_diff <= sr.width as i32
        } else {
            x_diff > -(self.width as i32)
        };

        let y_diff = self.pos.y - sr.pos.y;
        let y_overlaps = if y_diff >= 0 {
            y_diff <= sr.height as i32
        } else {
            y_diff > -(self.height as i32)
        };

        x_overlaps && y_overlaps
    }

    /// Draws the area to the screen as a rectangle
    fn draw(&self) {
        wasm4::rect(self.pos.x, self.pos.y, self.width, self.height);
    }
}

struct Projectile {
    area: Area,
    direction: Point,
    max_bounces: usize,
    bounce_amount: usize,
}

impl Projectile {
    fn new(area: Area, direction: Point, max_bounces: usize) -> Self {
        Self {
            area,
            direction,
            max_bounces,
            bounce_amount: 0,
        }
    }

    /// Moves the projectile in its direction, bouncing if it collides.
    /// Returns if it has reached max bounces.
    fn update(&mut self) -> bool {
        // Move x if needed, update direction & bounce amount if it bounced.
        if self.direction.x != 0 && !self.area.move_x(self.direction.x) {
            self.direction.x = -self.direction.x;
            self.bounce_amount += 1;
        }
        // Move y if needed, update direction & bounce amount if it bounced.
        if self.direction.y != 0 && !self.area.move_y(self.direction.y) {
            self.direction.y = -self.direction.y;
            self.bounce_amount += 1;
        }

        self.bounce_amount > self.max_bounces
    }
}

pub struct Game {
    player: Area,
    score: u8,
    high_score: u8,
    projectiles: Vec<Projectile>,
    frame_count: usize,
    rng: Pcg64,
    over: bool,
    restart: bool,
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
            player: Area {
                pos: Point { x: 69, y: 69 },
                width: 16,
                height: 16,
            },
            score: 0,
            high_score,
            projectiles: Vec::new(),
            frame_count: 0,
            rng: Pcg64::seed_from_u64(69420),
            over: false,
            restart: false,
        }
    }

    /// Updates game state, draws required items.
    pub fn update(&mut self) {
        if self.over {
            if self.score > self.high_score {
                self.high_score = self.score;
                unsafe { wasm4::diskw(self.score.to_le_bytes().as_ptr(), 1); }
            }

            wasm4::text(
                format!(
                    "Your score: {}\nHigh score: {}\nPress any button\nto restart.",
                    self.score,
                    self.high_score
                ),
                20, 20
            );

            if unsafe { *wasm4::GAMEPAD1 } == 0 {
                if !self.restart {
                    self.restart = true;
                }
            } else if self.restart {
                *self = Game::new();
            }

            return;
        }

        self.frame_count += 1;

        self.handle_input();

        if self.frame_count % 60 == 0 {
            let mut area = self.player;
            while area.overlaps(&self.player) {
                let width = self.rng.gen_range(1..10);
                let height = self.rng.gen_range(1..10);
                area = Area {
                    pos: Point {
                        x: self.rng.gen_range(0..160 - width),
                        y: self.rng.gen_range(0..160 - height),
                    },
                    width: width as u32,
                    height: height as u32,
                };
            }

            let x = self.rng.gen_range(-2..2);

            self.projectiles.push(Projectile::new(
                area,
                Point {
                    x,
                    y: if x == 0 {
                        if self.rng.gen() {
                            self.rng.gen_range(1..2)
                        } else {
                            self.rng.gen_range(-2..-1)
                        }
                    } else {
                        self.rng.gen_range(-2..2)
                    },
                },
                self.rng.gen_range(0..5),
            ));

            self.score += 1;
        }

        let mut i = 0;
        while i < self.projectiles.len() {
            if self.projectiles[i].update() {
                self.projectiles.swap_remove(i);
                continue;
            }

            if self.projectiles[i].area.overlaps(&self.player) { // Game over
                self.over = true;
                return;
            }

            self.projectiles[i].area.draw();
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
            self.player.move_y(-1);
        }
        if gamepad & wasm4::BUTTON_DOWN != 0 {
            self.player.move_y(1);
        }
        if gamepad & wasm4::BUTTON_LEFT != 0 {
            self.player.move_x(-1);
        }
        if gamepad & wasm4::BUTTON_RIGHT != 0 {
            self.player.move_x(1);
        }
    }
}
