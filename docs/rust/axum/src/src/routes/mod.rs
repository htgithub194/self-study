use axum::Router;

mod api;

pub fn routes() -> Router {
    Router::new().merge(api::routes())
}