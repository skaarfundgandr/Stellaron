use async_trait::async_trait;

use crate::domain::error::DomainError;
use crate::domain::models::annotation::{Annotation, NewAnnotation};
use crate::domain::models::author::Author;
use crate::domain::models::book::{Book, NewBook, UpdateBook};
use crate::domain::models::bookmark::{Bookmark, NewBookmark};
use crate::domain::models::collection::{Collection, NewCollection, UpdateCollection};
use crate::domain::models::journal_note::{JournalNote, NewJournalNote, UpdateJournalNote};
use crate::domain::models::publisher::Publisher;
use crate::domain::models::rating::{NewRating, Rating};
use crate::domain::models::reading_progress::{NewReadingProgress, ReadingProgress};
use crate::domain::models::reading_session::{
    EndReadingSession, NewReadingSession, ReadingSession,
};

/// Persistence operations for books.
#[async_trait]
pub trait BookRepository: Send + Sync {
    /// Returns all books in the library.
    ///
    /// # Returns
    ///
    /// A vector of all [`Book`] records.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn find_all(&self) -> Result<Vec<Book>, DomainError>;

    /// Returns the book with the given ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The book's database ID.
    ///
    /// # Returns
    ///
    /// `Some(Book)` if found, `None` otherwise.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn find_by_id(&self, id: i32) -> Result<Option<Book>, DomainError>;

    /// Inserts a new book and returns its generated ID.
    ///
    /// # Arguments
    ///
    /// * `book` - The new book data.
    ///
    /// # Returns
    ///
    /// The auto-generated primary key from the database.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on insert failure.
    async fn insert(&self, book: NewBook) -> Result<i32, DomainError>;

    /// Updates an existing book by ID.
    ///
    /// Only `Some` fields in [`UpdateBook`] are applied; `None` fields are left
    /// unchanged.
    ///
    /// # Arguments
    ///
    /// * `id` - The book's database ID.
    /// * `book` - Partial update data.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on update failure.
    async fn update(&self, id: i32, book: UpdateBook) -> Result<(), DomainError>;

    /// Deletes a book by ID.
    ///
    /// Related records (bookmarks, annotations, progress, author links) are
    /// cascade-deleted by SQLite foreign keys.
    ///
    /// # Arguments
    ///
    /// * `id` - The book's database ID.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn delete(&self, id: i32) -> Result<(), DomainError>;

    /// Returns the book matching the given SHA-256 checksum.
    ///
    /// Used for duplicate detection during import.
    ///
    /// # Arguments
    ///
    /// * `checksum` - The SHA-256 hex digest string.
    ///
    /// # Returns
    ///
    /// `Some(Book)` if a book with this checksum exists, `None` otherwise.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn find_by_checksum(&self, checksum: &str) -> Result<Option<Book>, DomainError>;

    /// Returns books whose title contains the given query string.
    ///
    /// Uses a SQL `LIKE` query (`%query%`) for substring matching. Case
    /// sensitivity depends on the SQLite collation.
    ///
    /// # Arguments
    ///
    /// * `title` - Substring to match against book titles.
    ///
    /// # Returns
    ///
    /// A (possibly empty) vector of matching [`Book`] records.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn search_by_title(&self, title: &str) -> Result<Vec<Book>, DomainError>;

    /// Imports a new book with author and publisher links in a single transaction.
    ///
    /// Inserts the book, links each author via the join table, and associates
    /// the publisher. Returns the fully-hydrated [`Book`] with its generated ID.
    ///
    /// # Arguments
    ///
    /// * `book` - The new book data.
    /// * `author_ids` - Slice of `(author_id, author_name)` tuples to link.
    /// * `publisher_id` - Optional publisher to associate.
    ///
    /// # Returns
    ///
    /// The fully-hydrated [`Book`] including generated ID and any foreign key
    /// links.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] if the insert or any link operation
    /// fails.
    async fn import_with_links(
        &self,
        book: NewBook,
        author_ids: &[(i32, String)],
        publisher_id: Option<i32>,
    ) -> Result<Book, DomainError>;
}

/// Persistence operations for authors.
#[async_trait]
pub trait AuthorRepository: Send + Sync {
    /// Returns an existing author by name, or creates and returns a new one.
    ///
    /// # Arguments
    ///
    /// * `name` - The author's full name.
    ///
    /// # Returns
    ///
    /// The existing or newly-created [`Author`].
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query or insert failure.
    async fn find_or_create(&self, name: &str) -> Result<Author, DomainError>;

