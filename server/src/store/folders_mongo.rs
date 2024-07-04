use crate::models::folders::Folder;
use super::Repository;
use super::mongo::MongoRepository;
use anyhow::Error;

pub struct FolderMongoRepository {
  inner: MongoRepository<Folder>,
}

impl FolderMongoRepository {
  pub fn new() -> Self {
    FolderMongoRepository { inner: MongoRepository::new() }
  }
}

impl Repository<Folder> for FolderMongoRepository {
  fn select_all(&self) -> Result<Vec<Folder>, Error> {
    todo!()
  }

  fn select_by_id(&self, id: i32) -> Result<Folder, Error> {
    todo!()
  }

  fn insert(&self, param: Folder) -> Result<Folder, Error> {
    todo!()
  }

  fn update(&self, param: Folder) -> Result<Folder, Error> {
    todo!()
  }

  fn delete_all(&self, id: i32) -> Result<i32, Error> {
    todo!()
  }

  fn delete_by_id(&self, id: i32) -> Result<i32, Error> {
    todo!()
  }
}
