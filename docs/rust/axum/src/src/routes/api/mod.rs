use axum::Router;
use tower_cookies::CookieManagerLayer;

mod login;
mod register;

pub fn routes() -> Router {
    Router::new()
    .nest("/api", register::route())
    .nest("/api", login::route())
}
