use anyhow::Error;
use axum::async_trait;

use crate::config::config_api::{ ApiProperties, CacheProvider };

pub mod memory;
pub mod redis;

#[async_trait]
pub trait ICache<T>: Send + Sync {
  async fn get(&self, key: String) -> Result<Option<T>, Error> where T: 'static + Send + Sync;
  async fn set(&self, key: String, value: T, expire: Option<i32>) -> Result<bool, Error>
    where T: 'static + Send + Sync;
  async fn delete(&self, key: String) -> Result<bool, Error>;
}

pub struct CacheContainer<T> where T: 'static + Send + Sync {
  memory_cache: Box<dyn ICache<T>>,
  redis_cache: Box<dyn ICache<T>>,
}

impl<T> CacheContainer<T> where T: 'static + Send + Sync {
  pub fn new(memory_cache: Box<dyn ICache<T>>, redis_cache: Box<dyn ICache<T>>) -> Self {
    CacheContainer {
      memory_cache,
      redis_cache,
    }
  }

  fn memory_cache(&self) -> &dyn ICache<T> {
    &*self.memory_cache
  }

  fn redis_cache(&self) -> &dyn ICache<T> {
    &*self.redis_cache
  }

  pub fn cache(&self, config: &ApiProperties) -> &dyn ICache<T> {
    match config.cache.provider {
      CacheProvider::Memory => self.memory_cache(),
      CacheProvider::Redis => self.redis_cache(),
    }
  }
}
