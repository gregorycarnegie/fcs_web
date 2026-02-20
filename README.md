# Face Crop Studio Web (Leptos)

This project recreates the `index.html` landing page using the `leptos` Rust crate (CSR mode).

## Project Layout

- `src/app.rs` top-level app composition
- `src/components/` section-level Leptos components (`NavBar`, `HeroSection`, etc.)
- `src/hooks/mod.rs` browser behavior hooks (scroll reveal observer)
- `style.css` external stylesheet loaded by Trunk
- `app.html` Trunk entrypoint
- `.github/workflows/ci.yml` build + smoke checks

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

- Smoke tests for key section IDs/content:

  ```bash
  cargo test
  ```

- WASM compile via Trunk:

  ```bash
  trunk build app.html
  ```

- Browser E2E checks (Playwright):

  ```bash
  npm install
  npx playwright install --with-deps chromium
  npm run e2e
  ```

- Visual regression snapshots (desktop + mobile):

  ```bash
  npm run e2e:visual:update
  npm run e2e:visual
  ```

  First command captures baseline PNGs in `e2e/visual.spec.ts-snapshots/`; second command fails on unexpected diffs.

## Notes

- `app.html` is the Trunk entry file for the Leptos version.
- CI runs `cargo build`, `cargo test`, and `trunk build app.html --release` on push/PR.
