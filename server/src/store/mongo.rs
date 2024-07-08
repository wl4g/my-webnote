use anyhow::Error;
use std::any::Any;
use std::marker::PhantomData;
use axum::async_trait;

use super::AsyncRepository;

pub struct MongoRepository<T: Any + Send + Sync> {
  phantom: PhantomData<T>,
}

impl<T: Any + Send + Sync> MongoRepository<T> {
  pub fn new() -> Self {
    MongoRepository { phantom: PhantomData }
  }
}

#[allow(unused)]
#[async_trait]
impl<T: Any + Send + Sync> AsyncRepository<T> for MongoRepository<T> {
  async fn select_all(&self) -> Result<Vec<T>, Error> {
    unimplemented!("select not implemented for MongoRepository")
  }

  async fn select_by_id(&self, id: i64) -> Result<T, Error> {
    unimplemented!("select_by_id not implemented for MongoRepository")
  }

  async fn insert(&self, param: T) -> Result<i64, Error> {
    unimplemented!("insert not implemented for MongoRepository")
  }

  async fn update(&self, param: T) -> Result<i64, Error> {
    unimplemented!("update not implemented for MongoRepository")
  }

  async fn delete_all(&self) -> Result<u64, Error> {
    unimplemented!("delete_all not implemented for MongoRepository")
  }

  async fn delete_by_id(&self, id: i64) -> Result<u64, Error> {
    unimplemented!("delete_by_id not implemented for MongoRepository")
  }
}
