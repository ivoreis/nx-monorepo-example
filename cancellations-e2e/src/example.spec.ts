import { test, expect } from '@playwright/test';

test('has title', async ({ page }) => {
  await page.goto('/admin/1234');

  // Expect h1 to contain a substring.
  expect(await page.locator('h1').innerText()).toContain('Cancellation Page');
});
