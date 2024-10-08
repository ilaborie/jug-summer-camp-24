/**
 * Register a web component
 *
 * This function is called from Rust.
 * This is a workaround to allow extending `HTMLElement` from Rust side
 *
 * @param {string} tag the component tag name
 * @param {string[]} attributes the watched attribute of the component
 * @param {*} builder a builder of a inner object, we delegate lifecycle methods to this object
 */
export function register(tag, attributes, builder) {
  customElements.define(
    tag,
    // This is an anonymous class
    class extends HTMLElement {
      constructor() {
        super();
        this.inner = builder.build(this);
      }

      connectedCallback() {
        this.inner.connectedCallback();
      }
      disconnectedCallback() {
        this.inner.disconnectedCallback();
      }

      static observedAttributes = attributes;

      attributeChangedCallback(name, oldValue, newValue) {
        this.inner.attributeChangedCallback(name, oldValue, newValue);
      }
    },
  );
}

// #[wasm_bindgen(module = "/src/glue.js")]
// extern "C" {
//     fn register(tag: &str, attributes: Vec<String>, b: Builder);
//     // register("rs-counter", vec!["name".to_string()], Builder);
// }
// 
// #[wasm_bindgen]
// pub struct Builder;
// 
// #[wasm_bindgen]
// impl Builder {
//     pub fn build(&self, host: Element) -> CountRustElement {
//         CountRustElement::new(host)
//     }
// }
