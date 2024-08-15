---
theme: uncover
size: 16:9
paginate: true
---

# Writing our Web Components in Rust?


---

## Roadmap

1. Vanilla Web Component
2. Rust Web Component
3. Dioxus Web Component

---

## Vanilla Web Component

---

### A Web standard

- [Custom Elements](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements) : create your own HTML tags
- [Shadow DOM](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_shadow_DOM) : DOM & CSS encapsulation
- [HTML Template](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_templates_and_slots) : reusable templating in HTML

---

### LIVE CODE

---

### Takeaway

- Read `string` attributes
- Send `CustomEvent` to notify the outside world
- Can expose methods and attributes if needed

- Universal
- Lot's of boilerplate => lib/framework/compiler
  [Lit](https://lit.dev/), [Stencil](https://stenciljs.com/), ...
- Challenges
  - theme
    [CSS variables](https://developer.mozilla.org/en-US/docs/Web/CSS/Using_CSS_custom_properties) or [`::part` pseudo-element](https://developer.mozilla.org/en-US/docs/Web/CSS/::part)
  - forms
    [ElementsInternals](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals) and `static formAssociated = true;`

---

## Rust Web Component

---

### Web Assembly (WASM)


- Binary format, easy to parse and compile into fast code
- Interop with JavaScript

- ~ Java bytecode

- can also be used outside the Web.

<!-- 
  Alternative for JS: GWT, Dart, Elm, ReasonML, ...
 -->

---

### Rust

- A language empowering everyone to build reliable and efficient software.
- No GC, and NO memory issue[^1]
- Community-driven
- FFI: interop with other worlds ([Pyo3](https://pyo3.rs/), [Neon](https://neon-rs.dev/), ...)

[^1]: In safe Rust

---

### Ownership & Borrowing

- **Ownership**
  Each value has one owner,
  and it's dropped when the owner is out of scope.

- **Borrowing**
  Immutable = multiple readers, no writers.
  Mutable = one writer, no readers.

---

### LIVE CODE

--- 

### Takeaway

- [`wasm-bindgen`](https://rustwasm.github.io/wasm-bindgen/)
- [`web-sys`](https://docs.rs/web-sys/latest/web_sys/)
- [Rust 🦀 and WebAssembly 🕸️](https://rustwasm.github.io/docs/book/)

- Tips:
  - try to reduce the size of interop code
  - some crate can help you [`gloo`](https://docs.rs/gloo/latest/gloo/)
  - your are allowed to write small piece of JavaScript

---


## Dioxus Web Component

---

### Dioxus

- Fullstack, crossplatform,lightning fast, fully typed.
- Inspirated by React (signals, VDOM)
- Web oriented, but have alternative renderer (TUI, Naive, ...)
- [Funded company](https://www.ycombinator.com/companies/dioxus-labs)

[Dioxus](https://dioxuslabs.com/)
[Dioxus Discord](https://discord.com/invite/XgGxMSkvUM)

---

### LIVE CODE

---

### Takeaway

- Dioxus provide a good DX
- ... if you are familiar with signals
- [`dioxus-web-component`](https://github.com/ilaborie/dioxus-web-component) remove a lot of boilerplate
- hide (most of) the complexity of `wasm-bindgen`

---

## Conclusion

---

### Yes we can!

Unless your browser is more than 5 years old

---

### TL;DR

Rust can be a powerful choice for Web Components/UI.

**Try it** if you seek type-safety and can work with a maturing ecosystem.

---

### Is it a good idea for Web Component/Web Application/UI ?

It depends!

```
vite v5.4.0 building for production...
✓ 14 modules transformed.
dist/index.html                           1.10 kB │ gzip:  0.48 kB
dist/assets/rust_wc_bg-NzLtFgor.wasm     25.51 kB
dist/assets/dioxus_wc_bg-DB6Otv1_.wasm  413.08 kB
dist/assets/index-DorIzgSS.css            0.08 kB │ gzip:  0.09 kB
dist/assets/index-BOjFjW9c.js            52.73 kB │ gzip: 13.57 kB
✓ built in 151ms
```

--- 
### Pro/Cons

Yes, if you
already know Rust
want type-safety
be able to live without a wide ecosystem
have enough time

rust_wc WASM size: 25kb
dioxus_wc WASM size: 403 KB


No, if you want
an rich and mature ecosystem
to prototype super-fast

---

### Going further

Rust UI ecosystem is maturing...
There are projects that might become the next big UI lib.
~~Stay tune~~ => Come to contribute.

---

## Thanks
