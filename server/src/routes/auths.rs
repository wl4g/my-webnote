use std::result::Result;
use std::result::Result::Ok;
use axum::{
    body::Body,
    extract::{ Query, Request, State },
    http::{ header, StatusCode },
    middleware::Next,
    response::{ Html, IntoResponse, Redirect },
    routing::get,
    Router,
};

use hyper::HeaderMap;
use oauth2::{ AuthorizationCode, CsrfToken, Scope, TokenResponse };

use openidconnect::{
    core::{ CoreAuthenticationFlow, CoreTokenResponse, CoreUserInfoClaims },
    reqwest::async_http_client,
    Nonce,
};

use tower_cookies::{ cookie::{ time::{ self }, CookieBuilder }, CookieManagerLayer };

use crate::{
    config::{ config_api::{ DEFAULT_404_HTML, DEFAULT_LOGIN_HTML }, resources::handle_static },
    context::state::AppState,
    handlers::auths::{ AuthHandler, IAuthHandler },
    types::auths::{ CallbackGithubRequest, CallbackOidcRequest, GithubUserInfo, LogoutRequest },
    utils::{ self, auths, webs },
};

pub const ROOT_URI: &str = "/";
pub const AUTH_CONNECT_OIDC_URI: &str = "/auth/connect/oidc";
pub const AUTH_CONNECT_GITHUB_URI: &str = "/auth/connect/github";
pub const AUTH_CALLBACK_OIDC_URI: &str = "/auth/callback/oidc";
pub const AUTH_CALLBACK_GITHUB_URI: &str = "/auth/callback/github";
pub const AUTH_LOGOUT_URI: &str = "/auth/logout";
pub const STATIC_RESOURCES_URI: &str = "/static/*file";

pub fn init() -> Router<AppState> {
    Router::new()
        .route(ROOT_URI, get(handle_page_root))
        .route(STATIC_RESOURCES_URI, get(handle_static))
        .route(AUTH_CONNECT_OIDC_URI, get(handle_connect_oidc))
        .route(AUTH_CONNECT_GITHUB_URI, get(handle_connect_github))
        .route(AUTH_CALLBACK_OIDC_URI, get(handle_callback_oidc))
        .route(AUTH_CALLBACK_GITHUB_URI, get(handle_callback_github))
        .route(AUTH_LOGOUT_URI, get(handle_logout))
        .fallback(handle_page_404) // Global auto internal forwarding when not found.
        .layer(CookieManagerLayer::new())
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next
) -> impl IntoResponse {
    let path = req.uri().path();

    // 1. Exclude paths that don't require authentication.
    // 1.1 Paths that must be excluded according to the authentication mechanism's requirements.
    // The root path is also excluded by default.
    if
        path == AUTH_CONNECT_OIDC_URI ||
        path == AUTH_CONNECT_GITHUB_URI ||
        path == AUTH_CALLBACK_OIDC_URI ||
        path == AUTH_CALLBACK_GITHUB_URI ||
        path == STATIC_RESOURCES_URI
    {
        return next.run(req).await;
    }
    // 1.2 According to the configuration of anonymous authentication path.
    let is_anonymous = state.config.auth_anonymous_glob_matcher
        .as_ref()
        .map(|glob| glob.is_match(path))
        .unwrap_or(false);
    if is_anonymous {
        // If it is an anonymous path, pass it directly.
        return next.run(req).await;
    }

    // 2. Verify for bearer token.
    if let Some(auth_header) = req.headers().get("Authorization") {
        // with Header
        if let std::result::Result::Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let ak = &auth_str[7..];
                if validate_token(&state, ak).await {
                    return next.run(req).await;
                }
            }
        }
    } else {
        // with Cookie
        let ak = req
            .headers()
            .get("Cookie")
            .map(|c| {
                let cookie_str = String::from_utf8(c.as_bytes().to_vec()).unwrap();
                webs::get_cookie_from_str(cookie_str.as_str(), &state.config.auth_jwt_ak_name)
            })
            .unwrap_or(None);
        if ak.is_some() {
            if validate_token(&state, ak.unwrap().as_str()).await {
                // If logged in, and redirect to home page
                if path == ROOT_URI {
                    return utils::auths::auth_resp_redirect_or_json(
                        &state.config,
                        &req.headers(),
                        &state.config.auth.success_url.to_owned().unwrap().as_str(),
                        StatusCode::OK,
                        "Logged",
                        None
                    );
                }
                return next.run(req).await;
            }
        }
    }

    utils::auths::auth_resp_redirect_or_json(
        &state.config,
        &req.headers(),
        &state.config.auth.login_url.to_owned().unwrap().as_str(),
        StatusCode::UNAUTHORIZED,
        "Logout",
        None
    )
}

