use crate::application::state::AppState;
use crate::domain::error::DomainError;
use crate::domain::models::journal_note::JournalNote;
use crate::domain::models::journal_note::{NewJournalNote, UpdateJournalNote};

pub async fn get_journal_notes(
    book_id: i32,
    state: &AppState,
) -> Result<Vec<JournalNote>, DomainError> {
    crate::application::journal_note::get_journal_notes(book_id, &state.journal_note_repo).await
}

pub async fn get_journal_note(
    id: i32,
    state: &AppState,
) -> Result<Option<JournalNote>, DomainError> {
    crate::application::journal_note::get_journal_note(id, &state.journal_note_repo).await
}

pub async fn add_journal_note(
    book_id: i32,
    title: Option<String>,
    body: String,
    chapter_title: Option<String>,
    page_number: Option<i32>,
    state: &AppState,
) -> Result<i32, DomainError> {
    crate::application::journal_note::add_journal_note(
        NewJournalNote {
            book_id,
            title,
            body,
            chapter_title,
            page_number,
        },
        &state.journal_note_repo,
    )
    .await
}

pub async fn update_journal_note(
    id: i32,
    title: Option<String>,
    body: Option<String>,
    state: &AppState,
) -> Result<(), DomainError> {
    crate::application::journal_note::update_journal_note(
        id,
        UpdateJournalNote { title, body },
        &state.journal_note_repo,
    )
    .await
}

pub async fn delete_journal_note(id: i32, state: &AppState) -> Result<(), DomainError> {
    crate::application::journal_note::delete_journal_note(id, &state.journal_note_repo).await
}
