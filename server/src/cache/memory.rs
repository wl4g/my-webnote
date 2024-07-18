use anyhow::{ Error, Ok };
use axum::async_trait;
use moka::policy::EvictionPolicy;

use moka::future::Cache;
use std::sync::Arc;
use std::time::Duration;
use std::usize;

use crate::config::config_api::MemoryProperties;

use super::ICache;

pub struct StringMemoryCache {
  cache: Arc<Cache<String, String>>,
}

impl StringMemoryCache {
  pub fn new(config: &MemoryProperties) -> Self {
    let mut builder = Cache::builder();
    if config.initial_capacity.is_some() {
      builder = builder.initial_capacity(config.initial_capacity.clone().unwrap() as usize);
    }
    if config.max_capacity.is_some() {
      builder = builder.max_capacity(config.max_capacity.clone().unwrap());
    }
    if config.ttl.is_some() {
      builder = builder.time_to_live(Duration::from_millis(config.ttl.clone().unwrap()));
    }
    if config.eviction_policy.is_some() {
      match config.eviction_policy.to_owned().unwrap().to_uppercase().as_str() {
        "LRU" => {
          builder = builder.eviction_policy(EvictionPolicy::lru());
        }
        "LFU" | "TINY_LFU" => {
          builder = builder.eviction_policy(EvictionPolicy::tiny_lfu());
        }
        _ => {
          builder = builder.eviction_policy(EvictionPolicy::default());
        }
      }
    }
    StringMemoryCache {
      cache: Arc::new(builder.build()),
    }
  }
}

#[allow(unused)]
#[async_trait]
impl ICache<String> for StringMemoryCache {
  async fn get(&self, key: String) -> Result<Option<String>, Error> {
    Ok(self.cache.get(&key).await)
  }

  async fn set(&self, key: String, value: String, expire: Option<i32>) -> Result<bool, Error> {
    self.cache.insert(key.clone(), value).await;
    tracing::info!("Inserted to key: {}, expire: {:?}", key, expire);
    Ok(true)
  }

  async fn delete(&self, key: String) -> Result<bool, Error> {
    self.cache.invalidate(&key).await;
    Ok(true)
  }
}
