use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct MoveRecord {
    pub id: Uuid,
    pub game_id: Uuid,
    pub move_number: i32,
    pub player_color: String,
    pub move_uci: String,
    pub move_san: String,
    pub position_before: String,
    pub position_after: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct NewMove {
    pub game_id: Uuid,
    pub move_number: i32,
    pub player_color: String,
    pub move_uci: String,
    pub move_san: String,
    pub position_before: String,
    pub position_after: String,
}
