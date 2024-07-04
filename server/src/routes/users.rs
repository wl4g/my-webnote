use axum::{
  routing::get,
  Router,
  extract::{ State, Query, Json },
  response::IntoResponse,
  http::StatusCode,
};
use crate::context::state::AppState;
use crate::models::users::User;
use crate::handlers::users::UserHandler;
use crate::models::users::QueryUserRequest;

pub fn init() -> Router<AppState> {
  Router::new().route("/sys/user/query", get(get_users))
}

pub async fn get_users(
  State(state): State<AppState>,
  Query(param): Query<QueryUserRequest>
) -> impl IntoResponse {
  let handler = UserHandler::new(&state);
  match handler.get_users().await {
    Ok(users) => (StatusCode::OK, Json(users)).into_response(),
    Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
  }
}

async fn create_user(State(state): State<AppState>, Json(user): Json<User>) -> impl IntoResponse {
  let handler = UserHandler::new(&state);
  match handler.create_user(user).await {
    Ok(created_user) => (StatusCode::CREATED, Json(created_user)).into_response(),
    Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
  }
}
