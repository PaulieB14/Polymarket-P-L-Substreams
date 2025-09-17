#!/usr/bin/env python3
"""
Working 7-Day Polymarket Data Streaming Script

This script streams 7 days of Polymarket data using proper block ranges
to avoid the block limit issues we encountered before.
"""

import subprocess
import json
import time
from datetime import datetime, timedelta

def get_polygon_endpoint():
    """Get the official Polygon endpoint"""
    return "polygon.streamingfast.io:443"

def get_current_block():
    """Get current block number (approximate)"""
    # This is a rough estimate - in production you'd want to get this from an RPC
    # For now, we'll use a recent block number
    return 879000000

def calculate_7_days_ago_block():
    """Calculate block number from 7 days ago"""
    # Polygon produces ~2 blocks per second
    # 7 days = 7 * 24 * 60 * 60 = 604,800 seconds
    # 604,800 * 2 = 1,209,600 blocks
    blocks_per_day = 2 * 60 * 60 * 24  # 172,800 blocks per day
    blocks_7_days = blocks_per_day * 7  # 1,209,600 blocks
    
    current_block = get_current_block()
    start_block = current_block - blocks_7_days
    
    print(f"📊 7-day range calculation:")
    print(f"   Current block (approx): {current_block:,}")
    print(f"   7 days ago block: {start_block:,}")
    print(f"   Total blocks to process: {blocks_7_days:,}")
    
    return start_block, current_block

def stream_data_in_chunks(start_block, end_block, chunk_size=1000):
    """Stream data in manageable chunks"""
    print(f"\n🚀 Starting 7-day data streaming...")
    print(f"   Range: {start_block:,} to {end_block:,}")
    print(f"   Chunk size: {chunk_size:,} blocks")
    
    all_data = {
        "userPnls": [],
        "markets": [],
        "tokenTransfers": [],
        "orderFills": [],
        "rewardClaims": [],
        "metadata": {
            "startBlock": start_block,
            "endBlock": end_block,
            "totalBlocks": end_block - start_block,
            "timestamp": datetime.now().isoformat()
        }
    }
    
    chunk_num = 0
    total_chunks = ((end_block - start_block) // chunk_size) + 1
    
    for chunk_start in range(start_block, end_block, chunk_size):
        chunk_end = min(chunk_start + chunk_size - 1, end_block)
        chunk_num += 1
        
        print(f"\n🔄 Chunk {chunk_num}/{total_chunks}: Blocks {chunk_start:,} to {chunk_end:,}")
        
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
            print(f"   Running: {' '.join(cmd)}")
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=300)
            
            if result.returncode == 0:
                print(f"   ✅ Chunk {chunk_num} completed successfully")
                
                # Parse the JSON output
                try:
                    # The output might contain multiple JSON objects, so we need to parse each line
                    for line in result.stdout.strip().split('\n'):
                        if line.strip() and line.startswith('{'):
                            try:
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
                                    
                            except json.JSONDecodeError:
                                continue
                                
                except Exception as e:
                    print(f"   ⚠️  Error parsing chunk data: {e}")
                    continue
                    
            else:
                print(f"   ❌ Chunk {chunk_num} failed:")
                print(f"      Error: {result.stderr}")
                continue
                
        except subprocess.TimeoutExpired:
            print(f"   ⏰ Chunk {chunk_num} timed out")
            continue
        except Exception as e:
            print(f"   ❌ Chunk {chunk_num} error: {e}")
            continue
    
    return all_data

def save_results(data, filename="polymarket_7days_data.json"):
    """Save the results to a file"""
    print(f"\n💾 Saving results to {filename}...")
    
    # Add summary statistics
    data["summary"] = {
        "totalUsers": len(data["userPnls"]),
        "totalMarkets": len(data["markets"]),
        "totalTransfers": len(data["tokenTransfers"]),
        "totalOrderFills": len(data["orderFills"]),
        "totalRewardClaims": len(data["rewardClaims"]),
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

def main():
    """Main function"""
    print("🎯 Polymarket 7-Day Data Streaming")
    print("=" * 50)
    
    # Calculate 7-day range
    start_block, end_block = calculate_7_days_ago_block()
    
    # Stream data in chunks
    data = stream_data_in_chunks(start_block, end_block, chunk_size=1000)
    
    # Save results
    save_results(data)
    
    print(f"\n🎉 7-day streaming completed!")
    print(f"   Data saved to: polymarket_7days_data.json")
    print(f"   Ready for dashboard creation!")

if __name__ == "__main__":
    main()
