use crate::models::settings::Settings;
use super::Repository;
use super::mongo::MongoRepository;
use anyhow::Error;

pub struct SettingsMongoRepository {
  inner: MongoRepository<Settings>,
}

impl SettingsMongoRepository {
  pub fn new() -> Self {
    SettingsMongoRepository { inner: MongoRepository::new() }
  }
}

impl Repository<Settings> for SettingsMongoRepository {
  fn select_all(&self) -> Result<Vec<Settings>, Error> {
    todo!()
  }

  fn select_by_id(&self, id: i32) -> Result<Settings, Error> {
    todo!()
  }

  fn insert(&self, param: Settings) -> Result<Settings, Error> {
    todo!()
  }

  fn update(&self, param: Settings) -> Result<Settings, Error> {
    todo!()
  }

  fn delete_all(&self, id: i32) -> Result<i32, Error> {
    todo!()
  }

  fn delete_by_id(&self, id: i32) -> Result<i32, Error> {
    todo!()
  }
}
