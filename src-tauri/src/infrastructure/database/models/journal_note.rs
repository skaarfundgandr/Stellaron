use crate::infrastructure::database::models::schema::journal_notes;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Selectable, PartialEq, Debug)]
#[diesel(table_name = journal_notes)]
#[diesel(primary_key(journal_note_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct JournalNoteRow {
    pub journal_note_id: Option<i32>,
    pub book_id: i32,
    pub title: Option<String>,
    pub body: String,
    pub chapter_title: Option<String>,
    pub page_number: Option<i32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = journal_notes)]
pub struct NewJournalNoteRow<'a> {
    pub book_id: i32,
    pub title: Option<&'a str>,
    pub body: &'a str,
    pub chapter_title: Option<&'a str>,
    pub page_number: Option<i32>,
}

#[derive(AsChangeset, PartialEq, Debug)]
#[diesel(table_name = journal_notes)]
pub struct UpdateJournalNoteRow<'a> {
    pub title: Option<&'a str>,
    pub body: Option<&'a str>,
    pub updated_at: Option<&'a str>,
}

impl From<JournalNoteRow> for crate::domain::models::journal_note::JournalNote {
    fn from(row: JournalNoteRow) -> Self {
        crate::domain::models::journal_note::JournalNote {
            id: row.journal_note_id.unwrap_or(0),
            book_id: row.book_id,
            title: row.title,
            body: row.body,
            chapter_title: row.chapter_title,
            page_number: row.page_number,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
