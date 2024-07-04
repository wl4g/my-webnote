pub mod mongo;
pub mod sqlite;
pub mod documents_mongo;
pub mod documents_sqlite;
pub mod folders_mongo;
pub mod folders_sqlite;
pub mod settings_sqlite;
pub mod settings_mongo;
pub mod users_sqlite;
pub mod users_mongo;

use anyhow::Error;

use crate::config::config::{ ApiConfig, DbType };

pub trait Repository<T>: Send {
  fn select_all(&self) -> Result<Vec<T>, Error> where T: 'static + Send + Sync;
  fn select_by_id(&self, id: i32) -> Result<T, Error> where T: 'static + Send + Sync;
  fn insert(&self, param: T) -> Result<T, Error> where T: 'static + Send + Sync;
  fn update(&self, param: T) -> Result<T, Error> where T: 'static + Send + Sync;
  fn delete_all(&self, id: i32) -> Result<i32, Error>;
  fn delete_by_id(&self, id: i32) -> Result<i32, Error>;
}

pub struct RepositoryContainer<T> where T: 'static + Send + Sync {
  sqlite_repo: Box<dyn Repository<T>>,
  mongo_repo: Box<dyn Repository<T>>,
}

impl<T> RepositoryContainer<T> where T: 'static + Send + Sync {
  pub fn new(sqlite_repo: Box<dyn Repository<T>>, mongo_repo: Box<dyn Repository<T>>) -> Self {
    RepositoryContainer {
      sqlite_repo,
      mongo_repo,
    }
  }

  fn sqlite_repo(&self) -> &dyn Repository<T> {
    &*self.sqlite_repo
  }

  fn mongo_repo(&self) -> &dyn Repository<T> {
    &*self.mongo_repo
  }

  pub fn repo(&mut self, config: &ApiConfig) -> &dyn Repository<T> {
    match config.service.db.db_type {
      DbType::Sqlite => self.sqlite_repo(),
      DbType::Mongo => self.mongo_repo(),
    }
  }
}
