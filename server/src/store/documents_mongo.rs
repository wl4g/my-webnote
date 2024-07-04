use crate::models::documents::Document;
use super::Repository;
use super::mongo::MongoRepository;
use anyhow::Error;

pub struct DocumentMongoRepository {
  inner: MongoRepository<Document>,
}

impl DocumentMongoRepository {
  pub fn new() -> Self {
    DocumentMongoRepository { inner: MongoRepository::new() }
  }
}

impl Repository<Document> for DocumentMongoRepository {
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
