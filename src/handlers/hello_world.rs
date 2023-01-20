use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HelloWorldResponse {
    message: String,
}

pub async fn hello_world() -> Json<HelloWorldResponse> {
    let hello_world = HelloWorldResponse {
        message: "Hello World!".to_string(),
    };

    Json(hello_world)
}
