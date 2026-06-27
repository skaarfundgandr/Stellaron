use std::sync::Arc;

use crate::domain::error::DomainError;
use crate::domain::models::rating::NewRating;
use crate::domain::models::rating::Rating;
use crate::domain::repository::RatingRepository;

pub async fn get_rating(
    book_id: i32,
    repo: &Arc<dyn RatingRepository>,
) -> Result<Option<Rating>, DomainError> {
    repo.find_by_book(book_id).await
}

pub async fn upsert_rating(
    input: NewRating,
    repo: &Arc<dyn RatingRepository>,
) -> Result<(), DomainError> {
    if !(1..=5).contains(&input.rating) {
        return Err(DomainError::Validation(format!(
            "Rating must be between 1 and 5, got {}",
            input.rating
        )));
    }
    repo.upsert(input).await
}

pub async fn delete_rating(
    book_id: i32,
    repo: &Arc<dyn RatingRepository>,
) -> Result<(), DomainError> {
    repo.delete(book_id).await
}

pub async fn list_ratings(repo: &Arc<dyn RatingRepository>) -> Result<Vec<Rating>, DomainError> {
    repo.find_all().await
}
