// use crate::models::documents::Document;
// use super::{mongo::MongoRepository, AsyncRepository};
// use anyhow::Error;
// use axum::async_trait;

// pub struct DocumentMongoRepository {
//   inner: MongoRepository<Document>,
// }

// impl DocumentMongoRepository {
//   pub fn new() -> Self {
//     DocumentMongoRepository { inner: MongoRepository::new() }
//   }
// }

// #[async_trait]
// impl AsyncRepository<Document> for DocumentMongoRepository {
//   fn select(&self) -> Result<Vec<Document>, Error> {
//     todo!()
//   }

//   fn select_by_id(&self, id: i64) -> Result<Document, Error> {
//     todo!()
//   }

//   fn insert(&self, param: Document) -> Result<i64, Error> {
//     todo!()
//   }

//   fn update(&self, param: Document) -> Result<u64, Error> {
//     todo!()
//   }

//   fn delete_all(&self) -> Result<u64, Error> {
//     todo!()
//   }

//   fn delete_by_id(&self, id: i64) -> Result<u64, Error> {
//     todo!()
//   }
// }
