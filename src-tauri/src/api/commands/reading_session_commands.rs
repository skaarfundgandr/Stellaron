use crate::api::handlers;
use crate::application::state::AppState;
use crate::domain::models::reading_session::ReadingSession;
use tauri::State;

#[tauri::command]
pub async fn start_reading_session(
    book_id: i32,
    start_position: String,
    chapter_title: Option<String>,
    page_number: Option<i32>,
    start_ts: String,
    state: State<'_, AppState>,
) -> Result<i32, String> {
    handlers::reading_session_handler::start_reading_session(
        book_id,
        start_position,
        chapter_title,
        page_number,
        start_ts,
        &state,
    )
    .await
    .map_err(|e| e.to_string())
}

#[allow(clippy::too_many_arguments)]
#[tauri::command]
pub async fn finish_reading_session(
    id: i32,
    end_position: Option<String>,
    chapter_title: Option<String>,
    page_number: Option<i32>,
    end_ts: String,
    duration_seconds: Option<i32>,
    pages_read: Option<i32>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    handlers::reading_session_handler::finish_reading_session(
        id,
        end_position,
        chapter_title,
        page_number,
        end_ts,
        duration_seconds,
        pages_read,
        &state,
    )
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_reading_sessions(
    book_id: i32,
    state: State<'_, AppState>,
) -> Result<Vec<ReadingSession>, String> {
    handlers::reading_session_handler::get_reading_sessions(book_id, &state)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_reading_session(
    id: i32,
    state: State<'_, AppState>,
) -> Result<Option<ReadingSession>, String> {
    handlers::reading_session_handler::get_reading_session(id, &state)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_reading_sessions(
    state: State<'_, AppState>,
) -> Result<Vec<ReadingSession>, String> {
    handlers::reading_session_handler::list_reading_sessions(&state)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_reading_session(id: i32, state: State<'_, AppState>) -> Result<(), String> {
    handlers::reading_session_handler::delete_reading_session(id, &state)
        .await
        .map_err(|e| e.to_string())
}
