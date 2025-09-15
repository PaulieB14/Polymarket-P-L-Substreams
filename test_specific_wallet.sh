#!/bin/bash

# Test specific wallet: 0x6596a3C7C2eA69D04F01F064AA4e914196BbA0a7

WALLET="0x6596a3C7C2eA69D04F01F064AA4e914196BbA0a7"
START_BLOCK=65000000
END_BLOCK=+10

echo "🎯 TESTING SPECIFIC WALLET: $WALLET"
echo "===================================="
echo ""

echo "📊 Running Substreams on blocks $START_BLOCK to $END_BLOCK..."
echo ""

# Run the Substreams and capture data
substreams run substreams.yaml map_pure_dune_pnl \
    --start-block $START_BLOCK \
    --stop-block $END_BLOCK \
    --substreams-endpoint polygon.streamingfast.io:443 | \
jq --arg wallet "$WALLET" '
.["@data"] | {
    totalUsers: .totalUsers,
    userCount: (.userPnls | length),
    transferCount: (.tokenTransfers | length),
    orderCount: (.orderFills | length),
    targetWalletFound: (.userPnls | map(select(.userAddress | ascii_downcase | contains($wallet | ascii_downcase))) | length > 0),
    sampleUsers: (.userPnls[0:3] | map({userAddress, totalPnl, holdings: (.holdings | length)}))
}'

echo ""
echo "🔍 To track this wallet in real-time:"
echo "1. Run: substreams run substreams.yaml map_pure_dune_pnl --start-block [BLOCK] --stop-block +1"
echo "2. Filter for your wallet address in the output"
echo "3. The Substreams will capture all their Polymarket activity"
echo ""
echo "💡 The wallet may not appear in these specific blocks if they haven't been active recently."
echo "   Try running on different block ranges or historical blocks when they were active."
