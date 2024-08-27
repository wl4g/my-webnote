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

use anyhow::Error;
use axum::async_trait;

use mongodb::Collection;
use mongodb::bson::doc;

use crate::config::config_serve::DbProperties;
use crate::types::settings::Settings;
use crate::types::{ PageRequest, PageResponse };
use super::AsyncRepository;
use super::mongo::MongoRepository;
use crate::{ dynamic_mongo_query, dynamic_mongo_insert, dynamic_mongo_update };

pub struct SettingsMongoRepository {
    #[allow(unused)]
    inner: Arc<MongoRepository<Settings>>,
    collection: Collection<Settings>,
}

impl SettingsMongoRepository {
    pub async fn new(config: &DbProperties) -> Result<Self, Error> {
        let inner = Arc::new(MongoRepository::new(config).await?);
        let collection = inner.get_database().collection("settings");
        Ok(SettingsMongoRepository { inner, collection })
    }
}

#[async_trait]
impl AsyncRepository<Settings> for SettingsMongoRepository {
    async fn select(
        &self,
        settings: Settings,
        page: PageRequest
    ) -> Result<(PageResponse, Vec<Settings>), Error> {
        match dynamic_mongo_query!(settings, self.collection, "update_time", page, Settings) {
            Ok(result) => {
                tracing::info!("query settings: {:?}", result);
                Ok((result.0, result.1))
            }
            Err(error) => Err(error),
        }
    }

    async fn select_by_id(&self, id: i64) -> Result<Settings, Error> {
        let filter = doc! { "id": id };
        let settings = self.collection
            .find_one(filter).await?
            .ok_or_else(|| Error::msg("Settings not found"))?;
        Ok(settings)
    }

    async fn insert(&self, mut settings: Settings) -> Result<i64, Error> {
        dynamic_mongo_insert!(settings, self.collection)
    }

    async fn update(&self, mut settings: Settings) -> Result<i64, Error> {
        dynamic_mongo_update!(settings, self.collection)
    }

    async fn delete_all(&self) -> Result<u64, Error> {
        let result = self.collection.delete_many(doc! {}).await?;
        Ok(result.deleted_count)
    }

    async fn delete_by_id(&self, id: i64) -> Result<u64, Error> {
        let filter = doc! { "id": id };
        let result = self.collection.delete_one(filter).await?;
        Ok(result.deleted_count)
    }
}
