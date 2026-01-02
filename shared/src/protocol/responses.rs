use serde::{Deserialize, Serialize};

use crate::types::{Game, GameInfo, Move, User, UserProfile};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResponse {
    pub game: Game,
    pub white_player: UserProfile,
    pub black_player: UserProfile,
    pub moves: Vec<Move>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameListResponse {
    pub games: Vec<GameInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveResponse {
    pub r#move: Move,
    pub game: Game,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveListResponse {
    pub moves: Vec<Move>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListResponse {
    pub users: Vec<UserProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PgnResponse {
    pub pgn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: Option<String>,
}
