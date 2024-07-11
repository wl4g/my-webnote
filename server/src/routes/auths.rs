use axum::{
  extract::{ Query, State },
  response::{ IntoResponse, Redirect },
  routing::{ get, post },
  Router,
};
use axum::response::Result;
use axum::http::StatusCode;

use oauth2::{ AuthorizationCode, Scope };
use tower_cookies::{
  cookie::{ time::{ self, macros::time }, CookieBuilder },
  Cookie,
  CookieManagerLayer,
  Cookies,
};

use crate::{
  context::state::AppState,
  handlers::auths::AuthHandler,
  types::auths::CallbackGithubRequest,
};

pub fn init() -> Router<AppState> {
  Router::new()
    .route("/auth/login/oidc", get(login_oidc))
    .route("/auth/login/github", get(login_github))
    .route("/auth/callback/github", get(callback_github))
    .route("/auth/callback/oidc", get(callback_oidc))
    .route("/auth/logout", post(logout))
    .layer(CookieManagerLayer::new())
}

#[utoipa::path(
  get,
  path = "/auth/login/oidc",
  responses((status = 200, description = "Login for OIDC.")),
  tag = ""
)]
pub async fn login_oidc(
  State(state): State<AppState>,
  Query(param): Query<CallbackGithubRequest>
) -> impl IntoResponse {
  match &state.oidc_client {
    Some(client) => {
      let (auth_url, _) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .url();
      Ok(Redirect::to(auth_url.as_str()))
    }
    None => Err(StatusCode::INTERNAL_SERVER_ERROR),
  }
}

#[utoipa::path(
  get,
  path = "/auth/login/github",
  responses((status = 200, description = "Login for Github.")),
  tag = ""
)]
pub async fn login_github(
  State(state): State<AppState>,
  Query(param): Query<CallbackGithubRequest>
) -> impl IntoResponse {
  match &state.github_client {
    Some(client) => {
      let (auth_url, _) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .url();
      Ok(Redirect::to(auth_url.as_str()))
    }
    None => Err(StatusCode::INTERNAL_SERVER_ERROR),
  }
}

#[utoipa::path(
  get,
  path = "/auth/callback/oidc",
  responses((status = 200, description = "Callback for OIDC.")),
  tag = ""
)]
pub async fn callback_oidc(
  State(state): State<AppState>,
  Query(param): Query<CallbackGithubRequest>
) -> impl IntoResponse {
  match &state.oidc_client {
    Some(client) => {
      let token_result = client
        .exchange_code(AuthorizationCode::new(param.code.unwrap()))
        .request_async(oauth2::reqwest::async_http_client).await;

      match token_result {
        Ok(token) => {
          // 处理成功获取的token
          // 例如，可以将token存储在session中，然后重定向到主页
          // 这里只是一个示例，你可能需要根据你的需求进行调整
          Redirect::to("/").into_response()
        }
        Err(e) => {
          // 处理token交换失败的情况
          (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to exchange token: {}", e),
          ).into_response()
        }
      }
    }
    None =>
      (StatusCode::INTERNAL_SERVER_ERROR, "OIDC client not configured".to_string()).into_response(),
  }
}

#[utoipa::path(
  get,
  path = "/auth/callback/github",
  responses((status = 200, description = "Callback for github.")),
  tag = ""
)]
pub async fn callback_github(
  State(state): State<AppState>,
  Query(param): Query<CallbackGithubRequest>
) -> impl IntoResponse {
  match &state.oidc_client {
    Some(client) => {
      let token_result = client
        .exchange_code(AuthorizationCode::new(param.code.unwrap()))
        .request_async(oauth2::reqwest::async_http_client).await;

      match token_result {
        Ok(token) => {
          // 处理成功获取的token
          // 例如，可以将token存储在session中，然后重定向到主页
          // 这里只是一个示例，你可能需要根据你的需求进行调整
          Redirect::to("/").into_response()
        }
        Err(e) => {
          // 处理token交换失败的情况
          (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to exchange token: {}", e),
          ).into_response()
        }
      }
    }
    None =>
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Github client not configured".to_string(),
      ).into_response(),
  }
}

#[utoipa::path(
  post,
  path = "/auth/login/logout",
  responses((status = 200, description = "Logout.")),
  tag = ""
)]
pub async fn logout(State(state): State<AppState>, cookies: Cookies) -> impl IntoResponse {
  // TODO using config '_revezone_sid'
  let cookie = CookieBuilder::new("_REVEZONE_SID", "bar")
    .max_age(time::Duration::ZERO)
    .path("/")
    .build();
  cookies.remove(cookie);
  Redirect::to("/")
}
