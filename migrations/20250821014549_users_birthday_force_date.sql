-- Force users.user_birthday to DATE (no down)

ALTER TABLE public.users
  ALTER COLUMN user_birthday TYPE DATE
  USING ((user_birthday AT TIME ZONE 'UTC')::date);
