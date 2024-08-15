
use wasm_bindgen::prelude::*;

// Declare a FFI
// See [web `alert`](https://developer.mozilla.org/en-US/docs/Web/API/Window/alert) function.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Entry point of the library
#[wasm_bindgen(start)]
fn start() {
    alert("🦀 Hello Rust");
}
