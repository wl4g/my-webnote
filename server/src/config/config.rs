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
  pub service: ServiceConfig,
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
  pub endpoint: Option<String>,
  #[serde(rename = "app-id")]
  pub app_id: Option<String>,
  pub app_secret: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GithubConfig {
  pub endpoint: Option<String>,
  #[serde(rename = "app-id")]
  pub app_id: Option<String>,
  pub app_secret: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
  pub file: String,
  pub pattern: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServiceConfig {
  pub db: DbConfig,
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
      service: ServiceConfig::default(),
    }
  }

  // see:https://github.com/mehcode/config-rs/blob/master/examples/simple/main.rs
  pub fn parse(path: &String) -> ApiConfig {
    let config = Config::builder()
      .add_source(config::File::with_name(path))
      .add_source(config::Environment::with_prefix("REVEZONE"))
      .build()
      .unwrap_or_else(|err| panic!("Error parsing config: {}", err))
      .try_deserialize::<ApiConfig>()
      .unwrap_or_else(|err| panic!("Error deserialize config: {}", err));

    config
  }

  //   pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
  //     let mut file = File::open(path)?;
  //     let mut contents = String::new();
  //     file.read_to_string(&mut contents)?;
  //     let config: RevezoneApiConfig = serde_yaml::from_str(&contents)?;
  //     Ok(config)
  //   }

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
      endpoint: None,
      app_id: None,
      app_secret: None,
    }
  }
}

impl Default for GithubConfig {
  fn default() -> Self {
    GithubConfig {
      endpoint: None,
      app_id: None,
      app_secret: None,
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

impl Default for ServiceConfig {
  fn default() -> Self {
    ServiceConfig {
      db: DbConfig::default(),
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

// macro_rules! generate_getters {
//   ($struct_name:ident, $($field:ident: $type:ty),+) => {
//         impl $struct_name {
//             $(
//                 pub fn $field(&self) -> $type {
//                     self.$field.clone().unwrap_or_else(|| $struct_name::default().$field.unwrap())
//                 }
//             )+
//         }
//   };
// }

// generate_getters!(ServerConfig,
//     bind: String,
//     mgmt_bind: String,
//     thread_max_pool: u32
// );
