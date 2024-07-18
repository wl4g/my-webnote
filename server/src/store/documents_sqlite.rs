use anyhow::{ Error, Ok };
use axum::async_trait;

use crate::config::config_api::DbProperties;
use crate::types::documents::Document;
use crate::types::PageRequest;
use crate::types::PageResponse;
use super::AsyncRepository;
use super::sqlite::SQLiteRepository;

pub struct DocumentSQLiteRepository {
    inner: SQLiteRepository<Document>,
}

impl DocumentSQLiteRepository {
    pub async fn new(config: &DbProperties) -> Result<Self, Error> {
        Ok(DocumentSQLiteRepository {
            inner: SQLiteRepository::new(config).await?,
        })
    }
}

#[async_trait]
impl AsyncRepository<Document> for DocumentSQLiteRepository {
    async fn select(
        &self,
        document: Document,
        page: PageRequest
    ) -> Result<(PageResponse, Vec<Document>), Error> {
        let result = dynamic_sqlite_query!(
            document,
            "documents",
            self.inner.get_pool(),
            "update_time",
            page,
            Document
        ).unwrap();

        tracing::info!("query documents: {:?}", result);
        Ok((result.0, result.1))
    }

    async fn select_by_id(&self, id: i64) -> Result<Document, Error> {
        let document = sqlx
            ::query_as::<_, Document>("SELECT * FROM documents WHERE id = $1")
            .bind(id)
            .fetch_one(self.inner.get_pool()).await
            .unwrap();

        tracing::info!("query document: {:?}", document);
        Ok(document)
    }

    async fn insert(&self, mut document: Document) -> Result<i64, Error> {
        let inserted_id = dynamic_sqlite_insert!(
            document,
            "documents",
            self.inner.get_pool()
        ).unwrap();
        tracing::info!("Inserted document.id: {:?}", inserted_id);
        Ok(inserted_id)
    }

    async fn update(&self, mut document: Document) -> Result<i64, Error> {
        let updated_id = dynamic_sqlite_update!(
            document,
            "documents",
            self.inner.get_pool()
        ).unwrap();
        tracing::info!("Updated document.id: {:?}", updated_id);
        Ok(updated_id)
    }

    async fn delete_all(&self) -> Result<u64, Error> {
        let delete_result = sqlx
            ::query("DELETE FROM documents")
            .execute(self.inner.get_pool()).await
            .unwrap();

        tracing::info!("Deleted result: {:?}", delete_result);
        Ok(delete_result.rows_affected())
    }

    async fn delete_by_id(&self, id: i64) -> Result<u64, Error> {
        let delete_result = sqlx
            ::query("DELETE FROM documents WHERE id = $1")
            .bind(id)
            .execute(self.inner.get_pool()).await
            .unwrap();

        tracing::info!("Deleted result: {:?}", delete_result);
        Ok(delete_result.rows_affected())
    }
}
