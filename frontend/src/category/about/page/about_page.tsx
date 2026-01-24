import { Link } from "react-router-dom";
import { Target, Heart, Globe, ArrowRight, Sparkles } from "lucide-react";

import { Button } from "@/components/ui/button";

export function AboutPage() {
  return (
    <div className="flex flex-col">
      {/* Hero Section */}
      <section className="relative overflow-hidden bg-gradient-to-br from-[#F0F3FF] via-white to-[#E8F4FF]">
        <div className="absolute inset-0 overflow-hidden">
          <div className="absolute -top-40 -right-40 w-80 h-80 bg-[#129DD8]/10 rounded-full blur-3xl" />
          <div className="absolute -bottom-40 -left-40 w-80 h-80 bg-[#4F71EB]/10 rounded-full blur-3xl" />
        </div>

        <div className="relative max-w-[1350px] mx-auto px-6 lg:px-8 py-20 lg:py-28">
          <div className="max-w-3xl mx-auto text-center">
            <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-white shadow-sm border mb-8">
              <Sparkles className="h-4 w-4 text-accent" />
              <span className="text-sm text-muted-foreground">About Us</span>
            </div>

            <h1 className="text-4xl md:text-5xl lg:text-6xl font-bold tracking-tight mb-6">
              <span className="text-gradient">Amazing Korean</span>
            </h1>

            <p className="text-lg md:text-xl text-muted-foreground max-w-2xl mx-auto leading-relaxed">
              전 세계 한국어 학습자들에게 효과적이고 즐거운
              <br className="hidden sm:block" />
              학습 경험을 제공합니다.
            </p>
          </div>
        </div>
      </section>

      {/* Mission Section */}
      <section className="py-20 lg:py-28">
        <div className="max-w-[1350px] mx-auto px-6 lg:px-8">
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-12 lg:gap-20 items-center">
            <div>
              <h2 className="text-3xl md:text-4xl font-bold mb-6">
                우리의 미션
              </h2>
              <p className="text-muted-foreground text-lg leading-relaxed mb-6">
                Amazing Korean은 한국어를 배우고자 하는 전 세계 학습자들에게
                최고의 학습 경험을 제공하기 위해 탄생했습니다.
              </p>
              <p className="text-muted-foreground text-lg leading-relaxed mb-8">
                최신 교육 기술과 전문 콘텐츠를 통해 누구나 쉽고 효과적으로
                한국어를 배울 수 있도록 돕고 있습니다.
              </p>
              <Button
                size="lg"
                asChild
                className="gradient-primary hover:opacity-90 text-white rounded-full px-8"
              >
                <Link to="/signup">
                  학습 시작하기
                  <ArrowRight className="ml-2 h-5 w-5" />
                </Link>
              </Button>
            </div>

            <div className="relative">
              <div className="bg-gradient-to-br from-[#4F71EB] to-[#129DD8] rounded-3xl p-10 text-white">
                <div className="text-6xl font-bold mb-2">2024</div>
                <div className="text-white/80 text-lg mb-8">서비스 시작</div>
                <div className="grid grid-cols-2 gap-6">
                  <div>
                    <div className="text-3xl font-bold">10,000+</div>
                    <div className="text-white/70">수강생</div>
                  </div>
                  <div>
                    <div className="text-3xl font-bold">50+</div>
                    <div className="text-white/70">전문 강사</div>
                  </div>
                  <div>
                    <div className="text-3xl font-bold">1,000+</div>
                    <div className="text-white/70">학습 콘텐츠</div>
                  </div>
                  <div>
                    <div className="text-3xl font-bold">30+</div>
                    <div className="text-white/70">국가</div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Values Section */}
      <section className="py-20 lg:py-28 bg-muted/30">
        <div className="max-w-[1350px] mx-auto px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-3xl md:text-4xl font-bold mb-4">핵심 가치</h2>
            <p className="text-muted-foreground text-lg max-w-2xl mx-auto">
              Amazing Korean이 추구하는 가치입니다.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            <div className="bg-white rounded-2xl p-8 shadow-card text-center">
              <div className="w-16 h-16 rounded-2xl gradient-primary flex items-center justify-center mx-auto mb-6">
                <Target className="h-8 w-8 text-white" />
              </div>
              <h3 className="text-xl font-semibold mb-3">효과적인 학습</h3>
              <p className="text-muted-foreground leading-relaxed">
                검증된 교수법과 체계적인 커리큘럼으로 학습 효과를 극대화합니다.
              </p>
            </div>

            <div className="bg-white rounded-2xl p-8 shadow-card text-center">
              <div className="w-16 h-16 rounded-2xl gradient-primary flex items-center justify-center mx-auto mb-6">
                <Heart className="h-8 w-8 text-white" />
              </div>
              <h3 className="text-xl font-semibold mb-3">학습자 중심</h3>
              <p className="text-muted-foreground leading-relaxed">
                학습자의 목표와 수준에 맞춘 맞춤형 학습 경험을 제공합니다.
              </p>
            </div>

            <div className="bg-white rounded-2xl p-8 shadow-card text-center">
              <div className="w-16 h-16 rounded-2xl gradient-primary flex items-center justify-center mx-auto mb-6">
                <Globe className="h-8 w-8 text-white" />
              </div>
              <h3 className="text-xl font-semibold mb-3">글로벌 접근성</h3>
              <p className="text-muted-foreground leading-relaxed">
                언제 어디서나 접근 가능한 온라인 플랫폼으로 학습 장벽을 낮춥니다.
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-20 lg:py-28 bg-[#051D55]">
        <div className="max-w-[1350px] mx-auto px-6 lg:px-8 text-center">
          <h2 className="text-3xl md:text-4xl font-bold text-white mb-4">
            함께 한국어를 배워보세요
          </h2>
          <p className="text-white/70 text-lg max-w-xl mx-auto mb-10">
            Amazing Korean과 함께라면 한국어 학습이 즐거워집니다.
          </p>
          <div className="flex flex-col sm:flex-row justify-center gap-4">
            <Button
              size="lg"
              asChild
              className="gradient-primary hover:opacity-90 text-white shadow-lg rounded-full px-8 h-14 text-base"
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
              <Link to="/videos">영상 둘러보기</Link>
            </Button>
          </div>
        </div>
      </section>
    </div>
  );
}
