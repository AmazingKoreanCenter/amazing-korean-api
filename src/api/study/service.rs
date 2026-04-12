use redis::AsyncCommands;
use tracing::warn;

use crate::api::admin::translation::repo::TranslationRepo;
use crate::api::auth::extractor::AuthUser;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use crate::types::{ContentType, StudyProgram, StudyTaskKind, StudyTaskLogAction};

// [Strict Mode] Import DTOs and Repo directly from the verified files
use super::dto::{
    FinishWritingSessionReq, StartWritingSessionReq, StudyDetailReq, StudyDetailRes, StudyListMeta,
    StudyListReq, StudyListResp, StudyListSort, StudyTaskDetailRes, SubmitAnswerReq,
    SubmitAnswerRes, TaskExplainRes, TaskStatusRes, WritingSessionListReq, WritingSessionListRes,
    WritingSessionRes, WritingStatsReq, WritingStatsRes,
};
use super::repo::StudyRepo;

pub struct StudyService;

impl StudyService {
    // =========================================================================
    // 1. List
    // =========================================================================

    /// 학습 목록 조회
    pub async fn list_studies(st: &AppState, req: StudyListReq) -> AppResult<StudyListResp> {
        let page = req.page.unwrap_or(1);
        let per_page = req.per_page.unwrap_or(10);

        if page == 0 {
            return Err(AppError::BadRequest("page must be >= 1".into()));
        }

        if per_page == 0 {
            return Err(AppError::BadRequest("per_page must be >= 1".into()));
        }

        if per_page > 100 {
            return Err(AppError::Unprocessable("per_page must be <= 100".into()));
        }

        let program = match req.program.as_deref() {
            None => None,
            Some(raw) => {
                let trimmed = raw.trim();
                if trimmed.is_empty() {
                    return Err(AppError::BadRequest("program must not be empty".into()));
                }

                let parsed = parse_study_program(trimmed)
                    .ok_or_else(|| AppError::Unprocessable(invalid_program_message()))?;
                Some(parsed)
            }
        };

        let sort = match req.sort.as_deref() {
            None => StudyListSort::Latest,
            Some(raw) => {
                let trimmed = raw.trim();
                if trimmed.is_empty() {
                    return Err(AppError::BadRequest("sort must not be empty".into()));
                }

                StudyListSort::parse(trimmed)
                    .ok_or_else(|| AppError::Unprocessable(invalid_sort_message()))?
            }
        };

        let (mut list, total_count) =
            StudyRepo::find_open_studies(&st.db, page, per_page, program, sort).await?;

        let per_page_i64 = i64::from(per_page);
        let total_pages_i64 = if total_count == 0 {
            0
        } else {
            (total_count + per_page_i64 - 1) / per_page_i64
        };

        if total_pages_i64 > u32::MAX as i64 {
            return Err(AppError::Internal("total_pages overflow".into()));
        }

        // 번역 주입
        if let Some(lang) = req.lang {
            let ids: Vec<i64> = list.iter().map(|s| i64::from(s.study_id)).collect();
            let translations = TranslationRepo::find_translations_for_contents(
                &st.db,
                ContentType::Study,
                &ids,
                lang,
            )
            .await?;

            for item in list.iter_mut() {
                let id = i64::from(item.study_id);
                if let Some(t) = translations.get(&(id, "title".to_string())) {
                    item.title = Some(t.text.clone());
                }
                if let Some(t) = translations.get(&(id, "subtitle".to_string())) {
                    item.subtitle = Some(t.text.clone());
                }
            }
        }

        Ok(StudyListResp {
            list,
            meta: StudyListMeta {
                page,
                per_page,
                total_count,
                total_pages: total_pages_i64 as u32,
            },
        })
    }

    // =========================================================================
    // 1-2. Study Detail (Study + Task List)
    // =========================================================================

