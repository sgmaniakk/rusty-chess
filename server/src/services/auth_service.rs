use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::users;
use crate::error::{AppError, Result};
use crate::models::NewUser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // User ID
    pub username: String, // Username
    pub exp: i64,         // Expiry timestamp
    pub iat: i64,         // Issued at timestamp
}

#[derive(Clone)]
pub struct AuthService {
    jwt_secret: String,
    jwt_expiry_days: i64,
}

impl AuthService {
    pub fn new(jwt_secret: String, jwt_expiry_days: i64) -> Self {
        Self {
            jwt_secret,
            jwt_expiry_days,
        }
    }

    /// Hash a password using bcrypt
    pub fn hash_password(&self, password: &str) -> Result<String> {
        let hashed = hash(password, DEFAULT_COST)?;
        Ok(hashed)
    }

    /// Verify a password against a hash
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let valid = verify(password, hash)?;
        Ok(valid)
    }

    /// Generate a JWT token for a user
    pub fn generate_token(&self, user_id: Uuid, username: &str) -> Result<String> {
        let now = Utc::now();
        let expiry = now + Duration::days(self.jwt_expiry_days);

        let claims = Claims {
            sub: user_id.to_string(),
            username: username.to_string(),
            exp: expiry.timestamp(),
            iat: now.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;

        Ok(token)
    }

    /// Validate a JWT token and extract claims
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    /// Register a new user
    pub async fn register(
        &self,
        pool: &PgPool,
        username: String,
        email: String,
        password: String,
    ) -> Result<(crate::models::User, String)> {
        // Validate input
        if username.is_empty() || username.len() > 50 {
            return Err(AppError::Validation(
                "Username must be 1-50 characters".to_string(),
            ));
        }

        if email.is_empty() || !email.contains('@') {
            return Err(AppError::Validation("Invalid email address".to_string()));
        }

        if password.len() < 6 {
            return Err(AppError::Validation(
                "Password must be at least 6 characters".to_string(),
            ));
        }

        // Check if username already exists
        if let Some(_) = users::find_by_username(pool, &username).await? {
            return Err(AppError::Validation("Username already taken".to_string()));
        }

        // Check if email already exists
        if let Some(_) = users::find_by_email(pool, &email).await? {
            return Err(AppError::Validation("Email already registered".to_string()));
        }

        // Hash password
        let password_hash = self.hash_password(&password)?;

        // Create user
        let new_user = NewUser {
            username: username.clone(),
            password_hash,
            email,
        };

        let user = users::create_user(pool, &new_user).await?;

        // Generate token
        let token = self.generate_token(user.id, &user.username)?;

        Ok((user, token))
    }

    /// Login a user
    pub async fn login(
        &self,
        pool: &PgPool,
        username: String,
        password: String,
    ) -> Result<(crate::models::User, String)> {
        // Find user
        let user = users::find_by_username(pool, &username)
            .await?
            .ok_or_else(|| AppError::Auth("Invalid username or password".to_string()))?;

        // Verify password
        if !self.verify_password(&password, &user.password_hash)? {
            return Err(AppError::Auth("Invalid username or password".to_string()));
        }

        // Generate token
        let token = self.generate_token(user.id, &user.username)?;

        // Update last seen
        users::update_last_seen(pool, user.id).await?;

        Ok((user, token))
    }
}
