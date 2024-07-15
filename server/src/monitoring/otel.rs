use std::sync::Arc;
use std::time::Duration;

use opentelemetry_sdk::trace::Config;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::runtime::Tokio;
use opentelemetry::{ global, KeyValue };
use opentelemetry_otlp::{ new_exporter, ExportConfig, Protocol };
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::Tracer;

use crate::config::config_api::ApiConfig;

pub async fn create_otel_tracer(config: &Arc<ApiConfig>) -> Option<Tracer> {
  let mut tracer = None;
  if config.monitoring.enabled {
    tracer = Some(
      opentelemetry_otlp
        ::new_pipeline()
        .tracing()
        .with_exporter(
          new_exporter()
            .tonic()
            .with_export_config(ExportConfig {
              endpoint: config.monitoring.otel.endpoint.to_string(),
              protocol: match config.monitoring.otel.protocol.to_lowercase().as_str() {
                "http/protobuf" => Protocol::HttpBinary,
                "grpc" => Protocol::Grpc,
                "http/json" => Protocol::HttpJson,
                _ => Protocol::HttpBinary,
              },
              timeout: Duration::from_millis(config.monitoring.otel.timeout.unwrap()),
            })
        )
        .with_trace_config(
          // Notice: More OTEL custom configuration use to environment: OTEL_SPAN_xxx, see to: opentelemetry_sdk::trace::config::default()
          Config::default().with_resource(
            Resource::new(vec![KeyValue::new("service.name", config.service_name.to_string())])
          )
        )
        .install_batch(Tokio)
        .expect("Failed to install OpenTelemetry tracer.")
    );

    // Make sure all trace data is refreshed.
    global::shutdown_tracer_provider();
  }

  tracer
}
