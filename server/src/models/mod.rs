pub mod documents;
pub mod folders;
pub mod settings;
pub mod users;

use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, Utc };
use sqlx::prelude::FromRow;
// use sqlx::{ Decode, FromRow };

static DEFAULT_BY: &'static str = "unknown";

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct BaseBean {
  pub id: Option<i64>,
  #[sqlx(rename = "create_time")]
  pub create_time: Option<DateTime<Utc>>,
  pub update_time: Option<DateTime<Utc>>,
  pub create_by: Option<String>,
  pub update_by: Option<String>,
  pub del_flag: Option<i32>,
}

impl BaseBean {
  pub fn new_default() -> Self {
    Self::new(Some(DEFAULT_BY.to_string()), Some(DEFAULT_BY.to_string()))
  }

  pub fn new(create_by: Option<String>, update_by: Option<String>) -> Self {
    Self {
      //id: Some(uuid::Uuid::new_v4().to_string()),
      id: Some(1001_i64), // TODO not implements, distributed generator next
      create_time: Some(Utc::now()),
      update_time: Some(Utc::now()),
      create_by: create_by,
      update_by: update_by,
      del_flag: Some(0),
    }
  }

  pub fn pre_update(&mut self, update_by: Option<String>) {
    self.update_time = Some(Utc::now());
    self.update_by = update_by;
  }
}
