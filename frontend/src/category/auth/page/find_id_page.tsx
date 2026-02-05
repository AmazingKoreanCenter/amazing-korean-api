import { zodResolver } from "@hookform/resolvers/zod";
import { Loader2 } from "lucide-react";
import { useForm } from "react-hook-form";
import { Link } from "react-router-dom";
import { useTranslation } from "react-i18next";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { findIdReqSchema, type FindIdReq } from "@/category/auth/types";

import { useFindId } from "../hook/use_find_id";

export function FindIdPage() {
  const { t } = useTranslation();
  const findIdMutation = useFindId();

  const form = useForm<FindIdReq>({
    resolver: zodResolver(findIdReqSchema),
    mode: "onChange",
    defaultValues: {
      name: "",
      email: "",
    },
  });

  const onSubmit = (values: FindIdReq) => {
    findIdMutation.mutate(values);
  };

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-background px-4 py-10">
      <Card className="w-full max-w-md">
        <CardHeader className="space-y-2">
          <CardTitle>{t("auth.findIdTitle")}</CardTitle>
          <CardDescription>
            {t("auth.findIdDescription")}
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
              {/* 이름 */}
              <FormField
                control={form.control}
                name="name"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>{t("auth.nameLabel")}</FormLabel>
                    <FormControl>
                      <Input
                        placeholder={t("auth.namePlaceholder")}
                        autoComplete="name"
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              {/* 이메일 */}
              <FormField
                control={form.control}
                name="email"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>{t("auth.emailLabel")}</FormLabel>
                    <FormControl>
                      <Input
                        type="email"
                        placeholder={t("auth.emailPlaceholder")}
                        autoComplete="email"
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <div className="flex items-center justify-end text-sm">
                <Link
                  to="/login"
                  className="text-primary underline-offset-4 hover:underline"
                >
                  {t("auth.backToLogin")}
                </Link>
              </div>

              <Button
                type="submit"
                className="w-full"
                disabled={findIdMutation.isPending}
              >
                {findIdMutation.isPending ? (
                  <>
                    <Loader2 className="h-4 w-4 animate-spin mr-2" />
                    {t("auth.requesting")}
                  </>
                ) : (
                  t("auth.findIdButton")
                )}
              </Button>
            </form>
          </Form>
        </CardContent>
      </Card>
    </div>
  );
}
