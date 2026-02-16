import { LegalPage } from "./legal_page";

const SECTIONS = Array.from({ length: 7 }, (_, i) => ({
  titleKey: `legal.privacy.s${i + 1}Title`,
  contentKey: `legal.privacy.s${i + 1}Content`,
}));

export function PrivacyPage() {
  return <LegalPage pageKey="privacy" sections={SECTIONS} />;
}
