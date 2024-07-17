use std::sync::Arc;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use super::config_api::ApiConfig;
use crate::routes::auths::{
    __path_handle_connect_oidc,
    __path_handle_connect_github,
    __path_handle_callback_github,
    __path_handle_callback_oidc,
    __path_handle_logout,
};
use crate::routes::users::{ __path_get_users, __path_save_user, __path_delete_user };
use crate::types::{ BaseBean, PageRequest, PageResponse };
use crate::types::users::{
    User,
    QueryUserRequest,
    QueryUserResponse,
    SaveUserRequest,
    SaveUserResponse,
    DeleteUserRequest,
    DeleteUserResponse,
};
use crate::types::auths::{ CallbackGithubRequest, CallbackOidcRequest, LogoutRequest };

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
        handle_connect_oidc,
        handle_connect_github,
        handle_callback_github,
        handle_callback_oidc,
        handle_logout,
        get_users,
        save_user,
        delete_user
    ),
    components(
        schemas(
            BaseBean,
            PageRequest,
            PageResponse,
            CallbackOidcRequest,
            CallbackGithubRequest,
            LogoutRequest,
            User,
            QueryUserRequest,
            QueryUserResponse,
            SaveUserRequest,
            SaveUserResponse,
            DeleteUserRequest,
            DeleteUserResponse
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
