// @generated automatically by Diesel CLI.

diesel::table! {
    annotations (annotation_id) {
        annotation_id -> Nullable<Integer>,
        book_id -> Integer,
        chapter_title -> Nullable<Text>,
        start_position -> Text,
        end_position -> Text,
        highlighted_text -> Nullable<Text>,
        note -> Nullable<Text>,
        color -> Nullable<Text>,
        created_at -> Nullable<Text>,
        updated_at -> Nullable<Text>,
    }
}

diesel::table! {
    authors (author_id) {
        author_id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    book_authors (book_id, author_id) {
        book_id -> Integer,
        author_id -> Integer,
    }
}

diesel::table! {
    bookmarks (bookmark_id) {
        bookmark_id -> Nullable<Integer>,
        book_id -> Integer,
        chapter_title -> Nullable<Text>,
        page_number -> Nullable<Integer>,
        position -> Text,
        created_at -> Nullable<Text>,
    }
}

diesel::table! {
    books (book_id) {
        book_id -> Nullable<Integer>,
        title -> Text,
        published_date -> Nullable<Text>,
        publisher_id -> Nullable<Integer>,
        isbn -> Nullable<Text>,
        file_type -> Text,
        file_path -> Text,
        cover_image_path -> Nullable<Text>,
        checksum -> Nullable<Text>,
        added_at -> Nullable<Text>,
    }
}

diesel::table! {
    collection_books (collection_id, book_id) {
        collection_id -> Integer,
        book_id -> Integer,
        added_at -> Nullable<Text>,
    }
}

diesel::table! {
    collections (collection_id) {
        collection_id -> Nullable<Integer>,
        name -> Text,
        description -> Nullable<Text>,
        accent_color -> Nullable<Text>,
        created_at -> Nullable<Text>,
    }
}

diesel::table! {
    journal_notes (journal_note_id) {
        journal_note_id -> Nullable<Integer>,
        book_id -> Integer,
        title -> Nullable<Text>,
        body -> Text,
        chapter_title -> Nullable<Text>,
        page_number -> Nullable<Integer>,
        created_at -> Nullable<Text>,
        updated_at -> Nullable<Text>,
    }
}

diesel::table! {
    publishers (publisher_id) {
        publisher_id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    ratings (rating_id) {
        rating_id -> Nullable<Integer>,
        book_id -> Integer,
        rating -> Integer,
        review -> Nullable<Text>,
        created_at -> Nullable<Text>,
        updated_at -> Nullable<Text>,
    }
}

diesel::table! {
    reading_progress (progress_id) {
        progress_id -> Nullable<Integer>,
        book_id -> Integer,
        current_position -> Text,
        chapter_title -> Nullable<Text>,
        page_number -> Nullable<Integer>,
        progress_percentage -> Nullable<Float>,
        last_read_at -> Nullable<Text>,
    }
}

diesel::table! {
    reading_sessions (session_id) {
        session_id -> Nullable<Integer>,
        book_id -> Integer,
        start_position -> Text,
        end_position -> Nullable<Text>,
        chapter_title -> Nullable<Text>,
        page_number -> Nullable<Integer>,
        start_ts -> Text,
        end_ts -> Nullable<Text>,
        duration_seconds -> Nullable<Integer>,
        pages_read -> Nullable<Integer>,
        created_at -> Nullable<Text>,
    }
}

diesel::joinable!(annotations -> books (book_id));
diesel::joinable!(book_authors -> authors (author_id));
diesel::joinable!(book_authors -> books (book_id));
diesel::joinable!(bookmarks -> books (book_id));
diesel::joinable!(books -> publishers (publisher_id));
diesel::joinable!(collection_books -> books (book_id));
diesel::joinable!(collection_books -> collections (collection_id));
diesel::joinable!(journal_notes -> books (book_id));
diesel::joinable!(ratings -> books (book_id));
diesel::joinable!(reading_progress -> books (book_id));
diesel::joinable!(reading_sessions -> books (book_id));

diesel::allow_tables_to_appear_in_same_query!(
    annotations,
    authors,
    book_authors,
    bookmarks,
    books,
    collection_books,
    collections,
    journal_notes,
    publishers,
    ratings,
    reading_progress,
    reading_sessions,
);
