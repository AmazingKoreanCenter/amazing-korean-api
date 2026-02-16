import { useState } from "react";
import { Link } from "react-router-dom";
import { Mail, Phone, MapPin, Award, FileCheck } from "lucide-react";
import { useTranslation } from "react-i18next";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";

const CERTIFICATIONS = [
  {
    id: "online-marketing",
    titleKey: "footer.certOnlineMarketing",
    image: "/images/certification_Online-Marketing_Business.png",
  },
  {
    id: "amazing-korean-center",
    titleKey: "footer.certAmazingKoreanCenter",
    image: "/images/certification_Amazing-Korean-Center.png",
  },
] as const;

export function Footer() {
  const { t } = useTranslation();
  const currentYear = new Date().getFullYear();
  const [selectedCert, setSelectedCert] = useState<typeof CERTIFICATIONS[number] | null>(null);

  return (
    <footer className="bg-[#051D55] text-white">
      {/* Main Footer Content */}
      <div className="max-w-[1350px] mx-auto px-6 lg:px-8 py-16">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-10 lg:gap-8">
          {/* Brand */}
          <div className="lg:col-span-1">
            <Link to="/" className="flex items-center gap-3 mb-6">
              <div className="w-10 h-10 rounded-xl gradient-primary flex items-center justify-center">
                <span className="text-white font-bold text-lg">A</span>
              </div>
              <span className="text-xl font-bold">{t("common.brandName")}</span>
            </Link>
            <p className="text-white/70 text-sm leading-relaxed mb-6 whitespace-pre-line">
              {t("footer.brandDescription")}
            </p>

            {/* Certification Badges */}
            <div className="space-y-3">
              <div className="flex items-center gap-2 text-white/50 text-xs">
                <Award className="h-3.5 w-3.5" />
                <span>{t("footer.certificationCheck")}</span>
              </div>
              <div className="flex flex-col gap-2">
                {CERTIFICATIONS.map((cert) => (
                  <Button
                    key={cert.id}
                    variant="ghost"
                    size="sm"
                    onClick={() => setSelectedCert(cert)}
                    className="justify-start text-white/70 hover:text-white hover:bg-white/10 px-3 h-9"
                  >
                    <FileCheck className="h-4 w-4 mr-2" />
                    {t(cert.titleKey)}
                  </Button>
                ))}
              </div>
            </div>
          </div>

          {/* Quick Links */}
          <div>
            <h3 className="text-base font-semibold mb-5">{t("footer.quickLinks")}</h3>
            <nav className="flex flex-col gap-3">
              <Link
                to="/about"
                className="text-white/70 hover:text-white text-sm transition-colors"
              >
                {t("footer.serviceIntro")}
              </Link>
              <Link
                to="/videos"
                className="text-white/70 hover:text-white text-sm transition-colors"
              >
                {t("footer.videoLearning")}
              </Link>
              <Link
                to="/studies"
                className="text-white/70 hover:text-white text-sm transition-colors"
              >
                {t("footer.structuredLearning")}
              </Link>
              <Link
                to="/lessons"
                className="text-white/70 hover:text-white text-sm transition-colors"
              >
                {t("footer.oneOnOneLesson")}
              </Link>
            </nav>
          </div>

          {/* Support */}
          <div>
            <h3 className="text-base font-semibold mb-5">{t("footer.support")}</h3>
            <nav className="flex flex-col gap-3">
              <Link
                to="/faq"
                className="text-white/70 hover:text-white text-sm transition-colors"
              >
                {t("footer.faq")}
              </Link>
              <Link
                to="/terms"
                className="text-white/70 hover:text-white text-sm transition-colors"
              >
                {t("footer.terms")}
              </Link>
              <Link
                to="/privacy"
                className="text-white/70 hover:text-white text-sm transition-colors"
              >
                {t("footer.privacy")}
              </Link>
              <Link
                to="/refund-policy"
                className="text-white/70 hover:text-white text-sm transition-colors"
              >
                {t("footer.refundPolicy")}
              </Link>
            </nav>
          </div>

          {/* Contact */}
          <div>
            <h3 className="text-base font-semibold mb-5">{t("footer.contact")}</h3>
            <div className="flex flex-col gap-4">
              <a
                href="mailto:amazingkoreancenter@gmail.com"
                className="flex items-center gap-3 text-white/70 hover:text-white text-sm transition-colors"
              >
                <Mail className="h-4 w-4 flex-shrink-0" />
                amazingkoreancenter@gmail.com
              </a>
              <div className="flex items-center gap-3 text-white/70 text-sm">
                <Phone className="h-4 w-4 flex-shrink-0" />
                0504-0821-5018
              </div>
              <div className="flex items-start gap-3 text-white/70 text-sm">
                <MapPin className="h-4 w-4 flex-shrink-0 mt-0.5" />
                <span>{t("footer.address")}</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Business Info Bar */}
      <div className="border-t border-white/10">
        <div className="max-w-[1350px] mx-auto px-6 lg:px-8 py-5">
          <p className="text-white/50 text-xs leading-relaxed text-center md:text-left">
            {t("footer.businessInfo")}
          </p>
        </div>
      </div>

      {/* Bottom Bar */}
      <div className="border-t border-white/10">
        <div className="max-w-[1350px] mx-auto px-6 lg:px-8 py-6">
          <div className="flex flex-col md:flex-row justify-between items-center gap-4">
            <p className="text-white/50 text-sm">
              {t("footer.copyright", { year: currentYear })}
            </p>
            <div className="flex items-center gap-6">
              <Link
                to="/terms"
                className="text-white/50 hover:text-white/70 text-sm transition-colors"
              >
                {t("footer.terms")}
              </Link>
              <Link
                to="/privacy"
                className="text-white/50 hover:text-white/70 text-sm transition-colors"
              >
                {t("footer.privacy")}
              </Link>
            </div>
          </div>
        </div>
      </div>

      {/* Certification Modal */}
      <Dialog open={!!selectedCert} onOpenChange={() => setSelectedCert(null)}>
        <DialogContent className="max-w-2xl p-0 overflow-hidden">
          <DialogHeader className="p-6 pb-0">
            <DialogTitle className="text-lg font-semibold">
              {selectedCert && t(selectedCert.titleKey)}
            </DialogTitle>
          </DialogHeader>
          <div className="p-6 pt-4">
            {selectedCert && (
              <img
                src={selectedCert.image}
                alt={t(selectedCert.titleKey)}
                className="w-full h-auto rounded-lg"
              />
            )}
          </div>
        </DialogContent>
      </Dialog>
    </footer>
  );
}
