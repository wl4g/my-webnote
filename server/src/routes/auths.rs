use axum::{
  body::Body,
  extract::{ Query, State },
  http::{ Request, StatusCode },
  middleware::Next,
  response::{ IntoResponse, Redirect, Response },
  routing::{ get, post },
  Router,
};

use oauth2::{ AuthorizationCode, CsrfToken, Scope, TokenResponse };

use openidconnect::{
  core::{ CoreAuthenticationFlow, CoreTokenResponse, CoreUserInfoClaims },
  reqwest::async_http_client,
  Nonce,
};

use tower_cookies::{ cookie::{ time::{ self }, CookieBuilder }, CookieManagerLayer, Cookies };

use crate::{
  context::state::AppState,
  handlers::auths::AuthHandler,
  types::auths::{ CallbackGithubRequest, CallbackOidcRequest, GithubUserInfo, LogoutRequest },
  utils,
};

pub fn init() -> Router<AppState> {
  Router::new()
    .route("/auth/login/oidc", get(connect_oidc))
    .route("/auth/login/github", get(connect_github))
    .route("/auth/callback/oidc", get(callback_oidc))
    .route("/auth/callback/github", get(callback_github))
    .route("/auth/logout", post(logout))
    .layer(CookieManagerLayer::new())
}

pub async fn auth_middleware(
  State(state): State<AppState>,
  req: Request<Body>,
  next: Next
) -> Result<Response, StatusCode> {
  let path = req.uri().path();

  // 1. Exclude paths that don't require authentication.
  if
    path == "/" ||
    path.starts_with("/auth/") ||
    path == "/logout" ||
    path.starts_with("/swagger-ui/") ||
    path.starts_with("/static/") ||
    path.starts_with("/healthz/") ||
    path.starts_with("/public/")
  {
    return Ok(next.run(req).await);
  }

  // 2. Verify for bearer token.
  if let Some(auth_header) = req.headers().get("Authorization") {
    if let std::result::Result::Ok(auth_str) = auth_header.to_str() {
      if auth_str.starts_with("Bearer ") {
        if validate_token(&state, &auth_str[7..]).await {
          return Ok(next.run(req).await);
        }
      }
    }
  }

  Err(StatusCode::UNAUTHORIZED)
}

async fn validate_token(state: &AppState, ak: &str) -> bool {
  // 1. Verify the token is valid.
  match utils::auths::validate_jwt(&state.config.auth, ak) {
    std::result::Result::Ok(claims) => {
      let exp = time::OffsetDateTime::from_unix_timestamp(claims.exp as i64).unwrap();
      let now = time::OffsetDateTime::now_utc();
      if exp > now {
        // 2. Verify whether the token is in the cancelled blacklist.
        let cache = state.string_cache.cache(&state.config);
        //let handler = AuthHandler::new(state);
        match cache.get(AuthHandler::build_logout_blacklist_key(ak)).await {
          std::result::Result::Ok(_) => {
            tracing::warn!("Invalid the token because in blacklist for {}", ak);
            false
          }
          Err(_) => {
            tracing::debug!("Valid the token because not in blacklist for {}", ak);
            true
          }
        }
      } else {
        tracing::debug!("Valid the token for {}", ak);
        false
      }
    }
    Err(_) => {
      tracing::warn!("Invalid the token because expired for {}", ak);
      false
    }
  }
}

#[utoipa::path(
  get,
  path = "/auth/login/oidc",
  responses((status = 200, description = "Login for OIDC.")),
  tag = ""
)]
pub async fn connect_oidc(State(state): State<AppState>) -> impl IntoResponse {
  match &state.oidc_client {
    Some(client) => {
      let (auth_url, csrf_token, nonce) = client
        .authorize_url(
          CoreAuthenticationFlow::AuthorizationCode,
          CsrfToken::new_random,
          Nonce::new_random
        )
        .add_scope(Scope::new(state.config.auth.oidc.scope.clone().unwrap()))
        .url();

      // TODO: 例如存储 csrf_token 和 nonce 以供后续安全校验使用
      tracing::debug!(
        "Connecting to OIDC url: {}, csrf: {:?}, nonce: {:?}",
        auth_url.as_str(),
        csrf_token,
        nonce
      );

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
pub async fn connect_github(State(state): State<AppState>) -> impl IntoResponse {
  match &state.github_client {
    Some(client) => {
      let (auth_url, _) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(Scope::new(state.config.auth.github.scope.clone().unwrap()))
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
  Query(param): Query<CallbackOidcRequest>
) -> impl IntoResponse {
  match &state.oidc_client {
    Some(client) => {
      let code = match param.code {
        Some(code) => code,
        None => {
          return (
            StatusCode::BAD_REQUEST,
            "Missing authorization code".to_string(),
          ).into_response();
        }
      };

      let token_result: Result<CoreTokenResponse, _> = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client).await;

      match token_result {
        Ok(token_response) => {
          let id_token = token_response
            .extra_fields()
            .id_token()
            .ok_or_else(|| (StatusCode::INTERNAL_SERVER_ERROR, "No ID token found".to_string()))
            .unwrap();

          let claims: &openidconnect::IdTokenClaims<openidconnect::EmptyAdditionalClaims, openidconnect::core::CoreGenderClaim> = match
            id_token.claims(&client.id_token_verifier(), &openidconnect::Nonce::new_random())
          {
            Ok(claims) => claims,
            Err(e) => {
              return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to verify ID token: {:?}", e),
              ).into_response();
            }
          };

          let access_token = token_response.access_token().clone();

          let userinfo_request = match client.user_info(access_token, None) {
            Ok(req) => req,
            Err(e) => {
              return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create user info request: {:?}", e),
              ).into_response();
            }
          };

          let userinfo: CoreUserInfoClaims = userinfo_request
            .request_async(async_http_client).await
            .map_err(|e| (
              StatusCode::INTERNAL_SERVER_ERROR,
              format!("Failed to get user info: {:?}", e),
            ))
            .unwrap();

          println!("User subject: {}", claims.subject().as_str());
          println!("User name: {:?}", userinfo.name());
          println!("User email: {:?}", userinfo.email());

          Redirect::to("/").into_response()
        }
        Err(e) => {
          (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to exchange token: {:?}", e),
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
          let url = state.config.auth.github.user_info_url
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
