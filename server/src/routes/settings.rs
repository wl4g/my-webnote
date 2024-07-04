use axum::{
  routing::get,
  Router,
  extract::{ State, Query, Json },
  response::IntoResponse,
  http::StatusCode,
};
use crate::context::state::AppState;
use crate::models::settings::Settings;
use crate::models::settings::QuerySettingsRequest;
use crate::handlers::settings::SettingsHandler;

pub fn init() -> Router<AppState> {
  Router::new().route("/sys/setting/query", get(get_settings))
}

pub async fn get_settings(
  State(state): State<AppState>,
  Query(param): Query<QuerySettingsRequest>
) -> impl IntoResponse {
  let handler = SettingsHandler::new(&state);
  match handler.get_settings().await {
    Ok(settings) => (StatusCode::OK, Json(settings)).into_response(),
    Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
  }
}

async fn create_setting(
  State(state): State<AppState>,
  Json(setting): Json<Settings>
) -> impl IntoResponse {
  let handler = SettingsHandler::new(&state);
  match handler.create_settings(setting).await {
    Ok(created_setting) => (StatusCode::CREATED, Json(created_setting)).into_response(),
    Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
  }
}
