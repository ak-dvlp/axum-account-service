use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// --- МОДЕЛИ ПОЛЬЗОВАТЕЛЯ (Users) ---

// Модель, которая в точности сопоставляется с таблицей в PostgreSQL
#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)] // Пароль никогда не должен уходить в JSON-ответах!
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// Структура для входящего JSON-запроса при регистрации/авторизации
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

// Структура ответа после успешной авторизации
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

// Безопасное представление пользователя для Front-end
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
}

// --- МОДЕЛИ ОБЪЕКТОВ (Items) ---

#[derive(Debug, Serialize, FromRow)]
pub struct Item {
    pub id: i32,
    pub user_id: Uuid, // Внешний ключ, связывающий объект с пользователем
    pub title: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// Структура для создания нового объекта
#[derive(Debug, Deserialize)]
pub struct CreateItemRequest {
    pub title: String,
    pub description: Option<String>,
}
