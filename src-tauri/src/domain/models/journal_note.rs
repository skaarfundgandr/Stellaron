#[derive(Debug, Clone, serde::Serialize)]
pub struct JournalNote {
    pub id: i32,
    pub book_id: i32,
    pub title: Option<String>,
    pub body: String,
    pub chapter_title: Option<String>,
    pub page_number: Option<i32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NewJournalNote {
    pub book_id: i32,
    pub title: Option<String>,
    pub body: String,
    pub chapter_title: Option<String>,
    pub page_number: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct UpdateJournalNote {
    pub title: Option<String>,
    pub body: Option<String>,
}
