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
pub struct Folder {
    #[serde(flatten)]
    pub base: BaseBean,
    pub pid: Option<i64>,
    pub key: Option<String>,
    pub name: Option<String>,
}

impl<'r> FromRow<'r, SqliteRow> for Folder {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Folder {
            base: BaseBean::from_row(row).unwrap(),
            pid: row.try_get("pid")?,
            key: row.try_get("key")?,
            name: row.try_get("name")?,
        })
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq, Validate, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct QueryFolderRequest {
    pub pid: Option<i64>,
    #[validate(length(min = 1, max = 64))]
    pub key: Option<String>,
    pub name: Option<String>,
}

impl QueryFolderRequest {
    pub fn to_folder(&self) -> Folder {
        Folder {
            base: BaseBean::new(None, None, None),
            pid: Some(self.pid.clone().unwrap_or_default()),
            key: Some(self.key.clone().unwrap_or_default()),
            name: Some(self.name.clone().unwrap_or_default()),
        }
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct QueryFolderResponse {
    pub page: Option<PageResponse>,
    pub data: Option<Vec<Folder>>,
}

// pub struct FolderWrapper(Folder); // add field childern and support transform to tree json.

impl QueryFolderResponse {
    pub fn new(page: PageResponse, data: Vec<Folder>) -> Self {
        QueryFolderResponse { page: Some(page), data: Some(data) }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq, Validate, utoipa::ToSchema)]
pub struct SaveFolderRequest {
    pub id: Option<i64>,
    pub pid: Option<i64>,
    #[validate(length(min = 1, max = 64))]
    pub key: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub name: Option<String>,
}

impl SaveFolderRequest {
    pub fn to_folder(&self) -> Folder {
        Folder {
            base: BaseBean::new_default(self.id),
            pid: self.pid,
            key: self.key.clone(),
            name: self.name.clone(),
        }
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct SaveFolderResponse {
    pub id: i64,
}

impl SaveFolderResponse {
    pub fn new(id: i64) -> Self {
        SaveFolderResponse { id }
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq, Validate, utoipa::ToSchema)]
pub struct DeleteFolderRequest {
    pub id: i64,
}

#[derive(Serialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct DeleteFolderResponse {
    pub count: u64,
}

impl DeleteFolderResponse {
    pub fn new(count: u64) -> Self {
        DeleteFolderResponse { count }
    }
}
