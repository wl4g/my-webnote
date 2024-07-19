use common_makestruct_macro::MakeStructWith;
// use common_smartcpy_macro::SmartCopy; // TODO: compile error
use sqlx::{ FromRow, sqlite::SqliteRow, Row };
use serde::{ Deserialize, Serialize };
use validator::Validate;

use super::{ BaseBean, PageResponse };

// Manual impl for decode.
// #[derive(Serialize, Deserialize, Clone, Debug, sqlx::sqlite::FromRow, sqlx::sqlite::Decode)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct User {
    #[serde(flatten)]
    pub base: BaseBean,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub oidc_claims_sub: Option<String>,
    pub oidc_claims_name: Option<String>,
    pub oidc_claims_email: Option<String>,
    pub github_claims_sub: Option<String>,
    pub github_claims_name: Option<String>,
    pub github_claims_email: Option<String>,
    pub google_claims_sub: Option<String>,
    pub google_claims_name: Option<String>,
    pub google_claims_email: Option<String>,
    pub lang: Option<String>,
}

impl Default for User {
    fn default() -> Self {
        User {
            base: BaseBean::new_default(None),
            name: None,
            email: None,
            phone: None,
            password: None,
            oidc_claims_sub: None,
            oidc_claims_name: None,
            oidc_claims_email: None,
            github_claims_sub: None,
            github_claims_name: None,
            github_claims_email: None,
            google_claims_sub: None,
            google_claims_name: None,
            google_claims_email: None,
            lang: None,
        }
    }
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
            oidc_claims_email: row.try_get("oidc_claims_email")?,
            github_claims_sub: row.try_get("github_claims_sub")?,
            github_claims_name: row.try_get("github_claims_name")?,
            github_claims_email: row.try_get("github_claims_email")?,
            google_claims_sub: row.try_get("google_claims_sub")?,
            google_claims_name: row.try_get("google_claims_name")?,
            google_claims_email: row.try_get("google_claims_email")?,
            lang: row.try_get("lang")?,
        })
    }
}

#[derive(
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Validate,
    utoipa::ToSchema,
    utoipa::IntoParams
    // PageableQueryRequest // Try using macro auto generated pageable query request.
)]
#[into_params(parameter_in = Query)]
pub struct QueryUserRequest {
    // #[serde(flatten)]
    // #[serde(default)]
    // #[serde(skip)]
    // #[param(style = Form)]
    // #[param(value_type=Option<String>)]
    // pub page: Option<super::PageRequest>, // It is difficult to pass parameters using http get/query when nested structures.
    #[validate(length(min = 1, max = 64))]
    pub name: Option<String>,
    #[validate(email)]
    #[validate(length(min = 1, max = 64))]
    pub email: Option<String>,
    #[validate(length(min = 1, max = 15))]
    pub phone: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub oidc_claims_sub: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub oidc_claims_name: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub oidc_claims_email: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub github_claims_sub: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub github_claims_name: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub github_claims_email: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub google_claims_sub: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub google_claims_name: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub google_claims_email: Option<String>,
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
            oidc_claims_email: None,
            github_claims_sub: None,
            github_claims_name: None,
            github_claims_email: None,
            google_claims_sub: None,
            google_claims_name: None,
            google_claims_email: None,
            lang: None,
        }
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct QueryUserResponse {
    pub page: Option<PageResponse>,
    pub data: Option<Vec<User>>,
}

impl QueryUserResponse {
    pub fn new(page: PageResponse, data: Vec<User>) -> Self {
        QueryUserResponse { page: Some(page), data: Some(data) }
    }
}

#[derive(
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Validate,
    utoipa::ToSchema,
    MakeStructWith
    // SmartCopy
)]
#[excludes(id)]
// #[smart_copy(target = "SaveUserRequestWith")]
pub struct SaveUserRequest {
    pub id: Option<i64>,
    #[validate(length(min = 1, max = 64))]
    pub name: Option<String>,
    #[validate(email)]
    #[validate(length(min = 1, max = 64))]
    pub email: Option<String>,
    #[validate(length(min = 1, max = 15))]
    pub phone: Option<String>,
    #[validate(length(min = 1, max = 512))]
    pub password: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub oidc_claims_sub: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub oidc_claims_name: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub oidc_claims_email: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub github_claims_sub: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub github_claims_name: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub github_claims_email: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub google_claims_sub: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub google_claims_name: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub google_claims_email: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub lang: Option<String>,
}

impl SaveUserRequest {
    pub fn to_user(&self) -> User {
        User {
            base: BaseBean::new_default(self.id),
            name: self.name.clone(), // self.name.as_ref().map(|n| n.to_string())
            email: self.email.clone(),
            phone: self.phone.clone(),
            password: self.password.clone(),
            oidc_claims_sub: self.oidc_claims_sub.clone(),
            oidc_claims_name: self.oidc_claims_name.clone(),
            oidc_claims_email: self.oidc_claims_email.clone(),
            github_claims_sub: self.github_claims_sub.clone(),
            github_claims_name: self.github_claims_name.clone(),
            github_claims_email: self.github_claims_email.clone(),
            google_claims_sub: self.google_claims_sub.clone(),
            google_claims_name: self.google_claims_name.clone(),
            google_claims_email: self.google_claims_email.clone(),
            lang: self.lang.clone(),
        }
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct SaveUserResponse {
    pub id: i64,
}

impl SaveUserResponse {
    pub fn new(id: i64) -> Self {
        SaveUserResponse { id }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq, Validate, utoipa::ToSchema)]
pub struct DeleteUserRequest {
    pub id: i64,
}

#[derive(Serialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct DeleteUserResponse {
    pub count: u64,
}

impl DeleteUserResponse {
    pub fn new(count: u64) -> Self {
        DeleteUserResponse { count }
    }
}
