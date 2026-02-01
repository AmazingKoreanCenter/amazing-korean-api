import { useNavigate } from "react-router-dom";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { ArrowLeft, UserPlus } from "lucide-react";
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
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

import { useCreateAdminUser } from "../hook/use_admin_users";
import { adminCreateUserReqSchema, type AdminCreateUserReq } from "../types";

export function AdminUserCreate() {
  const navigate = useNavigate();
  const createMutation = useCreateAdminUser();

  const form = useForm<AdminCreateUserReq>({
    resolver: zodResolver(adminCreateUserReqSchema),
    defaultValues: {
      email: "",
      password: "",
      name: "",
      nickname: "",
      language: "",
      country: "",
      birthday: "",
      gender: "none",
      user_auth: "learner",
    },
  });

  const onSubmit = async (data: AdminCreateUserReq) => {
    try {
      // 빈 문자열 필드는 undefined로 변환 (서버에 전송하지 않음)
      const submitData = {
        ...data,
        language: data.language || undefined,
        country: data.country || undefined,
        birthday: data.birthday || undefined,
      };
      await createMutation.mutateAsync(submitData);
      toast.success("User created successfully");
      navigate("/admin/users");
    } catch {
      toast.error("Failed to create user");
    }
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-4">
        <Button variant="ghost" onClick={() => navigate("/admin/users")}>
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back
        </Button>
        <h1 className="text-2xl font-bold">Create New User</h1>
      </div>

      <form
        onSubmit={form.handleSubmit(onSubmit, (errors) => {
          const errorFields = Object.keys(errors).join(", ");
          toast.error(`Please fill in required fields: ${errorFields}`);
        })}
      >
        <Card>
          <CardHeader>
            <CardTitle>User Information</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid gap-4 md:grid-cols-2">
              {/* Email */}
              <div className="space-y-2">
                <Label htmlFor="email">Email *</Label>
                <Input
                  id="email"
                  type="email"
                  placeholder="user@example.com"
                  {...form.register("email")}
                />
                {form.formState.errors.email && (
                  <p className="text-sm text-destructive">
                    {form.formState.errors.email.message}
                  </p>
                )}
              </div>

              {/* Name */}
              <div className="space-y-2">
                <Label htmlFor="name">Name *</Label>
                <Input
                  id="name"
                  placeholder="Full name"
                  {...form.register("name")}
                />
                {form.formState.errors.name && (
                  <p className="text-sm text-destructive">
                    {form.formState.errors.name.message}
                  </p>
                )}
              </div>

              {/* Nickname */}
              <div className="space-y-2">
                <Label htmlFor="nickname">Nickname *</Label>
                <Input
                  id="nickname"
                  placeholder="Display name"
                  {...form.register("nickname")}
                />
                {form.formState.errors.nickname && (
                  <p className="text-sm text-destructive">
                    {form.formState.errors.nickname.message}
                  </p>
                )}
              </div>

              {/* Password */}
              <div className="space-y-2">
                <Label htmlFor="password">Password *</Label>
                <Input
                  id="password"
                  type="password"
                  placeholder="Minimum 8 characters"
                  {...form.register("password")}
                />
                {form.formState.errors.password && (
                  <p className="text-sm text-destructive">
                    {form.formState.errors.password.message}
                  </p>
                )}
              </div>

              {/* Language */}
              <div className="space-y-2">
                <Label htmlFor="language">Language</Label>
                <Input
                  id="language"
                  placeholder="ko, en, etc."
                  {...form.register("language")}
                />
              </div>

              {/* Country */}
              <div className="space-y-2">
                <Label htmlFor="country">Country</Label>
                <Input
                  id="country"
                  placeholder="KR, US, etc."
                  {...form.register("country")}
                />
              </div>

              {/* Birthday */}
              <div className="space-y-2">
                <Label htmlFor="birthday">Birthday</Label>
                <Input id="birthday" type="date" {...form.register("birthday")} />
              </div>

              {/* Gender */}
              <div className="space-y-2">
                <Label>Gender</Label>
                <Select
                  value={form.watch("gender") ?? "none"}
                  onValueChange={(value) =>
                    form.setValue("gender", value as AdminCreateUserReq["gender"])
                  }
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select gender" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="none">None</SelectItem>
                    <SelectItem value="male">Male</SelectItem>
                    <SelectItem value="female">Female</SelectItem>
                    <SelectItem value="other">Other</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              {/* Role - HYMN 제외 (개발자 전용) */}
              <div className="space-y-2">
                <Label>Role</Label>
                <Select
                  value={form.watch("user_auth") ?? "learner"}
                  onValueChange={(value) =>
                    form.setValue("user_auth", value as AdminCreateUserReq["user_auth"])
                  }
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select role" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="learner">Learner</SelectItem>
                    <SelectItem value="manager">Manager</SelectItem>
                    <SelectItem value="admin">Admin</SelectItem>
                  </SelectContent>
                </Select>
              </div>
            </div>

            {/* Submit */}
            <div className="flex justify-end gap-2 pt-4">
              <Button
                type="button"
                variant="outline"
                onClick={() => navigate("/admin/users")}
              >
                Cancel
              </Button>
              <Button type="submit" disabled={createMutation.isPending}>
                <UserPlus className="mr-2 h-4 w-4" />
                {createMutation.isPending ? "Creating..." : "Create User"}
              </Button>
            </div>
          </CardContent>
        </Card>
      </form>
    </div>
  );
}
