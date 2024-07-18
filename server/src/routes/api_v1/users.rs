use axum::{
    extract::{ Json, Query, State },
    http::StatusCode,
    response::IntoResponse,
    routing::{ get, post },
    Router,
};

use crate::{
    context::state::AppState,
    handlers::api_v1::users::{ ApiV1Handler, IApiV1Handler },
    routes::ValidatedJson,
    types::{
        api_v1::users::{
            DeleteUserApiV1Request,
            DeleteUserApiV1Response,
            QueryUserApiV1Request,
            QueryUserApiV1Response,
            SaveUserApiV1Request,
            SaveUserApiV1Response,
        },
        PageRequest,
    },
};

pub fn init() -> Router<AppState> {
    Router::new()
        .route("/api/v1/user/query", get(handle_get_users))
        .route("/api/v1/user/save", post(handle_save_user))
        .route("/api/v1/user/delete", post(handle_delete_user))
}

#[utoipa::path(
    get,
    path = "/api/v1/user/query",
    params(QueryUserApiV1Request, PageRequest),
    responses((
        status = 200,
        description = "API Getting for all users.",
        body = QueryUserApiV1Response,
    )),
    tag = ""
)]
pub async fn handle_get_users(
    State(state): State<AppState>,
    Query(param): Query<QueryUserApiV1Request>,
    Query(page): Query<PageRequest>
) -> impl IntoResponse {
    match get_apiv1_handler(&state).find(param, page).await {
        Ok((page, data)) => Ok(Json(QueryUserApiV1Response::new(page, data))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/user/save",
    request_body = SaveUserApiV1Request,
    responses((status = 200, description = "API Save for user.", body = SaveUserApiV1Response)),
    tag = ""
)]
async fn handle_save_user(
    State(state): State<AppState>,
    ValidatedJson(param): ValidatedJson<SaveUserApiV1Request>
) -> impl IntoResponse {
    match get_apiv1_handler(&state).save(param).await {
        Ok(result) => Ok(Json(SaveUserApiV1Response::new(result))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/user/delete",
    request_body = DeleteUserApiV1Request,
    responses((status = 200, description = "API Delete for user.", body = DeleteUserApiV1Response)),
    tag = ""
)]
async fn handle_delete_user(
    State(state): State<AppState>,
    Json(param): Json<DeleteUserApiV1Request>
) -> impl IntoResponse {
    match get_apiv1_handler(&state).delete(param).await {
        Ok(result) => Ok(Json(DeleteUserApiV1Response::new(result))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

fn get_apiv1_handler(state: &AppState) -> Box<dyn IApiV1Handler + '_> {
    Box::new(ApiV1Handler::new(state))
}
