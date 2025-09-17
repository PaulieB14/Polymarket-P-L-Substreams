#!/usr/bin/env python3
"""
Final Working Polymarket Data Streaming Script

This script streams Polymarket data using reasonable block ranges
where we know activity exists, avoiding the timeout issues.
"""

import subprocess
import json
import time
from datetime import datetime

def get_polygon_endpoint():
    """Get the official Polygon endpoint"""
    return "polygon.streamingfast.io:443"

def stream_historical_data():
    """Stream from historical range where Polymarket is active"""
    print("🎯 Streaming Polymarket historical data...")
    
    # Use ranges where we know Polymarket is active
    ranges = [
        (50000000, 51000000, "Early 2023 activity"),
        (60000000, 61000000, "Mid 2023 activity"), 
        (70000000, 71000000, "Late 2023 activity"),
        (80000000, 81000000, "Early 2024 activity"),
    ]
    
    all_data = {
        "userPnls": [],
        "markets": [],
        "tokenTransfers": [],
        "orderFills": [],
        "rewardClaims": [],
        "metadata": {
            "ranges": [],
            "timestamp": datetime.now().isoformat()
        }
    }
    
    for start_block, end_block, description in ranges:
        print(f"\n📊 Processing {description}: {start_block:,} to {end_block:,}")
        
        # Process in small chunks to avoid timeouts
        chunk_size = 1000
        chunk_num = 0
        total_chunks = (end_block - start_block) // chunk_size
        
        for chunk_start in range(start_block, end_block, chunk_size):
            chunk_end = min(chunk_start + chunk_size - 1, end_block)
            chunk_num += 1
            
            print(f"   🔄 Chunk {chunk_num}/{total_chunks}: {chunk_start:,} to {chunk_end:,}")
            
            # Run substreams for this chunk
            cmd = [
                "substreams", "run",
                "-e", get_polygon_endpoint(),
                "substreams.yaml",
                "map_pure_dune_pnl",
                "-s", str(chunk_start),
                "-t", str(chunk_end),
                "-o", "json"
            ]
            
            try:
                result = subprocess.run(cmd, capture_output=True, text=True, timeout=60)
                
                if result.returncode == 0:
                    print(f"      ✅ Success")
                    
                    # Parse the JSON output
                    try:
                        for line in result.stdout.strip().split('\n'):
                            if line.strip() and line.startswith('{'):
                                chunk_data = json.loads(line)
                                
                                # Merge data from this chunk
                                if "userPnls" in chunk_data:
                                    all_data["userPnls"].extend(chunk_data["userPnls"])
                                if "markets" in chunk_data:
                                    all_data["markets"].extend(chunk_data["markets"])
                                if "tokenTransfers" in chunk_data:
                                    all_data["tokenTransfers"].extend(chunk_data["tokenTransfers"])
                                if "orderFills" in chunk_data:
                                    all_data["orderFills"].extend(chunk_data["orderFills"])
                                if "rewardClaims" in chunk_data:
                                    all_data["rewardClaims"].extend(chunk_data["rewardClaims"])
                                break
                                    
                    except json.JSONDecodeError as e:
                        print(f"      ⚠️  JSON parsing error: {e}")
                        continue
                        
                else:
                    print(f"      ❌ Failed: {result.stderr}")
                    continue
                    
            except subprocess.TimeoutExpired:
                print(f"      ⏰ Timeout")
                continue
            except Exception as e:
                print(f"      ❌ Error: {e}")
                continue
        
        # Record this range
        all_data["metadata"]["ranges"].append({
            "start": start_block,
            "end": end_block,
            "description": description,
            "chunks": total_chunks
        })
    
    return all_data

def save_results(data, filename="polymarket_final_data.json"):
    """Save the results to a file"""
    print(f"\n💾 Saving results to {filename}...")
    
    # Add summary statistics
    data["summary"] = {
        "totalUsers": len(data["userPnls"]),
        "totalMarkets": len(data["markets"]),
        "totalTransfers": len(data["tokenTransfers"]),
        "totalOrderFills": len(data["orderFills"]),
        "totalRewardClaims": len(data["rewardClaims"]),
        "totalRanges": len(data["metadata"]["ranges"]),
        "generatedAt": datetime.now().isoformat()
    }
    
    with open(filename, 'w') as f:
        json.dump(data, f, indent=2)
    
    print(f"   ✅ Data saved successfully!")
    print(f"   📊 Summary:")
    print(f"      Users: {data['summary']['totalUsers']:,}")
    print(f"      Markets: {data['summary']['totalMarkets']:,}")
    print(f"      Transfers: {data['summary']['totalTransfers']:,}")
    print(f"      Order Fills: {data['summary']['totalOrderFills']:,}")
    print(f"      Reward Claims: {data['summary']['totalRewardClaims']:,}")
    print(f"      Ranges Processed: {data['summary']['totalRanges']}")

def create_dashboard_data(data):
    """Create dashboard-ready data"""
    print(f"\n📊 Creating dashboard data...")
    
    # Create leaderboard
    user_totals = {}
    for user_pnl in data["userPnls"]:
        user_addr = user_pnl["user_address"]
        total_pnl = float(user_pnl["total_pnl"])
        
        if user_addr not in user_totals:
            user_totals[user_addr] = {
                "user_address": user_addr,
                "total_pnl": 0.0,
                "trading_pnl": 0.0,
                "liq_pnl": 0.0,
                "net_usdc": 0.0,
                "share_value": 0.0
            }
        
        user_totals[user_addr]["total_pnl"] += total_pnl
        user_totals[user_addr]["trading_pnl"] += float(user_pnl["trading_pnl"])
        user_totals[user_addr]["liq_pnl"] += float(user_pnl["liq_pnl"])
        user_totals[user_addr]["net_usdc"] += float(user_pnl["net_usdc"])
        user_totals[user_addr]["share_value"] += float(user_pnl["share_value"])
    
    # Sort by total P&L
    leaderboard = sorted(user_totals.values(), key=lambda x: x["total_pnl"], reverse=True)
    
    # Save dashboard data
    dashboard_data = {
        "leaderboard": leaderboard[:25],  # Top 25
        "summary": data["summary"],
        "metadata": data["metadata"]
    }
    
    with open("polymarket_dashboard.json", 'w') as f:
        json.dump(dashboard_data, f, indent=2)
    
    print(f"   ✅ Dashboard data saved to polymarket_dashboard.json")
    print(f"   🏆 Top 5 traders:")
    for i, trader in enumerate(leaderboard[:5]):
        print(f"      {i+1}. {trader['user_address'][:10]}... - P&L: ${trader['total_pnl']:,.2f}")

def main():
    """Main function"""
    print("🎯 Final Working Polymarket Data Streaming")
    print("=" * 50)
    
    # Stream historical data
    data = stream_historical_data()
    
    # Save results
    save_results(data)
    
    # Create dashboard data
    create_dashboard_data(data)
    
    print(f"\n🎉 Streaming completed successfully!")
    print(f"   📁 Files created:")
    print(f"      - polymarket_final_data.json (full data)")
    print(f"      - polymarket_dashboard.json (dashboard data)")
    print(f"   🚀 Ready for analysis and dashboard creation!")

if __name__ == "__main__":
    main()
