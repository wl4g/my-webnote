use anyhow::Error;
use axum::async_trait;
use std::any::Any;
use std::fs;
use std::marker::PhantomData;
use std::path::Path;
use sqlx::SqlitePool;

use crate::config::config::DbConfig;
use super::AsyncRepository;

pub struct SQLiteRepository<T: Any + Send + Sync> {
  phantom: PhantomData<T>,
  pool: SqlitePool,
}

impl<T: Any + Send + Sync> SQLiteRepository<T> {
  pub async fn new(config: &DbConfig) -> Result<Self, Error> {
    let db_dir = Path::new(&config.sqlite.dir);
    if !db_dir.exists() {
      fs::create_dir_all(db_dir).map_err(|e| {
        eprintln!("Failed to sqlite db create directory: {:?}", e);
        e
      })?;
    }

    // SQLite in-file database.
    let db_path = db_dir.join("sqlite.db"); // Use the full path
    match fs::File::create(&db_path) {
      Ok(_) => println!("Successfully created/opened the database file"),
      Err(e) => eprintln!("Failed to create/open the database file: {:?}", e),
    }
    let conn_url = format!("sqlite:{}", db_path.display());
    println!("Attempting to connect to: {}", conn_url);

    // SQLite in-memory database.
    // let conn_url = format!("sqlite::memory:");

    match SqlitePool::connect(&conn_url).await {
      Ok(pool) => {
        println!("Successfully connected to the database");
        Ok(SQLiteRepository {
          phantom: PhantomData,
          pool,
        })
      }
      Err(e) => {
        eprintln!("Database sqlite connection error: {:?}", e);
        eprintln!("Error details: {}", e);
        Err(e.into())
      }
    }
  }

  pub fn get_pool(&self) -> &SqlitePool {
    &self.pool
  }
}

#[allow(unused)]
#[async_trait]
impl<T: Any + Send + Sync> AsyncRepository<T> for SQLiteRepository<T> {
  async fn select_all(&self) -> Result<Vec<T>, Error> {
    unimplemented!("select not implemented for SQLiteRepository")
  }

  async fn select_by_id(&self, id: i64) -> Result<T, Error> {
    unimplemented!("select_by_id not implemented for SQLiteRepository")
  }

  async fn insert(&self, param: T) -> Result<i64, Error> {
    unimplemented!("insert not implemented for SQLiteRepository")
  }

  async fn update(&self, param: T) -> Result<u64, Error> {
    unimplemented!("update not implemented for SQLiteRepository")
  }

  async fn delete_all(&self) -> Result<u64, Error> {
    unimplemented!("delete_all not implemented for SQLiteRepository")
  }

  async fn delete_by_id(&self, id: i64) -> Result<u64, Error> {
    unimplemented!("delete_by_id not implemented for SQLiteRepository")
  }
}
