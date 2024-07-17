use std::any::Any;
use std::marker::PhantomData;

use anyhow::Error;
use axum::async_trait;

use mongodb::{ Client, Database, options::ClientOptions };

use super::AsyncRepository;
use crate::config::config_api::DbProperties;
use crate::types::{ PageResponse, PageRequest };

pub struct MongoRepository<T: Any + Send + Sync> {
    phantom: PhantomData<T>,
    database: Database,
}

impl<T: Any + Send + Sync> MongoRepository<T> {
    pub async fn new(config: &DbProperties) -> Result<Self, Error> {
        let client_options = ClientOptions::parse(
            &config.mongo.url.to_owned().expect("Mongo url missing configured")
        ).await?;
        let client = Client::with_options(client_options)?;
        let database = client.database(
            &config.mongo.database.to_owned().expect("Mongo database missing configured")
        );
        Ok(MongoRepository {
            phantom: PhantomData,
            database,
        })
    }

    pub fn get_database(&self) -> &Database {
        &self.database
    }
}

#[allow(unused)]
#[async_trait]
impl<T: Any + Send + Sync> AsyncRepository<T> for MongoRepository<T> {
    async fn select(
        &self,
        mut param: T,
        page: PageRequest
    ) -> Result<(PageResponse, Vec<T>), Error> {
        //use crate::dynamic_mongo_query;
        //match dynamic_mongo_query!(param, self.database.collection("users"), "update_time", page, User) {
        //    Ok(result) => {
        //        // println!("query users: {:?}", result);
        //        Ok((result.0, result.1))
        //    }
        //    Err(error) => Err(error),
        //}
        unimplemented!("select not implemented for MongoRepository")
    }

    async fn select_by_id(&self, id: i64) -> Result<T, Error> {
        unimplemented!("select_by_id not implemented for MongoRepository")
    }

    async fn insert(&self, param: T) -> Result<i64, Error> {
        unimplemented!("insert not implemented for MongoRepository")
    }

    async fn update(&self, param: T) -> Result<i64, Error> {
        unimplemented!("update not implemented for MongoRepository")
    }

    async fn delete_all(&self) -> Result<u64, Error> {
        unimplemented!("delete_all not implemented for MongoRepository")
    }

    async fn delete_by_id(&self, id: i64) -> Result<u64, Error> {
        unimplemented!("delete_by_id not implemented for MongoRepository")
    }
}

#[macro_export]
macro_rules! dynamic_mongo_query {
    ($bean:expr, $collection:expr, $order_by:expr, $page:expr, $($t:ty),+) => {
        {
            use mongodb::bson::{doc, Document};
            use futures::stream::TryStreamExt;

            let serialized = serde_json::to_value($bean).unwrap();
            let obj = serialized.as_object().unwrap();

            let mut filter = Document::new();
            for (key, value) in obj {
                if !value.is_null() {
                    let v = value.as_str().unwrap_or("");
                    if !v.is_empty() {
                        filter.insert(key, v);
                    }
                }
            }

            let options = mongodb::options::FindOptions::builder()
                .skip($page.get_offset() as u64)
                .limit($page.get_limit() as i64)
                .sort(doc! { $order_by: -1 })
                .build();

            // Queries to get total count.
            let total_count = $collection.count_documents(filter.clone()).await.unwrap();

            // Queries to get data.
            let cursor = $collection
                .find(filter)
                .skip(options.skip.unwrap())
                .limit(options.limit.unwrap())
                .sort(options.sort.unwrap()).await?;

            match cursor.try_collect().await {
                std::result::Result::Ok(result) => {
                  let page = PageResponse::new(
                      Some(total_count as i64),
                      Some($page.get_offset()),
                      Some($page.get_limit()));
                    Ok((page, result))
                },
                Err(error) => {
                    Err(error.into())
                }
            }
        }
    };
}

#[macro_export]
macro_rules! dynamic_mongo_insert {
    ($bean:expr, $collection:expr) => {
        {
            let id = $bean.base.pre_insert(Some(crate::types::DEFAULT_BY.to_string()));
            //use mongodb::bson::to_bson;
            //let serialized = to_bson(&$bean)?;
            //let obj = serialized.as_document().unwrap().clone();

            let result = $collection.insert_one(&$bean).await?;

            if let Some(inserted_id) = result.inserted_id.as_i64() {
                Ok(inserted_id)
            } else {
                Ok(id)
            }
        }
    };
}

#[macro_export]
macro_rules! dynamic_mongo_update {
    ($bean:expr, $collection:expr) => {
        {
            use mongodb::bson::{doc, to_bson};

            $bean.base.pre_update(Some(crate::types::DEFAULT_BY.to_string()));
            let id = $bean.base.id.unwrap();
            let serialized = to_bson(&$bean)?;
            let obj = serialized.as_document().unwrap().clone();

            let filter = doc! { "id": id };
            let update = doc! { "$set": obj };
            let result = $collection.update_one(filter, update).await?;

            if result.modified_count > 0 {
                Ok(id)
            } else {
                Ok(-1)
            }
        }
    };
}
