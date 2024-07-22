# Kevin P. Thorne - Portfolio Site

WASM

---

Uses [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) to 
run a completely custom (no `std` library) application. `wasm/`
contains the Rust application that renders bitmaps to a given
HTML Canvas element. `js/` handles the weird quirks of creating,
resizing, and dispatching events against a Canvas.

Here is my list of wins so far:
- Responsive renderer: rendered bitmap changes to whatever dimensions
the browser's view
- Scalable monospaced text with custom font
- Click event capture (soon to be any-event capture)
- Rough reimplementation of C++98's BitSet (thanks Google Gemini)