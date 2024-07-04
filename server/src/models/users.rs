use serde::{ Deserialize, Serialize };
use super::BaseBean;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
  #[serde(flatten)]
  pub base: Option<BaseBean>,
  pub name: String,
  pub email: String,
  pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct QueryUserRequest {
  pub name: Option<String>,
  pub email: Option<String>,
}

#[derive(Serialize)]
pub struct QueryUserResponse {
  users: Vec<User>,
}

#[derive(Deserialize)]
pub struct SaveUserRequest {
  name: String,
  email: String,
  password: String,
}

#[derive(Serialize)]
pub struct SaveUserResponse {
  user: User,
}

#[derive(Deserialize)]
pub struct DeleteUserRequest {
  id: String,
}

#[derive(Serialize)]
pub struct DeleteUserResponse {
  id: String,
}
