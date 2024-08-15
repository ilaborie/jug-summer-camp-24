#![allow(unused_imports)]
use dioxus::prelude::*;
use dioxus_web_component::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn App() -> Element {
    //     let mut current = use_signal(Option::<usize>::default);
    //     let on_count = move |event: CountEvent| {
    //         current.set(Some(event.count));
    //     };

    rsx! {
       h1  {
            "🧬 Hello Dioxus"
        }
    }
}

// pub struct CountEvent {
//     pub count: usize,
// }

// const STYLE: InjectedStyle = InjectedStyle::css(include_str!("counter.css"));

// FIXME Counter component

// #[wasm_bindgen(start)]
// fn start() {
//     register_counter();
// }
