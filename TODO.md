# 10/10 TODO

1. [x] Add real browser E2E tests
2. [x] Use Playwright to validate nav anchors, preset pill toggling, reveal behavior, and external links.

3. [x] Add visual regression snapshots
4. [x] Capture baseline screenshots (desktop + mobile) and fail CI on unexpected diffs.

5. [x] Complete accessibility pass
6. [x] Ensure semantic landmarks/heading order.
7. [x] Add visible keyboard focus states.
8. [x] Verify color contrast (WCAG AA).
9. [x] Add `aria-label` where needed for icon-heavy controls.

10. [x] Respect reduced-motion preferences
11. [x] Disable/soften reveal and hero animations under `prefers-reduced-motion`.

12. [ ] Improve performance budget
13. [ ] Optimize font loading (`preload`, subset/self-host if needed).
14. [ ] Audit bundle size and set Lighthouse/perf targets.

15. [ ] Strengthen responsive QA
16. [ ] Validate at common breakpoints (320, 375, 768, 1024, 1440).
17. [ ] Ensure no overflow/clipping and stable spacing/typography scale.

18. [ ] Harden content/config maintainability
19. [ ] Move repeated card/tier/preset data into static arrays/structs and render with `For`.

20. [ ] Add error/empty resilience checks
21. [ ] Ensure app still renders cleanly if dynamic pieces (future data/config) are missing.

22. [ ] Add code quality gates
23. [ ] CI: `cargo fmt --check`, `cargo clippy -D warnings`, plus existing build/tests.

24. [ ] Improve docs for contributors
25. [ ] Add a “Contributing” section: architecture map, component conventions, test commands, release checklist.
