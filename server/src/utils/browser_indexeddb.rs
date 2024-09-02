/*
 * SPDX-License-Identifier: GNU GENERAL PUBLIC LICENSE Version 3
 *
 * Copyleft (c) 2024 James Wong. This file is part of James Wong.
 * is free software: you can redistribute it and/or modify it under
 * the terms of the GNU General Public License as published by the
 * Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * James Wong is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with James Wong.  If not, see <https://www.gnu.org/licenses/>.
 *
 * IMPORTANT: Any software that fully or partially contains or uses materials
 * covered by this license must also be released under the GNU GPL license.
 * This includes modifications and derived works.
 */

use std::sync::Arc;

use idb::{
    Database,
    DatabaseEvent,
    Error,
    Factory,
    IndexParams,
    KeyPath,
    ObjectStoreParams,
    Query,
    TransactionMode,
};

use serde::{ Deserialize, Serialize };
use serde_json::Value;

use wasm_bindgen::JsValue;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdbConfigProperties {
    pub indexeddb_name: String,
    pub indexeddb_store_names: Vec<String>,
}

pub struct BrowserIndexedDBRepository {
    config: Arc<IdbConfigProperties>,
    //inner: Arc<idb::Database>,
}

pub struct CustomObjectStoreParams {
    pub name: String,
    pub auto_increment: bool,
    pub key_path: Option<KeyPath>,
    pub indexes: Option<Vec<CustomIndexParams>>,
}

pub struct CustomIndexParams {
    pub name: String,
    pub unique: bool,
    pub key_path: KeyPath,
    pub indexes: Option<Vec<IndexParams>>,
}

impl BrowserIndexedDBRepository {
    pub async fn new(
        config: Arc<IdbConfigProperties>
        //obj_store_params: Vec<CustomObjectStoreParams>
    ) -> Result<Self, Error> {
        //let inner = Arc::new(Self::init_db(&config.indexeddb_name, 1, obj_store_params).await?);
        Ok(BrowserIndexedDBRepository {
            config,
            // inner,
        })
    }

    #[allow(unused)]
    pub async fn build_default(config: Arc<IdbConfigProperties>) -> Vec<CustomObjectStoreParams> {
        let obj_store_params = vec![
            CustomObjectStoreParams {
                name: "folder".to_string(),
                auto_increment: false,
                key_path: None,
                indexes: None,
            },
            CustomObjectStoreParams {
                name: "file".to_string(),
                auto_increment: false,
                key_path: None,
                indexes: Some(
                    vec![CustomIndexParams {
                        name: "type".to_string(),
                        unique: false,
                        key_path: KeyPath::Single("type".to_string()),
                        indexes: None,
                    }]
                ),
            },
            CustomObjectStoreParams {
                name: "folder_file_mapping".to_string(),
                auto_increment: false,
                key_path: None,
                indexes: Some(
                    vec![
                        CustomIndexParams {
                            name: "folderKey".to_string(),
                            unique: false,
                            key_path: KeyPath::Single("folderKey".to_string()),
                            indexes: None,
                        },
                        CustomIndexParams {
                            name: "fileKey".to_string(),
                            unique: true,
                            key_path: KeyPath::Single("fileKey".to_string()),
                            indexes: None,
                        }
                    ]
                ),
            }
        ];
        obj_store_params
    }

    // TODO: Suspected problem: thread not-safe and low performance??
    // TODO: Unable use member of BrowserIndexedDBRepository, because idb:Database is not Send.
    async fn get_db(&self, config: Arc<IdbConfigProperties>) -> Result<Arc<idb::Database>, Error> {
        let obj_store_params = Self::build_default(config.clone()).await;
        Ok(Arc::new(Self::init_db(&config.indexeddb_name, 1, obj_store_params).await?))
    }

    async fn init_db(
        name: &str,
        version: u32,
        obj_store_params: Vec<CustomObjectStoreParams>
    ) -> Result<Database, Error> {
        // Get a factory instance from global scope
        let factory = Factory::new()?;

        // Create an open request for the database
        let mut open_request = factory.open(name, Some(version)).unwrap();

        // Add an upgrade handler for database
        open_request.on_upgrade_needed(|event| {
            // Get database instance from event
            let database = event.database().unwrap();

            for cfg in obj_store_params {
                // Prepare object store params.
                let mut store_params = ObjectStoreParams::new();
                store_params.auto_increment(cfg.auto_increment);
                store_params.key_path(cfg.key_path);

                // Create object store on database.
                let store = database.create_object_store(&cfg.name, store_params).unwrap();

                // Create indexs on object store.
                match cfg.indexes {
                    Some(indexs) => {
                        for index in indexs {
                            let mut index_params = IndexParams::new();
                            index_params.unique(index.unique);
                            store
                                .create_index(&index.name, index.key_path, Some(index_params))
                                .unwrap();
                        }
                    }
                    None => {}
                }
            }
        });

        // `await` open request
        open_request.await
    }

