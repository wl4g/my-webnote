// use anyhow::Error;
// use crate::context::state::AppState;
// use crate::models::folders::Folder;

// pub struct FolderHandler<'a> {
//   state: &'a AppState,
// }

// impl<'a> FolderHandler<'a> {
//   pub fn new(state: &'a AppState) -> Self {
//     Self { state }
//   }

//   pub async fn get_folders(&self) -> Result<Vec<Folder>, Error> {
//     let mut repo = self.state.folder_repo.lock().await;
//     repo.repo(&self.state.config).select_all()
//   }

//   pub async fn create_folder(&self, folder: Folder) -> Result<Folder, Error> {
//     let mut repo = self.state.folder_repo.lock().await;
//     repo.repo(&self.state.config).insert(folder)
//   }

//   // More functions ...
// }
