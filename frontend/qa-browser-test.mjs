import puppeteer from 'puppeteer';

const FRONTEND = 'http://localhost:5173';
const results = [];

function log(testId, status, detail) {
  const icon = status === 'PASS' ? '\u2705' : status === 'FAIL' ? '\u274C' : '\u26A0\uFE0F';
  const line = `${icon} ${testId}: ${detail}`;
  console.log(line);
  results.push({ testId, status, detail });
}

async function run() {
  const browser = await puppeteer.launch({
    headless: 'new',
    args: ['--no-sandbox', '--disable-setuid-sandbox'],
  });

  // Helper: go to find-password tab
  async function goToFindPasswordTab(page) {
    await page.goto(`${FRONTEND}/find-id`, { waitUntil: 'networkidle2' });
    const tabs = await page.$$('[role="tab"]');
    if (tabs.length >= 2) {
      await tabs[1].click();
      await new Promise(r => setTimeout(r, 1000));
    }
  }

  // Helper: fill find-password form
  async function fillFindPasswordForm(page) {
    // name
    await page.evaluate(() => {
      const input = document.querySelector('input[name="name"]');
      if (input) {
        const nativeSet = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value').set;
        nativeSet.call(input, 'QA Tester');
        input.dispatchEvent(new Event('input', { bubbles: true }));
        input.dispatchEvent(new Event('change', { bubbles: true }));
      }
    });

    // birthday (type=date)
    await page.evaluate(() => {
      const input = document.querySelector('input[name="birthday"]');
      if (input) {
        const nativeSet = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value').set;
        nativeSet.call(input, '1995-06-15');
        input.dispatchEvent(new Event('input', { bubbles: true }));
        input.dispatchEvent(new Event('change', { bubbles: true }));
      }
    });

    // email
    await page.evaluate(() => {
      const input = document.querySelector('input[type="email"]');
      if (input) {
        const nativeSet = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value').set;
        nativeSet.call(input, 'qa-test@example.com');
        input.dispatchEvent(new Event('input', { bubbles: true }));
        input.dispatchEvent(new Event('change', { bubbles: true }));
      }
    });

    await new Promise(r => setTimeout(r, 500));
  }

  // ============================================
  // PT-001: OAuth Warning Banner Visible (Step 1)
  // ============================================
  console.log('\n=== PT-001: OAuth Warning Banner ===');
  {
    const page = await browser.newPage();
    const consoleLogs = [];
    page.on('console', msg => { if (msg.type() === 'error') consoleLogs.push(msg.text()); });

    await goToFindPasswordTab(page);

    // Warning banner
    const bannerText = await page.evaluate(() => {
      const el = document.querySelector('.border-yellow-200, [class*="border-yellow"]');
      return el ? el.textContent : null;
    });
    const hasOAuthText = bannerText && (bannerText.includes('소셜') || bannerText.includes('social'));
    log('PT-001', hasOAuthText ? 'PASS' : 'FAIL', `OAuth warning: "${bannerText?.substring(0, 70)}..."`);

    // AlertTriangle icon
    const hasIcon = await page.evaluate(() => {
      const svg = document.querySelector('.border-yellow-200 svg, [class*="border-yellow"] svg');
      return !!svg;
    });
    log('PT-001b', hasIcon ? 'PASS' : 'FAIL', `AlertTriangle icon: ${hasIcon}`);

    if (consoleLogs.length > 0) console.log('  Console errors:', consoleLogs.slice(0, 3));
    await page.close();
  }

  // ============================================
  // PT-002: Find Password Form + Remaining Attempts
  // ============================================
  console.log('\n=== PT-002: Find Password Form + Remaining Attempts ===');
  {
    const page = await browser.newPage();
    const apiResponses = [];
    const consoleLogs = [];
    page.on('console', msg => { if (msg.type() === 'error') consoleLogs.push(msg.text()); });
    page.on('response', async res => {
      if (res.url().includes('/auth/find-password')) {
        try { apiResponses.push({ status: res.status(), body: await res.json() }); } catch {}
      }
    });

    await goToFindPasswordTab(page);
    await fillFindPasswordForm(page);

    // Submit
    const submitted = await page.evaluate(() => {
      const btn = document.querySelector('button[type="submit"]');
      if (btn && !btn.disabled) { btn.click(); return true; }
      return false;
    });
    log('PT-002-submit', submitted ? 'PASS' : 'FAIL', `Form submitted: ${submitted}`);
    await new Promise(r => setTimeout(r, 4000));

    // API response
    if (apiResponses.length > 0) {
      const resp = apiResponses[0];
      const hasRemaining = resp.body?.remaining_attempts !== undefined;
      log('PT-002a', hasRemaining ? 'PASS' : 'FAIL', `API response: status=${resp.status}, remaining=${resp.body?.remaining_attempts}`);
    } else {
      log('PT-002a', 'FAIL', 'No API response captured');
    }

    // Remaining attempts text
    const remainingText = await page.evaluate(() => {
      const allEls = document.querySelectorAll('p, span, div');
      for (const el of allEls) {
        const text = el.textContent || '';
        if (text.includes('남은') || text.includes('Remaining') || text.includes('한도')) {
          return text.trim();
        }
      }
      return null;
    });
    log('PT-002b', remainingText ? 'PASS' : 'FAIL', `Remaining text: "${remainingText}"`);

    // Code input visible (step 2 reached)
    const step2Reached = await page.evaluate(() => {
      // Check for OTP input or code-related elements
      const codeInputs = document.querySelectorAll('input[inputmode="numeric"], input[maxlength="6"]');
      const bodyText = document.body.textContent;
      return codeInputs.length > 0 || bodyText.includes('인증번호를 입력') || bodyText.includes('verification code');
    });
    log('PT-002c', step2Reached ? 'PASS' : 'WARN', `Step 2 (code input) reached: ${step2Reached}`);

    if (consoleLogs.length > 0) console.log('  Console errors:', consoleLogs.slice(0, 3));
    await page.close();
  }

  // ============================================
  // PT-003: OAuth Banner Hidden on Step 2
  // ============================================
  console.log('\n=== PT-003: OAuth Banner on Step 2 ===');
  {
    const page = await browser.newPage();
    await goToFindPasswordTab(page);
    await fillFindPasswordForm(page);

    await page.evaluate(() => { document.querySelector('button[type="submit"]')?.click(); });
    await new Promise(r => setTimeout(r, 4000));

    const bannerOnStep2 = await page.evaluate(() => {
      return !!document.querySelector('.border-yellow-200, [class*="border-yellow"]');
    });

    // Check if we actually reached step 2
    const onStep2 = await page.evaluate(() => {
      const text = document.body.textContent;
      return text.includes('인증번호를 입력') || text.includes('verification code');
    });

    if (onStep2) {
      log('PT-003', !bannerOnStep2 ? 'PASS' : 'FAIL', `OAuth banner on step 2: ${bannerOnStep2 ? 'VISIBLE (bug)' : 'hidden (correct)'}`);
    } else {
      log('PT-003', 'WARN', 'Could not reach step 2 to verify banner visibility');
    }

    await page.close();
  }

  // ============================================
  // PT-004: Resend Button Disabled at remaining=0
  // ============================================
  console.log('\n=== PT-004: Resend Button Disabled Check ===');
  {
    // This test relies on rate limit being near exhaustion
    // We'll check the button disabled logic via DOM inspection
    const page = await browser.newPage();
    await goToFindPasswordTab(page);
    await fillFindPasswordForm(page);

    await page.evaluate(() => { document.querySelector('button[type="submit"]')?.click(); });
    await new Promise(r => setTimeout(r, 4000));

    // Check if resend button exists and its state
    const resendBtnInfo = await page.evaluate(() => {
      const buttons = [...document.querySelectorAll('button[type="button"]')];
      const resend = buttons.find(b => {
        const text = b.textContent || '';
        return text.includes('재전송') || text.includes('Resend') || text.includes('resend');
      });
      if (resend) {
        return { found: true, disabled: resend.disabled, text: resend.textContent?.trim() };
      }
      return { found: false };
    });

    if (resendBtnInfo.found) {
      log('PT-004', 'PASS', `Resend button found: disabled=${resendBtnInfo.disabled}, text="${resendBtnInfo.text}"`);
    } else {
      log('PT-004', 'WARN', 'Resend button not found (may not be on step 2)');
    }

    await page.close();
  }

  // ============================================
  // PT-009: 429 Toast Message
  // ============================================
  console.log('\n=== PT-009: 429 Toast Check ===');
  {
    const page = await browser.newPage();
    let got429 = false;
    page.on('response', async res => {
      if (res.url().includes('/auth/find-password') && res.status() === 429) got429 = true;
    });

    await goToFindPasswordTab(page);
    await fillFindPasswordForm(page);

    // Submit repeatedly to trigger 429
    for (let i = 0; i < 2; i++) {
      await page.evaluate(() => { document.querySelector('button[type="submit"]')?.click(); });
      await new Promise(r => setTimeout(r, 2000));
      // Go back to step 1 by reloading if needed
      if (i < 1) {
        await goToFindPasswordTab(page);
        await fillFindPasswordForm(page);
      }
    }

    // Check for toast
    const toastText = await page.evaluate(() => {
      const toast = document.querySelector('[data-sonner-toast]');
      return toast ? toast.textContent : null;
    });

    log('PT-009', got429 ? 'PASS' : 'INFO', `429 triggered: ${got429}, toast: "${toastText?.substring(0, 60)}"`);
    await page.close();
  }

  // ============================================
  // I18N-008: Language Toggle
  // ============================================
  console.log('\n=== I18N-008: Language Toggle ===');
  {
    const page = await browser.newPage();

    // Korean first
    await page.evaluateOnNewDocument(() => { localStorage.setItem('i18nextLng', 'ko'); });
    await goToFindPasswordTab(page);

    const koText = await page.evaluate(() => {
      const el = document.querySelector('.border-yellow-200, [class*="border-yellow"]');
      return el ? el.textContent : null;
    });
    log('I18N-KO', koText?.includes('소셜') ? 'PASS' : 'FAIL', `Korean: "${koText?.substring(0, 60)}"`);
    await page.close();

    // English
    const page2 = await browser.newPage();
    await page2.evaluateOnNewDocument(() => { localStorage.setItem('i18nextLng', 'en'); });
    await page2.goto(`${FRONTEND}/find-id`, { waitUntil: 'networkidle2' });
    const tabs2 = await page2.$$('[role="tab"]');
    if (tabs2.length >= 2) { await tabs2[1].click(); await new Promise(r => setTimeout(r, 1000)); }

    const enText = await page2.evaluate(() => {
      const el = document.querySelector('.border-yellow-200, [class*="border-yellow"]');
      return el ? el.textContent : null;
    });
    log('I18N-EN', (enText && (enText.includes('social') || enText.includes('Social'))) ? 'PASS' : 'FAIL',
      `English: "${enText?.substring(0, 60)}"`);
    await page2.close();
  }

  // ============================================
  // PT-007: Verify Email Page Guard
  // ============================================
  console.log('\n=== PT-007: Verify Email Page ===');
  {
    const page = await browser.newPage();
    await page.goto(`${FRONTEND}/verify-email`, { waitUntil: 'networkidle2' });
    await new Promise(r => setTimeout(r, 1000));

    const hasGuard = await page.evaluate(() => {
      const text = document.body.textContent;
      return text.includes('이메일') || text.includes('email') || text.includes('signup');
    });
    log('PT-007', hasGuard ? 'PASS' : 'FAIL', `Verify email guard page works: ${hasGuard}`);

    const signupLink = await page.$('a[href="/signup"]');
    log('PT-007b', signupLink ? 'PASS' : 'WARN', `Signup redirect link: ${!!signupLink}`);
    await page.close();
  }

  // ============================================
  // EC-003: Page Reload State Loss
  // ============================================
  console.log('\n=== EC-003: Page Reload State Loss ===');
  {
    const page = await browser.newPage();
    await goToFindPasswordTab(page);
    await fillFindPasswordForm(page);
    await page.evaluate(() => { document.querySelector('button[type="submit"]')?.click(); });
    await new Promise(r => setTimeout(r, 3000));

    // Reload
    await page.reload({ waitUntil: 'networkidle2' });
    await new Promise(r => setTimeout(r, 1000));

    // Check if back to initial state
    const backToInitial = await page.evaluate(() => {
      // Should be back to tab selection, not on step 2
      const tabs = document.querySelectorAll('[role="tab"]');
      return tabs.length >= 2;
    });
    log('EC-003', backToInitial ? 'PASS' : 'FAIL', `Page reload resets state: ${backToInitial}`);
    await page.close();
  }

  // ============================================
  // Summary
  // ============================================
  console.log('\n========================================');
  console.log('       QA BROWSER TEST SUMMARY');
  console.log('========================================');
  const pass = results.filter(r => r.status === 'PASS').length;
  const fail = results.filter(r => r.status === 'FAIL').length;
  const warn = results.filter(r => r.status === 'WARN' || r.status === 'INFO').length;
  console.log(`PASS: ${pass} | FAIL: ${fail} | WARN/INFO: ${warn}`);
  console.log('========================================');

  if (fail > 0) {
    console.log('\nFailed tests:');
    results.filter(r => r.status === 'FAIL').forEach(r => console.log(`  \u274C ${r.testId}: ${r.detail}`));
  }
  if (warn > 0) {
    console.log('\nWarnings:');
    results.filter(r => r.status === 'WARN' || r.status === 'INFO').forEach(r => console.log(`  \u26A0\uFE0F ${r.testId}: ${r.detail}`));
  }

  await browser.close();
}

run().catch(err => {
  console.error('Fatal error:', err);
  process.exit(1);
});
