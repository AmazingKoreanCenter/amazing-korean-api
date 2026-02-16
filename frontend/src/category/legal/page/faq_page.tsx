import { LegalPage } from "./legal_page";

const SECTIONS = Array.from({ length: 8 }, (_, i) => ({
  titleKey: `legal.faq.s${i + 1}Title`,
  contentKey: `legal.faq.s${i + 1}Content`,
}));

export function FaqPage() {
  return <LegalPage pageKey="faq" sections={SECTIONS} />;
}