    pub async fn get(&self, store_name: &str, query: JsValue) -> Result<Option<Value>, Error> {
        // Create a read-only transaction
        let transaction = self
            .get_db(self.config.clone()).await
            .unwrap()
            .transaction(&[store_name], TransactionMode::ReadOnly)
            .unwrap();

        // Get the object store
        let store = transaction.object_store(store_name).unwrap();

        // Get the stored data
        let stored_value = store.get(query)?.await?;

        // Deserialize the stored data
        let stored_value: Option<Value> = stored_value.map(|value|
            serde_wasm_bindgen::from_value(value).unwrap()
        );

        // Wait for the transaction to complete (alternatively, you can also commit the transaction)
        transaction.await?;

        Ok(stored_value)
    }

    pub async fn get_all(
        &self,
        store_name: &str,
        query: Option<Query>,
        limit: Option<u32>
    ) -> Result<Option<Vec<Value>>, Error> {
        // Create a read-only transaction
        let transaction = self
            .get_db(self.config.clone()).await
            .unwrap()
            .transaction(&[store_name], TransactionMode::ReadOnly)
            .unwrap();

        // Get the object store
        let store = transaction.object_store(store_name).unwrap();

        // Get the stored data
        let stored_values = store.get_all(query, limit)?.await.unwrap();

        // Deserialize the stored data.
        let stored_values = stored_values
            .iter()
            .map(|value| serde_wasm_bindgen::from_value(value.into()).unwrap())
            .collect();

        // Wait for the transaction to complete (alternatively, you can also commit the transaction)
        transaction.await?;

        Ok(stored_values)
    }

    pub async fn get_all_keys(
        &self,
        store_name: &str,
        query: Option<Query>,
        limit: Option<u32>
    ) -> Result<Option<Vec<Value>>, Error> {
        // Create a read-only transaction
        let transaction = self
            .get_db(self.config.clone()).await
            .unwrap()
            .transaction(&[store_name], TransactionMode::ReadOnly)
            .unwrap();

        // Get the object store
        let store = transaction.object_store(store_name).unwrap();

        // Get the stored data
        let stored_values = store.get_all_keys(query, limit)?.await.unwrap();

        // Deserialize the stored data.
        let stored_values = stored_values
            .iter()
            .map(|value| serde_wasm_bindgen::from_value(value.into()).unwrap())
            .collect();

        // Wait for the transaction to complete (alternatively, you can also commit the transaction)
        transaction.await?;

        Ok(stored_values)
    }

    pub async fn add(
        &self,
        store_name: &str,
        value: &JsValue,
        key: Option<&JsValue>
    ) -> Result<JsValue, Error> {
        // Create a read-write transaction
        let transaction = self
            .get_db(self.config.clone()).await
            .unwrap()
            .transaction(&[store_name], TransactionMode::ReadWrite)?;

        // Get the object store
        let store = transaction.object_store(store_name).unwrap();

        // Add data to object store
        let id = store.add(value, key)?.await?;

        // Commit the transaction
        transaction.commit()?.await?;

        Ok(id)
    }

    pub async fn put(
        &self,
        store_name: &str,
        value: &JsValue,
        key: Option<&JsValue>
    ) -> Result<JsValue, Error> {
        // Create a read-write transaction
        let transaction = self
            .get_db(self.config.clone()).await
            .unwrap()
            .transaction(&[store_name], TransactionMode::ReadWrite)?;

        // Get the object store
        let store = transaction.object_store(store_name).unwrap();

        // Put data to object store
        let id = store.put(value, key)?.await?;

        // Commit the transaction
        transaction.commit()?.await?;

        Ok(id)
    }

    pub async fn delete(&self, store_name: &str, key: JsValue) -> Result<u32, Error> {
        // Create a read-write transaction
        let transaction = self
            .get_db(self.config.clone()).await
            .unwrap()
            .transaction(&[store_name], TransactionMode::ReadWrite)?;

        // Get the object store
        let store = transaction.object_store(store_name).unwrap();

        // Delete data to object store
        store.delete(key)?.await?;

        // Commit the transaction
        transaction.commit()?.await?;

        Ok(1)
    }
}
