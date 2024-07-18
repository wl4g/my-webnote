use std::sync::Arc;

use anyhow::{ Error, Ok };
use axum::async_trait;
use crate::context::state::AppState;
use crate::types::users::{ DeleteUserRequest, QueryUserRequest, SaveUserRequest, User };
use crate::types::{ PageRequest, PageResponse };

#[async_trait]
pub trait IUserHandler: Send {
    async fn get(
        &self,
        oidc_claims_sub: Option<String>,
        github_claims_sub: Option<String>,
        google_claims_sub: Option<String>
    ) -> Result<Option<Arc<User>>, Error>;

    async fn find(
        &self,
        param: QueryUserRequest,
        page: PageRequest
    ) -> Result<(PageResponse, Vec<User>), Error>;

    async fn save(&self, param: SaveUserRequest) -> Result<i64, Error>;

    async fn delete(&self, param: DeleteUserRequest) -> Result<u64, Error>;
}

pub struct UserHandler<'a> {
    state: &'a AppState,
}

impl<'a> UserHandler<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

#[async_trait]
impl<'a> IUserHandler for UserHandler<'a> {
    async fn get(
        &self,
        oidc_claims_sub: Option<String>,
        github_claims_sub: Option<String>,
        google_claims_sub: Option<String>
    ) -> Result<Option<Arc<User>>, Error> {
        let param = QueryUserRequest {
            name: None,
            email: None,
            phone: None,
            oidc_claims_sub,
            oidc_claims_name: None,
            oidc_claims_email: None,
            github_claims_sub,
            github_claims_name: None,
            github_claims_email: None,
            google_claims_sub,
            google_claims_name: None,
            google_claims_email: None,
        };
        let res = self.find(param, PageRequest::default()).await.unwrap().1;
        if res.len() > 0 {
            // Notice: Fuck, I don't know why?
            // 如果这里使用 Rc 包装而不是 Arc，那么在 routes/auths.rs中 fn init() { Router::new().route("/auth/callback/github", get(callback_github)) } 会报错泛型参数匹配错误.
            let user = Arc::new(res.get(0).unwrap().clone());
            return Ok(Some(user));
        } else {
            Ok(None)
        }
    }

    async fn find(
        &self,
        param: QueryUserRequest,
        page: PageRequest
    ) -> Result<(PageResponse, Vec<User>), Error> {
        let repo = self.state.user_repo.lock().await;
        repo.repo(&self.state.config).select(param.to_user(), page).await
    }

    //#[common_log_macro::biz_log("创建/更新了用户信息: id: {param.base.id}, name: {param.name}")]
    async fn save(&self, param: SaveUserRequest) -> Result<i64, Error> {
        let repo = self.state.user_repo.lock().await;
        if param.id.is_some() {
            repo.repo(&self.state.config).update(param.to_user()).await
        } else {
            repo.repo(&self.state.config).insert(param.to_user()).await
        }
    }

    async fn delete(&self, param: DeleteUserRequest) -> Result<u64, Error> {
        let repo = self.state.user_repo.lock().await;
        repo.repo(&self.state.config).delete_by_id(param.id).await
    }
}
