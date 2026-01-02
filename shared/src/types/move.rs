use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Color;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    pub id: Uuid,
    pub game_id: Uuid,
    pub move_number: i32,
    pub player_color: Color,
    pub move_uci: String,  // e.g., "e2e4", "e7e8q"
    pub move_san: String,  // e.g., "e4", "Nf3", "O-O"
    pub position_before: String, // FEN
    pub position_after: String,  // FEN
    pub timestamp: DateTime<Utc>,
}
