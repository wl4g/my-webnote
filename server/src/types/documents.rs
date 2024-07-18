use sqlx::{ FromRow, sqlite::SqliteRow, Row };
use serde::{ Deserialize, Serialize };
use validator::Validate;

use super::{ BaseBean, PageResponse };

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct Document {
    #[serde(flatten)]
    pub base: BaseBean,
    pub name: Option<String>,
}

impl<'r> FromRow<'r, SqliteRow> for Document {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Document {
            base: BaseBean::from_row(row).unwrap(),
            name: row.try_get("name")?,
        })
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq, Validate, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct QueryDocumentRequest {
    #[validate(length(min = 1, max = 64))]
    pub name: Option<String>,
}

impl QueryDocumentRequest {
    pub fn to_document(&self) -> Document {
        Document {
            base: BaseBean::new(None, None, None),
            name: Some(self.name.clone().unwrap_or_default()),
        }
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct QueryDocumentResponse {
    pub page: Option<PageResponse>,
    pub data: Option<Vec<Document>>,
}

impl QueryDocumentResponse {
    pub fn new(page: PageResponse, data: Vec<Document>) -> Self {
        QueryDocumentResponse { page: Some(page), data: Some(data) }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq, Validate, utoipa::ToSchema)]
pub struct SaveDocumentRequest {
    pub id: Option<i64>,
    #[validate(length(min = 1, max = 64))]
    pub name: Option<String>,
}

impl SaveDocumentRequest {
    pub fn to_document(&self) -> Document {
        Document {
            base: BaseBean::new_default(self.id),
            name: self.name.clone(),
        }
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct SaveDocumentResponse {
    pub id: i64,
}

impl SaveDocumentResponse {
    pub fn new(id: i64) -> Self {
        SaveDocumentResponse { id }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq, Validate, utoipa::ToSchema)]
pub struct DeleteDocumentRequest {
    pub id: i64,
}

#[derive(Serialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct DeleteDocumentResponse {
    pub count: u64,
}

impl DeleteDocumentResponse {
    pub fn new(count: u64) -> Self {
        DeleteDocumentResponse { count }
    }
}
