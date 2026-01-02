use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

use crate::models::{MoveRecord, NewMove};

/// Insert a new move
pub async fn create_move(pool: &PgPool, new_move: &NewMove) -> Result<MoveRecord> {
    let move_record = sqlx::query_as::<_, MoveRecord>(
        r#"
        INSERT INTO moves (
            game_id, move_number, player_color, move_uci,
            move_san, position_before, position_after
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, game_id, move_number, player_color, move_uci,
                  move_san, position_before, position_after, timestamp
        "#,
    )
    .bind(new_move.game_id)
    .bind(new_move.move_number)
    .bind(&new_move.player_color)
    .bind(&new_move.move_uci)
    .bind(&new_move.move_san)
    .bind(&new_move.position_before)
    .bind(&new_move.position_after)
    .fetch_one(pool)
    .await?;

    Ok(move_record)
}

/// Get all moves for a game
pub async fn list_by_game(pool: &PgPool, game_id: Uuid) -> Result<Vec<MoveRecord>> {
    let moves = sqlx::query_as::<_, MoveRecord>(
        r#"
        SELECT id, game_id, move_number, player_color, move_uci,
               move_san, position_before, position_after, timestamp
        FROM moves
        WHERE game_id = $1
        ORDER BY move_number ASC, player_color ASC
        "#,
    )
    .bind(game_id)
    .fetch_all(pool)
    .await?;

    Ok(moves)
}

/// Get the last move for a game
pub async fn get_last_move(pool: &PgPool, game_id: Uuid) -> Result<Option<MoveRecord>> {
    let move_record = sqlx::query_as::<_, MoveRecord>(
        r#"
        SELECT id, game_id, move_number, player_color, move_uci,
               move_san, position_before, position_after, timestamp
        FROM moves
        WHERE game_id = $1
        ORDER BY move_number DESC, player_color DESC
        LIMIT 1
        "#,
    )
    .bind(game_id)
    .fetch_optional(pool)
    .await?;

    Ok(move_record)
}

/// Count moves for a game
pub async fn count_by_game(pool: &PgPool, game_id: Uuid) -> Result<i64> {
    let count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM moves WHERE game_id = $1
        "#,
    )
    .bind(game_id)
    .fetch_one(pool)
    .await?;

    Ok(count.0)
}