    /// Returns all authors linked to the given book.
    ///
    /// # Arguments
    ///
    /// * `book_id` - The book's database ID.
    ///
    /// # Returns
    ///
    /// A vector of [`Author`] entities linked via `book_authors`.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn get_authors_by_book(&self, book_id: i32) -> Result<Vec<Author>, DomainError>;
}

/// Persistence operations for publishers.
#[async_trait]
pub trait PublisherRepository: Send + Sync {
    /// Returns the publisher with the given ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The publisher's database ID.
    ///
    /// # Returns
    ///
    /// `Some(Publisher)` if found, `None` otherwise.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn find_by_id(&self, id: i32) -> Result<Option<Publisher>, DomainError>;

    /// Returns an existing publisher by name, or creates and returns a new one.
    ///
    /// # Arguments
    ///
    /// * `name` - The publisher's name.
    ///
    /// # Returns
    ///
    /// The existing or newly-created [`Publisher`].
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query or insert failure.
    async fn find_or_create(&self, name: &str) -> Result<Publisher, DomainError>;
}

/// Persistence operations for the book-author many-to-many join table.
#[async_trait]
pub trait BookAuthorRepository: Send + Sync {
    /// Creates a link between a book and an author.
    ///
    /// # Arguments
    ///
    /// * `book_id` - The book's database ID.
    /// * `author_id` - The author's database ID.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on insert failure.
    async fn link(&self, book_id: i32, author_id: i32) -> Result<(), DomainError>;
}

/// Persistence operations for bookmarks.
#[async_trait]
pub trait BookmarkRepository: Send + Sync {
    /// Returns all bookmarks belonging to the given book.
    ///
    /// # Arguments
    ///
    /// * `book_id` - The book's database ID.
    ///
    /// # Returns
    ///
    /// A vector of [`Bookmark`] entities for the book.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn find_by_book(&self, book_id: i32) -> Result<Vec<Bookmark>, DomainError>;

    /// Creates a new bookmark.
    ///
    /// # Arguments
    ///
    /// * `bookmark` - The bookmark data to insert.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on insert failure.
    async fn insert(&self, bookmark: NewBookmark) -> Result<(), DomainError>;

    /// Deletes a bookmark by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The bookmark's database ID.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn delete(&self, id: i32) -> Result<(), DomainError>;
}

/// Persistence operations for annotations (highlights with optional notes).
#[async_trait]
pub trait AnnotationRepository: Send + Sync {
    /// Returns all annotations belonging to the given book.
    ///
    /// # Arguments
    ///
    /// * `book_id` - The book's database ID.
    ///
    /// # Returns
    ///
    /// A vector of [`Annotation`] entities for the book.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn find_by_book(&self, book_id: i32) -> Result<Vec<Annotation>, DomainError>;

    /// Creates a new annotation.
    ///
    /// # Arguments
    ///
    /// * `annotation` - The annotation data to insert.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on insert failure.
    async fn insert(&self, annotation: NewAnnotation) -> Result<(), DomainError>;

    /// Deletes an annotation by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The annotation's database ID.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn delete(&self, id: i32) -> Result<(), DomainError>;
}

/// Persistence operations for reading progress.
#[async_trait]
pub trait ReadingProgressRepository: Send + Sync {
    /// Returns the reading progress for a given book.
    ///
    /// # Arguments
    ///
    /// * `book_id` - The book's database ID.
    ///
    /// # Returns
    ///
    /// `Some(ReadingProgress)` if tracked, `None` if the book has never been
    /// opened.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn find_by_book(&self, book_id: i32) -> Result<Option<ReadingProgress>, DomainError>;

    /// Inserts or updates reading progress for a book (upsert).
    ///
    /// If a progress record already exists for the book, its fields are
    /// updated; otherwise a new record is created. The `last_read_at` timestamp
    /// is set automatically to the current UTC time.
    ///
    /// # Arguments
    ///
    /// * `progress` - The reading progress data to insert or merge.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn upsert(&self, progress: NewReadingProgress) -> Result<(), DomainError>;
}

// ---------------------------------------------------------------------------
// Ratings
// ---------------------------------------------------------------------------

