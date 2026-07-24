use crate::AppState;
use crate::models::{Claims, CreateItemRequest, Item};
use crate::routes::auth::AppError;
use axum::{Json, extract::State};

pub async fn create_item(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<CreateItemRequest>,
) -> Result<Json<Item>, AppError> {
    let item = sqlx::query_as::<_, Item>(
        "INSERT INTO items (user_id, title, description) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(claims.sub)
    .bind(payload.title)
    .bind(payload.description)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(item))
}

pub async fn get_items(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<Item>>, AppError> {
    let items = sqlx::query_as::<_, Item>("SELECT * FROM items WHERE user_id = $1")
        .bind(claims.sub)
        .fetch_all(&state.db)
        .await?;

    Ok(Json(items))
}
