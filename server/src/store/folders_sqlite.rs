// use crate::models::folders::Folder;
// use super::Repository;
// use super::sqlite::SQLiteRepository;
// use anyhow::Error;

// pub struct FolderSQLiteRepository {
//   inner: SQLiteRepository<Folder>,
// }

// impl FolderSQLiteRepository {
//   pub fn new() -> Self {
//     FolderSQLiteRepository { inner: SQLiteRepository::new() }
//   }
// }

// impl Repository<Folder> for FolderSQLiteRepository {
//   fn select(&self) -> Result<Vec<Folder>, Error> {
//     todo!()
//   }

//   fn select_by_id(&self, id: i32) -> Result<Folder, Error> {
//     todo!()
//   }

//   fn insert(&self, param: Folder) -> Result<Folder, Error> {
//     todo!()
//   }

//   fn update(&self, param: Folder) -> Result<Folder, Error> {
//     todo!()
//   }

//   fn delete_all(&self, id: i32) -> Result<i32, Error> {
//     todo!()
//   }

//   fn delete_by_id(&self, id: i32) -> Result<i32, Error> {
//     todo!()
//   }
// }
