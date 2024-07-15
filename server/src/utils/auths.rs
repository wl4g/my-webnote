use std::{ collections::HashMap, sync::Arc };

use chrono::{ Duration, Utc };
use jsonwebtoken::{ decode, encode, DecodingKey, EncodingKey, Header, Validation };
use serde::{ Deserialize, Serialize };

use crate::config::config_api::ApiConfig;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
  pub sub: String,
  pub exp: usize,
  pub ext: HashMap<String, String>,
}

pub fn create_jwt(
  config: &Arc<ApiConfig>,
  user_id: &str,
  is_refresh: bool,
  extra_claims: HashMap<String, String>
) -> String {
  let expiration = Utc::now()
    .checked_add_signed(
      Duration::milliseconds(
        if is_refresh {
          config.auth.jwt_validity_rk.unwrap()
        } else {
          config.auth.jwt_validity_ak.unwrap()
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
