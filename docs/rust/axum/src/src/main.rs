/* #region import modules */
use axum::{
    extract::{Path, Query},
    middleware,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use routes::routes;
use serde::Deserialize;

mod error;

mod routes;

/* #endregion */


/* #region Main */
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/hello_param", get(hello_param))
        .route("/hello_path/{name}", get(hello_path))
        .merge(routes::routes())
        .layer(middleware::map_response(map_response_fn))
        ;

    let listener: tokio::net::TcpListener =
        tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
/* #endregion */


/* #region Handler: hello world */
async fn hello_world() -> &'static str {
    "Hello, World!"
}
/* #endregion */


/* #region Route: Hello */
pub fn create_route_hello() -> Router {
    Router::new()
        .route("/hello_param", get(hello_param))
        .route("/hello_path/{name}", get(hello_path))
}
/* #endregion */


/* #region Handler + PARAM extractor */
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn hello_param(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - hello_param - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    format!("Hello {name}")
}

/* #endregion */


/* #region Handler +  PATH extractor */
async fn hello_path(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - hello_path - {name:?}", "HANDLER");

    format!("Hello {name}")
}
/* #endregion */


/* #region MiddleWare: for logging */
async fn map_response_fn(res: Response) -> Response {
    println!("->> {:<12} - map_response_fn", "RES_MAPPER");

    println!();

    res
}
/* #endregion */
