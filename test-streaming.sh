#!/bin/bash

# Polymarket P&L Substreams - Streaming Test Script
# This script tests the arbitrage detection by streaming a few blocks

echo "🚀 Testing Polymarket P&L Substreams v0.3.1"
echo "📦 Package: polymarket-pnl@v0.3.1"
echo "📊 Module: map_enhanced_pnl_with_neg_risk"
echo "🔢 Testing blocks: 60000000-60000002"
echo "─" | tr -d '\n' && printf "%0.s─" {1..80} && echo

# Test the enhanced P&L module
echo "🎯 Testing Enhanced P&L with Arbitrage Detection..."
substreams run polymarket-pnl@v0.3.1 map_enhanced_pnl_with_neg_risk \
  --start-block 60000000 \
  --stop-block 60000002

echo ""
echo "─" | tr -d '\n' && printf "%0.s─" {1..80} && echo
echo "�� Testing Negative Risk Market Analysis..."
substreams run polymarket-pnl@v0.3.1 map_neg_risk_market_analysis \
  --start-block 60000000 \
  --stop-block 60000002

echo ""
echo "─" | tr -d '\n' && printf "%0.s─" {1..80} && echo
echo "🎯 Testing ERC1155 Transfers..."
substreams run polymarket-pnl@v0.3.1 map_erc1155_transfer_single \
  --start-block 60000000 \
  --stop-block 60000002

echo ""
echo "✅ Streaming test completed!"
echo "📊 Check the output above for arbitrage opportunities and user data"
