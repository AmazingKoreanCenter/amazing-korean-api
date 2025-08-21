-- Ensure users.user_birthday is DATE (guarded)

--! up
DO $$
BEGIN
  IF EXISTS (
    SELECT 1 FROM information_schema.columns
     WHERE table_schema = 'public'
       AND table_name   = 'users'
       AND column_name  = 'user_birthday'
       AND data_type   <> 'date'
  ) THEN
    -- 타임존 영향 없이 결정적으로 변환하고 싶으면 AT TIME ZONE 'UTC' 사용
    ALTER TABLE public.users
      ALTER COLUMN user_birthday TYPE DATE
      USING ((user_birthday AT TIME ZONE 'UTC')::date);
  END IF;
END $$;

--! down
DO $$
BEGIN
  IF EXISTS (
    SELECT 1 FROM information_schema.columns
     WHERE table_schema = 'public'
       AND table_name   = 'users'
       AND column_name  = 'user_birthday'
       AND data_type     = 'date'
  ) THEN
    ALTER TABLE public.users
      ALTER COLUMN user_birthday TYPE TIMESTAMPTZ
      USING (user_birthday::timestamptz);
  END IF;
END $$;
