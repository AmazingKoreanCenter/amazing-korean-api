import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export function AboutPage() {
  return (
    <div className="container mx-auto px-4 py-8">
      {/* Hero Section */}
      <section className="text-center py-12">
        <h1 className="text-4xl font-bold tracking-tight text-primary mb-4">
          Amazing Korean
        </h1>
        <p className="text-xl text-muted-foreground max-w-2xl mx-auto">
          한국어 학습의 새로운 경험을 제공합니다.
        </p>
      </section>

      {/* Features Section */}
      <section className="py-12">
        <h2 className="text-2xl font-semibold text-center mb-8">서비스 특징</h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <Card>
            <CardHeader>
              <CardTitle className="text-lg">영상 학습</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-muted-foreground">
                다양한 한국어 영상 콘텐츠를 통해 자연스럽게 한국어를 배울 수 있습니다.
              </p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle className="text-lg">체계적인 학습</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-muted-foreground">
                단계별 학습 프로그램으로 체계적으로 한국어 실력을 향상시킬 수 있습니다.
              </p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle className="text-lg">1:1 수업</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-muted-foreground">
                전문 강사와 함께하는 맞춤형 수업으로 실력을 빠르게 향상시킬 수 있습니다.
              </p>
            </CardContent>
          </Card>
        </div>
      </section>

      {/* Mission Section */}
      <section className="py-12 bg-muted/30 rounded-lg px-8">
        <h2 className="text-2xl font-semibold text-center mb-6">우리의 미션</h2>
        <p className="text-center text-muted-foreground max-w-3xl mx-auto leading-relaxed">
          Amazing Korean은 전 세계 한국어 학습자들에게 효과적이고 즐거운 학습 경험을 제공하는 것을
          목표로 합니다. 최신 교육 기술과 전문 콘텐츠를 통해 누구나 쉽게 한국어를 배울 수 있도록
          돕고 있습니다.
        </p>
      </section>
    </div>
  );
}
