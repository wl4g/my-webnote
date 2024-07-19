use std::result::Result;
use std::result::Result::Ok;
use axum::{
    body::Body,
    extract::{ Query, Request, State },
    http::{ header, StatusCode },
    middleware::Next,
    response::{ Html, IntoResponse },
    routing::{ get, post },
    Router,
};

use hyper::HeaderMap;
use oauth2::{ AuthorizationCode, CsrfToken, Scope, TokenResponse };

use openidconnect::{
    core::{ CoreAuthenticationFlow, CoreTokenResponse, CoreUserInfoClaims },
    reqwest::async_http_client,
    Nonce,
};

use tower_cookies::{ cookie::{ time::{ self, Duration }, CookieBuilder }, CookieManagerLayer };

use crate::{
    config::{ config_api::DEFAULT_404_HTML, resources::handle_static },
    context::state::AppState,
    handlers::auths::{ AuthHandler, IAuthHandler },
    types::{
        auths::{
            CallbackGithubRequest,
            CallbackOidcRequest,
            GetPubKeyRequest,
            GetPubKeyResponse,
            GithubUserInfo,
            LogoutRequest,
            PasswordLoginRequest,
        },
        RespBase,
    },
    utils::{ self, auths::{ self, AuthUserClaims, SecurityContext }, webs },
};

use super::ValidatedJson;

pub const ROOT_URI: &str = "/";
pub const AUTH_CONNECT_OIDC_URI: &str = "/auth/connect/oidc";
pub const AUTH_CONNECT_GITHUB_URI: &str = "/auth/connect/github";
pub const AUTH_CALLBACK_OIDC_URI: &str = "/auth/callback/oidc";
pub const AUTH_CALLBACK_GITHUB_URI: &str = "/auth/callback/github";
pub const AUTH_LOGOUT_URI: &str = "/auth/logout";
pub const AUTH_LOGIN_PUBKEY_URI: &str = "/auth/login/pubkey";
pub const AUTH_LOGIN_VERIFY_URI: &str = "/auth/login/verify";
pub const STATIC_RESOURCES_URI: &str = "/static/*file";

pub const EXCLUDED_PATHS: [&str; 7] = [
    AUTH_CONNECT_OIDC_URI,
    AUTH_CONNECT_GITHUB_URI,
    AUTH_CALLBACK_OIDC_URI,
    AUTH_CALLBACK_GITHUB_URI,
    AUTH_LOGIN_PUBKEY_URI,
    AUTH_LOGIN_VERIFY_URI,
    STATIC_RESOURCES_URI,
];

pub const CSRF_TOKEN_NAME: &str = "csrf_token";

pub fn init() -> Router<AppState> {
    Router::new()
        //.route(ROOT_URI, get(handle_page_root))
        .route(STATIC_RESOURCES_URI, get(handle_static))
        .route(AUTH_CONNECT_OIDC_URI, get(handle_connect_oidc))
        .route(AUTH_CONNECT_GITHUB_URI, get(handle_connect_github))
        .route(AUTH_CALLBACK_OIDC_URI, get(handle_callback_oidc))
        .route(AUTH_CALLBACK_GITHUB_URI, get(handle_callback_github))
        .route(AUTH_LOGOUT_URI, get(handle_logout))
        .route(AUTH_LOGIN_PUBKEY_URI, post(handle_login_pubkey))
        .route(AUTH_LOGIN_VERIFY_URI, post(handle_login_verify))
        .fallback(handle_page_404) // Global auto internal forwarding when not found.
        .layer(CookieManagerLayer::new())
}

// --------------------------------------
// Global Authentication interceptors.
// --------------------------------------

