use crate::application::state::AppState;
use crate::domain::error::DomainError;
use crate::domain::models::rating::NewRating;
use crate::domain::models::rating::Rating;

pub async fn get_rating(book_id: i32, state: &AppState) -> Result<Option<Rating>, DomainError> {
    crate::application::rating::get_rating(book_id, &state.rating_repo).await
}

pub async fn upsert_rating(
    book_id: i32,
    rating: i32,
    review: Option<String>,
    state: &AppState,
) -> Result<(), DomainError> {
    crate::application::rating::upsert_rating(
        NewRating {
            book_id,
            rating,
            review,
        },
        &state.rating_repo,
    )
    .await
}

pub async fn delete_rating(book_id: i32, state: &AppState) -> Result<(), DomainError> {
    crate::application::rating::delete_rating(book_id, &state.rating_repo).await
}

pub async fn list_ratings(state: &AppState) -> Result<Vec<Rating>, DomainError> {
    crate::application::rating::list_ratings(&state.rating_repo).await
}
