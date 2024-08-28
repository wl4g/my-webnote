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

use logging::{ LogRouteHandle, LogStderrHandle };
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::SubscriberExt;

use crate::config::config_serve::WebServeConfig;
use crate::mgmt::apm::otel::create_otel_tracer;

pub mod logging;
pub mod metrics;
pub mod otel;

pub async fn init_components(
    config: &Arc<WebServeConfig>
) -> anyhow::Result<(LogRouteHandle, LogStderrHandle)> {
    metrics::init_metrics(config).await;

    let (route_layer, route_layer_handle) = tracing_subscriber::reload::Layer::new(
        logging::default_log_route_layer()
    );
    let (stderr_layer, stderr_layer_handle) = tracing_subscriber::reload::Layer::new(
        logging::default_log_stderr_layer(config)
    );
    let level_layer = logging::default_log_levels_layer();

    let subscriber = tracing_subscriber
        ::registry()
        .with(route_layer)
        .with(stderr_layer)
        .with(level_layer);

    // Create OpenTelemetry layer if tracer is available.
    let otel_layer = create_otel_tracer(config).await.map(OpenTelemetryLayer::new);
    // Add OpenTelemetry layer if available.
    let subscriber = subscriber.with(otel_layer);

    // set the subscriber as the default for the application
    tracing::subscriber::set_global_default(subscriber).unwrap();

    Ok((route_layer_handle, stderr_layer_handle))
}
