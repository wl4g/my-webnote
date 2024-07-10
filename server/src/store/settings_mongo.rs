// use crate::models::settings::Settings;
// use super::Repository;
// use super::mongo::MongoRepository;
// use anyhow::Error;

// pub struct SettingsMongoRepository {
//   inner: MongoRepository<Settings>,
// }

// impl SettingsMongoRepository {
//   pub fn new() -> Self {
//     SettingsMongoRepository { inner: MongoRepository::new() }
//   }
// }

// impl Repository<Settings> for SettingsMongoRepository {
//   fn select(&self) -> Result<Vec<Settings>, Error> {
//     todo!()
//   }

//   fn select_by_id(&self, id: i64) -> Result<Settings, Error> {
//     todo!()
//   }

//   fn insert(&self, param: Settings) -> Result<i64, Error> {
//     todo!()
//   }

//   fn update(&self, param: Settings) -> Result<u64, Error> {
//     todo!()
//   }

//   fn delete_all(&self) -> Result<u64, Error> {
//     todo!()
//   }

//   fn delete_by_id(&self, id: i64) -> Result<u64, Error> {
//     todo!()
//   }
// }
