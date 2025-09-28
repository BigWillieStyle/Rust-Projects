use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

// Define a struct for API responses
#[derive(Serialize)]
struct ApiResponse {
    message: String,
}
// Define a struct for POST request payloads
#[derive(Deserialize)]
struct CreateUser {
    name: String,
    age: u8,
}
#[tokio::main]
async fn main() {
    // Build the Axum application
    let app = Router::new()
        .route("/", get(root))
        .route("/users/:id", get(get_user))
        .route("/users", post(create_user));

    // Define the server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    
    // Start the Axum server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Root handler
async fn root() -> Json<ApiResponse> {
    Json(ApiResponse {
        message: "Welcome to the Rust Web Service!".to_string(),
    })
}

// GET /users/:id handler
async fn get_user(Path(id): Path<u32>) -> Json<ApiResponse> {
    Json(ApiResponse {
        message: format!("User with ID: {}", id),
    })
}
// POST /users handler
async fn create_user(Json(payload): Json<CreateUser>) -> Json<ApiResponse> {
    Json(ApiResponse {
        message: format!("Created user: {} (Age: {})", payload.name, payload.age),
    })
}