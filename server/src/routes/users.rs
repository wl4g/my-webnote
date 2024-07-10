use axum::{
  routing::get,
  routing::post,
  Router,
  extract::{ State, Query, Json },
  http::StatusCode,
};

use crate::{
  context::state::AppState,
  types::users::{ DeleteUserResponse, QueryUserResponse, SaveUserResponse },
};
use crate::handlers::users::UserHandler;
use crate::types::users::{ QueryUserRequest, SaveUserRequest, DeleteUserRequest };

pub fn init() -> Router<AppState> {
  Router::new()
    .route("/sys/user/query", post(get_users))
    .route("/sys/user/save", post(save_user))
    .route("/sys/user/delete", post(delete_user))
}

#[utoipa::path(
  post,
  path = "/sys/user/query",
  responses((status = 200, description = "Getting for all users.", body = QueryUserResponse)),
  tag = ""
)]
pub async fn get_users(
  State(state): State<AppState>,
  //Query(param): Query<QueryUserRequest>
  Json(param): Json<QueryUserRequest>
) -> Result<Json<QueryUserResponse>, StatusCode> {
  let handler = UserHandler::new(&state);
  match handler.find(param).await {
    Ok((page, data)) => Ok(Json(QueryUserResponse::new(page, data))),
    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
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
) -> Result<Json<SaveUserResponse>, StatusCode> {
  let handler: UserHandler = UserHandler::new(&state);
  match handler.save(param).await {
    Ok(result) => Ok(Json(SaveUserResponse::new(result))),
    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
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
) -> Result<Json<DeleteUserResponse>, StatusCode> {
  let handler = UserHandler::new(&state);
  match handler.delete(param).await {
    Ok(result) => Ok(Json(DeleteUserResponse::new(result))),
    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
  }
}
