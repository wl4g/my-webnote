use lazy_static::lazy_static;
use anyhow::{ Error, Ok };
use chrono::Utc;
use openidconnect::{ core::CoreUserInfoClaims, LanguageTag };
use crate::{
  context::state::AppState,
  types::{ auths::{ GithubUserInfo, LogoutRequest }, users::SaveUserRequest },
};

use super::users::UserHandler;

pub const AUTH_NONCE_PREFIX: &'static str = "auth:nonce:";
pub const AUTH_LOGOUT_BLACKLIST_PREFIX: &'static str = "auth:logout:blacklist:";

lazy_static! {
  pub static ref LANG_CLAIMS_NAME_KEY: LanguageTag = LanguageTag::new("name".to_owned());
}

pub struct AuthHandler<'a> {
  state: &'a AppState,
}

impl<'a> AuthHandler<'a> {
  pub fn new(state: &'a AppState) -> Self {
    Self { state }
  }

  pub async fn handle_auth_create_nonce(&self, sid: &str, nonce: String) -> Result<(), Error> {
    let cache = self.state.string_cache.cache(&self.state.config);

    let key = Self::build_logout_blacklist_key(sid);
    let value = nonce;

    // TODO: using expires config? To ensure safety, expire as soon as possible. 10s
    match cache.set(key, value, Some(10_000)).await {
      std::result::Result::Ok(_) => {
        tracing::info!("Created auth nonce for {}", sid);
        Ok(())
      }
      Err(e) => {
        tracing::error!("Created auth nonce failed for {}, cause: {}", sid, e);
        Err(e)
      }
    }
  }

  pub async fn handle_auth_get_nonce(&self, sid: &str) -> Result<Option<String>, Error> {
    let cache = self.state.string_cache.cache(&self.state.config);

    let key = Self::build_logout_blacklist_key(sid);

    match cache.get(key).await {
      std::result::Result::Ok(nonce) => {
        tracing::info!("Got auth nonce for {}", sid);
        Ok(nonce)
      }
      Err(e) => {
        tracing::error!("Get auth nonce failed for {}, cause: {}", sid, e);
        Err(e)
      }
    }
  }

  pub async fn handle_auth_callback_oidc(
    &self,
    userinfo: CoreUserInfoClaims
  ) -> Result<i64, Error> {
    let oidc_user_id = userinfo.subject().as_str();
    let oidc_user_name = userinfo.name().map(|n|
      n
        .get(Some(&LANG_CLAIMS_NAME_KEY))
        .map(|u| u.to_string())
        .unwrap_or_default()
    );

    let handler = UserHandler::new(self.state);

    // 1. Get user by oidc user_id
    let user = handler.get(Some(oidc_user_id.to_string()), None, None).await.unwrap();

    // 2. If user exists, update user github subject ID.
    let save_param;
    if user.is_some() {
      save_param = SaveUserRequest {
        id: user.unwrap().base.id,
        name: oidc_user_name,
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
        name: oidc_user_name.clone(),
        email: None,
        phone: None,
        password: None,
        oidc_claims_sub: Some(oidc_user_id.to_string()),
        oidc_claims_name: oidc_user_name,
        github_claims_sub: None,
        github_claims_name: None,
        google_claims_sub: None,
        google_claims_name: None,
      };
    }

    handler.save(save_param).await
  }

  pub async fn handle_auth_callback_github(&self, userinfo: GithubUserInfo) -> Result<i64, Error> {
    let github_user_id = userinfo.id.expect("github user_id is None");
    let github_user_name = userinfo.login.expect("github user_name is None");

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
    let key = Self::build_logout_blacklist_key(ak.as_str());
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

  pub fn build_auth_nonce_key(nonce: &str) -> String {
    format!("{}:{}", AUTH_NONCE_PREFIX, nonce)
  }

  pub fn build_logout_blacklist_key(access_token: &str) -> String {
    format!("{}:{}", AUTH_LOGOUT_BLACKLIST_PREFIX, access_token)
  }
}
