#!/usr/bin/env python3
"""
Polymarket User P&L Analyzer
Analyzes user activity from Substreams data
"""

import json
import sys
from datetime import datetime

def analyze_user_activity(data, target_address=None):
    """Analyze user activity from Substreams data"""
    
    if not data or 'userPnls' not in data:
        print("❌ No user P&L data found")
        return
    
    users = data['userPnls']
    if not users:
        print("❌ No users found in data")
        return
    
    print(f"📊 Found {len(users)} users in this block")
    print("=" * 60)
    
    # Filter by target address if provided
    if target_address:
        users = [u for u in users if target_address.lower() in u.get('userAddress', '').lower()]
        if not users:
            print(f"❌ No users found matching address: {target_address}")
            return
        print(f"🎯 Filtered to {len(users)} users matching: {target_address}")
        print("=" * 60)
    
    # Sort by total P&L (descending)
    users.sort(key=lambda x: float(x.get('totalPnl', '0')), reverse=True)
    
    for i, user in enumerate(users[:10], 1):  # Top 10 users
        address = user.get('userAddress', 'Unknown')
        net_usdc = user.get('netUsdc', '0')
        share_value = user.get('shareValue', '0')
        trading_pnl = user.get('tradingPnl', '0')
        liq_pnl = user.get('liqPnl', '0')
        total_pnl = user.get('totalPnl', '0')
        holdings = user.get('holdings', [])
        last_activity = user.get('lastActivity', 'Unknown')
        
        print(f"#{i} {address[:10]}...{address[-6:]}")
        print(f"   💰 Net USDC: ${net_usdc}")
        print(f"   📈 Share Value: ${share_value}")
        print(f"   📊 Trading P&L: ${trading_pnl}")
        print(f"   🎁 Liquidity P&L: ${liq_pnl}")
        print(f"   💎 Total P&L: ${total_pnl}")
        print(f"   🪙 Holdings: {len(holdings)} tokens")
        print(f"   ⏰ Last Activity: {last_activity}")
        print("-" * 40)

def main():
    if len(sys.argv) > 1:
        target_address = sys.argv[1]
    else:
        target_address = None
    
    # Read JSON from stdin
    try:
        data = json.load(sys.stdin)
        analyze_user_activity(data, target_address)
    except json.JSONDecodeError as e:
        print(f"❌ Error parsing JSON: {e}")
    except Exception as e:
        print(f"❌ Error: {e}")

if __name__ == "__main__":
    main()
