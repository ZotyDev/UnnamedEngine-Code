use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct StatusTranslation {
    pub status_id: i32,
    pub i18n_id: i32,
    pub title: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}