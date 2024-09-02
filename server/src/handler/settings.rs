use std::sync::Arc;

use anyhow::{ Error, Ok };
use axum::async_trait;
use crate::context::state::AppState;
use crate::types::settings::{
    DeleteSettingsRequest,
    QuerySettingsRequest,
    SaveSettingsRequest,
    Settings,
};
use crate::types::{ PageRequest, PageResponse };

#[async_trait]
pub trait ISettingsHandler: Send {
    async fn get(&self, name: Option<String>) -> Result<Option<Arc<Settings>>, Error>;

    async fn find(
        &self,
        param: QuerySettingsRequest,
        page: PageRequest
    ) -> Result<(PageResponse, Vec<Settings>), Error>;

    async fn save(&self, param: SaveSettingsRequest) -> Result<i64, Error>;

    async fn delete(&self, param: DeleteSettingsRequest) -> Result<u64, Error>;
}

pub struct SettingsHandler<'a> {
    state: &'a AppState,
}

impl<'a> SettingsHandler<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

#[async_trait]
impl<'a> ISettingsHandler for SettingsHandler<'a> {
    async fn get(&self, name: Option<String>) -> Result<Option<Arc<Settings>>, Error> {
        let param = QuerySettingsRequest {
            name,
        };
        let res = self.find(param, PageRequest::default()).await.unwrap().1;
        if res.len() > 0 {
            let settings = Arc::new(res.get(0).unwrap().clone());
            return Ok(Some(settings));
        } else {
            Ok(None)
        }
    }

    async fn find(
        &self,
        param: QuerySettingsRequest,
        page: PageRequest
    ) -> Result<(PageResponse, Vec<Settings>), Error> {
        let repo = self.state.settings_repo.lock().await;
        repo.get(&self.state.config).select(param.to_settings(), page).await
    }

    async fn save(&self, param: SaveSettingsRequest) -> Result<i64, Error> {
        let repo = self.state.settings_repo.lock().await;
        if param.id.is_some() {
            repo.get(&self.state.config).update(param.to_settings()).await
        } else {
            repo.get(&self.state.config).insert(param.to_settings()).await
        }
    }

    async fn delete(&self, param: DeleteSettingsRequest) -> Result<u64, Error> {
        let repo = self.state.settings_repo.lock().await;
        repo.get(&self.state.config).delete_by_id(param.id).await
    }
}
