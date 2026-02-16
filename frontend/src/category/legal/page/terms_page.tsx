import { LegalPage } from "./legal_page";

const SECTIONS = Array.from({ length: 7 }, (_, i) => ({
  titleKey: `legal.terms.s${i + 1}Title`,
  contentKey: `legal.terms.s${i + 1}Content`,
}));

export function TermsPage() {
  return <LegalPage pageKey="terms" sections={SECTIONS} />;
}