/// Persistence operations for ratings.
#[async_trait]
pub trait RatingRepository: Send + Sync {
    /// Returns the rating for a book, or `None` if not rated.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn find_by_book(&self, book_id: i32) -> Result<Option<Rating>, DomainError>;

    /// Inserts a new rating or updates the existing one (upsert by `book_id`).
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn upsert(&self, rating: NewRating) -> Result<(), DomainError>;

    /// Deletes the rating for a book.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn delete(&self, book_id: i32) -> Result<(), DomainError>;

    /// Returns all ratings in the library.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn find_all(&self) -> Result<Vec<Rating>, DomainError>;
}

// ---------------------------------------------------------------------------
// Collections
// ---------------------------------------------------------------------------

/// Persistence operations for collections.
#[async_trait]
pub trait CollectionRepository: Send + Sync {
    /// Returns all collections.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn find_all(&self) -> Result<Vec<Collection>, DomainError>;

    /// Returns the collection with the given ID, or `None`.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn find_by_id(&self, id: i32) -> Result<Option<Collection>, DomainError>;

    /// Creates a new collection and returns its generated ID.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on insert failure.
    async fn create(&self, collection: NewCollection) -> Result<i32, DomainError>;

    /// Updates an existing collection. Only `Some` fields in [`UpdateCollection`]
    /// are applied.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on update failure.
    async fn update(&self, id: i32, collection: UpdateCollection) -> Result<(), DomainError>;

    /// Deletes a collection. Membership rows cascade-delete by SQLite foreign keys.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn delete(&self, id: i32) -> Result<(), DomainError>;

    /// Adds a book to a collection. Idempotent — re-adding is a no-op.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on insert failure.
    async fn add_book(&self, collection_id: i32, book_id: i32) -> Result<(), DomainError>;

    /// Removes a book from a collection.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on delete failure.
    async fn remove_book(&self, collection_id: i32, book_id: i32) -> Result<(), DomainError>;

    /// Returns all books belonging to the given collection, ordered by `added_at`.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn get_books(&self, collection_id: i32) -> Result<Vec<Book>, DomainError>;

    /// Returns all collections that contain the given book.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn get_collections_for_book(&self, book_id: i32) -> Result<Vec<Collection>, DomainError>;
}

// ---------------------------------------------------------------------------
// Reading Sessions
// ---------------------------------------------------------------------------

/// Persistence operations for reading sessions.
#[async_trait]
pub trait ReadingSessionRepository: Send + Sync {
    /// Returns all sessions for a book, ordered by `start_ts`.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn find_by_book(&self, book_id: i32) -> Result<Vec<ReadingSession>, DomainError>;

    /// Returns the session with the given ID, or `None`.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn find_by_id(&self, id: i32) -> Result<Option<ReadingSession>, DomainError>;

    /// Returns all reading sessions in the library.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn find_all(&self) -> Result<Vec<ReadingSession>, DomainError>;

    /// Creates a new reading session and returns its generated ID.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on insert failure.
    async fn create(&self, session: NewReadingSession) -> Result<i32, DomainError>;

    /// Finalizes a session by writing its end fields (`end_position`, `end_ts`,
    /// `duration_seconds`, `pages_read`, etc.).
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on update failure.
    async fn finish(&self, id: i32, end: EndReadingSession) -> Result<(), DomainError>;

    /// Deletes a session by ID.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn delete(&self, id: i32) -> Result<(), DomainError>;
}

// ---------------------------------------------------------------------------
// Journal Notes
// ---------------------------------------------------------------------------

/// Persistence operations for journal notes.
#[async_trait]
pub trait JournalNoteRepository: Send + Sync {
    /// Returns all journal notes for a book.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn find_by_book(&self, book_id: i32) -> Result<Vec<JournalNote>, DomainError>;

    /// Returns the journal note with the given ID, or `None`.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn find_by_id(&self, id: i32) -> Result<Option<JournalNote>, DomainError>;

    /// Inserts a new journal note and returns its generated ID.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on insert failure.
    async fn insert(&self, note: NewJournalNote) -> Result<i32, DomainError>;

    /// Updates an existing journal note. Only `Some` fields in [`UpdateJournalNote`]
    /// are applied.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on update failure.
    async fn update(&self, id: i32, note: UpdateJournalNote) -> Result<(), DomainError>;

    /// Deletes a journal note by ID.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::Database`] on query failure.
    async fn delete(&self, id: i32) -> Result<(), DomainError>;
}

// ---------------------------------------------------------------------------
