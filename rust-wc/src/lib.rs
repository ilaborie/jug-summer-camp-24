use std::cell::Cell;

use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::{CustomEvent, CustomEventInit, Element, ShadowRoot, ShadowRootInit, ShadowRootMode};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(module = "/src/glue.js")]
extern "C" {
    fn register(tag: &str, attrs: Vec<String>, builder: Builder);
}

#[wasm_bindgen(start)]
fn start() {
    register("rs-counter", vec!["name".to_string()], Builder {});
    // alert("🦀 Hello Rust");
}

#[wasm_bindgen]
struct Builder {}

#[wasm_bindgen]
impl Builder {
    pub fn build(host: Element) -> CountRustElement {
        CountRustElement::new(host)
    }
}
// FIXME register web component + extends

#[wasm_bindgen]
pub struct CountEvent {
    pub count: usize,
}

#[wasm_bindgen]
pub struct CountRustElement {
    host: Element,
    root: ShadowRoot,
    count: Cell<usize>,
    name_el: Option<Element>,
    count_el: Option<Element>,
    listener: Option<EventListener>,
}

#[wasm_bindgen]
impl CountRustElement {
    #[wasm_bindgen(constructor)]
    pub fn new(host: Element) -> Self {
        let root = host
            .attach_shadow(&ShadowRootInit::new(ShadowRootMode::Closed))
            .unwrap_throw();
        alert("toto");
        Self {
            host,
            root,
            name_el: None,
            count: Cell::new(0),
            count_el: None,
            listener: None,
        }
    }

    #[wasm_bindgen(js_name = "connectedCallback")]
    pub fn connected_callback(&mut self) {
        // Set content (shadowed)
        let css = include_str!("counter.css");
        let name = self.host.get_attribute("name").unwrap_or_default();
        let html = format!(
            "
                <style>{css}</style>
                <label>{name}</label>
                <button>{}</button>
            ",
            self.count.get()
        );
        self.root.set_inner_html(&html);

        // Keep reference on dynamic elements
        self.name_el = self.root.query_selector("label").unwrap_or_default();
        self.count_el = self.root.query_selector("button").unwrap_or_default();

        // Register click listener
        if let Some(el) = &self.count_el {
            // FIXME register click listener
            let count = self.count.clone();
            let el2 = el.clone();
            let host = self.host.clone();

            let lstn = EventListener::new(el, "click", move |_event| {
                // Update counter
                let new_count = count.get() + 1;
                count.set(new_count);
                el2.set_text_content(Some(&new_count.to_string()));
                // alert("toto");

                // Sending a custom event
                let detail = CountEvent { count: new_count };
                let mut init = CustomEventInit::new();
                init.detail(&detail.into());
                let event = CustomEvent::new_with_event_init_dict("count", &init).unwrap_throw();
                host.dispatch_event(&event).unwrap_throw();
            });
            self.listener = Some(lstn);
        }
    }

    #[wasm_bindgen(js_name = "disconnectedCallback")]
    pub fn disconnected_callback(&mut self) {
        self.listener.take();
        self.name_el.take();
        self.count_el.take();
        self.root.set_inner_html("");
    }

    #[wasm_bindgen(js_name = "attributeChangedCallback")]
    pub fn attribute_changed_callback(
        &mut self,
        name: &str,
        _old_value: Option<String>,
        new_value: Option<String>,
    ) {
        if name == "name" {
            if let Some(el) = &self.name_el {
                el.set_text_content(new_value.as_deref());
            }
        }
    }
}

#[wasm_bindgen(typescript_custom_section)]
const CUSTOM_TS: &'static str = r#"
type CountRustElement = {} & HTMLElement;
declare global {
  interface HTMLElementTagNameMap {
    "rs-counter": CountRustElement;
  }
}
"#;
