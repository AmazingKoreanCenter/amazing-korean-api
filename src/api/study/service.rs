use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::state::AppState;
use crate::types::StudyTaskKind;

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
        let (data, total_count) = StudyRepo::find_list_dynamic(&st.db, &req).await?;

        // 3. Calc Meta
        let total_pages = if total_count == 0 {
            0
        } else {
            (total_count + req.per_page as i64 - 1) / req.per_page as i64
        };

        Ok(StudyListRes {
            meta: StudyListMeta {
                total_count,
                total_pages,
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
        user_id: i64,
        task_id: i64,
        req: SubmitAnswerReq,
    ) -> AppResult<SubmitAnswerRes> {
        // 1. DB에서 정답 정보 조회
        // (repo.rs에서 find_answer_info가 (Kind, AnswerString)을 반환하도록 구현됨)
        let (kind, correct_answer_str) = StudyRepo::find_answer_info(&st.db, task_id)
            .await?
            .ok_or(AppError::NotFound)?;

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
            // [음성] 일단은 제출하면 정답 처리 (추후 AI 분석 연동 가능)
            (StudyTaskKind::Voice, SubmitAnswerReq::Voice { audio_url: _ }) => true,

            // [오류] 문제 유형과 제출 형식이 다름
            _ => return Err(AppError::BadRequest("Task kind mismatch".into())),
        };

        let score = if is_correct { 100 } else { 0 };

        // 3. 로그 기록 (Upsert)
        StudyRepo::upsert_log(&st.db, user_id, task_id, is_correct, score).await?;

        // 4. 결과 반환
        // 틀렸을 경우, 즉시 정답을 보여줄지 여부는 기획에 따름 (여기선 Option으로 처리)
        // 정책: 틀렸더라도 즉시 정답을 알려주지 않으려면 None 처리
        let correct_answer_res = if is_correct {
            Some(correct_answer_str)
        } else {
            None // 오답 시 정답 미공개 (해설 API를 통해 확인 유도)
        };

        Ok(SubmitAnswerRes {
            task_id,
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
        // Task 존재 확인
        if (StudyRepo::find_task_detail(&st.db, task_id).await?).is_none() {
            return Err(AppError::NotFound);
        }

        let status = StudyRepo::find_task_status(&st.db, user_id, task_id).await?;
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