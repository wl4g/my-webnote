use anyhow::{ Error, Ok };

use crate::config::config::DbConfig;
use crate::models::users::User;
use super::Repository;
use super::sqlite::SQLiteRepository;

pub struct UserSQLiteRepository {
  inner: SQLiteRepository<User>,
}

impl UserSQLiteRepository {
  pub async fn new(config: &DbConfig) -> Result<Self, Error> {
    Ok(UserSQLiteRepository {
      inner: SQLiteRepository::new(config).await?,
    })
  }
}

impl Repository<User> for UserSQLiteRepository {
  fn select_all(&self) -> Result<Vec<User>, Error> {
    todo!()
  }

  fn select_by_id(&self, id: i32) -> Result<User, Error> {
    todo!()
  }

  fn insert(&self, param: User) -> Result<User, Error> {
    todo!()
  }

  fn update(&self, param: User) -> Result<User, Error> {
    todo!()
  }

  fn delete_all(&self, id: i32) -> Result<i32, Error> {
    todo!()
  }

  fn delete_by_id(&self, id: i32) -> Result<i32, Error> {
    todo!()
  }
}
