use axum::http::{Method, header};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;

mod middleware;
mod models;
mod routes;
mod utils;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
}

#[tokio::main]
async fn main() {
    // Загрузка переменных окружения
    dotenvy::dotenv().ok();

    // Настройка и инициализация subscriber tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "axum_account_service=info,axum=info".to_string()),
        ))
        .init();

    let db_url = std::env::var("DATABASE_URL")
        .expect("Переменная окружения DATABASE_URL должна быть задана в .env файле");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Не удалось подключиться к базе данных");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any) // Разрешаем всё для этапа разработки основы проекта
        // .allow_origin("https://mydomain.com".parse::<axum::http::HeaderValue>().unwrap())
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);

    let state = AppState { db: pool };

    // Инициализация маршрутов
    let app = routes::create_router(state.clone())
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Сервер запущен на http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
