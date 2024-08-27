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

use std::sync::Arc;

use clap::{ Command, Arg };

use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

#[cfg(feature = "tokio-console")]
use console_subscriber::ConsoleLayer;
// use tower_http::trace::TraceLayer;

use lazy_static::lazy_static;
use prometheus::{ Registry, Counter, Histogram, Encoder, TextEncoder };
use axum_prometheus::PrometheusMetricLayer;

use tokio::task::JoinHandle;
use tokio::net::TcpListener;
use tokio::sync::oneshot;

use axum::Router;
use axum::routing::get;

use crate::config::config_serve;
use crate::config::config_serve::WebServeConfig;
use crate::config::swagger;
use crate::context::state::AppState;
use crate::monitoring::otel::create_otel_tracer;
use crate::monitoring::health::init as health_router;
use crate::routes::auths::auth_middleware;
use crate::routes::auths::init as auth_router;
use crate::routes::users::init as user_router;
use crate::routes::documents::init as document_router;
use crate::routes::folders::init as folder_router;
use crate::routes::settings::init as settings_router;

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
async fn init_custom_metrics(config: &Arc<WebServeConfig>) {
    REGISTRY.register(Box::new(MY_HTTP_REQUESTS_TOTAL.clone())).expect(
        "collector can be registered"
    );
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
async fn init_tracing(config: &Arc<WebServeConfig>) {
    // Intialize setup logger levels.
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "debug".into())
        // .add_directive("debug".parse().unwrap()) // default level.
        .add_directive("mywebnote=debug".parse().unwrap())
        .add_directive("hyper=warn".parse().unwrap())
        .add_directive("tokio=trace".parse().unwrap()); // Notice: Must be at trace level to collect

    // Initialize layer with tokio console exporter.
    let subscriber = tracing_subscriber
        ::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(env_filter);

    // Create OpenTelemetry layer if tracer is available.
    let otel_layer = create_otel_tracer(config).await.map(OpenTelemetryLayer::new);

    // Add OpenTelemetry layer if available.
    let subscriber = subscriber.with(otel_layer);

    // Add console layer if feature is enabled.
    // Notice: Use optional dependencies to avoid slow automatic compilation during debugging
    // (because if rely on console-subscriber, need to enable RUSTFLAGS="--cfg tokio_unstable" which
    // will invalidate the compile-time cache).
    #[cfg(feature = "tokio-console")]
    let subscriber = subscriber.with(ConsoleLayer::builder().with_default_env().spawn());

    subscriber.init();
}

#[allow(unused)]
async fn start_mgmt_server(
    config: &Arc<WebServeConfig>,
    signal_sender: oneshot::Sender<()>
) -> JoinHandle<()> {
    let (prometheus_layer, _) = PrometheusMetricLayer::pair();

    let app: Router = Router::new().route("/metrics", get(metrics)).layer(prometheus_layer);

    let bind_addr = config.server.mgmt_bind.clone();
    info!("Starting Management server on {}", bind_addr);

    tokio::spawn(async move {
        // TODO When started call to signal sender.
        let _ = signal_sender.send(());
        axum::serve(
            tokio::net::TcpListener::bind(&bind_addr).await.unwrap(),
            app.into_make_service()
        ).await.unwrap_or_else(|e| panic!("Error starting management server: {}", e));
    })
}

async fn start_server(config: &Arc<WebServeConfig>) {
    let app_state = AppState::new(&config).await;
    tracing::info!("Register Web server middlewares ...");

    // 1. Merge the biz modules routes.
    let expose_routes = Router::new()
        .merge(auth_router())
        .merge(user_router())
        .merge(document_router())
        .merge(folder_router())
        .merge(settings_router());

    // 2. Merge of all routes.
    let mut app_routes = match &config.server.context_path {
        Some(cp) => {
            Router::new()
                .merge(health_router())
                .nest(&cp, expose_routes) // support the context-path.
                .with_state(app_state.clone()) // TODO: remove clone
        }
        None => {
            Router::new().merge(health_router()).merge(expose_routes).with_state(app_state.clone()) // TODO: remove clone
        }
    };

    // 3. Merge the swagger router.
    if config.swagger.enabled {
        app_routes = app_routes.merge(swagger::init_swagger(&config));
    }

    // 4. Finally add the (auth) middlewares.
    // Notice: The settings of middlewares are in order, which will affect the priority of route matching.
    // The later the higher the priority? For example, if auth_middleware is set at the end, it will
    // enter when requesting '/', otherwise it will not enter if it is set at the front, and will
    // directly enter handle_root().
    app_routes = app_routes.layer(
        ServiceBuilder::new()
            .layer(axum::middleware::from_fn_with_state(app_state, auth_middleware))
            // Optional: add logs to tracing.
            .layer(
                TraceLayer::new_for_http().make_span_with(|request: &axum::http::Request<_>| {
                    tracing::info_span!(
                            "http_request",
                            method = %request.method(),
                            uri = %request.uri(),
                        )
                })
            )
    );
    //.route_layer(axum::Extension(app_state));

    let bind_addr = &config.server.bind;
    tracing::info!("Starting web server on {}", bind_addr);

    axum::serve(
        TcpListener::bind(&bind_addr).await.unwrap(),
        app_routes.into_make_service()
    ).await.unwrap_or_else(|e| panic!("Error starting API server: {}", e));

    tracing::info!("Web server is ready");
}

pub fn build_cli() -> Command {
    Command::new("serve")
        .about("My Webnote web server.")
        // .arg_required_else_help(true) // When no args are provided, show help.
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Web Server configuration path.")
                .value_name("FILE")
        )
}

#[allow(unused)]
#[tokio::main]
pub async fn handle_cli(matches: &clap::ArgMatches) -> () {
    //let config_path = matches
    //    .get_one::<String>("config")
    //    .map(std::path::PathBuf::from)
    //    // .unwrap_or_else(|| std::path::PathBuf::from("/etc/serve.yaml"))
    //    .unwrap_or_default()
    //    .to_string_lossy()
    //    .into_owned();

    let config = config_serve::get_config();

    init_tracing(&config).await;
    init_custom_metrics(&config).await;

    let (signal_sender, signal_receiver) = oneshot::channel();
    let mgmt_handle = start_mgmt_server(&config, signal_sender).await;

    signal_receiver.await.expect("Management server failed to start");
    info!("Management server is ready");

    start_server(&config).await;

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
        let matches = app
            .try_get_matches_from(vec!["", "start", "--config", "config.yaml"])
            .unwrap();
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
