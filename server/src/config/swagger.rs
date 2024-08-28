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

use std::collections::BTreeMap;
use std::sync::Arc;

use utoipa::OpenApi;
use utoipa::openapi::{ Paths, PathItem };
use utoipa_swagger_ui::SwaggerUi;

use super::config_serve::{ self, WebServeConfig };
use crate::{
    route::{
        api_v1::users::{
            __path_handle_apiv1_delete_user,
            __path_handle_apiv1_get_users,
            __path_handle_apiv1_save_user,
        },
        auths::{
            __path_handle_callback_github,
            __path_handle_callback_oidc,
            __path_handle_connect_github,
            __path_handle_connect_oidc,
            __path_handle_logout,
            __path_handle_password_pubkey,
            __path_handle_password_verify,
        },
        document::{
            __path_handle_delete_document,
            __path_handle_query_documents,
            __path_handle_save_document,
        },
        folder::{
            __path_handle_delete_folder,
            __path_handle_query_folders,
            __path_handle_save_folder,
        },
        settings::{
            __path_handle_delete_settings,
            __path_handle_query_settings,
            __path_handle_save_settings,
        },
        user::{
            __path_handle_delete_user,
            __path_handle_get_current_user,
            __path_handle_post_current_user,
            __path_handle_query_users,
            __path_handle_save_user,
        },
    },
    utils::auths,
};

use crate::types::{
    BaseBean,
    PageRequest,
    PageResponse,
    auth::{
        CallbackGithubRequest,
        CallbackOidcRequest,
        PasswordPubKeyRequest,
        PasswordPubKeyResponse,
        PasswordLoginRequest,
        LogoutRequest,
    },
    user::{
        User,
        QueryUserRequest,
        QueryUserResponse,
        SaveUserRequest,
        SaveUserRequestWith,
        SaveUserResponse,
        DeleteUserRequest,
        DeleteUserResponse,
    },
    api_v1::users::{
        QueryUserApiV1Request,
        QueryUserApiV1Response,
        SaveUserApiV1Request,
        SaveUserApiV1Response,
        DeleteUserApiV1Request,
        DeleteUserApiV1Response,
    },
    document::{
        Document,
        QueryDocumentRequest,
        QueryDocumentResponse,
        SaveDocumentRequest,
        SaveDocumentResponse,
        DeleteDocumentRequest,
        DeleteDocumentResponse,
        DocumentType,
    },
    folder::{
        Folder,
        QueryFolderRequest,
        QueryFolderResponse,
        SaveFolderRequest,
        SaveFolderResponse,
        DeleteFolderRequest,
        DeleteFolderResponse,
    },
    settings::{
        Settings,
        QuerySettingsRequest,
        QuerySettingsResponse,
        SaveSettingsRequest,
        SaveSettingsResponse,
        DeleteSettingsRequest,
        DeleteSettingsResponse,
    },
};

