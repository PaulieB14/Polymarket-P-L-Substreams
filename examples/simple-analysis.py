#!/usr/bin/env python3

import json
import re

# Count arbitrage opportunities
with open('real-streaming-data.json', 'r') as f:
    content = f.read()

# Count blocks
blocks = content.count('"@block":')
print(f"📦 Total blocks processed: {blocks}")

# Count arbitrage opportunities
arbitrage_count = content.count('"arbitrageOpportunities"')
print(f"🎯 Arbitrage opportunity records: {arbitrage_count}")

# Count users
user_count = content.count('"userAddress"')
print(f"👥 User records: {user_count}")

# Extract profit amounts
profit_matches = re.findall(r'"profit": "([0-9.]+)"', content)
if profit_matches:
    total_profit = sum(float(p) for p in profit_matches)
    print(f"💰 Total profit detected: ${total_profit:.2f}")
    print(f"📈 Average profit per opportunity: ${total_profit/len(profit_matches):.4f}")

# Extract market IDs
market_matches = re.findall(r'"marketId": "([^"]+)"', content)
unique_markets = set(market_matches)
print(f"🏪 Unique markets: {len(unique_markets)}")

print(f"\n✅ Analysis complete!")
