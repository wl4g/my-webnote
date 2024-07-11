use axum::{
  extract::{ Query, State },
  response::{ IntoResponse, Redirect },
  routing::{ get, post },
  Router,
};
use axum::http::StatusCode;

use oauth2::{ AuthorizationCode, Scope, TokenResponse };
use tower_cookies::{ cookie::{ time::{ self }, CookieBuilder }, CookieManagerLayer, Cookies };

use crate::{
  context::state::AppState,
  handlers::auths::AuthHandler,
  types::auths::{ CallbackGithubRequest, GithubUserInfo, LogoutRequest },
};

pub fn init() -> Router<AppState> {
  Router::new()
    .route("/auth/login/oidc", get(login_oidc))
    .route("/auth/login/github", get(login_github))
    .route("/auth/callback/oidc", get(callback_oidc))
    .route("/auth/callback/github", get(callback_github))
    .route("/auth/logout", post(logout))
    .layer(CookieManagerLayer::new())
}

#[utoipa::path(
  get,
  path = "/auth/login/oidc",
  responses((status = 200, description = "Login for OIDC.")),
  tag = ""
)]
pub async fn login_oidc(State(state): State<AppState>) -> impl IntoResponse {
  match &state.oidc_client {
    Some(client) => {
      let (auth_url, _) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(Scope::new(state.config.server.auth.oidc.scope.clone().unwrap()))
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
pub async fn login_github(State(state): State<AppState>) -> impl IntoResponse {
  match &state.github_client {
    Some(client) => {
      let (auth_url, _) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(Scope::new(state.config.server.auth.github.scope.clone().unwrap()))
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
          let errmsg = match e {
            oauth2::RequestTokenError::ServerResponse(resp) => {
              resp
                .error_description()
                .map(|s| s.as_str())
                .unwrap_or_default()
                .to_string()
            }
            _ => "Unknown error".to_string(),
          };
          (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to exchange token: {:?}", errmsg),
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
  match &state.github_client {
    Some(client) => {
      let token_result = client
        .exchange_code(AuthorizationCode::new(param.code.expect("Missing authorization code")))
        .request_async(oauth2::reqwest::async_http_client).await;

      match token_result {
        Ok(token) => {
          let url = state.config.server.auth.github.user_info_url
            .clone()
            .expect("Missing 'user_info_url' configure");

          // see:https://docs.github.com/en/rest/users/users?apiVersion=2022-11-28#get-a-user
          let resp = state.default_http_client
            .get(&url)
            // see:https://docs.github.com/en/rest/using-the-rest-api/getting-started-with-the-rest-api?apiVersion=2022-11-28#user-agent-required
            .header(reqwest::header::USER_AGENT, "The-Rust-App-Reqwest/1.0")
            .bearer_auth(token.access_token().secret())
            .send().await
            .expect("Could not to sending get github user info.");

          let user_info: serde_json::Value = match resp.json().await {
            Ok(info) => info,
            Err(e) => {
              let errmsg = format!("Failed to parse github user info: {}", e);
              println!("{}", errmsg);
              return (StatusCode::INTERNAL_SERVER_ERROR, errmsg).into_response();
            }
          };
          println!("Received github user info {:?}", user_info);

          // TODO 未知原因 github 正常返回 json，但解码失败，暂先手动解析.
          let id = user_info["id"].to_string();
          let login = user_info["login"].to_string();
          let github_user = GithubUserInfo::default(Some(id), Some(login));

          //   let res = match AuthHandler::new(&state).handle_auth_github(github_user).await {
          //     Ok(_) => {
          //       // Add session id to cookie.
          //       //   let cookie = CookieBuilder::new("_WL4G_REVEZONE_SID", "bar")
          //       //     .max_age(time::Duration::ZERO)
          //       //     .path("/")
          //       //     .build();
          //       Redirect::to("/").into_response()
          //     }
          //     Err(e) => { (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response() }
          //   };

          //   let res: hyper::Response<axum::body::Body> = Redirect::to("/").into_response();
          //   res

          AuthHandler::new(&state).handle_auth_github(github_user);

          Redirect::to("/").into_response()
        }
        Err(e) => {
          let cause = match e {
            oauth2::RequestTokenError::ServerResponse(resp) => {
              resp
                .error_description()
                .map(|s| s.as_str())
                .unwrap_or_default()
                .to_string()
            }
            _ => "Unknown error".to_string(),
          };
          (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to exchange token: {:?}", cause),
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
  let result = AuthHandler::new(&state).handle_logout(LogoutRequest {
    access_token: None,
    refresh_token: None,
  }).await;

  match result {
    Ok(_) => {
      // TODO using config '_revezone_sid'
      let cookie = CookieBuilder::new("_WL4G_REVEZONE_SID", "bar")
        .max_age(time::Duration::ZERO)
        .path("/")
        .build();
      cookies.remove(cookie);
      Redirect::to("/")
    }
    Err(e) => {
      println!("Failed to logout. {:?}", e);
      Redirect::to("/")
    }
  }
}
