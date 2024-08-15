/**
 * Register a web component
 *
 * This function is called from Rust.
 * This is a workaround to allow extending `HTLMElement` from Rust side
 *
 * @param {string} tag the component tag name
 * @param {string[]} attributes the watched attribute of the component
 * @param {*} wc a builder of a inner object, we delegate lifecylcle methods to
 *               this object
 */
export function register(tag, attributes, wc) {
  customElements.define(
    tag,
    // This is an anonymous class
    class extends HTMLElement {
      constructor() {
        super();
        this.inner = wc.inner(this);
      }

      connectedCallback() {
        this.inner.connectedCallback();
      }

      static observedAttributes = attributes;

      attributeChangedCallback(name, oldValue, newValue) {
        this.inner.attributeChangedCallback(name, oldValue, newValue);
      }

      // TODO adoptedCallback,disconnectedCallback
    },
  );
}
