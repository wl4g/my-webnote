// use anyhow::{Error, Ok};
// use crate::context::state::AppState;
// use crate::models::documents::Document;

// pub struct DocumentHandler<'a> {
//   state: &'a AppState,
// }

// impl<'a> DocumentHandler<'a> {
//   pub fn new(state: &'a AppState) -> Self {
//     Self { state }
//   }

//   pub async fn get_documents(&self) -> Result<Vec<Document>, Error> {
//     let mut repo = self.state.document_repo.lock().await;
//     repo.repo(&self.state.config).select()
//   }

//   pub async fn create_document(&self, document: Document) -> Result<Document, Error> {
//     let mut repo = self.state.document_repo.lock().await;
//     repo.repo(&self.state.config).insert(document)
//   }

//   // More functions ...
// }
