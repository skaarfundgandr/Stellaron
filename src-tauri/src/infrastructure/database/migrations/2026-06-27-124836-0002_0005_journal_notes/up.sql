CREATE TABLE journal_notes (
    journal_note_id INTEGER PRIMARY KEY AUTOINCREMENT,
    book_id        INTEGER NOT NULL,
    title          TEXT,
    body           TEXT NOT NULL,
    chapter_title  TEXT,
    page_number    INTEGER,
    created_at     TEXT DEFAULT (datetime('now')),
    updated_at     TEXT DEFAULT (datetime('now')),
    FOREIGN KEY (book_id) REFERENCES books(book_id) ON DELETE CASCADE
);
CREATE INDEX idx_journal_notes_book ON journal_notes(book_id);
