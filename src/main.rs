use axum::{routing::get, Router};
use hyper::{client::HttpConnector, Body};
use std::net::SocketAddr;
use tower_http::{
    compression::CompressionLayer,
    trace::{DefaultOnResponse, TraceLayer},
};
use tracing::info;

type Client = hyper::client::Client<HttpConnector, Body>;

mod handlers;
use handlers::{hello_world, proxy};

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

    let client = Client::new();
    let app: Router = Router::new()
        .route("/ingress", get(hello_world))
        .fallback(proxy)
        .with_state(client)
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

/* 
use axum::{routing::get, Router};
use hyper::{Body, Method, Request};
use std::net::SocketAddr;
use tower::{make::Shared, ServiceExt};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

use tracing::info;
use tracing_subscriber::util::SubscriberInitExt;

mod handlers;
use handlers::{hello_world, proxy};

#[tokio::main]
pub async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_http_proxy=trace,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router_svc: Router = Router::new().route("/ingress", get(hello_world));

    let service = tower::service_fn(move |req: Request<Body>| {
        let router_svc = router_svc.clone();
        async move {
            if req.method() == Method::CONNECT {
                proxy(req).await
            } else {
                router_svc.oneshot(req).await.map_err(|err| match err {})
            }
        }
    });


    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(Shared::new(service))
        .await
        .unwrap();
}
*/