pub async fn auth_middleware(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next
) -> impl IntoResponse {
    let path = req.uri().path();

    // 1. Exclude paths that don't require authentication.
    // 1.1 Paths that must be excluded according to the authentication mechanism's requirements.
    // The root path is also excluded by default.
    if EXCLUDED_PATHS.contains(&path) {
        return next.run(req).await;
    }

    // 1.2 According to the configuration of anonymous authentication path.
    if
        state.config.auth_anonymous_glob_matcher
            .as_ref()
            .map(|glob| glob.is_match(path))
            .unwrap_or(false)
    {
        // If it is an anonymous path, pass it directly.
        return next.run(req).await;
    }

    // 2. Verify for bearer token.
    let (is_authenticated, claims) = if let Some(auth_header) = req.headers().get("Authorization") {
        // 2.1 with Header
        if let std::result::Result::Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let ak = &auth_str[7..];
                validate_token(&state, ak).await
            } else {
                // for compatibility no 'Bearer' prefix.
                validate_token(&state, auth_str).await
            }
        } else {
            (false, None)
        }
    } else {
        // 2.2 with Cookie
        let ak = req
            .headers()
            .get("Cookie")
            .map(|c| {
                let cookie_str = String::from_utf8(c.as_bytes().to_vec()).unwrap();
                webs::get_cookie_from_str(cookie_str.as_str(), &state.config.auth_jwt_ak_name)
            })
            .unwrap_or(None);
        if ak.is_some() {
            validate_token(&state, ak.unwrap().as_str()).await
        } else {
            (false, None)
        }
    };

    if is_authenticated {
        // 3. Bind authenticated info to context.
        tracing::info!("Authenticated user: {:?}", claims);
        SecurityContext::get_instance().bind(claims).await;

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

        // 4. Pass to call next routes.
        return next.run(req).await;
    }

    // 5. Unauthenticated Response
    utils::auths::auth_resp_redirect_or_json(
        &state.config,
        &req.headers(),
        &state.config.auth.login_url.to_owned().unwrap().as_str(),
        StatusCode::UNAUTHORIZED,
        "Logout",
        None
    )
}

async fn validate_token(state: &AppState, ak: &str) -> (bool, Option<AuthUserClaims>) {
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
                        (logout.is_none(), Some(claims))
                    }
                    Err(_) => {
                        tracing::debug!("Valid the token because not in blacklist for {}", ak);
                        (true, Some(claims))
                    }
                }
            } else {
                tracing::debug!("Valid the token for {}", ak);
                (false, Some(claims))
            }
        }
        Err(_) => {
            tracing::warn!("Invalid the token because expired for {}", ak);
            (false, None)
        }
    }
}

// // Notice: The settings of middlewares are in order, which will affect the priority of route matching.
// // The later the higher the priority? For example, if auth_middleware is set at the end, it will
// // enter when requesting '/', otherwise it will not enter if it is set at the front, and will
// // directly enter handle_root().
// async fn handle_page_root() -> impl IntoResponse {
//     handle_page_login().await
// }
// async fn handle_page_login() -> impl IntoResponse {
//     (StatusCode::OK, Html(DEFAULT_LOGIN_HTML))
// }

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

// --------------------------------------
// OIDC/Github OAuth2 login.
// --------------------------------------

#[utoipa::path(
    get,
    path = AUTH_CONNECT_OIDC_URI,
    responses((status = 200, description = "Login for OIDC.")),
    tag = "Authentication"
)]
async fn handle_connect_oidc(
    State(state): State<AppState>,
    headers: header::HeaderMap
) -> impl IntoResponse {
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
                    // crsf 校验 nonce 的机制仅支持浏览器环境, 如 Android/iOS 等 CS 客户端可忽略.
                    let csrf_cookie = CookieBuilder::new("_csrf_token", csrf_token.secret())
                        .path("/")
                        .http_only(true)
                        .secure(true)
                        .max_age(
                            Duration::milliseconds(
                                state.config.auth.jwt_validity_ak.unwrap() as i64
                            )
                        )
                        .build();
                    return auths::auth_resp_redirect_or_json(
                        &state.config,
                        &headers,
                        auth_url.as_str(),
                        StatusCode::OK,
                        "ok",
                        Some((None, None, Some(csrf_cookie)))
                    );
                }
                Err(e) => {
                    let errmsg = format!("Failed to create nonce. {:?}", e);
                    tracing::error!(errmsg);
                    return auths::auth_resp_redirect_or_json(
                        &state.config,
                        &headers,
                        &state.config.auth.login_url.to_owned().unwrap(),
                        StatusCode::INTERNAL_SERVER_ERROR,
                        errmsg.as_str(),
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
                format!("OIDC client not configured").as_str(),
                None
            );
        }
    }
}

