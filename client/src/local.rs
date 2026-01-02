mod display;

use chess::{Board, ChessMove, BoardStatus, MoveGen};
use std::io::{self, Write};
use std::str::FromStr;

use display::{display_board, display_status};

fn main() {
    println!("=== Rusty Chess - Local Two Player Mode ===\n");
    println!("Enter moves in UCI format (e.g., e2e4, e7e8q for promotion)");
    println!("Type 'quit' to exit, 'moves' to see legal moves\n");

    let mut board = Board::default();
    let mut move_history: Vec<String> = Vec::new();

    loop {
        // Display the board
        display_board(&board);
        display_status(&board);

        // Check if game is over
        match board.status() {
            BoardStatus::Checkmate | BoardStatus::Stalemate => {
                println!("\nGame Over!");
                println!("\nMove history:");
                for (i, mv) in move_history.iter().enumerate() {
                    if i % 2 == 0 {
                        print!("{}. {} ", (i / 2) + 1, mv);
                    } else {
                        println!("{}", mv);
                    }
                }
                if move_history.len() % 2 == 1 {
                    println!();
                }
                break;
            }
            BoardStatus::Ongoing => {}
        }

        // Get move input
        print!("Enter move: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Handle special commands
        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            println!("Thanks for playing!");
            break;
        }

        if input.eq_ignore_ascii_case("moves") {
            show_legal_moves(&board);
            continue;
        }

        if input.eq_ignore_ascii_case("help") {
            show_help();
            continue;
        }

        // Try to parse and make the move
        match ChessMove::from_str(input) {
            Ok(chess_move) => {
                if board.legal(chess_move) {
                    move_history.push(input.to_string());
                    board = board.make_move_new(chess_move);
                } else {
                    println!("❌ Illegal move! Try again.");
                }
            }
            Err(_) => {
                println!("❌ Invalid move format! Use UCI notation (e.g., e2e4)");
            }
        }
    }
}

fn show_legal_moves(board: &Board) {
    println!("\nLegal moves:");
    let mut moves: Vec<String> = MoveGen::new_legal(board)
        .map(|m| m.to_string())
        .collect();
    moves.sort();

    for (i, mv) in moves.iter().enumerate() {
        print!("{:6}", mv);
        if (i + 1) % 8 == 0 {
            println!();
        }
    }
    println!("\n");
}

fn show_help() {
    println!("\n=== Help ===");
    println!("Enter moves in UCI format:");
    println!("  - Normal move: e2e4 (from e2 to e4)");
    println!("  - Castling: e1g1 (kingside), e1c1 (queenside)");
    println!("  - Promotion: e7e8q (promote to queen)");
    println!("    Promotion pieces: q=queen, r=rook, b=bishop, n=knight");
    println!("\nCommands:");
    println!("  moves - Show all legal moves");
    println!("  help  - Show this help");
    println!("  quit  - Exit the game");
    println!();
}
