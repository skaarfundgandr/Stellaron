#[derive(Debug, Clone, serde::Serialize)]
pub struct Rating {
    pub id: i32,
    pub book_id: i32,
    pub rating: i32,
    pub review: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NewRating {
    pub book_id: i32,
    pub rating: i32,
    pub review: Option<String>,
}
