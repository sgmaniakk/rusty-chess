use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGameRequest {
    pub opponent_username: String,
    pub player_color: Option<String>, // "white", "black", or None for random
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitMoveRequest {
    pub move_uci: String, // e.g., "e2e4", "e1g1" (castling), "e7e8q" (promotion)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForfeitGameRequest {
    pub game_id: Uuid,
}
