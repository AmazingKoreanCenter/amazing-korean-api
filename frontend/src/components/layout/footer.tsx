import { useState } from "react";
import { Link } from "react-router-dom";
import { Mail, Phone, MapPin, Award, FileCheck } from "lucide-react";
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
    title: "통신판매업 신고증",
    image: "/images/certification_Online-Marketing_Business.png",
  },
  {
    id: "amazing-korean-center",
    title: "어메이징코리안센터 인증",
    image: "/images/certification_Amazing-Korean-Center.png",
  },
];

export function Footer() {
  const currentYear = new Date().getFullYear();
  const [selectedCert, setSelectedCert] = useState<typeof CERTIFICATIONS[0] | null>(null);

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
              <span className="text-xl font-bold">Amazing Korean</span>
            </Link>
            <p className="text-white/70 text-sm leading-relaxed mb-6">
              전 세계 한국어 학습자를 위한
              <br />
              최고의 온라인 학습 플랫폼
            </p>

            {/* Certification Badges */}
            <div className="space-y-3">
              <div className="flex items-center gap-2 text-white/50 text-xs">
                <Award className="h-3.5 w-3.5" />
                <span>인증서 확인</span>
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
                    {cert.title}
                  </Button>
                ))}
              </div>
            </div>
          </div>

          {/* Quick Links */}
          <div>
            <h3 className="text-base font-semibold mb-5">바로가기</h3>
            <nav className="flex flex-col gap-3">
              <Link
                to="/about"
                className="text-white/70 hover:text-white text-sm transition-colors"
              >
                서비스 소개
              </Link>
              <Link
                to="/videos"
                className="text-white/70 hover:text-white text-sm transition-colors"
              >
                영상 학습
              </Link>
              <Link
                to="/studies"
                className="text-white/70 hover:text-white text-sm transition-colors"
              >
                체계적 학습
              </Link>
              <Link
                to="/lessons"
                className="text-white/70 hover:text-white text-sm transition-colors"
              >
                1:1 수업
              </Link>
            </nav>
          </div>

          {/* Support */}
          <div>
            <h3 className="text-base font-semibold mb-5">고객지원</h3>
            <nav className="flex flex-col gap-3">
              <Link
                to="/faq"
                className="text-white/70 hover:text-white text-sm transition-colors"
              >
                자주 묻는 질문
              </Link>
              <Link
                to="/terms"
                className="text-white/70 hover:text-white text-sm transition-colors"
              >
                이용약관
              </Link>
              <Link
                to="/privacy"
                className="text-white/70 hover:text-white text-sm transition-colors"
              >
                개인정보처리방침
              </Link>
            </nav>
          </div>

          {/* Contact */}
          <div>
            <h3 className="text-base font-semibold mb-5">연락처</h3>
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
                <span>세종시 한누리대로 350 6층 SB3호</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Business Info Bar */}
      <div className="border-t border-white/10">
        <div className="max-w-[1350px] mx-auto px-6 lg:px-8 py-5">
          <p className="text-white/50 text-xs leading-relaxed text-center md:text-left">
            ㈜ 힘 | 대표 : 김경륜 | 사업자등록번호 : 505-88-03252 | 통신판매신고 : 제 2025-세종아름-0402호
          </p>
        </div>
      </div>

      {/* Bottom Bar */}
      <div className="border-t border-white/10">
        <div className="max-w-[1350px] mx-auto px-6 lg:px-8 py-6">
          <div className="flex flex-col md:flex-row justify-between items-center gap-4">
            <p className="text-white/50 text-sm">
              &copy; {currentYear} Amazing Korean. All rights reserved.
            </p>
            <div className="flex items-center gap-6">
              <Link
                to="/terms"
                className="text-white/50 hover:text-white/70 text-sm transition-colors"
              >
                이용약관
              </Link>
              <Link
                to="/privacy"
                className="text-white/50 hover:text-white/70 text-sm transition-colors"
              >
                개인정보처리방침
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
              {selectedCert?.title}
            </DialogTitle>
          </DialogHeader>
          <div className="p-6 pt-4">
            {selectedCert && (
              <img
                src={selectedCert.image}
                alt={selectedCert.title}
                className="w-full h-auto rounded-lg"
              />
            )}
          </div>
        </DialogContent>
      </Dialog>
    </footer>
  );
}
