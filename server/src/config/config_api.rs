use anyhow::Ok;
use serde::Deserialize;
// use std::fs::File;
// use std::io::Read;
// use std::path::Path;
use config::Config;

#[derive(Debug, Deserialize, Clone)]
pub struct ApiConfig {
  pub server: ServerConfig,
  pub logging: LoggingConfig,
  pub swagger: SwaggerConfig,
  pub db: DbConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
  pub bind: String,
  #[serde(rename = "mgmt-bind")]
  pub mgmt_bind: String,
  #[serde(rename = "thread-max-pool")]
  pub thread_max_pool: u32,
  pub cors: CorsConfig,
  pub auths: AuthConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CorsConfig {
  pub hosts: Vec<String>,
  pub headers: Vec<String>,
  pub methods: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
  pub oidc: OidcConfig,
  pub github: GithubConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OidcConfig {
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
}

// see:https://github.com/settings/developers
// see:https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps
#[derive(Debug, Deserialize, Clone)]
pub struct GithubConfig {
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
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
  pub file: String,
  pub pattern: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SwaggerConfig {
  pub enabled: bool,
  pub title: String,
  pub description: String,
  pub version: String,
  pub license_name: String,
  pub license_url: String,
  pub contact_name: String,
  pub contact_email: String,
  pub contact_url: String,
  pub terms_of_service: String,
  //pub security_definitions: vec![],
  pub swagger_ui_path: String,
  pub swagger_openapi_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DbConfig {
  #[serde(rename = "type")]
  pub db_type: DbType,
  pub sqlite: SqliteConfig,
  pub mongo: MongoConfig,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum DbType {
  Sqlite,
  Mongo,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SqliteConfig {
  pub dir: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MongoConfig {
  pub url: String,
}

impl ApiConfig {
  pub fn default() -> ApiConfig {
    ApiConfig {
      server: ServerConfig::default(),
      logging: LoggingConfig::default(),
      swagger: SwaggerConfig::default(),
      db: DbConfig::default(),
    }
  }

  // see:https://github.com/mehcode/config-rs/blob/master/examples/simple/main.rs
  pub fn parse(path: &String) -> ApiConfig {
    // serde_yaml::from_str(&contents)?;

    let config = Config::builder()
      .add_source(config::File::with_name(path))
      .add_source(config::Environment::with_prefix("REVEZONE"))
      .build()
      .unwrap_or_else(|err| panic!("Error parsing config: {}", err))
      .try_deserialize::<ApiConfig>()
      .unwrap_or_else(|err| panic!("Error deserialize config: {}", err));

    config
  }

  pub fn validate(self) -> Result<ApiConfig, anyhow::Error> {
    // // Validate server configuration
    // if let server = &self.server {
    //   if let thread_max_pool = server.thread_max_pool {
    //     if thread_max_pool == 0 {
    //       anyhow::bail!("thread-max-pool must be greater than 0");
    //     }
    //   }
    // }

    // // Validate database configuration
    // if let revezone = &self.revezone {
    //   if let db = &revezone.db {
    //     if let db_type = &db.db_type {
    //       match db_type {
    //         DbType::Sqlite => {
    //           if db.sqlite.is_none() {
    //             anyhow::bail!("SQLite configuration is missing");
    //           }
    //         }
    //         DbType::Mongo => {
    //           if db.mongo {
    //             anyhow::bail!("MongoDB configuration is missing");
    //           }
    //         }
    //       }
    //     } else {
    //       anyhow::bail!("Database type is not specified");
    //     }
    //   } else {
    //     anyhow::bail!("Database configuration is missing");
    //   }
    // } else {
    //   anyhow::bail!("Revezone configuration is missing");
    // }

    Ok(self)
  }
}

impl Default for ServerConfig {
  fn default() -> Self {
    ServerConfig {
      bind: "0.0.0.0:8888".to_string(),
      mgmt_bind: "0.0.0.0:11700".to_string(),
      thread_max_pool: 4,
      cors: CorsConfig::default(),
      auths: AuthConfig::default(),
    }
  }
}

impl Default for CorsConfig {
  fn default() -> Self {
    CorsConfig {
      hosts: vec!["*".to_string()],
      headers: vec!["*".to_string()],
      methods: vec!["*".to_string()],
    }
  }
}

impl Default for AuthConfig {
  fn default() -> Self {
    AuthConfig {
      oidc: OidcConfig::default(),
      github: GithubConfig::default(),
    }
  }
}

impl Default for OidcConfig {
  fn default() -> Self {
    OidcConfig {
      enabled: Some(false),
      client_id: None,
      client_secret: None,
      auth_url: None,
      token_url: None,
      redirect_url: None,
    }
  }
}

impl Default for GithubConfig {
  fn default() -> Self {
    GithubConfig {
      enabled: Some(false),
      client_id: None,
      client_secret: None,
      auth_url: None,
      token_url: None,
      redirect_url: None,
    }
  }
}

impl Default for LoggingConfig {
  fn default() -> Self {
    LoggingConfig {
      file: "info".to_string(),
      pattern: "pretty".to_string(),
    }
  }
}

impl Default for SwaggerConfig {
  fn default() -> Self {
    SwaggerConfig {
      enabled: true,
      title: "Excalidraw Revezone API Server".to_string(),
      description: "The Excalidraw Revezone API Server".to_string(),
      version: "1.0.0".to_string(),
      license_name: "Apache 2.0".to_string(),
      license_url: "https://www.apache.org/licenses/LICENSE-2.0".to_string(),
      contact_name: "Revezone API".to_string(),
      contact_email: "jameswong1376@gmail.com".to_string(),
      contact_url: "https://github.com/wl4g/revezone".to_string(),
      terms_of_service: "api/terms-of-service".to_string(),
      //security_definitions: vec![],
      swagger_ui_path: "/swagger-ui".to_string(),
      swagger_openapi_url: "/api-docs/openapi.json".to_string(),
    }
  }
}

impl Default for DbConfig {
  fn default() -> Self {
    DbConfig {
      db_type: DbType::Sqlite,
      sqlite: SqliteConfig::default(),
      mongo: MongoConfig::default(),
    }
  }
}

impl Default for SqliteConfig {
  fn default() -> Self {
    SqliteConfig {
      dir: "/tmp/revezone_db".to_string(),
    }
  }
}

impl Default for MongoConfig {
  fn default() -> Self {
    MongoConfig {
      url: "mongodb://localhost:27017".to_string(),
    }
  }
}
