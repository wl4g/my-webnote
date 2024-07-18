use axum::{
    extract::{ Json, Query, State },
    http::StatusCode,
    response::IntoResponse,
    routing::{ get, post },
    Router,
};

use crate::{
    context::state::AppState,
    handlers::users::IUserHandler,
    types::{ users::{ DeleteUserResponse, QueryUserResponse, SaveUserResponse }, PageRequest },
};
use crate::handlers::users::UserHandler;
use crate::types::users::{ QueryUserRequest, SaveUserRequest, DeleteUserRequest };

use super::ValidatedJson;

pub fn init() -> Router<AppState> {
    Router::new()
        .route("/sys/user/query", get(handle_get_users))
        .route("/sys/user/save", post(handle_save_user))
        .route("/sys/user/delete", post(handle_delete_user))
}

#[utoipa::path(
    get,
    path = "/sys/user/query",
    params(QueryUserRequest, PageRequest),
    responses((status = 200, description = "Getting for all users.", body = QueryUserResponse)),
    tag = ""
)]
pub async fn handle_get_users(
    State(state): State<AppState>,
    Query(param): Query<QueryUserRequest>,
    Query(page): Query<PageRequest>
) -> impl IntoResponse {
    match get_user_handler(&state).find(param, page).await {
        Ok((page, data)) => Ok(Json(QueryUserResponse::new(page, data))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/sys/user/save",
    request_body = SaveUserRequest,
    responses((status = 200, description = "Save for user.", body = SaveUserResponse)),
    tag = ""
)]
async fn handle_save_user(
    State(state): State<AppState>,
    ValidatedJson(param): ValidatedJson<SaveUserRequest>
) -> impl IntoResponse {
    match get_user_handler(&state).save(param).await {
        Ok(result) => Ok(Json(SaveUserResponse::new(result))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/sys/user/delete",
    request_body = DeleteUserRequest,
    responses((status = 200, description = "Delete for user.", body = DeleteUserResponse)),
    tag = ""
)]
async fn handle_delete_user(
    State(state): State<AppState>,
    Json(param): Json<DeleteUserRequest>
) -> impl IntoResponse {
    match get_user_handler(&state).delete(param).await {
        Ok(result) => Ok(Json(DeleteUserResponse::new(result))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

fn get_user_handler(state: &AppState) -> Box<dyn IUserHandler + '_> {
    Box::new(UserHandler::new(state))
}
