use serde::{ Deserialize, Serialize };
use super::BaseBean;

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
  #[serde(flatten)]
  pub base: Option<BaseBean>,
  pub name: String,
  pub email: String,
  pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct QuerySettingsRequest {
  pub name: Option<String>,
  pub email: Option<String>,
}

#[derive(Serialize)]
pub struct QuerySettingsResponse {
  users: Vec<Settings>,
}

#[derive(Deserialize)]
pub struct SaveSettingsRequest {
  name: String,
  email: String,
  password: String,
}

#[derive(Serialize)]
pub struct SaveSettingsResponse {
  user: Settings,
}

#[derive(Deserialize)]
pub struct DeleteSettingsRequest {
  id: String,
}

#[derive(Serialize)]
pub struct DeleteSettingsResponse {
  id: String,
}
