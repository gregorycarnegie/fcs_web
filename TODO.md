# Security Audit TODO

## P0 - High Priority

1. [x] Add Content Security Policy (CSP) and core browser hardening headers
2. [x] Add `frame-ancestors 'none'` (or equivalent clickjacking protection)
3. [ ] Add `X-Content-Type-Options: nosniff`
4. [x] Add strict `Referrer-Policy` (for example `strict-origin-when-cross-origin` or stricter)
5. [ ] Prefer server-delivered security headers; use `<meta http-equiv>` only as fallback for static hosting

## P1 - Medium Priority

6. [x] Remove runtime dependency on third-party fonts
7. [x] Eliminate Google Fonts runtime loading from `src/app.rs` (or self-host if reintroduced)
8. [x] Remove runtime dependency on third-party QR image generation
9. [x] Replace `api.qrserver.com` usage in `src/components/donate.rs` with local or first-party QR generation
10. [x] Update CSP allowlist to match only required origins after self-hosting changes

## P1 - CI / Supply Chain Hardening

11. [ ] Pin GitHub Actions to full commit SHAs (not moving tags)
12. [x] Replace `npm install` with `npm ci` in CI for reproducible installs
13. [x] Add explicit minimal `permissions:` block to CI workflow
14. [ ] Review and limit third-party CI steps/tools to required scope only

## P2 - Privacy / Operational Hardening

15. [x] Stop using Lighthouse `temporary-public-storage` unless explicitly required
16. [x] Switch Lighthouse report upload to private/internal storage or disable upload

## P1 - Dependency Vulnerability Coverage

17. [ ] Add Rust dependency audit to CI (`cargo-audit`)
18. [ ] Add Node dependency audit to CI (`npm audit` with chosen fail threshold)
19. [ ] Document remediation policy (how/when to patch vulnerable dependencies)

## Notes

- Current app code does not show direct XSS sinks like `innerHTML` or eval-style usage.
- Existing `target="_blank"` links correctly include `rel="noopener noreferrer"`.
