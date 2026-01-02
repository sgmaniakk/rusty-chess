use chess::{Board, Color, Piece, Square, File, Rank};

/// Display the chess board with Unicode pieces
pub fn display_board(board: &Board) {
    println!("\n  a b c d e f g h");

    for rank in (0..8).rev() {
        print!("{} ", rank + 1);

        for file in 0..8 {
            let square = Square::make_square(
                Rank::from_index(rank),
                File::from_index(file),
            );

            let piece_str = if let Some(piece) = board.piece_on(square) {
                let color = board.color_on(square).unwrap();
                piece_to_unicode(piece, color)
            } else {
                // Checkered pattern
                if (rank + file) % 2 == 0 {
                    "·"
                } else {
                    " "
                }
            };

            print!("{} ", piece_str);
        }

        println!("{}", rank + 1);
    }

    println!("  a b c d e f g h\n");
}

/// Convert a piece to Unicode character
fn piece_to_unicode(piece: Piece, color: Color) -> &'static str {
    match (piece, color) {
        (Piece::King, Color::White) => "♔",
        (Piece::Queen, Color::White) => "♕",
        (Piece::Rook, Color::White) => "♖",
        (Piece::Bishop, Color::White) => "♗",
        (Piece::Knight, Color::White) => "♘",
        (Piece::Pawn, Color::White) => "♙",
        (Piece::King, Color::Black) => "♚",
        (Piece::Queen, Color::Black) => "♛",
        (Piece::Rook, Color::Black) => "♜",
        (Piece::Bishop, Color::Black) => "♝",
        (Piece::Knight, Color::Black) => "♞",
        (Piece::Pawn, Color::Black) => "♟",
    }
}

/// Display the current game status
pub fn display_status(board: &Board) {
    let turn = board.side_to_move();
    println!("Turn: {}", if turn == Color::White { "White" } else { "Black" });

    // Check for check
    if board.checkers().popcnt() > 0 {
        println!("CHECK!");
    }

    // Display game status
    use chess::BoardStatus;
    match board.status() {
        BoardStatus::Checkmate => {
            let winner = if turn == Color::White { "Black" } else { "White" };
            println!("\n🎉 CHECKMATE! {} wins!", winner);
        }
        BoardStatus::Stalemate => {
            println!("\nSTALEMATE! Game is a draw.");
        }
        BoardStatus::Ongoing => {}
    }
}
