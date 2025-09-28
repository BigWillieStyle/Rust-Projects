use axum::{
    Json, Router,
    extract::Path,
    routing::{get, post},
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

#[derive(Deserialize)]
struct CreateCustomer {
    email_address: String,
    street_address1: String,
    street_address2: String,
    city: String,
    state: String,
    zip_code: String,
}

#[tokio::main]
async fn main() {
    // Build the Axum application
    let app = Router::new()
        .route("/", get(root))
        .route("/users/:id", get(get_user))
        .route("/users", post(create_user))
        .route("/customers/:email_address", get(get_customer))
        .route("/customers", post(create_customer));

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

// GET /customers/:id handler
async fn get_customer(Path(email_address): Path<String>) -> Json<ApiResponse> {
    Json(ApiResponse {
        message: format!("Customer with Email Address: {}", email_address),
    })
}

// POST /customers handler
async fn create_customer(Json(payload): Json<CreateCustomer>) -> Json<ApiResponse> {
    Json(ApiResponse {
        message: format!(
            "Created customer: {} (Street Address1: {}, Street Address2: {}, City: {}, State: {}, Zip Code: {})",
            payload.email_address,
            payload.street_address1,
            payload.street_address2,
            payload.city,
            payload.state,
            payload.zip_code
        ),
    })
}
