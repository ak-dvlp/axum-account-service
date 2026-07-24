use crate::AppState;
use crate::models::{Claims, User, UserResponse};
use crate::routes::auth::AppError;
use axum::{Json, extract::State};

pub async fn get_profile(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<UserResponse>, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(claims.sub)
        .fetch_one(&state.db)
        .await?;

    Ok(Json(UserResponse {
        id: user.id,
        email: user.email,
    }))
}

// !!! Добавить метод по смене адреса эл. почты
