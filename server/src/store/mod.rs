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

pub mod mongo;
#[macro_use]
pub mod sqlite;
pub mod documents_mongo;
pub mod documents_sqlite;
pub mod folders_mongo;
pub mod folders_sqlite;
pub mod settings_sqlite;
pub mod settings_mongo;
pub mod users_sqlite;
pub mod users_mongo;
pub mod browser_indexeddb;

use anyhow::Error;
use axum::async_trait;

use crate::{
    config::config_serve::{ WebServeProperties, DbType },
    types::{ PageResponse, PageRequest },
};

#[async_trait] // solution2: async fn + dyn polymorphism problem.
pub trait AsyncRepository<T>: Send {
    // solution1: async fn + dyn polymorphism problem.
    // fn select(&self) -> Box<dyn Future<Output = Result<Page<T>, Error>> + Send>;
    async fn select(&self, mut param: T, page: PageRequest) -> Result<(PageResponse, Vec<T>), Error>
        where T: 'static + Send + Sync;
    async fn select_by_id(&self, id: i64) -> Result<T, Error> where T: 'static + Send + Sync;
    async fn insert(&self, mut param: T) -> Result<i64, Error> where T: 'static + Send + Sync;
    async fn update(&self, mut param: T) -> Result<i64, Error> where T: 'static + Send + Sync;
    async fn delete_all(&self) -> Result<u64, Error>;
    async fn delete_by_id(&self, id: i64) -> Result<u64, Error>;
}

pub struct RepositoryContainer<T> where T: 'static + Send + Sync {
    sqlite_repo: Box<dyn AsyncRepository<T>>,
    mongo_repo: Box<dyn AsyncRepository<T>>,
}

impl<T> RepositoryContainer<T> where T: 'static + Send + Sync {
    pub fn new(
        sqlite_repo: Box<dyn AsyncRepository<T>>,
        mongo_repo: Box<dyn AsyncRepository<T>>
    ) -> Self {
        RepositoryContainer {
            sqlite_repo,
            mongo_repo,
        }
    }

    fn sqlite_repo(&self) -> &dyn AsyncRepository<T> {
        &*self.sqlite_repo
    }

    fn mongo_repo(&self) -> &dyn AsyncRepository<T> {
        &*self.mongo_repo
    }

    pub fn repo(/*&mut self*/ &self, config: &WebServeProperties) -> &dyn AsyncRepository<T> {
        match config.db.db_type {
            DbType::Sqlite => self.sqlite_repo(),
            DbType::Mongo => self.mongo_repo(),
        }
    }
}
