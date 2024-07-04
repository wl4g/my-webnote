use super::Repository;
use anyhow::Error;
use std::any::Any;
use std::marker::PhantomData;

pub struct MongoRepository<T: Any + Send + Sync> {
  phantom: PhantomData<T>,
}

impl<T: Any + Send + Sync> MongoRepository<T> {
  pub fn new() -> Self {
    MongoRepository { phantom: PhantomData }
  }
}

impl<T: Any + Send + Sync> Repository<T> for MongoRepository<T> {
  fn select_all(&self) -> Result<Vec<T>, Error> {
    // MongoDB 通用查询逻辑
    unimplemented!("select not implemented for MongoRepository")
  }

  fn select_by_id(&self, id: i32) -> Result<T, Error> {
    // MongoDB 通用按 ID 查询逻辑
    unimplemented!("select_by_id not implemented for MongoRepository")
  }

  fn insert(&self, param: T) -> Result<T, Error> {
    // MongoDB 通用插入逻辑
    unimplemented!("insert not implemented for MongoRepository")
  }

  fn update(&self, param: T) -> Result<T, Error> {
    // MongoDB 通用更新逻辑
    unimplemented!("update not implemented for MongoRepository")
  }

  fn delete_all(&self, id: i32) -> Result<i32, Error> {
    // MongoDB 通用删除所有逻辑
    unimplemented!("delete_all not implemented for MongoRepository")
  }

  fn delete_by_id(&self, id: i32) -> Result<i32, Error> {
    // MongoDB 通用按 ID 删除逻辑
    unimplemented!("delete_by_id not implemented for MongoRepository")
  }
}
