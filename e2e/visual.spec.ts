import { expect, test } from "@playwright/test";

test.describe("visual regression", () => {
  test("landing page full render remains stable", async ({ page }, testInfo) => {
    await page.goto("/");
    await page.waitForLoadState("networkidle");

    const viewportHeight = page.viewportSize()?.height ?? 800;

    // Remove motion for deterministic screenshots.
    await page.addStyleTag({
      content: `
        *,
        *::before,
        *::after {
          animation: none !important;
          transition: none !important;
          caret-color: transparent !important;
        }
        html {
          scroll-behavior: auto !important;
        }
        body {
          overflow-y: scroll !important;
        }
        /* Mobile full-page captures can oscillate with 100vh behavior. */
        .hero {
          min-height: ${viewportHeight}px !important;
        }
      `
    });

    await expect(page).toHaveScreenshot(`landing-full-page-${testInfo.project.name}.png`, {
      fullPage: true,
      animations: "disabled",
      maxDiffPixelRatio: 0.01,
      timeout: 15_000
    });
  });
});
