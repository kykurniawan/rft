pub mod common {
    use axum::Json;
    use serde_json::{Value, json};

    pub async fn index() -> &'static str {
        "Hello, world!"
    }

    pub async fn fruits() -> Json<Value> {
        Json(json!({
            "data": [
                "apple",
                "banana",
                "orange",
            ]
        }))
    }
}
