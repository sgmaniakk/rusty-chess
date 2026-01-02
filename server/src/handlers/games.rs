use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::db::{games, moves as db_moves, users};
use crate::error::{AppError, Result};
use crate::middleware::AuthUser;
use crate::AppState;
use shared::protocol::{
    CreateGameRequest, GameListResponse, GameResponse, MoveListResponse, MoveResponse,
    PgnResponse, SubmitMoveRequest,
};
use shared::types::{Color, GameInfo, Move, UserProfile};

/// List games for the authenticated user
pub async fn list_games(
    Extension(auth): Extension<AuthUser>,
    State(state): State<AppState>,
) -> Result<Json<GameListResponse>> {
    let user_id = auth.user_id;

    let user_games = games::list_by_user(&state.db, user_id).await?;

    let games_info: Vec<GameInfo> = user_games
        .into_iter()
        .map(|g| GameInfo {
            id: g.id,
            white_player_username: g.white_player_username,
            black_player_username: g.black_player_username,
            status: g.status.parse().unwrap_or(shared::types::GameStatus::Active),
            current_turn: if g.current_turn == "white" {
                Color::White
            } else {
                Color::Black
            },
            move_deadline: g.move_deadline,
            created_at: g.created_at,
        })
        .collect();

    Ok(Json(GameListResponse { games: games_info }))
}

/// Create a new game
pub async fn create_game(
    Extension(auth): Extension<AuthUser>,
    State(state): State<AppState>,
    Json(request): Json<CreateGameRequest>,
) -> Result<(StatusCode, Json<GameResponse>)> {
    let user_id = auth.user_id;

    // Find opponent by username
    let opponent = users::find_by_username(&state.db, &request.opponent_username)
        .await?
        .ok_or_else(|| AppError::NotFound("Opponent not found".to_string()))?;

    // Determine colors
    let (white_id, black_id) = match request.player_color.as_deref() {
        Some("white") => (user_id, opponent.id),
        Some("black") => (opponent.id, user_id),
        _ => {
            // Random assignment
            use rand::Rng;
            if rand::thread_rng().gen_bool(0.5) {
                (user_id, opponent.id)
            } else {
                (opponent.id, user_id)
            }
        }
    };

    // Create game
    let game = state
        .game_service
        .create_game(&state.db, white_id, black_id)
        .await?;

    // Get player info
    let white_player = users::find_by_id(&state.db, white_id).await?.unwrap();
    let black_player = users::find_by_id(&state.db, black_id).await?.unwrap();

    let response = GameResponse {
        game: shared::types::Game {
            id: game.id,
            white_player_id: game.white_player_id,
            black_player_id: game.black_player_id,
            current_position: game.current_position,
            status: game.status.parse().unwrap_or(shared::types::GameStatus::Active),
            current_turn: if game.current_turn == "white" {
                Color::White
            } else {
                Color::Black
            },
            move_deadline: game.move_deadline,
            created_at: game.created_at,
            completed_at: game.completed_at,
        },
        white_player: UserProfile {
            id: white_player.id,
            username: white_player.username,
        },
        black_player: UserProfile {
            id: black_player.id,
            username: black_player.username,
        },
        moves: vec![],
    };

    Ok((StatusCode::CREATED, Json(response)))
}

