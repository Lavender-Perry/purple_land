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
                // TODO: put sprite arrays in a seperate file
            }
        }
    }

    /// Updates game state, draws required items.
    pub fn update(&self) {
        self.player.draw();
    }
}
