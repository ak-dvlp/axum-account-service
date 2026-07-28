use crate::AppState;
use crate::models::{AuthRequest, AuthResponse, User, UserResponse};
use crate::utils::{generate_jwt, verify_password};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use uuid::Uuid;

pub enum AppError {
    InvalidCredentials,
    TokenCreation,
    InvalidToken,
    DatabaseError(sqlx::Error),
    PasswordHashing,
    UserAlreadyExists,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Неверный email или пароль"),
            AppError::TokenCreation => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Ошибка создания токена")
            }
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "Невалидный токен"),
            AppError::DatabaseError(err) => {
                tracing::error!(error = ?err, "Произошел сбой при работе с базой данных");
                (StatusCode::INTERNAL_SERVER_ERROR, "Ошибка базы данных")
            }
            AppError::PasswordHashing => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Ошибка обработки пароля")
            }
            AppError::UserAlreadyExists => (
                StatusCode::BAD_REQUEST,
                "Пользователь с таким email уже существует",
            ),
        };

        (status, Json(json!({ "error": error_message }))).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err)
    }
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    // Хэширование пароля
    let hashed =
        crate::utils::hash_password(&payload.password).map_err(|_| AppError::PasswordHashing)?;

    let user_id = Uuid::new_v4();

    // Выполнение запроса к БД
    let result = sqlx::query("INSERT INTO users (id, email, password_hash) VALUES ($1, $2, $3)")
        .bind(user_id)
        .bind(&payload.email)
        .bind(&hashed)
        .execute(&state.db)
        .await;

    // Обработка результата
    match result {
        Ok(_) => {
            let response = UserResponse {
                id: user_id,
                email: payload.email,
            };

            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(err) => {
            // Проверка уникальности адреса эл. почты (код ошибки 23505 в Postgres)
            if let Some(code) = err.as_database_error().and_then(|de| de.code()) {
                if code == "23505" {
                    return Err(AppError::UserAlreadyExists);
                }
            }

            // Журналирование любых других ошибок БД перед возвратом
            tracing::error!(error = ?err, "Ошибка при регистрации пользователя в БД");
            Err(AppError::DatabaseError(err))
        }
    }
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Поиск пользователя в БД по адресу эл. почты
    let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&state.db)
        .await;

    // Обработка результата поиска через match
    let user = match result {
        Ok(Some(user)) => user,
        Ok(None) => {
            tracing::warn!(email = %payload.email, "Попытка входа с несуществующим email");
            return Err(AppError::InvalidCredentials);
        }
        Err(err) => {
            tracing::error!(error = ?err, email = %payload.email, "Ошибка БД при поиске пользователя для входа");
            return Err(AppError::DatabaseError(err));
        }
    };

    // Проверка корректности пароля через bcrypt
    let is_valid = verify_password(&payload.password, &user.password_hash).map_err(|err| {
        tracing::error!(error = ?err, "Ошибка при верификации хэша пароля");
        AppError::PasswordHashing
    })?;

    if !is_valid {
        tracing::warn!(email = %payload.email, "Неудачная попытка входа: неверный пароль");
        return Err(AppError::InvalidCredentials);
    }

    // Генерация JWT-токена на основе UUID пользователя
    let token = generate_jwt(user.id).map_err(|err| {
        tracing::error!(error = ?err, user_id = %user.id, "Не удалось сгенерировать JWT для пользователя");
        AppError::TokenCreation
    })?;

    tracing::info!(user_id = %user.id, "Пользователь успешно авторизован");

    // Возврат токена и безопасных данных профиля
    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            email: user.email,
        },
    }))
}
