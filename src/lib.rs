#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;

#[no_mangle]
fn start() {
    unsafe { *wasm4::PALETTE = [0xe8ccff, 0xcd8fff, 0xac47ff, 0x410075]; }
}

#[no_mangle]
fn update() {
    // Testing palette, remove later
    unsafe { *wasm4::DRAW_COLORS = 2; }
    wasm4::rect(0, 0, 60, 60);
    unsafe { *wasm4::DRAW_COLORS = 3; }
    wasm4::rect(61, 61, 60, 60);
    unsafe { *wasm4::DRAW_COLORS = 4; }
    wasm4::rect(61, 0, 60, 60);
}
