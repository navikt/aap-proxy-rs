use std::net::SocketAddr;

use axum::Router;
use axum::body::Body;
use axum::http::{Method, Request};
use axum::routing::get;
use tower::make::Shared;
use tower::ServiceExt;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use crate::proxy;

use crate::routes::{actuator_live, actuator_ready, root};

pub async fn proxy() {
    let proxy_svc = Router::new()
        .route("/", get(root))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let service = tower::service_fn(move |req: Request<Body>| {
        let proxy_svc = proxy_svc.clone();
        async move {
            match req.method() {
                &Method::CONNECT => proxy::fss(req).await,
                _ => proxy_svc.oneshot(req).await.map_err(|err| match err {}),
            }
        }
    });

    let proxy_addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("proxy on {}", proxy_addr);

    axum::Server::bind(&proxy_addr)
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(Shared::new(service)).await
        .unwrap();
}

pub async fn actuators() {
    let actuator_svc = Router::new()
        .route("/", get(root))
        .route("/actuator/live", get(actuator_live))
        .route("/actuator/ready", get(actuator_ready));

    let app_addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    tracing::info!("actuators on {}", app_addr);

    axum::Server::bind(&app_addr)
        .serve(actuator_svc.into_make_service())
        .await
        .unwrap();
}