    /// Study 상세 조회 (Study 정보 + Task 목록)
    pub async fn get_study_detail(
        st: &AppState,
        study_id: i32,
        req: StudyDetailReq,
    ) -> AppResult<StudyDetailRes> {
        let page = req.page.unwrap_or(1);
        let per_page = req.per_page.unwrap_or(10);

        if page == 0 {
            return Err(AppError::BadRequest("page must be >= 1".into()));
        }

        if per_page == 0 {
            return Err(AppError::BadRequest("per_page must be >= 1".into()));
        }

        if per_page > 100 {
            return Err(AppError::Unprocessable("per_page must be <= 100".into()));
        }

        // Get Study
        let study = StudyRepo::get_study_by_id(&st.db, study_id)
            .await?
            .ok_or(AppError::NotFound)?;

        // Get Tasks
        let (tasks, total_count) =
            StudyRepo::get_tasks_by_study_id(&st.db, study_id, page, per_page).await?;

        let per_page_i64 = i64::from(per_page);
        let total_pages_i64 = if total_count == 0 {
            0
        } else {
            (total_count + per_page_i64 - 1) / per_page_i64
        };

        if total_pages_i64 > u32::MAX as i64 {
            return Err(AppError::Internal("total_pages overflow".into()));
        }

        // 번역 주입
        let mut title = study.title;
        let mut subtitle = study.subtitle;

        if let Some(lang) = req.lang {
            let translations = TranslationRepo::find_translations_for_contents(
                &st.db,
                ContentType::Study,
                &[i64::from(study.study_id)],
                lang,
            )
            .await?;

            let id = i64::from(study.study_id);
            if let Some(t) = translations.get(&(id, "title".to_string())) {
                title = Some(t.text.clone());
            }
            if let Some(t) = translations.get(&(id, "subtitle".to_string())) {
                subtitle = Some(t.text.clone());
            }
        }

        Ok(StudyDetailRes {
            study_id: study.study_id,
            study_idx: study.study_idx,
            program: study.program,
            title,
            subtitle,
            state: study.state,
            tasks,
            meta: StudyListMeta {
                page,
                per_page,
                total_count,
                total_pages: total_pages_i64 as u32,
            },
        })
    }

    // =========================================================================
    // 2. Detail
    // =========================================================================

    /// 학습 문제 상세 조회
    pub async fn get_study_task(
        st: &AppState,
        task_id: i32,
        auth: Option<AuthUser>,
    ) -> AppResult<StudyTaskDetailRes> {
        let task = StudyRepo::find_task_detail(&st.db, i64::from(task_id)).await?;
        let task = task.ok_or(AppError::NotFound)?;

        if let Some(AuthUser(claims)) = auth {
            if let Err(err) = StudyRepo::log_task_action(
                &st.db,
                claims.sub,
                &claims.session_id,
                task_id,
                StudyTaskLogAction::View,
            )
            .await
            {
                warn!(
                    error = ?err,
                    user_id = claims.sub,
                    task_id,
                    "Failed to log study task view"
                );
            }
        }

        Ok(task)
    }

    // =========================================================================
    // 3. Action (Grading & Log)
    // =========================================================================

    /// 정답 제출 및 채점
    pub async fn submit_answer(
        st: &AppState,
        auth_user: AuthUser,
        task_id: i32,
        req: SubmitAnswerReq,
    ) -> AppResult<SubmitAnswerRes> {
        let AuthUser(claims) = auth_user;

        // [Rate Limiting] 과도한 답안 제출 방지
        let rl_key = format!("rl:study_submit:{}", claims.sub);
        let mut redis_conn = st
            .redis
            .get()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let attempts: i64 = redis_conn
            .incr(&rl_key, 1)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let _: () = redis_conn
            .expire(&rl_key, st.cfg.rate_limit_study_window_sec)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        if attempts > st.cfg.rate_limit_study_max {
            return Err(AppError::TooManyRequests(
                "STUDY_429_TOO_MANY_SUBMISSIONS".into(),
            ));
        }

        let answer_key = StudyRepo::find_answer_key(&st.db, task_id).await?;
        let answer_key = answer_key.ok_or(AppError::NotFound)?;

        let req_kind = match &req {
            SubmitAnswerReq::Choice { .. } => StudyTaskKind::Choice,
            SubmitAnswerReq::Typing { .. } => StudyTaskKind::Typing,
            SubmitAnswerReq::Voice { .. } => StudyTaskKind::Voice,
            SubmitAnswerReq::Writing { .. } => StudyTaskKind::Writing,
        };

        if req_kind != answer_key.kind {
            return Err(AppError::BadRequest("Task kind mismatch".into()));
        }

        let submitted = match &req {
            SubmitAnswerReq::Choice { pick } => {
                if *pick < 1 || *pick > 4 {
                    return Err(AppError::Unprocessable(
                        "pick must be between 1 and 4".into(),
                    ));
                }
                pick.to_string()
            }
            SubmitAnswerReq::Typing { text }
            | SubmitAnswerReq::Voice { text }
            | SubmitAnswerReq::Writing { text, .. } => {
                let trimmed = text.trim();
                if trimmed.is_empty() {
                    return Err(AppError::BadRequest("text must not be empty".into()));
                }
                trimmed.to_string()
            }
        };

        let answer_trimmed = answer_key.answer.trim();
        let is_correct = submitted == answer_trimmed;

        let payload = serde_json::to_value(&req)
            .map_err(|e| AppError::Internal(format!("Failed to serialize payload: {e}")))?;

        StudyRepo::submit_grade_tx(
            &st.db,
            claims.sub,
            &claims.session_id,
            task_id,
            is_correct,
            &payload,
        )
        .await?;

        let correct_answer = if is_correct {
            None
        } else {
            Some(answer_key.answer)
        };

        Ok(SubmitAnswerRes {
            is_correct,
            correct_answer,
            explanation: None,
        })
    }

