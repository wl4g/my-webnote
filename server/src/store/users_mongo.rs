use crate::types::{ users::User, PageResponse, PageRequest };
use super::{ mongo::MongoRepository, AsyncRepository };
use anyhow::Error;
use axum::async_trait;

pub struct UserMongoRepository {
  inner: MongoRepository<User>,
}

impl UserMongoRepository {
  pub fn new() -> Self {
    UserMongoRepository { inner: MongoRepository::new() }
  }
}

#[allow(unused)]
#[async_trait]
impl AsyncRepository<User> for UserMongoRepository {
  async fn select(
    &self,
    mut param: User,
    page: PageRequest
  ) -> Result<(PageResponse, Vec<User>), Error> {
    todo!()
  }

  async fn select_by_id(&self, id: i64) -> Result<User, Error> {
    todo!()
  }

  async fn insert(&self, param: User) -> Result<i64, Error> {
    todo!()
  }

  async fn update(&self, param: User) -> Result<i64, Error> {
    todo!()
  }

  async fn delete_all(&self) -> Result<u64, Error> {
    todo!()
  }

  async fn delete_by_id(&self, id: i64) -> Result<u64, Error> {
    todo!()
  }
}
