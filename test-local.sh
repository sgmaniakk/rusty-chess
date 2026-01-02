#!/bin/bash
# Simple test of local mode with automated moves

echo "Testing Rusty Chess Local Mode"
echo "Playing a few moves automatically..."

# Scholar's mate sequence
echo -e "e2e4\ne7e5\nf1c4\nb8c6\nd1h5\ng8f6\nh5f7\nquit\n" | cargo run --bin rusty-chess-local

echo ""
echo "Test complete!"
