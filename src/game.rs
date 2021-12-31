mod sprites;
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
}

trait Drawable {
    /// Draws the object to the screen
    fn draw(&self);
}

struct Sprite {
    area: Area,
    flags: u32,
    sprite: Vec<u8>,
}

impl Drawable for Sprite {
    fn draw(&self) {
        wasm4::blit(
            &self.sprite,
            self.area.pos.x,
            self.area.pos.y,
            self.area.width,
            self.area.height,
            self.flags,
        );
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

impl Drawable for Projectile {
    fn draw(&self) {
        wasm4::rect(
            self.area.pos.x,
            self.area.pos.y,
            self.area.width,
            self.area.height,
        );
    }
}

pub struct Game {
    player: Sprite,
    score: usize,
    projectiles: Vec<Projectile>,
    frame_count: usize,
    rng: Pcg64,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Sprite {
                area: Area {
                    pos: Point { x: 0, y: 0 },
                    width: 16,
                    height: 16,
                },
                flags: wasm4::BLIT_2BPP,
                sprite: sprites::TEST_PLAYER.to_vec(),
            },
            score: 0,
            projectiles: Vec::new(),
            frame_count: 0,
            rng: Pcg64::seed_from_u64(69420),
        }
    }

    /// Updates game state, draws required items.
    pub fn update(&mut self) {
        self.frame_count += 1;

        self.handle_input();

        if self.frame_count % 60 == 0 {
            let mut area = self.player.area;
            while area.overlaps(&self.player.area) {
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
            if self.projectiles[i].area.overlaps(&self.player.area) {
                *self = Game::new();
                return;
            }

            self.projectiles[i].draw();
            i += 1;
        }

        let prev_draw_colors = unsafe { *wasm4::DRAW_COLORS };
        unsafe {
            *wasm4::DRAW_COLORS = 0x42;
        }
        wasm4::text(&self.score.to_string(), 0, 0);
        unsafe {
            *wasm4::DRAW_COLORS = prev_draw_colors;
        }

        self.player.draw();
    }

    /// Takes required actions depending on state of gamepad
    fn handle_input(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };

        if gamepad & wasm4::BUTTON_UP != 0 {
            self.player.area.move_y(-1);
        }
        if gamepad & wasm4::BUTTON_DOWN != 0 {
            self.player.area.move_y(1);
        }
        if gamepad & wasm4::BUTTON_LEFT != 0 {
            self.player.area.move_x(-1);
        }
        if gamepad & wasm4::BUTTON_RIGHT != 0 {
            self.player.area.move_x(1);
        }
    }
}
