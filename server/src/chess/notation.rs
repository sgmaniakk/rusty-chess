use chess::{Board, ChessMove, Piece, File, Rank, MoveGen};
use anyhow::{Result, anyhow};

/// Convert a ChessMove to Standard Algebraic Notation (SAN)
pub fn move_to_san(board: &Board, chess_move: ChessMove) -> Result<String> {
    if !board.legal(chess_move) {
        return Err(anyhow!("Illegal move cannot be converted to SAN"));
    }

    let source = chess_move.get_source();
    let dest = chess_move.get_dest();
    let promotion = chess_move.get_promotion();

    // Get the piece being moved
    let piece = board.piece_on(source)
        .ok_or_else(|| anyhow!("No piece on source square"))?;

    // Check for castling
    if piece == Piece::King {
        if source.get_file() == File::E {
            if dest.get_file() == File::G {
                return Ok("O-O".to_string()); // Kingside castling
            } else if dest.get_file() == File::C {
                return Ok("O-O-O".to_string()); // Queenside castling
            }
        }
    }

    let mut san = String::new();

    // Add piece letter (except for pawns)
    if piece != Piece::Pawn {
        san.push(piece_to_char(piece));
    }

    // Disambiguate if necessary (for pieces other than pawns and kings)
    if piece != Piece::Pawn && piece != Piece::King {
        let disambiguation = get_disambiguation(board, chess_move)?;
        san.push_str(&disambiguation);
    }

    // Capture notation
    let is_capture = board.piece_on(dest).is_some();
    if is_capture {
        if piece == Piece::Pawn {
            // Pawn captures include the source file
            san.push(file_to_char(source.get_file()));
        }
        san.push('x');
    } else if piece == Piece::Pawn && source.get_file() != dest.get_file() {
        // En passant
        san.push(file_to_char(source.get_file()));
        san.push('x');
    }

    // Destination square
    san.push(file_to_char(dest.get_file()));
    san.push(rank_to_char(dest.get_rank()));

    // Promotion
    if let Some(promo_piece) = promotion {
        san.push('=');
        san.push(piece_to_char(promo_piece));
    }

    // Check or checkmate
    let new_board = board.make_move_new(chess_move);
    match new_board.status() {
        chess::BoardStatus::Checkmate => san.push('#'),
        chess::BoardStatus::Ongoing => {
            if new_board.checkers().popcnt() > 0 {
                san.push('+');
            }
        }
        _ => {}
    }

    Ok(san)
}

/// Get disambiguation string for a move (file, rank, or both)
fn get_disambiguation(board: &Board, chess_move: ChessMove) -> Result<String> {
    let source = chess_move.get_source();
    let dest = chess_move.get_dest();
    let piece = board.piece_on(source)
        .ok_or_else(|| anyhow!("No piece on source square"))?;

    // Find all pieces of the same type that can move to the destination
    let mut same_type_moves = Vec::new();
    for m in MoveGen::new_legal(board) {
        if m.get_dest() == dest {
            if let Some(p) = board.piece_on(m.get_source()) {
                if p == piece && m.get_source() != source {
                    same_type_moves.push(m);
                }
            }
        }
    }

    if same_type_moves.is_empty() {
        return Ok(String::new());
    }

    // Check if file disambiguation is enough
    let same_file = same_type_moves.iter()
        .any(|m| m.get_source().get_file() == source.get_file());

    // Check if rank disambiguation is enough
    let same_rank = same_type_moves.iter()
        .any(|m| m.get_source().get_rank() == source.get_rank());

    if !same_file {
        // File is unique
        Ok(file_to_char(source.get_file()).to_string())
    } else if !same_rank {
        // Rank is unique
        Ok(rank_to_char(source.get_rank()).to_string())
    } else {
        // Need both file and rank
        Ok(format!(
            "{}{}",
            file_to_char(source.get_file()),
            rank_to_char(source.get_rank())
        ))
    }
}

fn piece_to_char(piece: Piece) -> char {
    match piece {
        Piece::King => 'K',
        Piece::Queen => 'Q',
        Piece::Rook => 'R',
        Piece::Bishop => 'B',
        Piece::Knight => 'N',
        Piece::Pawn => ' ', // Not used, but included for completeness
    }
}

fn file_to_char(file: File) -> char {
    match file {
        File::A => 'a',
        File::B => 'b',
        File::C => 'c',
        File::D => 'd',
        File::E => 'e',
        File::F => 'f',
        File::G => 'g',
        File::H => 'h',
    }
}

fn rank_to_char(rank: Rank) -> char {
    match rank {
        Rank::First => '1',
        Rank::Second => '2',
        Rank::Third => '3',
        Rank::Fourth => '4',
        Rank::Fifth => '5',
        Rank::Sixth => '6',
        Rank::Seventh => '7',
        Rank::Eighth => '8',
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_pawn_move_to_san() {
        let board = Board::default();
        let chess_move = ChessMove::from_str("e2e4").unwrap();
        let san = move_to_san(&board, chess_move).unwrap();
        assert_eq!(san, "e4");
    }

    #[test]
    fn test_knight_move_to_san() {
        let board = Board::default();
        let chess_move = ChessMove::from_str("g1f3").unwrap();
        let san = move_to_san(&board, chess_move).unwrap();
        assert_eq!(san, "Nf3");
    }

    #[test]
    fn test_castling_kingside() {
        // Position after 1.e4 e5 2.Nf3 Nf6 3.Bc4 Bc5
        let fen = "rnbqk2r/pppp1ppp/5n2/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4";
        let board = Board::from_str(fen).unwrap();
        let chess_move = ChessMove::from_str("e1g1").unwrap();
        let san = move_to_san(&board, chess_move).unwrap();
        assert_eq!(san, "O-O");
    }
}
