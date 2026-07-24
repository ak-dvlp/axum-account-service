use bcrypt::{DEFAULT_COST, hash, verify};

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

pub fn generate_jwt(user_id: uuid::Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "super_secret_key".to_string());

    let expiration = (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize;

    let claims = crate::models::Claims {
        sub: user_id,
        exp: expiration,
    };

    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
}
