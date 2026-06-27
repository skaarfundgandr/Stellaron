use crate::infrastructure::database::models::schema::reading_sessions;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Selectable, PartialEq, Debug)]
#[diesel(table_name = reading_sessions)]
#[diesel(primary_key(session_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ReadingSessionRow {
    pub session_id: Option<i32>,
    pub book_id: i32,
    pub start_position: String,
    pub end_position: Option<String>,
    pub chapter_title: Option<String>,
    pub page_number: Option<i32>,
    pub start_ts: String,
    pub end_ts: Option<String>,
    pub duration_seconds: Option<i32>,
    pub pages_read: Option<i32>,
    pub created_at: Option<String>,
}

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = reading_sessions)]
pub struct NewReadingSessionRow<'a> {
    pub book_id: i32,
    pub start_position: &'a str,
    pub chapter_title: Option<&'a str>,
    pub page_number: Option<i32>,
    pub start_ts: &'a str,
}

impl From<ReadingSessionRow> for crate::domain::models::reading_session::ReadingSession {
    fn from(row: ReadingSessionRow) -> Self {
        crate::domain::models::reading_session::ReadingSession {
            id: row.session_id.unwrap_or(0),
            book_id: row.book_id,
            start_position: row.start_position,
            end_position: row.end_position,
            chapter_title: row.chapter_title,
            page_number: row.page_number,
            start_ts: row.start_ts,
            end_ts: row.end_ts,
            duration_seconds: row.duration_seconds,
            pages_read: row.pages_read,
            created_at: row.created_at,
        }
    }
}
