use std::sync::Arc;
use oauth2::basic::BasicClient;
use tokio::sync::Mutex;

use crate::cache::memory::StringMemoryCache;
use crate::cache::redis::StringRedisCache;
use crate::cache::CacheContainer;
// use crate::monitoring::health::{ MongoChecker, RedisClusterChecker, SQLiteChecker };
use crate::types::documents::Document;
use crate::types::folders::Folder;
use crate::types::settings::Settings;
use crate::types::users::User;
use crate::config::config_api::ApiConfig;
use crate::store::{
    RepositoryContainer,
    documents_sqlite::DocumentSQLiteRepository,
    documents_mongo::DocumentMongoRepository,
    folders_sqlite::FolderSQLiteRepository,
    folders_mongo::FolderMongoRepository,
    settings_sqlite::SettingsSQLiteRepository,
    settings_mongo::SettingsMongoRepository,
    users_sqlite::UserSQLiteRepository,
    users_mongo::UserMongoRepository,
};
use crate::utils::{ self, httpclients };

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<ApiConfig>,
    // The basic operators.
    pub string_cache: Arc<CacheContainer<String>>,
    pub oidc_client: Option<Arc<openidconnect::core::CoreClient>>,
    pub github_client: Option<Arc<BasicClient>>,
    pub default_http_client: Arc<reqwest::Client>,
    // The modules repositories.
    pub user_repo: Arc<Mutex<RepositoryContainer<User>>>,
    pub document_repo: Arc<Mutex<RepositoryContainer<Document>>>,
    pub folder_repo: Arc<Mutex<RepositoryContainer<Folder>>>,
    pub settings_repo: Arc<Mutex<RepositoryContainer<Settings>>>,
    // // The health checker.
    // pub sqlite_checker: SQLiteChecker,
    // pub mongo_checker: MongoChecker,
    // pub redis_cluster_checker: RedisClusterChecker,
}

impl AppState {
    pub async fn new(config: &Arc<ApiConfig>) -> AppState {
        let cache_config = &config.cache;

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

        // Build DB repositories.
        let db_config = &config.db;
        let user_repo_container = RepositoryContainer::new(
            Box::new(UserSQLiteRepository::new(&db_config).await.unwrap()),
            Box::new(UserMongoRepository::new(&db_config).await.unwrap())
        );
        let document_repo_container = RepositoryContainer::new(
            Box::new(DocumentSQLiteRepository::new(&db_config).await.unwrap()),
            Box::new(DocumentMongoRepository::new(&db_config).await.unwrap())
        );
        let folder_repo_container = RepositoryContainer::new(
            Box::new(FolderSQLiteRepository::new(&db_config).await.unwrap()),
            Box::new(FolderMongoRepository::new(&db_config).await.unwrap())
        );
        let settings_repo_container = RepositoryContainer::new(
            Box::new(SettingsSQLiteRepository::new(&db_config).await.unwrap()),
            Box::new(SettingsMongoRepository::new(&db_config).await.unwrap())
        );

        let app_state = AppState {
            // Notice: Arc object clone only increments the reference counter, and does not copy the actual data block.
            config: config.clone(),
            // The basic operators.
            string_cache: Arc::new(cache_container),
            oidc_client: auth_clients.0,
            github_client: auth_clients.1,
            default_http_client: Arc::new(http_client),
            // The modules repositories.
            user_repo: Arc::new(Mutex::new(user_repo_container)),
            document_repo: Arc::new(Mutex::new(document_repo_container)),
            folder_repo: Arc::new(Mutex::new(folder_repo_container)),
            settings_repo: Arc::new(Mutex::new(settings_repo_container)),
            // // The health checker.
            // sqlite_checker: SQLiteChecker::new(),
            // mongo_checker: MongoChecker::new(),
            // redis_cluster_checker: RedisClusterChecker::new(),
        };

        // Build DI container.
        // let mut di_container = syrette::DIContainer::new();
        // di_container.bind::<dyn IUserHandler>().to::<UserHandler>()?;

        app_state
    }
}
