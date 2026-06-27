use async_trait::async_trait;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Integer;
use diesel_async::{AsyncConnection, RunQueryDsl};

use crate::domain::error::DomainError;
use crate::domain::models::book::Book;
use crate::domain::models::collection::Collection;
use crate::domain::models::collection::{NewCollection, UpdateCollection};
use crate::domain::repository::CollectionRepository;
use crate::infrastructure::database::database::connect_from_pool;
use crate::infrastructure::database::models::book::BookRow;
use crate::infrastructure::database::models::collection::{
    CollectionRow, NewCollectionBookRow, NewCollectionRow, UpdateCollectionRow,
};
use crate::infrastructure::database::models::schema::{books, collection_books, collections};

#[derive(QueryableByName)]
struct LastInsertRow {
    #[diesel(sql_type = Integer)]
    collection_id: i32,
}

pub struct CollectionRepoImpl;

impl CollectionRepoImpl {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CollectionRepoImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CollectionRepository for CollectionRepoImpl {
    async fn find_all(&self) -> Result<Vec<Collection>, DomainError> {
        let mut conn = connect_from_pool().await?;

        let rows = collections::dsl::collections
            .load::<CollectionRow>(&mut conn)
            .await?;
        Ok(rows.into_iter().map(Collection::from).collect())
    }

    async fn find_by_id(&self, find_id: i32) -> Result<Option<Collection>, DomainError> {
        let mut conn = connect_from_pool().await?;

        let rows = collections::dsl::collections
            .filter(collections::collection_id.eq(find_id))
            .limit(1)
            .load::<CollectionRow>(&mut conn)
            .await?;
        match rows.into_iter().next() {
            Some(row) => Ok(Some(Collection::from(row))),
            None => Ok(None),
        }
    }

    async fn create(&self, new_collection: NewCollection) -> Result<i32, DomainError> {
        let mut conn = connect_from_pool().await?;

        let id = conn
            .transaction(async |connection| {
                diesel::insert_into(collections::table)
                    .values(&NewCollectionRow {
                        name: &new_collection.name,
                        description: new_collection.description.as_deref(),
                        accent_color: new_collection.accent_color.as_deref(),
                    })
                    .execute(connection)
                    .await?;

                let result = sql_query("SELECT last_insert_rowid() as collection_id")
                    .get_result::<LastInsertRow>(connection)
                    .await?;
                Ok::<i32, diesel::result::Error>(result.collection_id)
            })
            .await?;

        Ok(id)
    }

    async fn update(&self, find_id: i32, update: UpdateCollection) -> Result<(), DomainError> {
        let mut conn = connect_from_pool().await?;

        conn.transaction(async |connection| {
            diesel::update(
                collections::dsl::collections.filter(collections::collection_id.eq(find_id)),
            )
            .set(&UpdateCollectionRow {
                name: update.name.as_deref(),
                description: update.description.as_deref(),
                accent_color: update.accent_color.as_deref(),
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
                collections::dsl::collections.filter(collections::collection_id.eq(find_id)),
            )
            .execute(connection)
            .await?;
            Ok::<(), diesel::result::Error>(())
        })
        .await?;

        Ok(())
    }

    async fn add_book(
        &self,
        find_collection_id: i32,
        find_book_id: i32,
    ) -> Result<(), DomainError> {
        let mut conn = connect_from_pool().await?;

        conn.transaction(async |connection| {
            diesel::insert_into(collection_books::table)
                .values(&NewCollectionBookRow {
                    collection_id: find_collection_id,
                    book_id: find_book_id,
                })
                .on_conflict_do_nothing()
                .execute(connection)
                .await?;
            Ok::<(), diesel::result::Error>(())
        })
        .await?;

        Ok(())
    }

    async fn remove_book(
        &self,
        find_collection_id: i32,
        find_book_id: i32,
    ) -> Result<(), DomainError> {
        let mut conn = connect_from_pool().await?;

        conn.transaction(async |connection| {
            diesel::delete(
                collection_books::dsl::collection_books
                    .filter(collection_books::collection_id.eq(find_collection_id))
                    .filter(collection_books::book_id.eq(find_book_id)),
            )
            .execute(connection)
            .await?;
            Ok::<(), diesel::result::Error>(())
        })
        .await?;

        Ok(())
    }

    async fn get_books(&self, find_collection_id: i32) -> Result<Vec<Book>, DomainError> {
        let mut conn = connect_from_pool().await?;

        let rows = collection_books::dsl::collection_books
            .inner_join(books::dsl::books)
            .filter(collection_books::collection_id.eq(find_collection_id))
            .order(collection_books::added_at.asc())
            .select(books::all_columns)
            .load::<BookRow>(&mut conn)
            .await?;
        Ok(rows.into_iter().map(Book::from).collect())
    }

    async fn get_collections_for_book(
        &self,
        find_book_id: i32,
    ) -> Result<Vec<Collection>, DomainError> {
        let mut conn = connect_from_pool().await?;

        let rows = collection_books::dsl::collection_books
            .inner_join(collections::dsl::collections)
            .filter(collection_books::book_id.eq(find_book_id))
            .select(collections::all_columns)
            .load::<CollectionRow>(&mut conn)
            .await?;
        Ok(rows.into_iter().map(Collection::from).collect())
    }
}
