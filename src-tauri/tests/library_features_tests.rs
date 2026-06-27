use std::sync::Arc;

use stellaron_lib::domain::error::DomainError;
use stellaron_lib::domain::models::book::NewBook;
use stellaron_lib::domain::models::collection::{NewCollection, UpdateCollection};
use stellaron_lib::domain::models::journal_note::{NewJournalNote, UpdateJournalNote};
use stellaron_lib::domain::models::rating::NewRating;
use stellaron_lib::domain::models::reading_progress::NewReadingProgress;
use stellaron_lib::domain::models::reading_session::{EndReadingSession, NewReadingSession};
use stellaron_lib::domain::repository::{
    BookRepository, CollectionRepository, JournalNoteRepository, RatingRepository,
    ReadingProgressRepository, ReadingSessionRepository,
};

use stellaron_lib::application::repository::book_repo::BookRepoImpl;
use stellaron_lib::application::repository::collection_repo::CollectionRepoImpl;
use stellaron_lib::application::repository::journal_note_repo::JournalNoteRepoImpl;
use stellaron_lib::application::repository::rating_repo::RatingRepoImpl;
use stellaron_lib::application::repository::reading_progress_repo::ReadingProgressRepoImpl;
use stellaron_lib::application::repository::reading_session_repo::ReadingSessionRepoImpl;

struct TestRepos {
    book: Arc<dyn BookRepository>,
    rating: Arc<dyn RatingRepository>,
    collection: Arc<dyn CollectionRepository>,
    session: Arc<dyn ReadingSessionRepository>,
    progress: Arc<dyn ReadingProgressRepository>,
    note: Arc<dyn JournalNoteRepository>,
}

impl TestRepos {
    fn new() -> Self {
        Self {
            book: Arc::new(BookRepoImpl::new()),
            rating: Arc::new(RatingRepoImpl::new()),
            collection: Arc::new(CollectionRepoImpl::new()),
            session: Arc::new(ReadingSessionRepoImpl::new()),
            progress: Arc::new(ReadingProgressRepoImpl::new()),
            note: Arc::new(JournalNoteRepoImpl::new()),
        }
    }
}

async fn create_test_book(repos: &TestRepos, title: &str) -> i32 {
    repos
        .book
        .insert(NewBook {
            title: title.to_string(),
            published_date: None,
            publisher_id: None,
            isbn: None,
            file_type: "epub".to_string(),
            file_path: format!("/tmp/{}.epub", title),
            cover_image_path: None,
            checksum: Some(format!("test-checksum-{}", title)),
        })
        .await
        .expect("Failed to create test book")
}

// ---------------------------------------------------------------------------
// Ratings
// ---------------------------------------------------------------------------

#[tokio::test]
#[serial_test::serial]
async fn test_rating_upsert_updates_value() {
    let repos = TestRepos::new();
    let book_id = create_test_book(&repos, "rating-upsert-test").await;

    stellaron_lib::application::rating::upsert_rating(
        NewRating {
            book_id,
            rating: 3,
            review: None,
        },
        &repos.rating,
    )
    .await
    .unwrap();

    let r = repos.rating.find_by_book(book_id).await.unwrap();
    assert!(r.is_some());
    assert_eq!(r.unwrap().rating, 3);

    stellaron_lib::application::rating::upsert_rating(
        NewRating {
            book_id,
            rating: 5,
            review: None,
        },
        &repos.rating,
    )
    .await
    .unwrap();

    let r = repos.rating.find_by_book(book_id).await.unwrap();
    assert!(r.is_some());
    assert_eq!(r.unwrap().rating, 5);

    let all = repos.rating.find_all().await.unwrap();
    assert!(all.iter().any(|r| r.book_id == book_id));
}

#[tokio::test]
#[serial_test::serial]
async fn test_rating_validation_rejects_out_of_range() {
    let repos = TestRepos::new();
    let book_id = create_test_book(&repos, "rating-validation-test").await;

    let result = stellaron_lib::application::rating::upsert_rating(
        NewRating {
            book_id,
            rating: 0,
            review: None,
        },
        &repos.rating,
    )
    .await;
    assert!(matches!(result, Err(DomainError::Validation(_))));

    let result = stellaron_lib::application::rating::upsert_rating(
        NewRating {
            book_id,
            rating: 6,
            review: None,
        },
        &repos.rating,
    )
    .await;
    assert!(matches!(result, Err(DomainError::Validation(_))));
}

