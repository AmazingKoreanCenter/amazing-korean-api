import { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { ArrowLeft, Save, ChevronLeft, ChevronRight } from "lucide-react";
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
import { Switch } from "@/components/ui/switch";
import { Skeleton } from "@/components/ui/skeleton";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Badge } from "@/components/ui/badge";

import {
  useAdminUserDetail,
  useUpdateAdminUser,
  useAdminUserLogs,
  useUserSelfLogs,
} from "../hook/use_admin_users";
import {
  adminUpdateUserReqSchema,
  type AdminUpdateUserReq,
  type AdminUserLogItem,
  type UserLogItem,
} from "../types";

export function AdminUserDetail() {
  const { userId } = useParams<{ userId: string }>();
  const navigate = useNavigate();
  const id = Number(userId);

  const { data: user, isLoading, isError } = useAdminUserDetail(id);
  const updateMutation = useUpdateAdminUser();

  // 10초 쿨다운 상태
  const [cooldown, setCooldown] = useState(0);

  // 로그 페이지네이션 상태
  const [adminLogsPage, setAdminLogsPage] = useState(1);
  const [userLogsPage, setUserLogsPage] = useState(1);
  const logsPageSize = 10;

  // 로그 조회 훅
  const { data: adminLogs, isLoading: adminLogsLoading } = useAdminUserLogs(id, {
    page: adminLogsPage,
    size: logsPageSize,
  });
  const { data: userLogs, isLoading: userLogsLoading } = useUserSelfLogs(id, {
    page: userLogsPage,
    size: logsPageSize,
  });

  useEffect(() => {
    if (cooldown > 0) {
      const timer = setTimeout(() => setCooldown(cooldown - 1), 1000);
      return () => clearTimeout(timer);
    }
  }, [cooldown]);

  const form = useForm<AdminUpdateUserReq>({
    resolver: zodResolver(adminUpdateUserReqSchema),
    defaultValues: {
      email: "",
      name: "",
      nickname: "",
      password: "",
      language: "",
      country: "",
      birthday: "",
      gender: "none",
      user_state: true,
      user_auth: "learner",
    },
  });

  // user 데이터가 로드되면 폼 값 업데이트
  useEffect(() => {
    if (user) {
      form.reset({
        email: user.email,
        name: user.name,
        nickname: user.nickname ?? "",
        password: "",
        language: user.language ?? "",
        country: user.country ?? "",
        birthday: user.birthday ?? "",
        gender: user.gender,
        user_state: user.user_state,
        user_auth: user.user_auth,
      });
    }
  }, [user, form]);

  const onSubmit = async (data: AdminUpdateUserReq) => {
    try {
      // 빈 문자열 password는 undefined로 변환 (서버에 전송하지 않음)
      const submitData = {
        ...data,
        password: data.password || undefined,
      };
      await updateMutation.mutateAsync({ id, data: submitData });
      toast.success("User updated successfully");
      setCooldown(10); // 10초 쿨다운 시작
      // 잠시 후 리스트로 이동
      setTimeout(() => {
        navigate("/admin/users");
      }, 1500);
    } catch {
      toast.error("Failed to update user");
    }
  };

  const isButtonDisabled = updateMutation.isPending || cooldown > 0;

  if (isLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-48" />
        <Card>
          <CardHeader>
            <Skeleton className="h-6 w-32" />
          </CardHeader>
          <CardContent className="space-y-4">
            {Array.from({ length: 6 }).map((_, i) => (
              <div key={i} className="space-y-2">
                <Skeleton className="h-4 w-20" />
                <Skeleton className="h-10 w-full" />
              </div>
            ))}
          </CardContent>
        </Card>
      </div>
    );
  }

  if (isError || !user) {
    return (
      <div className="space-y-4">
        <Button variant="ghost" onClick={() => navigate("/admin/users")}>
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back to Users
        </Button>
        <p className="text-destructive">User not found</p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-4">
        <Button variant="ghost" onClick={() => navigate("/admin/users")}>
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back
        </Button>
        <h1 className="text-2xl font-bold">Edit User #{user.id}</h1>
      </div>

      <form onSubmit={form.handleSubmit(onSubmit, (errors) => {
        const errorFields = Object.keys(errors).join(", ");
        toast.error(`Please fill in required fields: ${errorFields}`);
      })}>
        <Card>
          <CardHeader>
            <CardTitle>User Information</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid gap-4 md:grid-cols-2">
              {/* Email */}
              <div className="space-y-2">
                <Label htmlFor="email">Email</Label>
                <Input id="email" type="email" {...form.register("email")} />
                {form.formState.errors.email && (
                  <p className="text-sm text-destructive">
                    {form.formState.errors.email.message}
                  </p>
                )}
              </div>

              {/* Name */}
              <div className="space-y-2">
                <Label htmlFor="name">Name</Label>
                <Input id="name" {...form.register("name")} />
                {form.formState.errors.name && (
                  <p className="text-sm text-destructive">
                    {form.formState.errors.name.message}
                  </p>
                )}
              </div>

              {/* Nickname */}
              <div className="space-y-2">
                <Label htmlFor="nickname">Nickname</Label>
                <Input id="nickname" {...form.register("nickname")} />
              </div>

              {/* Password (optional) */}
              <div className="space-y-2">
                <Label htmlFor="password">New Password (optional)</Label>
                <Input
                  id="password"
                  type="password"
                  placeholder="Leave blank to keep current"
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
                <Input id="language" {...form.register("language")} />
              </div>

              {/* Country */}
              <div className="space-y-2">
                <Label htmlFor="country">Country</Label>
                <Input id="country" {...form.register("country")} />
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
                    form.setValue("gender", value as AdminUpdateUserReq["gender"])
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
                    form.setValue("user_auth", value as AdminUpdateUserReq["user_auth"])
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

              {/* Active Status */}
              <div className="flex items-center justify-between rounded-lg border p-4">
                <div className="space-y-0.5">
                  <Label>Active Status</Label>
                  <p className="text-sm text-muted-foreground">
                    User can access the platform
                  </p>
                </div>
                <Switch
                  checked={form.watch("user_state") ?? true}
                  onCheckedChange={(checked) => form.setValue("user_state", checked)}
                />
              </div>

              {/* Created At (read-only) */}
              <div className="space-y-2">
                <Label>Created At</Label>
                <Input
                  value={new Date(user.created_at).toLocaleString()}
                  disabled
                  className="bg-muted"
                />
              </div>

              {/* Quit At (read-only) */}
              {user.quit_at && (
                <div className="space-y-2">
                  <Label>Quit At</Label>
                  <Input
                    value={new Date(user.quit_at).toLocaleString()}
                    disabled
                    className="bg-muted"
                  />
                </div>
              )}
            </div>

            {/* Submit */}
            <div className="flex justify-end pt-4">
              <Button type="submit" disabled={isButtonDisabled}>
                <Save className="mr-2 h-4 w-4" />
                {updateMutation.isPending
                  ? "Saving..."
                  : cooldown > 0
                    ? `Wait ${cooldown}s`
                    : "Save Changes"}
              </Button>
            </div>
          </CardContent>
        </Card>
      </form>

      {/* Change History */}
      <Card>
        <CardHeader>
          <CardTitle>Change History</CardTitle>
        </CardHeader>
        <CardContent>
          <Tabs defaultValue="admin-logs">
            <TabsList className="mb-4">
              <TabsTrigger value="admin-logs">
                Admin Changes
                {adminLogs?.meta.total_count ? (
                  <Badge variant="secondary" className="ml-2">
                    {adminLogs.meta.total_count}
                  </Badge>
                ) : null}
              </TabsTrigger>
              <TabsTrigger value="user-logs">
                User Changes
                {userLogs?.meta.total_count ? (
                  <Badge variant="secondary" className="ml-2">
                    {userLogs.meta.total_count}
                  </Badge>
                ) : null}
              </TabsTrigger>
            </TabsList>

            {/* Admin Changes Tab */}
            <TabsContent value="admin-logs">
              {adminLogsLoading ? (
                <div className="space-y-2">
                  {Array.from({ length: 3 }).map((_, i) => (
                    <Skeleton key={i} className="h-16 w-full" />
                  ))}
                </div>
              ) : adminLogs?.items.length === 0 ? (
                <p className="text-muted-foreground text-center py-8">
                  No admin changes recorded
                </p>
              ) : (
                <div className="space-y-4">
                  <div className="rounded-md border">
                    <table className="w-full text-sm">
                      <thead className="border-b bg-muted/50">
                        <tr>
                          <th className="h-10 px-4 text-left font-medium">Date</th>
                          <th className="h-10 px-4 text-left font-medium">Admin</th>
                          <th className="h-10 px-4 text-left font-medium">Action</th>
                          <th className="h-10 px-4 text-left font-medium">Changes</th>
                        </tr>
                      </thead>
                      <tbody>
                        {adminLogs?.items.map((log: AdminUserLogItem) => (
                          <tr key={log.id} className="border-b">
                            <td className="p-4 whitespace-nowrap">
                              {new Date(log.created_at).toLocaleString()}
                            </td>
                            <td className="p-4">{log.admin_email || `Admin #${log.admin_id}`}</td>
                            <td className="p-4">
                              <Badge variant={log.action === "create" ? "default" : "secondary"}>
                                {log.action}
                              </Badge>
                            </td>
                            <td className="p-4">
                              <AdminLogChanges before={log.before} after={log.after} />
                            </td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>

                  {/* Admin Logs Pagination */}
                  {adminLogs && adminLogs.meta.total_pages > 1 && (
                    <div className="flex items-center justify-between">
                      <p className="text-sm text-muted-foreground">
                        Page {adminLogs.meta.current_page} of {adminLogs.meta.total_pages}
                      </p>
                      <div className="flex gap-2">
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => setAdminLogsPage((p) => Math.max(1, p - 1))}
                          disabled={adminLogsPage === 1}
                        >
                          <ChevronLeft className="h-4 w-4" />
                          Prev
                        </Button>
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => setAdminLogsPage((p) => p + 1)}
                          disabled={adminLogsPage >= adminLogs.meta.total_pages}
                        >
                          Next
                          <ChevronRight className="h-4 w-4" />
                        </Button>
                      </div>
                    </div>
                  )}
                </div>
              )}
            </TabsContent>

            {/* User Changes Tab */}
            <TabsContent value="user-logs">
              {userLogsLoading ? (
                <div className="space-y-2">
                  {Array.from({ length: 3 }).map((_, i) => (
                    <Skeleton key={i} className="h-16 w-full" />
                  ))}
                </div>
              ) : userLogs?.items.length === 0 ? (
                <p className="text-muted-foreground text-center py-8">
                  No user changes recorded
                </p>
              ) : (
                <div className="space-y-4">
                  <div className="rounded-md border">
                    <table className="w-full text-sm">
                      <thead className="border-b bg-muted/50">
                        <tr>
                          <th className="h-10 px-4 text-left font-medium">Date</th>
                          <th className="h-10 px-4 text-left font-medium">Action</th>
                          <th className="h-10 px-4 text-left font-medium">Status</th>
                          <th className="h-10 px-4 text-left font-medium">Changes</th>
                        </tr>
                      </thead>
                      <tbody>
                        {userLogs?.items.map((log: UserLogItem) => (
                          <tr key={log.id} className="border-b">
                            <td className="p-4 whitespace-nowrap">
                              {new Date(log.created_at).toLocaleString()}
                            </td>
                            <td className="p-4">
                              <Badge variant="outline">{log.action}</Badge>
                            </td>
                            <td className="p-4">
                              <Badge variant={log.success ? "default" : "destructive"}>
                                {log.success ? "Success" : "Failed"}
                              </Badge>
                            </td>
                            <td className="p-4">
                              <UserLogChanges log={log} />
                            </td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>

                  {/* User Logs Pagination */}
                  {userLogs && userLogs.meta.total_pages > 1 && (
                    <div className="flex items-center justify-between">
                      <p className="text-sm text-muted-foreground">
                        Page {userLogs.meta.current_page} of {userLogs.meta.total_pages}
                      </p>
                      <div className="flex gap-2">
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => setUserLogsPage((p) => Math.max(1, p - 1))}
                          disabled={userLogsPage === 1}
                        >
                          <ChevronLeft className="h-4 w-4" />
                          Prev
                        </Button>
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => setUserLogsPage((p) => p + 1)}
                          disabled={userLogsPage >= userLogs.meta.total_pages}
                        >
                          Next
                          <ChevronRight className="h-4 w-4" />
                        </Button>
                      </div>
                    </div>
                  )}
                </div>
              )}
            </TabsContent>
          </Tabs>
        </CardContent>
      </Card>
    </div>
  );
}

// Helper component: Admin log changes diff view
function AdminLogChanges({
  before,
  after,
}: {
  before: Record<string, unknown> | null;
  after: Record<string, unknown> | null;
}) {
  if (!before && !after) return <span className="text-muted-foreground">-</span>;

  const changes: string[] = [];

  if (after) {
    Object.keys(after).forEach((key) => {
      const beforeVal = before?.[key];
      const afterVal = after[key];
      if (JSON.stringify(beforeVal) !== JSON.stringify(afterVal)) {
        changes.push(`${key}: ${beforeVal ?? "(empty)"} → ${afterVal ?? "(empty)"}`);
      }
    });
  }

  if (changes.length === 0) {
    return <span className="text-muted-foreground">No changes</span>;
  }

  return (
    <ul className="text-xs space-y-1">
      {changes.map((change, i) => (
        <li key={i}>{change}</li>
      ))}
    </ul>
  );
}

// Helper component: User log changes view
function UserLogChanges({ log }: { log: UserLogItem }) {
  const changes: string[] = [];

  if (log.email) changes.push(`Email: ${log.email}`);
  if (log.nickname) changes.push(`Nickname: ${log.nickname}`);
  if (log.language) changes.push(`Language: ${log.language}`);
  if (log.country) changes.push(`Country: ${log.country}`);
  if (log.birthday) changes.push(`Birthday: ${log.birthday}`);
  if (log.gender) changes.push(`Gender: ${log.gender}`);
  if (log.password_changed) changes.push("Password changed");

  if (changes.length === 0) {
    return <span className="text-muted-foreground">-</span>;
  }

  return (
    <ul className="text-xs space-y-1">
      {changes.map((change, i) => (
        <li key={i}>{change}</li>
      ))}
    </ul>
  );
}
