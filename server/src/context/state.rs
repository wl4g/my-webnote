use std::sync::Arc;
use oauth2::basic::BasicClient;
use tokio::sync::Mutex;

use crate::cache::memory::StringMemoryCache;
use crate::cache::redis::StringRedisCache;
use crate::cache::CacheContainer;
// use crate::models::documents::Document;
// use crate::models::folders::Folder;
// use crate::models::settings::Settings;
use crate::types::users::User;
use crate::config::config_api::ApiConfig;
use crate::store::{
    RepositoryContainer,
    // documents_sqlite::DocumentSQLiteRepository,
    // documents_mongo::DocumentMongoRepository,
    // folders_sqlite::FolderSQLiteRepository,
    // folders_mongo::FolderMongoRepository,
    // settings_sqlite::SettingsSQLiteRepository,
    // settings_mongo::SettingsMongoRepository,
    users_sqlite::UserSQLiteRepository,
    users_mongo::UserMongoRepository,
};
use crate::utils::{ self, httpclients };

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<ApiConfig>,
    // pub document_repo: Arc<Mutex<RepositoryContainer<Document>>>,
    // pub folder_repo: Arc<Mutex<RepositoryContainer<Folder>>>,
    // pub settings_repo: Arc<Mutex<RepositoryContainer<Settings>>>,
    pub user_repo: Arc<Mutex<RepositoryContainer<User>>>,
    pub string_cache: Arc<CacheContainer<String>>,
    pub oidc_client: Option<Arc<openidconnect::core::CoreClient>>,
    pub github_client: Option<Arc<BasicClient>>,
    pub default_http_client: Arc<reqwest::Client>,
}

impl AppState {
    pub async fn new(config: &Arc<ApiConfig>) -> AppState {
        let cache_config = &config.cache;

        // Build DB repositories.
        let db_config = &config.db;
        // let document_repo_container = RepositoryContainer::new(
        //   Box::new(DocumentSQLiteRepository::new()),
        //   Box::new(DocumentMongoRepository::new())
        // );
        // let folder_repo_container = RepositoryContainer::new(
        //   Box::new(FolderSQLiteRepository::new()),
        //   Box::new(FolderMongoRepository::new())
        // );
        // let settings_repo_container = RepositoryContainer::new(
        //   Box::new(SettingsSQLiteRepository::new()),
        //   Box::new(SettingsMongoRepository::new())
        // );
        let user_repo_container = RepositoryContainer::new(
            Box::new(UserSQLiteRepository::new(&db_config).await.unwrap()),
            Box::new(UserMongoRepository::new(&db_config).await.unwrap())
        );

        // Build cacher.
        let cache_container = CacheContainer::new(
            Box::new(StringMemoryCache::new(&cache_config.memory)),
            Box::new(StringRedisCache::new(&cache_config.redis))
        );

        // Build auth clients.
        let auth_clients = (
            utils::oidcs
                ::create_oidc_client(&config.auth.oidc).await
                .map(|client| Arc::new(client)),
            utils::oauth2
                ::create_oauth2_client(&config.auth.github).await
                .map(|client| Arc::new(client)),
        );

        // Build tool http client.
        let http_client = httpclients::build_default();

        let app_state = AppState {
            // Notice: Arc object clone only increments the reference counter, and does not copy the actual data block.
            config: config.clone(),
            //document_repo: Arc::new(Mutex::new(document_repo_container)),
            //folder_repo: Arc::new(Mutex::new(folder_repo_container)),
            //settings_repo: Arc::new(Mutex::new(settings_repo_container)),
            user_repo: Arc::new(Mutex::new(user_repo_container)),
            string_cache: Arc::new(cache_container),
            oidc_client: auth_clients.0,
            github_client: auth_clients.1,
            default_http_client: Arc::new(http_client),
        };

        app_state
    }
}
