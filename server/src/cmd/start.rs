use std::path::PathBuf;
use std::sync::Arc;
use anyhow::Ok;

use clap::{ Command, Arg };

use tracing::info;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;
use console_subscriber::ConsoleLayer;
// use tower_http::trace::TraceLayer;

use lazy_static::lazy_static;
use prometheus::{ Registry, Counter, Histogram, Encoder, TextEncoder };
use axum_prometheus::PrometheusMetricLayer;

use tokio::task::JoinHandle;
use tokio::sync::oneshot;

use axum::Router;
use axum::routing::get;

use crate::config::config::ApiConfig;
use crate::context::state::AppState;
use crate::routes::documents::init as document_router;
use crate::routes::folders::init as folder_router;
use crate::routes::settings::init as settings_router;
use crate::routes::users::init as user_router;

lazy_static! {
  pub static ref REGISTRY: Registry = Registry::new();

  pub static ref MY_HTTP_REQUESTS_TOTAL: Counter = Counter::new(
    "my_http_requests_total",
    "My Total number of HTTP requests"
  ).expect("My metric can be created");

  pub static ref MY_HTTP_REQUEST_DURATION: Histogram = Histogram::with_opts(
    prometheus::HistogramOpts::new(
      "http_request_duration_seconds",
      "My HTTP request duration in seconds"
    )
  ).expect("My metric can be created");
  // Register more metrics...
}

#[allow(unused)]
fn init_custom_metrics(config: &ApiConfig) {
  REGISTRY.register(Box::new(MY_HTTP_REQUESTS_TOTAL.clone())).expect("collector can be registered");
  REGISTRY.register(Box::new(MY_HTTP_REQUEST_DURATION.clone())).expect(
    "collector can be registered"
  );
  // Register more metrics...
}

#[allow(unused)]
async fn metrics() -> String {
  let encoder = TextEncoder::new();
  let mut buffer = Vec::new();
  encoder.encode(&REGISTRY.gather(), &mut buffer).unwrap();
  String::from_utf8(buffer).unwrap()
}

#[allow(unused)]
fn init_tracing(config: &ApiConfig) {
  // TODO using config setup logging.

  // Intialize setup logger levels.
  let env_filter = EnvFilter::try_from_default_env()
    .unwrap_or_else(|_| "debug".into())
    // .add_directive("debug".parse().unwrap()) // default level.
    .add_directive("playground_quickstart=debug".parse().unwrap())
    .add_directive("hyper=warn".parse().unwrap())
    .add_directive("tokio=trace".parse().unwrap()); // Notice: Must be at trace level to collect

  // Initialize layer with tokio console exporter.
  let console_layer = ConsoleLayer::builder().with_default_env().spawn();
  tracing_subscriber
    ::registry()
    .with(console_layer)
    .with(tracing_subscriber::fmt::layer())
    .with(env_filter)
    .init();
}

#[allow(unused)]
async fn start_server(config: ApiConfig) {
  let config_arc = Arc::new(config);
  let (prometheus_layer, _) = PrometheusMetricLayer::pair();

  let app_state = AppState::new(&config_arc);

  let app = Router::new()
    .merge(document_router())
    .merge(folder_router())
    .merge(settings_router())
    .merge(user_router())
    .layer(prometheus_layer)
    .with_state(app_state);
  // .route_layer(axum::Extension(app_state));
  //.layer(TraceLayer::new_for_http()); // Optional: add logs to tracing.

  let bind_addr = &config_arc.server.bind;
  info!("Starting API server on {}", bind_addr);

  axum
    ::serve(tokio::net::TcpListener::bind(&bind_addr).await.unwrap(), app.into_make_service()).await
    .unwrap();
}

#[allow(unused)]
async fn start_mgmt_server(
  config: &ApiConfig,
  signal_sender: oneshot::Sender<()>
) -> JoinHandle<()> {
  let app: Router = Router::new().route("/metrics", get(metrics));

  let bind_addr = config.server.mgmt_bind.clone();
  info!("Starting MGMT server on {}", bind_addr);

  tokio::spawn(async move {
    // TODO When started call to signal sender.
    let _ = signal_sender.send(());
    info!("Starting MGMT Axum server on {}", bind_addr);

    axum
      ::serve(
        tokio::net::TcpListener::bind(&bind_addr).await.unwrap(),
        app.into_make_service()
      ).await
      .unwrap();
  })
}

fn load_config(path: String) -> Result<ApiConfig, anyhow::Error> {
  if path.is_empty() { Ok(ApiConfig::default()) } else { Ok(ApiConfig::parse(&path).validate()?) }
}

pub fn build_cli() -> Command {
  Command::new("start")
    .about("Start API server.")
    // .arg_required_else_help(true) // When no args are provided, show help.
    .arg(
      Arg::new("config")
        .short('c')
        .long("config")
        .help("Server configuration path.")
        .value_name("FILE")
    )
}

#[tokio::main]
pub async fn handle_cli(matches: &clap::ArgMatches) -> () {
  let config_path = matches
    .get_one::<String>("config")
    .map(PathBuf::from)
    // .unwrap_or_else(|| PathBuf::from("/etc/revezone/server.yaml"))
    .unwrap_or_default();

  let config = load_config(config_path.to_string_lossy().into_owned()).unwrap();

  init_tracing(&config);
  init_custom_metrics(&config);

  let (signal_sender, signal_receiver) = oneshot::channel();
  let mgmt_handle = start_mgmt_server(&config, signal_sender).await;

  signal_receiver.await.expect("MGMT server failed to start");
  info!("MGMT server is ready");

  start_server(config).await;

  mgmt_handle.await.unwrap();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_cli_no_args() {
    let app = build_cli();
    let matches = app.try_get_matches_from(vec![""]).unwrap();
    assert!(matches.subcommand_name().is_none());
  }

  #[test]
  fn test_cli_start_command() {
    let app = build_cli();
    let matches = app.try_get_matches_from(vec!["", "start"]).unwrap();
    assert_eq!(matches.subcommand_name(), Some("start"));
  }

  #[test]
  fn test_cli_start_with_config() {
    let app = build_cli();
    let matches = app.try_get_matches_from(vec!["", "start", "--config", "config.yaml"]).unwrap();
    let start_matches = matches.subcommand_matches("start").unwrap();
    assert_eq!(start_matches.get_one::<String>("config").unwrap(), "config.yaml");
  }

  #[test]
  fn test_cli_invalid_command() {
    let app = build_cli();
    let result = app.try_get_matches_from(vec!["", "invalid"]);
    assert!(result.is_err());
  }
}
