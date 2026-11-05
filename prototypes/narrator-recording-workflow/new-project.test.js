const assert = require('node:assert/strict');
const { chromium } = require('playwright');

(async () => {
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage();
    const click = name => page.getByRole('button', { name, exact: true }).click();
    await page.goto('http://127.0.0.1:18765/');
    await click('Continue recording');
    await click('Next text →');
    await click('Record');
    await click('Stop');
    await click('← My Project');
    await click('＋ New Project');
    await click('Choose your EPUB file');
    await click('← Back');
    assert.equal(await page.locator('h1').innerText(), 'Choose your book');
    await click('Choose your EPUB file');
    await click('Continue');
    await click('Test microphone');
    await click('Yes, continue');
    assert.equal(await page.locator('#test-state').innerText(), 'recording · part 1 · ready · 0 saved');
    assert.equal(await page.getByRole('button', { name: 'My recordings', exact: true }).count(), 0);
    await click('Record');
    await click('Stop');
    await click('My recordings');
    assert.equal(await page.locator('h1').innerText(), '1 of 3 parts recorded');
    await click('Browse all recordings');
    assert.deepEqual(await page.locator('.soft-row').allInnerTexts(), [
      'Chapter One · Part 1\nRecording saved\nReview',
      'Chapter One · Part 2\nNot recorded yet',
      'Chapter Two · Part 1\nNot recorded yet',
    ]);
    console.log('New Project starts empty and contains only its own Take');

    await page.goto('http://127.0.0.1:18765/');
    await click('Continue recording');
    for (let part = 0; part < 3; part++) {
      await click('Record');
      await click('Stop');
    }
    await click('← My Project');
    await click('＋ New Project');
    await page.waitForTimeout(600);
    assert.equal(await page.locator('h1').innerText(), 'Choose your book');
    console.log('A previous Project cannot open final review after New Project');
  } finally {
    await browser.close();
  }
})().catch(error => {
  console.error(error);
  process.exitCode = 1;
});
