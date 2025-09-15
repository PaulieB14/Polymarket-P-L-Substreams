#!/bin/bash

# User Activity Tracker for Polymarket Substreams
# Usage: ./track_user_activity.sh <wallet_address> [start_block] [end_block]

USER_ADDRESS=${1:-""}
START_BLOCK=${2:-65000000}
END_BLOCK=${3:-+5}

if [ -z "$USER_ADDRESS" ]; then
    echo "Usage: $0 <wallet_address> [start_block] [end_block]"
    echo "Example: $0 0x1234...abcd 65000000 +10"
    exit 1
fi

echo "🔍 Tracking activity for address: $USER_ADDRESS"
echo "📊 Block range: $START_BLOCK to $END_BLOCK"
echo ""

# Run Substreams and filter for the specific user
substreams run substreams.yaml map_pure_dune_pnl \
    --start-block $START_BLOCK \
    --stop-block $END_BLOCK \
    --substreams-endpoint polygon.streamingfast.io:443 | \
jq --arg addr "$USER_ADDRESS" '
.data.userPnls[]? | 
select(.userAddress | ascii_downcase | contains($addr | ascii_downcase)) |
{
    userAddress: .userAddress,
    netUsdc: .netUsdc,
    shareValue: .shareValue,
    tradingPnl: .tradingPnl,
    liqPnl: .liqPnl,
    totalPnl: .totalPnl,
    holdings: .holdings,
    lastActivity: .lastActivity
}'
