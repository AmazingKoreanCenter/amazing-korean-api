import { Link } from "react-router-dom";

export function Footer() {
  const currentYear = new Date().getFullYear();

  return (
    <footer className="border-t bg-muted/50">
      <div className="container mx-auto px-4 py-8">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
          {/* Company Info */}
          <div className="space-y-4">
            <h3 className="text-lg font-semibold text-primary">Amazing Korean</h3>
            <div className="text-sm text-muted-foreground space-y-1">
              <p>대표: [대표자명]</p>
              <p>사업자등록번호: [사업자번호]</p>
              <p>주소: [회사 주소]</p>
            </div>
          </div>

          {/* Contact Info */}
          <div className="space-y-4">
            <h3 className="text-lg font-semibold">연락처</h3>
            <div className="text-sm text-muted-foreground space-y-1">
              <p>이메일: contact@amazingkorean.net</p>
              <p>전화: [전화번호]</p>
            </div>
          </div>

          {/* Quick Links */}
          <div className="space-y-4">
            <h3 className="text-lg font-semibold">바로가기</h3>
            <nav className="flex flex-col gap-2 text-sm">
              <Link
                to="/about"
                className="text-muted-foreground hover:text-primary transition-colors"
              >
                서비스 소개
              </Link>
              <Link
                to="/terms"
                className="text-muted-foreground hover:text-primary transition-colors"
              >
                이용약관
              </Link>
              <Link
                to="/privacy"
                className="text-muted-foreground hover:text-primary transition-colors"
              >
                개인정보처리방침
              </Link>
            </nav>
          </div>
        </div>

        {/* Copyright */}
        <div className="mt-8 pt-8 border-t text-center text-sm text-muted-foreground">
          <p>&copy; {currentYear} Amazing Korean. All rights reserved.</p>
        </div>
      </div>
    </footer>
  );
}