#[tokio::test]
#[serial_test::serial]
async fn test_rating_delete() {
    let repos = TestRepos::new();
    let book_id = create_test_book(&repos, "rating-delete-test").await;

    stellaron_lib::application::rating::upsert_rating(
        NewRating {
            book_id,
            rating: 4,
            review: None,
        },
        &repos.rating,
    )
    .await
    .unwrap();

    stellaron_lib::application::rating::delete_rating(book_id, &repos.rating)
        .await
        .unwrap();

    let r = repos.rating.find_by_book(book_id).await.unwrap();
    assert!(r.is_none());
}

// ---------------------------------------------------------------------------
// Collections
// ---------------------------------------------------------------------------

#[tokio::test]
#[serial_test::serial]
async fn test_collection_full_crud() {
    let repos = TestRepos::new();

    let col_id = repos
        .collection
        .create(NewCollection {
            name: "Favorites".to_string(),
            description: Some("A curated shortlist".to_string()),
            accent_color: Some("#ff6b35".to_string()),
        })
        .await
        .unwrap();

    let col = repos.collection.find_by_id(col_id).await.unwrap();
    assert!(col.is_some());
    assert_eq!(col.unwrap().name, "Favorites");

    repos
        .collection
        .update(
            col_id,
            UpdateCollection {
                name: Some("Best".to_string()),
                description: None,
                accent_color: None,
            },
        )
        .await
        .unwrap();

    let col = repos.collection.find_by_id(col_id).await.unwrap();
    assert!(col.is_some());
    let col = col.unwrap();
    assert_eq!(col.name, "Best");
    assert_eq!(col.description.as_deref(), Some("A curated shortlist"));
    assert_eq!(col.accent_color.as_deref(), Some("#ff6b35"));

    repos.collection.delete(col_id).await.unwrap();

    let col = repos.collection.find_by_id(col_id).await.unwrap();
    assert!(col.is_none());
}

#[tokio::test]
#[serial_test::serial]
async fn test_collection_add_remove_books() {
    let repos = TestRepos::new();
    let book1 = create_test_book(&repos, "coll-book-1").await;
    let book2 = create_test_book(&repos, "coll-book-2").await;

    let col_id = repos
        .collection
        .create(NewCollection {
            name: "Reading List".to_string(),
            description: None,
            accent_color: None,
        })
        .await
        .unwrap();

    repos.collection.add_book(col_id, book1).await.unwrap();
    repos.collection.add_book(col_id, book2).await.unwrap();

    let books = repos.collection.get_books(col_id).await.unwrap();
    assert_eq!(books.len(), 2);
    let ids: Vec<i32> = books.iter().map(|b| b.id).collect();
    assert!(ids.contains(&book1));
    assert!(ids.contains(&book2));

    repos.collection.remove_book(col_id, book1).await.unwrap();

    let books = repos.collection.get_books(col_id).await.unwrap();
    assert_eq!(books.len(), 1);
    assert_eq!(books[0].id, book2);

    // book1 was removed, so it should no longer appear in this collection
    let cols = repos
        .collection
        .get_collections_for_book(book1)
        .await
        .unwrap();
    assert!(!cols.iter().any(|c| c.id == col_id));
}

