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

- Rust code quality gates:

  ```bash
  npm run lint:fmt
  npm run lint:clippy
  ```

  Or run all Rust checks in one command:

  ```bash
  npm run check:rust
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

- Responsive QA checks at `320`, `375`, `768`, `1024`, `1440`:

  ```bash
  npm run e2e:responsive
  ```

- Visual regression snapshots (desktop + mobile):

  ```bash
  npm run e2e:visual:update
  npm run e2e:visual
  ```

  First command captures baseline PNGs in `e2e/visual.spec.ts-snapshots/`; second command fails on unexpected diffs.

- Performance budget checks (bundle + Lighthouse thresholds):

  ```bash
  npm run perf:ci
  ```

  This runs a release build, enforces bundle budgets (`scripts/check_bundle_sizes.sh`), and applies Lighthouse assertions (`lighthouserc.json`).

## Notes

- `app.html` is the Trunk entry file for the Leptos version.
- CI runs `cargo fmt --check`, `cargo clippy -- -D warnings`, dependency audits (`npm audit`, `cargo audit`), Rust build/tests, Trunk release build, performance checks, and Playwright tests on push/PR.

## Security Headers

- `app.html` includes meta-based security controls to keep local/dev behavior safe by default.
- Production deployments should enforce headers at the web server/CDN layer.
- A deploy header template is provided in `_headers` (copied into `dist/` by Trunk).

### Dependency Remediation Policy

1. CI fails on `npm audit` high/critical findings and any `cargo audit` advisory.
2. Patch direct dependencies first (`npm update`, `cargo update -p <crate>`), then rerun full checks.
3. If a fix is unavailable, document the risk and mitigation in PR notes before merging.
4. Remove temporary exceptions once upstream patches are released.

## Contributing

### Architecture map

- Entry: `src/main.rs` mounts `App` from `src/app.rs`.
- Composition: `src/app.rs` wires top-level sections and global resources (fonts, main layout).
- Sections: `src/components/` contains each page section (`nav`, `hero`, `features`, `tech`, `download`, `donate`, `footer`, etc.).
- Behavior hooks: `src/hooks/mod.rs` owns browser behavior such as reveal observer logic.
- Styling: `style.css` is the shared stylesheet consumed by Trunk.
- Browser tests: `e2e/` includes functional, responsive, and visual Playwright coverage.

### Component conventions

- Keep sections isolated in `src/components/<section>.rs` and re-export from `src/components/mod.rs`.
- Prefer data-driven rendering via typed structs/static arrays plus `For` for repeated cards, tiers, and presets.
- Keep external links explicit with `target="_blank"` + `rel="noopener noreferrer"` where applicable.
- Maintain accessibility defaults: semantic landmarks, keyboard focus visibility, and `aria-label` for icon-heavy controls.
- Preserve reduced-motion behavior and avoid introducing mandatory animations.

### Test and check commands

- Rust checks:
  - `npm run check:rust`
- E2E + visual + responsive:
  - `npm run e2e`
  - `npm run e2e:responsive`
  - `npm run e2e:visual`
- Refresh visual baselines when intentional UI changes occur:
  - `npm run e2e:visual:update`
- Performance budgets:
  - `npm run perf:ci`

### Release checklist

1. Run `npm run check:rust`.
2. Run `npm run e2e` and confirm no unexpected failures.
3. If UI changed, run `npm run e2e:visual:update`, review snapshot diffs, then run `npm run e2e:visual`.
4. Run `npm run perf:ci` and confirm Lighthouse/bundle gates pass.
5. Run `trunk build app.html --release` and verify `dist/` output is updated as expected.
6. Update docs/TODO if scope changed and prepare PR with a concise change summary.
