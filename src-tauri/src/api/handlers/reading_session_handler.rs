use crate::application::state::AppState;
use crate::domain::error::DomainError;
use crate::domain::models::reading_session::ReadingSession;
use crate::domain::models::reading_session::{EndReadingSession, NewReadingSession};

pub async fn start_reading_session(
    book_id: i32,
    start_position: String,
    chapter_title: Option<String>,
    page_number: Option<i32>,
    start_ts: String,
    state: &AppState,
) -> Result<i32, DomainError> {
    crate::application::reading_session::start_session(
        NewReadingSession {
            book_id,
            start_position,
            chapter_title,
            page_number,
            start_ts,
        },
        &state.reading_session_repo,
    )
    .await
}

#[allow(clippy::too_many_arguments)]
pub async fn finish_reading_session(
    id: i32,
    end_position: Option<String>,
    chapter_title: Option<String>,
    page_number: Option<i32>,
    end_ts: String,
    duration_seconds: Option<i32>,
    pages_read: Option<i32>,
    state: &AppState,
) -> Result<(), DomainError> {
    crate::application::reading_session::finish_session(
        id,
        EndReadingSession {
            end_position,
            chapter_title,
            page_number,
            end_ts,
            duration_seconds,
            pages_read,
        },
        &state.reading_session_repo,
        &state.reading_progress_repo,
    )
    .await
}

pub async fn get_reading_sessions(
    book_id: i32,
    state: &AppState,
) -> Result<Vec<ReadingSession>, DomainError> {
    crate::application::reading_session::get_sessions_for_book(book_id, &state.reading_session_repo)
        .await
}

pub async fn get_reading_session(
    id: i32,
    state: &AppState,
) -> Result<Option<ReadingSession>, DomainError> {
    crate::application::reading_session::get_session(id, &state.reading_session_repo).await
}

pub async fn list_reading_sessions(state: &AppState) -> Result<Vec<ReadingSession>, DomainError> {
    crate::application::reading_session::list_sessions(&state.reading_session_repo).await
}

pub async fn delete_reading_session(id: i32, state: &AppState) -> Result<(), DomainError> {
    crate::application::reading_session::delete_session(id, &state.reading_session_repo).await
}