async fn validate_token(state: &AppState, ak: &str) -> bool {
    // 1. Verify the token is valid.
    match auths::validate_jwt(&state.config, ak) {
        std::result::Result::Ok(claims) => {
            let exp = time::OffsetDateTime::from_unix_timestamp(claims.exp as i64).unwrap();
            let now = time::OffsetDateTime::now_utc();
            if exp > now {
                // 2. Verify whether the token is in the cancelled blacklist.
                let cache = state.string_cache.cache(&state.config);
                match cache.get(get_auth_handler(state).build_logout_blacklist_key(ak)).await {
                    std::result::Result::Ok(logout) => {
                        tracing::warn!("Invalid the token because in blacklist for {}", ak);
                        logout.is_none()
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

async fn handle_page_root() -> impl IntoResponse {
    handle_page_login().await
}

async fn handle_page_login() -> impl IntoResponse {
    (StatusCode::OK, Html(DEFAULT_LOGIN_HTML))
}

// /*
//  * When unauthentication auto internal forword example:
//  *
//  *  let protected_route = get(|| async {
//  *      if !has_permission() {
//  *          return handle_403().await;
//  *      }
//  *      // Some logical process ...
//  *  });
//  */
// async fn handle_page_403() -> impl IntoResponse {
//     (StatusCode::FORBIDDEN, Html(DEFAULT_403_HTML))
// }

async fn handle_page_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Html(DEFAULT_404_HTML))
}

#[utoipa::path(
    get,
    path = AUTH_CONNECT_OIDC_URI,
    responses((status = 200, description = "Login for OIDC.")),
    tag = "Authentication"
)]
pub async fn handle_connect_oidc(State(state): State<AppState>) -> impl IntoResponse {
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

            tracing::debug!(
                "Connecting to OIDC url: {}, csrf: {:?}, nonce: {:?}",
                auth_url.as_str(),
                csrf_token,
                nonce
            );

            match
                get_auth_handler(&state).handle_auth_create_nonce(
                    csrf_token.secret(),
                    nonce.secret().to_string()
                ).await
            {
                std::result::Result::Ok(_) => {
                    // TODO: 此基于 cookie crsf 校验 nonce 的机制仅支持浏览器环境, 若是 Android/iOS 如何设计更优雅?移动端非web其实不需要crsf?
                    let headers = webs::create_cookie_headers("_csrf_token", csrf_token.secret());
                    (headers, Redirect::to(auth_url.as_str())).into_response()
                }
                Err(e) => {
                    tracing::error!("Create nonce failed: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[utoipa::path(
    get,
    path = AUTH_CONNECT_GITHUB_URI,
    responses((status = 200, description = "Login for Github.")),
    tag = "Authentication"
)]
pub async fn handle_connect_github(State(state): State<AppState>) -> impl IntoResponse {
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
    path = AUTH_CALLBACK_OIDC_URI,
    responses((status = 200, description = "Callback for OIDC.")),
    tag = "Authentication"
)]
pub async fn handle_callback_oidc(
    State(state): State<AppState>,
    Query(param): Query<CallbackOidcRequest>,
    headers: header::HeaderMap
) -> impl IntoResponse {
    match &state.oidc_client {
        Some(client) => {
            let code = match param.code {
                Some(code) => code,
                None => {
                    return auths::auth_resp_redirect_or_json(
                        &state.config,
                        &headers,
                        &state.config.auth.login_url.to_owned().unwrap(),
                        StatusCode::BAD_REQUEST,
                        format!("Missing authentication code").as_str(),
                        None
                    );
                }
            };

            let token_result: Result<CoreTokenResponse, _> = client
                .exchange_code(AuthorizationCode::new(code))
                .request_async(async_http_client).await;

            match token_result {
                Ok(token_response) => {
                    let id_token = match token_response.extra_fields().id_token() {
                        Some(token) => token,
                        None => {
                            return auths::auth_resp_redirect_or_json(
                                &state.config,
                                &headers,
                                &state.config.auth.login_url.to_owned().unwrap(),
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("No ID token found").as_str(),
                                None
                            );
                        }
                    };

                    // TODO: 此基于 cookie crsf 校验 nonce 的机制仅支持浏览器环境, 若是 Android/iOS 如何设计更优雅?移动端非web其实不需要crsf?
                    let csrf_token = match webs::get_cookie_from_headers("_csrf_token", &headers) {
                        Some(token) => token,
                        None => {
                            return auths::auth_resp_redirect_or_json(
                                &state.config,
                                &headers,
                                &state.config.auth.login_url.to_owned().unwrap(),
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("No csrf token found").as_str(),
                                None
                            );
                        }
                    };

                    let nonce_string = match
                        get_auth_handler(&state).handle_auth_get_nonce(csrf_token.as_str()).await
                    {
                        Ok(Some(nonce)) => nonce,
                        _ => {
                            return auths::auth_resp_redirect_or_json(
                                &state.config,
                                &headers,
                                &state.config.auth.login_url.to_owned().unwrap(),
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("failed to get oidc authing nonce").as_str(),
                                None
                            );
                        }
                    };
                    let nonce = openidconnect::Nonce::new(nonce_string);

                    let claims = match id_token.claims(&client.id_token_verifier(), &nonce) {
                        Ok(claims) => claims,
                        Err(e) => {
                            return auths::auth_resp_redirect_or_json(
                                &state.config,
                                &headers,
                                &state.config.auth.login_url.to_owned().unwrap(),
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("failed to verify ID token: {:?}", e).as_str(),
                                None
                            );
                        }
                    };

                    let access_token = token_response.access_token().clone();
                    let userinfo_request = match client.user_info(access_token, None) {
                        Ok(req) => req,
                        Err(e) => {
                            return auths::auth_resp_redirect_or_json(
                                &state.config,
                                &headers,
                                &state.config.auth.login_url.to_owned().unwrap(),
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("failed to create user info request: {:?}", e).as_str(),
                                None
                            );
                        }
                    };

                    let userinfo: CoreUserInfoClaims = match
                        userinfo_request.request_async(async_http_client).await
                    {
                        Ok(info) => info,
                        Err(e) => {
                            return auths::auth_resp_redirect_or_json(
                                &state.config,
                                &headers,
                                &state.config.auth.login_url.to_owned().unwrap(),
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("failed to get user info claims: {:?}", e).as_str(),
                                None
                            );
                        }
                    };

                    let user_id = claims.subject().to_string();
                    tracing::debug!("User subject: {}", user_id);
                    tracing::debug!("User name: {:?}", userinfo.name());
                    tracing::debug!("User email: {:?}", userinfo.email());

                    let result = match
                        get_auth_handler(&state).handle_auth_callback_oidc(userinfo).await
                    {
                        Ok(c) => {
                            if c > 0 {
                                get_auth_handler(&state).handle_login_success(
                                    &state.config,
                                    &user_id,
                                    &headers
                                ).await
                            } else {
                                return auths::auth_resp_redirect_or_json(
                                    &state.config,
                                    &headers,
                                    &state.config.auth.login_url.to_owned().unwrap(),
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    "Failed to bind oidc user",
                                    None
                                );
                            }
                        }
                        Err(e) => {
                            return auths::auth_resp_redirect_or_json(
                                &state.config,
                                &headers,
                                &state.config.auth.login_url.to_owned().unwrap(),
                                StatusCode::INTERNAL_SERVER_ERROR,
                                e.to_string().as_str(),
                                None
                            );
                        }
                    };
                    result
                }
                Err(e) => {
                    return auths::auth_resp_redirect_or_json(
                        &state.config,
                        &headers,
                        &state.config.auth.login_url.to_owned().unwrap(),
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("failed exchange token: {:?}", e).as_str(),
                        None
                    );
                }
            }
        }
        None => {
            return auths::auth_resp_redirect_or_json(
                &state.config,
                &headers,
                &state.config.auth.login_url.to_owned().unwrap(),
                StatusCode::INTERNAL_SERVER_ERROR,
                "Oidc client not configured",
                None
            );
        }
    }
}

#[utoipa::path(
    get,
    path = AUTH_CALLBACK_GITHUB_URI,
    responses((status = 200, description = "Callback for github.")),
    tag = "Authentication"
)]
pub async fn handle_callback_github(
    State(state): State<AppState>,
    Query(param): Query<CallbackGithubRequest>,
    headers: HeaderMap
) -> impl IntoResponse {
    match &state.github_client {
        Some(client) => {
            let token_result = client
                .exchange_code(
                    AuthorizationCode::new(param.code.expect("Missing authorization code"))
                )
                .request_async(oauth2::reqwest::async_http_client).await;

            match token_result {
                Ok(token) => {
                    let url = state.config.auth.github.user_info_url
                        .clone()
                        .expect("Missing 'user_info_url' configure");

                    // see:https://docs.github.com/en/rest/users/users?apiVersion=2022-11-28#get-a-user
                    let resp = match
                        state.default_http_client
                            .get(&url)
                            // see:https://docs.github.com/en/rest/using-the-rest-api/getting-started-with-the-rest-api?apiVersion=2022-11-28#user-agent-required
                            .header(reqwest::header::USER_AGENT, "The-Rust-App-Reqwest/1.0")
                            .bearer_auth(token.access_token().secret())
                            .send().await
                    {
                        Ok(resp) => { resp }
                        Err(e) => {
                            return auths::auth_resp_redirect_or_json(
                                &state.config,
                                &headers,
                                &state.config.auth.login_url.to_owned().unwrap(),
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!(
                                    "failed to sending get github user info. {:?}",
                                    e.to_string()
                                ).as_str(),
                                None
                            );
                        }
                    };

                    let user_info: GithubUserInfo = match resp.json().await {
                        Ok(info) => info,
                        Err(e) => {
                            let errmsg = format!("Failed to parse github user info: {:?}", e);
                            tracing::error!("{}", errmsg);
                            //return (StatusCode::INTERNAL_SERVER_ERROR, errmsg).into_response();
                            return auths::auth_resp_redirect_or_json(
                                &state.config,
                                &headers,
                                &state.config.auth.login_url.to_owned().unwrap(),
                                StatusCode::INTERNAL_SERVER_ERROR,
                                errmsg.as_str(),
                                None
                            );
                        }
                    };
                    tracing::info!("Received github user info {:?}", user_info);

                    let user_id = user_info.id.unwrap_or(-1 as i64);
                    let github_user = GithubUserInfo::default(
                        Some(user_id),
                        Some(user_info.login.unwrap().to_string())
                    );

                    // TODO: using dependency injection to get the handler
                    let result = match
                        get_auth_handler(&state).handle_auth_callback_github(github_user).await
                    {
                        Ok(c) => {
                            if c > 0 {
                                get_auth_handler(&state).handle_login_success(
                                    &state.config,
                                    user_id.to_string().as_str(),
                                    &headers
                                ).await
                            } else {
                                return auths::auth_resp_redirect_or_json(
                                    &state.config,
                                    &headers,
                                    &state.config.auth.login_url.to_owned().unwrap(),
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    "Failed to bind github user",
                                    None
                                );
                            }
                        }
                        Err(e) => {
                            return auths::auth_resp_redirect_or_json(
                                &state.config,
                                &headers,
                                &state.config.auth.login_url.to_owned().unwrap(),
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("{:?}", e.to_string()).as_str(),
                                None
                            );
                        }
                    };
                    result
                }
                Err(e) => {
                    let cause = match e {
                        oauth2::RequestTokenError::ServerResponse(resp) => {
                            resp.error_description()
                                .map(|s| s.as_str())
                                .unwrap_or_default()
                                .to_string()
                        }
                        _ => "Unknown error".to_string(),
                    };
                    return auths::auth_resp_redirect_or_json(
                        &state.config,
                        &headers,
                        &state.config.auth.login_url.to_owned().unwrap(),
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("failed to exchange token. reason: {}", cause).as_str(),
                        None
                    );
                }
            }
        }
        None => {
            return auths::auth_resp_redirect_or_json(
                &state.config,
                &headers,
                &state.config.auth.login_url.to_owned().unwrap(),
                StatusCode::INTERNAL_SERVER_ERROR,
                "Github client not configured",
                None
            );
        }
    }
}

// #[utoipa::path(
//     post,
//     path = AUTH_LOGOUT_URI,
//     request_body(
//         content = Option<LogoutRequest>,
//         description = "Optional logout request parameters",
//         content_type = "application/json",
//         example = json!({"access_token": null, "refresh_token": null}),
//     ),
//     responses((status = 200, description = "Logout.")),
//     tag = "Authentication"
// )]
// pub async fn handle_logout(
//     State(state): State<AppState>,
//     request: axum::extract::Request<Body>
// ) -> impl IntoResponse {
//     let headers = &request.headers().clone();
//     let body = request.into_body();

//     let cookie_ak = webs::get_cookie_from_headers(&state.config.auth_jwt_ak_name, headers);
//     let cookie_rk = webs::get_cookie_from_headers(&state.config.auth_jwt_rk_name, headers);

//     let param: LogoutRequest = match
//         serde_json::from_slice(
//             &(match axum::body::to_bytes(body, usize::MAX).await {
//                 Ok(bytes) => bytes,
//                 Err(_) => {
//                     return auths::auth_resp_redirect_or_json(
//                         &state.config,
//                         headers,
//                         &state.config.auth.login_url.to_owned().unwrap(),
//                         StatusCode::BAD_REQUEST,
//                         "Read request body failed",
//                         None
//                     );
//                 }
//             })
//         )
//     {
//         Ok(param) => param,
//         Err(_) => {
//             return auths::auth_resp_redirect_or_json(
//                 &state.config,
//                 headers,
//                 &state.config.auth.login_url.to_owned().unwrap(),
//                 StatusCode::BAD_REQUEST,
//                 "Invalid parameter json",
//                 None
//             );
//         }
//     };

//     let logout = LogoutRequest {
//         access_token: param.access_token.or_else(|| cookie_ak),
//         refresh_token: param.refresh_token.or_else(|| cookie_rk),
//     };

//     match get_auth_handler(&state).handle_logout(logout).await {
//         Ok(_) => {
//             let removal_ak = CookieBuilder::new(state.config.auth_jwt_ak_name.to_string(), "_")
//                 .removal()
//                 .build();
//             let removal_rk = CookieBuilder::new(state.config.auth_jwt_rk_name.to_string(), "_")
//                 .removal()
//                 .build();

//             // Response::builder()
//             //     .status(StatusCode::FOUND)
//             //     .header(header::LOCATION, "/")
//             //     .header(header::SET_COOKIE, removal_ak.to_string())
//             //     .header(header::SET_COOKIE, removal_rk.to_string())
//             //     .body(axum::body::Body::empty())
//             //     .unwrap()
//             auths::auth_resp_redirect_or_json(
//                 &state.config,
//                 headers,
//                 &state.config.auth.login_url.to_owned().unwrap().as_str(),
//                 StatusCode::BAD_REQUEST,
//                 "Bad Parameters",
//                 Some((removal_ak, removal_rk))
//             )
//         }
//         Err(e) => {
//             tracing::error!("Failed to logout. {:?}", e);
//             return auths::auth_resp_redirect_or_json(
//                 &state.config,
//                 headers,
//                 &state.config.auth.login_url.to_owned().unwrap(),
//                 StatusCode::BAD_REQUEST,
//                 e.to_string().as_str(),
//                 None
//             );
//         }
//     }
// }

#[utoipa::path(
    get,
    path = AUTH_LOGOUT_URI,
    params(LogoutRequest),
    responses((status = 200, description = "Logout.")),
    tag = "Authentication"
)]
pub async fn handle_logout(
    State(state): State<AppState>,
    headers: header::HeaderMap,
    Query(param): Query<LogoutRequest>
) -> impl IntoResponse {
    let cookie_ak = webs::get_cookie_from_headers(&state.config.auth_jwt_ak_name, &headers);
    let cookie_rk = webs::get_cookie_from_headers(&state.config.auth_jwt_rk_name, &headers);

    let logout = LogoutRequest {
        access_token: param.access_token.or_else(|| cookie_ak),
        refresh_token: param.refresh_token.or_else(|| cookie_rk),
    };

    match get_auth_handler(&state).handle_logout(logout).await {
        Ok(_) => {
            let removal_ak = CookieBuilder::new(state.config.auth_jwt_ak_name.to_string(), "_")
                .removal()
                .build();
            let removal_rk = CookieBuilder::new(state.config.auth_jwt_rk_name.to_string(), "_")
                .removal()
                .build();

            auths::auth_resp_redirect_or_json(
                &state.config,
                &headers,
                &state.config.auth.login_url.to_owned().unwrap().as_str(),
                StatusCode::OK,
                "Bad Parameters",
                Some((removal_ak, removal_rk))
            )
        }
        Err(e) => {
            tracing::error!("Failed to logout. {:?}", e);
            return auths::auth_resp_redirect_or_json(
                &state.config,
                &headers,
                &state.config.auth.login_url.to_owned().unwrap(),
                StatusCode::BAD_REQUEST,
                e.to_string().as_str(),
                None
            );
        }
    }
}

fn get_auth_handler(state: &AppState) -> Box<dyn IAuthHandler + '_> {
    // TODO: using dependency injection to get the handler
    Box::new(AuthHandler::new(state))
}
