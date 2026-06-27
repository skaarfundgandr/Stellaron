CREATE TABLE reading_sessions (
    session_id       INTEGER PRIMARY KEY AUTOINCREMENT,
    book_id          INTEGER NOT NULL,
    start_position   TEXT NOT NULL,
    end_position     TEXT,
    chapter_title    TEXT,
    page_number      INTEGER,
    start_ts         TEXT NOT NULL,
    end_ts           TEXT,
    duration_seconds INTEGER,
    pages_read       INTEGER,
    created_at       TEXT DEFAULT (datetime('now')),
    FOREIGN KEY (book_id) REFERENCES books(book_id) ON DELETE CASCADE
);
CREATE INDEX idx_reading_sessions_book    ON reading_sessions(book_id);
CREATE INDEX idx_reading_sessions_started ON reading_sessions(start_ts);
