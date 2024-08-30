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
        // alert("toto");
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
