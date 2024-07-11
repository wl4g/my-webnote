use std::sync::Arc;
use oauth2::basic::BasicClient;
use oauth2::{ AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl };
use tokio::sync::Mutex;

use crate::cache::memory::StringMemoryCache;
use crate::cache::redis::StringRedisCache;
use crate::cache::CacheContainer;
// use crate::models::documents::Document;
// use crate::models::folders::Folder;
// use crate::models::settings::Settings;
use crate::types::users::User;
use crate::config::config_api::{ ApiConfig, AuthConfig };
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
use crate::utils::httpclients;

#[derive(Clone)]
pub struct AppState {
  pub config: Arc<ApiConfig>,
  pub string_cache: Arc<CacheContainer<String>>,
  pub default_http_client: Arc<reqwest::Client>,
  pub oidc_client: Option<Arc<BasicClient>>,
  pub github_client: Option<Arc<BasicClient>>,
  // pub document_repo: Arc<Mutex<RepositoryContainer<Document>>>,
  // pub folder_repo: Arc<Mutex<RepositoryContainer<Folder>>>,
  // pub settings_repo: Arc<Mutex<RepositoryContainer<Settings>>>,
  pub user_repo: Arc<Mutex<RepositoryContainer<User>>>,
}

impl AppState {
  pub async fn new(config_arc: &Arc<ApiConfig>) -> AppState {
    let cache_config = &config_arc.cache;
    let cache_container = CacheContainer::new(
      Box::new(StringMemoryCache::new(&cache_config.memory)),
      Box::new(StringRedisCache::new(&cache_config.redis))
    );

    // The tools http client.
    let http_client = httpclients::build_default();
    let result = Self::build_oauth2_clients(&http_client, &config_arc.server.auth).await;

    let db_config = &config_arc.db;
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
      Box::new(UserMongoRepository::new())
    );

    let app_state = AppState {
      config: config_arc.clone(), // Arc 对象 clone 只是引用计数器+1, 无内存拷贝
      string_cache: Arc::new(cache_container),
      default_http_client: Arc::new(http_client),
      oidc_client: Some(Arc::new(result.0.unwrap())),
      github_client: Some(Arc::new(result.1.unwrap())),
      //   document_repo: Arc::new(Mutex::new(document_repo_container)),
      //   folder_repo: Arc::new(Mutex::new(folder_repo_container)),
      //   settings_repo: Arc::new(Mutex::new(settings_repo_container)),
      user_repo: Arc::new(Mutex::new(user_repo_container)),
    };

    app_state
  }

  async fn build_oauth2_clients(
    http_client: &reqwest::Client,
    auth_config: &AuthConfig
  ) -> (Option<BasicClient>, Option<BasicClient>) {
    let oidc_config = &auth_config.oidc;
    let github_config = &auth_config.github;

    let oidc_client = if oidc_config.enabled.unwrap_or(false) {
      // Get the auth_url,token_url,userinfo_url from the oidc discovery endpoint.
      let url = oidc_config.discovery_endpoint
        .clone()
        .expect("Missing 'discovery_endpoint' configured");

      let resp = http_client.get(url).send().await.unwrap();

      let json: serde_json::Value = resp
        .json().await
        .expect("Could't to get OIDC endpoints from discovery.");
      //.unwrap_or_else(|_| { serde_json::Value::String("{}".to_string()) });

      let auth_url = json["authorization_endpoint"]
        .as_str()
        .expect("Missing 'authorization_endpoint' response")
        .to_string();
      let token_url = json["token_endpoint"]
        .as_str()
        .expect("Missing 'token_endpoint' response")
        .to_string();
      //let userinfo_url = json["userinfo_endpoint"].as_str().expect("Missing 'userinfo_endpoint' response").to_string();

      Some(
        BasicClient::new(
          ClientId::new(
            oidc_config.client_id.as_ref().expect("Missing client id configure").to_owned()
          ),
          Some(
            ClientSecret::new(
              oidc_config.client_secret.clone().expect("Missing client secret configure")
            )
          ),
          AuthUrl::new(auth_url).unwrap(),
          Some(TokenUrl::new(token_url).unwrap())
        ).set_redirect_uri(
          RedirectUrl::new(oidc_config.redirect_url.as_ref().unwrap().to_string()).unwrap()
        )
      )
    } else {
      None
    };

    let github_client = if github_config.enabled.unwrap_or(false) {
      Some(
        BasicClient::new(
          ClientId::new(github_config.client_id.as_ref().unwrap().clone()),
          Some(ClientSecret::new(github_config.client_secret.as_ref().unwrap().clone())),
          AuthUrl::new(github_config.auth_url.as_ref().unwrap().clone()).unwrap(),
          Some(TokenUrl::new(github_config.token_url.as_ref().unwrap().clone()).unwrap())
        ).set_redirect_uri(
          RedirectUrl::new(github_config.redirect_url.as_ref().unwrap().clone()).unwrap()
        )
      )
    } else {
      None
    };

    (oidc_client, github_client)
  }
}
