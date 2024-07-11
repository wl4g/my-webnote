use std::sync::Arc;
use oauth2::basic::BasicClient;
use oauth2::{ AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl };
use tokio::sync::Mutex;

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

#[derive(Clone)]
pub struct AppState {
  pub config: Arc<ApiConfig>,
  pub oidc_client: Option<BasicClient>,
  pub github_client: Option<BasicClient>,
  // pub document_repo: Arc<Mutex<RepositoryContainer<Document>>>,
  // pub folder_repo: Arc<Mutex<RepositoryContainer<Folder>>>,
  // pub settings_repo: Arc<Mutex<RepositoryContainer<Settings>>>,
  pub user_repo: Arc<Mutex<RepositoryContainer<User>>>,
}

impl AppState {
  pub async fn new(config_arc: &Arc<ApiConfig>) -> AppState {
    let auth_config = &config_arc.server.auths;
    let oidc_config = &auth_config.oidc;
    let github_config = &auth_config.github;
    let db_config = &config_arc.db;

    // TODO: remove clone, using ref string.
    let oidc_client = if oidc_config.enabled.unwrap_or(false) {
      Some(
        BasicClient::new(
          ClientId::new(oidc_config.client_id.clone().unwrap()),
          Some(ClientSecret::new(oidc_config.client_secret.clone().unwrap())),
          AuthUrl::new(oidc_config.auth_url.clone().unwrap()).unwrap(),
          Some(TokenUrl::new(oidc_config.token_url.clone().unwrap()).unwrap())
        ).set_redirect_uri(RedirectUrl::new(oidc_config.redirect_url.clone().unwrap()).unwrap())
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
      oidc_client: oidc_client,
      github_client: github_client,
      //   document_repo: Arc::new(Mutex::new(document_repo_container)),
      //   folder_repo: Arc::new(Mutex::new(folder_repo_container)),
      //   settings_repo: Arc::new(Mutex::new(settings_repo_container)),
      user_repo: Arc::new(Mutex::new(user_repo_container)),
    };

    app_state
  }
}
