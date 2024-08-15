# How about writing our Web Components in Rust?

## Requirements

* NodeJS 20+, see [Download Node](https://nodejs.org/en/download/package-manager)
* Rust development tools installed, see [rustup](https://rustup.rs/)
* Extra rust tools:
  * [`wasm-pack`](https://github.com/rustwasm/wasm-pack) for building WASM
  * [`dioxus-cli`](https://dioxuslabs.com/learn/0.5/getting_started) the [Dioxus](https://dioxuslabs.com/) companion CLI
  * [`cargo-watch`](https://github.com/watchexec/cargo-watch) to watch an rebuild the code

Rust tools can be installed with `cargo install <tool>`
or with `cargo binstall <tool>` if you have [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall) installed 


This project use [mise](https://mise.jdx.dev/) to set requirements

To install node dependencies, use `npm ci`

Then you can build WASM in `rust-wc/` and `dioxus-wc/` with `wasm-pack build --release --no-pack --target web`


## Try it

Just need to run `mise run dev` or `npm run watch & npm run dev` to launch the DEV mode.

Then open <http://localhost:5173>




