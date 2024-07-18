use std::sync::Arc;

use anyhow::{ Error, Ok };
use axum::async_trait;
use crate::context::state::AppState;
use crate::types::api_v1::users::{
    DeleteUserApiV1Request,
    QueryUserApiV1Request,
    SaveUserApiV1Request,
};
use crate::types::users::User;
use crate::types::{ PageRequest, PageResponse };

#[async_trait]
pub trait IApiV1Handler: Send {
    async fn get(
        &self,
        oidc_claims_sub: Option<String>,
        github_claims_sub: Option<String>,
        google_claims_sub: Option<String>
    ) -> Result<Option<Arc<User>>, Error>;

    async fn find(
        &self,
        param: QueryUserApiV1Request,
        page: PageRequest
    ) -> Result<(PageResponse, Vec<User>), Error>;

    async fn save(&self, param: SaveUserApiV1Request) -> Result<i64, Error>;

    async fn delete(&self, param: DeleteUserApiV1Request) -> Result<u64, Error>;
}

pub struct ApiV1Handler<'a> {
    state: &'a AppState,
}

impl<'a> ApiV1Handler<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

#[async_trait]
impl<'a> IApiV1Handler for ApiV1Handler<'a> {
    async fn get(
        &self,
        oidc_claims_sub: Option<String>,
        github_claims_sub: Option<String>,
        google_claims_sub: Option<String>
    ) -> Result<Option<Arc<User>>, Error> {
        let param = QueryUserApiV1Request {
            name: None,
            email: None,
            phone: None,
            oidc_claims_sub: oidc_claims_sub,
            github_claims_sub: github_claims_sub,
            google_claims_sub: google_claims_sub,
        };
        let res = self.find(param, PageRequest::default()).await.unwrap().1;
        if res.len() > 0 {
            let user = Arc::new(res.get(0).unwrap().clone());
            return Ok(Some(user));
        } else {
            Ok(None)
        }
    }

    async fn find(
        &self,
        param: QueryUserApiV1Request,
        page: PageRequest
    ) -> Result<(PageResponse, Vec<User>), Error> {
        let repo = self.state.user_repo.lock().await;
        repo.repo(&self.state.config).select(param.to_user(), page).await
    }

    async fn save(&self, param: SaveUserApiV1Request) -> Result<i64, Error> {
        let repo = self.state.user_repo.lock().await;
        if param.id.is_some() {
            repo.repo(&self.state.config).update(param.to_user()).await
        } else {
            repo.repo(&self.state.config).insert(param.to_user()).await
        }
    }

    async fn delete(&self, param: DeleteUserApiV1Request) -> Result<u64, Error> {
        let repo = self.state.user_repo.lock().await;
        repo.repo(&self.state.config).delete_by_id(param.id).await
    }
}
