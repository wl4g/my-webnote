use axum::{
    extract::{ Json, Query, State },
    http::StatusCode,
    response::IntoResponse,
    routing::{ get, post },
    Router,
};

use crate::{
    context::state::AppState,
    handler::folder::IFolderHandler,
    types::{
        folder::{ DeleteFolderResponse, QueryFolderResponse, SaveFolderResponse },
        PageRequest,
    },
    utils::auths::SecurityContext,
};
use crate::handler::folder::FolderHandler;
use crate::types::folder::{ QueryFolderRequest, SaveFolderRequest, DeleteFolderRequest };

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
