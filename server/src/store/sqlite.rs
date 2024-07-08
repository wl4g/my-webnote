use std::any::Any;
use std::marker::PhantomData;
use std::fs;
use std::path::Path;

use anyhow::Error;
use axum::async_trait;

use tracing::{ info, debug };
use sqlx::{ migrate::MigrateDatabase, Pool, Sqlite, SqlitePool };
use crate::config::config_api::DbConfig;
use super::AsyncRepository;

//
// const MIGRATION_INIT_SQL: &str = include_str!("../../migrations/20240710083754_init.sql");

pub struct SQLiteRepository<T: Any + Send + Sync> {
  phantom: PhantomData<T>,
  pool: SqlitePool,
}

impl<T: Any + Send + Sync> SQLiteRepository<T> {
  // see:https://tms-dev-blog.com/rust-sqlx-basics-with-sqlite/#Adding_a_migration_script
  pub async fn new(config: &DbConfig) -> Result<Self, Error> {
    let db_dir = Path::new(&config.sqlite.dir);
    if !db_dir.exists() {
      fs::create_dir_all(db_dir).map_err(|e| {
        eprintln!("Failed to sqlite db create directory: {:?}", e);
        e
      })?;
    }

    let db_url: String = format!("sqlite://{}/sqlite.db", &config.sqlite.dir).to_string();
    if !Sqlite::database_exists(db_url.as_str()).await.unwrap_or(false) {
      info!("Creating database {}", db_url);
      match Sqlite::create_database(db_url.as_str()).await {
        Ok(_) => println!("Create db success"),
        Err(error) => panic!("Error to create db: {}", error),
      }
    } else {
      println!("Database already exists and skip init migration.");
    }
    // SQLite in-memory database.
    // let db_url = format!("sqlite::memory:");

    match SqlitePool::connect(&db_url).await {
      Ok(pool) => {
        println!("Successfully connected to the database");
        let pool = Self::init_migration(pool).await;

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

  async fn init_migration(pool: Pool<Sqlite>) -> Pool<Sqlite> {
    // let default_dir = std::env
    //   ::current_dir()
    //   .map(|s| s.to_str().unwrap())
    //   .unwrap();
    // let migrations_dir = std::env
    //   ::var("CARGO_MANIFEST_DIR")
    //   .unwrap_or_else(|_| default_dir.to_string());
    // let migrations_dir = std::path::Path::new(&current_dir).join("./migrations");
    // let results = sqlx::migrate::Migrator::new(migrations).await.unwrap().run(&pool).await;
    // debug!("Migration result: {:?}", results);
    // match results {
    //   Ok(_) => println!("Migration success"),
    //   Err(error) => {
    //     panic!("error: {}", error);
    //   }
    // }

    let results = sqlx::migrate!("./migrations").run(&pool).await;
    debug!("Migration result: {:?}", results);
    match results {
      Ok(_) => println!("Migration success"),
      Err(error) => {
        panic!("Error migration: {}", error);
      }
    }
    pool
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
    unimplemented!("insert not implemented for SQLiteRepository");
    let pool = self.get_pool();
  }

  async fn update(&self, param: T) -> Result<i64, Error> {
    unimplemented!("update not implemented for SQLiteRepository")
  }

  async fn delete_all(&self) -> Result<u64, Error> {
    unimplemented!("delete_all not implemented for SQLiteRepository")
  }

  async fn delete_by_id(&self, id: i64) -> Result<u64, Error> {
    unimplemented!("delete_by_id not implemented for SQLiteRepository")
  }
}

macro_rules! dynamic_sqlite_insert {
  ($bean:expr, $table:expr, $pool:expr) => {
    {
        let id = $bean.base.pre_insert(Some(DEFAULT_BY.to_string())); // TODO dynamic get login principal.
        let serialized = serde_json::to_value($bean).unwrap();
        let obj = serialized.as_object().unwrap();

        let mut fields = Vec::new();
        let mut values = Vec::new();
        let mut params = Vec::new();
        for (key, value) in obj {
            if !value.is_null() {
                fields.push(key.as_str());
                values.push("?");
                params.push(value.to_string());
            }
        }
        if fields.is_empty() {
            return Ok(-1);
        }

        // let fields_str = fields
        //  .iter()
        //  .map(|s| s.as_str())
        //  .collect::<Vec<&str>>()
        //  .join(",");
        let query = format!("INSERT INTO {} ({}) VALUES ({})", $table, fields.join(","), values.join(","));

        let mut operator = sqlx::query(&query);
        for param in params.iter() {
            operator = operator.bind(param);
        }

        match operator.execute($pool).await {
            std::result::Result::Ok(result) => {
                if result.rows_affected() > 0 {
                    return Ok(id);
                } else {
                    return Ok(-1);
                }
            },
            Err(e) => Err(Error::from(e)),
        }
    }
  };
}

macro_rules! dynamic_sqlite_update {
  ($bean:expr, $table:expr, $pool:expr) => {
        {
            $bean.base.pre_update(Some(DEFAULT_BY.to_string())); // TODO dynamic get login principal.
            let id = $bean.base.id.unwrap();
            let serialized = serde_json::to_value($bean).unwrap();
            let obj = serialized.as_object().unwrap();

            let mut fields = Vec::new();
            let mut params = Vec::new();
            for (key, value) in obj {
                if !value.is_null() {
                    fields.push(format!("{} = ?", key));
                    params.push(value.to_string());
                }
            }
            if fields.is_empty() {
                return Ok(0);
            }

            let query = format!("UPDATE {} SET {} WHERE id = ?", $table, fields.join(", "));
            let mut operator = sqlx::query(&query);
            for param in params.iter() {
                operator = operator.bind(param);
            }
            operator = operator.bind(id);

            match operator.execute($pool).await {
                std::result::Result::Ok(result) => {
                    if result.rows_affected() > 0 {
                        return Ok(id);
                    } else {
                        return Ok(-1);
                    }
                },
                Err(e) => Err(Error::from(e)),
            }
        }
  };
}
