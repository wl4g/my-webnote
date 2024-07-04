use axum::{
  routing::get,
  Router,
  extract::{ State, Query, Json },
  response::IntoResponse,
  http::StatusCode,
};
use crate::context::state::AppState;
use crate::models::folders::Folder;
use crate::models::folders::QueryFolderRequest;
use crate::handlers::folders::FolderHandler;

pub fn init() -> Router<AppState> {
  Router::new().route("/sys/folder/query", get(get_folders))
}

pub async fn get_folders(
  State(state): State<AppState>,
  Query(param): Query<QueryFolderRequest>
) -> impl IntoResponse {
  let handler = FolderHandler::new(&state);
  match handler.get_folders().await {
    Ok(folders) => (StatusCode::OK, Json(folders)).into_response(),
    Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
  }
}

async fn create_folder(
  State(state): State<AppState>,
  Json(folder): Json<Folder>
) -> impl IntoResponse {
  let handler = FolderHandler::new(&state);
  match handler.create_folder(folder).await {
    Ok(created_folder) => (StatusCode::CREATED, Json(created_folder)).into_response(),
    Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
  }
}
