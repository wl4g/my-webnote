use sqlx::{ FromRow, sqlite::SqliteRow, Row };
use serde::{ Deserialize, Serialize };
use validator::Validate;

use super::{ BaseBean, PageResponse };

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct Settings {
    #[serde(flatten)]
    pub base: BaseBean,
    pub name: Option<String>,
}

impl<'r> FromRow<'r, SqliteRow> for Settings {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Settings {
            base: BaseBean::from_row(row).unwrap(),
            name: row.try_get("name")?,
        })
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq, Validate, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct QuerySettingsRequest {
    #[validate(length(min = 1, max = 64))]
    pub name: Option<String>,
}

impl QuerySettingsRequest {
    pub fn to_settings(&self) -> Settings {
        Settings {
            base: BaseBean::new(None, None, None),
            name: Some(self.name.clone().unwrap_or_default()),
        }
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct QuerySettingsResponse {
    pub page: Option<PageResponse>,
    pub data: Option<Vec<Settings>>,
}

impl QuerySettingsResponse {
    pub fn new(page: PageResponse, data: Vec<Settings>) -> Self {
        QuerySettingsResponse { page: Some(page), data: Some(data) }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq, Validate, utoipa::ToSchema)]
pub struct SaveSettingsRequest {
    pub id: Option<i64>,
    #[validate(length(min = 1, max = 64))]
    pub name: Option<String>,
}

impl SaveSettingsRequest {
    pub fn to_settings(&self) -> Settings {
        Settings {
            base: BaseBean::new_default(self.id),
            name: self.name.clone(),
        }
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct SaveSettingsResponse {
    pub id: i64,
}

impl SaveSettingsResponse {
    pub fn new(id: i64) -> Self {
        SaveSettingsResponse { id }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq, Validate, utoipa::ToSchema)]
pub struct DeleteSettingsRequest {
    pub id: i64,
}

#[derive(Serialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct DeleteSettingsResponse {
    pub count: u64,
}

impl DeleteSettingsResponse {
    pub fn new(count: u64) -> Self {
        DeleteSettingsResponse { count }
    }
}
