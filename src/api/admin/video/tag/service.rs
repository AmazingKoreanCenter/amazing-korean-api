#![allow(unused_imports)]

use super::dto::{TagItem, TagsModifyReq, VideoTagsRes};
use crate::error::{AppError, AppResult};
use crate::AppState;

fn normalize(name: &str) -> Result<String, AppError> {
    let s = name
        .split_whitespace()
        .filter(|p| !p.is_empty())
        .collect::<Vec<&str>>()
        .join(" ");
    if s.is_empty() || s.len() > 32 {
        return Err(AppError::BadRequest("tag length 1..32".into()));
    }
    // 간단 허용문자 검증(영문/숫자/공백/하이픈/언더스코어)
    if !s
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == ' ' || c == '-' || c == '_')
    {
        return Err(AppError::BadRequest("tag has invalid characters".into()));
    }
    Ok(s)
}

fn slugify(name: &str) -> String {
    name.trim()
        .to_ascii_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>()
}

pub async fn add_tags(
    st: &AppState,
    video_id: i64,
    req: TagsModifyReq,
    actor_user_id: i64,
) -> AppResult<VideoTagsRes> {
    if req.tags.is_empty() || req.tags.len() > 50 {
        return Err(AppError::BadRequest("tags length 1..50".into()));
    }
    let names: Vec<String> = req
        .tags
        .iter()
        .map(|t| normalize(t))
        .collect::<Result<Vec<String>, AppError>>()?;
    let tag_pairs = super::repo::ensure_tags(&st.db, &names).await?; // (id, name)
    let ids: Vec<i64> = tag_pairs.iter().map(|(id, _)| *id).collect();
    super::repo::add_tags_to_video(&st.db, video_id, &ids, actor_user_id).await?;
    let current = super::repo::list_tags_of_video(&st.db, video_id).await?;
    let items = current
        .into_iter()
        .map(|(id, name)| super::dto::TagItem {
            tag_id: id,
            slug: slugify(&name),
            name: name.to_string(),
        })
        .collect();
    Ok(VideoTagsRes {
        video_id,
        tags: items,
    })
}

pub async fn remove_tags(
    st: &AppState,
    video_id: i64,
    req: TagsModifyReq,
    actor_user_id: i64,
) -> AppResult<()> {
    if req.tags.is_empty() {
        return Ok(()); // no-op
    }
    let names: Vec<String> = req.tags.iter().filter_map(|t| normalize(t).ok()).collect();
    if names.is_empty() {
        return Ok(());
    }
    let tag_pairs = super::repo::ensure_tags(&st.db, &names).await?; // 없는 태그는 무시
    let ids: Vec<i64> = tag_pairs.into_iter().map(|(id, _)| id).collect();
    let _ = super::repo::remove_tags_from_video(&st.db, video_id, &ids, actor_user_id).await?;
    Ok(())
}
