use anyhow::{ Error, Ok };
use axum::async_trait;

use crate::config::config_api::DbProperties;
use crate::types::settings::Settings;
use crate::types::PageRequest;
use crate::types::PageResponse;
use super::AsyncRepository;
use super::sqlite::SQLiteRepository;

pub struct SettingsSQLiteRepository {
    inner: SQLiteRepository<Settings>,
}

impl SettingsSQLiteRepository {
    pub async fn new(config: &DbProperties) -> Result<Self, Error> {
        Ok(SettingsSQLiteRepository {
            inner: SQLiteRepository::new(config).await?,
        })
    }
}

#[async_trait]
impl AsyncRepository<Settings> for SettingsSQLiteRepository {
    async fn select(
        &self,
        settings: Settings,
        page: PageRequest
    ) -> Result<(PageResponse, Vec<Settings>), Error> {
        let result = dynamic_sqlite_query!(
            settings,
            "settings",
            self.inner.get_pool(),
            "update_time",
            page,
            Settings
        ).unwrap();

        tracing::info!("query settings: {:?}", result);
        Ok((result.0, result.1))
    }

    async fn select_by_id(&self, id: i64) -> Result<Settings, Error> {
        let settings = sqlx
            ::query_as::<_, Settings>("SELECT * FROM settings WHERE id = $1")
            .bind(id)
            .fetch_one(self.inner.get_pool()).await
            .unwrap();

        tracing::info!("query settings: {:?}", settings);
        Ok(settings)
    }

    async fn insert(&self, mut settings: Settings) -> Result<i64, Error> {
        let inserted_id = dynamic_sqlite_insert!(
            settings,
            "settings",
            self.inner.get_pool()
        ).unwrap();
        tracing::info!("Inserted settings.id: {:?}", inserted_id);
        Ok(inserted_id)
    }

    async fn update(&self, mut settings: Settings) -> Result<i64, Error> {
        let updated_id = dynamic_sqlite_update!(
            settings,
            "settings",
            self.inner.get_pool()
        ).unwrap();
        tracing::info!("Updated settings.id: {:?}", updated_id);
        Ok(updated_id)
    }

    async fn delete_all(&self) -> Result<u64, Error> {
        let delete_result = sqlx
            ::query("DELETE FROM settings")
            .execute(self.inner.get_pool()).await
            .unwrap();

        tracing::info!("Deleted result: {:?}", delete_result);
        Ok(delete_result.rows_affected())
    }

    async fn delete_by_id(&self, id: i64) -> Result<u64, Error> {
        let delete_result = sqlx
            ::query("DELETE FROM settings WHERE id = $1")
            .bind(id)
            .execute(self.inner.get_pool()).await
            .unwrap();

        tracing::info!("Deleted result: {:?}", delete_result);
        Ok(delete_result.rows_affected())
    }
}
