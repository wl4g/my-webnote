use crate::models::users::User;
use super::Repository;
use super::mongo::MongoRepository;
use anyhow::Error;

pub struct UserMongoRepository {
  inner: MongoRepository<User>,
}

impl UserMongoRepository {
  pub fn new() -> Self {
    UserMongoRepository { inner: MongoRepository::new() }
  }
}

impl Repository<User> for UserMongoRepository {
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
