use anyhow::{ Error, Ok };
use chrono::Utc;
use crate::{
  context::state::AppState,
  types::{ auths::{ GithubUserInfo, LogoutRequest }, users::SaveUserRequest },
};

use super::users::UserHandler;

pub const LOGOUT_BLACKLIST_PREFIX: &'static str = "logout:blacklist:";

pub struct AuthHandler<'a> {
  state: &'a AppState,
}

impl<'a> AuthHandler<'a> {
  pub fn new(state: &'a AppState) -> Self {
    Self { state }
  }

  pub async fn handle_auth_github(&self, user_info: GithubUserInfo) -> Result<i64, Error> {
    let github_user_id = user_info.id.expect("github user_id is None");
    let github_user_name = user_info.login.expect("github user_name is None");

    let handler = UserHandler::new(self.state);

    // 1. Get user by github_user_id
    let user = handler.get(None, Some(github_user_id.to_string()), None).await.unwrap();

    // 2. If user exists, update user github subject ID.
    let save_param;
    if user.is_some() {
      save_param = SaveUserRequest {
        id: user.unwrap().base.id,
        name: Some(github_user_name.to_string()),
        email: None,
        phone: None,
        password: None,
        oidc_claims_sub: None,
        oidc_claims_name: None,
        github_claims_sub: None,
        github_claims_name: None,
        google_claims_sub: None,
        google_claims_name: None,
      };
    } else {
      // 3. If user not exists, create user by github login, which auto register user.
      save_param = SaveUserRequest {
        id: None,
        name: Some(github_user_name.to_string()),
        email: None,
        phone: None,
        password: None,
        oidc_claims_sub: None,
        oidc_claims_name: None,
        github_claims_sub: Some(github_user_id.to_string()),
        github_claims_name: Some(github_user_name.to_string()),
        google_claims_sub: None,
        google_claims_name: None,
      };
    }

    handler.save(save_param).await
  }

  pub async fn handle_logout(&self, param: LogoutRequest) -> Result<(), Error> {
    let cache = self.state.string_cache.cache(&self.state.config);

    // Add current jwt token to cache blacklist, expiration time is less than now time - id_token issue time.
    let ak = param.access_token.expect("access_token is None");
    let key = format!("{}:{}", LOGOUT_BLACKLIST_PREFIX, ak);
    let value = Utc::now().timestamp_millis().to_string();
    match cache.set(key, value, Some(3600_000)).await {
      std::result::Result::Ok(_) => {
        tracing::info!("Logout success for {}", ak);
        Ok(())
      }
      Err(e) => {
        tracing::error!("Logout failed: {}, cause: {}", ak, e);
        Err(e)
      }
    }
  }
}
