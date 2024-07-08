use anyhow::{ Error, Ok };
use std::any::Any;
use std::marker::PhantomData;
use sqlx::SqlitePool;

use super::Repository;
use crate::config::config::DbConfig;

pub struct SQLiteRepository<T: Any + Send + Sync> {
  phantom: PhantomData<T>,
  pool: SqlitePool,
}

impl<T: Any + Send + Sync> SQLiteRepository<T> {
  pub async fn new(config: &DbConfig) -> Result<Self, Error> {
    let conn_url = format!("file:{}/sqlite.db", config.sqlite.dir);
    Ok(SQLiteRepository {
      phantom: PhantomData,
      pool: SqlitePool::connect(&conn_url).await?,
    })
  }
}

impl<T: Any + Send + Sync> Repository<T> for SQLiteRepository<T> {
  fn select_all(&self) -> Result<Vec<T>, Error> {
    // SQLite 通用查询逻辑
    unimplemented!("select not implemented for SQLiteRepository")
  }

  fn select_by_id(&self, id: i32) -> Result<T, Error> {
    // SQLite 通用按 ID 查询逻辑
    unimplemented!("select_by_id not implemented for SQLiteRepository")
  }

  fn insert(&self, param: T) -> Result<T, Error> {
    // SQLite 通用插入逻辑
    unimplemented!("insert not implemented for SQLiteRepository")
  }

  fn update(&self, param: T) -> Result<T, Error> {
    // SQLite 通用更新逻辑
    unimplemented!("update not implemented for SQLiteRepository")
  }

  fn delete_all(&self, id: i32) -> Result<i32, Error> {
    // SQLite 通用删除所有逻辑
    unimplemented!("delete_all not implemented for SQLiteRepository")
  }

  fn delete_by_id(&self, id: i32) -> Result<i32, Error> {
    // SQLite 通用按 ID 删除逻辑
    unimplemented!("delete_by_id not implemented for SQLiteRepository")
  }
}
