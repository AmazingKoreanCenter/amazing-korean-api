import { Link } from "react-router-dom";
import { Play, BookOpen, Users, ArrowRight, CheckCircle2 } from "lucide-react";

import { Button } from "@/components/ui/button";

export default function HomePage() {
  return (
    <div className="flex flex-col">
      {/* Hero Section */}
      <section className="relative overflow-hidden bg-gradient-to-br from-[#F0F3FF] via-white to-[#E8F4FF]">
        {/* Background Decoration */}
        <div className="absolute inset-0 overflow-hidden">
          <div className="absolute -top-40 -right-40 w-80 h-80 bg-[#129DD8]/10 rounded-full blur-3xl" />
          <div className="absolute -bottom-40 -left-40 w-80 h-80 bg-[#4F71EB]/10 rounded-full blur-3xl" />
        </div>

        <div className="relative max-w-[1350px] mx-auto px-6 lg:px-8 py-20 lg:py-32">
          <div className="max-w-3xl mx-auto text-center">
            {/* Badge */}
            <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-white shadow-sm border mb-8">
              <span className="w-2 h-2 rounded-full bg-green-500 animate-pulse" />
              <span className="text-sm text-muted-foreground">
                전 세계 한국어 학습자와 함께
              </span>
            </div>

            {/* Main Heading */}
            <h1 className="text-4xl md:text-5xl lg:text-6xl font-bold tracking-tight mb-6">
              <span className="text-gradient">한국어 학습</span>의
              <br className="hidden sm:block" />
              새로운 시작
            </h1>

            {/* Description */}
            <p className="text-lg md:text-xl text-muted-foreground max-w-2xl mx-auto mb-10 leading-relaxed">
              영상, 학습 자료, 1:1 수업까지
              <br className="hidden sm:block" />
              효과적이고 즐거운 한국어 학습을 경험하세요.
            </p>

            {/* CTA Buttons */}
            <div className="flex flex-col sm:flex-row justify-center gap-4">
              <Button
                size="lg"
                asChild
                className="gradient-primary hover:opacity-90 text-white shadow-lg hover:shadow-xl transition-all rounded-full px-8 h-14 text-base"
              >
                <Link to="/signup">
                  무료로 시작하기
                  <ArrowRight className="ml-2 h-5 w-5" />
                </Link>
              </Button>
              <Button
                size="lg"
                variant="outline"
                asChild
                className="rounded-full px-8 h-14 text-base border-2 hover:bg-muted/50"
              >
                <Link to="/about">서비스 알아보기</Link>
              </Button>
            </div>

            {/* Trust Indicators */}
            <div className="flex flex-wrap justify-center gap-8 mt-12 pt-12 border-t">
              <div className="text-center">
                <div className="text-2xl font-bold text-primary">1,000+</div>
                <div className="text-sm text-muted-foreground">학습 영상</div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-primary">50+</div>
                <div className="text-sm text-muted-foreground">전문 강사</div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-primary">10,000+</div>
                <div className="text-sm text-muted-foreground">수강생</div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-20 lg:py-28">
        <div className="max-w-[1350px] mx-auto px-6 lg:px-8">
          {/* Section Header */}
          <div className="text-center mb-16">
            <h2 className="text-3xl md:text-4xl font-bold mb-4">
              Amazing Korean의 학습 방법
            </h2>
            <p className="text-muted-foreground text-lg max-w-2xl mx-auto">
              체계적인 커리큘럼과 다양한 학습 도구로
              한국어 실력을 효과적으로 향상시킵니다.
            </p>
          </div>

          {/* Feature Cards */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            {/* Video Learning */}
            <div className="group relative bg-white rounded-2xl p-8 shadow-card hover:shadow-card-hover transition-all duration-300 border">
              <div className="w-14 h-14 rounded-xl gradient-primary flex items-center justify-center mb-6 group-hover:scale-110 transition-transform">
                <Play className="h-7 w-7 text-white" />
              </div>
              <h3 className="text-xl font-semibold mb-3">영상 학습</h3>
              <p className="text-muted-foreground mb-6 leading-relaxed">
                다양한 주제의 한국어 영상을 통해 자연스럽게 듣기와 말하기 실력을 향상시킵니다.
              </p>
              <ul className="space-y-2 mb-6">
                <li className="flex items-center gap-2 text-sm text-muted-foreground">
                  <CheckCircle2 className="h-4 w-4 text-accent" />
                  <span>자막 지원</span>
                </li>
                <li className="flex items-center gap-2 text-sm text-muted-foreground">
                  <CheckCircle2 className="h-4 w-4 text-accent" />
                  <span>반복 재생 기능</span>
                </li>
                <li className="flex items-center gap-2 text-sm text-muted-foreground">
                  <CheckCircle2 className="h-4 w-4 text-accent" />
                  <span>진도 관리</span>
                </li>
              </ul>
              <Button
                variant="ghost"
                asChild
                className="p-0 h-auto text-primary hover:text-primary/80 group-hover:translate-x-1 transition-transform"
              >
                <Link to="/videos" className="flex items-center gap-1">
                  영상 보기 <ArrowRight className="h-4 w-4" />
                </Link>
              </Button>
            </div>

            {/* Structured Learning */}
            <div className="group relative bg-white rounded-2xl p-8 shadow-card hover:shadow-card-hover transition-all duration-300 border">
              <div className="w-14 h-14 rounded-xl gradient-primary flex items-center justify-center mb-6 group-hover:scale-110 transition-transform">
                <BookOpen className="h-7 w-7 text-white" />
              </div>
              <h3 className="text-xl font-semibold mb-3">체계적 학습</h3>
              <p className="text-muted-foreground mb-6 leading-relaxed">
                단계별 커리큘럼으로 문법, 어휘, 표현을 체계적으로 학습할 수 있습니다.
              </p>
              <ul className="space-y-2 mb-6">
                <li className="flex items-center gap-2 text-sm text-muted-foreground">
                  <CheckCircle2 className="h-4 w-4 text-accent" />
                  <span>레벨별 커리큘럼</span>
                </li>
                <li className="flex items-center gap-2 text-sm text-muted-foreground">
                  <CheckCircle2 className="h-4 w-4 text-accent" />
                  <span>연습 문제</span>
                </li>
                <li className="flex items-center gap-2 text-sm text-muted-foreground">
                  <CheckCircle2 className="h-4 w-4 text-accent" />
                  <span>학습 기록</span>
                </li>
              </ul>
              <Button
                variant="ghost"
                asChild
                className="p-0 h-auto text-primary hover:text-primary/80 group-hover:translate-x-1 transition-transform"
              >
                <Link to="/studies" className="flex items-center gap-1">
                  학습하기 <ArrowRight className="h-4 w-4" />
                </Link>
              </Button>
            </div>

            {/* 1:1 Lessons */}
            <div className="group relative bg-white rounded-2xl p-8 shadow-card hover:shadow-card-hover transition-all duration-300 border">
              <div className="w-14 h-14 rounded-xl gradient-primary flex items-center justify-center mb-6 group-hover:scale-110 transition-transform">
                <Users className="h-7 w-7 text-white" />
              </div>
              <h3 className="text-xl font-semibold mb-3">1:1 수업</h3>
              <p className="text-muted-foreground mb-6 leading-relaxed">
                전문 강사와 함께하는 맞춤형 수업으로 빠르게 실력을 향상시킵니다.
              </p>
              <ul className="space-y-2 mb-6">
                <li className="flex items-center gap-2 text-sm text-muted-foreground">
                  <CheckCircle2 className="h-4 w-4 text-accent" />
                  <span>전문 강사진</span>
                </li>
                <li className="flex items-center gap-2 text-sm text-muted-foreground">
                  <CheckCircle2 className="h-4 w-4 text-accent" />
                  <span>맞춤형 피드백</span>
                </li>
                <li className="flex items-center gap-2 text-sm text-muted-foreground">
                  <CheckCircle2 className="h-4 w-4 text-accent" />
                  <span>유연한 스케줄</span>
                </li>
              </ul>
              <Button
                variant="ghost"
                asChild
                className="p-0 h-auto text-primary hover:text-primary/80 group-hover:translate-x-1 transition-transform"
              >
                <Link to="/lessons" className="flex items-center gap-1">
                  수업 보기 <ArrowRight className="h-4 w-4" />
                </Link>
              </Button>
            </div>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-20 lg:py-28 bg-[#051D55]">
        <div className="max-w-[1350px] mx-auto px-6 lg:px-8 text-center">
          <h2 className="text-3xl md:text-4xl font-bold text-white mb-4">
            지금 바로 시작하세요
          </h2>
          <p className="text-white/70 text-lg max-w-xl mx-auto mb-10">
            회원가입하고 무료 콘텐츠를 체험해보세요.
            <br />
            한국어 학습의 새로운 경험이 기다리고 있습니다.
          </p>
          <div className="flex flex-col sm:flex-row justify-center gap-4">
            <Button
              size="lg"
              asChild
              className="gradient-primary hover:opacity-90 text-white shadow-lg hover:shadow-xl transition-all rounded-full px-8 h-14 text-base"
            >
              <Link to="/signup">
                무료로 시작하기
                <ArrowRight className="ml-2 h-5 w-5" />
              </Link>
            </Button>
            <Button
              size="lg"
              variant="outline"
              asChild
              className="rounded-full px-8 h-14 text-base border-2 border-white/30 text-white hover:bg-white/10 hover:border-white/50"
            >
              <Link to="/login">이미 계정이 있으신가요?</Link>
            </Button>
          </div>
        </div>
      </section>
    </div>
  );
}
