pub mod documents;
pub mod folders;
pub mod settings;
pub mod users;

use serde::{ Deserialize, Serialize };
use chrono::Utc;
use sqlx::prelude::FromRow;

use crate::utils::snowflake::SnowflakeIdGenerator;
// use sqlx::{ Decode, FromRow };

pub static DEFAULT_BY: &'static str = "0";

#[derive(Serialize, Deserialize, Clone, Debug, FromRow, utoipa::ToSchema)]
pub struct BaseBean {
  pub id: Option<i64>,
  pub status: Option<i8>,
  pub create_by: Option<String>,
  #[sqlx(rename = "create_time")]
  pub create_time: Option<i64>,
  pub update_by: Option<String>,
  pub update_time: Option<i64>,
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

#[derive(Deserialize, Clone, Debug, utoipa::ToSchema)]
pub struct PageRequest {
  pub num: Option<i32>, // page number.
  pub limit: Option<i32>, // The per page records count.
  // For large data of fast-queries cached condition acceleration.
  // pub cached_forward_last_min_id: Option<i64>,
  // pub cached_backend_last_max_id: Option<i64>,
}

impl PageRequest {
  pub fn get_offset(&self) -> i32 {
    let n = self.num.unwrap_or(1);
    if n <= 0 {
      0
    } else {
      (n - 1) * self.get_limit()
    }
  }

  pub fn get_limit(&self) -> i32 {
    let l = self.limit.unwrap_or(10);
    if l <= 0 {
      0
    } else {
      l
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, utoipa::ToSchema)]
pub struct PageResponse {
  pub total: Option<i64>, // The current conditions snapshot data of total records count.
  pub num: Option<i32>, // page number.
  pub limit: Option<i32>, // The per page records count.
  // For large data of fast-queries cached condition acceleration.
  // pub cached_forward_last_min_id: Option<i64>,
  // pub cached_backend_last_max_id: Option<i64>,
}

impl PageResponse {
  pub fn new(total: Option<i64>, num: Option<i32>, limit: Option<i32>) -> Self {
    Self {
      total: total,
      num: num,
      limit,
    }
  }
}
