//! Thin delegation layer between Tauri commands and application use cases.
//!
//! Handlers construct domain input structs from raw parameters and forward
//! them to the appropriate application-layer function.

pub mod annotation_handler;
pub mod book_handler;
pub mod bookmark_handler;
pub mod collection_handler;
pub mod journal_note_handler;
pub mod library_handler;
pub mod metadata_handler;
pub mod rating_handler;
pub mod reading_progress_handler;
pub mod reading_session_handler;
