use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::{
    compression::CompressionLayer,
    trace::{DefaultOnResponse, TraceLayer},
};
use tracing::info;

mod handlers;

/* *
Proxy is not our job, it should be the work of NGINX
    this system requires an `ingress.yml` and matching NGINX config files.
Workflow:
    1.
        a. user sends a request to update ingress.yml
            POST /ingress
            - auth: <auth-key>
            - body: ingress.yml
        b. server reads ingress.yml and updates NGINX config
            b.1. create NGINX config files from fields in ingress.yml
            b.2. reload NGINX
    2. user sends a request to update module
        POST /ingress/services
        - auth: <auth-key>
        - body: <file to be run, extracted for run>
*/
// TODO: read ingress.yml config file to get routings for proxy
// TODO: support update ingress.yml on the fly
//          `PUT        /ingress`
// TODO: support updating proxy modules
//          `PUT|PATCH  /ingress/modules`

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    let app: Router = Router::new()
        .route("/ingress", get(handlers::hello_world))
        .route("/ingress", post(handlers::update_ingressyml))
        .layer(CompressionLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .on_body_chunk(())
                .on_eos(())
                .on_request(())
                .on_response(DefaultOnResponse::new().level(tracing::Level::DEBUG)),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
