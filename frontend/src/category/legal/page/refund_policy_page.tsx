import { LegalPage } from "./legal_page";

const SECTIONS = Array.from({ length: 5 }, (_, i) => ({
  titleKey: `legal.refund.s${i + 1}Title`,
  contentKey: `legal.refund.s${i + 1}Content`,
}));

export function RefundPolicyPage() {
  return <LegalPage pageKey="refund" sections={SECTIONS} />;
}
