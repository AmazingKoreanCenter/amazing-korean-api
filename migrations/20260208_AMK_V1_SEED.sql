-- AMK V1 시드 데이터 (2026-02-08)
-- 로컬 DB에서 추출한 콘텐츠 데이터 (video, study, lesson)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);

--
-- Data for Name: lesson; Type: TABLE DATA
--

INSERT INTO public.lesson VALUES (1, NULL, '1', '한글 자음 연습', '동영상:1 / 연습:9', 'video : 1 / typing : 9', '2026-01-02 08:13:30.33287+00', '2026-01-02 08:13:30.33287+00', 'ready', 'public');
INSERT INTO public.lesson VALUES (2, NULL, '2', '동사 시제 연습 - 객관식', '동영상:5 / 연습:5', 'video : 5 / choice : 5', '2026-01-02 08:13:30.33287+00', '2026-01-02 08:13:30.33287+00', 'ready', 'public');
INSERT INTO public.lesson VALUES (3, NULL, '3', '동사 시제 연습 - 말하기', '동영상:5 / 연습:5', 'video : 5 / typing : 5', '2026-01-02 08:13:30.33287+00', '2026-01-02 08:13:30.33287+00', 'ready', 'public');
INSERT INTO public.lesson VALUES (5, NULL, '5', '레슨_테스트_02', '연습:10_2', 'voice : 5 / typing : 5', '2026-01-08 04:45:19.181769+00', '2026-01-08 06:07:27.502258+00', 'ready', 'public');
INSERT INTO public.lesson VALUES (6, NULL, '6', '레슨_테스트_03', '연습:10_3', 'voice : 5 / typing : 5', '2026-01-08 04:45:19.190289+00', '2026-01-08 06:07:27.512346+00', 'ready', 'public');
INSERT INTO public.lesson VALUES (7, NULL, '7', '레슨_테스트_04', '연습:10_4', 'voice : 5 / typing : 5', '2026-01-08 04:45:19.200708+00', '2026-01-08 06:07:27.520383+00', 'ready', 'public');
INSERT INTO public.lesson VALUES (8, NULL, '8', '레슨_테스트_05', '연습:10_5', 'voice : 5 / typing : 5', '2026-01-08 04:45:19.207034+00', '2026-01-08 06:07:27.525978+00', 'ready', 'public');
INSERT INTO public.lesson VALUES (4, NULL, '4', '레슨_테스트_01', '동영상:5 / 연습: 5', 'video : 5 / voice : 5', '2026-01-08 04:29:22.908901+00', '2026-01-08 06:23:15.696565+00', 'ready', 'public');

--
-- Data for Name: study; Type: TABLE DATA
--

INSERT INTO public.study VALUES (1, NULL, 'test-1', 'open', 'basic_word', '한글 자음 연습', '"ㅏ"로 자음 연습 하기', '가, 나, 다, 라, 마...', '2026-01-01 08:09:39.18471+00', '2026-01-01 08:09:39.18471+00', 'public');
INSERT INTO public.study VALUES (2, NULL, 'test-2', 'open', 'topik_read', 'TOPIK I 읽기 문제 연습', '그림 유형 1', '10문제로 읽기 그림 유형 1 마스터!', '2026-01-01 08:09:39.18471+00', '2026-01-01 08:09:39.18471+00', 'public');
INSERT INTO public.study VALUES (3, NULL, 'test-3', 'open', 'topik_listen', 'TOPIK I 듣기 문제 연습', '5번 모음집', '10문제로 듣기 5번 유형 마스터!', '2026-01-01 08:09:39.18471+00', '2026-01-01 08:09:39.18471+00', 'public');
INSERT INTO public.study VALUES (4, NULL, 'test-4', 'ready', 'topik_write', 'TOPIK I 쓰기 문제 연습', '시험 대비 집중 과정', '선생님이 직접 관리하는 클래스', '2026-01-01 08:09:39.18471+00', '2026-01-01 08:09:39.18471+00', 'public');
INSERT INTO public.study VALUES (5, NULL, 'test-5', 'close', 'basic_900', '기초 한국어 900', '900문장으로 끝나는 한국어 기초', '효율적인 한국어 공부 방법!!!!!!!', '2026-01-01 08:09:39.18471+00', '2026-01-01 08:09:39.18471+00', 'public');
INSERT INTO public.study VALUES (6, NULL, 'test-6', 'ready', 'basic_900', '관리자 테스트용 제목_0', '관리자 테스트용 부제목_0', '관리자 테스트용 설명란_0.', '2026-01-07 02:31:50.160394+00', '2026-01-07 03:23:04.776127+00', 'public');
INSERT INTO public.study VALUES (7, NULL, 'test-7', 'ready', 'basic_900', '관리자 테스트용 제목_01', '관리자 테스트용 부제목_01', '관리자 테스트용 설명란_01.', '2026-01-07 02:56:53.475058+00', '2026-01-07 03:48:34.563665+00', 'public');
INSERT INTO public.study VALUES (8, NULL, 'test-8', 'ready', 'basic_900', '관리자 테스트용 제목_02', '관리자 테스트용 부제목_02', '관리자 테스트용 설명란_02.', '2026-01-07 02:56:53.485316+00', '2026-01-07 03:48:34.579357+00', 'public');
INSERT INTO public.study VALUES (9, NULL, 'test-9', 'ready', 'basic_900', '관리자 테스트용 제목_03', '관리자 테스트용 부제목_03', '관리자 테스트용 설명란_03.', '2026-01-07 02:56:53.490416+00', '2026-01-07 03:48:34.58517+00', 'public');

