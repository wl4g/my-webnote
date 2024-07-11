use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, utoipa::ToSchema)]
pub struct CallbackGithubRequest {
  pub code: Option<String>,
}

#[derive(Deserialize, Clone, Debug, utoipa::ToSchema)]
pub struct CallbackOidcRequest {
  pub code: Option<String>,
}

#[derive(Deserialize, Clone, Debug, utoipa::ToSchema)]
pub struct LogoutRequest {
  //pub auth_type: Option<String>,
  pub access_token: Option<String>,
  pub refresh_token: Option<String>,
}
