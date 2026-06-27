use crate::infrastructure::database::models::schema::{collection_books, collections};
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Selectable, PartialEq, Debug)]
#[diesel(table_name = collections)]
#[diesel(primary_key(collection_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CollectionRow {
    pub collection_id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub accent_color: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = collections)]
pub struct NewCollectionRow<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub accent_color: Option<&'a str>,
}

#[derive(AsChangeset, PartialEq, Debug)]
#[diesel(table_name = collections)]
pub struct UpdateCollectionRow<'a> {
    pub name: Option<&'a str>,
    pub description: Option<&'a str>,
    pub accent_color: Option<&'a str>,
}

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = collection_books)]
pub struct NewCollectionBookRow {
    pub collection_id: i32,
    pub book_id: i32,
}

#[derive(Queryable, Selectable, PartialEq, Debug)]
#[diesel(table_name = collection_books)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CollectionBookRow {
    pub collection_id: i32,
    pub book_id: i32,
    pub added_at: Option<String>,
}

impl From<CollectionRow> for crate::domain::models::collection::Collection {
    fn from(row: CollectionRow) -> Self {
        crate::domain::models::collection::Collection {
            id: row.collection_id.unwrap_or(0),
            name: row.name,
            description: row.description,
            accent_color: row.accent_color,
            created_at: row.created_at,
        }
    }
}

impl From<CollectionBookRow> for crate::domain::models::collection::CollectionBook {
    fn from(row: CollectionBookRow) -> Self {
        crate::domain::models::collection::CollectionBook {
            collection_id: row.collection_id,
            book_id: row.book_id,
            added_at: row.added_at,
        }
    }
}
