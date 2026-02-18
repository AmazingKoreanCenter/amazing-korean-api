import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { ShieldCheck, Copy, Download, Check, Loader2 } from "lucide-react";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { mfaSetup, mfaVerifySetup } from "@/category/auth/auth_api";
import type { MfaSetupRes, MfaVerifySetupRes } from "@/category/auth/types";

type Step = "qr" | "verify" | "backup";

export function AdminMfaSetupPage() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const queryClient = useQueryClient();

  const [step, setStep] = useState<Step>("qr");
  const [setupData, setSetupData] = useState<MfaSetupRes | null>(null);
  const [code, setCode] = useState("");
  const [backupCodes, setBackupCodes] = useState<string[]>([]);
  const [showManualKey, setShowManualKey] = useState(false);
  const [backupConfirmed, setBackupConfirmed] = useState(false);
  const [copied, setCopied] = useState(false);

  // Step 1: MFA 설정 시작
  const setupMutation = useMutation({
    mutationFn: () => mfaSetup(),
    onSuccess: (data: MfaSetupRes) => {
      setSetupData(data);
    },
    onError: () => {
      toast.error(t("common.requestFailed"));
    },
  });

  // Step 2: TOTP 코드 검증
  const verifyMutation = useMutation({
    mutationFn: (verifyCode: string) => mfaVerifySetup({ code: verifyCode }),
    onSuccess: (data: MfaVerifySetupRes) => {
      setBackupCodes(data.backup_codes);
      setStep("backup");
      toast.success(t("mfa.toastMfaEnabled"));
      // Invalidate user me query to refresh mfa_enabled
      queryClient.invalidateQueries({ queryKey: ["user", "me"] });
    },
    onError: () => {
      toast.error(t("mfa.errorInvalidCode"));
      setCode("");
    },
  });

  const handleStartSetup = () => {
    setupMutation.mutate();
  };

  const handleVerify = (e: React.FormEvent) => {
    e.preventDefault();
    if (code.length < 6) return;
    verifyMutation.mutate(code);
  };

  const handleCopyBackupCodes = async () => {
    const text = backupCodes.join("\n");
    await navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleDownloadBackupCodes = () => {
    const text = `Amazing Korean - MFA Backup Codes\n${"=".repeat(40)}\n\n${backupCodes.map((c, i) => `${i + 1}. ${c}`).join("\n")}\n\nEach code can only be used once.\nKeep these codes in a safe place.`;
    const blob = new Blob([text], { type: "text/plain" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "amazing-korean-mfa-backup-codes.txt";
    a.click();
    URL.revokeObjectURL(url);
  };

  const handleComplete = () => {
    navigate("/admin", { replace: true });
  };

  // Step 1: QR 코드 스캔 (초기 화면 + QR 표시)
  if (step === "qr") {
    return (
      <div className="flex min-h-[80vh] items-center justify-center px-4">
        <Card className="w-full max-w-lg">
          <CardHeader className="text-center space-y-2">
            <div className="flex justify-center mb-2">
              <ShieldCheck className="h-12 w-12 text-primary" />
            </div>
            <CardTitle className="text-2xl">{t("mfa.setupTitle")}</CardTitle>
            <CardDescription>{t("mfa.setupRequired")}</CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            {!setupData ? (
              <>
                <p className="text-sm text-muted-foreground text-center">
                  {t("mfa.setupStep1Desc")}
                </p>
                <Button
                  className="w-full"
                  onClick={handleStartSetup}
                  disabled={setupMutation.isPending}
                >
                  {setupMutation.isPending ? (
                    <>
                      <Loader2 className="h-4 w-4 animate-spin mr-2" />
                      {t("common.loading")}
                    </>
                  ) : (
                    t("mfa.setupStep1Title")
                  )}
                </Button>
              </>
            ) : (
              <>
                <p className="text-sm text-center font-medium">
                  {t("mfa.setupStep1Title")}
                </p>
                <p className="text-sm text-muted-foreground text-center">
                  {t("mfa.setupStep1Desc")}
                </p>

                {/* QR Code */}
                <div className="flex justify-center">
                  <img
                    src={setupData.qr_code_data_uri}
                    alt="MFA QR Code"
                    className="w-48 h-48 border rounded-lg"
                  />
                </div>

                {/* Manual Key */}
                <div className="text-center">
                  <button
                    type="button"
                    className="text-sm text-primary hover:underline"
                    onClick={() => setShowManualKey(!showManualKey)}
                  >
                    {t("mfa.setupManualEntry")}
                  </button>
                  {showManualKey && (
                    <div className="mt-2 p-3 bg-muted rounded-lg">
                      <code className="text-xs break-all select-all">
                        {setupData.secret}
                      </code>
                    </div>
                  )}
                </div>

                <Button
                  className="w-full"
                  onClick={() => setStep("verify")}
                >
                  {t("mfa.nextButton")}
                </Button>
              </>
            )}
          </CardContent>
        </Card>
      </div>
    );
  }

  // Step 2: 코드 확인
  if (step === "verify") {
    return (
      <div className="flex min-h-[80vh] items-center justify-center px-4">
        <Card className="w-full max-w-lg">
          <CardHeader className="text-center space-y-2">
            <CardTitle className="text-xl">{t("mfa.setupStep2Title")}</CardTitle>
            <CardDescription>{t("mfa.setupStep2Desc")}</CardDescription>
          </CardHeader>
          <CardContent>
            <form onSubmit={handleVerify} className="space-y-4">
              <div className="space-y-2">
                <label htmlFor="totp-code" className="text-sm font-medium">
                  {t("mfa.codeLabel")}
                </label>
                <Input
                  id="totp-code"
                  type="text"
                  inputMode="numeric"
                  autoComplete="one-time-code"
                  placeholder={t("mfa.codePlaceholder")}
                  value={code}
                  onChange={(e) => setCode(e.target.value.replace(/\D/g, ""))}
                  maxLength={6}
                  autoFocus
                />
              </div>
              <Button
                type="submit"
                className="w-full"
                disabled={verifyMutation.isPending || code.length < 6}
              >
                {verifyMutation.isPending ? (
                  <>
                    <Loader2 className="h-4 w-4 animate-spin mr-2" />
                    {t("auth.verifying")}
                  </>
                ) : (
                  t("mfa.verifyButton")
                )}
              </Button>
              <Button
                type="button"
                variant="ghost"
                className="w-full"
                onClick={() => {
                  setStep("qr");
                  setCode("");
                }}
              >
                {t("common.back")}
              </Button>
            </form>
          </CardContent>
        </Card>
      </div>
    );
  }

  // Step 3: 백업 코드 저장
  return (
    <div className="flex min-h-[80vh] items-center justify-center px-4">
      <Card className="w-full max-w-lg">
        <CardHeader className="text-center space-y-2">
          <div className="flex justify-center mb-2">
            <Check className="h-12 w-12 text-status-success" />
          </div>
          <CardTitle className="text-xl">{t("mfa.setupStep3Title")}</CardTitle>
          <CardDescription>{t("mfa.setupStep3Desc")}</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          {/* Backup codes grid */}
          <div className="grid grid-cols-2 gap-2">
            {backupCodes.map((bcode, idx) => (
              <div
                key={idx}
                className="p-2 bg-muted rounded text-center font-mono text-sm select-all"
              >
                {bcode}
              </div>
            ))}
          </div>

          {/* Copy + Download buttons */}
          <div className="flex gap-2">
            <Button
              variant="outline"
              className="flex-1"
              onClick={handleCopyBackupCodes}
            >
              {copied ? (
                <>
                  <Check className="h-4 w-4 mr-2" />
                  {t("common.confirm")}
                </>
              ) : (
                <>
                  <Copy className="h-4 w-4 mr-2" />
                  {t("mfa.setupCopyAll")}
                </>
              )}
            </Button>
            <Button
              variant="outline"
              className="flex-1"
              onClick={handleDownloadBackupCodes}
            >
              <Download className="h-4 w-4 mr-2" />
              {t("mfa.setupDownload")}
            </Button>
          </div>

          {/* Confirm checkbox */}
          <label className="flex items-center gap-2 text-sm cursor-pointer">
            <input
              type="checkbox"
              checked={backupConfirmed}
              onChange={(e) => setBackupConfirmed(e.target.checked)}
              className="rounded"
            />
            {t("mfa.setupConfirm")}
          </label>

          <Button
            className="w-full"
            disabled={!backupConfirmed}
            onClick={handleComplete}
          >
            {t("mfa.setupComplete")}
          </Button>
        </CardContent>
      </Card>
    </div>
  );
}
