use crate::infrastructure::database::models::schema::ratings;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Selectable, PartialEq, Debug)]
#[diesel(table_name = ratings)]
#[diesel(primary_key(rating_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RatingRow {
    pub rating_id: Option<i32>,
    pub book_id: i32,
    pub rating: i32,
    pub review: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = ratings)]
pub struct NewRatingRow<'a> {
    pub book_id: i32,
    pub rating: i32,
    pub review: Option<&'a str>,
}

impl From<RatingRow> for crate::domain::models::rating::Rating {
    fn from(row: RatingRow) -> Self {
        crate::domain::models::rating::Rating {
            id: row.rating_id.unwrap_or(0),
            book_id: row.book_id,
            rating: row.rating,
            review: row.review,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
