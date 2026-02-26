import { PageMeta } from "@/components/page_meta";
import { LegalPage } from "./legal_page";

const SECTIONS = Array.from({ length: 8 }, (_, i) => ({
  titleKey: `legal.faq.s${i + 1}Title`,
  contentKey: `legal.faq.s${i + 1}Content`,
}));

export function FaqPage() {
  return (
    <>
      <PageMeta titleKey="seo.faq.title" descriptionKey="seo.faq.description" />
      <LegalPage pageKey="faq" sections={SECTIONS} />
    </>
  );
}
