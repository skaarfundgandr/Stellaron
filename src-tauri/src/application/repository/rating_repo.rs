use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, RunQueryDsl};

use crate::domain::error::DomainError;
use crate::domain::models::rating::NewRating;
use crate::domain::models::rating::Rating;
use crate::domain::repository::RatingRepository;
use crate::infrastructure::database::database::connect_from_pool;
use crate::infrastructure::database::models::rating::{NewRatingRow, RatingRow};
use crate::infrastructure::database::models::schema::ratings;

pub struct RatingRepoImpl;

impl RatingRepoImpl {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RatingRepoImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RatingRepository for RatingRepoImpl {
    async fn find_by_book(&self, find_book_id: i32) -> Result<Option<Rating>, DomainError> {
        let mut conn = connect_from_pool().await?;

        let rows = ratings::dsl::ratings
            .filter(ratings::book_id.eq(find_book_id))
            .limit(1)
            .load::<RatingRow>(&mut conn)
            .await?;
        match rows.into_iter().next() {
            Some(row) => Ok(Some(Rating::from(row))),
            None => Ok(None),
        }
    }

    async fn upsert(&self, new_rating: NewRating) -> Result<(), DomainError> {
        let mut conn = connect_from_pool().await?;
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        conn.transaction(async |connection| {
            diesel::insert_into(ratings::table)
                .values(&NewRatingRow {
                    book_id: new_rating.book_id,
                    rating: new_rating.rating,
                    review: new_rating.review.as_deref(),
                })
                .on_conflict(ratings::book_id)
                .do_update()
                .set((
                    ratings::rating.eq(new_rating.rating),
                    ratings::review.eq(&new_rating.review),
                    ratings::updated_at.eq(&now),
                ))
                .execute(connection)
                .await?;
            Ok::<(), diesel::result::Error>(())
        })
        .await?;

        Ok(())
    }

    async fn delete(&self, find_book_id: i32) -> Result<(), DomainError> {
        let mut conn = connect_from_pool().await?;

        conn.transaction(async |connection| {
            diesel::delete(ratings::dsl::ratings.filter(ratings::book_id.eq(find_book_id)))
                .execute(connection)
                .await?;
            Ok::<(), diesel::result::Error>(())
        })
        .await?;

        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Rating>, DomainError> {
        let mut conn = connect_from_pool().await?;

        let rows = ratings::dsl::ratings.load::<RatingRow>(&mut conn).await?;
        Ok(rows.into_iter().map(Rating::from).collect())
    }
}
