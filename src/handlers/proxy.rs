use axum::{
    extract::State,
    http::{uri::Uri, Request, Response},
};
use hyper::{client::HttpConnector, Body};

type Client = hyper::client::Client<HttpConnector, Body>;

pub async fn proxy(State(client): State<Client>, mut request: Request<Body>) -> Response<Body> {
    let path = request.uri().path();
    let path_query = request
        .uri()
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or(path);

    let uri = format!("http://localhost:8081{}", path_query);

    *request.uri_mut() = Uri::try_from(uri).unwrap();

    client.request(request).await.unwrap()
}

/*
use axum::{
    body::{self, Body},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};
use hyper::upgrade::Upgraded;
use tokio::net::TcpStream;
use tracing::{error, warn};

pub async fn proxy(req: Request<Body>) -> Result<Response, hyper::Error> {
    match req.uri().authority().map(|auth| auth.to_string()) {
        Some(host_addr) => {
            tokio::task::spawn(async move {
                match hyper::upgrade::on(req).await {
                    Ok(upgraded) => {
                        if let Err(e) = tunnel(upgraded, host_addr).await {
                            error!("tunnel error: {}", e);
                        }
                    }
                    Err(e) => error!("upgrade error: {}", e),
                }
            });

            Ok(Response::new(body::boxed(body::Empty::new())))
        }
        None => {
            warn!("CONNECT host is not socket addr: {:?}", req.uri());

            Ok((
                StatusCode::BAD_REQUEST,
                "CONNECT must be to a socket address",
            )
                .into_response())
        }
    }
}

async fn tunnel(mut upgraded: Upgraded, addr: String) -> std::io::Result<()> {
    let mut server = TcpStream::connect(addr).await?;

    let (from_client, from_server) =
        tokio::io::copy_bidirectional(&mut upgraded, &mut server).await?;

    tracing::debug!(
        "client wrote {} bytes and received {} bytes",
        from_client,
        from_server
    );

    Ok(())
}
*/
