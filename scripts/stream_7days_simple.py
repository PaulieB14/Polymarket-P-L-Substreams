#!/usr/bin/env python3
"""
Simple 7-day streaming for Polymarket dashboard
No local indexing - just stream data directly
"""

import json
import subprocess
import sys
import time
from datetime import datetime
from typing import Dict, Any

def estimate_recent_blocks():
    """Estimate recent block numbers for 7 days"""
    # Polygon produces ~1 block every 2 seconds
    # 7 days = 7 * 24 * 60 * 60 / 2 = 302,400 blocks
    current_time = int(time.time())
    current_block = current_time // 2
    seven_days_ago_block = current_block - 302400
    
    print(f"📅 7-Day Range: {seven_days_ago_block:,} to {current_block:,}")
    print(f"📊 Total blocks: {current_block - seven_days_ago_block:,}")
    return seven_days_ago_block, current_block

def stream_blocks_in_chunks(start_block: int, end_block: int, chunk_size: int = 100):
    """Stream blocks in small chunks to avoid timeouts"""
    print(f"\n🚀 STREAMING 7 DAYS OF POLYMARKET DATA")
    print("=" * 60)
    
    all_data = {
        "userPnls": [],
        "marketData": [],
        "tokenTransfers": [],
        "orderFills": [],
        "rewardClaims": []
    }
    
    current_start = start_block
    chunk_count = 0
    successful_chunks = 0
    
    while current_start < end_block:
        current_end = min(current_start + chunk_size, end_block)
        chunk_count += 1
        
        print(f"\n📦 Chunk {chunk_count}: blocks {current_start:,} to {current_end:,}")
        
        try:
            cmd = [
                "substreams", "run", "substreams.yaml", "map_pure_dune_pnl",
                "--start-block", str(current_start),
                "--stop-block", str(current_end)
            ]
            
            print("   ⏳ Running Substreams...")
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=120)
            
            if result.returncode == 0:
                # Extract data from this chunk
                chunk_data = extract_data_from_output(result.stdout)
                if chunk_data:
                    # Merge data
                    all_data["userPnls"].extend(chunk_data.get("userPnls", []))
                    all_data["marketData"].extend(chunk_data.get("marketData", []))
                    all_data["tokenTransfers"].extend(chunk_data.get("tokenTransfers", []))
                    all_data["orderFills"].extend(chunk_data.get("orderFills", []))
                    all_data["rewardClaims"].extend(chunk_data.get("rewardClaims", []))
                    
                    users_found = len(chunk_data.get("userPnls", []))
                    print(f"   ✅ Found {users_found} users")
                    successful_chunks += 1
                else:
                    print(f"   ⚠️  No data in this chunk")
            else:
                print(f"   ❌ Chunk failed: {result.stderr[:100]}...")
            
            current_start = current_end
            
            # Small delay to avoid overwhelming the endpoint
            time.sleep(1)
            
        except subprocess.TimeoutExpired:
            print(f"   ⏰ Chunk {chunk_count} timed out, skipping...")
            current_start = current_end
        except Exception as e:
            print(f"   ❌ Error in chunk {chunk_count}: {e}")
            current_start = current_end
    
    print(f"\n📊 STREAMING SUMMARY")
    print("=" * 40)
    print(f"✅ Successful chunks: {successful_chunks}/{chunk_count}")
    print(f"👥 Total users found: {len(set(user['userAddress'] for user in all_data['userPnls']))}")
    print(f"📈 Markets: {len(all_data['marketData'])}")
    print(f"🔄 Transfers: {len(all_data['tokenTransfers'])}")
    print(f"📋 Orders: {len(all_data['orderFills'])}")
    print(f"🎁 Claims: {len(all_data['rewardClaims'])}")
    
    return all_data

def extract_data_from_output(output: str) -> Dict[str, Any]:
    """Extract data from Substreams output"""
    try:
        lines = output.strip().split('\n')
        
        for line in lines:
            if '"@data"' in line and '"userPnls"' in line:
                # Find the data section
                start_idx = line.find('"@data"')
                if start_idx != -1:
                    data_start = line.find('{', start_idx)
                    if data_start != -1:
                        # Find the matching closing brace
                        brace_count = 0
                        data_end = data_start
                        for i, char in enumerate(line[data_start:], data_start):
                            if char == '{':
                                brace_count += 1
                            elif char == '}':
                                brace_count -= 1
                                if brace_count == 0:
                                    data_end = i + 1
                                    break
                        
                        data_json = line[data_start:data_end]
                        data = json.loads(data_json)
                        return data
    except Exception as e:
        print(f"   ⚠️  Data extraction error: {e}")
    
    return {}

