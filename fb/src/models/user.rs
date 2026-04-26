use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub password_hash: String,
}
