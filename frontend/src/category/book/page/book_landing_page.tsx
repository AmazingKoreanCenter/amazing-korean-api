import { useParams, Link } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { useEffect } from "react";
import { BookOpen, ShoppingCart, GraduationCap, Video, FileText, Globe } from "lucide-react";

import { Button } from "@/components/ui/button";
import { PageMeta } from "@/components/page_meta";
import { HeroSection } from "@/components/sections/hero_section";
import { SectionContainer } from "@/components/sections/section_container";
import { useAuthStore } from "@/hooks/use_auth_store";
import { changeLanguage } from "@/i18n";

import { findBookByISBN, formatISBN, ALL_STUDENT_BOOKS, BOOK_PRICE, BOOK_PAGES } from "../book_data";
import type { BookInfo } from "../types";

export function BookLandingPage() {
  const { isbn } = useParams<{ isbn: string }>();
  const { isLoggedIn } = useAuthStore();
  const book = isbn ? findBookByISBN(isbn) : null;

  useEffect(() => {
    if (book) {
      changeLanguage(book.i18nCode);
    }
  }, [book]);

  if (!book) return <NotFoundView />;
  return <BookInfoView book={book} isLoggedIn={isLoggedIn} />;
}

function NotFoundView() {
  const { t } = useTranslation();

  return (
    <>
      <PageMeta titleKey="seo.book.notFoundTitle" descriptionKey="seo.book.notFoundDescription" />
      <SectionContainer size="lg">
        <div className="flex flex-col items-center justify-center text-center space-y-4">
          <div className="w-16 h-16 rounded-full bg-muted flex items-center justify-center">
            <BookOpen className="h-8 w-8 text-muted-foreground" />
          </div>
          <h1 className="text-2xl font-bold">{t("book.notFound")}</h1>
          <p className="text-muted-foreground max-w-md">{t("book.notFoundDesc")}</p>
          <div className="flex gap-4 mt-4">
            <Button asChild variant="outline" className="rounded-full">
              <Link to="/">{t("book.goHome")}</Link>
            </Button>
            <Button asChild className="gradient-primary text-white rounded-full">
              <Link to="/textbook">{t("book.viewCatalog")}</Link>
            </Button>
          </div>
        </div>
      </SectionContainer>
    </>
  );
}

function BookInfoView({ book, isLoggedIn }: { book: BookInfo; isLoggedIn: boolean }) {
  const { t } = useTranslation();
  const editionText = t(book.edition === "student" ? "book.editionStudent" : "book.editionTeacher");

  const benefits = [
    { icon: Video, label: t("book.benefitVideos") },
    { icon: FileText, label: t("book.benefitQuizzes") },
    { icon: Globe, label: t("book.benefitLanguages") },
  ];

  const otherBooks = ALL_STUDENT_BOOKS.filter((b) => b.langKey !== book.langKey);

  return (
    <>
      <PageMeta
        titleKey="seo.book.title"
        titleParams={{ language: book.nameLocal }}
        descriptionKey="seo.book.description"
        descriptionParams={{ language: book.nameLocal }}
      />

      {/* Section 1: Hero + Primary CTA */}
      <HeroSection
        badge={
          <>
            <img src={`/flags/${book.flagFile}`} alt="" className="w-6 h-6" />
            <span className="font-medium">{book.nameLocal}</span>
            <span className="text-muted-foreground">·</span>
            <span className="text-muted-foreground">{book.nameKorean}</span>
          </>
        }
        title={
          <>
            Amazing Korean Basic
            <br />
            <span className="text-gradient">놀라운 한국어 기초</span>
          </>
        }
        subtitle={`${editionText} · ISBN ${formatISBN(book.isbn13)}`}
        size="sm"
      >
        <div className="flex items-center justify-center gap-4 mt-4 text-sm text-muted-foreground">
          <span>{BOOK_PAGES}p</span>
          <span>·</span>
          <span>A4</span>
          <span>·</span>
          <span className="font-semibold text-foreground">{BOOK_PRICE}</span>
        </div>

        <div className="mt-8 space-y-3 max-w-sm mx-auto">
          <Button
            asChild
            className="w-full gradient-primary text-white shadow-lg hover:shadow-xl rounded-full h-14 text-base"
          >
            <Link to={isLoggedIn ? "/" : "/signup"}>
              <GraduationCap className="mr-2 h-5 w-5" />
              {t("book.ctaPrimary")}
            </Link>
          </Button>
          <div className="grid grid-cols-2 gap-3">
            <Button asChild variant="outline" className="rounded-full h-12 text-sm border-2">
              <Link to="/ebook">
                <BookOpen className="mr-2 h-4 w-4" />
                {t("book.buyEbook")}
              </Link>
            </Button>
            <Button asChild variant="outline" className="rounded-full h-12 text-sm border-2">
              <Link to="/textbook">
                <ShoppingCart className="mr-2 h-4 w-4" />
                {t("book.orderTextbook")}
              </Link>
            </Button>
          </div>
        </div>
      </HeroSection>

      {/* Section 2: 서비스 안내 + 다른 언어 */}
      <SectionContainer size="md">
        <h2 className="text-2xl md:text-3xl font-bold tracking-tight text-center mb-8 break-keep">
          {t("book.benefitsTitle")}
        </h2>
        <div className="grid grid-cols-3 gap-4 max-w-lg mx-auto text-center">
          {benefits.map((item) => (
            <div key={item.label} className="space-y-2">
              <div className="w-12 h-12 rounded-xl gradient-primary mx-auto flex items-center justify-center">
                <item.icon className="h-6 w-6 text-white" />
              </div>
              <div className="text-sm text-muted-foreground">{item.label}</div>
            </div>
          ))}
        </div>

        {otherBooks.length > 0 && (
          <div className="mt-12 pt-8 border-t">
            <h3 className="text-lg font-semibold text-center mb-4">
              {t("book.otherLanguages")}
            </h3>
            <div className="flex flex-wrap justify-center gap-2">
              {otherBooks.map((b) => (
                <Link
                  key={b.langKey}
                  to={`/book/${b.isbn13}`}
                  className="flex items-center gap-1.5 px-3 py-2 rounded-full border hover:bg-muted/50 transition-colors text-sm"
                >
                  <img src={`/flags/${b.flagFile}`} alt="" className="w-5 h-5" />
                  <span>{b.nameLocal}</span>
                </Link>
              ))}
            </div>
          </div>
        )}
      </SectionContainer>

      {/* Section 3: 하단 CTA 반복 */}
      <SectionContainer size="sm" className="bg-muted/30 border-t">
        <div className="max-w-sm mx-auto text-center space-y-4">
          <h2 className="text-xl font-bold">{t("book.ctaBottomTitle")}</h2>
          <Button
            asChild
            className="w-full gradient-primary text-white shadow-lg rounded-full h-14 text-base"
          >
            <Link to={isLoggedIn ? "/" : "/signup"}>
              <GraduationCap className="mr-2 h-5 w-5" />
              {t("book.ctaPrimary")}
            </Link>
          </Button>
          <div className="grid grid-cols-2 gap-3">
            <Button asChild variant="outline" className="rounded-full h-12 text-sm border-2">
              <Link to="/ebook">{t("book.buyEbook")}</Link>
            </Button>
            <Button asChild variant="outline" className="rounded-full h-12 text-sm border-2">
              <Link to={isLoggedIn ? "/ebook/my" : "/signup"}>
                {isLoggedIn ? t("book.myEbooks") : t("book.signupFree")}
              </Link>
            </Button>
          </div>
        </div>
      </SectionContainer>
    </>
  );
}
