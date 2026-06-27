CREATE TABLE collections (
    collection_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name         TEXT NOT NULL,
    description  TEXT,
    accent_color TEXT,
    created_at   TEXT DEFAULT (datetime('now'))
);

CREATE TABLE collection_books (
    collection_id INTEGER NOT NULL,
    book_id       INTEGER NOT NULL,
    added_at      TEXT DEFAULT (datetime('now')),
    PRIMARY KEY (collection_id, book_id),
    FOREIGN KEY (collection_id) REFERENCES collections(collection_id) ON DELETE CASCADE,
    FOREIGN KEY (book_id)       REFERENCES books(book_id)           ON DELETE CASCADE
);
CREATE INDEX idx_collection_books_collection ON collection_books(collection_id);
CREATE INDEX idx_collection_books_book       ON collection_books(book_id);
