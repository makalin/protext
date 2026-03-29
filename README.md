# Protext

**Protext** is a Rust + WebAssembly text measurement and line-wrapping engine that uses the browser's Canvas 2D API as the measurement source of truth.

The current project provides:

- A browser-backed `ProtextEngine` exported through `wasm-bindgen`
- Font-aware width measurement through `measureText`
- Greedy line wrapping with explicit paragraph-break preservation
- Structured layout output for JavaScript consumers
- Layout summary helpers for line count, max width, and estimated height
- Structured measurement helpers for text, words, and paragraph counts
- Unit tests for the pure layout core and browser tests for the Wasm API
- A multi-demo browser showcase

## Status

This repository is a working foundation, not a full typography engine yet.

Implemented today:

- Canvas-backed measurement
- Rust layout core
- Long-token splitting when a word exceeds the line width
- JavaScript measurement, layout, and summary APIs
- Buildable `wasm-pack` package

Not implemented yet:

- Unicode line breaking per UAX #14
- Bidi support
- Hyphenation
- Node.js or Deno-specific adapters

## Installation

```bash
cargo install wasm-pack
wasm-pack build --target web
```

The generated package is written to `pkg/`.

## JavaScript Usage

```js
import init, { ProtextEngine } from "./pkg/protext.js";

async function start() {
  await init();

  const engine = new ProtextEngine();
  engine.set_font("600 16px Inter, sans-serif");

  const text = "Protext brings high-performance typography to the web.";
  const width = engine.measure_width(text);
  const lines = Array.from(engine.layout_lines(text, 220));
  const detailed = Array.from(engine.layout_text(text, 220));

  console.log("width:", width);
  console.log("lines:", lines);
  console.log("layout:", detailed);
}

start();
```

## Public API

`ProtextEngine` exposes:

- `new()`
- `set_font(font: string)`
- `font(): string`
- `measure_width(text: string): number`
- `measure_text(text: string): { text, width, characterCount, wordCount, paragraphCount, empty }`
- `layout_lines(text: string, max_width: number): string[]`
- `layout_text(text: string, max_width: number): Array<{ text, width, paragraphIndex, hardBreak }>`
- `line_count(text: string, max_width: number): number`
- `max_line_width(text: string, max_width: number): number`
- `estimate_height(text: string, max_width: number, line_height: number): number`
- `layout_summary(text: string, max_width: number, line_height: number): { lineCount, maxLineWidth, totalHeight, paragraphCount }`

`layout_text()` returns one object per laid out line:

```json
{
  "text": "Protext brings",
  "width": 118.25,
  "paragraphIndex": 0,
  "hardBreak": false
}
```

`measure_text()` returns:

```json
{
  "text": "alpha beta",
  "width": 74.25,
  "characterCount": 10,
  "wordCount": 2,
  "paragraphCount": 1,
  "empty": false
}
```

`layout_summary()` returns:

```json
{
  "lineCount": 3,
  "maxLineWidth": 118.25,
  "totalHeight": 72,
  "paragraphCount": 1
}
```

## Examples

A multi-demo browser showcase lives in `examples/`.

Build the package:

```bash
wasm-pack build --target web
```

Serve the repository root with a static server:

```bash
python3 -m http.server 8000
```

Then open:

```text
http://localhost:8000/examples/index.html
```

## Testing

Run host-side unit tests:

```bash
cargo test
```

Build the browser package:

```bash
wasm-pack build --target web
```

Run browser/Wasm tests if your local Chrome and ChromeDriver pairing supports `wasm-pack` correctly:

```bash
wasm-pack test --headless --chrome
```

In this environment, the Rust crate and demo assets verify correctly, but the ChromeDriver run still fails with a local `http status: 404` from the browser test runner. That is an execution-environment issue, not a Rust compilation issue.

## Demo Suite

The repository now ships a gallery and five demos:

- `examples/index.html`
- `examples/playground.html`
- `examples/kinetic-lines.html`
- `examples/poster-lab.html`
- `examples/canvas-overlay.html`
- `examples/resize-lab.html`

Quick commands:

```bash
make check
make test
make build
make serve
```

## Project Structure

- `src/lib.rs`: Wasm exports and the layout core
- `tests/web.rs`: browser-side Wasm tests
- `examples/index.html`: demo gallery landing page
- `examples/shared.css`: shared visual system for the demo suite
- `examples/shared.js`: shared wasm/bootstrap helpers for demos
- `examples/playground.html`: interactive layout console
- `examples/kinetic-lines.html`: animated line reflow demo
- `examples/poster-lab.html`: poster composition demo
- `examples/canvas-overlay.html`: canvas line-bound overlay demo
- `examples/resize-lab.html`: resize stress-test demo
- `Makefile`: common check, test, build, and serve commands
- `Cargo.toml`: crate metadata and dependencies
- `pkg/`: generated Wasm output

## License

MIT

---

**Developed by:** [Mehmet T. AKALIN](https://github.com/makalin)  
**Company:** [Digital Vision](https://dv.com.tr)  
**License:** MIT
