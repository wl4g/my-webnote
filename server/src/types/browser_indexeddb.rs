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

use serde::{ Deserialize, Serialize };
use sqlx::{ FromRow, sqlite::SqliteRow, Row };
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct IndexedValue {
    pub value: Option<String>,
}

impl<'r> FromRow<'r, SqliteRow> for IndexedValue {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(IndexedValue {
            // key: row.try_get("key")?,
            value: row.try_get("value")?,
        })
    }
}

// get

#[derive(Deserialize, Clone, Debug, PartialEq, Validate, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetIndexedRecordRequest {
    #[serde(rename = "storeName")]
    #[validate(length(min = 1, max = 64))]
    pub store_name: String,
    #[validate(length(min = 1, max = 64))]
    pub key: Option<String>,
}

#[derive(Serialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct GetIndexedRecordResponse {
    pub record: Option<IndexedValue>,
}

impl GetIndexedRecordResponse {
    pub fn new(record: Option<IndexedValue>) -> Self {
        GetIndexedRecordResponse { record }
    }
}

// get_all

#[derive(Deserialize, Clone, Debug, PartialEq, Validate, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetAllIndexedRecordRequest {
    #[serde(rename = "storeName")]
    #[validate(length(min = 1, max = 64))]
    pub store_name: String,
}

#[derive(Serialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct GetAllIndexedRecordResponse {
    pub records: Option<Vec<IndexedValue>>,
}

impl GetAllIndexedRecordResponse {
    pub fn new(records: Option<Vec<IndexedValue>>) -> Self {
        GetAllIndexedRecordResponse { records }
    }
}

// get_all_keys

#[derive(Deserialize, Clone, Debug, PartialEq, Validate, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetAllKeysIndexedRecordRequest {
    #[serde(rename = "storeName")]
    #[validate(length(min = 1, max = 64))]
    pub store_name: String,
}

#[derive(Serialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct GetAllKeysIndexedRecordResponse {
    pub records: Option<Vec<String>>,
}

impl GetAllKeysIndexedRecordResponse {
    pub fn new(records: Option<Vec<String>>) -> Self {
        GetAllKeysIndexedRecordResponse { records }
    }
}

// add,put

#[derive(Deserialize, Clone, Debug, PartialEq, Validate, utoipa::ToSchema)]
pub struct SaveIndexedRecordRequest {
    #[serde(rename = "storeName")]
    #[validate(length(min = 1, max = 64))]
    pub store_name: String,
    #[validate(length(min = 0, max = 65535))]
    pub value: String,
    #[validate(length(min = 1, max = 64))]
    pub key: Option<String>,
}

#[derive(Serialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct SaveIndexedRecordResponse {
    pub key: String,
}

impl SaveIndexedRecordResponse {
    pub fn new(key: String) -> Self {
        SaveIndexedRecordResponse { key }
    }
}

// delete

#[derive(Deserialize, Clone, Debug, PartialEq, Validate, utoipa::ToSchema)]
pub struct DeleteIndexedRecordRequest {
    #[serde(rename = "storeName")]
    #[validate(length(min = 1, max = 64))]
    pub store_name: String,
    #[validate(length(min = 1, max = 64))]
    pub key: String,
}

#[derive(Serialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct DeleteIndexedRecordResponse {
    pub count: u32,
}

impl DeleteIndexedRecordResponse {
    pub fn new(count: u32) -> Self {
        DeleteIndexedRecordResponse { count }
    }
}
