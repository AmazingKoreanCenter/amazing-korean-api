use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::state::AppState;
use crate::types::StudyTaskKind;

// [Strict Mode] Import DTOs and Repo directly from the verified files
use super::dto::{
    StudyListMeta, StudyListReq, StudyListRes, StudyTaskDetailRes, SubmitAnswerReq,
    SubmitAnswerRes, TaskExplainRes, TaskStatusRes,
};
use super::repo::StudyRepo;

pub struct StudyService;

impl StudyService {
    // =========================================================================
    // 1. List
    // =========================================================================

    /// 학습 목록 조회
    pub async fn list_studies(st: &AppState, req: StudyListReq) -> AppResult<StudyListRes> {
        // 1. Validation
        if let Err(e) = req.validate() {
            return Err(AppError::Unprocessable(e.to_string()));
        }

        // 2. Repo Call (Data + Total Count)
        // Repo returns (Vec<StudyListItem>, i64)
        let (data, total_count) = StudyRepo::find_list_dynamic(&st.db, &req).await?;

        // 3. Calc Meta
        // [Strict Mode] Use i64 for calculation to prevent overflow, then cast to i32 for DTO
        let per_page = req.per_page as i64;
        let total_pages = if total_count == 0 {
            0
        } else {
            (total_count + per_page - 1) / per_page
        };

        Ok(StudyListRes {
            meta: StudyListMeta {
                total_count: total_count as i32, // [Strict Mode] Explicit Cast i64 -> i32
                total_pages: total_pages as i32, // [Strict Mode] Explicit Cast i64 -> i32
                current_page: req.page,
                per_page: req.per_page,
            },
            data,
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