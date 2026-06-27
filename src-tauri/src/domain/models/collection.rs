#[derive(Debug, Clone, serde::Serialize)]
pub struct Collection {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub accent_color: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CollectionBook {
    pub collection_id: i32,
    pub book_id: i32,
    pub added_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NewCollection {
    pub name: String,
    pub description: Option<String>,
    pub accent_color: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateCollection {
    pub name: Option<String>,
    pub description: Option<String>,
    pub accent_color: Option<String>,
}
