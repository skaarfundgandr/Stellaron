use crate::api::handlers;
use crate::application::state::AppState;
use crate::domain::models::journal_note::JournalNote;
use tauri::State;

#[tauri::command]
pub async fn add_journal_note(
    book_id: i32,
    title: Option<String>,
    body: String,
    chapter_title: Option<String>,
    page_number: Option<i32>,
    state: State<'_, AppState>,
) -> Result<i32, String> {
    handlers::journal_note_handler::add_journal_note(
        book_id,
        title,
        body,
        chapter_title,
        page_number,
        &state,
    )
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_journal_notes(
    book_id: i32,
    state: State<'_, AppState>,
) -> Result<Vec<JournalNote>, String> {
    handlers::journal_note_handler::get_journal_notes(book_id, &state)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_journal_note(
    id: i32,
    state: State<'_, AppState>,
) -> Result<Option<JournalNote>, String> {
    handlers::journal_note_handler::get_journal_note(id, &state)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_journal_note(
    id: i32,
    title: Option<String>,
    body: Option<String>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    handlers::journal_note_handler::update_journal_note(id, title, body, &state)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_journal_note(id: i32, state: State<'_, AppState>) -> Result<(), String> {
    handlers::journal_note_handler::delete_journal_note(id, &state)
        .await
        .map_err(|e| e.to_string())
}
