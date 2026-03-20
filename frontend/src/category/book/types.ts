import { z } from "zod";

export const bookEditionSchema = z.enum(["student", "teacher"]);
export type BookEdition = z.infer<typeof bookEditionSchema>;

export interface BookInfo {
  isbn13: string;
  langKey: string;
  i18nCode: string;
  edition: BookEdition;
  nameLocal: string;
  nameKorean: string;
  sealColor: string;
  flagFile: string;
}
