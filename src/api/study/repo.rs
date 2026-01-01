use sqlx::{PgPool, Postgres, QueryBuilder};

use super::dto::StudyListItem;
use crate::types::StudyProgram;

pub struct StudyRepo {
    pool: PgPool,
}

impl StudyRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn count_open_studies(
        &self,
        program: Option<StudyProgram>,
    ) -> Result<i64, sqlx::Error> {
        let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT COUNT(*) FROM study WHERE study_state = 'open'::study_state_enum",
        );

        if let Some(program) = program {
            qb.push(" AND study_program = ").push_bind(program);
        }

        let count = qb.build_query_scalar::<i64>().fetch_one(&self.pool).await?;
        Ok(count)
    }

    pub async fn find_open_studies(
        &self,
        per_page: u64,
        offset: u64,
        program: Option<StudyProgram>,
        order_by: &str,
    ) -> Result<Vec<StudyListItem>, sqlx::Error> {
        let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT study_id::bigint as study_id, study_idx, study_program, \
             study_title as title, study_subtitle as subtitle, \
             study_created_at as created_at \
             FROM study WHERE study_state = 'open'::study_state_enum",
        );

        if let Some(program) = program {
            qb.push(" AND study_program = ").push_bind(program);
        }

        qb.push(" ORDER BY ");
        qb.push(order_by);
        qb.push(" LIMIT ").push_bind(per_page as i64);
        qb.push(" OFFSET ").push_bind(offset as i64);

        let rows = qb
            .build_query_as::<StudyListItem>()
            .fetch_all(&self.pool)
            .await?;
        Ok(rows)
    }
}
