use axum::{extract::Json, routing::get, routing::post, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;

// Define the structure for FormData, matching the UI
#[derive(Serialize, Deserialize, Clone)]
struct FormData {
    date: String, // Expected ISO date format
    reason: String,
}

#[tokio::main]
async fn main() {
    // Create Axum router
    let app = Router::new()
        .route("/", get(root))
        .route("/perform", post(perform));

    // Define the server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    // Run the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Json<serde_json::Value> {
    Json(json!({"status": "OK"}))
}

// Handler for processing requests
async fn perform(Json(payload): Json<FormData>) -> Json<String> {
    // Simulate processing the request
    println!(
        "Processing request: Date = {}, Reason = {}",
        payload.date, payload.reason
    );

    // Respond with a success message
    Json(format!(
        "Request received for date: {}, reason: {}",
        payload.date, payload.reason
    ))
}
