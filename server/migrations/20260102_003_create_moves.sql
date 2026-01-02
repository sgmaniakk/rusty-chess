-- Create moves table
CREATE TABLE IF NOT EXISTS moves (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    game_id UUID NOT NULL REFERENCES games(id) ON DELETE CASCADE,
    move_number INT NOT NULL,
    player_color VARCHAR(5) NOT NULL,
    move_uci VARCHAR(10) NOT NULL,
    move_san VARCHAR(20) NOT NULL,
    position_before TEXT NOT NULL,
    position_after TEXT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT valid_player_color CHECK (player_color IN ('white', 'black')),
    CONSTRAINT unique_game_move UNIQUE (game_id, move_number, player_color)
);

-- Create indexes for efficient queries
CREATE INDEX idx_moves_game ON moves(game_id, move_number);
CREATE INDEX idx_moves_timestamp ON moves(timestamp DESC);
