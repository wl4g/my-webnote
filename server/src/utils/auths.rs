use std::{ collections::HashMap, sync::Arc };

use axum::body::Body;
use chrono::{ Duration, Utc };
use hyper::{ HeaderMap, Response, StatusCode };
use jsonwebtoken::{ decode, encode, DecodingKey, EncodingKey, Header, Validation };
use serde::{ Deserialize, Serialize };
use tower_cookies::cookie::Cookie;

use crate::{
    config::config_api::ApiConfig,
    types::auths::{ LoggedResponse, TokenWrapper },
    utils::webs,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub ext: Option<HashMap<String, String>>,
}

pub fn create_jwt(
    config: &Arc<ApiConfig>,
    user_id: &str,
    is_refresh: bool,
    extra_claims: Option<HashMap<String, String>>
) -> String {
    let expiration = Utc::now()
        .checked_add_signed(
            Duration::milliseconds(
                if is_refresh {
                    config.auth.jwt_validity_rk.unwrap() as i64
                } else {
                    config.auth.jwt_validity_ak.unwrap() as i64
                }
            )
        )
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
        ext: extra_claims,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.auth.jwt_secret.to_owned().unwrap().as_ref())
    ).expect("failed to encode jwt")
}

pub fn validate_jwt(
    config: &Arc<ApiConfig>,
    token: &str
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.auth.jwt_secret.to_owned().unwrap().as_ref()),
        &validation
    )?;
    Ok(token_data.claims)
}

pub fn auth_resp_redirect_or_json(
    config: &Arc<ApiConfig>,
    headers: &HeaderMap,
    redirect_url: &str,
    status: StatusCode,
    message: &str,
    cookies: Option<(Cookie, Cookie)>
) -> Response<Body> {
    let (ak, rk) = match &cookies {
        Some(pair) => {
            (
                Some(TokenWrapper {
                    value: pair.0.value().to_string(),
                    expires_in: config.auth.jwt_validity_ak.unwrap(),
                }),
                Some(TokenWrapper {
                    value: pair.1.value().to_string(),
                    expires_in: config.auth.jwt_validity_rk.unwrap(),
                }),
            )
        }
        None => (None, None),
    };

    let json = LoggedResponse {
        errcode: status.as_u16() as i16,
        errmsg: message.to_string(),
        access_token: ak,
        refresh_token: rk,
        redirect_url: Some(redirect_url.to_owned()),
    };
    let json_str = serde_json::to_string(&json).unwrap();

    webs::response_redirect_or_json(status, headers, cookies, redirect_url, &message, &json_str)
}
