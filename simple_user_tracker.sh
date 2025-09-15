#!/bin/bash

# Simple User Activity Tracker
# Usage: ./simple_user_tracker.sh [wallet_address]

TARGET_ADDRESS=${1:-""}
START_BLOCK=65000000
END_BLOCK=+3

echo "🔍 Polymarket User Activity Tracker"
echo "=================================="

if [ -n "$TARGET_ADDRESS" ]; then
    echo "🎯 Looking for address: $TARGET_ADDRESS"
else
    echo "📊 Showing recent user activity"
fi

echo ""

# Get the latest block data and extract user info
substreams run substreams.yaml map_pure_dune_pnl \
    --start-block $START_BLOCK \
    --stop-block $END_BLOCK \
    --substreams-endpoint polygon.streamingfast.io:443 2>/dev/null | \
grep -A 20 '"userPnls"' | \
grep -E '"userAddress"|"totalPnl"|"tradingPnl"|"lastActivity"' | \
head -40

echo ""
echo "💡 To track a specific user, run:"
echo "   ./simple_user_tracker.sh 0x1234...abcd"
