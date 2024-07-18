pub mod api_v1;
pub mod auths;
pub mod documents;
pub mod folders;
pub mod settings;
pub mod users;

use serde::{ Deserialize, Serialize };
use chrono::Utc;
use sqlx::prelude::FromRow;
use validator::Validate;

use crate::utils::snowflake::SnowflakeIdGenerator;
// use sqlx::{ Decode, FromRow };

pub static DEFAULT_BY: &'static str = "0";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, FromRow, utoipa::ToSchema)]
pub struct BaseBean {
    pub id: Option<i64>,
    pub status: Option<i8>,
    #[schema(read_only)]
    pub create_by: Option<String>,
    #[sqlx(rename = "create_time")]
    #[schema(read_only)]
    pub create_time: Option<i64>,
    #[schema(read_only)]
    pub update_by: Option<String>,
    #[schema(read_only)]
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
            id: id,
            status: Some(0),
            create_by: create_by,
            create_time: Some(now),
            update_by: update_by,
            update_time: Some(now),
            del_flag: Some(0),
        }
    }

    pub fn pre_insert(&mut self, create_by: Option<String>) -> i64 {
        self.id = Some(SnowflakeIdGenerator::default_next_jssafe());
        self.create_by = create_by;
        self.create_time = Some(Utc::now().timestamp_millis());
        self.del_flag = Some(0);
        self.id.unwrap()
    }

    pub fn pre_update(&mut self, update_by: Option<String>) {
        self.update_by = update_by;
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
