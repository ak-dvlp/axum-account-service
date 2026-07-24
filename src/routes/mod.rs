use axum::{
    Router,
    routing::{get, post},
};

pub mod auth;
pub mod items;
pub mod user;

pub fn create_router(state: crate::AppState) -> Router {
    Router::new()
        // Публичные маршруты
        .route("/health", get(|| async { "OK" }))
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        // Маршруты с авторизацией
        .route("/api/user/profile", get(user::get_profile))
        .route("/api/items", post(items::create_item).get(items::get_items))
        // Передача пула соединений во все обработчики
        .with_state(state)
}
