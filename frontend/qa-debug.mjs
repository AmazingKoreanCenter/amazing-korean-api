import puppeteer from "puppeteer";

const BASE_URL = "http://localhost:5173";

async function main() {
  const browser = await puppeteer.launch({
    headless: 'new',
    args: ["--no-sandbox", "--disable-setuid-sandbox"],
  });

  const page = await browser.newPage();
  await page.setViewport({ width: 1280, height: 800 });

  // 1. /find-id 페이지
  await page.goto(`${BASE_URL}/find-id`, { waitUntil: "networkidle2" });
  console.log("=== /find-id 초기 상태 ===");

  const tabs = await page.$$('[role="tab"]');
  console.log(`탭 개수: ${tabs.length}`);
  for (let i = 0; i < tabs.length; i++) {
    const text = await page.evaluate((el) => el.textContent, tabs[i]);
    const selected = await page.evaluate((el) => el.getAttribute("aria-selected"), tabs[i]);
    console.log(`  탭 ${i}: "${text}" (selected: ${selected})`);
  }

  // 두 번째 탭 클릭
  if (tabs.length > 1) {
    await tabs[1].click();
    await new Promise((r) => setTimeout(r, 1000));
    console.log("\n=== 비밀번호 찾기 탭 클릭 후 ===");
  }

  // Input 확인
  const inputs = await page.evaluate(() => {
    return [...document.querySelectorAll('input')].map(i => ({
      name: i.name, type: i.type, id: i.id,
      placeholder: i.placeholder, value: i.value,
      tagName: i.tagName,
    }));
  });
  console.log(`\nInput 개수: ${inputs.length}`);
  inputs.forEach(i => console.log(`  name="${i.name}" type="${i.type}" id="${i.id}" placeholder="${i.placeholder}"`));

  // Button 확인
  const buttons = await page.evaluate(() => {
    return [...document.querySelectorAll('button')].map(b => ({
      type: b.type, text: b.textContent?.trim().substring(0, 50),
      disabled: b.disabled,
    }));
  });
  console.log(`\nButton 개수: ${buttons.length}`);
  buttons.forEach(b => console.log(`  type="${b.type}" disabled=${b.disabled} text="${b.text}"`));

  // 경고 배너 확인
  const warningInfo = await page.evaluate(() => {
    const el = document.querySelector('.border-yellow-200, [class*="yellow"]');
    return el ? { text: el.textContent?.substring(0, 100), classes: el.className.substring(0, 100) } : null;
  });
  console.log(`\n경고 배너: ${warningInfo ? warningInfo.text : '없음'}`);

  // Screenshot
  await page.screenshot({ path: '/tmp/qa-findpw.png', fullPage: true });

  // 2. Login 페이지
  console.log("\n\n=== /login 페이지 ===");
  await page.goto(`${BASE_URL}/login`, { waitUntil: "networkidle2" });
  const loginInputs = await page.evaluate(() => {
    return [...document.querySelectorAll('input')].map(i => ({
      name: i.name, type: i.type, placeholder: i.placeholder,
    }));
  });
  console.log(`Input 개수: ${loginInputs.length}`);
  loginInputs.forEach(i => console.log(`  name="${i.name}" type="${i.type}" placeholder="${i.placeholder}"`));

  await browser.close();
}

main().catch(console.error);