#[derive(utoipa::OpenApi)]
#[openapi(
    info(
        version = "1.0.0",
        title = "My Webnote API",
        description = "The My Webnote API",
        license(name = "Apache 2.0", url = "https://www.apache.org/licenses/LICENSE-2.0"),
        contact(
            name = "My Webnote",
            url = "https://github.com/wl4g/my-webnote",
            email = "jameswong1376@gmail.com"
        )
    ),
    //security((), "my_auth" = ["read:items", "edit:items"], "token_jwt" = []),
    external_docs(url = "https://github.com/wl4g/my-webnote", description = "More about our APIs"),
    paths(
        // Module of Auth
        handle_connect_oidc,
        handle_connect_github,
        handle_callback_github,
        handle_callback_oidc,
        handle_password_pubkey,
        handle_password_verify,
        handle_logout,
        // Module of User
        handle_get_current_user,
        handle_post_current_user,
        handle_query_users,
        handle_save_user,
        handle_delete_user,
        handle_apiv1_get_users,
        handle_apiv1_save_user,
        handle_apiv1_delete_user,
        // Module of Document
        handle_query_documents,
        handle_save_document,
        handle_delete_document,
        // Module of Folder
        handle_query_folders,
        handle_save_folder,
        handle_delete_folder,
        // Module of Settings
        handle_query_settings,
        handle_save_settings,
        handle_delete_settings
    ),
    components(
        schemas(
            // Module of Basic
            BaseBean,
            PageRequest,
            PageResponse,
            // Module of Auth
            CallbackOidcRequest,
            CallbackGithubRequest,
            PasswordPubKeyRequest,
            PasswordPubKeyResponse,
            PasswordLoginRequest,
            LogoutRequest,
            // Module of User
            User,
            QueryUserRequest,
            QueryUserResponse,
            SaveUserRequest,
            SaveUserRequestWith,
            SaveUserResponse,
            DeleteUserRequest,
            DeleteUserResponse,
            QueryUserApiV1Request,
            QueryUserApiV1Response,
            SaveUserApiV1Request,
            SaveUserApiV1Response,
            DeleteUserApiV1Request,
            DeleteUserApiV1Response,
            // Module of Document
            Document,
            QueryDocumentRequest,
            QueryDocumentResponse,
            SaveDocumentRequest,
            SaveDocumentResponse,
            DeleteDocumentRequest,
            DeleteDocumentResponse,
            DocumentType,
            // Module of Folder
            Folder,
            QueryFolderRequest,
            QueryFolderResponse,
            SaveFolderRequest,
            SaveFolderResponse,
            DeleteFolderRequest,
            DeleteFolderResponse,
            // Module of Settings
            Settings,
            QuerySettingsRequest,
            QuerySettingsResponse,
            SaveSettingsRequest,
            SaveSettingsResponse,
            DeleteSettingsRequest,
            DeleteSettingsResponse
        )
    ),
    modifiers(&ApiPathPrefixer)
)]
struct ApiDoc;

struct ApiPathPrefixer;

impl utoipa::Modify for ApiPathPrefixer {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let ctx_path = &config_serve::get_config().server.context_path;

        let old_paths = std::mem::take(&mut openapi.paths);
        let mut new_paths_map: BTreeMap<String, PathItem> = old_paths.paths
            .into_iter()
            .map(|(path, item)| {
                (
                    match ctx_path {
                        Some(cp) => format!("{}{}", cp, path), // Add the prefix context path.
                        None => path,
                    },
                    item,
                )
            })
            .collect();

        openapi.paths = Paths::new();
        openapi.paths.paths.append(&mut new_paths_map);
    }
}

pub fn init_swagger(config: &Arc<WebServeConfig>) -> SwaggerUi {
    // Manual build of OpenAPI.
    // use utoipa::openapi::{ ContactBuilder, InfoBuilder, LicenseBuilder, Paths };
    // let info = InfoBuilder::new()
    //   .title(config.swagger.title.to_string())
    //   .version(config.swagger.version.to_string())
    //   .description(Some(config.swagger.description.to_string()))
    //   .license(
    //       Some(
    //         LicenseBuilder::new()
    //           .name(config.swagger.license_name.to_string())
    //           .url(Some(config.swagger.license_url.to_string()))
    //           .build()
    //       )
    //     )
    //   .contact(
    //       Some(
    //         ContactBuilder::new()
    //           .name(Some(config.swagger.contact_name.to_string()))
    //           .url(Some(config.swagger.contact_url.to_string()))
    //           .email(Some(config.swagger.contact_email.to_string()))
    //           .build()
    //       )
    //     )
    //   .build();
    // let paths = Paths::new();
    // let openapi = utoipa::openapi::OpenApi::new(info, paths);

    // Auto build of OpenAPI.
    let openapi = ApiDoc::openapi();

    let swagger_ui_path = auths::join_context_path(
        config,
        config.swagger.swagger_ui_path.to_string()
    );
    let openapi_url = auths::join_context_path(
        config,
        config.swagger.swagger_openapi_url.to_string()
    );

    SwaggerUi::new(swagger_ui_path).url(openapi_url, openapi)
}
