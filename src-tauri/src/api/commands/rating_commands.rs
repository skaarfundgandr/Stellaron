use crate::api::handlers;
use crate::application::state::AppState;
use crate::domain::models::rating::Rating;
use tauri::State;

#[tauri::command]
pub async fn get_rating(
    book_id: i32,
    state: State<'_, AppState>,
) -> Result<Option<Rating>, String> {
    handlers::rating_handler::get_rating(book_id, &state)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn upsert_rating(
    book_id: i32,
    rating: i32,
    review: Option<String>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    handlers::rating_handler::upsert_rating(book_id, rating, review, &state)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_rating(book_id: i32, state: State<'_, AppState>) -> Result<(), String> {
    handlers::rating_handler::delete_rating(book_id, &state)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_ratings(state: State<'_, AppState>) -> Result<Vec<Rating>, String> {
    handlers::rating_handler::list_ratings(&state)
        .await
        .map_err(|e| e.to_string())
}
