use anyhow::Error;
use axum::async_trait;
use serde_json;
use crate::{
    context::state::AppState,
    types::browser_indexeddb::{
        DeleteIndexedRecordRequest,
        GetAllIndexedRecordRequest,
        GetIndexedRecordRequest,
        IndexedValue,
        SaveIndexedRecordRequest,
    },
};

#[async_trait]
pub trait IBrowserIndexedDBHandler: Send {
    async fn get(&self, param: GetIndexedRecordRequest) -> Result<Option<IndexedValue>, Error>;

    async fn get_all(
        &self,
        param: GetAllIndexedRecordRequest
    ) -> Result<Option<Vec<IndexedValue>>, Error>;

    async fn get_all_keys(
        &self,
        param: GetAllIndexedRecordRequest
    ) -> Result<Option<Vec<String>>, Error>;

    async fn add(&self, param: SaveIndexedRecordRequest) -> Result<String, Error>;

    async fn put(&self, param: SaveIndexedRecordRequest) -> Result<String, Error>;

    async fn delete(&self, param: DeleteIndexedRecordRequest) -> Result<u32, Error>;
}

pub struct BrowserIndexedDBHandlerImpl<'a> {
    state: &'a AppState,
}

impl<'a> BrowserIndexedDBHandlerImpl<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }
}

#[async_trait]
impl<'a> IBrowserIndexedDBHandler for BrowserIndexedDBHandlerImpl<'a> {
    async fn get(&self, param: GetIndexedRecordRequest) -> Result<Option<IndexedValue>, Error> {
        // let result = self.state.string_cache.get(&self.state.config).hget(
        //     param.store_name,
        //     param.key.map(|k| vec![k])
        // ).await?;
        // let result = result
        //     .map(|vs|
        //         vs
        //             .into_iter()
        //             .map(|v| serde_json::from_str::<IndexedValue>(v.as_str()))
        //             .collect::<Result<Vec<_>, _>>()
        //     )
        //     .transpose()?;
        // Ok(result.and_then(|v| v.into_iter().next()))

        let result = self.state.string_cache.get(&self.state.config).hget(
            param.store_name,
            param.key.map(|k| vec![k])
        ).await?;

        match result {
            Some(vs) => {
                if let Some(v) = vs.into_iter().next() {
                    let indexed_value = serde_json::from_str(&v)?;
                    Ok(Some(indexed_value))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    async fn get_all(
        &self,
        param: GetAllIndexedRecordRequest
    ) -> Result<Option<Vec<IndexedValue>>, Error> {
        let result = self.state.string_cache
            .get(&self.state.config)
            .hget_all(param.store_name).await;
        let result = result
            .map(|opt|
                opt.map(|vs|
                    vs
                        .into_iter()
                        .map(|v| serde_json::de::from_str(v.as_str()).unwrap())
                        .collect()
                )
            )
            .unwrap_or(None);
        Ok(result)
    }

    async fn get_all_keys(
        &self,
        param: GetAllIndexedRecordRequest
    ) -> Result<Option<Vec<String>>, Error> {
        let keys = self.state.string_cache.get(&self.state.config).hkeys(param.store_name).await?;
        if keys.is_empty() {
            Ok(None)
        } else {
            Ok(Some(keys))
        }
    }

    async fn add(&self, param: SaveIndexedRecordRequest) -> Result<String, Error> {
        let value = serde_json
            ::to_string(
                &(IndexedValue {
                    value: Some(param.value),
                })
            )
            .unwrap();

        #[allow(unused)]
        let result = self.state.string_cache
            .get(&self.state.config)
            .hset_nx(param.store_name, param.key.unwrap_or_default(), value).await?;

        Ok("OK".to_string())
    }

    async fn put(&self, param: SaveIndexedRecordRequest) -> Result<String, Error> {
        let value = serde_json
            ::to_string(
                &(IndexedValue {
                    value: Some(param.value),
                })
            )
            .unwrap();
        let field_values = vec![(param.key.unwrap_or_default(), value)];

        #[allow(unused)]
        let result = self.state.string_cache
            .get(&self.state.config)
            .hset(param.store_name, Some(field_values)).await?;

        Ok("OK".to_string())
    }

    async fn delete(&self, param: DeleteIndexedRecordRequest) -> Result<u32, Error> {
        let result = self.state.string_cache
            .get(&self.state.config)
            .hdel(param.store_name, param.key).await?;
        Ok(if result { 1 } else { 0 })
    }
}
