use axum::{
    extract::{ Json, Query, State },
    http::StatusCode,
    response::IntoResponse,
    routing::{ get, post },
    Router,
};

use crate::{
    context::state::AppState,
    handlers::folders::IFolderHandler,
    types::{
        folders::{ DeleteFolderResponse, QueryFolderResponse, SaveFolderResponse },
        PageRequest,
    },
    utils::auths::SecurityContext,
};
use crate::handlers::folders::FolderHandler;
use crate::types::folders::{ QueryFolderRequest, SaveFolderRequest, DeleteFolderRequest };

use super::ValidatedJson;

pub fn init() -> Router<AppState> {
    Router::new()
        .route("/modules/folder/query", get(handle_query_folders))
        .route("/modules/folder/save", post(handle_save_folder))
        .route("/modules/folder/delete", post(handle_delete_folder))
}

#[utoipa::path(
    get,
    path = "/modules/folder/query",
    params(QueryFolderRequest, PageRequest),
    responses((status = 200, description = "Getting for all folders.", body = QueryFolderResponse)),
    tag = "Folder"
)]
pub async fn handle_query_folders(
    State(state): State<AppState>,
    Query(param): Query<QueryFolderRequest>,
    Query(page): Query<PageRequest>
) -> impl IntoResponse {
    let cur_folder = SecurityContext::get_instance().get().await;
    tracing::info!("current folder: {:?}", cur_folder);

    match get_folder_handler(&state).find(param, page).await {
        Ok((page, data)) => Ok(Json(QueryFolderResponse::new(page, data))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/modules/folder/save",
    request_body = SaveFolderRequest,
    responses((status = 200, description = "Save for folder.", body = SaveFolderResponse)),
    tag = "Folder"
)]
async fn handle_save_folder(
    State(state): State<AppState>,
    ValidatedJson(param): ValidatedJson<SaveFolderRequest>
) -> impl IntoResponse {
    match get_folder_handler(&state).save(param).await {
        Ok(result) => Ok(Json(SaveFolderResponse::new(result))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/modules/folder/delete",
    request_body = DeleteFolderRequest,
    responses((status = 200, description = "Delete for folder.", body = DeleteFolderResponse)),
    tag = "Folder"
)]
async fn handle_delete_folder(
    State(state): State<AppState>,
    Json(param): Json<DeleteFolderRequest>
) -> impl IntoResponse {
    match get_folder_handler(&state).delete(param).await {
        Ok(result) => Ok(Json(DeleteFolderResponse::new(result))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

fn get_folder_handler(state: &AppState) -> Box<dyn IFolderHandler + '_> {
    Box::new(FolderHandler::new(state))
}
