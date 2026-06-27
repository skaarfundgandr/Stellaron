use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Integer;
use diesel_async::{AsyncConnection, RunQueryDsl};

use crate::domain::error::DomainError;
use crate::domain::models::journal_note::JournalNote;
use crate::domain::models::journal_note::{NewJournalNote, UpdateJournalNote};
use crate::domain::repository::JournalNoteRepository;
use crate::infrastructure::database::database::connect_from_pool;
use crate::infrastructure::database::models::journal_note::{
    JournalNoteRow, NewJournalNoteRow, UpdateJournalNoteRow,
};
use crate::infrastructure::database::models::schema::journal_notes;

#[derive(QueryableByName)]
struct LastInsertRow {
    #[diesel(sql_type = Integer)]
    journal_note_id: i32,
}

pub struct JournalNoteRepoImpl;

impl JournalNoteRepoImpl {
    pub fn new() -> Self {
        Self
    }
}

impl Default for JournalNoteRepoImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl JournalNoteRepository for JournalNoteRepoImpl {
    async fn find_by_book(&self, find_book_id: i32) -> Result<Vec<JournalNote>, DomainError> {
        let mut conn = connect_from_pool().await?;

        let rows = journal_notes::dsl::journal_notes
            .filter(journal_notes::book_id.eq(find_book_id))
            .load::<JournalNoteRow>(&mut conn)
            .await?;
        Ok(rows.into_iter().map(JournalNote::from).collect())
    }

    async fn find_by_id(&self, find_id: i32) -> Result<Option<JournalNote>, DomainError> {
        let mut conn = connect_from_pool().await?;

        let rows = journal_notes::dsl::journal_notes
            .filter(journal_notes::journal_note_id.eq(find_id))
            .limit(1)
            .load::<JournalNoteRow>(&mut conn)
            .await?;
        match rows.into_iter().next() {
            Some(row) => Ok(Some(JournalNote::from(row))),
            None => Ok(None),
        }
    }

    async fn insert(&self, new_note: NewJournalNote) -> Result<i32, DomainError> {
        let mut conn = connect_from_pool().await?;

        let id = conn
            .transaction(async |connection| {
                diesel::insert_into(journal_notes::table)
                    .values(&NewJournalNoteRow {
                        book_id: new_note.book_id,
                        title: new_note.title.as_deref(),
                        body: &new_note.body,
                        chapter_title: new_note.chapter_title.as_deref(),
                        page_number: new_note.page_number,
                    })
                    .execute(connection)
                    .await?;

                let result = sql_query("SELECT last_insert_rowid() as journal_note_id")
                    .get_result::<LastInsertRow>(connection)
                    .await?;
                Ok::<i32, diesel::result::Error>(result.journal_note_id)
            })
            .await?;

        Ok(id)
    }

    async fn update(&self, find_id: i32, update: UpdateJournalNote) -> Result<(), DomainError> {
        let mut conn = connect_from_pool().await?;
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        conn.transaction(async |connection| {
            diesel::update(
                journal_notes::dsl::journal_notes
                    .filter(journal_notes::journal_note_id.eq(find_id)),
            )
            .set(&UpdateJournalNoteRow {
                title: update.title.as_deref(),
                body: update.body.as_deref(),
                updated_at: Some(&now),
            })
            .execute(connection)
            .await?;
            Ok::<(), diesel::result::Error>(())
        })
        .await?;

        Ok(())
    }

    async fn delete(&self, find_id: i32) -> Result<(), DomainError> {
        let mut conn = connect_from_pool().await?;

        conn.transaction(async |connection| {
            diesel::delete(
                journal_notes::dsl::journal_notes
                    .filter(journal_notes::journal_note_id.eq(find_id)),
            )
            .execute(connection)
            .await?;
            Ok::<(), diesel::result::Error>(())
        })
        .await?;

        Ok(())
    }
}
