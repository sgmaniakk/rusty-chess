# Rusty Chess ♔

A network-based correspondence chess application built in Rust with a client-server architecture. Play chess with 3-day move deadlines, real-time notifications, and a beautiful terminal UI.

## Features

- **Network Play**: Client-server architecture for remote play
- **Move Deadlines**: 3-day timer per move with automatic forfeit
- **Real-time Updates**: WebSocket notifications for opponent moves
- **Terminal UI**: Beautiful Unicode chess board (♔♕♖♗♘♙)
- **Move History**: Full game history in algebraic notation
- **PGN Export**: Export games to standard PGN format

## Tech Stack

- **Chess Logic**: [`chess` crate](https://docs.rs/chess/latest/chess/)
- **Server**: Axum + Tokio
- **Database**: PostgreSQL with SQLx
- **Terminal UI**: Ratatui
- **Auth**: JWT tokens with bcrypt

## Prerequisites

### Option 1: Docker (Recommended)
- [Docker](https://www.docker.com/get-started)
- Rust 1.75+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)

### Option 2: Local PostgreSQL
- PostgreSQL 14+
- Rust 1.75+

## Quick Start

### 1. Clone and Setup

```bash
cd rusty-chess
cp .env.example .env
```

### 2. Start Database

**With Docker:**
```bash
docker compose up -d
```

**With Local PostgreSQL:**
```bash
createdb rusty_chess
# Update .env with your database credentials
```

### 3. Run Migrations

```bash
cargo install sqlx-cli --no-default-features --features postgres
cd server
sqlx migrate run
```

### 4. Start Server

```bash
cargo run --bin rusty-chess-server
```

### 5. Start Client (in another terminal)

```bash
cargo run --bin rusty-chess-client
```

## Architecture

```
rusty-chess/
├── server/          # Axum server with chess logic
├── client/          # Ratatui terminal UI
└── shared/          # Shared types and protocols
```

### Database Schema

- **users**: Player accounts with bcrypt passwords
- **games**: Game state with FEN positions and deadlines
- **moves**: Complete move history in UCI and SAN notation

### API Endpoints

```
POST   /api/auth/register    # Create account
POST   /api/auth/login       # Get JWT token
GET    /api/games            # List your games
POST   /api/games            # Challenge opponent
GET    /api/games/{id}       # Game details
POST   /api/games/{id}/moves # Submit move
GET    /api/games/{id}/pgn   # Export PGN
```

### WebSocket Messages

- `MoveMade` - Opponent's move notification
- `DeadlineWarning` - Approaching deadline alert (24h, 6h, 1h)
- `GameStatusChanged` - Game over notification

## Development

### Build
```bash
cargo build
```

### Test
```bash
cargo test
```

### Check
```bash
cargo check
```

## License

MIT

## Author

Matt Anderson
