use crate::models::users::User;
use super::Repository;
use super::sqlite::SQLiteRepository;
use anyhow::Error;

pub struct UserSQLiteRepository {
  inner: SQLiteRepository<User>,
}

impl UserSQLiteRepository {
  pub fn new() -> Self {
    UserSQLiteRepository { inner: SQLiteRepository::new() }
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
