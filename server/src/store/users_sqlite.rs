use anyhow::{ Error, Ok };
use axum::async_trait;

use crate::config::config_api::DbConfig;
use crate::types::users::User;
use crate::types::DEFAULT_BY;
use super::AsyncRepository;
use super::sqlite::SQLiteRepository;

pub struct UserSQLiteRepository {
  inner: SQLiteRepository<User>,
}

impl UserSQLiteRepository {
  pub async fn new(config: &DbConfig) -> Result<Self, Error> {
    Ok(UserSQLiteRepository {
      inner: SQLiteRepository::new(config).await?,
    })
  }
}

#[async_trait]
impl AsyncRepository<User> for UserSQLiteRepository {
  async fn select_all(&self) -> Result<Vec<User>, Error> {
    // see:https://tms-dev-blog.com/rust-sqlx-basics-with-sqlite/
    // see:https://github.com/launchbadge/sqlx/blob/main/sqlx-core/src/query_as.rs
    let result = sqlx
      ::query_as::<_, User>("SELECT * FROM users")
      .fetch_all(self.inner.get_pool()).await;

    result.map_err(|e| {
      println!("Error to select all: {:?}", e);
      Error::msg(e.to_string())
    })
  }

  async fn select_by_id(&self, id: i64) -> Result<User, Error> {
    let user = sqlx
      ::query_as::<_, User>("SELECT id, name FROM users WHERE id = $1")
      .bind(id)
      .fetch_one(self.inner.get_pool()).await
      .unwrap();

    println!("query user: {:?}", user);
    Ok(user)
  }

  async fn insert(&self, mut user: User) -> Result<i64, Error> {
    let inserted_id = dynamic_sqlite_insert!(user, "users", self.inner.get_pool()).unwrap();
    println!("Inserted user.id: {:?}", inserted_id);
    Ok(inserted_id)

    // //  let result = sqlx
    // //   ::query(
    // //     r#"
    // //     INSERT INTO users (id, name, email, password, create_by, create_time, update_by, update_time, del_flag)
    // //      VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    // //     "#
    // //   )
    // //   .bind(user.base.id)
    // //   .bind(user.name)
    // //   .bind(user.email)
    // //   .bind(user.phone)
    // //   .bind(user.password) // TODO persistent encrypt password
    // //   .bind(user.base.create_by)
    // //   .bind(user.base.create_time)
    // //   .bind(user.base.update_by)
    // //   .bind(user.base.update_time)
    // //   .bind(user.base.del_flag)
    // //   .execute(self.inner.get_pool()).await
    // //   .unwrap();
    // println!("Inserted result: {:?}, user.id: {:?}", result, id);

    // Ok(id)
  }

  async fn update(&self, mut user: User) -> Result<i64, Error> {
    let updated_id = dynamic_sqlite_update!(user, "users", self.inner.get_pool()).unwrap();
    println!("Updated user.id: {:?}", updated_id);
    Ok(updated_id)

    // let id = param.base.id.ok_or_else(|| Error::msg("User id is required for update"))?;
    // let update_result = sqlx
    //   ::query("UPDATE users SET name = $1, email = $2 WHERE id = $3")
    //   .bind(param.name)
    //   .bind(param.email)
    //   .bind(id)
    //   .execute(self.inner.get_pool()).await
    //   .unwrap();
    // println!("updated result: {:?}", update_result);
    // Ok(update_result.rows_affected() as i64)
  }

  async fn delete_all(&self) -> Result<u64, Error> {
    let delete_result = sqlx
      ::query("DELETE FROM users")
      .execute(self.inner.get_pool()).await
      .unwrap();

    println!("Deleted result: {:?}", delete_result);
    Ok(delete_result.rows_affected())
  }

  async fn delete_by_id(&self, id: i64) -> Result<u64, Error> {
    let delete_result = sqlx
      ::query("DELETE FROM users WHERE id = $1")
      .bind(id)
      .execute(self.inner.get_pool()).await
      .unwrap();

    println!("Deleted result: {:?}", delete_result);
    Ok(delete_result.rows_affected())
  }
}