--
-- Data for Name: study_task; Type: TABLE DATA
--

INSERT INTO public.study_task VALUES (1, 1, NULL, 'typing', 1, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (2, 1, NULL, 'typing', 2, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (3, 1, NULL, 'typing', 3, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (4, 1, NULL, 'typing', 4, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (5, 1, NULL, 'typing', 5, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (6, 1, NULL, 'typing', 6, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (7, 1, NULL, 'typing', 7, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (8, 1, NULL, 'typing', 8, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (9, 1, NULL, 'typing', 9, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (10, 1, NULL, 'typing', 10, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (11, 1, NULL, 'typing', 11, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (12, 1, NULL, 'typing', 12, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (13, 1, NULL, 'typing', 13, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (14, 1, NULL, 'typing', 14, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (15, 2, NULL, 'choice', 1, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (16, 2, NULL, 'choice', 2, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (17, 2, NULL, 'choice', 3, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (18, 2, NULL, 'choice', 4, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (19, 2, NULL, 'choice', 5, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (20, 2, NULL, 'choice', 6, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (21, 2, NULL, 'choice', 7, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (22, 2, NULL, 'choice', 8, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (23, 2, NULL, 'choice', 9, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (24, 2, NULL, 'choice', 10, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (25, 3, NULL, 'voice', 1, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (26, 3, NULL, 'voice', 2, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (27, 3, NULL, 'voice', 3, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (28, 3, NULL, 'voice', 4, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (29, 3, NULL, 'voice', 5, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (30, 4, NULL, 'typing', 1, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (31, 4, NULL, 'typing', 2, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (32, 4, NULL, 'typing', 3, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (33, 4, NULL, 'typing', 4, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (34, 4, NULL, 'typing', 5, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (35, 5, NULL, 'choice', 1, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (36, 5, NULL, 'choice', 2, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (37, 5, NULL, 'choice', 3, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (38, 5, NULL, 'choice', 4, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (39, 5, NULL, 'choice', 5, '2026-01-01 08:10:15.386886+00', '2026-01-01 08:10:15.386886+00');
INSERT INTO public.study_task VALUES (41, 6, NULL, 'choice', 2, '2026-01-07 06:00:36.659421+00', '2026-01-07 06:00:36.659421+00');
INSERT INTO public.study_task VALUES (42, 6, NULL, 'choice', 3, '2026-01-07 06:00:36.676053+00', '2026-01-07 06:00:36.676053+00');
INSERT INTO public.study_task VALUES (43, 6, NULL, 'choice', 4, '2026-01-07 06:00:36.684915+00', '2026-01-07 06:00:36.684915+00');
INSERT INTO public.study_task VALUES (40, 6, NULL, 'choice', 1, '2026-01-07 05:39:16.193224+00', '2026-01-07 06:02:43.011397+00');
INSERT INTO public.study_task VALUES (45, 7, NULL, 'typing', 2, '2026-01-07 07:01:21.114445+00', '2026-01-08 00:51:24.771561+00');
INSERT INTO public.study_task VALUES (46, 7, NULL, 'typing', 3, '2026-01-07 07:13:22.10023+00', '2026-01-08 00:51:24.778389+00');
INSERT INTO public.study_task VALUES (47, 7, NULL, 'typing', 4, '2026-01-07 07:13:22.11499+00', '2026-01-08 00:51:24.786556+00');
INSERT INTO public.study_task VALUES (44, 7, NULL, 'typing', 1, '2026-01-07 06:46:43.125112+00', '2026-01-08 01:05:30.370715+00');

--
-- Data for Name: video; Type: TABLE DATA
--

INSERT INTO public.video VALUES (17, NULL, 'video_fffdae0c-94b5-4ccc-9f22-148d8929f4a6', 'ready', 'private', 'https://vimeo.com/222222222', '2026-01-06 03:48:38.966333+00', '2026-01-06 03:48:38.966333+00', NULL, NULL);
INSERT INTO public.video VALUES (18, NULL, 'lecture_error_handling', 'ready', 'public', 'https://vimeo.com/333333333', '2026-01-06 03:48:38.971548+00', '2026-01-06 07:05:15.615213+00', NULL, NULL);
INSERT INTO public.video VALUES (19, NULL, 'log_test_idx_01', 'ready', 'public', 'https://vimeo.com/log12345', '2026-01-06 07:51:32.906253+00', '2026-01-06 07:52:27.237276+00', NULL, NULL);
INSERT INTO public.video VALUES (1, NULL, 'bulk_update_idx_01', 'open', 'private', 'https://vimeo.com/1150298052', '2025-12-31 03:57:13.450495+00', '2026-01-06 07:18:31.663995+00', NULL, NULL);
INSERT INTO public.video VALUES (2, NULL, 'test-2', 'open', 'public', 'https://vimeo.com/1150298070', '2025-12-31 03:57:13.450495+00', '2026-01-06 07:18:31.680545+00', NULL, NULL);
INSERT INTO public.video VALUES (3, NULL, 'bulk_update_idx_03', 'open', 'public', 'https://vimeo.com/1150298083', '2025-12-31 03:57:13.450495+00', '2026-01-06 07:18:31.69306+00', NULL, NULL);
INSERT INTO public.video VALUES (4, NULL, 'test-4', 'open', 'public', 'https://vimeo.com/1150298083', '2025-12-31 03:57:13.450495+00', '2025-12-31 03:57:13.450495+00', NULL, NULL);
INSERT INTO public.video VALUES (5, NULL, 'test-5', 'open', 'public', 'https://vimeo.com/1150298112', '2025-12-31 03:57:13.450495+00', '2025-12-31 03:57:13.450495+00', NULL, NULL);
INSERT INTO public.video VALUES (6, NULL, 'test-6', 'open', 'paid', 'https://vimeo.com/1150298982', '2025-12-31 03:57:13.450495+00', '2025-12-31 03:57:13.450495+00', NULL, NULL);
INSERT INTO public.video VALUES (7, NULL, 'test-7', 'open', 'paid', 'https://vimeo.com/1150298998', '2025-12-31 03:57:13.450495+00', '2025-12-31 03:57:13.450495+00', NULL, NULL);
INSERT INTO public.video VALUES (8, NULL, 'test-8', 'open', 'paid', 'https://vimeo.com/1150299011', '2025-12-31 03:57:13.450495+00', '2025-12-31 03:57:13.450495+00', NULL, NULL);
INSERT INTO public.video VALUES (9, NULL, 'test-9', 'open', 'paid', 'https://vimeo.com/1150299023', '2025-12-31 03:57:13.450495+00', '2025-12-31 03:57:13.450495+00', NULL, NULL);
INSERT INTO public.video VALUES (10, NULL, 'test-10', 'close', 'public', 'https://vimeo.com/1150529015', '2025-12-31 03:57:13.450495+00', '2025-12-31 03:57:13.450495+00', NULL, NULL);
INSERT INTO public.video VALUES (11, NULL, 'test-11', 'ready', 'public', 'https://vimeo.com/1150529027', '2025-12-31 03:57:13.450495+00', '2025-12-31 03:57:13.450495+00', NULL, NULL);
INSERT INTO public.video VALUES (15, NULL, 'update_test_01', 'ready', 'private', 'https://vimeo.com/1154532657', '2026-01-06 03:26:27.618603+00', '2026-01-06 06:32:35.32412+00', NULL, NULL);
INSERT INTO public.video VALUES (16, NULL, 'lecture_async_deep_01', 'ready', 'public', 'https://vimeo.com/1154532686', '2026-01-06 03:48:38.957096+00', '2026-01-06 03:48:38.957096+00', NULL, NULL);

--
-- Data for Name: lesson_item; Type: TABLE DATA
--

INSERT INTO public.lesson_item VALUES (1, 1, 'video', 1, NULL);
INSERT INTO public.lesson_item VALUES (1, 2, 'task', NULL, 1);
INSERT INTO public.lesson_item VALUES (1, 3, 'task', NULL, 2);
INSERT INTO public.lesson_item VALUES (1, 4, 'task', NULL, 3);
INSERT INTO public.lesson_item VALUES (1, 5, 'task', NULL, 4);
INSERT INTO public.lesson_item VALUES (1, 6, 'task', NULL, 5);
INSERT INTO public.lesson_item VALUES (1, 7, 'task', NULL, 6);
INSERT INTO public.lesson_item VALUES (1, 8, 'task', NULL, 7);
INSERT INTO public.lesson_item VALUES (1, 9, 'task', NULL, 8);
INSERT INTO public.lesson_item VALUES (1, 10, 'task', NULL, 9);
INSERT INTO public.lesson_item VALUES (2, 1, 'video', 3, NULL);
INSERT INTO public.lesson_item VALUES (2, 2, 'task', NULL, 15);
INSERT INTO public.lesson_item VALUES (2, 3, 'video', 4, NULL);
INSERT INTO public.lesson_item VALUES (2, 4, 'task', NULL, 16);
INSERT INTO public.lesson_item VALUES (2, 5, 'video', 5, NULL);
INSERT INTO public.lesson_item VALUES (2, 6, 'task', NULL, 17);
INSERT INTO public.lesson_item VALUES (2, 7, 'video', 6, NULL);
INSERT INTO public.lesson_item VALUES (2, 8, 'task', NULL, 18);
INSERT INTO public.lesson_item VALUES (2, 9, 'video', 7, NULL);
INSERT INTO public.lesson_item VALUES (2, 10, 'task', NULL, 19);
INSERT INTO public.lesson_item VALUES (3, 1, 'video', 8, NULL);
INSERT INTO public.lesson_item VALUES (3, 2, 'task', NULL, 25);
INSERT INTO public.lesson_item VALUES (3, 3, 'video', 9, NULL);
INSERT INTO public.lesson_item VALUES (3, 4, 'task', NULL, 26);
INSERT INTO public.lesson_item VALUES (3, 5, 'video', 10, NULL);
INSERT INTO public.lesson_item VALUES (3, 6, 'task', NULL, 27);
INSERT INTO public.lesson_item VALUES (3, 7, 'video', 2, NULL);
INSERT INTO public.lesson_item VALUES (3, 8, 'task', NULL, 28);
INSERT INTO public.lesson_item VALUES (3, 9, 'video', 11, NULL);
INSERT INTO public.lesson_item VALUES (3, 10, 'task', NULL, 29);
INSERT INTO public.lesson_item VALUES (4, 9, 'video', 19, NULL);
INSERT INTO public.lesson_item VALUES (4, 11, 'video', 19, NULL);
INSERT INTO public.lesson_item VALUES (4, 12, 'task', NULL, 29);
INSERT INTO public.lesson_item VALUES (4, 13, 'video', 18, NULL);
INSERT INTO public.lesson_item VALUES (4, 14, 'task', NULL, 28);
INSERT INTO public.lesson_item VALUES (4, 15, 'video', 17, NULL);
INSERT INTO public.lesson_item VALUES (4, 16, 'task', NULL, 27);
INSERT INTO public.lesson_item VALUES (4, 17, 'video', 16, NULL);
INSERT INTO public.lesson_item VALUES (4, 18, 'task', NULL, 26);
INSERT INTO public.lesson_item VALUES (4, 20, 'task', NULL, 25);

--
-- Data for Name: study_task_choice; Type: TABLE DATA
--

INSERT INTO public.study_task_choice VALUES (16, '위 그림 중 고양이는 무엇 입니까?', '1.', '2.', '3.', '4.', 2, NULL, 'https://media.tenor.com/pQ4-GdwCeg0AAAAm/dil-atan-kedi.webp');
INSERT INTO public.study_task_choice VALUES (17, '위 그림 중 고양이는 무엇 입니까?', '1.', '2.', '3.', '4.', 3, NULL, 'https://media.tenor.com/28cCOSYKZtYAAAAm/banana-cat-cat-banana.webp');
INSERT INTO public.study_task_choice VALUES (18, '위 그림 중 고양이는 무엇 입니까?', '1.', '2.', '3.', '4.', 4, NULL, 'https://media.tenor.com/cufstBCUfvgAAAAM/cat-hitting.gif');
INSERT INTO public.study_task_choice VALUES (19, '위 그림 중 고양이는 무엇 입니까?', '1.', '2.', '3.', '4.', 1, NULL, 'https://media.tenor.com/adLokhFRSiQAAAAm/kasper-dancing.webp');
INSERT INTO public.study_task_choice VALUES (20, '위 그림 중 고양이는 무엇 입니까?', '1.', '2.', '3.', '4.', 2, NULL, 'https://media.tenor.com/X-jA_vmTHUYAAAAM/yapapa-yapapa-cat.gif');
INSERT INTO public.study_task_choice VALUES (21, '위 그림 중 고양이는 무엇 입니까?', '1.', '2.', '3.', '4.', 3, NULL, 'https://media.tenor.com/FLBmfkn2QJ0AAAAM/im-good.gif');
INSERT INTO public.study_task_choice VALUES (22, '위 그림 중 고양이는 무엇 입니까?', '1.', '2.', '3.', '4.', 4, NULL, 'https://media.tenor.com/5-FNjhEQhHIAAAAM/stupid-cat-cat.gif');
INSERT INTO public.study_task_choice VALUES (23, '위 그림 중 고양이는 무엇 입니까?', '1.', '2.', '3.', '4.', 1, NULL, 'https://media.tenor.com/aQs6_kuMlTIAAAAM/head-empty-cat.gif');
INSERT INTO public.study_task_choice VALUES (24, '위 그림 중 고양이는 무엇 입니까?', '1.', '2.', '3.', '4.', 2, NULL, 'https://media.tenor.com/jexT0EwvhtAAAAAM/scoobert-mad-cat.gif');
INSERT INTO public.study_task_choice VALUES (35, '다음 중 한국 사람은 누구입니까?', '1.', '2.', '3.', '4.', 1, NULL, 'https://media.tenor.com/KnZh4o6AUn8AAAA1/triples-seoyeon.webp');
INSERT INTO public.study_task_choice VALUES (36, '다음 중 한국 사람은 누구입니까?', '1.', '2.', '3.', '4.', 2, NULL, 'https://media.tenor.com/0E0Dq87kTdcAAAAM/triples-triples-wink.gif');
INSERT INTO public.study_task_choice VALUES (37, '다음 중 한국 사람은 누구입니까?', '1.', '2.', '3.', '4.', 1, NULL, 'https://media.tenor.com/I2yK-d7ySYAAAAA1/ian-ang-hearts2hearts.webp');
INSERT INTO public.study_task_choice VALUES (38, '다음 중 한국 사람은 누구입니까?', '1.', '2.', '3.', '4.', 3, NULL, 'https://media.tenor.com/qdrQxHgEPkkAAAAM/h2h-jiwoo-look-around.gif');
INSERT INTO public.study_task_choice VALUES (39, '다음 중 한국 사람은 누구입니까?', '1.', '2.', '3.', '4.', 4, NULL, 'https://media.tenor.com/KnGZyZj6E_oAAAAM/hearts2hearts-h2h.gif');
INSERT INTO public.study_task_choice VALUES (15, '위 그림 중 고양이는 무엇 입니까?', '1.', '2.', '3.', '4.', 1, NULL, 'https://media1.tenor.com/m/xLLfA5HW0-0AAAAC/cat.gif');
INSERT INTO public.study_task_choice VALUES (41, '관리자 테스트용 스터디 태스크 질문_2', '1_test_2', '2_test_2', '3_test_2', '4_test_2', 2, 'AUDIO_TEST_URL_2', 'IMAGE_TEST_URL_2');
INSERT INTO public.study_task_choice VALUES (42, '관리자 테스트용 스터디 태스크 질문_3', '1_test_3', '2_test_3', '3_test_3', '4_test_3', 3, 'AUDIO_TEST_URL_3', 'IMAGE_TEST_URL_3');
INSERT INTO public.study_task_choice VALUES (43, '관리자 테스트용 스터디 태스크 질문_4', '1_test_4', '2_test_4', '3_test_4', '4_test_4', 4, 'AUDIO_TEST_URL_4', 'IMAGE_TEST_URL_4');
INSERT INTO public.study_task_choice VALUES (40, '관리자 테스트용 스터디 태스크 질문_1', '1_test_1', '2_test_1', '3_test_1', '4_test_1', 1, 'AUDIO_TEST_URL_1', 'IMAGE_TEST_URL_1');

--
-- Data for Name: study_task_typing; Type: TABLE DATA
--

INSERT INTO public.study_task_typing VALUES (1, '위 글자를 써보세요.', '가', 'https://media.tenor.com/3UvnQWmVmUYAAAAm/doge.webp');
INSERT INTO public.study_task_typing VALUES (2, '위 글자를 써보세요.', '나', 'https://media.tenor.com/Ez8G-p1fYLMAAAAm/emote-dog.webp');
INSERT INTO public.study_task_typing VALUES (3, '위 글자를 써보세요.', '다', 'https://media.tenor.com/OTeNWwAMDmAAAAAM/dog-sunset.gif');
INSERT INTO public.study_task_typing VALUES (4, '위 글자를 써보세요.', '라', 'https://media.tenor.com/Gz3VfRJSysMAAAAM/meme-dog-smile-dog.gif');
INSERT INTO public.study_task_typing VALUES (5, '위 글자를 써보세요.', '마', 'https://media.tenor.com/t_c6v95GzCgAAAAm/cute-pug.webp');
INSERT INTO public.study_task_typing VALUES (6, '위 글자를 써보세요.', '바', 'https://media.tenor.com/elPYP8YQVOAAAAAM/vest-dog.gif');
INSERT INTO public.study_task_typing VALUES (7, '위 글자를 써보세요.', '사', 'https://media.tenor.com/z4wyl-wGu8gAAAAm/dog-love-cool-doge.webp');
INSERT INTO public.study_task_typing VALUES (8, '위 글자를 써보세요.', '아', 'https://media.tenor.com/zmyMkoQ_YoUAAAAM/scared-dog.gif');
INSERT INTO public.study_task_typing VALUES (9, '위 글자를 써보세요.', '자', 'https://media.tenor.com/Xmr01nUr0I0AAAAm/cute-dog.webp');
INSERT INTO public.study_task_typing VALUES (10, '위 글자를 써보세요.', '차', 'https://media.tenor.com/FI7dxlR-1f4AAAA1/golden-retriever-rollercoaster.webp');
INSERT INTO public.study_task_typing VALUES (11, '위 글자를 써보세요.', '카', 'https://media.tenor.com/Ile97eFHbdIAAAA1/doggy-cute.webp');
INSERT INTO public.study_task_typing VALUES (12, '위 글자를 써보세요.', '파', 'https://media.tenor.com/ZoBlxLmmAHQAAAA1/dog-shivering.webp');
INSERT INTO public.study_task_typing VALUES (13, '위 글자를 써보세요.', '타', 'https://media.tenor.com/XlzVCeCUYLIAAAAM/dog-smile-shyboos.gif');
INSERT INTO public.study_task_typing VALUES (14, '위 글자를 써보세요.', '하', 'https://media.tenor.com/xltrSkbkhTcAAAAm/perro.webp');
INSERT INTO public.study_task_typing VALUES (30, '이 그림은 무엇 입니까?', '소닉', 'https://media.tenor.com/-9zguDBCg5MAAAAM/sonic-bluesphere.gif');
INSERT INTO public.study_task_typing VALUES (31, '이 그림은 무엇 입니까?', '위처', 'https://media.tenor.com/LorCIgug2dAAAAAM/gwent-geralt.gif');
INSERT INTO public.study_task_typing VALUES (32, '이 그림은 무엇 입니까?', '엘든링', 'https://media.tenor.com/t1KbzWJ9sGkAAAAM/elden-ring-action-rpg.gif');
INSERT INTO public.study_task_typing VALUES (33, '이 그림은 무엇 입니까?', '워해머', 'https://media.tenor.com/n-77Nv7XdQEAAAAM/spacemarine-spacemarine2.gif');
INSERT INTO public.study_task_typing VALUES (34, '이 그림은 무엇 입니까?', '워크래프트', 'https://media.tenor.com/mnL628TQ7wEAAAA1/arthas-arthas-menethil.webp');
INSERT INTO public.study_task_typing VALUES (45, '테스트_질문_디버깅_01', '테스트_답변_디버깅_01', 'TEST_URL_DEBUGING_01');
INSERT INTO public.study_task_typing VALUES (46, '테스트_질문_디버깅_02', '테스트_답변_디버깅_02', 'TEST_URL_DEBUGING_02');
INSERT INTO public.study_task_typing VALUES (47, '테스트_질문_디버깅_03', '테스트_답변_디버깅_03', 'TEST_URL_DEBUGING_03');
INSERT INTO public.study_task_typing VALUES (44, '테스트_질문_디버깅_000', '테스트_답변_디버깅_000', 'TEST_URL_DEBUGING_000');

--
-- Data for Name: study_task_voice; Type: TABLE DATA
--

INSERT INTO public.study_task_voice VALUES (25, '이 노래의 느낌은 무엇 입니까?', '신나다.', 'https://cdn.pixabay.com/audio/2025/08/29/audio_fba9035557.mp3', NULL);
INSERT INTO public.study_task_voice VALUES (26, '이 노래의 느낌은 무엇 입니까?', '슬프다.', 'https://cdn.pixabay.com/audio/2024/01/13/audio_640c5ca4a9.mp3', NULL);
INSERT INTO public.study_task_voice VALUES (27, '이 노래의 느낌은 무엇 입니까?', '재미있다.', 'https://cdn.pixabay.com/audio/2025/11/13/audio_05ddf9c6d1.mp3', NULL);
INSERT INTO public.study_task_voice VALUES (28, '이 노래의 느낌은 무엇 입니까?', '웅장하다.', 'https://cdn.pixabay.com/audio/2025/10/28/audio_4285598df8.mp3', NULL);
INSERT INTO public.study_task_voice VALUES (29, '이 노래의 느낌은 무엇 입니까?', '그립다.', 'https://cdn.pixabay.com/audio/2023/09/17/audio_fee1f2b797.mp3', NULL);

--
-- Data for Name: video_tag; Type: TABLE DATA
--

INSERT INTO public.video_tag VALUES (4, 'basic-sentence-900-04', '동사 시제 연습 2', '연습 동사 : 읽다, 앉다, 잃다, 많다...', '2025-12-31 03:54:39.285301+00');
INSERT INTO public.video_tag VALUES (5, 'basic-sentence-900-05', '동사 시제 연습 3', '연습 동사 : 들다, 열다, 울다, 만들다...', '2025-12-31 03:54:39.285301+00');
INSERT INTO public.video_tag VALUES (6, 'topik-1-06', '동사 시제 연습 4', '연습 동사 : 뛰다, 쉬다, 되다, 가다...', '2025-12-31 03:54:39.285301+00');
INSERT INTO public.video_tag VALUES (7, 'topik-1-07', '동사 시제 연습 5', '연습 동사 : 서다, 켜다, 오다, 보다...', '2025-12-31 03:54:39.285301+00');
INSERT INTO public.video_tag VALUES (8, 'topik-1-08', '동사 시제 연습 6', '연습 동사 : 식사하다, 요리하다, 공부하다, 숙제하다...', '2025-12-31 03:54:39.285301+00');
INSERT INTO public.video_tag VALUES (9, 'topik-1-09', '동사 시제 연습 7', '연습 동사 : 연주하다, 축하하다, 이사하다, 청소하다...', '2025-12-31 03:54:39.285301+00');
INSERT INTO public.video_tag VALUES (10, 'topik-2-01', '동사 시제 연습 8', '연습 동사 : 쓰다, 기쁘다, 이쁘다, 슬프다...', '2025-12-31 03:54:39.285301+00');
INSERT INTO public.video_tag VALUES (11, 'topik-2-02', '문장 연습 1', '연습 동사 : ~가 ~이다, ~하다', '2025-12-31 03:54:39.285301+00');
INSERT INTO public.video_tag VALUES (13, 'tag_async_deep_01', 'Rust 비동기 심화 1강', 'Tokio 런타임의 구조와 동작 원리', '2026-01-06 03:48:38.957096+00');
INSERT INTO public.video_tag VALUES (14, 'tag_ad0670a9-ad50-40dc-a', 'WebAssembly 기초 (자동생성 테스트)', NULL, '2026-01-06 03:48:38.966333+00');
INSERT INTO public.video_tag VALUES (12, 'update_key_01', 'Rust 심화 수정', '설명 업데이트', '2026-01-06 03:26:27.618603+00');
INSERT INTO public.video_tag VALUES (15, 'tag_patch_isolated_v1', 'Rust 태그 수정 API 테스트', '이 요청은 오직 태그 정보만 변경합니다.', '2026-01-06 03:48:38.971548+00');
INSERT INTO public.video_tag VALUES (1, 'tag_bulk_patch_v3', 'Rust 벌크 태그 수정 (Full Option)', '제목, 부제목, 키까지 모두 변경합니다.', '2025-12-31 03:54:39.285301+00');
INSERT INTO public.video_tag VALUES (2, 'basic-sentence-900-02', '공개 상태만 변경', '이 비디오는 오직 부제목만 수정되었습니다.', '2025-12-31 03:54:39.285301+00');
INSERT INTO public.video_tag VALUES (3, 'tag_bulk_partial_v3', '제목과 키만 변경', 'URL과 IDX만 부분 수정', '2025-12-31 03:54:39.285301+00');
INSERT INTO public.video_tag VALUES (16, 'log_test_key_01', '로그 리팩토링 후 수정됨', 'ADMIN_VIDEO_LOG 테이블 확인용', '2026-01-06 07:51:32.906253+00');

--
-- Data for Name: video_tag_map; Type: TABLE DATA
--

INSERT INTO public.video_tag_map VALUES (1, 1);
INSERT INTO public.video_tag_map VALUES (2, 2);
INSERT INTO public.video_tag_map VALUES (3, 3);
INSERT INTO public.video_tag_map VALUES (4, 4);
INSERT INTO public.video_tag_map VALUES (5, 5);
INSERT INTO public.video_tag_map VALUES (6, 6);
INSERT INTO public.video_tag_map VALUES (7, 7);
INSERT INTO public.video_tag_map VALUES (8, 8);
INSERT INTO public.video_tag_map VALUES (9, 9);
INSERT INTO public.video_tag_map VALUES (10, 10);
INSERT INTO public.video_tag_map VALUES (11, 11);
INSERT INTO public.video_tag_map VALUES (15, 12);
INSERT INTO public.video_tag_map VALUES (16, 13);
INSERT INTO public.video_tag_map VALUES (17, 14);
INSERT INTO public.video_tag_map VALUES (18, 15);
INSERT INTO public.video_tag_map VALUES (19, 16);

--
-- Sequence 리셋 (IDENTITY 시퀀스를 데이터의 최대값 이후로 설정)
--

SELECT pg_catalog.setval('public.lesson_lesson_id_seq', 8, true);
SELECT pg_catalog.setval('public.study_study_id_seq', 9, true);
SELECT pg_catalog.setval('public.study_task_study_task_id_seq', 47, true);
SELECT pg_catalog.setval('public.video_tag_video_tag_id_seq', 16, true);
SELECT pg_catalog.setval('public.video_video_id_seq', 19, true);
