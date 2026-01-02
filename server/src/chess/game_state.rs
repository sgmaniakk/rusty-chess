use chess::{Board, BoardStatus, ChessMove, Color as ChessColor, Square, Piece, MoveGen};
use serde::{Deserialize, Serialize};
use shared::types::Color;
use std::str::FromStr;
use anyhow::{Result, anyhow};

/// Wrapper around the chess crate's Board with serialization support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    fen: String,
}

impl GameState {
    /// Create a new game with the starting position
    pub fn new() -> Self {
        Self {
            fen: Board::default().to_string(),
        }
    }

    /// Create a GameState from a FEN string
    pub fn from_fen(fen: &str) -> Result<Self> {
        // Validate FEN by parsing it
        Board::from_str(fen)
            .map_err(|_| anyhow!("Invalid FEN string"))?;

        Ok(Self {
            fen: fen.to_string(),
        })
    }

    /// Get the current board
    pub fn board(&self) -> Result<Board> {
        Board::from_str(&self.fen)
            .map_err(|_| anyhow!("Failed to parse board from FEN"))
    }

    /// Get the FEN string
    pub fn fen(&self) -> &str {
        &self.fen
    }

    /// Get the current side to move
    pub fn current_turn(&self) -> Result<Color> {
        let board = self.board()?;
        Ok(match board.side_to_move() {
            ChessColor::White => Color::White,
            ChessColor::Black => Color::Black,
        })
    }

    /// Check if a move is legal
    pub fn is_legal_move(&self, move_uci: &str) -> Result<bool> {
        let board = self.board()?;
        let chess_move = ChessMove::from_str(move_uci)
            .map_err(|_| anyhow!("Invalid UCI move format"))?;

        Ok(board.legal(chess_move))
    }

    /// Apply a move and return the new game state
    pub fn make_move(&self, move_uci: &str) -> Result<(GameState, String)> {
        let board = self.board()?;
        let chess_move = ChessMove::from_str(move_uci)
            .map_err(|_| anyhow!("Invalid UCI move format"))?;

        if !board.legal(chess_move) {
            return Err(anyhow!("Illegal move"));
        }

        // Convert to SAN before making the move
        let san = super::notation::move_to_san(&board, chess_move)?;

        // Make the move
        let new_board = board.make_move_new(chess_move);

        Ok((
            GameState {
                fen: new_board.to_string(),
            },
            san,
        ))
    }

    /// Get the game status
    pub fn status(&self) -> Result<BoardStatus> {
        let board = self.board()?;
        Ok(board.status())
    }

    /// Check if the game is over
    pub fn is_game_over(&self) -> Result<bool> {
        Ok(self.status()? != BoardStatus::Ongoing)
    }

    /// Get a list of all legal moves in UCI format
    pub fn legal_moves(&self) -> Result<Vec<String>> {
        let board = self.board()?;
        let moves: Vec<String> = MoveGen::new_legal(&board)
            .map(|m| m.to_string())
            .collect();
        Ok(moves)
    }

    /// Get piece at a square
    pub fn piece_at(&self, square: Square) -> Result<Option<(Piece, ChessColor)>> {
        let board = self.board()?;
        Ok(board.piece_on(square).and_then(|piece| {
            board.color_on(square).map(|color| (piece, color))
        }))
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = GameState::new();
        assert!(game.board().is_ok());
        assert_eq!(game.current_turn().unwrap(), Color::White);
    }

    #[test]
    fn test_make_move() {
        let game = GameState::new();
        let (new_game, san) = game.make_move("e2e4").unwrap();
        assert_eq!(san, "e4");
        assert_ne!(game.fen(), new_game.fen());
    }

    #[test]
    fn test_illegal_move() {
        let game = GameState::new();
        assert!(game.make_move("e2e5").is_err());
    }

    #[test]
    fn test_from_fen() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let game = GameState::from_fen(fen).unwrap();
        assert_eq!(game.current_turn().unwrap(), Color::Black);
    }
}
