pub mod documents;
pub mod folders;
pub mod settings;
pub mod users;

use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, Utc };
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct BaseBean {
  id: String,
  create_time: DateTime<Utc>,
  update_time: DateTime<Utc>,
  create_by: String,
  update_by: String,
  del_flag: i32,
}

#[allow(dead_code)]
impl BaseBean {
  pub fn new() -> Self {
    Self {
      id: Uuid::new_v4().to_string(),
      create_time: Utc::now(),
      update_time: Utc::now(),
      create_by: "admin".to_string(), // TODO: 实现真实的用户认证
      update_by: "admin".to_string(), // TODO: 实现真实的用户认证
      del_flag: 0,
    }
  }

  pub fn pre_update(&mut self) {
    self.update_time = Utc::now();
    self.update_by = "admin".to_string(); // TODO: 实现真实的用户认证
  }
}