#[tokio::test]
#[serial_test::serial]
async fn test_collection_delete_cascades_membership() {
    let repos = TestRepos::new();
    let book1 = create_test_book(&repos, "coll-cascade-1").await;
    let book2 = create_test_book(&repos, "coll-cascade-2").await;

    let col_id = repos
        .collection
        .create(NewCollection {
            name: "Temp".to_string(),
            description: None,
            accent_color: None,
        })
        .await
        .unwrap();

    repos.collection.add_book(col_id, book1).await.unwrap();
    repos.collection.add_book(col_id, book2).await.unwrap();

    repos.collection.delete(col_id).await.unwrap();

    let books = repos.collection.get_books(col_id).await.unwrap();
    assert!(books.is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn test_collection_add_book_idempotent() {
    let repos = TestRepos::new();
    let book_id = create_test_book(&repos, "coll-idempotent-test").await;

    let col_id = repos
        .collection
        .create(NewCollection {
            name: "Idempotent".to_string(),
            description: None,
            accent_color: None,
        })
        .await
        .unwrap();

    repos.collection.add_book(col_id, book_id).await.unwrap();
    repos.collection.add_book(col_id, book_id).await.unwrap();

    let books = repos.collection.get_books(col_id).await.unwrap();
    assert_eq!(books.len(), 1);
}

// ---------------------------------------------------------------------------
// Reading Sessions
// ---------------------------------------------------------------------------

#[tokio::test]
#[serial_test::serial]
async fn test_session_finish_updates_progress() {
    let repos = TestRepos::new();
    let book_id = create_test_book(&repos, "session-finish-test").await;

    let session_id = repos
        .session
        .create(NewReadingSession {
            book_id,
            start_position: "page-0".to_string(),
            chapter_title: None,
            page_number: None,
            start_ts: "2025-01-01T00:00:00Z".to_string(),
        })
        .await
        .unwrap();

    stellaron_lib::application::reading_session::finish_session(
        session_id,
        EndReadingSession {
            end_position: Some("page-100".to_string()),
            chapter_title: None,
            page_number: None,
            end_ts: "2025-01-01T01:00:00Z".to_string(),
            duration_seconds: Some(3600),
            pages_read: Some(100),
        },
        &repos.session,
        &repos.progress,
    )
    .await
    .unwrap();

    let progress = repos.progress.find_by_book(book_id).await.unwrap();
    assert!(progress.is_some());
    let p = progress.unwrap();
    assert_eq!(p.current_position, "page-100");

    let session = repos.session.find_by_id(session_id).await.unwrap();
    assert!(session.is_some());
    let s = session.unwrap();
    assert!(s.end_ts.is_some());
    assert!(s.end_position.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn test_session_finish_no_end_position_skips_progress() {
    let repos = TestRepos::new();
    let book_id = create_test_book(&repos, "session-no-progress-test").await;

    let session_id = repos
        .session
        .create(NewReadingSession {
            book_id,
            start_position: "page-0".to_string(),
            chapter_title: None,
            page_number: None,
            start_ts: "2025-01-01T00:00:00Z".to_string(),
        })
        .await
        .unwrap();

    stellaron_lib::application::reading_session::finish_session(
        session_id,
        EndReadingSession {
            end_position: None,
            chapter_title: None,
            page_number: None,
            end_ts: "2025-01-01T01:00:00Z".to_string(),
            duration_seconds: None,
            pages_read: None,
        },
        &repos.session,
        &repos.progress,
    )
    .await
    .unwrap();

    let progress = repos.progress.find_by_book(book_id).await.unwrap();
    assert!(progress.is_none());

    let session = repos.session.find_by_id(session_id).await.unwrap();
    assert!(session.is_some());
    assert!(session.unwrap().end_ts.is_some());
}

#[tokio::test]
#[serial_test::serial]
async fn test_session_finish_overwrites_progress() {
    let repos = TestRepos::new();
    let book_id = create_test_book(&repos, "session-overwrite-test").await;

    let sess_a = repos
        .session
        .create(NewReadingSession {
            book_id,
            start_position: "page-0".to_string(),
            chapter_title: None,
            page_number: None,
            start_ts: "2025-01-01T00:00:00Z".to_string(),
        })
        .await
        .unwrap();

    stellaron_lib::application::reading_session::finish_session(
        sess_a,
        EndReadingSession {
            end_position: Some("page-50".to_string()),
            chapter_title: None,
            page_number: None,
            end_ts: "2025-01-01T00:30:00Z".to_string(),
            duration_seconds: Some(1800),
            pages_read: Some(50),
        },
        &repos.session,
        &repos.progress,
    )
    .await
    .unwrap();

    let progress = repos.progress.find_by_book(book_id).await.unwrap();
    assert_eq!(progress.unwrap().current_position, "page-50");

    let sess_b = repos
        .session
        .create(NewReadingSession {
            book_id,
            start_position: "page-50".to_string(),
            chapter_title: None,
            page_number: None,
            start_ts: "2025-01-01T00:30:00Z".to_string(),
        })
        .await
        .unwrap();

    stellaron_lib::application::reading_session::finish_session(
        sess_b,
        EndReadingSession {
            end_position: Some("page-200".to_string()),
            chapter_title: None,
            page_number: None,
            end_ts: "2025-01-01T01:30:00Z".to_string(),
            duration_seconds: Some(3600),
            pages_read: Some(150),
        },
        &repos.session,
        &repos.progress,
    )
    .await
    .unwrap();

    let progress = repos.progress.find_by_book(book_id).await.unwrap();
    assert_eq!(progress.unwrap().current_position, "page-200");
}

#[tokio::test]
#[serial_test::serial]
async fn test_session_finish_preserves_progress_percentage() {
    let repos = TestRepos::new();
    let book_id = create_test_book(&repos, "session-preserve-pct-test").await;

    stellaron_lib::application::reading_progress::update_progress(
        NewReadingProgress {
            book_id,
            current_position: "loc-10".to_string(),
            chapter_title: None,
            page_number: None,
            progress_percentage: Some(45.0),
        },
        &repos.progress,
    )
    .await
    .unwrap();

    let session_id = repos
        .session
        .create(NewReadingSession {
            book_id,
            start_position: "loc-10".to_string(),
            chapter_title: None,
            page_number: None,
            start_ts: "2025-01-01T00:00:00Z".to_string(),
        })
        .await
        .unwrap();

    stellaron_lib::application::reading_session::finish_session(
        session_id,
        EndReadingSession {
            end_position: Some("loc-50".to_string()),
            chapter_title: None,
            page_number: None,
            end_ts: "2025-01-01T00:30:00Z".to_string(),
            duration_seconds: Some(1800),
            pages_read: Some(40),
        },
        &repos.session,
        &repos.progress,
    )
    .await
    .unwrap();

    let progress = repos.progress.find_by_book(book_id).await.unwrap();
    assert!(progress.is_some());
    let p = progress.unwrap();
    assert_eq!(p.current_position, "loc-50");
    assert_eq!(p.progress_percentage, Some(45.0));
}

#[tokio::test]
#[serial_test::serial]
async fn test_session_finish_missing_id_errors() {
    let repos = TestRepos::new();

    let result = stellaron_lib::application::reading_session::finish_session(
        99999999,
        EndReadingSession {
            end_position: Some("page-10".to_string()),
            chapter_title: None,
            page_number: None,
            end_ts: "2025-01-01T01:00:00Z".to_string(),
            duration_seconds: None,
            pages_read: None,
        },
        &repos.session,
        &repos.progress,
    )
    .await;
    assert!(matches!(result, Err(DomainError::NotFound)));
}

// ---------------------------------------------------------------------------
// Journal Notes
// ---------------------------------------------------------------------------

#[tokio::test]
#[serial_test::serial]
async fn test_journal_note_crud() {
    let repos = TestRepos::new();
    let book_id = create_test_book(&repos, "journal-crud-test").await;

    let note_id = repos
        .note
        .insert(NewJournalNote {
            book_id,
            title: Some("Reflections".to_string()),
            body: "Lorem ipsum".to_string(),
            chapter_title: None,
            page_number: None,
        })
        .await
        .unwrap();

    let notes = repos.note.find_by_book(book_id).await.unwrap();
    assert_eq!(notes.len(), 1);

    let note = repos.note.find_by_id(note_id).await.unwrap();
    assert!(note.is_some());
    let note = note.unwrap();
    assert_eq!(note.title.as_deref(), Some("Reflections"));
    assert_eq!(note.body, "Lorem ipsum");

    repos
        .note
        .update(
            note_id,
            UpdateJournalNote {
                title: None,
                body: Some("Updated body text".to_string()),
            },
        )
        .await
        .unwrap();

    let note = repos.note.find_by_id(note_id).await.unwrap().unwrap();
    assert_eq!(note.title.as_deref(), Some("Reflections"));
    assert_eq!(note.body, "Updated body text");

    repos.note.delete(note_id).await.unwrap();

    let notes = repos.note.find_by_book(book_id).await.unwrap();
    assert!(notes.is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn test_journal_note_multiple() {
    let repos = TestRepos::new();
    let book_id = create_test_book(&repos, "journal-multi-test").await;

    repos
        .note
        .insert(NewJournalNote {
            book_id,
            title: Some("Note 1".to_string()),
            body: "Body 1".to_string(),
            chapter_title: None,
            page_number: None,
        })
        .await
        .unwrap();

    repos
        .note
        .insert(NewJournalNote {
            book_id,
            title: Some("Note 2".to_string()),
            body: "Body 2".to_string(),
            chapter_title: None,
            page_number: None,
        })
        .await
        .unwrap();

    let notes = repos.note.find_by_book(book_id).await.unwrap();
    assert_eq!(notes.len(), 2);
}

// ---------------------------------------------------------------------------
// Cross-feature
// ---------------------------------------------------------------------------

#[tokio::test]
#[serial_test::serial]
async fn test_cross_feature_book_with_all_features() {
    let repos = TestRepos::new();
    let book_id = create_test_book(&repos, "cross-feature-test").await;

    // Rating
    stellaron_lib::application::rating::upsert_rating(
        NewRating {
            book_id,
            rating: 4,
            review: None,
        },
        &repos.rating,
    )
    .await
    .unwrap();

    // Collection
    let col_id = repos
        .collection
        .create(NewCollection {
            name: "Cross Feature".to_string(),
            description: None,
            accent_color: None,
        })
        .await
        .unwrap();
    repos.collection.add_book(col_id, book_id).await.unwrap();

    // Reading session + progress
    let session_id = repos
        .session
        .create(NewReadingSession {
            book_id,
            start_position: "chapter-0".to_string(),
            chapter_title: None,
            page_number: None,
            start_ts: "2025-01-01T00:00:00Z".to_string(),
        })
        .await
        .unwrap();

    stellaron_lib::application::reading_session::finish_session(
        session_id,
        EndReadingSession {
            end_position: Some("chapter-3".to_string()),
            chapter_title: None,
            page_number: None,
            end_ts: "2025-01-01T02:00:00Z".to_string(),
            duration_seconds: Some(7200),
            pages_read: None,
        },
        &repos.session,
        &repos.progress,
    )
    .await
    .unwrap();

    // Journal note
    repos
        .note
        .insert(NewJournalNote {
            book_id,
            title: Some("My Thoughts".to_string()),
            body: "Great book so far".to_string(),
            chapter_title: None,
            page_number: None,
        })
        .await
        .unwrap();

    // Verify all retrievable
    let rating = repos.rating.find_by_book(book_id).await.unwrap();
    assert!(rating.is_some());
    assert_eq!(rating.unwrap().rating, 4);

    let collections = repos
        .collection
        .get_collections_for_book(book_id)
        .await
        .unwrap();
    assert!(!collections.is_empty());

    let progress = repos.progress.find_by_book(book_id).await.unwrap();
    assert!(progress.is_some());
    assert_eq!(progress.unwrap().current_position, "chapter-3");

    let notes = repos.note.find_by_book(book_id).await.unwrap();
    assert_eq!(notes.len(), 1);

    // Delete book cascades
    repos.book.delete(book_id).await.unwrap();

    assert!(repos.rating.find_by_book(book_id).await.unwrap().is_none());
    assert!(
        repos
            .collection
            .get_collections_for_book(book_id)
            .await
            .unwrap()
            .is_empty()
    );
    assert!(
        repos
            .progress
            .find_by_book(book_id)
            .await
            .unwrap()
            .is_none()
    );
    assert!(repos.note.find_by_book(book_id).await.unwrap().is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn test_collection_books_returns_full_book() {
    let repos = TestRepos::new();
    let book1 = create_test_book(&repos, "coll-full-book-1").await;
    let book2 = create_test_book(&repos, "coll-full-book-2").await;

    let col_id = repos
        .collection
        .create(NewCollection {
            name: "Full Books".to_string(),
            description: None,
            accent_color: None,
        })
        .await
        .unwrap();

    repos.collection.add_book(col_id, book1).await.unwrap();
    repos.collection.add_book(col_id, book2).await.unwrap();

    let books = repos.collection.get_books(col_id).await.unwrap();
    assert_eq!(books.len(), 2);

    for book in &books {
        assert!(!book.title.is_empty());
        assert!(book.file_type.is_some());
        assert!(book.file_path.is_some());
    }
}
