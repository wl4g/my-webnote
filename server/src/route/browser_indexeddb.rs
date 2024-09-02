/*
 * SPDX-License-Identifier: GNU GENERAL PUBLIC LICENSE Version 3
 *
 * Copyleft (c) 2024 James Wong. This file is part of James Wong.
 * is free software: you can redistribute it and/or modify it under
 * the terms of the GNU General Public License as published by the
 * Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * James Wong is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with James Wong.  If not, see <https://www.gnu.org/licenses/>.
 *
 * IMPORTANT: Any software that fully or partially contains or uses materials
 * covered by this license must also be released under the GNU GPL license.
 * This includes modifications and derived works.
 */

use axum::{
    extract::{ Json, Query, State },
    http::StatusCode,
    response::IntoResponse,
    routing::{ get, post },
    Router,
};

use crate::{
    context::state::AppState,
    handler::browser_indexeddb_v2::{ BrowserIndexedDBHandlerImpl, IBrowserIndexedDBHandler },
    types::browser_indexeddb::{
        DeleteIndexedRecordRequest,
        DeleteIndexedRecordResponse,
        GetAllIndexedRecordRequest,
        GetAllIndexedRecordResponse,
        GetAllKeyIndexedRecordResponse,
        GetIndexedRecordRequest,
        GetIndexedRecordResponse,
        SaveIndexedRecordRequest,
        SaveIndexedRecordResponse,
    },
};

use super::ValidatedJson;

pub fn init() -> Router<AppState> {
    Router::new()
        .route("/modules/browser_indexeddb/get", get(handle_browser_indexeddb_get))
        .route("/modules/browser_indexeddb/get_all", get(handle_browser_indexeddb_get_all))
        .route("/modules/browser_indexeddb/get_all_key", get(handle_browser_indexeddb_get_all_keys))
        .route("/modules/browser_indexeddb/add", post(handle_add_browser_indexeddb))
        .route("/modules/browser_indexeddb/put", post(handle_put_browser_indexeddb))
        .route("/modules/browser_indexeddb/delete", post(handle_delete_browser_indexeddb))
}

#[utoipa::path(
    get,
    path = "/modules/browser_indexeddb/query",
    params(GetIndexedRecordRequest),
    responses((
        status = 200,
        description = "Getting record for browser indexeddbs.",
        body = QueryFolderResponse,
    )),
    tag = "Browser IndexedDB"
)]
pub async fn handle_browser_indexeddb_get(
    State(state): State<AppState>,
    Query(param): Query<GetIndexedRecordRequest>
) -> Result<Json<GetIndexedRecordResponse>, StatusCode> {
    match get_browser_indexeddb_handler(&state).get(param).await {
        Ok(res) => Ok(Json(GetIndexedRecordResponse::new(res))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    get,
    path = "/modules/browser_indexeddb/get_all",
    params(GetAllIndexedRecordRequest),
    responses((
        status = 200,
        description = "Getting record for browser indexeddbs.",
        body = QueryFolderResponse,
    )),
    tag = "Browser IndexedDB"
)]
pub async fn handle_browser_indexeddb_get_all(
    State(state): State<AppState>,
    Query(param): Query<GetAllIndexedRecordRequest>
) -> impl IntoResponse {
    match get_browser_indexeddb_handler(&state).get_all(param).await {
        Ok(res) => Ok(Json(GetAllIndexedRecordResponse::new(res))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    get,
    path = "/modules/browser_indexeddb/get_all_keys",
    params(GetAllIndexedRecordRequest),
    responses((
        status = 200,
        description = "Getting keys for browser indexeddbs.",
        body = QueryFolderResponse,
    )),
    tag = "Browser IndexedDB"
)]
pub async fn handle_browser_indexeddb_get_all_keys(
    State(state): State<AppState>,
    Query(param): Query<GetAllIndexedRecordRequest>
) -> impl IntoResponse {
    match get_browser_indexeddb_handler(&state).get_all_keys(param).await {
        Ok(res) => Ok(Json(GetAllKeyIndexedRecordResponse::new(res))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/modules/browser_indexeddb/add",
    request_body = SaveIndexedRecordRequest,
    responses((
        status = 200,
        description = "Add for browser_indexeddb.",
        body = SaveIndexedRecordResponse,
    )),
    tag = "Browser IndexedDB"
)]
async fn handle_add_browser_indexeddb(
    State(state): State<AppState>,
    ValidatedJson(param): ValidatedJson<SaveIndexedRecordRequest>
) -> impl IntoResponse {
    match get_browser_indexeddb_handler(&state).add(param).await {
        Ok(res) => Ok(Json(SaveIndexedRecordResponse::new(res))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/modules/browser_indexeddb/put",
    request_body = SaveIndexedRecordRequest,
    responses((
        status = 200,
        description = "Put for browser_indexeddb.",
        body = SaveIndexedRecordResponse,
    )),
    tag = "Browser IndexedDB"
)]
async fn handle_put_browser_indexeddb(
    State(state): State<AppState>,
    ValidatedJson(param): ValidatedJson<SaveIndexedRecordRequest>
) -> impl IntoResponse {
    match get_browser_indexeddb_handler(&state).put(param).await {
        Ok(res) => Ok(Json(SaveIndexedRecordResponse::new(res))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/modules/browser_indexeddb/delete",
    request_body = DeleteFolderRequest,
    responses((
        status = 200,
        description = "Delete for browser_indexeddb.",
        body = DeleteFolderResponse,
    )),
    tag = "Browser IndexedDB"
)]
async fn handle_delete_browser_indexeddb(
    State(state): State<AppState>,
    Json(param): Json<DeleteIndexedRecordRequest>
) -> impl IntoResponse {
    match get_browser_indexeddb_handler(&state).delete(param).await {
        Ok(res) => Ok(Json(DeleteIndexedRecordResponse::new(res))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

fn get_browser_indexeddb_handler(state: &AppState) -> Box<dyn IBrowserIndexedDBHandler + '_> {
    Box::new(BrowserIndexedDBHandlerImpl::new(state))
}
