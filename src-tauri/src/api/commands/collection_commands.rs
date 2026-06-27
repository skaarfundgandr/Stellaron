use crate::api::handlers;
use crate::application::state::AppState;
use crate::domain::models::book::Book;
use crate::domain::models::collection::Collection;
use tauri::State;

#[tauri::command]
pub async fn list_collections(state: State<'_, AppState>) -> Result<Vec<Collection>, String> {
    handlers::collection_handler::list_collections(&state)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_collection(
    id: i32,
    state: State<'_, AppState>,
) -> Result<Option<Collection>, String> {
    handlers::collection_handler::get_collection(id, &state)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_collection(
    name: String,
    description: Option<String>,
    accent_color: Option<String>,
    state: State<'_, AppState>,
) -> Result<i32, String> {
    handlers::collection_handler::create_collection(name, description, accent_color, &state)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_collection(
    id: i32,
    name: Option<String>,
    description: Option<String>,
    accent_color: Option<String>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    handlers::collection_handler::update_collection(id, name, description, accent_color, &state)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_collection(id: i32, state: State<'_, AppState>) -> Result<(), String> {
    handlers::collection_handler::delete_collection(id, &state)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_book_to_collection(
    collection_id: i32,
    book_id: i32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    handlers::collection_handler::add_book_to_collection(collection_id, book_id, &state)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_book_from_collection(
    collection_id: i32,
    book_id: i32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    handlers::collection_handler::remove_book_from_collection(collection_id, book_id, &state)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_collection_books(
    collection_id: i32,
    state: State<'_, AppState>,
) -> Result<Vec<Book>, String> {
    handlers::collection_handler::get_collection_books(collection_id, &state)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_collections_for_book(
    book_id: i32,
    state: State<'_, AppState>,
) -> Result<Vec<Collection>, String> {
    handlers::collection_handler::get_collections_for_book(book_id, &state)
        .await
        .map_err(|e| e.to_string())
}
