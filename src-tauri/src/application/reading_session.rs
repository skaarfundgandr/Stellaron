use std::sync::Arc;

use crate::domain::error::DomainError;
use crate::domain::models::reading_progress;
use crate::domain::models::reading_session::ReadingSession;
use crate::domain::models::reading_session::{EndReadingSession, NewReadingSession};
use crate::domain::repository::{ReadingProgressRepository, ReadingSessionRepository};

pub async fn start_session(
    input: NewReadingSession,
    repo: &Arc<dyn ReadingSessionRepository>,
) -> Result<i32, DomainError> {
    repo.create(input).await
}

pub async fn finish_session(
    id: i32,
    end: EndReadingSession,
    session_repo: &Arc<dyn ReadingSessionRepository>,
    progress_repo: &Arc<dyn ReadingProgressRepository>,
) -> Result<(), DomainError> {
    let session = session_repo.find_by_id(id).await?;
    let session = session.ok_or(DomainError::NotFound)?;

    session_repo.finish(id, end.clone()).await?;

    if let Some(ref pos) = end.end_position {
        let preserved_percentage = progress_repo
            .find_by_book(session.book_id)
            .await?
            .and_then(|p| p.progress_percentage);

        let progress = reading_progress::NewReadingProgress {
            book_id: session.book_id,
            current_position: pos.clone(),
            chapter_title: end.chapter_title.clone(),
            page_number: end.page_number,
            progress_percentage: preserved_percentage,
        };
        progress_repo.upsert(progress).await?;
    }

    Ok(())
}

pub async fn get_sessions_for_book(
    book_id: i32,
    repo: &Arc<dyn ReadingSessionRepository>,
) -> Result<Vec<ReadingSession>, DomainError> {
    repo.find_by_book(book_id).await
}

pub async fn get_session(
    id: i32,
    repo: &Arc<dyn ReadingSessionRepository>,
) -> Result<Option<ReadingSession>, DomainError> {
    repo.find_by_id(id).await
}

pub async fn list_sessions(
    repo: &Arc<dyn ReadingSessionRepository>,
) -> Result<Vec<ReadingSession>, DomainError> {
    repo.find_all().await
}

pub async fn delete_session(
    id: i32,
    repo: &Arc<dyn ReadingSessionRepository>,
) -> Result<(), DomainError> {
    repo.delete(id).await
}
