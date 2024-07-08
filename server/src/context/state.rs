use std::sync::Arc;
use tokio::sync::Mutex;

// use crate::models::documents::Document;
// use crate::models::folders::Folder;
// use crate::models::settings::Settings;
use crate::models::users::User;
use crate::config::config::ApiConfig;
use crate::store::{
  RepositoryContainer,
  //   documents_sqlite::DocumentSQLiteRepository,
  //   documents_mongo::DocumentMongoRepository,
  //   folders_sqlite::FolderSQLiteRepository,
  //   folders_mongo::FolderMongoRepository,
  //   settings_sqlite::SettingsSQLiteRepository,
  //   settings_mongo::SettingsMongoRepository,
  users_sqlite::UserSQLiteRepository,
  users_mongo::UserMongoRepository,
};

#[derive(Clone)]
pub struct AppState {
  pub config: Arc<ApiConfig>,
  //   pub document_repo: Arc<Mutex<RepositoryContainer<Document>>>,
  //   pub folder_repo: Arc<Mutex<RepositoryContainer<Folder>>>,
  //   pub settings_repo: Arc<Mutex<RepositoryContainer<Settings>>>,
  pub user_repo: Arc<Mutex<RepositoryContainer<User>>>,
}

impl AppState {
  pub async fn new(config_arc: &Arc<ApiConfig>) -> AppState {
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

    let db_config = &config_arc.service.db;
    let user_sqlite_repo = Box::new(UserSQLiteRepository::new(&db_config).await.unwrap());
    let user_repo_container = RepositoryContainer::new(
      user_sqlite_repo,
      Box::new(UserMongoRepository::new())
    );

    let app_state = AppState {
      config: config_arc.clone(), // Arc 对象 clone 只是引用计数器+1, 无内存拷贝
      //   document_repo: Arc::new(Mutex::new(document_repo_container)),
      //   folder_repo: Arc::new(Mutex::new(folder_repo_container)),
      //   settings_repo: Arc::new(Mutex::new(settings_repo_container)),
      user_repo: Arc::new(Mutex::new(user_repo_container)),
    };

    app_state
  }
}
