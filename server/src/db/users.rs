use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

use crate::models::{User, NewUser};

/// Create a new user
pub async fn create_user(pool: &PgPool, new_user: &NewUser) -> Result<User> {
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, password_hash, email)
        VALUES ($1, $2, $3)
        RETURNING id, username, password_hash, email, created_at, last_seen
        "#,
    )
    .bind(&new_user.username)
    .bind(&new_user.password_hash)
    .bind(&new_user.email)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Find a user by username
pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<User>> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, username, password_hash, email, created_at, last_seen
        FROM users
        WHERE username = $1
        "#,
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Find a user by ID
pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, username, password_hash, email, created_at, last_seen
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Find a user by email
pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, username, password_hash, email, created_at, last_seen
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// List all users (for challenges)
pub async fn list_users(pool: &PgPool) -> Result<Vec<User>> {
    let users = sqlx::query_as::<_, User>(
        r#"
        SELECT id, username, password_hash, email, created_at, last_seen
        FROM users
        ORDER BY username ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

/// Update last seen timestamp
pub async fn update_last_seen(pool: &PgPool, user_id: Uuid) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE users
        SET last_seen = NOW()
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}
