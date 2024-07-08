use axum::{
  routing::get,
  routing::post,
  Router,
  extract::{ State, Query, Json },
  response::IntoResponse,
  http::StatusCode,
};

use crate::context::state::AppState;
use crate::handlers::users::UserHandler;
use crate::types::users::{ QueryUserRequest, SaveUserRequest, DeleteUserRequest };

pub fn init() -> Router<AppState> {
  Router::new()
    .route("/sys/user/query", get(get_users))
    .route("/sys/user/save", post(save_user))
    .route("/sys/user/delete", post(delete_user))
}

#[utoipa::path(
  get,
  path = "/sys/user/query",
  responses((status = 200, description = "Getting for all users.", body = QueryUserResponse)),
  tag = ""
)]
pub async fn get_users(
  State(state): State<AppState>,
  Query(param): Query<QueryUserRequest>
) -> impl IntoResponse {
  let handler = UserHandler::new(&state);
  match handler.find(param).await {
    Ok(users) => (StatusCode::OK, Json(users)).into_response(),
    Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
  }
}

#[utoipa::path(
  post,
  path = "/sys/user/save",
  responses((status = 200, description = "Save for user.", body = SaveUserResponse)),
  tag = ""
)]
async fn save_user(
  State(state): State<AppState>,
  Json(param): Json<SaveUserRequest>
) -> impl IntoResponse {
  let handler = UserHandler::new(&state);
  match handler.save(param).await {
    Ok(created_user) => (StatusCode::CREATED, Json(created_user)).into_response(),
    Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
  }
}

#[utoipa::path(
  post,
  path = "/sys/user/delete",
  responses((status = 200, description = "Delete for user.", body = DeleteUserRequest)),
  tag = ""
)]
async fn delete_user(
  State(state): State<AppState>,
  Json(param): Json<DeleteUserRequest>
) -> impl IntoResponse {
  let handler = UserHandler::new(&state);
  match handler.delete(param).await {
    Ok(created_user) => (StatusCode::CREATED, Json(created_user)).into_response(),
    Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
  }
}
