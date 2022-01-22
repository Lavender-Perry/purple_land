#!/bin/sh
cart="target/wasm32-unknown-unknown/release/cart.wasm"
output="build/purple_land"
cargo build --release
wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code -o $cart $cart
wasm-opt -Oz -o $cart $cart
w4 bundle \
    --html $output-html.html \
    --windows $output-win.exe \
    --mac $output-mac \
    --linux $output-lin \
    --title "Purple Land" \
    --description "A game." \
    $cart
