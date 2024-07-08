use serde::{ Deserialize, Serialize };
// use sqlx::{ Decode, FromRow };
use sqlx::{ FromRow, sqlite::SqliteRow, Row };

use super::BaseBean;

// #[derive(Serialize, Deserialize, Clone, Debug, FromRow, Decode)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
  #[serde(flatten)]
  pub base: BaseBean,
  pub name: String,
  pub email: String,
  pub password: Option<String>,
}

impl<'r> FromRow<'r, SqliteRow> for User {
  fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
    Ok(User {
      base: BaseBean::from_row(row).unwrap(),
      name: row.try_get("name")?,
      email: row.try_get("email")?,
      password: row.try_get("password")?,
    })
  }
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
