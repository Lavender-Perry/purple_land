# Purple Land
A game for the WASM-4 fantasy console.

## Playing the game
Use the arrow keys to move. \
Downloading the HTML file from Releases & running it in your browser is recommended,
but there are also native binaries provided,
or you can build it all yourself using the instructions below.

## Building
Install the wasm32-unknown-unknown Rust toolchain
(`rustup target add wasm32-unknown-unknown` with [rustup](https://rustup.rs) installed),
[Binaryen](https://webassembly.github.io/binaryen) (for wasm-opt),
[wasm-snip](https://github.com/rustwasm/wasm-snip), and [WASM-4](https://wasm4.org).
\
Then run `build.sh`.  The result will be in the build/ directory.

## Reporting Bugs
Please make sure the bug is reproducible on the HTML version before reporting it.
