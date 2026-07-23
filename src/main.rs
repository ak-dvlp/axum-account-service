mod auth;
mod models;

use axum::{
    Router,
    http::{Method, StatusCode, header},
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

// Структура для разделяемого состояния приложения (пока пустая, скоро добавим пул)
// #[derive(Clone)]
// struct AppState {
//     pool: sqlx::PgPool,
// }

// Общее состояние приложения (понадобится для JWT и SQLx в следующих шагах)
#[derive(Clone)]
struct AppState {
    db: sqlx::PgPool,
}

#[tokio::main]
async fn main() {
    // Загрузка переменных окружения
    dotenvy::dotenv().ok();

    // Инициализация логирования (полезно для отладки запросов)
    tracing_subscriber::fmt::init();

    // Получение строки подключения из переменной окружения
    let database_url = std::env::var("DATABASE_URL")
        .expect("Переменная окружения DATABASE_URL должна быть задана в .env файле");

    // Создание пула соединений с базой данных
    // Создание пула подключений к PostgreSQL
    let pool = PgPoolOptions::new()
        .max_connections(5) // Ограничение количества соединений
        .connect(&database_url)
        .await
        .expect("Не удалось подключиться к базе данных");

    let state = AppState { db: pool };

    // Настройка CORS (разрешаем всё для этапа разработки основы проекта)
    // let cors = CorsLayer::permissive();
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);

    // Создание маршрутизатора Axum
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/auth/register", post(auth::register_handler))
        .layer(cors)
        .with_state(state);

    // Запуск сервера
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("🚀 Сервер запущен на http://{addr}");

    axum::serve(listener, app).await.unwrap();

    // Тестовая конечная точка для проверки работоспособности
    async fn health_check() -> StatusCode {
        StatusCode::OK
    }
}
