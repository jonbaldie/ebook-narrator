const { chromium } = require('playwright');
const assert = require('node:assert/strict');

(async () => {
  const browser = await chromium.launch({ headless: true, args: ['--no-sandbox'] });
  try {
    const page = await browser.newPage();
    const click = name => page.getByRole('button', { name, exact: true }).click();
    const show = async label => console.log(`${label}: ${await page.locator('h1').innerText()} | ${await page.locator('#test-state').innerText()}`);
    const fresh = () => page.goto('http://127.0.0.1:18766/');
    const recording = async () => { await click('Record'); await page.getByRole('button', { name: 'Stop', exact: true }).waitFor(); };
    await fresh();
    await click('＋ New Project'); await click('Choose your EPUB file'); await click('← Back');
    assert.equal(await page.locator('h1').innerText(), 'Choose your book');
    await click('Choose your EPUB file'); await click('Continue'); await click('Test microphone');
    await click('Try again'); await click('Test microphone'); await click('Yes, continue');
    await show('book choice and microphone retry');
    await recording(); await click('Pause'); await show('pause'); await click('Resume'); await click('Stop'); await show('first stop');
    await click('My recordings'); await click('Browse all recordings');
    console.log('recordings: ' + (await page.locator('.soft-list').innerText()).replace(/\n/g, ' | '));
    await click('Review'); await click('Adjust the start or end'); await page.getByLabel('Start of recording').fill('30'); await click('Done'); await click('Record again'); await show('record again');
    for (let i = 0; i < 3; i++) { await recording(); await click('Stop'); }
    await page.getByRole('button', { name: 'Start final review' }).waitFor(); await show('all parts');
    await click('Start final review'); for (let i = 0; i < 3; i++) await click('Continue');
    await click('Export audiobook'); await show('export'); await page.reload(); await show('reload');
    for (let i = 1; i <= 2; i++) {
      await fresh(); await click('Continue recording'); await recording(); await click('Stop');
      await click('← My Project'); await click('＋ New Project'); await click('Choose your EPUB file');
      await click('Continue'); await click('Test microphone'); await click('Yes, continue');
      await show('new project after prior recording ' + i);
      await click('My recordings'); await click('Browse all recordings');
      console.log('new project recordings ' + i + ': ' + (await page.locator('.soft-list').innerText()).replace(/\n/g, ' | '));
    }
  } finally { await browser.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
