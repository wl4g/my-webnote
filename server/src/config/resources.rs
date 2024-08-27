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

use axum::{ extract::State, response::IntoResponse };
use hyper::{ header, StatusCode, Uri };
use rust_embed::RustEmbed;

use crate::{ context::state::AppState, utils::auths };

#[derive(RustEmbed)]
#[folder = "static/"]
struct Asset;

pub async fn handle_static(State(state): State<AppState>, uri: Uri) -> impl IntoResponse {
    let mut path = auths::clean_context_path(&state.config.server.context_path, uri.path());
    path = path.trim_start_matches("/static/").trim_start_matches('/');

    let context_path = &state.config.server.context_path.to_owned().unwrap_or_default();
    match Asset::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            // Check if the content is HTML.
            if mime.essence_str() == "text/html" {
                let html_content = String::from_utf8_lossy(&content.data);
                // Replace with the actual context path.
                let modified_content = html_content.replace(r#"{{context_path}}"#, context_path);
                (
                    StatusCode::OK,
                    [(header::CONTENT_TYPE, mime.as_ref())],
                    modified_content.into_bytes(),
                ).into_response()
            } else {
                // For non-HTML content, directly response.
                (
                    StatusCode::OK,
                    [(header::CONTENT_TYPE, mime.as_ref())],
                    content.data,
                ).into_response()
            }
        }
        None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
    }
}
