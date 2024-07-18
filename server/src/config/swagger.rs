use std::sync::Arc;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use super::config_api::ApiConfig;
use crate::routes::{
    auths::{
        __path_handle_connect_oidc,
        __path_handle_connect_github,
        __path_handle_callback_github,
        __path_handle_callback_oidc,
        __path_handle_logout,
    },
    users::{ __path_handle_get_users, __path_handle_save_user, __path_handle_delete_user },
    api_v1::users::{
        __path_handle_apiv1_get_users,
        __path_handle_apiv1_save_user,
        __path_handle_apiv1_delete_user,
    },
    documents::{
        __path_handle_get_documents,
        __path_handle_save_document,
        __path_handle_delete_document,
    },
    folders::{ __path_handle_get_folders, __path_handle_save_folder, __path_handle_delete_folder },
    settings::{
        __path_handle_get_settings,
        __path_handle_save_settings,
        __path_handle_delete_settings,
    },
};

use crate::types::{
    BaseBean,
    PageRequest,
    PageResponse,
    auths::{ CallbackGithubRequest, CallbackOidcRequest, LogoutRequest },
    users::{
        User,
        QueryUserRequest,
        QueryUserResponse,
        SaveUserRequest,
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
    documents::{
        Document,
        QueryDocumentRequest,
        QueryDocumentResponse,
        SaveDocumentRequest,
        SaveDocumentResponse,
        DeleteDocumentRequest,
        DeleteDocumentResponse,
    },
    folders::{
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
        handle_logout,
        // Module of User
        handle_get_users,
        handle_save_user,
        handle_delete_user,
        handle_apiv1_get_users,
        handle_apiv1_save_user,
        handle_apiv1_delete_user,
        // Module of Document
        handle_get_documents,
        handle_save_document,
        handle_delete_document,
        // Module of Folder
        handle_get_folders,
        handle_save_folder,
        handle_delete_folder,
        // Module of Settings
        handle_get_settings,
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
            LogoutRequest,
            // Module of User
            User,
            QueryUserRequest,
            QueryUserResponse,
            SaveUserRequest,
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
    )
)]
struct ApiDoc;

pub fn init_swagger(config: &Arc<ApiConfig>) -> SwaggerUi {
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

    SwaggerUi::new(config.swagger.swagger_ui_path.to_string()).url(
        config.swagger.swagger_openapi_url.to_string(),
        openapi
    )
}
