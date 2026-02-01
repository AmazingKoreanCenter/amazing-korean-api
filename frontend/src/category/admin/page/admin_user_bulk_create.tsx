import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { ArrowLeft, Upload, FileText, AlertCircle, CheckCircle } from "lucide-react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";

import { useCreateAdminUsersBulk } from "../hook/use_admin_users";
import type { AdminCreateUserReq, AdminBulkCreateUserRes } from "../types";

interface ParsedUser extends AdminCreateUserReq {
  rowNumber: number;
  error?: string;
}

export function AdminUserBulkCreate() {
  const navigate = useNavigate();
  const bulkCreateMutation = useCreateAdminUsersBulk();

  const [parsedUsers, setParsedUsers] = useState<ParsedUser[]>([]);
  const [result, setResult] = useState<AdminBulkCreateUserRes | null>(null);

  const parseCSV = (text: string): ParsedUser[] => {
    const lines = text.trim().split("\n");
    if (lines.length < 2) return [];

    const headers = lines[0].toLowerCase().split(",").map((h) => h.trim());
    const emailIdx = headers.indexOf("email");
    const passwordIdx = headers.indexOf("password");
    const nameIdx = headers.indexOf("name");
    const nicknameIdx = headers.indexOf("nickname");
    const roleIdx = headers.indexOf("role");
    const languageIdx = headers.indexOf("language");
    const countryIdx = headers.indexOf("country");
    const birthdayIdx = headers.indexOf("birthday");
    const genderIdx = headers.indexOf("gender");

    if (emailIdx === -1 || passwordIdx === -1 || nameIdx === -1 || nicknameIdx === -1) {
      toast.error("CSV must have email, password, name, nickname columns");
      return [];
    }

    const users: ParsedUser[] = [];
    for (let i = 1; i < lines.length; i++) {
      const values = lines[i].split(",").map((v) => v.trim());
      if (values.length < 4) continue;

      const user: ParsedUser = {
        rowNumber: i + 1,
        email: values[emailIdx] || "",
        password: values[passwordIdx] || "",
        name: values[nameIdx] || "",
        nickname: values[nicknameIdx] || "",
        user_auth: roleIdx !== -1 ? values[roleIdx] : "learner",
        language: languageIdx !== -1 && values[languageIdx] ? values[languageIdx] : undefined,
        country: countryIdx !== -1 && values[countryIdx] ? values[countryIdx] : undefined,
        birthday: birthdayIdx !== -1 && values[birthdayIdx] ? values[birthdayIdx] : undefined,
        gender: genderIdx !== -1 && values[genderIdx] ? values[genderIdx] as "none" | "male" | "female" | "other" : undefined,
      };

      // 간단한 유효성 검사
      if (!user.email || !user.email.includes("@")) {
        user.error = "Invalid email";
      } else if (!user.password || user.password.length < 8) {
        user.error = "Password must be at least 8 characters";
      } else if (!user.name) {
        user.error = "Name is required";
      } else if (!user.nickname) {
        user.error = "Nickname is required";
      } else if (user.gender && !["none", "male", "female", "other"].includes(user.gender)) {
        user.error = "Invalid gender (use: none, male, female, other)";
      }

      users.push(user);
    }

    return users;
  };

  const handleFileUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (event) => {
      const text = event.target?.result as string;
      const users = parseCSV(text);
      setParsedUsers(users);
      setResult(null);
    };
    reader.readAsText(file);
  };

  const handleSubmit = async () => {
    const validUsers = parsedUsers.filter((u) => !u.error);
    if (validUsers.length === 0) {
      toast.error("No valid users to create");
      return;
    }

    try {
      const items = validUsers.map(({ email, password, name, nickname, user_auth, language, country, birthday, gender }) => ({
        email,
        password,
        name,
        nickname,
        user_auth,
        language: language || undefined,
        country: country || undefined,
        birthday: birthday || undefined,
        gender: gender || undefined,
      }));

      const res = await bulkCreateMutation.mutateAsync({ items });
      setResult(res);
      toast.success(`Created ${res.summary.success} users`);
    } catch {
      toast.error("Bulk creation failed");
    }
  };

  const validCount = parsedUsers.filter((u) => !u.error).length;
  const invalidCount = parsedUsers.filter((u) => u.error).length;

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-4">
        <Button variant="ghost" onClick={() => navigate("/admin/users")}>
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back
        </Button>
        <h1 className="text-2xl font-bold">Bulk Create Users</h1>
      </div>

      {/* CSV Format Guide */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <FileText className="h-5 w-5" />
            CSV Format
          </CardTitle>
          <CardDescription>
            Upload a CSV file with user data. Required: email, password, name, nickname. Optional: role, language, country, birthday, gender
          </CardDescription>
        </CardHeader>
        <CardContent>
          <pre className="bg-muted p-4 rounded-md text-sm overflow-x-auto">
{`email,password,name,nickname,role,language,country,birthday,gender
user1@example.com,password123,User One,user1,learner,ko,KR,1990-01-15,male
user2@example.com,password456,User Two,user2,manager,en,US,1985-06-20,female`}
          </pre>
          <p className="text-sm text-muted-foreground mt-2">
            Gender values: none, male, female, other. Birthday format: YYYY-MM-DD
          </p>
        </CardContent>
      </Card>

      {/* File Upload */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Upload className="h-5 w-5" />
            Upload CSV
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="csv-file">Select CSV File</Label>
              <Input
                id="csv-file"
                type="file"
                accept=".csv"
                onChange={handleFileUpload}
              />
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Preview */}
      {parsedUsers.length > 0 && !result && (
        <Card>
          <CardHeader>
            <CardTitle>Preview ({parsedUsers.length} rows)</CardTitle>
            <CardDescription>
              <span className="text-green-600">{validCount} valid</span>
              {invalidCount > 0 && (
                <span className="text-destructive ml-2">{invalidCount} invalid</span>
              )}
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="rounded-md border max-h-96 overflow-y-auto">
              <table className="w-full text-sm">
                <thead className="border-b bg-muted/50 sticky top-0">
                  <tr>
                    <th className="h-10 px-4 text-left font-medium">Row</th>
                    <th className="h-10 px-4 text-left font-medium">Email</th>
                    <th className="h-10 px-4 text-left font-medium">Name</th>
                    <th className="h-10 px-4 text-left font-medium">Nickname</th>
                    <th className="h-10 px-4 text-left font-medium">Role</th>
                    <th className="h-10 px-4 text-left font-medium">Language</th>
                    <th className="h-10 px-4 text-left font-medium">Country</th>
                    <th className="h-10 px-4 text-left font-medium">Birthday</th>
                    <th className="h-10 px-4 text-left font-medium">Gender</th>
                    <th className="h-10 px-4 text-left font-medium">Status</th>
                  </tr>
                </thead>
                <tbody>
                  {parsedUsers.map((user) => (
                    <tr key={user.rowNumber} className="border-b">
                      <td className="p-4">{user.rowNumber}</td>
                      <td className="p-4">{user.email}</td>
                      <td className="p-4">{user.name}</td>
                      <td className="p-4">{user.nickname}</td>
                      <td className="p-4">{user.user_auth}</td>
                      <td className="p-4">{user.language || "-"}</td>
                      <td className="p-4">{user.country || "-"}</td>
                      <td className="p-4">{user.birthday || "-"}</td>
                      <td className="p-4">{user.gender || "-"}</td>
                      <td className="p-4">
                        {user.error ? (
                          <Badge variant="destructive" className="flex items-center gap-1 w-fit">
                            <AlertCircle className="h-3 w-3" />
                            {user.error}
                          </Badge>
                        ) : (
                          <Badge variant="outline" className="flex items-center gap-1 w-fit text-green-600">
                            <CheckCircle className="h-3 w-3" />
                            Valid
                          </Badge>
                        )}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>

            <div className="flex justify-end gap-2 pt-4">
              <Button
                variant="outline"
                onClick={() => setParsedUsers([])}
              >
                Clear
              </Button>
              <Button
                onClick={handleSubmit}
                disabled={validCount === 0 || bulkCreateMutation.isPending}
              >
                {bulkCreateMutation.isPending
                  ? "Creating..."
                  : `Create ${validCount} Users`}
              </Button>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Result */}
      {result && (
        <Card>
          <CardHeader>
            <CardTitle>Result</CardTitle>
            <CardDescription>
              Total: {result.summary.total}, Success: {result.summary.success}, Failed: {result.summary.failure}
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="rounded-md border max-h-96 overflow-y-auto">
              <table className="w-full text-sm">
                <thead className="border-b bg-muted/50 sticky top-0">
                  <tr>
                    <th className="h-10 px-4 text-left font-medium">Email</th>
                    <th className="h-10 px-4 text-left font-medium">Status</th>
                    <th className="h-10 px-4 text-left font-medium">Message</th>
                  </tr>
                </thead>
                <tbody>
                  {result.results.map((item, idx) => (
                    <tr key={idx} className="border-b">
                      <td className="p-4">{item.email}</td>
                      <td className="p-4">
                        <Badge variant={item.status === 201 ? "outline" : "destructive"}>
                          {item.status}
                        </Badge>
                      </td>
                      <td className="p-4">
                        {item.error ? item.error.message : "Created"}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>

            <div className="flex justify-end gap-2 pt-4">
              <Button onClick={() => navigate("/admin/users")}>
                Back to Users
              </Button>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
