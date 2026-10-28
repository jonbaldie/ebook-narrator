const { chromium } = require('playwright');
(async () => {
 const browser = await chromium.launch({headless:true,args:['--no-sandbox']});
 try {
  const page = await browser.newPage();
  const state = async (label) => console.log(label + ': ' + await page.locator('#test-state').innerText() + ' | ' + await page.locator('h1').innerText());
  const click = async (name) => page.getByRole('button',{name,exact:true}).click();
  await page.goto('http://127.0.0.1:18765/'); await state('start');
  await click('＋ New Project'); await state('new'); await click('Choose your EPUB file'); await state('book'); await click('Continue'); await click('Test microphone'); await state('test'); await click('Try again'); await click('Test microphone'); await click('Yes, continue'); await state('ready');
  await click('Record'); await page.getByRole('button',{name:'Pause',exact:true}).waitFor(); await click('Pause'); await state('paused'); await click('Resume'); await click('Stop'); await state('stopped first');
  await click('My recordings'); await click('Browse all recordings'); console.log('browse: '+(await page.locator('.soft-list').innerText()).replace(/\n/g,' | ')); await click('Review'); await click('Adjust the start or end'); await page.getByLabel('Start of recording').fill('30'); await click('Done'); await state('trim done'); await click('Record again'); await state('record again');
  for (let i=0;i<3;i++){ await click('Record'); await page.getByRole('button',{name:'Stop',exact:true}).waitFor(); await click('Stop'); }
  await page.getByRole('button',{name:'Start final review'}).waitFor(); await state('complete'); await click('Start final review'); for(let i=0;i<3;i++) await click('Continue'); await click('Export audiobook'); await state('export'); await page.reload(); await state('reload');
  for(let i=1;i<=2;i++){await page.reload(); await click('Continue recording'); await click('Record'); await page.getByRole('button',{name:'Stop',exact:true}).waitFor(); await click('Next text →'); await state('navigation during recording '+i); await click('Stop'); await click('My recordings'); await click('Browse all recordings'); console.log('replay '+i+': '+(await page.locator('.soft-list').innerText()).replace(/\n/g,' | '));}
 } finally { await browser.close(); }
})().catch(e=>{console.error(e); process.exitCode=1});
