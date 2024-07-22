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
