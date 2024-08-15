use wasm_bindgen::prelude::*;
use web_sys::{Element, ShadowRoot, ShadowRootInit, ShadowRootMode};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// #[wasm_bindgen(start)]
// fn start() {
//     alert("🦀 Hello Rust");
// }

// FIXME register web component

// #[wasm_bindgen]
// pub struct CountEvent {
//     pub count: usize,
// }

// #[wasm_bindgen]
// pub struct CountRustElement {
//     host: Element,
//     root: ShadowRoot,
//     count: usize,
//     name_el: Option<Element>,
//     count_el: Option<Element>,
// }

// #[wasm_bindgen]
// impl CountRustElement {

// #[wasm_bindgen(constructor)]
// pub fn new(host: &Element) -> Self {
//     let root = host
//         .attach_shadow(&ShadowRootInit::new(ShadowRootMode::Closed))
//         .unwrap_throw();
//     Self {
//         host: host.clone(),
//         root,
//         name_el: None,
//         count: 0,
//         count_el: None,
//     }
// }

// #[wasm_bindgen(js_name = "connectedCallback")]
// pub fn connected_callback(&mut self) {
//     // Set content (shadowed)
//     let css = include_str!("counter.css");
//     let name = self.host.get_attribute("name").unwrap_or_default();
//     let html = format!(
//         "
//             <style>{css}</style>
//             <label>{name}</label>
//             <button>{}</button>
//         ",
//         self.count
//     );
//     self.root.set_inner_html(&html);

//     // Keep reference on dynamic elements
//     self.name_el = self.root.query_selector("label").unwrap_or_default();
//     self.count_el = self.root.query_selector("button").unwrap_or_default();

//     // Register click listener
//     if let Some(el) = &self.count_el {
//         // FIXME register click listener

// Update counter
// let new_count = self.count + 1;
// self.count = new_count;

// Sending a custom event
// let detail = CountEvent { count: new_count };
// let mut init = CustomEventInit::new();
// init.detail(&detail.into());
// let event = CustomEvent::new_with_event_init_dict("count", &init).unwrap_throw();
// host.dispatch_event(&event).unwrap_throw();

//     }
// }

// #[wasm_bindgen(js_name = "disconnectedCallback")]
// pub fn disconnected_callback(&mut self) {
//     self.name_el.take();
//     self.count_el.take();
//     self.root.set_inner_html("");
// }

// #[wasm_bindgen(js_name = "attributeChangedCallback")]
// pub fn attribute_changed_callback(
//     &mut self,
//     name: &str,
//     _old_value: Option<String>,
//     new_value: Option<String>,
// ) {
//     if name == "name" {
//         if let Some(el) = &self.name_el {
//             el.set_text_content(new_value.as_deref());
//         }
//     }
// }

// }

// #[wasm_bindgen(typescript_custom_section)]
// const CUSTOM_TS: &'static str = r#"
// type CountRustElement = {} & HTMLElement;
// declare global {
//   interface HTMLElementTagNameMap {
//     "rs-counter": CountRustElement;
//   }
// }
// "#;
