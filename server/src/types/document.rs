/*
 * SPDX-License-Identifier: GNU GENERAL PUBLIC LICENSE Version 3
 *
 * Copyleft (c) 2024 James Wong. This file is part of James Wong.
 * is free software: you can redistribute it and/or modify it under
 * the terms of the GNU General Public License as published by the
 * Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * James Wong is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with James Wong.  If not, see <https://www.gnu.org/licenses/>.
 *
 * IMPORTANT: Any software that fully or partially contains or uses materials
 * covered by this license must also be released under the GNU GPL license.
 * This includes modifications and derived works.
 */

use sqlx::{ FromRow, sqlite::SqliteRow, Row };
use serde::{ Deserialize, Serialize };
use validator::Validate;

use super::{ BaseBean, PageResponse };

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct Document {
    #[serde(flatten)]
    pub base: BaseBean,
    pub key: Option<String>,
    pub name: Option<String>,
    pub folder_key: Option<String>,
    // Notice:
    // 1. (SQLite) Because the ORM library is not used for the time being, the fields are dynamically
    // parsed based on serde_json, so the #[serde(rename="xx")] annotation is effective.
    // 2. (MongoDB) The underlying BSON serialization is also based on serde, so using #[serde(rename="xx")] is also valid
    // TODO: It is recommended to use an ORM framework, see: https://github.com/diesel-rs/diesel
    #[serde(rename = "type")]
    pub doc_type: Option<DocumentType>,
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub enum DocumentType {
    Board,
    Note,
}

// The Beautiful transformation from string to enum
impl TryFrom<String> for DocumentType {
    type Error = sqlx::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "board" => Ok(DocumentType::Board),
            "note" => Ok(DocumentType::Note),
            _ => Err(sqlx::Error::ColumnNotFound("Invalid document type".into())),
        }
    }
}

impl<'r> FromRow<'r, SqliteRow> for Document {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Document {
            base: BaseBean::from_row(row).unwrap(),
            key: row.try_get("key")?,
            name: row.try_get("name")?,
            folder_key: row.try_get("folder_key")?,
            doc_type: Some(DocumentType::try_from(row.try_get::<String, _>("type")?)?),
            content: row.try_get("content")?,
        })
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq, Validate, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct QueryDocumentRequest {
    #[validate(length(min = 1, max = 64))]
    pub key: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub name: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub folder_key: Option<String>,
    #[serde(rename = "type")]
    pub doc_type: Option<DocumentType>,
}

impl QueryDocumentRequest {
    pub fn to_document(&self) -> Document {
        Document {
            base: BaseBean::new(None, None, None),
            key: Some(self.key.to_owned().unwrap_or_default()),
            name: Some(self.name.to_owned().unwrap_or_default()),
            folder_key: Some(self.folder_key.to_owned().unwrap_or_default()),
            doc_type: self.doc_type.to_owned(),
            content: None,
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
    pub key: Option<String>,
    #[validate(length(min = 0, max = 64))]
    pub name: Option<String>,
    #[validate(length(min = 0, max = 64))]
    #[serde(rename = "folderKey")]
    pub folder_key: Option<String>,
    #[serde(rename = "type")]
    pub doc_type: Option<DocumentType>,
    #[validate(length(min = 0, max = 8192))]
    pub content: Option<String>,
}

impl SaveDocumentRequest {
    pub fn to_document(&self) -> Document {
        Document {
            base: BaseBean::new_default(self.id),
            key: self.key.to_owned(),
            name: self.name.to_owned(),
            folder_key: self.folder_key.to_owned(),
            doc_type: self.doc_type.to_owned(),
            content: self.content.to_owned(),
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
