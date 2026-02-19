# Face Crop Studio Web (Leptos)

This project recreates the `index.html` landing page using the `leptos` Rust crate (CSR mode).

## Prerequisites

- Rust toolchain installed
- `wasm32-unknown-unknown` target:

  ```bash
  rustup target add wasm32-unknown-unknown
  ```

- Trunk (for serving the WASM app):

  ```bash
  cargo install trunk
  ```

## Run Locally (Browser)

1. Start the dev server:

   ```bash
   trunk serve app.html --open
   ```

2. Trunk will print a local URL (usually `http://127.0.0.1:8080`).
3. Confirm the page loads and test:
   - navbar anchor links (`Features`, `Download`, `Donate`)
   - reveal animations while scrolling
   - preset pill active-state toggling in the architecture section

## Build Checks

- Native compile sanity check:

  ```bash
  cargo build
  ```

- WASM compile via Trunk:

  ```bash
  trunk build app.html
  ```

## Notes

- `index.html` is kept as the original static reference.
- `app.html` is the Trunk entry file for the Leptos version.
