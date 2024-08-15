use std::cell::Cell;
use std::rc::Rc;

use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::{CustomEvent, CustomEventInit, Element, ShadowRoot, ShadowRootInit, ShadowRootMode};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(module = "/src/glue.js")]
extern "C" {
    fn register(tag: &str, attributes: Vec<String>, web_component: Builder);
}

#[wasm_bindgen]
pub struct Builder;

#[wasm_bindgen]
impl Builder {
    pub fn build(&self, host: &Element) -> CountRustElement {
        CountRustElement::new(host)
    }
}

#[wasm_bindgen(start)]
fn start() {
    register("rs-counter", vec!["name".to_string()], Builder);
}

#[wasm_bindgen]
pub struct CountEvent {
    pub count: usize,
}

#[wasm_bindgen]
pub struct CountRustElement {
    host: Element,
    root: ShadowRoot,
    count: Rc<Cell<usize>>,
    name_el: Option<Element>,
    count_el: Option<Element>,
    listener: Option<EventListener>,
}

#[wasm_bindgen]
impl CountRustElement {
    #[wasm_bindgen(constructor)]
    pub fn new(host: &Element) -> Self {
        let root = host
            .attach_shadow(&ShadowRootInit::new(ShadowRootMode::Closed))
            .unwrap_throw(); // throw a JS Error if fail

        Self {
            host: host.clone(),
            root,
            name_el: None,
            count: Rc::default(), // interior mutability pattern
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
        if let Some(btn) = &self.count_el {
            let count = self.count.clone();
            let btn2 = btn.clone();
            let host = self.host.clone();

            // Event listener use RAII pattern to remove listener when value is dropped
            let listener = EventListener::new(
                btn,
                "click",
                // clone captured valued + move to avoid sending reference to the closure
                move |_event| {
                    // Update counter
                    let new_count = count.get() + 1;
                    count.set(new_count);
                    btn2.set_text_content(Some(&new_count.to_string()));

                    // Sending a custom event
                    let mut init = CustomEventInit::new();
                    let detail = CountEvent { count: new_count };
                    init.detail(&detail.into());
                    let event =
                        CustomEvent::new_with_event_init_dict("count", &init).unwrap_throw();
                    host.dispatch_event(&event).unwrap_throw();
                },
            );
            self.listener = Some(listener);
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
        // Update component if name attribute changed
        if name == "name" {
            if let Some(el) = &self.name_el {
                el.set_text_content(new_value.as_deref());
            }
        }
    }
}

// Export custom typescript section
#[wasm_bindgen(typescript_custom_section)]
const CUSTOM_TS: &'static str = r#"
type CountRustElement = {} & HTMLElement;
declare global {
  interface HTMLElementTagNameMap {
    "rs-counter": CountRustElement;
  }
}
"#;
