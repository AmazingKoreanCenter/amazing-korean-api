-- Drop unused tables: course, pay

--! up
DROP TABLE IF EXISTS public.pay;
DROP TABLE IF EXISTS public.course;

--! down
-- (Optional) 필요하다면 되살리는 CREATE TABLE을 여기에 추가하세요.
-- 기존 정의는 20250812052946_core_user_course_pay.sql을 참고해 복원하면 됩니다.