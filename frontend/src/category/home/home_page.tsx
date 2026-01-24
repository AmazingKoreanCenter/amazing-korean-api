import { Link } from "react-router-dom";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export default function HomePage() {
  return (
    <div className="flex flex-col">
      {/* Hero Section */}
      <section className="py-20 px-4 bg-gradient-to-b from-primary/5 to-background">
        <div className="container mx-auto text-center">
          <h1 className="text-4xl md:text-5xl font-bold tracking-tight text-primary mb-6">
            한국어 학습의 새로운 시작
          </h1>
          <p className="text-xl text-muted-foreground max-w-2xl mx-auto mb-8">
            Amazing Korean과 함께 효과적이고 즐거운 한국어 학습을 경험하세요.
            영상, 학습 자료, 1:1 수업까지 모든 것을 한 곳에서 만나보세요.
          </p>
          <div className="flex flex-wrap justify-center gap-4">
            <Button size="lg" asChild>
              <Link to="/videos">영상 보러가기</Link>
            </Button>
            <Button size="lg" variant="outline" asChild>
              <Link to="/about">서비스 소개</Link>
            </Button>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-16 px-4">
        <div className="container mx-auto">
          <h2 className="text-2xl font-semibold text-center mb-10">
            Amazing Korean의 학습 방법
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <Card className="hover:shadow-lg transition-shadow">
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <span className="text-2xl">🎬</span>
                  영상 학습
                </CardTitle>
              </CardHeader>
              <CardContent>
                <p className="text-muted-foreground mb-4">
                  다양한 주제의 한국어 영상을 통해 자연스럽게 듣기와 말하기 실력을 향상시킵니다.
                </p>
                <Button variant="ghost" asChild className="p-0">
                  <Link to="/videos">영상 보기 →</Link>
                </Button>
              </CardContent>
            </Card>

            <Card className="hover:shadow-lg transition-shadow">
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <span className="text-2xl">📚</span>
                  체계적 학습
                </CardTitle>
              </CardHeader>
              <CardContent>
                <p className="text-muted-foreground mb-4">
                  단계별 커리큘럼으로 문법, 어휘, 표현을 체계적으로 학습할 수 있습니다.
                </p>
                <Button variant="ghost" asChild className="p-0">
                  <Link to="/studies">학습하기 →</Link>
                </Button>
              </CardContent>
            </Card>

            <Card className="hover:shadow-lg transition-shadow">
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <span className="text-2xl">👩‍🏫</span>
                  1:1 수업
                </CardTitle>
              </CardHeader>
              <CardContent>
                <p className="text-muted-foreground mb-4">
                  전문 강사와 함께하는 맞춤형 수업으로 빠르게 실력을 향상시킵니다.
                </p>
                <Button variant="ghost" asChild className="p-0">
                  <Link to="/lessons">수업 보기 →</Link>
                </Button>
              </CardContent>
            </Card>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-16 px-4 bg-muted/50">
        <div className="container mx-auto text-center">
          <h2 className="text-2xl font-semibold mb-4">
            지금 바로 시작하세요
          </h2>
          <p className="text-muted-foreground mb-6 max-w-xl mx-auto">
            회원가입하고 무료 콘텐츠를 체험해보세요.
          </p>
          <div className="flex flex-wrap justify-center gap-4">
            <Button size="lg" asChild>
              <Link to="/signup">무료로 시작하기</Link>
            </Button>
            <Button size="lg" variant="outline" asChild>
              <Link to="/login">로그인</Link>
            </Button>
          </div>
        </div>
      </section>
    </div>
  );
}
