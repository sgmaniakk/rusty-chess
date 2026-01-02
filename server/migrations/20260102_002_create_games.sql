-- Create games table
CREATE TABLE IF NOT EXISTS games (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    white_player_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    black_player_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    current_position TEXT NOT NULL DEFAULT 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1', -- Starting FEN
    game_state JSONB NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    current_turn VARCHAR(5) NOT NULL DEFAULT 'white',
    move_deadline TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    CONSTRAINT different_players CHECK (white_player_id != black_player_id),
    CONSTRAINT valid_status CHECK (status IN ('active', 'white_won', 'black_won', 'draw', 'abandoned')),
    CONSTRAINT valid_turn CHECK (current_turn IN ('white', 'black'))
);

-- Create indexes for efficient queries
CREATE INDEX idx_games_white_player ON games(white_player_id);
CREATE INDEX idx_games_black_player ON games(black_player_id);
CREATE INDEX idx_games_status ON games(status);
CREATE INDEX idx_games_deadline ON games(move_deadline) WHERE status = 'active';
CREATE INDEX idx_games_created_at ON games(created_at DESC);
