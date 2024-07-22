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
        .route("/api/v1/user/query", get(handle_apiv1_get_users))
        .route("/api/v1/user/save", post(handle_apiv1_save_user))
        .route("/api/v1/user/delete", post(handle_apiv1_delete_user))
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
    tag = "API/v1"
)]
pub async fn handle_apiv1_get_users(
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
    tag = "API/v1"
)]
async fn handle_apiv1_save_user(
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
    tag = "API/v1"
)]
async fn handle_apiv1_delete_user(
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
