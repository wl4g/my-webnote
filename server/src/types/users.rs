use serde::{ Deserialize, Serialize };
// use sqlx::{ Decode, FromRow };
use sqlx::{ FromRow, sqlite::SqliteRow, Row };

use super::BaseBean;

// #[derive(Serialize, Deserialize, Clone, Debug, FromRow, Decode)]
#[derive(Serialize, Deserialize, Clone, Debug, utoipa::ToSchema)]
pub struct User {
  #[serde(flatten)]
  pub base: BaseBean,
  pub name: Option<String>,
  pub email: Option<String>,
  pub phone: Option<String>,
  pub password: Option<String>,
  pub oidc_claims_sub: Option<String>,
  pub oidc_claims_name: Option<String>,
  pub github_claims_sub: Option<String>,
  pub github_claims_name: Option<String>,
  pub google_claims_sub: Option<String>,
  pub google_claims_name: Option<String>,
}

impl<'r> FromRow<'r, SqliteRow> for User {
  fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
    Ok(User {
      base: BaseBean::from_row(row).unwrap(),
      name: row.try_get("name")?,
      email: row.try_get("email")?,
      phone: row.try_get("phone")?,
      password: row.try_get("password")?,
      oidc_claims_sub: row.try_get("oidc_claims_sub")?,
      oidc_claims_name: row.try_get("oidc_claims_name")?,
      github_claims_sub: row.try_get("github_claims_sub")?,
      github_claims_name: row.try_get("github_claims_name")?,
      google_claims_sub: row.try_get("google_claims_sub")?,
      google_claims_name: row.try_get("google_claims_name")?,
    })
  }
}

#[derive(Deserialize, Clone, Debug, utoipa::ToSchema)]
pub struct QueryUserRequest {
  pub name: Option<String>,
  pub email: Option<String>,
  pub phone: Option<String>,
}

impl QueryUserRequest {
  pub fn to_user(&self) -> User {
    User {
      base: BaseBean::new(None, None, None),
      name: Some(self.name.clone().unwrap_or_default()),
      email: Some(self.email.clone().unwrap_or_default()),
      phone: self.phone.clone(),
      password: None,
      oidc_claims_sub: None,
      oidc_claims_name: None,
      github_claims_sub: None,
      github_claims_name: None,
      google_claims_sub: None,
      google_claims_name: None,
    }
  }
}

#[derive(Serialize, Clone, Debug, utoipa::ToSchema)]
pub struct QueryUserResponse {
  users: Vec<User>,
}

impl QueryUserResponse {
  pub fn new(users: Vec<User>) -> Self {
    QueryUserResponse { users }
  }
}

#[derive(Deserialize, Clone, Debug, utoipa::ToSchema)]
pub struct SaveUserRequest {
  pub id: Option<i64>,
  pub name: Option<String>,
  pub email: Option<String>,
  pub phone: Option<String>,
  pub password: Option<String>,
  pub oidc_claims_sub: Option<String>,
  pub oidc_claims_name: Option<String>,
  pub github_claims_sub: Option<String>,
  pub github_claims_name: Option<String>,
  pub google_claims_sub: Option<String>,
  pub google_claims_name: Option<String>,
}

impl SaveUserRequest {
  pub fn to_user(&self) -> User {
    User {
      base: BaseBean::new_default(self.id),
      // name: self.name.as_ref().map(|n| n.to_string()),
      name: self.name.clone(),
      email: self.email.clone(),
      phone: self.phone.clone(),
      password: self.password.clone(),
      oidc_claims_sub: self.oidc_claims_sub.clone(),
      oidc_claims_name: self.oidc_claims_name.clone(),
      github_claims_sub: self.github_claims_sub.clone(),
      github_claims_name: self.github_claims_name.clone(),
      google_claims_sub: self.google_claims_sub.clone(),
      google_claims_name: self.google_claims_name.clone(),
    }
  }
}

#[derive(Serialize, Clone, Debug, utoipa::ToSchema)]
pub struct SaveUserResponse {
  pub id: i64,
}

#[derive(Deserialize, Clone, Debug, utoipa::ToSchema)]
pub struct DeleteUserRequest {
  pub id: i64,
}

#[derive(Serialize, Clone, Debug, utoipa::ToSchema)]
pub struct DeleteUserResponse {
  pub count: u64,
}
