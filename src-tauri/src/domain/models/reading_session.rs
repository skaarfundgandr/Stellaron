#[derive(Debug, Clone, serde::Serialize)]
pub struct ReadingSession {
    pub id: i32,
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

#[derive(Debug, Clone)]
pub struct NewReadingSession {
    pub book_id: i32,
    pub start_position: String,
    pub chapter_title: Option<String>,
    pub page_number: Option<i32>,
    pub start_ts: String,
}

#[derive(Debug, Clone)]
pub struct EndReadingSession {
    pub end_position: Option<String>,
    pub chapter_title: Option<String>,
    pub page_number: Option<i32>,
    pub end_ts: String,
    pub duration_seconds: Option<i32>,
    pub pages_read: Option<i32>,
}
