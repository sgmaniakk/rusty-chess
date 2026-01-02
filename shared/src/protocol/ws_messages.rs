use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

use crate::types::{Color, GameStatus};

// Client → Server messages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    Subscribe { game_id: Uuid },
    Unsubscribe { game_id: Uuid },
    Ping,
}

// Server → Client messages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    MoveMade {
        game_id: Uuid,
        move_san: String,
        move_uci: String,
        position_fen: String,
        deadline: DateTime<Utc>,
    },
    GameStatusChanged {
        game_id: Uuid,
        status: GameStatus,
        winner: Option<Color>,
        reason: String,
    },
    DeadlineWarning {
        game_id: Uuid,
        #[serde(with = "duration_serde")]
        time_remaining: Duration,
    },
    ChallengeReceived {
        game_id: Uuid,
        opponent: String,
        your_color: Color,
    },
    Pong,
    Error {
        message: String,
    },
}

// Custom serialization for Duration
mod duration_serde {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(duration.as_secs())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(Duration::from_secs(secs))
    }
}
