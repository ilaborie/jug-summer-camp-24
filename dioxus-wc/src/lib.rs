use dioxus::prelude::*;
use dioxus_web_component::{web_component, InjectedStyle};
use wasm_bindgen::prelude::*;

#[component]
pub fn App() -> Element {
    let mut current = use_signal(Option::<usize>::default);

    let on_count = move |event: CountEvent| {
        current.set(Some(event.count));
    };

    rsx! {
        div { class: "container",
            h1 { "🧬 Hello Dioxus" }

            Counter {
                name: "Plop",
                on_count,
            }

            if let Some(count) = current() {
                code { "{count}" }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CountEvent {
    pub count: usize,
}

// const STYLE: InjectedStyle = InjectedStyle::css(include_str!("counter.css"));

/// Counter component
#[component]
pub fn Counter(name: Option<String>, on_count: EventHandler<CountEvent>) -> Element {
    let mut count = use_signal(usize::default);

    let name = name.unwrap_or_default();

    let onclick = move |_event| {
        count += 1;
        on_count.call(CountEvent { count: count() });
    };

    rsx! {
        label { "{name}"}
        button {
            onclick,
            "{count}"
        }
    }
}

// #[wasm_bindgen(start)]
// pub fn start() {
//     register_counter();
// }
