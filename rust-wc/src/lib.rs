use std::cell::Cell;

use wasm_bindgen::prelude::*;
use web_sys::{CustomEvent, CustomEventInit, Element, ShadowRoot, ShadowRootInit, ShadowRootMode};

// Declare a FFI
// See [web `alert`](https://developer.mozilla.org/en-US/docs/Web/API/Window/alert) function.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Declare our helper JS function
#[wasm_bindgen(module = "/src/glue.js")]
extern "C" {
    fn register(tag: &str, attributes: Vec<String>, web_component: CustomElementBuilder);
}

// Entry point of the library
#[wasm_bindgen(start)]
fn start() {
    register("rs-counter", vec!["name".to_string()], CustomElementBuilder);
}

/// Builder for a `CountRustElement`
#[wasm_bindgen]
pub struct CustomElementBuilder;

#[wasm_bindgen]
impl CustomElementBuilder {
    pub fn build(&self, host: &Element) -> CountRustElement {
        CountRustElement::new(host)
    }
}

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub struct CountEvent {
    pub count: usize,
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct CountRustElement {
    host: Element,
    root: ShadowRoot,
    // See <https://doc.rust-lang.org/book/ch15-00-smart-pointers.html>
    // for a recap on interior-mutability
    count: Cell<usize>,
    name_el: Option<Element>,
    count_el: Option<Element>,
}

#[wasm_bindgen]
impl CountRustElement {
    /// Create a new instance
    ///
    /// Note that this methods does not require to be accessible in JS context,
    /// we could move it into an `impl` block without the `#[wasm_bindgen]`
    #[wasm_bindgen(constructor)]
    pub fn new(host: &Element) -> Self {
        let root = host
            .attach_shadow(&ShadowRootInit::new(ShadowRootMode::Closed))
            .unwrap_throw(); // throw a JS Error if fail

        Self {
            host: host.clone(),
            root,
            name_el: None,
            count: Cell::new(0),
            count_el: None,
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
            // We cannot send references in the closure (unless 'static)
            // => we clone and need interior-mutability for the count
            let count = self.count.clone();
            let count_el = el.clone();
            let host = self.host.clone();
            let on_click = Closure::<dyn FnMut()>::new(move || {
                // Update counter
                let new_count = count.get() + 1;
                count.set(new_count);
                count_el.set_text_content(Some(&new_count.to_string()));
                // Sending a custom event
                let mut init = CustomEventInit::new();
                let detail = CountEvent { count: new_count };
                init.detail(&detail.into());
                let event = CustomEvent::new_with_event_init_dict("count", &init).unwrap_throw();
                host.dispatch_event(&event).unwrap_throw();
            });
            el.add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref())
                .unwrap_throw();
            // FIXME memory-leak
            // We could improve that, but, this is not the topic here
            // See <https://rustwasm.github.io/wasm-bindgen/examples/closures.html>
            // An easy way to fix that, is to use the `gloo_events` crate
            // See <https://docs.rs/gloo-events/latest/gloo_events/struct.EventListener.html>
            on_click.forget();
        }
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
