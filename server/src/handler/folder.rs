use std::sync::Arc;

use anyhow::{ Error, Ok };
use axum::async_trait;
use crate::context::state::AppState;
use crate::types::folder::{ DeleteFolderRequest, QueryFolderRequest, SaveFolderRequest, Folder };
use crate::types::{ PageRequest, PageResponse };

#[async_trait]
pub trait IFolderHandler: Send {
    async fn get(&self, name: Option<String>) -> Result<Option<Arc<Folder>>, Error>;

    async fn find(
        &self,
        param: QueryFolderRequest,
        page: PageRequest
    ) -> Result<(PageResponse, Vec<Folder>), Error>;

    async fn save(&self, param: SaveFolderRequest) -> Result<i64, Error>;

    async fn delete(&self, param: DeleteFolderRequest) -> Result<u64, Error>;
}

pub struct FolderHandler<'a> {
    state: &'a AppState,
}

impl<'a> FolderHandler<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

#[async_trait]
impl<'a> IFolderHandler for FolderHandler<'a> {
    async fn get(&self, name: Option<String>) -> Result<Option<Arc<Folder>>, Error> {
        let param = QueryFolderRequest {
            pid: None,
            key: None,
            name,
        };
        let res = self.find(param, PageRequest::default()).await.unwrap().1;
        if res.len() > 0 {
            let folder = Arc::new(res.get(0).unwrap().clone());
            return Ok(Some(folder));
        } else {
            Ok(None)
        }
    }

    async fn find(
        &self,
        param: QueryFolderRequest,
        page: PageRequest
    ) -> Result<(PageResponse, Vec<Folder>), Error> {
        let repo = self.state.folder_repo.lock().await;
        repo.repo(&self.state.config).select(param.to_folder(), page).await
    }

    async fn save(&self, param: SaveFolderRequest) -> Result<i64, Error> {
        let repo = self.state.folder_repo.lock().await;
        if param.id.is_some() {
            repo.repo(&self.state.config).update(param.to_folder()).await
        } else {
            repo.repo(&self.state.config).insert(param.to_folder()).await
        }
    }

    async fn delete(&self, param: DeleteFolderRequest) -> Result<u64, Error> {
        let repo = self.state.folder_repo.lock().await;
        repo.repo(&self.state.config).delete_by_id(param.id).await
    }
}