    /// 내 문제 풀이 상태 조회
    pub async fn get_task_status(
        st: &AppState,
        auth_user: AuthUser,
        task_id: i32,
    ) -> AppResult<TaskStatusRes> {
        let AuthUser(claims) = auth_user;

        if !StudyRepo::exists_task(&st.db, task_id).await? {
            return Err(AppError::NotFound);
        }

        let status = StudyRepo::find_task_status(&st.db, claims.sub, task_id).await?;
        let status = status.unwrap_or(TaskStatusRes {
            try_count: 0,
            is_solved: false,
            last_attempt_at: None,
        });

        if let Err(err) = StudyRepo::log_task_action(
            &st.db,
            claims.sub,
            &claims.session_id,
            task_id,
            StudyTaskLogAction::Status,
        )
        .await
        {
            warn!(
                error = ?err,
                user_id = claims.sub,
                task_id,
                "Failed to log study task status"
            );
        }

        Ok(status)
    }

    // =========================================================================
    // 4. Explanation
    // =========================================================================

    /// 해설 조회
    pub async fn get_task_explain(
        st: &AppState,
        auth_user: AuthUser,
        task_id: i32,
    ) -> AppResult<TaskExplainRes> {
        let AuthUser(claims) = auth_user;

        let try_count = StudyRepo::get_try_count(&st.db, claims.sub, task_id).await?;
        if try_count < 1 {
            return Err(AppError::Forbidden("Forbidden".to_string()));
        }

        let row = StudyRepo::find_task_explain(&st.db, task_id).await?;
        let row = row.ok_or(AppError::NotFound)?;

        let resources = match row.explain_media_url {
            Some(url) => vec![url],
            None => Vec::new(),
        };

        let response = TaskExplainRes {
            title: row.explain_title,
            explanation: row.explain_text,
            resources,
        };

        if let Err(err) = StudyRepo::log_task_action(
            &st.db,
            claims.sub,
            &claims.session_id,
            task_id,
            StudyTaskLogAction::Explain,
        )
        .await
        {
            warn!(
                error = ?err,
                user_id = claims.sub,
                task_id,
                "Failed to log study task explain"
            );
        }

        Ok(response)
    }

    // =========================================================================
    // 5. Writing Practice Sessions
    // =========================================================================

    /// 한글 자판 연습 세션 시작
    pub async fn start_writing_session(
        st: &AppState,
        auth_user: AuthUser,
        req: StartWritingSessionReq,
    ) -> AppResult<WritingSessionRes> {
        let AuthUser(claims) = auth_user;

        if let Some(task_id) = req.study_task_id {
            if !StudyRepo::exists_writing_task(&st.db, task_id).await? {
                return Err(AppError::NotFound);
            }
        }

        StudyRepo::create_writing_session(
            &st.db,
            claims.sub,
            req.study_task_id,
            req.writing_level,
            req.writing_practice_type,
        )
        .await
    }

