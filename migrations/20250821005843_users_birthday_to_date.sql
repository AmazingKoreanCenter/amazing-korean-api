-- Change users.user_birthday TIMESTAMPTZ -> DATE

--! up
ALTER TABLE users
  ALTER COLUMN user_birthday TYPE DATE USING (user_birthday::date);

--! down
ALTER TABLE users
  ALTER COLUMN user_birthday TYPE TIMESTAMPTZ USING (user_birthday::timestamptz);
