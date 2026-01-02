use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;
use anyhow::Result;

use crate::models::{Game, NewGame, GameWithPlayers};

/// Create a new game
pub async fn create_game(pool: &PgPool, new_game: &NewGame) -> Result<Game> {
    let game = sqlx::query_as::<_, Game>(
        r#"
        INSERT INTO games (
            white_player_id, black_player_id, current_position,
            game_state, status, current_turn
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, white_player_id, black_player_id, current_position,
                  game_state, status, current_turn, move_deadline,
                  created_at, completed_at
        "#,
    )
    .bind(new_game.white_player_id)
    .bind(new_game.black_player_id)
    .bind(&new_game.current_position)
    .bind(&new_game.game_state)
    .bind(&new_game.status)
    .bind(&new_game.current_turn)
    .fetch_one(pool)
    .await?;

    Ok(game)
}

/// Find a game by ID
pub async fn find_by_id(pool: &PgPool, game_id: Uuid) -> Result<Option<Game>> {
    let game = sqlx::query_as::<_, Game>(
        r#"
        SELECT id, white_player_id, black_player_id, current_position,
               game_state, status, current_turn, move_deadline,
               created_at, completed_at
        FROM games
        WHERE id = $1
        "#,
    )
    .bind(game_id)
    .fetch_optional(pool)
    .await?;

    Ok(game)
}

/// List games for a user
pub async fn list_by_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<GameWithPlayers>> {
    let games = sqlx::query_as::<_, GameWithPlayers>(
        r#"
        SELECT
            g.id, g.white_player_id, g.black_player_id,
            w.username as white_player_username,
            b.username as black_player_username,
            g.current_position, g.status, g.current_turn,
            g.move_deadline, g.created_at
        FROM games g
        JOIN users w ON g.white_player_id = w.id
        JOIN users b ON g.black_player_id = b.id
        WHERE g.white_player_id = $1 OR g.black_player_id = $1
        ORDER BY g.created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(games)
}

/// List active games for a user
pub async fn list_active_by_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<GameWithPlayers>> {
    let games = sqlx::query_as::<_, GameWithPlayers>(
        r#"
        SELECT
            g.id, g.white_player_id, g.black_player_id,
            w.username as white_player_username,
            b.username as black_player_username,
            g.current_position, g.status, g.current_turn,
            g.move_deadline, g.created_at
        FROM games g
        JOIN users w ON g.white_player_id = w.id
        JOIN users b ON g.black_player_id = b.id
        WHERE (g.white_player_id = $1 OR g.black_player_id = $1)
          AND g.status = 'active'
        ORDER BY g.move_deadline ASC NULLS LAST
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(games)
}

/// Update game state after a move
pub async fn update_after_move(
    pool: &PgPool,
    game_id: Uuid,
    new_position: &str,
    new_state: &JsonValue,
    new_turn: &str,
    deadline: DateTime<Utc>,
) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE games
        SET current_position = $1,
            game_state = $2,
            current_turn = $3,
            move_deadline = $4
        WHERE id = $5
        "#,
    )
    .bind(new_position)
    .bind(new_state)
    .bind(new_turn)
    .bind(deadline)
    .bind(game_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Update game status (for game over, forfeit, etc.)
pub async fn update_status(
    pool: &PgPool,
    game_id: Uuid,
    new_status: &str,
) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE games
        SET status = $1,
            completed_at = NOW()
        WHERE id = $2
        "#,
    )
    .bind(new_status)
    .bind(game_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Find games with expired deadlines
pub async fn find_expired_deadlines(pool: &PgPool) -> Result<Vec<Game>> {
    let games = sqlx::query_as::<_, Game>(
        r#"
        SELECT id, white_player_id, black_player_id, current_position,
               game_state, status, current_turn, move_deadline,
               created_at, completed_at
        FROM games
        WHERE status = 'active'
          AND move_deadline IS NOT NULL
          AND move_deadline < NOW()
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(games)
}

/// Find games with approaching deadlines
pub async fn find_approaching_deadlines(
    pool: &PgPool,
    hours_remaining: i32,
) -> Result<Vec<Game>> {
    let games = sqlx::query_as::<_, Game>(
        r#"
        SELECT id, white_player_id, black_player_id, current_position,
               game_state, status, current_turn, move_deadline,
               created_at, completed_at
        FROM games
        WHERE status = 'active'
          AND move_deadline IS NOT NULL
          AND move_deadline > NOW()
          AND move_deadline < NOW() + INTERVAL '1 hour' * $1
        "#,
    )
    .bind(hours_remaining)
    .fetch_all(pool)
    .await?;

    Ok(games)
}
