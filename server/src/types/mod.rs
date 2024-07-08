pub mod documents;
pub mod folders;
pub mod settings;
pub mod users;

use serde::{ Deserialize, Serialize };
use chrono::Utc;
use sqlx::prelude::FromRow;

use crate::utils::snowflake::SnowflakeIdGenerator;
// use sqlx::{ Decode, FromRow };

pub static DEFAULT_BY: &'static str = "unknown";

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
      create_time: Some(now),
      update_time: Some(now),
      create_by: create_by,
      update_by: update_by,
      del_flag: Some(0),
    }
  }

  pub fn pre_insert(&mut self, create_by: Option<String>) -> i64 {
    self.id = Some(SnowflakeIdGenerator::default_next_jssafe());
    self.create_time = Some(Utc::now().timestamp_millis());
    self.create_by = create_by;
    self.del_flag = Some(0);
    self.id.unwrap()
  }

  pub fn pre_update(&mut self, update_by: Option<String>) {
    self.update_time = Some(Utc::now().timestamp_millis());
    self.update_by = update_by;
    self.del_flag = Some(0);
  }
}
