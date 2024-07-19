use axum::{
    extract::{ Json, Query, State },
    http::StatusCode,
    response::IntoResponse,
    routing::{ get, post },
    Router,
};

use crate::{
    context::state::AppState,
    handlers::documents::IDocumentHandler,
    types::{
        documents::{ DeleteDocumentResponse, QueryDocumentResponse, SaveDocumentResponse },
        PageRequest,
    },
    utils::auths::SecurityContext,
};
use crate::handlers::documents::DocumentHandler;
use crate::types::documents::{ QueryDocumentRequest, SaveDocumentRequest, DeleteDocumentRequest };

use super::ValidatedJson;

pub fn init() -> Router<AppState> {
    Router::new()
        .route("/modules/document/query", get(handle_query_documents))
        .route("/modules/document/save", post(handle_save_document))
        .route("/modules/document/delete", post(handle_delete_document))
}

#[utoipa::path(
    get,
    path = "/modules/document/query",
    params(QueryDocumentRequest, PageRequest),
    responses((
        status = 200,
        description = "Getting for all documents.",
        body = QueryDocumentResponse,
    )),
    tag = "Document"
)]
pub async fn handle_query_documents(
    State(state): State<AppState>,
    Query(param): Query<QueryDocumentRequest>,
    Query(page): Query<PageRequest>
) -> impl IntoResponse {
    let cur_document = SecurityContext::get_instance().get().await;
    tracing::info!("current document: {:?}", cur_document);

    match get_document_handler(&state).find(param, page).await {
        Ok((page, data)) => Ok(Json(QueryDocumentResponse::new(page, data))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/modules/document/save",
    request_body = SaveDocumentRequest,
    responses((status = 200, description = "Save for document.", body = SaveDocumentResponse)),
    tag = "Document"
)]
async fn handle_save_document(
    State(state): State<AppState>,
    ValidatedJson(param): ValidatedJson<SaveDocumentRequest>
) -> impl IntoResponse {
    match get_document_handler(&state).save(param).await {
        Ok(result) => Ok(Json(SaveDocumentResponse::new(result))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/modules/document/delete",
    request_body = DeleteDocumentRequest,
    responses((status = 200, description = "Delete for document.", body = DeleteDocumentResponse)),
    tag = "Document"
)]
async fn handle_delete_document(
    State(state): State<AppState>,
    Json(param): Json<DeleteDocumentRequest>
) -> impl IntoResponse {
    match get_document_handler(&state).delete(param).await {
        Ok(result) => Ok(Json(DeleteDocumentResponse::new(result))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

fn get_document_handler(state: &AppState) -> Box<dyn IDocumentHandler + '_> {
    Box::new(DocumentHandler::new(state))
}
