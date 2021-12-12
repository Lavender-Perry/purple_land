mod sprites;

use crate::wasm4;

trait Drawable {
    /// Draws self to screen
    fn draw(&self);
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Drawable for Point {
    fn draw(&self) {
        wasm4::rect(self.x, self.y, 1, 1);
    }
}

struct Sprite {
    pos: Point,
    width: u32,
    height: u32,
    flags: u32,
    sprite: Vec<u8>,
}

impl Sprite {
    /// Moves the sprite on the x-axis by amt (amt < 0 is left, amt > 0 is right)
    fn move_x(&mut self, amt: i32) {
        self.pos.x += amt;
        if self.pos.x < 0 {
            self.pos.x = 0;
        } else {
            let upper_limit = 160 - self.width as i32;
            if self.pos.x > upper_limit {
                self.pos.x = upper_limit;
            }
        }
    }

    /// Moves the sprite on the y-axis by amt (amt < 0 is up, amt > 0 is down)
    fn move_y(&mut self, amt: i32) {
        self.pos.y += amt;
        if self.pos.y < 0 {
            self.pos.y = 0;
        } else {
            let upper_limit = 160 - self.height as i32;
            if self.pos.y > upper_limit {
                self.pos.y = upper_limit;
            }
        }
    }
}

impl Drawable for Sprite {
    fn draw(&self) {
        wasm4::blit(
            &self.sprite,
            self.pos.x, self.pos.y,
            self.width, self.height,
            self.flags);
    }
}

pub struct Game {
    player: Sprite,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Sprite {
                pos: Point { x: 0, y: 0, },
                width: 16,
                height: 16,
                flags: wasm4::BLIT_2BPP,
                sprite: sprites::TEST_PLAYER.to_vec(),
            },
        }
    }

    /// Updates game state, draws required items.
    pub fn update(&mut self) {
        self.handle_input();
        self.player.draw();
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
