use std::sync::Arc;

use crate::domain::error::DomainError;
use crate::domain::models::journal_note::JournalNote;
use crate::domain::models::journal_note::{NewJournalNote, UpdateJournalNote};
use crate::domain::repository::JournalNoteRepository;

pub async fn get_journal_notes(
    book_id: i32,
    repo: &Arc<dyn JournalNoteRepository>,
) -> Result<Vec<JournalNote>, DomainError> {
    repo.find_by_book(book_id).await
}

pub async fn get_journal_note(
    id: i32,
    repo: &Arc<dyn JournalNoteRepository>,
) -> Result<Option<JournalNote>, DomainError> {
    repo.find_by_id(id).await
}

pub async fn add_journal_note(
    input: NewJournalNote,
    repo: &Arc<dyn JournalNoteRepository>,
) -> Result<i32, DomainError> {
    repo.insert(input).await
}

pub async fn update_journal_note(
    id: i32,
    input: UpdateJournalNote,
    repo: &Arc<dyn JournalNoteRepository>,
) -> Result<(), DomainError> {
    repo.update(id, input).await
}

pub async fn delete_journal_note(
    id: i32,
    repo: &Arc<dyn JournalNoteRepository>,
) -> Result<(), DomainError> {
    repo.delete(id).await
}
