const assert = require('node:assert/strict');
const { chromium } = require('playwright');

(async () => {
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage();
    await page.goto('http://127.0.0.1:18765/');
    await page.getByRole('button', { name: 'New Project' }).click();
    await page.getByRole('button', { name: 'Choose your EPUB file' }).click();
    assert.equal(await page.locator('h1').innerText(), 'Your book is ready');
    await page.getByRole('button', { name: 'Back' }).click();
    assert.equal(await page.locator('h1').innerText(), 'Choose your book');
    assert.match(await page.locator('#test-state').innerText(), /^choose-book ·/);
    await page.getByRole('button', { name: 'Choose your EPUB file' }).click();
    assert.equal(await page.locator('h1').innerText(), 'Your book is ready');
    console.log('Back returns to book selection and permits another choice');
  } finally {
    await browser.close();
  }
})().catch(error => {
  console.error(error);
  process.exitCode = 1;
});
