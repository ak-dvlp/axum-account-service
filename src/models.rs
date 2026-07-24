use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// Добавляем структуру Claims для JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // Теперь ID пользователя — это Uuid
    pub exp: usize,
}

// --- МОДЕЛИ ПОЛЬЗОВАТЕЛЯ (Users) ---

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)] // Пароль никогда не должен уходить в JSON-ответах!
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
}

// --- МОДЕЛИ ОБЪЕКТОВ (Items) ---

#[derive(Debug, Serialize, FromRow)]
pub struct Item {
    pub id: i32,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateItemRequest {
    pub title: String,
    pub description: Option<String>,
}
