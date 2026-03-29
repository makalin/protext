.PHONY: check test build smoke serve

check:
	cargo check
	cargo check --target wasm32-unknown-unknown

test:
	cargo test

build:
	wasm-pack build --target web

serve:
	python3 -m http.server 8000

smoke:
	@echo "Serve the repo root, then open:"
	@echo "  /examples/index.html"
	@echo "  /examples/playground.html"
	@echo "  /examples/kinetic-lines.html"
	@echo "  /examples/poster-lab.html"
	@echo "  /examples/canvas-overlay.html"
	@echo "  /examples/resize-lab.html"
