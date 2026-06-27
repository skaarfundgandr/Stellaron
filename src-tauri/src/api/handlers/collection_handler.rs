use crate::application::state::AppState;
use crate::domain::error::DomainError;
use crate::domain::models::book::Book;
use crate::domain::models::collection::Collection;
use crate::domain::models::collection::{NewCollection, UpdateCollection};

pub async fn list_collections(state: &AppState) -> Result<Vec<Collection>, DomainError> {
    crate::application::collection::list_collections(&state.collection_repo).await
}

pub async fn get_collection(id: i32, state: &AppState) -> Result<Option<Collection>, DomainError> {
    crate::application::collection::get_collection(id, &state.collection_repo).await
}

pub async fn create_collection(
    name: String,
    description: Option<String>,
    accent_color: Option<String>,
    state: &AppState,
) -> Result<i32, DomainError> {
    crate::application::collection::create_collection(
        NewCollection {
            name,
            description,
            accent_color,
        },
        &state.collection_repo,
    )
    .await
}

pub async fn update_collection(
    id: i32,
    name: Option<String>,
    description: Option<String>,
    accent_color: Option<String>,
    state: &AppState,
) -> Result<(), DomainError> {
    crate::application::collection::update_collection(
        id,
        UpdateCollection {
            name,
            description,
            accent_color,
        },
        &state.collection_repo,
    )
    .await
}

pub async fn delete_collection(id: i32, state: &AppState) -> Result<(), DomainError> {
    crate::application::collection::delete_collection(id, &state.collection_repo).await
}

pub async fn add_book_to_collection(
    collection_id: i32,
    book_id: i32,
    state: &AppState,
) -> Result<(), DomainError> {
    crate::application::collection::add_book_to_collection(
        collection_id,
        book_id,
        &state.collection_repo,
    )
    .await
}

pub async fn remove_book_from_collection(
    collection_id: i32,
    book_id: i32,
    state: &AppState,
) -> Result<(), DomainError> {
    crate::application::collection::remove_book_from_collection(
        collection_id,
        book_id,
        &state.collection_repo,
    )
    .await
}

pub async fn get_collection_books(
    collection_id: i32,
    state: &AppState,
) -> Result<Vec<Book>, DomainError> {
    crate::application::collection::get_collection_books(collection_id, &state.collection_repo)
        .await
}

pub async fn get_collections_for_book(
    book_id: i32,
    state: &AppState,
) -> Result<Vec<Collection>, DomainError> {
    crate::application::collection::get_collections_for_book(book_id, &state.collection_repo).await
}
