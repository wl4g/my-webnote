use anyhow::{ Error, Ok };
use axum::async_trait;

use crate::config::config_api::DbProperties;
use crate::types::folders::Folder;
use crate::types::PageRequest;
use crate::types::PageResponse;
use super::AsyncRepository;
use super::sqlite::SQLiteRepository;

pub struct FolderSQLiteRepository {
    inner: SQLiteRepository<Folder>,
}

impl FolderSQLiteRepository {
    pub async fn new(config: &DbProperties) -> Result<Self, Error> {
        Ok(FolderSQLiteRepository {
            inner: SQLiteRepository::new(config).await?,
        })
    }
}

#[async_trait]
impl AsyncRepository<Folder> for FolderSQLiteRepository {
    async fn select(
        &self,
        folder: Folder,
        page: PageRequest
    ) -> Result<(PageResponse, Vec<Folder>), Error> {
        let result = dynamic_sqlite_query!(
            folder,
            "folders",
            self.inner.get_pool(),
            "update_time",
            page,
            Folder
        ).unwrap();

        tracing::info!("query folders: {:?}", result);
        Ok((result.0, result.1))
    }

    async fn select_by_id(&self, id: i64) -> Result<Folder, Error> {
        let folder = sqlx
            ::query_as::<_, Folder>("SELECT * FROM folders WHERE id = $1")
            .bind(id)
            .fetch_one(self.inner.get_pool()).await
            .unwrap();

        tracing::info!("query folder: {:?}", folder);
        Ok(folder)
    }

    async fn insert(&self, mut folder: Folder) -> Result<i64, Error> {
        let inserted_id = dynamic_sqlite_insert!(folder, "folders", self.inner.get_pool()).unwrap();
        tracing::info!("Inserted folder.id: {:?}", inserted_id);
        Ok(inserted_id)
    }

    async fn update(&self, mut folder: Folder) -> Result<i64, Error> {
        let updated_id = dynamic_sqlite_update!(folder, "folders", self.inner.get_pool()).unwrap();
        tracing::info!("Updated folder.id: {:?}", updated_id);
        Ok(updated_id)
    }

    async fn delete_all(&self) -> Result<u64, Error> {
        let delete_result = sqlx
            ::query("DELETE FROM folders")
            .execute(self.inner.get_pool()).await
            .unwrap();

        tracing::info!("Deleted result: {:?}", delete_result);
        Ok(delete_result.rows_affected())
    }

    async fn delete_by_id(&self, id: i64) -> Result<u64, Error> {
        let delete_result = sqlx
            ::query("DELETE FROM folders WHERE id = $1")
            .bind(id)
            .execute(self.inner.get_pool()).await
            .unwrap();

        tracing::info!("Deleted result: {:?}", delete_result);
        Ok(delete_result.rows_affected())
    }
}
