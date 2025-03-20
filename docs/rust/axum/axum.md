# Axum


* Axum is a Rust crate for building a web server.


* Axum consist of:
    * *hyper* for handle http protocol
    Or, *Tonic* for gRPC
    * *Tokio* as runtime for async tasks
    * *matchit* for matching a request to a Route
    * *tower* for middleware

* Actually, tower plays a very important role in axum.


* Here is the brieft overview of how a request flow inside Axum:


![infra](images/infra.drawio.svg "Infrastructure of Axum")


```rust
// example from: https://crates.io/crates/axum
#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// handler: root
async fn root() -> &'static str {
    "Hello, World!"
}

// handler: create_user
async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    
    (StatusCode::CREATED, Json(user))

}
```