import { PageMeta } from "@/components/page_meta";
import { LegalPage } from "./legal_page";

const SECTIONS = Array.from({ length: 4 }, (_, i) => ({
  titleKey: `legal.refund.s${i + 1}Title`,
  contentKey: `legal.refund.s${i + 1}Content`,
}));

export function RefundPolicyPage() {
  return (
    <>
      <PageMeta titleKey="seo.refundPolicy.title" descriptionKey="seo.refundPolicy.description" />
      <LegalPage pageKey="refund" sections={SECTIONS} />
    </>
  );
}
