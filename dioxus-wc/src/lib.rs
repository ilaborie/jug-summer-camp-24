use dioxus::prelude::*;
use dioxus_web_component::{web_component, InjectedStyle};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    register_counter();
}

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub struct CountEvent {
    pub count: usize,
}

/// Counter component
#[web_component(tag = "dx-counter", style = InjectedStyle::css(include_str!("counter.css")))]
pub fn Counter(name: Option<String>, on_count: EventHandler<CountEvent>) -> Element {
    let mut count = use_signal(usize::default);

    let name = name.unwrap_or_default();

    let onclick = move |_event| {
        count += 1;
        on_count.call(CountEvent { count: count() });
    };

    rsx! {
        label { "{name}" }
        button { onclick, "{count}" }
    }
}
