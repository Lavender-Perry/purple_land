mod sprites;

use crate::wasm4;

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

struct ScreenRegion {
    pos: Point,
    width: u32,
    height: u32,
}

impl ScreenRegion {
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
}

trait Drawable {
    /// Draws the object to the screen
    fn draw(&self);
}

struct Sprite {
    screen_region: ScreenRegion,
    flags: u32,
    sprite: Vec<u8>,
}

impl Drawable for Sprite {
    fn draw(&self) {
        wasm4::blit(
            &self.sprite,
            self.screen_region.pos.x,
            self.screen_region.pos.y,
            self.screen_region.width,
            self.screen_region.height,
            self.flags,
        );
    }
}

struct Projectile {
    screen_region: ScreenRegion,
    direction: Point,
    max_bounces: u32,
    bounce_amount: u32,
}

impl Projectile {
    fn new(
        direction: Point,
        pos: Point,
        width: Option<u32>,
        height: Option<u32>,
        max_bounces: Option<u32>,
    ) -> Self {
        Self {
            screen_region: ScreenRegion {
                pos,
                width: width.unwrap_or(3),
                height: height.unwrap_or(3),
            },
            direction,
            max_bounces: max_bounces.unwrap_or(3),
            bounce_amount: 0,
        }
    }

    /// Moves the projectile in its direction, bouncing if it collides.
    /// Returns if it has reached max bounces.
    fn update(&mut self) -> bool {
        // Move x if needed, update direction & bounce amount if it bounced.
        if self.direction.x != 0 && !self.screen_region.move_x(self.direction.x) {
            self.direction.x = -self.direction.x;
            self.bounce_amount += 1;
        }
        // Move y if needed, update direction & bounce amount if it bounced.
        if self.direction.y != 0 && !self.screen_region.move_y(self.direction.y) {
            self.direction.y = -self.direction.y;
            self.bounce_amount += 1;
        }

        self.bounce_amount > self.max_bounces
    }
}

impl Drawable for Projectile {
    fn draw(&self) {
        wasm4::rect(
            self.screen_region.pos.x,
            self.screen_region.pos.y,
            self.screen_region.width,
            self.screen_region.height,
        );
    }
}

pub struct Game {
    player: Sprite,
    projectiles: Vec<Projectile>,
    frame_count: usize,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Sprite {
                screen_region: ScreenRegion {
                    pos: Point { x: 0, y: 0 },
                    width: 16,
                    height: 16,
                },
                flags: wasm4::BLIT_2BPP,
                sprite: sprites::TEST_PLAYER.to_vec(),
            },
            projectiles: vec![
                Projectile::new(
                    Point { x: 1, y: 1 },
                    Point { x: 81, y: 81 },
                    None,
                    None,
                    None,
                ),
                Projectile::new(
                    Point { x: 1, y: -1 },
                    Point { x: 81, y: 80 },
                    None,
                    None,
                    None,
                ),
                Projectile::new(
                    Point { x: -1, y: 1 },
                    Point { x: 80, y: 81 },
                    None,
                    None,
                    None,
                ),
                Projectile::new(
                    Point { x: -1, y: -1 },
                    Point { x: 80, y: 80 },
                    None,
                    None,
                    None,
                ),
            ],
            frame_count: 0,
        }
    }

    /// Updates game state, draws required items.
    pub fn update(&mut self) {
        self.frame_count += 1;

        self.handle_input();

        let mut i = 0;
        while i < self.projectiles.len() {
            if self.projectiles[i].update() {
                self.projectiles.swap_remove(i);
                continue;
            }
            self.projectiles[i].draw();
            i += 1;
        }

        self.player.draw();
    }

    /// Takes required actions depending on state of gamepad
    fn handle_input(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };

        if gamepad & wasm4::BUTTON_UP != 0 {
            self.player.screen_region.move_y(-1);
        }
        if gamepad & wasm4::BUTTON_DOWN != 0 {
            self.player.screen_region.move_y(1);
        }
        if gamepad & wasm4::BUTTON_LEFT != 0 {
            self.player.screen_region.move_x(-1);
        }
        if gamepad & wasm4::BUTTON_RIGHT != 0 {
            self.player.screen_region.move_x(1);
        }
    }
}
