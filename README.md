# Protext 🦀 🕸️

**Protext** is a high-performance text layout and line-breaking engine written in **Rust** and compiled to **WebAssembly**.

It is designed to solve the "Canvas Measurement Problem" by using the browser's native Canvas API as the ground truth for typography, while moving the heavy lifting of layout logic, Unicode segmentation, and boundary calculations into a memory-safe, blazing-fast Rust core.

## 🚀 Key Features

  * **Rust Core:** Leverages Rust's speed and safety for complex text-wrapping algorithms.
  * **Wasm-Native:** Zero-overhead communication between JavaScript and the layout engine.
  * **Canvas Ground Truth:** Uses `web-sys` to access the browser's `measureText` API, ensuring 100% visual consistency with the UI.
  * **Zero Dependencies:** Lightweight and focused on performance.
  * **Server-Side Ready:** Can be extended to run in Node.js or Deno environments.

## 📦 Installation

To use **Protext** in your web project, you'll first need to build the Wasm package:

```bash
# Install wasm-pack if you haven't already
cargo install wasm-pack

# Build the project for the browser
wasm-pack build --target web
```

## 🛠️ Usage (JavaScript/TypeScript)

```javascript
import init, { ProtextEngine } from './pkg/protext.js';

async function start() {
    // Initialize the Wasm module
    await init();

    // Create a new layout engine instance
    const engine = new ProtextEngine();

    // Set your typography styles
    engine.set_font("600 16px 'Inter', sans-serif");

    // Measure text with Rust-speed and Canvas-accuracy
    const text = "Protext brings high-performance typography to the web.";
    const width = engine.measure_width(text);
    
    console.log(`Measured Width: ${width}px`);
}

start();
```

## 🏗️ Project Structure

  * `src/lib.rs`: The Rust source containing the `ProtextEngine` and layout logic.
  * `Cargo.toml`: Project metadata and `web-sys` dependencies.
  * `pkg/`: (Generated) The compiled Wasm and JS glue code.

## 🛠 Development

### Prerequisites

  * [Rust](https://www.rust-lang.org/) (latest stable)
  * [wasm-pack](https://www.google.com/search?q=https://rustwasm.github.io/wasm-pack/)

### Running Tests

```bash
wasm-pack test --headless --chrome
```

## 🤝 Contributing

This project is an evolution of the Pretext concept, optimized for low-level systems performance. Contributions regarding Unicode line-breaking (UAX \#14) and Bidi support are highly welcome.

-----

**Developed by:** [Mehmet T. AKALIN](https://github.com/makalin)  
**Company:** [Digital Vision](https://dv.com.tr)  
**License:** MIT
