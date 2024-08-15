import rustCounter from "../rust-wc/pkg/rust_wc";
import dioxusCounter from "../dioxus-wc/pkg/dioxus_wc";

import "./style.css";
import counterCss from "./counter.css?raw";

// 1 - Handmade web-component element
type CountEvent = { count: number };

class CounterElement extends HTMLElement {
  private root: ShadowRoot;
  private count = 0;
  private nameEl?: Element;
  private countEl?: Element;

  constructor() {
    super();
    this.root = this.attachShadow({ mode: "closed" });
  }

  connectedCallback() {
    const name = this.getAttribute("name") ?? "";
    this.root.innerHTML = `
      <style>${counterCss}</style>
      <label>${name}</label>
      <button>${this.count}</button>
    `;
    this.nameEl = this.root.querySelector("label")!;
    this.countEl = this.root.querySelector("button")!;

    this.countEl.addEventListener("click", () => {
      const count = ++this.count;
      this.countEl!.textContent = `${this.count}`;

      const detail = { count };
      const event = new CustomEvent<CountEvent>("count", { detail });
      this.dispatchEvent(event);
    });
  }

  static observedAttributes = ["name"];

  attributeChangedCallback(name: string, _oldValue: string, newValue: string) {
    if (name === "name" && this.nameEl) {
      this.nameEl.textContent = newValue;
    }
  }
}

// Register custom elements
customElements.define("js-counter", CounterElement);

// typing for the custom element
declare global {
  interface HTMLElementTagNameMap {
    "js-counter": CounterElement;
  }
}

// Playing with the component
const play = (selector: string) => {
  document.querySelectorAll(selector).forEach((el) => {
    // Dynamic update the name attribute after 3s
    setTimeout(() => el.setAttribute("name", "Plop & Plaf!"), 3000);

    // Listen to the count event
    el.addEventListener("count", (event) => {
      const { detail } = event as CustomEvent<CountEvent>;
      // el.nextSibling!.textContent = JSON.stringify(detail);
      el.nextSibling!.textContent = `${detail.count}`;
    });
  });
};

play("js-counter");

// 2 - Pure Rust web component
// const pg = document.querySelector(".playground");
rustCounter() //
  .then(() => play("rs-counter"));

// 3 - Rust Dioxus web component
dioxusCounter() //
  .then(() => play("dx-counter"));