/// Get game details
pub async fn get_game(
    Extension(auth): Extension<AuthUser>,
    State(state): State<AppState>,
    Path(game_id): Path<Uuid>,
) -> Result<Json<GameResponse>> {
    let user_id = auth.user_id;

    // Get game
    let game = games::find_by_id(&state.db, game_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Game not found".to_string()))?;

    // Verify user is a player
    if game.white_player_id != user_id && game.black_player_id != user_id {
        return Err(AppError::BadRequest(
            "You are not a player in this game".to_string(),
        ));
    }

    // Get players
    let white_player = users::find_by_id(&state.db, game.white_player_id)
        .await?
        .unwrap();
    let black_player = users::find_by_id(&state.db, game.black_player_id)
        .await?
        .unwrap();

    // Get moves
    let game_moves = db_moves::list_by_game(&state.db, game_id).await?;

    let moves: Vec<Move> = game_moves
        .into_iter()
        .map(|m| Move {
            id: m.id,
            game_id: m.game_id,
            move_number: m.move_number,
            player_color: if m.player_color == "white" {
                Color::White
            } else {
                Color::Black
            },
            move_uci: m.move_uci,
            move_san: m.move_san,
            position_before: m.position_before,
            position_after: m.position_after,
            timestamp: m.timestamp,
        })
        .collect();

    let response = GameResponse {
        game: shared::types::Game {
            id: game.id,
            white_player_id: game.white_player_id,
            black_player_id: game.black_player_id,
            current_position: game.current_position,
            status: game.status.parse().unwrap_or(shared::types::GameStatus::Active),
            current_turn: if game.current_turn == "white" {
                Color::White
            } else {
                Color::Black
            },
            move_deadline: game.move_deadline,
            created_at: game.created_at,
            completed_at: game.completed_at,
        },
        white_player: UserProfile {
            id: white_player.id,
            username: white_player.username,
        },
        black_player: UserProfile {
            id: black_player.id,
            username: black_player.username,
        },
        moves,
    };

    Ok(Json(response))
}

/// Submit a move
pub async fn submit_move(
    Extension(auth): Extension<AuthUser>,
    State(state): State<AppState>,
    Path(game_id): Path<Uuid>,
    Json(request): Json<SubmitMoveRequest>,
) -> Result<Json<MoveResponse>> {
    let user_id = auth.user_id;

    let (move_record, game) = state
        .game_service
        .submit_move(&state.db, game_id, user_id, request.move_uci)
        .await?;

    let response = MoveResponse {
        r#move: Move {
            id: move_record.id,
            game_id: move_record.game_id,
            move_number: move_record.move_number,
            player_color: if move_record.player_color == "white" {
                Color::White
            } else {
                Color::Black
            },
            move_uci: move_record.move_uci,
            move_san: move_record.move_san,
            position_before: move_record.position_before,
            position_after: move_record.position_after,
            timestamp: move_record.timestamp,
        },
        game: shared::types::Game {
            id: game.id,
            white_player_id: game.white_player_id,
            black_player_id: game.black_player_id,
            current_position: game.current_position,
            status: game.status.parse().unwrap_or(shared::types::GameStatus::Active),
            current_turn: if game.current_turn == "white" {
                Color::White
            } else {
                Color::Black
            },
            move_deadline: game.move_deadline,
            created_at: game.created_at,
            completed_at: game.completed_at,
        },
    };

    Ok(Json(response))
}

/// Get move history for a game
pub async fn get_moves(
    Extension(auth): Extension<AuthUser>,
    State(state): State<AppState>,
    Path(game_id): Path<Uuid>,
) -> Result<Json<MoveListResponse>> {
    let user_id = auth.user_id;

    // Verify game exists and user is a player
    let game = games::find_by_id(&state.db, game_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Game not found".to_string()))?;

    if game.white_player_id != user_id && game.black_player_id != user_id {
        return Err(AppError::BadRequest(
            "You are not a player in this game".to_string(),
        ));
    }

    // Get moves
    let game_moves = db_moves::list_by_game(&state.db, game_id).await?;

    let moves: Vec<Move> = game_moves
        .into_iter()
        .map(|m| Move {
            id: m.id,
            game_id: m.game_id,
            move_number: m.move_number,
            player_color: if m.player_color == "white" {
                Color::White
            } else {
                Color::Black
            },
            move_uci: m.move_uci,
            move_san: m.move_san,
            position_before: m.position_before,
            position_after: m.position_after,
            timestamp: m.timestamp,
        })
        .collect();

    Ok(Json(MoveListResponse { moves }))
}

/// Export game as PGN
pub async fn export_pgn(
    Extension(auth): Extension<AuthUser>,
    State(state): State<AppState>,
    Path(game_id): Path<Uuid>,
) -> Result<Json<PgnResponse>> {
    let user_id = auth.user_id;

    // Verify game exists and user is a player
    let game = games::find_by_id(&state.db, game_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Game not found".to_string()))?;

    if game.white_player_id != user_id && game.black_player_id != user_id {
        return Err(AppError::BadRequest(
            "You are not a player in this game".to_string(),
        ));
    }

    // Generate PGN
    let pgn = state.game_service.generate_pgn(&state.db, game_id).await?;

    Ok(Json(PgnResponse { pgn }))
}
