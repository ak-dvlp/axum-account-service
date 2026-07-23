use crate::AppState;
use crate::models::{RegisterRequest, UserResponse};
use axum::{Json, extract::State, http::StatusCode};
use bcrypt::{DEFAULT_COST, hash};
use uuid::Uuid; // Импорт моделей

// Пользовательский тип ошибки для удобного возврата HTTP-статусов
pub enum AuthError {
    EmailAlreadyExists,
    HashingError,
    DatabaseError,
}

// Превращение ошибки в понятный HTTP-ответ для Axum
impl axum::response::IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AuthError::EmailAlreadyExists => (StatusCode::CONFLICT, "Email уже зарегистрирован"),
            AuthError::HashingError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Ошибка шифрования пароля",
            ),
            AuthError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "Ошибка базы данных"),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

// Обработка регистрации нового пользователя
pub async fn register_handler(
    // State(pool): State<sqlx::PgPool>, // Извлекаем пул БД из общего состояния приложения
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>, // Извлекаем тело запроса
) -> Result<(StatusCode, Json<UserResponse>), AuthError> {
    // 1. Хэширование пароля с солью по умолчанию
    let hashed_password =
        hash(payload.password, DEFAULT_COST).map_err(|_| AuthError::HashingError)?;

    // 2. Генерация нового UUID 4-ой версии для пользователя
    let user_id = Uuid::new_v4();

    // 3. Сохранение пользователя в PostgreSQL
    // Используется макрос query! для проверки SQL во время компиляции (требует DATABASE_URL)
    // Либо используем обычный sqlx::query, если не настроен sqlx-prepared-data
    let result = sqlx::query!(
        r#"
        INSERT INTO users (id, email, password_hash, created_at)
        VALUES ($1, $2, $3, NOW())
        "#,
        user_id,
        payload.email,
        hashed_password
    )
    .execute(&state.db)
    .await;

    // 4. Обработка результата вставки
    match result {
        Ok(_) => {
            let response = UserResponse {
                id: user_id,
                email: payload.email,
            };

            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(err) => {
            // Проверка ошибки уникальности (код 23505 в Postgres — уникальный ключ нарушен)
            if let Some(db_err) = err.as_database_error() {
                if db_err.code() == Some(std::borrow::Cow::Borrowed("23505")) {
                    return Err(AuthError::EmailAlreadyExists);
                }
            }

            Err(AuthError::DatabaseError)
        }
    }
}
