use crate::models::Claims;
use crate::routes::auth::AppError;
use axum::{extract::FromRequestParts, http::request::Parts};
use jsonwebtoken::{DecodingKey, Validation, decode};

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or(AppError::InvalidToken)?;

        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::InvalidToken);
        }

        let token = &auth_header[7..];
        let jwt_secret =
            std::env::var("JWT_SECRET").unwrap_or_else(|_| "super_secret_key".to_string());

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
