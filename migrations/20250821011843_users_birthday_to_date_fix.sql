--! up
ALTER TABLE public.users
  ALTER COLUMN user_birthday TYPE DATE USING (user_birthday::date);

--! down
ALTER TABLE public.users
  ALTER COLUMN user_birthday TYPE TIMESTAMPTZ USING (user_birthday::timestamptz);
