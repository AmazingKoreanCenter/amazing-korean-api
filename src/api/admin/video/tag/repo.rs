use crate::error::AppResult;
use sqlx::{PgPool, Row};

pub async fn ensure_tags(db: &PgPool, names: &[String]) -> AppResult<Vec<(i64, String)>> {
    let mut tx = db.begin().await?;
    for n in names {
        sqlx::query(
            r#"INSERT INTO video_tag (tag_name) VALUES ($1) ON CONFLICT (tag_name) DO NOTHING"#,
        )
        .bind(n)
        .execute(&mut *tx)
        .await?;
    }
    let rows =
        sqlx::query(r#"SELECT video_tag_id, tag_name FROM video_tag WHERE tag_name = ANY($1)"#)
            .bind(names)
            .fetch_all(&mut *tx)
            .await?;
    tx.commit().await?;
    Ok(rows
        .into_iter()
        .map(|r| (r.get("video_tag_id"), r.get("tag_name")))
        .collect())
}

pub async fn add_tags_to_video(
    db: &PgPool,
    video_id: i64,
    tag_ids: &[i64],
    _actor_user_id: i64,
) -> AppResult<()> {
    let mut tx = db.begin().await?;
    for tid in tag_ids {
        sqlx::query(r#"INSERT INTO video_tag_map (video_id, video_tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"#)
            .bind(video_id)
            .bind(tid)
            .execute(&mut *tx)
            .await?;
    }
    // updated_by_user_id 등 변경 이력 누적이 필요하다면 여기서 처리(TODO).
    tx.commit().await?;
    Ok(())
}

pub async fn remove_tags_from_video(
    db: &PgPool,
    video_id: i64,
    tag_ids: &[i64],
    _actor_user_id: i64,
) -> AppResult<u64> {
    let res =
        sqlx::query(r#"DELETE FROM video_tag_map WHERE video_id=$1 AND video_tag_id = ANY($2)"#)
            .bind(video_id)
            .bind(tag_ids)
            .execute(db)
            .await?;
    Ok(res.rows_affected())
}

pub async fn list_tags_of_video(db: &PgPool, video_id: i64) -> AppResult<Vec<(i64, String)>> {
    let rows = sqlx::query(
        r#"
        SELECT t.video_tag_id, t.tag_name
        FROM video_tag_map m
        JOIN video_tag t ON t.video_tag_id = m.video_tag_id
        WHERE m.video_id = $1
        ORDER BY t.tag_name ASC
        "#,
    )
    .bind(video_id)
    .fetch_all(db)
    .await?;
    Ok(rows
        .into_iter()
        .map(|r| (r.get("video_tag_id"), r.get("tag_name")))
        .collect())
}
