CREATE TABLE ratings (
    rating_id   INTEGER PRIMARY KEY AUTOINCREMENT,
    book_id     INTEGER NOT NULL UNIQUE,
    rating      INTEGER NOT NULL CHECK (rating BETWEEN 1 AND 5),
    review      TEXT,
    created_at  TEXT DEFAULT (datetime('now')),
    updated_at  TEXT DEFAULT (datetime('now')),
    FOREIGN KEY (book_id) REFERENCES books(book_id) ON DELETE CASCADE
);
CREATE INDEX idx_ratings_book ON ratings(book_id);
