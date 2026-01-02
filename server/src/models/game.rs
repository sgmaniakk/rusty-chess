use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, FromRow)]
pub struct Game {
    pub id: Uuid,
    pub white_player_id: Uuid,
    pub black_player_id: Uuid,
    pub current_position: String,
    pub game_state: JsonValue,
    pub status: String,
    pub current_turn: String,
    pub move_deadline: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct NewGame {
    pub white_player_id: Uuid,
    pub black_player_id: Uuid,
    pub current_position: String,
    pub game_state: JsonValue,
    pub status: String,
    pub current_turn: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct GameWithPlayers {
    pub id: Uuid,
    pub white_player_id: Uuid,
    pub black_player_id: Uuid,
    pub white_player_username: String,
    pub black_player_username: String,
    pub current_position: String,
    pub status: String,
    pub current_turn: String,
    pub move_deadline: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
