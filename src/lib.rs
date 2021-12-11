#[cfg(feature = "buddy-alloc")]
mod alloc;

mod game;
mod wasm4;

#[macro_use]
extern crate lazy_static;

use game::Game;
use std::sync::Mutex;

lazy_static! { static ref GAME: Mutex<Game> = Mutex::new(Game::new()); }

#[no_mangle]
fn start() {
    unsafe { *wasm4::PALETTE = [0xe8ccff, 0xcd8fff, 0xac47ff, 0x410075]; }
}

#[no_mangle]
fn update() {
    GAME.lock().expect("game state").update();
}
