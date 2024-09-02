use std::sync::Arc;

use anyhow::{ Error, Ok };
use axum::async_trait;
use crate::context::state::AppState;
use crate::types::document::{
    DeleteDocumentRequest,
    QueryDocumentRequest,
    SaveDocumentRequest,
    Document,
};
use crate::types::{ PageRequest, PageResponse };

#[async_trait]
pub trait IDocumentHandler: Send {
    async fn get(&self, name: Option<String>) -> Result<Option<Arc<Document>>, Error>;

    async fn find(
        &self,
        param: QueryDocumentRequest,
        page: PageRequest
    ) -> Result<(PageResponse, Vec<Document>), Error>;

    async fn save(&self, param: SaveDocumentRequest) -> Result<i64, Error>;

    async fn delete(&self, param: DeleteDocumentRequest) -> Result<u64, Error>;
}

pub struct DocumentHandler<'a> {
    state: &'a AppState,
}

impl<'a> DocumentHandler<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

#[async_trait]
impl<'a> IDocumentHandler for DocumentHandler<'a> {
    async fn get(&self, name: Option<String>) -> Result<Option<Arc<Document>>, Error> {
        let param = QueryDocumentRequest {
            key: None,
            name,
            folder_key: None,
            doc_type: None,
        };
        let res = self.find(param, PageRequest::default()).await.unwrap().1;
        if res.len() > 0 {
            let document = Arc::new(res.get(0).unwrap().clone());
            return Ok(Some(document));
        } else {
            Ok(None)
        }
    }

    async fn find(
        &self,
        param: QueryDocumentRequest,
        page: PageRequest
    ) -> Result<(PageResponse, Vec<Document>), Error> {
        let repo = self.state.document_repo.lock().await;
        repo.get(&self.state.config).select(param.to_document(), page).await
    }

    async fn save(&self, param: SaveDocumentRequest) -> Result<i64, Error> {
        let repo = self.state.document_repo.lock().await;
        if param.id.is_some() {
            repo.get(&self.state.config).update(param.to_document()).await
        } else {
            repo.get(&self.state.config).insert(param.to_document()).await
        }
    }

    async fn delete(&self, param: DeleteDocumentRequest) -> Result<u64, Error> {
        let repo = self.state.document_repo.lock().await;
        repo.get(&self.state.config).delete_by_id(param.id).await
    }
}
