#!/usr/bin/env python3
"""
Demo Table for Polymarket Data

This shows what the data structure looks like and creates a sample table
"""

import json
from datetime import datetime

def create_demo_data():
    """Create demo data to show the table structure"""
    demo_data = {
        "userPnls": [
            {
                "user_address": "0x1234567890abcdef1234567890abcdef12345678",
                "net_usdc": "1500.50",
                "share_value": "2500.75",
                "trading_pnl": "4000.25",
                "liq_pnl": "100.00",
                "total_pnl": "4100.25"
            },
            {
                "user_address": "0xabcdef1234567890abcdef1234567890abcdef12",
                "net_usdc": "-500.25",
                "share_value": "1200.50",
                "trading_pnl": "700.25",
                "liq_pnl": "50.00",
                "total_pnl": "750.25"
            },
            {
                "user_address": "0x9876543210fedcba9876543210fedcba98765432",
                "net_usdc": "2000.00",
                "share_value": "0.00",
                "trading_pnl": "2000.00",
                "liq_pnl": "0.00",
                "total_pnl": "2000.00"
            },
            {
                "user_address": "0xfedcba0987654321fedcba0987654321fedcba09",
                "net_usdc": "-1200.75",
                "share_value": "800.25",
                "trading_pnl": "-400.50",
                "liq_pnl": "25.00",
                "total_pnl": "-375.50"
            },
            {
                "user_address": "0x5555555555555555555555555555555555555555",
                "net_usdc": "300.00",
                "share_value": "1500.00",
                "trading_pnl": "1800.00",
                "liq_pnl": "200.00",
                "total_pnl": "2000.00"
            }
        ],
        "markets": [
            {
                "condition_id": "0xabc123def456789",
                "question": "Will Bitcoin reach $100,000 by end of 2024?",
                "token0": "0x1111111111111111111111111111111111111111",
                "token1": "0x2222222222222222222222222222222222222222"
            },
            {
                "condition_id": "0xdef456abc123789",
                "question": "Will Ethereum reach $5,000 by end of 2024?",
                "token0": "0x3333333333333333333333333333333333333333",
                "token1": "0x4444444444444444444444444444444444444444"
            }
        ],
        "tokenTransfers": [
            {
                "transaction_hash": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
                "from_address": "0x0000000000000000000000000000000000000000",
                "to_address": "0x1234567890abcdef1234567890abcdef12345678",
                "token_id": "1",
                "value": "1000000000000000000",
                "block_number": "50000001"
            }
        ],
        "orderFills": [
            {
                "transaction_hash": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
                "maker_asset_id": "1",
                "taker_asset_id": "2",
                "maker_amount_filled": "500000000000000000",
                "taker_amount_filled": "1000000000000000000",
                "fee": "2500000000000000"
            }
        ],
        "rewardClaims": [
            {
                "transaction_hash": "0x9876543210fedcba9876543210fedcba9876543210fedcba9876543210fedcba",
                "airdrop_recipient": "0x1234567890abcdef1234567890abcdef12345678",
                "lc_amount": "100.50",
                "usd_amount": "100.50"
            }
        ]
    }
    return demo_data

def print_user_pnl_table(data):
    """Print a formatted table of user P&L data"""
    print("\n" + "="*120)
    print("🏆 POLYMARKET TRADER LEADERBOARD")
    print("="*120)
    print(f"{'Rank':<4} {'User Address':<44} {'Net USDC':<12} {'Share Value':<12} {'Trading P&L':<12} {'Liquidity P&L':<12} {'Total P&L':<12}")
    print("-"*120)
    
    # Sort by total P&L
    sorted_users = sorted(data["userPnls"], key=lambda x: float(x["total_pnl"]), reverse=True)
    
    for i, user in enumerate(sorted_users, 1):
        user_addr = user["user_address"][:10] + "..." + user["user_address"][-6:]
        net_usdc = f"${float(user['net_usdc']):,.2f}"
        share_value = f"${float(user['share_value']):,.2f}"
        trading_pnl = f"${float(user['trading_pnl']):,.2f}"
        liq_pnl = f"${float(user['liq_pnl']):,.2f}"
        total_pnl = f"${float(user['total_pnl']):,.2f}"
        
        print(f"{i:<4} {user_addr:<44} {net_usdc:<12} {share_value:<12} {trading_pnl:<12} {liq_pnl:<12} {total_pnl:<12}")

def print_markets_table(data):
    """Print a formatted table of markets"""
    print("\n" + "="*100)
    print("📊 ACTIVE MARKETS")
    print("="*100)
    print(f"{'Condition ID':<20} {'Question':<60} {'Token0':<20}")
    print("-"*100)
    
    for market in data["markets"]:
        condition_id = market["condition_id"][:10] + "..." + market["condition_id"][-6:]
        question = market["question"][:57] + "..." if len(market["question"]) > 60 else market["question"]
        token0 = market["token0"][:10] + "..." + market["token0"][-6:]
        
        print(f"{condition_id:<20} {question:<60} {token0:<20}")

def print_summary_stats(data):
    """Print summary statistics"""
    print("\n" + "="*60)
    print("📈 SUMMARY STATISTICS")
    print("="*60)
    
    total_users = len(data["userPnls"])
    total_markets = len(data["markets"])
    total_transfers = len(data["tokenTransfers"])
    total_orders = len(data["orderFills"])
    total_rewards = len(data["rewardClaims"])
    
    # Calculate totals
    total_pnl = sum(float(user["total_pnl"]) for user in data["userPnls"])
    total_trading_pnl = sum(float(user["trading_pnl"]) for user in data["userPnls"])
    total_liq_pnl = sum(float(user["liq_pnl"]) for user in data["userPnls"])
    
    print(f"Total Users: {total_users:,}")
    print(f"Total Markets: {total_markets:,}")
    print(f"Total Transfers: {total_transfers:,}")
    print(f"Total Order Fills: {total_orders:,}")
    print(f"Total Reward Claims: {total_rewards:,}")
    print(f"")
    print(f"Total P&L: ${total_pnl:,.2f}")
    print(f"Total Trading P&L: ${total_trading_pnl:,.2f}")
    print(f"Total Liquidity P&L: ${total_liq_pnl:,.2f}")
    
    # Top performers
    sorted_users = sorted(data["userPnls"], key=lambda x: float(x["total_pnl"]), reverse=True)
    if sorted_users:
        best_trader = sorted_users[0]
        print(f"")
        print(f"🏆 Best Trader: {best_trader['user_address'][:10]}...")
        print(f"   Total P&L: ${float(best_trader['total_pnl']):,.2f}")

def main():
    """Main function"""
    print("🎯 POLYMARKET DATA DEMO TABLE")
    print("="*60)
    print("This shows what the data structure looks like when streaming works")
    print("="*60)
    
    # Create demo data
    data = create_demo_data()
    
    # Print tables
    print_user_pnl_table(data)
    print_markets_table(data)
    print_summary_stats(data)
    
    print("\n" + "="*60)
    print("✅ This is what your Polymarket leaderboard will look like!")
    print("   Once quota resets, you can stream real data using the same structure.")
    print("="*60)

if __name__ == "__main__":
    main()
