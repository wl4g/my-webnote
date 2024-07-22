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

pub mod api_v1;
pub mod auths;
pub mod documents;
pub mod folders;
pub mod settings;
pub mod users;

use anyhow::Error;
use hyper::StatusCode;
use serde::{ Deserialize, Serialize };
use chrono::Utc;
use sqlx::prelude::FromRow;
use validator::Validate;

use crate::utils::{ auths::SecurityContext, snowflake::SnowflakeIdGenerator };
// use sqlx::{ Decode, FromRow };

pub static DEFAULT_BY: &'static str = "0";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, FromRow, utoipa::ToSchema)]
pub struct BaseBean {
    #[schema(rename = "id")]
    pub id: Option<i64>,
    #[schema(rename = "status")]
    pub status: Option<i8>,
    #[sqlx(rename = "create_by")]
    #[schema(read_only = true)]
    // Notice: Since we are currently using serde serialization to implement custom ORM,
    // the #[serde(rename=xx)] rename will not only take effect on the restful APIs but
    // also on the DB, so for simplicity, we will unifed use underscores.
    //#[serde(rename = "createBy")]
    pub create_by: Option<String>,
    #[schema(read_only = true)]
    pub create_time: Option<i64>,
    #[schema(read_only = true)]
    pub update_by: Option<String>,
    #[schema(read_only = true)]
    pub update_time: Option<i64>,
    #[serde(skip)]
    pub del_flag: Option<i32>,
}

impl BaseBean {
    pub fn new_default(id: Option<i64>) -> Self {
        // Self::new(Some(DEFAULT_BY.to_string()), Some(DEFAULT_BY.to_string()))
        Self::new(id, None, None)
    }

    pub fn new(id: Option<i64>, create_by: Option<String>, update_by: Option<String>) -> Self {
        let now = Utc::now().timestamp_millis();
        Self {
            id,
            status: Some(0),
            create_by,
            create_time: Some(now),
            update_by,
            update_time: Some(now),
            del_flag: Some(0),
        }
    }

    pub async fn pre_insert(&mut self, create_by: Option<String>) -> i64 {
        let by = create_by
            .or(SecurityContext::get_instance().get_current_email().await)
            .or(SecurityContext::get_instance().get_current_uname().await)
            .or(Some(DEFAULT_BY.to_string()));

        self.id = Some(SnowflakeIdGenerator::default_next_jssafe());
        self.create_by = by;
        self.create_time = Some(Utc::now().timestamp_millis());
        self.del_flag = Some(0);
        self.id.unwrap()
    }

    pub async fn pre_update(&mut self, update_by: Option<String>) {
        let by = update_by
            .or(SecurityContext::get_instance().get_current_email().await)
            .or(SecurityContext::get_instance().get_current_uname().await)
            .or(Some(DEFAULT_BY.to_string()));

        self.update_by = by;
        self.update_time = Some(Utc::now().timestamp_millis());
        self.del_flag = Some(0);
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq, Validate, utoipa::ToSchema, utoipa::IntoParams)]
pub struct PageRequest {
    #[schema(example = "1")]
    #[validate(range(min = 1, max = 1000))]
    pub num: Option<u32>, // page number.
    #[schema(example = "10")]
    #[validate(range(min = 1, max = 1000))]
    pub limit: Option<u32>, // The per page records count.
    // For large data of fast-queries cached condition acceleration.
    // pub cached_forward_last_min_id: Option<i64>,
    // pub cached_backend_last_max_id: Option<i64>,
}

impl PageRequest {
    pub fn default() -> PageRequest {
        PageRequest {
            num: Some(1),
            limit: Some(10),
            // cached_forward_last_min_id: None,
            // cached_backend_last_max_id: None,
        }
    }
    pub fn get_offset(&self) -> u32 {
        let n = self.num.unwrap_or(1);
        if n < 1 {
            1
        } else {
            (n - 1) * self.get_limit()
        }
    }

    pub fn get_limit(&self) -> u32 {
        let l = self.limit.unwrap_or(10);
        if l < 1 {
            1
        } else {
            l
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, utoipa::ToSchema)]
pub struct PageResponse {
    pub total: Option<i64>, // The current conditions snapshot data of total records count.
    pub num: Option<u32>, // page number.
    pub limit: Option<u32>, // The per page records count.
    // For large data of fast-queries cached condition acceleration.
    // pub cached_forward_last_min_id: Option<i64>,
    // pub cached_backend_last_max_id: Option<i64>,
}

impl PageResponse {
    pub fn new(total: Option<i64>, num: Option<u32>, limit: Option<u32>) -> Self {
        Self {
            total: total,
            num: num,
            limit,
        }
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, Validate, utoipa::ToSchema, utoipa::IntoParams)]
pub(crate) struct RespBase {
    pub(crate) errcode: Option<i8>,
    pub(crate) errmsg: Option<String>,
}

impl RespBase {
    pub(crate) fn success() -> Self {
        Self {
            errcode: Some(0),
            errmsg: Some("ok".to_string()),
        }
    }

    pub(crate) fn error(e: Error) -> Self {
        Self {
            errcode: Some(StatusCode::INTERNAL_SERVER_ERROR.as_u16() as i8),
            errmsg: Some(e.to_string()),
        }
    }

    #[allow(unused)]
    pub(crate) fn errmsg(errmsg: &str) -> Self {
        Self {
            errcode: Some(StatusCode::INTERNAL_SERVER_ERROR.as_u16() as i8),
            errmsg: Some(errmsg.to_owned()),
        }
    }

    pub(crate) fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
