use oauth2::{ basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl };

use crate::config::config_api::OAuth2Config;

// Using unified abstraction as OAuth2Config base class.
pub async fn create_oauth2_client(oauth2_config: &OAuth2Config) -> Option<BasicClient> {
  if oauth2_config.enabled.unwrap_or(false) {
    Some(
      BasicClient::new(
        ClientId::new(oauth2_config.client_id.as_ref().unwrap().clone()),
        Some(ClientSecret::new(oauth2_config.client_secret.as_ref().unwrap().clone())),
        AuthUrl::new(oauth2_config.auth_url.as_ref().unwrap().clone()).unwrap(),
        Some(TokenUrl::new(oauth2_config.token_url.as_ref().unwrap().clone()).unwrap())
      ).set_redirect_uri(
        RedirectUrl::new(oauth2_config.redirect_url.as_ref().unwrap().clone()).unwrap()
      )
    )
  } else {
    None
  }
}
