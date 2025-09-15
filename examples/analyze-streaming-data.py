#!/usr/bin/env python3

"""
Polymarket P&L Substreams - Data Analysis Script

This script analyzes the real streaming data we captured to extract
key metrics and insights about arbitrage opportunities.

Usage: python analyze-streaming-data.py
"""

import json
import sys
from datetime import datetime
from typing import Dict, List, Any

def load_json_data(filename: str) -> List[Dict[str, Any]]:
    """Load and parse JSON streaming data"""
    try:
        with open(filename, 'r') as f:
            data = f.read()
        
        # Split by block separators and parse each block
        blocks = []
        for line in data.strip().split('\n'):
            if line.strip() and line.startswith('{'):
                try:
                    block_data = json.loads(line)
                    blocks.append(block_data)
                except json.JSONDecodeError:
                    continue
        return blocks
    except FileNotFoundError:
        print(f"❌ Error: {filename} not found")
        return []
    except Exception as e:
        print(f"❌ Error loading {filename}: {e}")
        return []

def analyze_arbitrage_opportunities(blocks: List[Dict[str, Any]]) -> Dict[str, Any]:
    """Analyze arbitrage opportunities from streaming data"""
    total_opportunities = 0
    total_profit = 0.0
    unique_markets = set()
    
    for block in blocks:
        if '@data' in block and 'arbitrageOpportunities' in block['@data']:
            opportunities = block['@data']['arbitrageOpportunities']
            total_opportunities += len(opportunities)
            
            for opp in opportunities:
                profit = float(opp.get('profit', 0))
                total_profit += profit
                unique_markets.add(opp.get('marketId', ''))
    
    return {
        'total_opportunities': total_opportunities,
        'total_profit': total_profit,
        'unique_markets': len(unique_markets),
        'average_profit_per_opportunity': total_profit / max(total_opportunities, 1)
    }

def analyze_user_activity(blocks: List[Dict[str, Any]]) -> Dict[str, Any]:
    """Analyze user activity from streaming data"""
    total_users = 0
    total_trades = 0
    unique_users = set()
    
    for block in blocks:
        if '@data' in block and 'userPnls' in block['@data']:
            users = block['@data']['userPnls']
            total_users += len(users)
            
            for user in users:
                unique_users.add(user.get('userAddress', ''))
                trades = int(user.get('totalTrades', 0))
                total_trades += trades
    
    return {
        'total_user_records': total_users,
        'unique_users': len(unique_users),
        'total_trades': total_trades,
        'average_trades_per_user': total_trades / max(len(unique_users), 1)
    }

def analyze_blocks(blocks: List[Dict[str, Any]]) -> Dict[str, Any]:
    """Analyze block-level metrics"""
    if not blocks:
        return {}
    
    first_block = blocks[0]['@block']
    last_block = blocks[-1]['@block']
    block_range = last_block - first_block + 1
    
    return {
        'first_block': first_block,
        'last_block': last_block,
        'block_range': block_range,
        'total_blocks': len(blocks)
    }

def print_analysis_table(title: str, data: Dict[str, Any]):
    """Print analysis data in a nice table format"""
    print(f"\n📊 {title}")
    print("─" * 60)
    
    for key, value in data.items():
        if isinstance(value, float):
            print(f"{key:30} | {value:>15.4f}")
        else:
            print(f"{key:30} | {value:>15}")
    print("─" * 60)

def main():
    print("🚀 Polymarket P&L Substreams - Data Analysis")
    print("=" * 60)
    
    # Load streaming data
    pnl_blocks = load_json_data('real-streaming-data.json')
    arbitrage_blocks = load_json_data('arbitrage-opportunities.json')
    
    if not pnl_blocks and not arbitrage_blocks:
        print("❌ No data files found. Run the streaming examples first.")
        return
    
    # Analyze arbitrage opportunities
    if arbitrage_blocks:
        arbitrage_analysis = analyze_arbitrage_opportunities(arbitrage_blocks)
        print_analysis_table("ARBITRAGE OPPORTUNITIES", arbitrage_analysis)
    
    # Analyze user activity
    if pnl_blocks:
        user_analysis = analyze_user_activity(pnl_blocks)
        print_analysis_table("USER ACTIVITY", user_analysis)
    
    # Analyze blocks
    if pnl_blocks:
        block_analysis = analyze_blocks(pnl_blocks)
        print_analysis_table("BLOCK ANALYSIS", block_analysis)
    
    # Summary insights
    print(f"\n💡 KEY INSIGHTS")
    print("─" * 60)
    
    if arbitrage_blocks:
        arb_analysis = analyze_arbitrage_opportunities(arbitrage_blocks)
        print(f"🎯 Found {arb_analysis['total_opportunities']} arbitrage opportunities")
        print(f"💰 Total potential profit: ${arb_analysis['total_profit']:.2f}")
        print(f"📈 Average profit per opportunity: ${arb_analysis['average_profit_per_opportunity']:.4f}")
    
    if pnl_blocks:
        user_analysis = analyze_user_activity(pnl_blocks)
        print(f"👥 Processed {user_analysis['unique_users']} unique users")
        print(f"📊 Total trades tracked: {user_analysis['total_trades']}")
        print(f"⚡ Average trades per user: {user_analysis['average_trades_per_user']:.1f}")
    
    print(f"\n✅ Analysis complete!")
    print(f"📁 Data files: real-streaming-data.json, arbitrage-opportunities.json")

if __name__ == "__main__":
    main()
