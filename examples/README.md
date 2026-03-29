# Examples

This folder now contains a demo gallery plus five focused demos:

- `index.html`: demo gallery
- `playground.html`: interactive measurement and layout console
- `kinetic-lines.html`: animated width sweep and live line reflow
- `poster-lab.html`: responsive poster composition
- `canvas-overlay.html`: line-bound visualization on canvas
- `resize-lab.html`: resize stress test using `ResizeObserver`

Build the package first:

```bash
wasm-pack build --target web
```

Serve the repository root with any static file server:

```bash
python3 -m http.server 8000
```

Then open:

```text
http://localhost:8000/examples/index.html
```
