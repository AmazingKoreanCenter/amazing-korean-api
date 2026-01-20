use crate::error::{AppError, AppResult};
use crate::state::AppState;
use crate::types::{StudyProgram, StudyTaskKind};

// [Strict Mode] Import DTOs and Repo directly from the verified files
use super::dto::{
    StudyListMeta, StudyListReq, StudyListResp, StudyListSort, StudyTaskDetailRes, SubmitAnswerReq,
    SubmitAnswerRes, TaskExplainRes, TaskStatusRes,
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

        let (list, total_count) =
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
    // 2. Detail
    // =========================================================================

    /// 학습 문제 상세 조회
    pub async fn get_study_task(st: &AppState, task_id: i64) -> AppResult<StudyTaskDetailRes> {
        let task = StudyRepo::find_task_detail(&st.db, task_id).await?;
        task.ok_or(AppError::NotFound)
    }

    // =========================================================================
    // 3. Action (Grading & Log)
    // =========================================================================

    /// 정답 제출 및 채점
    pub async fn submit_answer(
        st: &AppState,
        _user_id: i64, // Used in FIXME
        task_id: i64,
        req: SubmitAnswerReq,
    ) -> AppResult<SubmitAnswerRes> {
        // 1. DB에서 문제 정보(Kind)와 정답(Answer) 조회
        // [Strict Mode] repo.rs에 find_answer_info가 없으므로, find_task_detail(Kind)과 find_explanation(Answer)을 조합하여 해결함.
        
        // A. Kind 조회
        let task_detail = StudyRepo::find_task_detail(&st.db, task_id)
            .await?
            .ok_or(AppError::NotFound)?;
        
        // B. Answer 조회 (해설 테이블에서 정답 추출)
        let explanation = StudyRepo::find_explanation(&st.db, task_id)
            .await?
            .ok_or(AppError::NotFound)?; // 문제 상세가 있으면 해설(정답 포함)도 있어야 함 (데이터 무결성 가정)

        let kind = task_detail.kind;
        // [Strict Mode] Nullable String handling
        let correct_answer_str = explanation.correct_answer.unwrap_or_default();

        // 2. 채점 로직 (Grading)
        // Request 타입과 DB의 Task Kind가 일치하는지 확인 후 정답 비교
        let is_correct = match (kind, req) {
            // [객관식] 번호 비교 (DB: "1" vs Req: 1)
            (StudyTaskKind::Choice, SubmitAnswerReq::Choice { pick }) => {
                pick.to_string() == correct_answer_str
            }
            // [주관식] 텍스트 비교 (공백 제거 후 비교)
            (StudyTaskKind::Typing, SubmitAnswerReq::Typing { text }) => {
                text.trim() == correct_answer_str.trim()
            }
            // [음성] 일단은 제출하면 정답 처리 (추후 AI 음성 분석 로직 구현 시 사용 예정)
            (StudyTaskKind::Voice, SubmitAnswerReq::Voice { .. }) => true,

            // [오류] 문제 유형과 제출 형식이 다름 (DB는 Choice인데 유저가 Typing을 보냄 등)
            _ => return Err(AppError::BadRequest("Task kind mismatch".into())),
        };

        let score = if is_correct { 100 } else { 0 };

        // 3. 로그 기록 (Upsert)
        // [Strict Mode] repo.rs에 upsert_log 함수가 존재하지 않음. 주석 처리.
        // FIXME: Implement StudyRepo::upsert_log(&st.db, user_id, task_id, is_correct, score).await?;

        // 4. 결과 반환
        // 틀렸을 경우, 즉시 정답을 보여줄지 여부는 기획 정책에 따름
        let correct_answer_res = if is_correct {
            Some(correct_answer_str)
        } else {
            None // 오답 시 정답 미공개 (해설 API를 통해 확인 유도)
        };

        Ok(SubmitAnswerRes {
            task_id: task_id as i32, // [Strict Mode] Cast i64 -> i32
            is_correct,
            score,
            correct_answer: correct_answer_res,
        })
    }

    /// 내 문제 풀이 상태 조회
    pub async fn get_task_status(
        st: &AppState,
        user_id: i64,
        task_id: i64,
    ) -> AppResult<TaskStatusRes> {
        // Task 존재 확인 (유효하지 않은 task_id 요청 방지)
        if (StudyRepo::find_task_detail(&st.db, task_id).await?).is_none() {
            return Err(AppError::NotFound);
        }

        // repo.rs의 find_status는 DB에서 i32 캐스팅을 수행하여 TaskStatusRes를 반환함
        let status = StudyRepo::find_status(&st.db, task_id, user_id).await?;
        Ok(status)
    }

    // =========================================================================
    // 4. Explanation
    // =========================================================================

    /// 해설 조회
    pub async fn get_task_explanation(
        st: &AppState,
        task_id: i64,
    ) -> AppResult<TaskExplainRes> {
        let explanation = StudyRepo::find_explanation(&st.db, task_id).await?;
        explanation.ok_or(AppError::NotFound)
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
