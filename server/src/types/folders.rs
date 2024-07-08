use serde::{ Deserialize, Serialize };
use super::BaseBean;

#[derive(Serialize, Deserialize, Clone)]
pub struct Folder {
  #[serde(flatten)]
  pub base: Option<BaseBean>,
  pub name: String,
  pub email: String,
  pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct QueryFolderRequest {
  pub name: Option<String>,
  pub email: Option<String>,
}

#[derive(Serialize)]
pub struct QueryFolderResponse {
  users: Vec<Folder>,
}

#[derive(Deserialize)]
pub struct SaveFolderRequest {
  name: String,
  email: String,
  password: String,
}

#[derive(Serialize)]
pub struct SaveFolderResponse {
  user: Folder,
}

#[derive(Deserialize)]
pub struct DeleteFolderRequest {
  id: String,
}

#[derive(Serialize)]
pub struct DeleteFolderResponse {
  id: String,
}
