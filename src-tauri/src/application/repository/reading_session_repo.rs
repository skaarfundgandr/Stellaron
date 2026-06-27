use async_trait::async_trait;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Integer;
use diesel_async::{AsyncConnection, RunQueryDsl};

use crate::domain::error::DomainError;
use crate::domain::models::reading_session::ReadingSession;
use crate::domain::models::reading_session::{EndReadingSession, NewReadingSession};
use crate::domain::repository::ReadingSessionRepository;
use crate::infrastructure::database::database::connect_from_pool;
use crate::infrastructure::database::models::reading_session::{
    NewReadingSessionRow, ReadingSessionRow,
};
use crate::infrastructure::database::models::schema::reading_sessions;

#[derive(QueryableByName)]
struct LastInsertRow {
    #[diesel(sql_type = Integer)]
    session_id: i32,
}

pub struct ReadingSessionRepoImpl;

impl ReadingSessionRepoImpl {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ReadingSessionRepoImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ReadingSessionRepository for ReadingSessionRepoImpl {
    async fn find_by_book(&self, find_book_id: i32) -> Result<Vec<ReadingSession>, DomainError> {
        let mut conn = connect_from_pool().await?;

        let rows = reading_sessions::dsl::reading_sessions
            .filter(reading_sessions::book_id.eq(find_book_id))
            .load::<ReadingSessionRow>(&mut conn)
            .await?;
        Ok(rows.into_iter().map(ReadingSession::from).collect())
    }

    async fn find_by_id(&self, find_id: i32) -> Result<Option<ReadingSession>, DomainError> {
        let mut conn = connect_from_pool().await?;

        let rows = reading_sessions::dsl::reading_sessions
            .filter(reading_sessions::session_id.eq(find_id))
            .limit(1)
            .load::<ReadingSessionRow>(&mut conn)
            .await?;
        match rows.into_iter().next() {
            Some(row) => Ok(Some(ReadingSession::from(row))),
            None => Ok(None),
        }
    }

    async fn find_all(&self) -> Result<Vec<ReadingSession>, DomainError> {
        let mut conn = connect_from_pool().await?;

        let rows = reading_sessions::dsl::reading_sessions
            .load::<ReadingSessionRow>(&mut conn)
            .await?;
        Ok(rows.into_iter().map(ReadingSession::from).collect())
    }

    async fn create(&self, new_session: NewReadingSession) -> Result<i32, DomainError> {
        let mut conn = connect_from_pool().await?;

        let id = conn
            .transaction(async |connection| {
                diesel::insert_into(reading_sessions::table)
                    .values(&NewReadingSessionRow {
                        book_id: new_session.book_id,
                        start_position: &new_session.start_position,
                        chapter_title: new_session.chapter_title.as_deref(),
                        page_number: new_session.page_number,
                        start_ts: &new_session.start_ts,
                    })
                    .execute(connection)
                    .await?;

                let result = sql_query("SELECT last_insert_rowid() as session_id")
                    .get_result::<LastInsertRow>(connection)
                    .await?;
                Ok::<i32, diesel::result::Error>(result.session_id)
            })
            .await?;

        Ok(id)
    }

    async fn finish(&self, find_id: i32, end: EndReadingSession) -> Result<(), DomainError> {
        let mut conn = connect_from_pool().await?;

        conn.transaction(async |connection| {
            diesel::update(
                reading_sessions::dsl::reading_sessions
                    .filter(reading_sessions::session_id.eq(find_id)),
            )
            .set((
                reading_sessions::end_position.eq(&end.end_position),
                reading_sessions::chapter_title.eq(&end.chapter_title),
                reading_sessions::page_number.eq(end.page_number),
                reading_sessions::end_ts.eq(&end.end_ts),
                reading_sessions::duration_seconds.eq(end.duration_seconds),
                reading_sessions::pages_read.eq(end.pages_read),
            ))
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
                reading_sessions::dsl::reading_sessions
                    .filter(reading_sessions::session_id.eq(find_id)),
            )
            .execute(connection)
            .await?;
            Ok::<(), diesel::result::Error>(())
        })
        .await?;

        Ok(())
    }
}
