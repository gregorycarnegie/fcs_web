import { expect, test } from "@playwright/test";

test.describe("landing page e2e", () => {
  test.skip(({ isMobile }) => isMobile, "Desktop-only functional nav/preset checks.");

  test.beforeEach(async ({ page }) => {
    await page.goto("/");
  });

  test("navbar anchors navigate to target sections", async ({ page }) => {
    const anchors: Array<{ label: string; hash: string }> = [
      { label: "Features", hash: "#features" },
      { label: "Download", hash: "#download" },
      { label: "Donate", hash: "#donate" }
    ];

    for (const anchor of anchors) {
      await page.getByRole("link", { name: anchor.label, exact: true }).click();
      await expect(page).toHaveURL(new RegExp(`${anchor.hash}$`));
      await expect(page.locator(anchor.hash)).toBeVisible();
    }
  });

  test("preset pills do not persist an active state", async ({ page }) => {
    const corePill = page.locator(".preset-pill", { hasText: "yunet-core" });
    const cliPill = page.locator(".preset-pill", { hasText: "yunet-cli" });

    await expect(corePill).not.toHaveClass(/active/);
    await expect(cliPill).not.toHaveClass(/active/);

    await cliPill.click();

    await expect(cliPill).not.toHaveClass(/active/);
    await expect(corePill).not.toHaveClass(/active/);
  });

  test("scroll reveal behavior marks cards as visible", async ({ page }) => {
    const featureCard = page.locator(".feature-card").first();

    await expect(featureCard).toHaveClass(/reveal/);
    await expect(featureCard).not.toHaveClass(/visible/);

    await featureCard.scrollIntoViewIfNeeded();
    await expect(featureCard).toHaveClass(/visible/);
  });

  test("external links use expected destinations", async ({ page }) => {
    const externalLinks = page.locator("a[target=\"_blank\"]");
    const total = await externalLinks.count();

    expect(total).toBeGreaterThan(0);

    for (let i = 0; i < total; i++) {
      const href = await externalLinks.nth(i).getAttribute("href");
      expect(href, `missing href for external link #${i + 1}`).toBeTruthy();
      expect(href).toMatch(/^https?:\/\//);
    }

    await expect(page.getByRole("link", { name: "GitHub", exact: true }).first()).toHaveAttribute(
      "href",
      /github\.com\/gregorycarnegie\/iron_cropper/
    );
  });
});