def create_dashboard(data: Dict[str, Any]):
    """Create Dune-style dashboard from streamed data"""
    print(f"\n📊 CREATING 7-DAY DASHBOARD")
    print("=" * 60)
    
    if not data["userPnls"]:
        print("❌ No user data found")
        return None
    
    # Calculate metrics
    total_users = len(set(user["userAddress"] for user in data["userPnls"]))
    total_profits = sum(float(user.get("totalPnl", "0")) for user in data["userPnls"] if float(user.get("totalPnl", "0")) > 0)
    total_losses = abs(sum(float(user.get("totalPnl", "0")) for user in data["userPnls"] if float(user.get("totalPnl", "0")) < 0))
    
    print(f"👥 Total Users: {total_users:,}")
    print(f"💰 Total Profits: ${total_profits:,.2f}")
    print(f"📉 Total Losses: ${total_losses:,.2f}")
    print(f"📈 Net P&L: ${total_profits - total_losses:,.2f}")
    
    # Create leaderboard
    users = data["userPnls"]
    users.sort(key=lambda x: float(x.get("totalPnl", "0")), reverse=True)
    
    print(f"\n🏆 TOP 20 TRADERS (7 DAYS)")
    print("-" * 80)
    print(f"{'Rank':<4} {'User':<12} {'Net USDC':<12} {'Trading P&L':<12} {'Total P&L':<12}")
    print("-" * 80)
    
    for i, user in enumerate(users[:20], 1):
        addr = user["userAddress"][:10] if user["userAddress"].startswith("000000000000000000000000") else user["userAddress"][:10]
        net_usdc = float(user.get("netUsdc", "0"))
        trading_pnl = float(user.get("tradingPnl", "0"))
        total_pnl = float(user.get("totalPnl", "0"))
        
        print(f"{i:<4} {addr:<12} ${net_usdc:<11.2f} ${trading_pnl:<11.2f} ${total_pnl:<11.2f}")
    
    # Activity summary
    print(f"\n📊 7-DAY ACTIVITY SUMMARY")
    print("-" * 40)
    print(f"Markets Created: {len(data['marketData']):,}")
    print(f"Token Transfers: {len(data['tokenTransfers']):,}")
    print(f"Order Fills: {len(data['orderFills']):,}")
    print(f"Reward Claims: {len(data['rewardClaims']):,}")
    
    return {
        "total_users": total_users,
        "total_profits": total_profits,
        "total_losses": total_losses,
        "net_pnl": total_profits - total_losses,
        "leaderboard": users[:20],
        "activity": {
            "markets": len(data['marketData']),
            "transfers": len(data['tokenTransfers']),
            "orders": len(data['orderFills']),
            "claims": len(data['rewardClaims'])
        }
    }

def save_dashboard_data(dashboard_data: Dict[str, Any], filename: str = "polymarket_7day_dashboard.json"):
    """Save dashboard data to file"""
    with open(filename, 'w') as f:
        json.dump(dashboard_data, f, indent=2)
    print(f"\n💾 Dashboard data saved to: {filename}")

def main():
    """Main function"""
    print("🎯 POLYMARKET 7-DAY STREAMING DASHBOARD")
    print("=" * 60)
    print("📊 Streaming data directly (no local indexing)")
    print("🚀 Building Dune-style dashboard")
    print()
    
    # Estimate block range
    start_block, end_block = estimate_recent_blocks()
    
    print(f"\n⚠️  This will process approximately {(end_block - start_block):,} blocks")
    print("   Processing in small chunks to avoid timeouts...")
    
    response = input("\nContinue with 7-day streaming? (y/N): ").strip().lower()
    if response != 'y':
        print("❌ Cancelled by user")
        return
    
    # Stream data
    data = stream_blocks_in_chunks(start_block, end_block)
    
    if not data["userPnls"]:
        print("❌ No user data found in 7-day range")
        print("   This might be a quiet period for Polymarket activity")
        return
    
    # Create dashboard
    dashboard_data = create_dashboard(data)
    
    if dashboard_data:
        # Save data
        save_dashboard_data(dashboard_data)
        
        print(f"\n🎉 7-DAY DASHBOARD COMPLETE!")
        print("=" * 60)
        print("✅ 7 days of data streamed")
        print("✅ Dune-style dashboard created")
        print("✅ Data saved for analysis")
        print("\n💡 Ready to build web interface!")
    else:
        print("❌ Dashboard creation failed")

if __name__ == "__main__":
    main()
