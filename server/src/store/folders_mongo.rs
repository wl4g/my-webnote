// use crate::models::folders::Folder;
// use super::Repository;
// use super::mongo::MongoRepository;
// use anyhow::Error;

// pub struct FolderMongoRepository {
//   inner: MongoRepository<Folder>,
// }

// impl FolderMongoRepository {
//   pub fn new() -> Self {
//     FolderMongoRepository { inner: MongoRepository::new() }
//   }
// }

// impl Repository<Folder> for FolderMongoRepository {
//   fn select_all(&self) -> Result<Vec<Folder>, Error> {
//     todo!()
//   }

//   fn select_by_id(&self, id: i64) -> Result<Folder, Error> {
//     todo!()
//   }

//   fn insert(&self, param: Folder) -> Result<i64, Error> {
//     todo!()
//   }

//   fn update(&self, param: Folder) -> Result<u64, Error> {
//     todo!()
//   }

//   fn delete_all(&self) -> Result<u64, Error> {
//     todo!()
//   }

//   fn delete_by_id(&self, id: i64) -> Result<u64, Error> {
//     todo!()
//   }
// }
