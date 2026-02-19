import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { Mail, Send } from "lucide-react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";

import { useSendTestEmail } from "../hook/use_admin_email";
import { testEmailReqSchema, type TestEmailReq, type EmailTemplateType } from "../types";

const templateOptions: { value: EmailTemplateType; label: string; description: string }[] = [
  {
    value: "password_reset",
    label: "비밀번호 재설정",
    description: "비밀번호 재설정 인증 코드 (테스트: 123456)",
  },
  {
    value: "email_verification",
    label: "이메일 인증",
    description: "회원가입 이메일 인증 코드 (테스트: 654321)",
  },
  {
    value: "welcome",
    label: "환영 이메일",
    description: "신규 가입 환영 메시지",
  },
];

export function AdminEmailTest() {
  const sendMutation = useSendTestEmail();

  const form = useForm<TestEmailReq>({
    resolver: zodResolver(testEmailReqSchema),
    defaultValues: {
      to: "",
      template: "password_reset",
    },
  });

  const onSubmit = async (data: TestEmailReq) => {
    try {
      const result = await sendMutation.mutateAsync(data);
      toast.success(result.message);
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : "이메일 발송에 실패했습니다.";
      toast.error(errorMessage);
    }
  };

  const selectedTemplate = templateOptions.find(
    (opt) => opt.value === form.watch("template")
  );

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold flex items-center gap-2">
          <Mail className="h-6 w-6" />
          이메일 테스트
        </h1>
        <p className="text-muted-foreground">
          AWS SES 이메일 발송 기능을 테스트합니다.
        </p>
      </div>

      <div className="grid gap-6 md:grid-cols-2">
        {/* 발송 폼 */}
        <Card>
          <CardHeader>
            <CardTitle>테스트 이메일 발송</CardTitle>
            <CardDescription>
              템플릿을 선택하고 수신자 이메일을 입력하세요.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
              {/* 수신자 이메일 */}
              <div className="space-y-2">
                <Label htmlFor="to">수신자 이메일 *</Label>
                <Input
                  id="to"
                  type="email"
                  placeholder="test@example.com"
                  {...form.register("to")}
                />
                {form.formState.errors.to && (
                  <p className="text-sm text-destructive">
                    {form.formState.errors.to.message}
                  </p>
                )}
              </div>

              {/* 템플릿 선택 */}
              <div className="space-y-2">
                <Label>이메일 템플릿 *</Label>
                <Select
                  value={form.watch("template")}
                  onValueChange={(value) =>
                    form.setValue("template", value as EmailTemplateType)
                  }
                >
                  <SelectTrigger>
                    <SelectValue placeholder="템플릿 선택" />
                  </SelectTrigger>
                  <SelectContent>
                    {templateOptions.map((opt) => (
                      <SelectItem key={opt.value} value={opt.value}>
                        {opt.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                {selectedTemplate && (
                  <p className="text-sm text-muted-foreground">
                    {selectedTemplate.description}
                  </p>
                )}
              </div>

              {/* 발송 버튼 */}
              <Button
                type="submit"
                className="w-full"
                disabled={sendMutation.isPending}
              >
                <Send className="mr-2 h-4 w-4" />
                {sendMutation.isPending ? "발송 중..." : "테스트 이메일 발송"}
              </Button>
            </form>
          </CardContent>
        </Card>

        {/* 안내 */}
        <div className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>템플릿 미리보기</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              {selectedTemplate?.value === "password_reset" && (
                <div className="border rounded-lg p-4 bg-muted">
                  <h3 className="font-semibold mb-2">비밀번호 재설정</h3>
                  <p className="text-sm text-muted-foreground mb-4">
                    아래 인증 코드를 입력하여 비밀번호를 재설정하세요.
                  </p>
                  <div className="bg-card border rounded-lg p-4 text-center">
                    <span className="text-3xl font-bold tracking-widest">123456</span>
                  </div>
                  <p className="text-xs text-muted-foreground mt-2">
                    이 코드는 10분 후 만료됩니다.
                  </p>
                </div>
              )}
              {selectedTemplate?.value === "email_verification" && (
                <div className="border rounded-lg p-4 bg-muted">
                  <h3 className="font-semibold mb-2">이메일 인증</h3>
                  <p className="text-sm text-muted-foreground mb-4">
                    Amazing Korean에 가입해 주셔서 감사합니다!
                    아래 인증 코드를 입력하여 이메일을 인증하세요.
                  </p>
                  <div className="bg-card border rounded-lg p-4 text-center">
                    <span className="text-3xl font-bold tracking-widest">654321</span>
                  </div>
                  <p className="text-xs text-muted-foreground mt-2">
                    이 코드는 10분 후 만료됩니다.
                  </p>
                </div>
              )}
              {selectedTemplate?.value === "welcome" && (
                <div className="border rounded-lg p-4 bg-muted">
                  <h3 className="font-semibold mb-2">테스트 사용자님, 환영합니다!</h3>
                  <p className="text-sm text-muted-foreground mb-4">
                    Amazing Korean에 가입해 주셔서 감사합니다.
                    지금 바로 한국어 학습을 시작해 보세요!
                  </p>
                  <Button size="sm" variant="outline" disabled>
                    학습 시작하기
                  </Button>
                </div>
              )}
            </CardContent>
          </Card>

          <Card className="bg-primary/5 border-primary/20">
            <CardContent className="pt-4">
              <p className="text-sm text-primary">
                <strong>참고:</strong> AWS SES가 샌드박스 모드인 경우, 인증된 이메일 주소로만 발송할 수 있습니다.
                운영 환경에서는 샌드박스 해제가 필요합니다.
              </p>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}
