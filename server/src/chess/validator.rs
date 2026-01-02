use chess::ChessMove;
use std::str::FromStr;
use anyhow::{Result, anyhow};

use super::GameState;

/// Validate a UCI move string format
pub fn validate_uci_format(move_uci: &str) -> Result<()> {
    if move_uci.len() < 4 || move_uci.len() > 5 {
        return Err(anyhow!("UCI move must be 4-5 characters"));
    }

    // Try to parse it as a ChessMove
    ChessMove::from_str(move_uci)
        .map_err(|_| anyhow!("Invalid UCI move format"))?;

    Ok(())
}

/// Validate that a move is legal in the given game state
pub fn validate_move(game_state: &GameState, move_uci: &str) -> Result<()> {
    // First validate the format
    validate_uci_format(move_uci)?;

    // Check if the move is legal
    if !game_state.is_legal_move(move_uci)? {
        return Err(anyhow!("Illegal move"));
    }

    Ok(())
}

/// Check if a game has ended and return the result
pub fn check_game_result(game_state: &GameState) -> Result<Option<GameResult>> {
    use chess::BoardStatus;

    let status = game_state.status()?;

    Ok(match status {
        BoardStatus::Checkmate => {
            // The side to move is in checkmate, so they lost
            Some(GameResult::Checkmate)
        }
        BoardStatus::Stalemate => Some(GameResult::Stalemate),
        BoardStatus::Ongoing => None,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Checkmate,
    Stalemate,
}

impl GameResult {
    pub fn is_draw(&self) -> bool {
        matches!(self, GameResult::Stalemate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_uci_format() {
        assert!(validate_uci_format("e2e4").is_ok());
        assert!(validate_uci_format("e7e8q").is_ok());
        assert!(validate_uci_format("e1g1").is_ok());
        assert!(validate_uci_format("e2").is_err());
        assert!(validate_uci_format("e2e4e5").is_err());
    }

    #[test]
    fn test_validate_move() {
        let game_state = GameState::new();
        assert!(validate_move(&game_state, "e2e4").is_ok());
        assert!(validate_move(&game_state, "e2e5").is_err());
    }

    #[test]
    fn test_check_game_result() {
        let game_state = GameState::new();
        assert_eq!(check_game_result(&game_state).unwrap(), None);

        // Fool's mate position
        let fen = "rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3";
        let game_state = GameState::from_fen(fen).unwrap();
        assert_eq!(
            check_game_result(&game_state).unwrap(),
            Some(GameResult::Checkmate)
        );
    }
}
