use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let mut count_event = use_signal(Option::<usize>::default);

    let on_count = move |event: CountEvent| {
        count_event.set(Some(event.count));
    };

    rsx! {
        div { class: "container",
            div {
                display: "flex",
                align_items: "center",
                gap: "1rem",
                Counter {
                    name: "plop",
                    on_count,
                }
                code {
                    if let Some(count) = count_event() {
                        "{count}"
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CountEvent {
    pub count: usize,
}

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
