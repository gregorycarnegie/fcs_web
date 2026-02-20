import { expect, test } from "@playwright/test";

const breakpoints = [320, 375, 768, 1024, 1440] as const;

test.describe("responsive QA", () => {
  for (const width of breakpoints) {
    test(`layout remains stable at ${width}px`, async ({ page, isMobile }) => {
      test.skip(!isMobile && width < 768, "320/375 breakpoints are validated in mobile viewport mode.");

      await page.setViewportSize({ width, height: 900 });
      await page.goto("/");
      await page.waitForLoadState("networkidle");
      await page.evaluate(async () => {
        await document.fonts.ready;
      });

      const hasHorizontalOverflow = await page.evaluate(() => {
        return document.documentElement.scrollWidth > window.innerWidth + 1;
      });
      expect(hasHorizontalOverflow).toBe(false);

      await expect(page.locator(".hero")).toBeVisible();
      await expect(page.locator("#features")).toBeVisible();
      await expect(page.locator("#download")).toBeVisible();
      await expect(page.locator("#donate")).toBeVisible();
    });
  }

  test("hero heading typography scales down on narrow screens", async ({ page }) => {
    const getHeadingSize = async (width: number) => {
      await page.setViewportSize({ width, height: 900 });
      await page.goto("/");
      return page.locator("h1").evaluate((el) => parseFloat(getComputedStyle(el).fontSize));
    };

    const size320 = await getHeadingSize(320);
    const size768 = await getHeadingSize(768);
    const size1440 = await getHeadingSize(1440);

    expect(size320).toBeLessThan(size768);
    expect(size768).toBeLessThan(size1440);
  });
});
