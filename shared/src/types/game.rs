use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::White => write!(f, "white"),
            Color::Black => write!(f, "black"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameStatus {
    Active,
    WhiteWon,
    BlackWon,
    Draw,
    Abandoned,
}

impl std::fmt::Display for GameStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameStatus::Active => write!(f, "active"),
            GameStatus::WhiteWon => write!(f, "white_won"),
            GameStatus::BlackWon => write!(f, "black_won"),
            GameStatus::Draw => write!(f, "draw"),
            GameStatus::Abandoned => write!(f, "abandoned"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: Uuid,
    pub white_player_id: Uuid,
    pub black_player_id: Uuid,
    pub current_position: String, // FEN notation
    pub status: GameStatus,
    pub current_turn: Color,
    pub move_deadline: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInfo {
    pub id: Uuid,
    pub white_player_username: String,
    pub black_player_username: String,
    pub status: GameStatus,
    pub current_turn: Color,
    pub move_deadline: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
