use axum::response::{Json, IntoResponse, Response};

#[derive(Serialize)]
struct Index {
    message: String,
}

pub async fn index() -> Json<Index> {
    let index = Index {
        message: "Hello World!".to_string(),
    };

    Json(index)
}
