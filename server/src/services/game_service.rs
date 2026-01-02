use chrono::{Duration, Utc};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::chess::{check_game_result, validate_move, GameResult, GameState};
use crate::db::{games, moves, users};
use crate::error::{AppError, Result};
use crate::models::{NewGame, NewMove};
use shared::types::{Color, GameStatus};

#[derive(Clone)]
pub struct GameService {
    move_deadline_hours: i64,
}

impl GameService {
    pub fn new(move_deadline_hours: i64) -> Self {
        Self {
            move_deadline_hours,
        }
    }

    /// Create a new game between two players
    pub async fn create_game(
        &self,
        pool: &PgPool,
        white_player_id: Uuid,
        black_player_id: Uuid,
    ) -> Result<crate::models::Game> {
        // Verify both players exist
        users::find_by_id(pool, white_player_id)
            .await?
            .ok_or_else(|| AppError::NotFound("White player not found".to_string()))?;

        users::find_by_id(pool, black_player_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Black player not found".to_string()))?;

        if white_player_id == black_player_id {
            return Err(AppError::BadRequest(
                "Cannot create game with yourself".to_string(),
            ));
        }

        // Create initial game state
        let game_state = GameState::new();
        let fen = game_state.fen().to_string();

        let new_game = NewGame {
            white_player_id,
            black_player_id,
            current_position: fen,
            game_state: json!({ "fen": game_state.fen() }),
            status: "active".to_string(),
            current_turn: "white".to_string(),
        };

        let game = games::create_game(pool, &new_game).await?;

        Ok(game)
    }

    /// Submit a move for a game
    pub async fn submit_move(
        &self,
        pool: &PgPool,
        game_id: Uuid,
        user_id: Uuid,
        move_uci: String,
    ) -> Result<(crate::models::MoveRecord, crate::models::Game)> {
        // Get the game
        let game = games::find_by_id(pool, game_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Game not found".to_string()))?;

        // Check game is active
        if game.status != "active" {
            return Err(AppError::BadRequest("Game is not active".to_string()));
        }

        // Determine which player is making the move
        let player_color = if user_id == game.white_player_id {
            Color::White
        } else if user_id == game.black_player_id {
            Color::Black
        } else {
            return Err(AppError::BadRequest(
                "You are not a player in this game".to_string(),
            ));
        };

        // Check if it's the player's turn
        let current_turn = if game.current_turn == "white" {
            Color::White
        } else {
            Color::Black
        };

        if player_color != current_turn {
            return Err(AppError::BadRequest("It's not your turn".to_string()));
        }

        // Load game state and validate move
        let game_state = GameState::from_fen(&game.current_position)?;
        validate_move(&game_state, &move_uci)?;

        // Make the move
        let (new_state, san) = game_state.make_move(&move_uci)?;

        // Count existing moves to determine move number
        let move_count = moves::count_by_game(pool, game_id).await?;
        let move_number = (move_count / 2) + 1;

        // Create move record
        let new_move = NewMove {
            game_id,
            move_number: move_number as i32,
            player_color: player_color.to_string(),
            move_uci,
            move_san: san,
            position_before: game.current_position.clone(),
            position_after: new_state.fen().to_string(),
        };

        let move_record = moves::create_move(pool, &new_move).await?;

        // Check for game over
        let game_result = check_game_result(&new_state)?;
        let new_status = if let Some(result) = game_result {
            match result {
                GameResult::Checkmate => {
                    // Current player (who just moved) wins
                    match player_color {
                        Color::White => "white_won",
                        Color::Black => "black_won",
                    }
                }
                GameResult::Stalemate => "draw",
            }
        } else {
            "active"
        };

        // Update game state
        let next_turn = player_color.opposite().to_string();
        let deadline = Utc::now() + Duration::hours(self.move_deadline_hours);

        games::update_after_move(
            pool,
            game_id,
            new_state.fen(),
            &json!({ "fen": new_state.fen() }),
            &next_turn,
            deadline,
        )
        .await?;

        // If game is over, update status
        if new_status != "active" {
            games::update_status(pool, game_id, new_status).await?;
        }

        // Fetch updated game
        let updated_game = games::find_by_id(pool, game_id)
            .await?
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Game disappeared")))?;

        Ok((move_record, updated_game))
    }

    /// Generate PGN for a game
    pub async fn generate_pgn(
        &self,
        pool: &PgPool,
        game_id: Uuid,
    ) -> Result<String> {
        // Get game
        let game = games::find_by_id(pool, game_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Game not found".to_string()))?;

        // Get players
        let white_player = users::find_by_id(pool, game.white_player_id)
            .await?
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("White player not found")))?;

        let black_player = users::find_by_id(pool, game.black_player_id)
            .await?
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Black player not found")))?;

        // Get all moves
        let all_moves = moves::list_by_game(pool, game_id).await?;

        // Build PGN
        let mut pgn = String::new();

        // PGN headers
        pgn.push_str(&format!("[Event \"Rusty Chess Correspondence Game\"]\n"));
        pgn.push_str(&format!("[Site \"Rusty Chess\"]\n"));
        pgn.push_str(&format!(
            "[Date \"{}\"]\n",
            game.created_at.format("%Y.%m.%d")
        ));
        pgn.push_str(&format!("[White \"{}\"]\n", white_player.username));
        pgn.push_str(&format!("[Black \"{}\"]\n", black_player.username));
        pgn.push_str(&format!("[Result \"{}\"]\n", game_status_to_pgn(&game.status)));
        pgn.push_str("\n");

        // Moves
        let mut move_text = String::new();
        for (i, mv) in all_moves.iter().enumerate() {
            if i % 2 == 0 {
                // White's move
                let move_num = (i / 2) + 1;
                move_text.push_str(&format!("{}. {} ", move_num, mv.move_san));
            } else {
                // Black's move
                move_text.push_str(&format!("{} ", mv.move_san));
            }
        }

        // Add result
        move_text.push_str(&game_status_to_pgn(&game.status));
        pgn.push_str(&move_text);
        pgn.push('\n');

        Ok(pgn)
    }
}

fn game_status_to_pgn(status: &str) -> &str {
    match status {
        "white_won" => "1-0",
        "black_won" => "0-1",
        "draw" => "1/2-1/2",
        _ => "*",
    }
}