#[utoipa::path(
    get,
    path = AUTH_CONNECT_GITHUB_URI,
    responses((status = 200, description = "Login for Github.")),
    tag = "Authentication"
)]
async fn handle_connect_github(
    State(state): State<AppState>,
    headers: header::HeaderMap
) -> impl IntoResponse {
    match &state.github_client {
        Some(client) => {
            let (auth_url, _) = client
                .authorize_url(oauth2::CsrfToken::new_random)
                .add_scope(Scope::new(state.config.auth.github.scope.clone().unwrap()))
                .url();
            return auths::auth_resp_redirect_or_json(
                &state.config,
                &headers,
                auth_url.as_str(),
                StatusCode::INTERNAL_SERVER_ERROR,
                "ok",
                None
            );
        }
        None => {
            return auths::auth_resp_redirect_or_json(
                &state.config,
                &headers,
                &state.config.auth.login_url.to_owned().unwrap(),
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Github oauth2 client not configured").as_str(),
                None
            );
        }
    }
}

#[utoipa::path(
    get,
    path = AUTH_CALLBACK_OIDC_URI,
    responses((status = 200, description = "Callback for OIDC.")),
    tag = "Authentication"
)]
async fn handle_callback_oidc(
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
                    // 已直接获取 userinfo 信息更全, 因此 ID TOKEN 等暂无用途.
                    // let id_token = match token_response.extra_fields().id_token() {
                    //     Some(token) => token,
                    //     None => {
                    //         return auths::auth_resp_redirect_or_json(
                    //             &state.config,
                    //             &headers,
                    //             &state.config.auth.login_url.to_owned().unwrap(),
                    //             StatusCode::INTERNAL_SERVER_ERROR,
                    //             format!("No ID token found").as_str(),
                    //             None
                    //         );
                    //     }
                    // };
                    // // TODO: 此基于 cookie crsf 校验 nonce 的机制仅支持浏览器环境, 若是 Android/iOS 如何设计更优雅?移动端非web其实不需要crsf?
                    // let csrf_token = match webs::get_cookie_from_headers("_csrf_token", &headers) {
                    //     Some(token) => token,
                    //     None => {
                    //         return auths::auth_resp_redirect_or_json(
                    //             &state.config,
                    //             &headers,
                    //             &state.config.auth.login_url.to_owned().unwrap(),
                    //             StatusCode::INTERNAL_SERVER_ERROR,
                    //             format!("No csrf token found").as_str(),
                    //             None
                    //         );
                    //     }
                    // };
                    // let nonce_string = match
                    //     get_auth_handler(&state).handle_auth_get_nonce(csrf_token.as_str()).await
                    // {
                    //     Ok(Some(nonce)) => nonce,
                    //     _ => {
                    //         return auths::auth_resp_redirect_or_json(
                    //             &state.config,
                    //             &headers,
                    //             &state.config.auth.login_url.to_owned().unwrap(),
                    //             StatusCode::INTERNAL_SERVER_ERROR,
                    //             format!("failed to get oidc authing nonce").as_str(),
                    //             None
                    //         );
                    //     }
                    // };
                    // let nonce = openidconnect::Nonce::new(nonce_string);
                    // let claims = match id_token.claims(&client.id_token_verifier(), &nonce) {
                    //     Ok(claims) => claims,
                    //     Err(e) => {
                    //         return auths::auth_resp_redirect_or_json(
                    //             &state.config,
                    //             &headers,
                    //             &state.config.auth.login_url.to_owned().unwrap(),
                    //             StatusCode::INTERNAL_SERVER_ERROR,
                    //             format!("failed to verify ID token: {:?}", e).as_str(),
                    //             None
                    //         );
                    //     }
                    // };

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

                    //let oidc_sub = claims.subject().to_string();
                    let oidc_name = userinfo
                        .preferred_username()
                        .map(|c| c.to_string())
                        .unwrap_or_default();
                    let oidc_email = userinfo
                        .email()
                        .map(|c| c.to_string())
                        .unwrap_or_default();

                    tracing::debug!("Received oidc user info: {:?}", userinfo);
                    // tracing::debug!("User oidc subject: {:?}", oidc_name);
                    // tracing::debug!("User oidc name: {:?}", oidc_name);
                    // tracing::debug!("User oidc email: {:?}", oidc_email);

                    let result = match
                        get_auth_handler(&state).handle_auth_callback_oidc(userinfo).await
                    {
                        Ok(uid) => {
                            if uid > 0 {
                                get_auth_handler(&state).handle_login_success(
                                    &state.config,
                                    uid,
                                    &oidc_name,
                                    &oidc_email,
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
async fn handle_callback_github(
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

                    let github_sub = user_info.id;
                    let github_uname = user_info.login;
                    let github_email = user_info.email;
                    let github_user = GithubUserInfo::default(
                        github_sub,
                        github_uname.to_owned(),
                        github_email.to_owned()
                    );

                    // TODO: using dependency injection to get the handler
                    let result = match
                        get_auth_handler(&state).handle_auth_callback_github(
                            github_user.clone()
                        ).await
                    {
                        Ok(uid) => {
                            if uid > 0 {
                                get_auth_handler(&state).handle_login_success(
                                    &state.config,
                                    uid,
                                    github_uname.unwrap_or_default().as_str(),
                                    github_email.unwrap_or_default().as_str(),
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

// --------------------------------------
// Password Login.
// --------------------------------------

#[utoipa::path(
    post,
    path = AUTH_LOGIN_PUBKEY_URI,
    request_body = GetPubKeyRequest,
    responses((status = 200, description = "Login pubkey.")),
    tag = "Authentication"
)]
#[allow(unused)]
async fn handle_login_pubkey(
    State(state): State<AppState>,
    ValidatedJson(param): ValidatedJson<GetPubKeyRequest>
) -> impl IntoResponse {
    let base64_pubkey = get_auth_handler(&state).handle_login_pubkey(param).await.ok();
    let result = serde_json
        ::to_string(&(GetPubKeyResponse { pubkey: base64_pubkey.unwrap() }))
        .unwrap();
    (StatusCode::OK, result.to_string()).into_response()
}

#[utoipa::path(
    post,
    path = AUTH_LOGIN_VERIFY_URI,
    request_body(
        content = Option<PasswordLoginRequest>,
        description = "Password login request parameters",
        content_type = "application/json",
        example = json!({"username": null, "password": null, "fingerprint_token": null}),
    ),
    responses((status = 200, description = "Password login.")),
    tag = "Authentication"
)]
pub async fn handle_login_verify(
    State(state): State<AppState>,
    request: axum::extract::Request<Body>
) -> impl IntoResponse {
    let headers = &request.headers().clone();
    let body = request.into_body();

    let param: PasswordLoginRequest = match
        serde_json::from_slice(
            &(match axum::body::to_bytes(body, usize::MAX).await {
                Ok(bytes) => bytes,
                Err(e) => {
                    tracing::warn!("Unable to read password login request failed. reason: {:?}", e);
                    return auths::auth_resp_redirect_or_json(
                        &state.config,
                        headers,
                        &state.config.auth.login_url.to_owned().unwrap(),
                        StatusCode::BAD_REQUEST,
                        "Unable to read password login request failed",
                        None
                    );
                }
            })
        )
    {
        Ok(param) => param,
        Err(e) => {
            tracing::warn!("Invalid password login parameter json. reason: {:?}", e);
            return auths::auth_resp_redirect_or_json(
                &state.config,
                headers,
                &state.config.auth.login_url.to_owned().unwrap(),
                StatusCode::BAD_REQUEST,
                "Invalid password login parameter json",
                None
            );
        }
    };

    match get_auth_handler(&state).handle_login_verify(param).await {
        Ok(user) => {
            get_auth_handler(&state).handle_login_success(
                &state.config,
                user.base.id.unwrap(),
                &user.name.to_owned().unwrap_or_default().to_string(),
                &user.email.to_owned().unwrap_or_default().to_string(),
                &headers
            ).await
        }
        Err(e) => {
            let errmsg = format!("Failed to login. {:?}", e.to_string());
            tracing::warn!("{}", errmsg);
            let result = RespBase::errmsg(errmsg.as_str());
            (StatusCode::OK, serde_json::to_string(&result).unwrap()).into_response()
        }
    }
}

// ----- Logout -----

#[utoipa::path(
    get,
    path = AUTH_LOGOUT_URI,
    params(LogoutRequest),
    responses((status = 200, description = "Logout.")),
    tag = "Authentication"
)]
async fn handle_logout(
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
                Some((Some(removal_ak), Some(removal_rk), None))
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
