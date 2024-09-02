use anyhow::Error;
use axum::async_trait;
use idb::Query;
use crate::{
    context::state::AppState,
    types::browser_indexeddb::{
        DeleteIndexedRecordRequest,
        GetAllIndexedRecordRequest,
        GetIndexedRecordRequest,
        IndexedRecord,
        SaveIndexedRecordRequest,
    },
};

#[async_trait(?Send)] // 必须标记为 Send, 否则由于Wasm中未使用Send导致线程不安全, 进而 rustc err
pub trait IBrowserIndexedDBHandler: Send {
    async fn get(&self, param: GetIndexedRecordRequest) -> Result<Option<IndexedRecord>, Error>;

    async fn get_all(
        &self,
        param: GetAllIndexedRecordRequest
    ) -> Result<Option<Vec<IndexedRecord>>, Error>;

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

#[async_trait(?Send)]
impl<'a> IBrowserIndexedDBHandler for BrowserIndexedDBHandlerImpl<'a> {
    async fn get(&self, param: GetIndexedRecordRequest) -> Result<Option<IndexedRecord>, Error> {
        let query = serde_wasm_bindgen::to_value(&param.key).unwrap();

        let repo = self.state.browser_indexeded_repo.lock().await;
        let res: Result<Option<IndexedRecord>, idb::Error> = repo
            .get(&param.store_name, query).await
            .map(|opt| { opt.map(|value| { serde_json::from_value(value).unwrap() }) });

        match res {
            Ok(record) => Ok(record),
            Err(err) => Err(Error::msg(err.to_string())),
        }
    }

    async fn get_all(
        &self,
        param: GetAllIndexedRecordRequest
    ) -> Result<Option<Vec<IndexedRecord>>, Error> {
        let query = serde_wasm_bindgen::to_value(&param.key).unwrap();

        let handler = self.state.browser_indexeded_repo.lock().await;
        let res: Result<Option<Vec<IndexedRecord>>, idb::Error> = handler
            .get_all(&param.store_name, Some(Query::Key(query)), Some(100)).await
            .map(|opt|
                opt.map(|values| {
                    values
                        // 若使用 iter() 则 map 中 value 是借用引用类型, 若后续使用需所有权对象, 则还需 clone 浪费
                        .into_iter()
                        .map(|value| {
                            let record: IndexedRecord = serde_json::from_value(value).unwrap();
                            record
                        })
                        .collect::<Vec<_>>() // 若不加泛型则表示收集单条, rustc err
                })
            );

        match res {
            Ok(records) => Ok(records),
            Err(err) => Err(Error::msg(err.to_string())),
        }
    }

    async fn get_all_keys(
        &self,
        param: GetAllIndexedRecordRequest
    ) -> Result<Option<Vec<String>>, Error> {
        let query = serde_wasm_bindgen::to_value(&param.key).unwrap();
        let handler = self.state.browser_indexeded_repo.lock().await;
        let res = handler
            .get_all_keys(&param.store_name, Some(Query::Key(query)), Some(100)).await
            .map(|opt|
                opt.map(|values| {
                    values
                        .iter()
                        .map(|value| value.to_string())
                        .collect::<Vec<_>>() // 若不加泛型则表示收集单条, rustc err
                })
            );

        match res {
            Ok(keys) => Ok(keys),
            Err(err) => Err(Error::msg(err.to_string())),
        }
    }

    async fn add(&self, param: SaveIndexedRecordRequest) -> Result<String, Error> {
        let value = serde_wasm_bindgen::to_value(&param.value).unwrap();
        let key = &param.key;
        let ref_key = &key.clone().map(|k| serde_wasm_bindgen::to_value(&k).unwrap());
        let in_ref_key = match &ref_key {
            Some(value) => Some(value),
            None => None,
        };
        let handler = self.state.browser_indexeded_repo.lock().await;
        let res = handler.add(&param.store_name, &value, in_ref_key).await;

        match res {
            Ok(_) => Ok(key.clone().unwrap_or_default()),
            Err(err) => Err(Error::msg(err.to_string())),
        }
    }

    async fn put(&self, param: SaveIndexedRecordRequest) -> Result<String, Error> {
        let value = serde_wasm_bindgen::to_value(&param.value).unwrap();
        let key = &param.key;
        let ref_key = &key.clone().map(|k| serde_wasm_bindgen::to_value(&k).unwrap());
        let in_ref_key = match &ref_key {
            Some(value) => Some(value),
            None => None,
        };
        let handler = self.state.browser_indexeded_repo.lock().await;
        let res = handler.put(&param.store_name, &value, in_ref_key).await;

        match res {
            Ok(_) => Ok(param.key.unwrap_or_default()),
            Err(err) => Err(Error::msg(err.to_string())),
        }
    }

    async fn delete(&self, param: DeleteIndexedRecordRequest) -> Result<u32, Error> {
        let key = serde_wasm_bindgen::to_value(&param.key).unwrap();

        let handler = self.state.browser_indexeded_repo.lock().await;
        let res = handler.delete(&param.store_name, key).await;

        match res {
            Ok(count) => Ok(count),
            Err(err) => Err(Error::msg(err.to_string())),
        }
    }
}
