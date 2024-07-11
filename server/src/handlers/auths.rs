use anyhow::Error;
use crate::{ context::state::AppState, types::auths::LogoutRequest };

pub struct AuthHandler<'a> {
  state: &'a AppState,
}

impl<'a> AuthHandler<'a> {
  pub fn new(state: &'a AppState) -> Self {
    Self { state }
  }

  pub async fn logout(&self, param: LogoutRequest) -> Result<(), Error> {
    // TODO

    // 1. Add current jwt token to cache blacklist, expiration time is less than now time - id_token issue time.
    // 2. logging

    todo!()
  }
}
