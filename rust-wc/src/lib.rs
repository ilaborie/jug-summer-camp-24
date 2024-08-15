use std::cell::Cell;

use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::{CustomEvent, CustomEventInit, Element, ShadowRoot, ShadowRootInit, ShadowRootMode};

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

#[wasm_bindgen(module = "/src/glue.js")]
extern "C" {
    fn register(tag: &str, attributes: Vec<String>, web_component: CustomElementBuilder);
}

#[wasm_bindgen(start)]
fn start() {
    register("rs-counter", vec!["name".to_string()], CustomElementBuilder);
}

#[wasm_bindgen]
pub struct CustomElementBuilder;

#[wasm_bindgen]
impl CustomElementBuilder {
    pub fn build(&self, host: &Element) -> CountRustElement {
        CountRustElement::new(host)
    }
}

#[wasm_bindgen]
pub struct CountEvent {
    pub count: usize,
}

#[wasm_bindgen]
pub struct CountRustElement {
    host: Element,
    root: ShadowRoot,
    count: usize,
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
            count: 0,
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
            self.count
        );
        self.root.set_inner_html(&html);

        // Keep reference on dynamic elements
        self.name_el = self.root.query_selector("label").unwrap_or_default();
        self.count_el = self.root.query_selector("button").unwrap_or_default();

        // Register click listener
        if let Some(el) = &self.count_el {
            let mut count = self.count;
            let count_el = el.clone();
            let host = self.host.clone();

            let listener = EventListener::new(el, "click", move |_event| {
                // Update counter
                let new_count = count + 1;
                count = new_count;
                count_el.set_text_content(Some(&new_count.to_string()));

                // Sending a custom event
                let mut init = CustomEventInit::new();
                let detail = CountEvent { count: new_count };
                init.detail(&detail.into());
                let event = CustomEvent::new_with_event_init_dict("count", &init).unwrap_throw();
                host.dispatch_event(&event).unwrap_throw();
            });
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
