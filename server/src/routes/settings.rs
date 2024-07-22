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
    handlers::settings::ISettingsHandler,
    types::{
        settings::{ DeleteSettingsResponse, QuerySettingsResponse, SaveSettingsResponse },
        PageRequest,
    },
    utils::auths::SecurityContext,
};
use crate::handlers::settings::SettingsHandler;
use crate::types::settings::{ QuerySettingsRequest, SaveSettingsRequest, DeleteSettingsRequest };

use super::ValidatedJson;

pub fn init() -> Router<AppState> {
    Router::new()
        .route("/sys/settings/query", get(handle_query_settings))
        .route("/sys/settings/save", post(handle_save_settings))
        .route("/sys/settings/delete", post(handle_delete_settings))
}

#[utoipa::path(
    get,
    path = "/sys/settings/query",
    params(QuerySettingsRequest, PageRequest),
    responses((
        status = 200,
        description = "Getting for all settings.",
        body = QuerySettingsResponse,
    )),
    tag = "Settings"
)]
pub async fn handle_query_settings(
    State(state): State<AppState>,
    Query(param): Query<QuerySettingsRequest>,
    Query(page): Query<PageRequest>
) -> impl IntoResponse {
    let cur_settings = SecurityContext::get_instance().get().await;
    tracing::info!("current settings: {:?}", cur_settings);

    match get_settings_handler(&state).find(param, page).await {
        Ok((page, data)) => Ok(Json(QuerySettingsResponse::new(page, data))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/sys/settings/save",
    request_body = SaveSettingsRequest,
    responses((status = 200, description = "Save for settings.", body = SaveSettingsResponse)),
    tag = "Settings"
)]
async fn handle_save_settings(
    State(state): State<AppState>,
    ValidatedJson(param): ValidatedJson<SaveSettingsRequest>
) -> impl IntoResponse {
    match get_settings_handler(&state).save(param).await {
        Ok(result) => Ok(Json(SaveSettingsResponse::new(result))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/sys/settings/delete",
    request_body = DeleteSettingsRequest,
    responses((status = 200, description = "Delete for settings.", body = DeleteSettingsResponse)),
    tag = "Settings"
)]
async fn handle_delete_settings(
    State(state): State<AppState>,
    Json(param): Json<DeleteSettingsRequest>
) -> impl IntoResponse {
    match get_settings_handler(&state).delete(param).await {
        Ok(result) => Ok(Json(DeleteSettingsResponse::new(result))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

fn get_settings_handler(state: &AppState) -> Box<dyn ISettingsHandler + '_> {
    Box::new(SettingsHandler::new(state))
}