    /// 세션 완료 (결과 저장)
    pub async fn finish_writing_session(
        st: &AppState,
        auth_user: AuthUser,
        session_id: i64,
        req: FinishWritingSessionReq,
    ) -> AppResult<WritingSessionRes> {
        let AuthUser(claims) = auth_user;

        if req.total_chars < 0 || req.correct_chars < 0 {
            return Err(AppError::BadRequest(
                "total_chars and correct_chars must be >= 0".into(),
            ));
        }
        if req.correct_chars > req.total_chars {
            return Err(AppError::Unprocessable(
                "correct_chars must be <= total_chars".into(),
            ));
        }
        if req.duration_ms < 0 {
            return Err(AppError::BadRequest("duration_ms must be >= 0".into()));
        }

        let accuracy_rate = if req.total_chars == 0 {
            0.0
        } else {
            (f64::from(req.correct_chars) / f64::from(req.total_chars)) * 100.0
        };

        let chars_per_minute = if req.duration_ms <= 0 {
            0.0
        } else {
            f64::from(req.total_chars) * 60_000.0 / req.duration_ms as f64
        };

        // 소수점 2자리 반올림 (NUMERIC(5,2)/(7,2) 오버플로우 방지)
        let accuracy_rate = (accuracy_rate * 100.0).round() / 100.0;
        let chars_per_minute = (chars_per_minute * 100.0).round() / 100.0;

        // NUMERIC(5,2) 범위: -999.99 ~ 999.99 / NUMERIC(7,2): -99999.99 ~ 99999.99
        let accuracy_rate = accuracy_rate.clamp(0.0, 100.0);
        let chars_per_minute = chars_per_minute.clamp(0.0, 99_999.99);

        let mistakes_json = serde_json::to_value(&req.mistakes)
            .map_err(|e| AppError::Internal(format!("Failed to serialize mistakes: {e}")))?;

        let res = StudyRepo::finish_writing_session(
            &st.db,
            session_id,
            claims.sub,
            req.total_chars,
            req.correct_chars,
            accuracy_rate,
            chars_per_minute,
            mistakes_json,
        )
        .await?;

        res.ok_or(AppError::NotFound)
    }

    /// 세션 목록 조회
    pub async fn list_writing_sessions(
        st: &AppState,
        auth_user: AuthUser,
        req: WritingSessionListReq,
    ) -> AppResult<WritingSessionListRes> {
        let AuthUser(claims) = auth_user;

        let page = req.page.unwrap_or(1);
        let per_page = req.per_page.unwrap_or(20);

        if page == 0 {
            return Err(AppError::BadRequest("page must be >= 1".into()));
        }
        if per_page == 0 {
            return Err(AppError::BadRequest("per_page must be >= 1".into()));
        }
        if per_page > 100 {
            return Err(AppError::Unprocessable("per_page must be <= 100".into()));
        }

        let (list, total_count) =
            StudyRepo::list_writing_sessions(&st.db, claims.sub, &req).await?;

        let per_page_i64 = i64::from(per_page);
        let total_pages_i64 = if total_count == 0 {
            0
        } else {
            (total_count + per_page_i64 - 1) / per_page_i64
        };

        if total_pages_i64 > u32::MAX as i64 {
            return Err(AppError::Internal("total_pages overflow".into()));
        }

        Ok(WritingSessionListRes {
            list,
            meta: StudyListMeta {
                page,
                per_page,
                total_count,
                total_pages: total_pages_i64 as u32,
            },
        })
    }

    /// 한글 자판 연습 통계
    pub async fn get_writing_stats(
        st: &AppState,
        auth_user: AuthUser,
        req: WritingStatsReq,
    ) -> AppResult<WritingStatsRes> {
        let AuthUser(claims) = auth_user;

        let days = req.days.unwrap_or(30);
        if days == 0 {
            return Err(AppError::BadRequest("days must be >= 1".into()));
        }
        if days > 365 {
            return Err(AppError::Unprocessable("days must be <= 365".into()));
        }
        let days = days as i32;

        let (total_sessions, avg_accuracy, avg_cpm) =
            StudyRepo::writing_stats_overall(&st.db, claims.sub, days).await?;
        let level_breakdown =
            StudyRepo::writing_stats_by_level(&st.db, claims.sub, days).await?;
        let recent_trend =
            StudyRepo::writing_stats_daily(&st.db, claims.sub, days).await?;
        let weak_chars =
            StudyRepo::writing_stats_weak_chars(&st.db, claims.sub, days, 10).await?;

        Ok(WritingStatsRes {
            total_sessions,
            avg_accuracy,
            avg_cpm,
            level_breakdown,
            recent_trend,
            weak_chars,
        })
    }
}

fn parse_study_program(value: &str) -> Option<StudyProgram> {
    match value {
        "basic_pronunciation" => Some(StudyProgram::BasicPronunciation),
        "basic_word" => Some(StudyProgram::BasicWord),
        "basic_900" => Some(StudyProgram::Basic900),
        "topik_read" => Some(StudyProgram::TopikRead),
        "topik_listen" => Some(StudyProgram::TopikListen),
        "topik_write" => Some(StudyProgram::TopikWrite),
        "tbc" => Some(StudyProgram::Tbc),
        _ => None,
    }
}

fn invalid_program_message() -> String {
    "program must be one of: basic_pronunciation, basic_word, basic_900, topik_read, topik_listen, topik_write, tbc".into()
}

fn invalid_sort_message() -> String {
    "sort must be one of: latest, oldest, alphabetical".into()
}
