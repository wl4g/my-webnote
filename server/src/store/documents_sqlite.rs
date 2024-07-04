use crate::models::documents::Document;
use super::Repository;
use super::sqlite::SQLiteRepository;
use anyhow::Error;

pub struct DocumentSQLiteRepository {
  inner: SQLiteRepository<Document>,
}

impl DocumentSQLiteRepository {
  pub fn new() -> Self {
    DocumentSQLiteRepository { inner: SQLiteRepository::new() }
  }
}

impl Repository<Document> for DocumentSQLiteRepository {
  fn select_all(&self) -> Result<Vec<Document>, Error> {
    todo!()
  }

  fn select_by_id(&self, id: i32) -> Result<Document, Error> {
    todo!()
  }

  fn insert(&self, param: Document) -> Result<Document, Error> {
    todo!()
  }

  fn update(&self, param: Document) -> Result<Document, Error> {
    todo!()
  }

  fn delete_all(&self, id: i32) -> Result<i32, Error> {
    todo!()
  }

  fn delete_by_id(&self, id: i32) -> Result<i32, Error> {
    todo!()
  }
}
