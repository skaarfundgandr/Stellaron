use std::sync::Arc;

use crate::domain::error::DomainError;
use crate::domain::models::book::Book;
use crate::domain::models::collection::Collection;
use crate::domain::models::collection::{NewCollection, UpdateCollection};
use crate::domain::repository::CollectionRepository;

pub async fn list_collections(
    repo: &Arc<dyn CollectionRepository>,
) -> Result<Vec<Collection>, DomainError> {
    repo.find_all().await
}

pub async fn get_collection(
    id: i32,
    repo: &Arc<dyn CollectionRepository>,
) -> Result<Option<Collection>, DomainError> {
    repo.find_by_id(id).await
}

pub async fn create_collection(
    input: NewCollection,
    repo: &Arc<dyn CollectionRepository>,
) -> Result<i32, DomainError> {
    repo.create(input).await
}

pub async fn update_collection(
    id: i32,
    input: UpdateCollection,
    repo: &Arc<dyn CollectionRepository>,
) -> Result<(), DomainError> {
    repo.update(id, input).await
}

pub async fn delete_collection(
    id: i32,
    repo: &Arc<dyn CollectionRepository>,
) -> Result<(), DomainError> {
    repo.delete(id).await
}

pub async fn add_book_to_collection(
    collection_id: i32,
    book_id: i32,
    repo: &Arc<dyn CollectionRepository>,
) -> Result<(), DomainError> {
    repo.add_book(collection_id, book_id).await
}

pub async fn remove_book_from_collection(
    collection_id: i32,
    book_id: i32,
    repo: &Arc<dyn CollectionRepository>,
) -> Result<(), DomainError> {
    repo.remove_book(collection_id, book_id).await
}

pub async fn get_collection_books(
    collection_id: i32,
    repo: &Arc<dyn CollectionRepository>,
) -> Result<Vec<Book>, DomainError> {
    repo.get_books(collection_id).await
}

pub async fn get_collections_for_book(
    book_id: i32,
    repo: &Arc<dyn CollectionRepository>,
) -> Result<Vec<Collection>, DomainError> {
    repo.get_collections_for_book(book_id).await
}
