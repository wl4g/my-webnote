use anyhow::Error;
use axum::async_trait;
use redis::{
  cluster::{ ClusterClient, ClusterClientBuilder },
  cluster_async::ClusterConnection,
  RedisResult,
};
use std::{ sync::Arc, time::Duration };

use crate::config::config_api::RedisConfig;

use super::ICache;

pub struct StringRedisCache {
  client: Arc<ClusterClient>,
}

impl StringRedisCache {
  pub fn new(config: &RedisConfig) -> Self {
    let mut builder = ClusterClientBuilder::new(config.nodes.clone());
    if config.username.is_some() {
      builder = builder.username(config.username.clone().unwrap());
    }
    if config.password.is_some() {
      builder = builder.password(config.password.clone().unwrap());
    }
    if config.connection_timeout.is_some() {
      builder = builder.connection_timeout(
        Duration::from_millis(config.connection_timeout.clone().unwrap())
      );
    }
    if config.response_timeout.is_some() {
      builder = builder.response_timeout(
        Duration::from_millis(config.response_timeout.clone().unwrap())
      );
    }
    if config.retries.is_some() {
      builder = builder.retries(config.retries.clone().unwrap());
    }
    if config.max_retry_wait.is_some() {
      builder = builder.max_retry_wait(config.max_retry_wait.clone().unwrap());
    }
    if config.min_retry_wait.is_some() {
      builder = builder.min_retry_wait(config.min_retry_wait.clone().unwrap());
    }
    if config.read_from_replicas.is_some() {
      builder = builder.read_from_replicas();
    }
    let client = builder.build().expect("Failed to build redis cluster client");
    StringRedisCache { client: Arc::new(client) }
  }

  async fn get_async_connection(&self) -> Result<ClusterConnection, Error> {
    self.client.get_async_connection().await.map_err(Error::from)
  }
}

#[async_trait]
impl ICache<String> for StringRedisCache {
  async fn get(&self, key: String) -> Result<Option<String>, Error> {
    let mut con = self.get_async_connection().await?;
    let result: RedisResult<Option<String>> = redis
      ::cmd("GET")
      .arg(key)
      .query_async(&mut con).await;
    Ok(result?)
  }

  async fn set(&self, key: String, value: String, expire: Option<i32>) -> Result<bool, Error> {
    let mut con: ClusterConnection = self.get_async_connection().await?;
    let result: RedisResult<()> = if let Some(seconds) = expire {
      redis::cmd("SETEX").arg(key).arg(seconds).arg(value).query_async(&mut con).await
    } else {
      redis::cmd("SET").arg(key).arg(value).query_async(&mut con).await
    };
    Ok(result.is_ok())
  }

  async fn delete(&self, key: String) -> Result<bool, Error> {
    let mut con = self.get_async_connection().await?;
    let result: RedisResult<i32> = redis::cmd("DEL").arg(key).query_async(&mut con).await;
    Ok(result.map(|n| n == 1).unwrap_or(false))
  }
}
