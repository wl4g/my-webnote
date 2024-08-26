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

use std::{ ops::Deref, sync::Arc, time::Duration };

use anyhow::Ok;
use globset::{ Glob, GlobSet, GlobSetBuilder };
use serde::Deserialize;
// use std::fs::File;
// use std::io::Read;
// use std::path::Path;
use config::Config;
use validator::Validate;

use crate::monitoring::health::HEALTHZ_URI;

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct ApiProperties {
    #[serde(rename = "service-name")]
    #[validate(length(min = 1, max = 32))]
    pub service_name: String,
    #[serde(default = "ServerProperties::default")]
    pub server: ServerProperties,
    #[serde(default = "LoggingProperties::default")]
    pub logging: LoggingProperties,
    #[serde(default = "DbProperties::default")]
    pub db: DbProperties,
    #[serde(default = "CacheProperties::default")]
    pub cache: CacheProperties,
    #[serde(default = "AuthProperties::default")]
    pub auth: AuthProperties,
    #[serde(default = "SwaggerProperties::default")]
    pub swagger: SwaggerProperties,
    #[serde(default = "MonitoringProperties::default")]
    pub monitoring: MonitoringProperties,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerProperties {
    pub bind: String,
    #[serde(rename = "mgmt-bind")]
    pub mgmt_bind: String,
    #[serde(rename = "thread-max-pool")]
    pub thread_max_pool: u32,
    #[serde(default = "CorsProperties::default")]
    pub cors: CorsProperties,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CorsProperties {
    pub hosts: Vec<String>,
    pub headers: Vec<String>,
    pub methods: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingProperties {
    pub file: String,
    pub pattern: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DbProperties {
    #[serde(rename = "type")]
    pub db_type: DbType,
    pub sqlite: SqliteProperties,
    pub mongo: MongoProperties,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum DbType {
    Sqlite,
    Mongo,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SqliteProperties {
    pub dir: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MongoProperties {
    pub url: Option<String>,
    pub database: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CacheProperties {
    pub provider: CacheProvider,
    pub memory: MemoryProperties,
    pub redis: RedisProperties,
}

#[derive(Debug, Deserialize, Clone)]
pub enum CacheProvider {
    Memory,
    Redis,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MemoryProperties {
    #[serde(rename = "initial-capacity")]
    pub initial_capacity: Option<u32>,
    #[serde(rename = "max-capacity")]
    pub max_capacity: Option<u64>,
    pub ttl: Option<u64>,
    #[serde(rename = "eviction-policy")]
    pub eviction_policy: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisProperties {
    pub nodes: Vec<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    #[serde(rename = "connection-timeout")]
    pub connection_timeout: Option<u64>,
    #[serde(rename = "response-timeout")]
    pub response_timeout: Option<u64>,
    pub retries: Option<u32>,
    #[serde(rename = "max-retry-wait")]
    pub max_retry_wait: Option<u64>,
    #[serde(rename = "min-retry-wait")]
    pub min_retry_wait: Option<u64>,
    #[serde(rename = "read-from-replicas")]
    pub read_from_replicas: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthProperties {
    #[serde(rename = "jwt-ak-name")]
    pub jwt_ak_name: Option<String>,
    #[serde(rename = "jwt-rk-name")]
    pub jwt_rk_name: Option<String>,
    #[serde(rename = "jwt-validity-ak")]
    pub jwt_validity_ak: Option<u64>,
    #[serde(rename = "jwt-validity-rk")]
    pub jwt_validity_rk: Option<u64>,
    #[serde(rename = "jwt-secret")]
    pub jwt_secret: Option<String>,
    #[serde(rename = "anonymous-paths")]
    pub anonymous_paths: Option<Vec<String>>,
    pub oidc: OidcProperties,
    pub github: GithubProperties,
    #[serde(rename = "login-url")]
    pub login_url: Option<String>,
    #[serde(rename = "success-url")]
    pub success_url: Option<String>,
    #[serde(rename = "unauthz-url")]
    pub unauthz_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OidcProperties {
    pub enabled: Option<bool>,
    #[serde(rename = "client-id")]
    pub client_id: Option<String>,
    #[serde(rename = "client-secret")]
    pub client_secret: Option<String>,
    #[serde(rename = "issue-url")]
    pub issue_url: Option<String>,
    #[serde(rename = "redirect-url")]
    pub redirect_url: Option<String>,
    #[serde(rename = "scope")]
    pub scope: Option<String>,
}

// see:https://github.com/settings/developers
// see:https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps
#[derive(Debug, Deserialize, Clone)]
pub struct OAuth2Properties {
    pub enabled: Option<bool>,
    #[serde(rename = "client-id")]
    pub client_id: Option<String>,
    #[serde(rename = "client-secret")]
    pub client_secret: Option<String>,
    #[serde(rename = "auth-url")]
    pub auth_url: Option<String>,
    #[serde(rename = "token-url")]
    pub token_url: Option<String>,
    #[serde(rename = "redirect-url")]
    pub redirect_url: Option<String>,
    // see:https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/scopes-for-oauth-apps
    #[serde(rename = "scope")]
    pub scope: Option<String>,
    #[serde(rename = "user-info-url")]
    pub user_info_url: Option<String>,
}

// see:https://github.com/settings/developers
// see:https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps
#[derive(Debug, Deserialize, Clone)]
pub struct GithubProperties(OAuth2Properties);

// Copy all OAuth2Config functions to GithubConfig.
impl Deref for GithubProperties {
    type Target = OAuth2Properties;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct SwaggerProperties {
    pub enabled: bool,
    // pub title: String,
    // pub description: String,
    // pub version: String,
    // pub license_name: String,
    // pub license_url: String,
    // pub contact_name: String,
    // pub contact_email: String,
    // pub contact_url: String,
    // pub terms_of_service: String,
    // //pub security_definitions: vec![],
    pub swagger_ui_path: String,
    pub swagger_openapi_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MonitoringProperties {
    pub enabled: bool,
    #[serde(default = "OtelProperties::default")]
    pub otel: OtelProperties,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OtelProperties {
    pub endpoint: String,
    pub protocol: String,
    pub timeout: Option<u64>,
    // Notice: More OTEL custom configuration use to environment: OTEL_SPAN_xxx, see to: opentelemetry_sdk::trace::config::default()
}

impl ApiProperties {
    pub fn default() -> ApiProperties {
        ApiProperties {
            service_name: String::from("the-mywebnote"),
            server: ServerProperties::default(),
            logging: LoggingProperties::default(),
            db: DbProperties::default(),
            cache: CacheProperties::default(),
            auth: AuthProperties::default(),
            swagger: SwaggerProperties::default(),
            monitoring: MonitoringProperties::default(),
        }
    }

    pub fn validate(self) -> Result<ApiProperties, anyhow::Error> {
        //self.validate();
        Ok(self)
    }

    pub fn to_use_config(&self) -> Arc<ApiConfig> {
        ApiConfig::new(&self)
    }

    // see:https://github.com/mehcode/config-rs/blob/master/examples/simple/main.rs
    pub fn parse(path: &String) -> ApiProperties {
        // serde_yaml::from_str(&contents)?;

        let config = Config::builder()
            .add_source(config::File::with_name(path))
            .add_source(config::Environment::with_prefix("MYWEBNOTE"))
            .build()
            .unwrap_or_else(|err| panic!("Error parsing config: {}", err))
            .try_deserialize::<ApiProperties>()
            .unwrap_or_else(|err| panic!("Error deserialize config: {}", err));

        config
    }
}

impl Default for ServerProperties {
    fn default() -> Self {
        ServerProperties {
            bind: "0.0.0.0:8888".to_string(),
            mgmt_bind: "0.0.0.0:11700".to_string(),
            thread_max_pool: 4,
            cors: CorsProperties::default(),
        }
    }
}

impl Default for CorsProperties {
    fn default() -> Self {
        CorsProperties {
            hosts: vec!["*".to_string()],
            headers: vec!["*".to_string()],
            methods: vec!["*".to_string()],
        }
    }
}

impl Default for LoggingProperties {
    fn default() -> Self {
        LoggingProperties {
            file: "info".to_string(),
            pattern: "pretty".to_string(),
        }
    }
}

impl Default for DbProperties {
    fn default() -> Self {
        DbProperties {
            db_type: DbType::Sqlite,
            sqlite: SqliteProperties::default(),
            mongo: MongoProperties::default(),
        }
    }
}

impl Default for SqliteProperties {
    fn default() -> Self {
        SqliteProperties {
            dir: Some(String::from("/tmp/mywebnote_db")),
        }
    }
}

impl Default for MongoProperties {
    fn default() -> Self {
        MongoProperties {
            url: Some(String::from("mongodb://localhost:27017")),
            database: Some(String::from("mywebnote")),
        }
    }
}

impl Default for OidcProperties {
    fn default() -> Self {
        OidcProperties {
            enabled: Some(false),
            client_id: None,
            client_secret: None,
            issue_url: None,
            redirect_url: None,
            scope: Some("openid profile email".to_string()),
        }
    }
}

impl Default for OAuth2Properties {
    fn default() -> Self {
        OAuth2Properties {
            enabled: Some(false),
            client_id: None,
            client_secret: None,
            auth_url: None,
            token_url: None,
            redirect_url: None,
            // see:https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/scopes-for-oauth-apps
            scope: Some(
                "openid profile user:email user:follow read:user read:project public_repo".to_string()
            ),
            user_info_url: None,
        }
    }
}

impl Default for GithubProperties {
    fn default() -> Self {
        // Beautifully impls for like java extends.
        GithubProperties(OAuth2Properties::default())
    }
}

impl Default for CacheProperties {
    fn default() -> Self {
        CacheProperties {
            provider: CacheProvider::Memory,
            memory: MemoryProperties::default(),
            redis: RedisProperties::default(),
        }
    }
}

impl Default for MemoryProperties {
    fn default() -> Self {
        MemoryProperties {
            initial_capacity: Some(32),
            max_capacity: Some(65535),
            ttl: Some(3600),
            eviction_policy: Some("lru".to_string()),
        }
    }
}

impl Default for RedisProperties {
    fn default() -> Self {
        RedisProperties {
            nodes: vec!["redis://127.0.0.1:6379".to_string()],
            username: None,
            password: None,
            connection_timeout: Some(3000),
            response_timeout: Some(6000),
            retries: Some(1),
            max_retry_wait: Some(65536),
            min_retry_wait: Some(1280),
            read_from_replicas: Some(false),
        }
    }
}

impl Default for AuthProperties {
    fn default() -> Self {
        AuthProperties {
            jwt_ak_name: Some(String::from("_ak")),
            jwt_rk_name: Some(String::from("_rk")),
            jwt_validity_ak: Some(3600_000),
            jwt_validity_rk: Some(86400_000),
            jwt_secret: Some("changeit".to_string()),
            anonymous_paths: None,
            oidc: OidcProperties::default(),
            github: GithubProperties::default(),
            login_url: Some(String::from("/static/login.html")),
            success_url: Some(String::from("/static/index.html")),
            unauthz_url: Some(String::from("/static/403.html")),
        }
    }
}

impl Default for SwaggerProperties {
    fn default() -> Self {
        SwaggerProperties {
            enabled: true,
            // title: "My Webnote API Server".to_string(),
            // description: "The My Webnote API Server".to_string(),
            // version: "1.0.0".to_string(),
            // license_name: "Apache 2.0".to_string(),
            // license_url: "https://www.apache.org/licenses/LICENSE-2.0".to_string(),
            // contact_name: "MyWebnote API".to_string(),
            // contact_email: "jameswong1376@gmail.com".to_string(),
            // contact_url: "https://github.com/wl4g/my-webnote".to_string(),
            // terms_of_service: "api/terms-of-service".to_string(),
            // //security_definitions: vec![],
            swagger_ui_path: "/swagger-ui".to_string(),
            swagger_openapi_url: "/api-docs/openapi.json".to_string(),
        }
    }
}

impl Default for MonitoringProperties {
    fn default() -> Self {
        MonitoringProperties {
            enabled: true,
            otel: OtelProperties::default(),
        }
    }
}

impl Default for OtelProperties {
    fn default() -> Self {
        OtelProperties {
            endpoint: String::from("http://localhost:4317"),
            protocol: String::from("grpc"),
            timeout: Some(Duration::from_secs(10).as_millis() as u64),
        }
    }
}

pub struct ApiConfig {
    pub inner: ApiProperties,
    pub auth_jwt_ak_name: String,
    pub auth_jwt_rk_name: String,
    pub auth_anonymous_glob_matcher: Option<GlobSet>,
}

impl Deref for ApiConfig {
    type Target = ApiProperties;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ApiConfig {
    pub fn new(config: &ApiProperties) -> Arc<ApiConfig> {
        // Build to auth anonymous glob matcher.
        let globset;
        if config.auth.anonymous_paths.is_some() {
            let mut builder = GlobSetBuilder::new();
            for path in config.auth.anonymous_paths.as_ref().unwrap() {
                builder.add(Glob::new(path).unwrap());
            }
            globset = Some(builder.build().unwrap());
        } else {
            // Add internal components routes to defaults.
            let mut builder = GlobSetBuilder::new();
            builder.add(Glob::new(HEALTHZ_URI).unwrap());
            builder.add(Glob::new(format!("{}/**", HEALTHZ_URI).as_str()).unwrap());
            // The default accessing to swagger ui required authentication.
            //builder.add(Glob::new(&config.swagger.swagger_ui_path).unwrap());
            //builder.add(Glob::new(&config.swagger.swagger_openapi_url).unwrap());
            builder.add(Glob::new("/public/**").unwrap());
            builder.add(Glob::new("/static/**").unwrap());
            globset = Some(builder.build().unwrap());
        }

        Arc::new(ApiConfig {
            inner: config.clone(),
            auth_jwt_ak_name: config.auth.jwt_ak_name
                .to_owned()
                .unwrap_or(String::from("_ak"))
                .to_string(),
            auth_jwt_rk_name: config.auth.jwt_rk_name
                .to_owned()
                .unwrap_or(String::from("_rk"))
                .to_string(),
            auth_anonymous_glob_matcher: globset,
        })
    }
}

pub const DEFAULT_INDEX_HTML: &str = include_str!("../../static/index.html");
pub const DEFAULT_LOGIN_HTML: &str = include_str!("../../static/login.html");
pub const DEFAULT_404_HTML: &str = include_str!("../../static/404.html");
pub const DEFAULT_403_HTML: &str = include_str!("../../static/403.html");
