#!/usr/bin/env python3
"""
Reasonable Range Polymarket Data Streaming

This script streams data from a reasonable range where we know Polymarket is active.
"""

import subprocess
import json
import time
from datetime import datetime

def get_polygon_endpoint():
    """Get the official Polygon endpoint"""
    return "polygon.streamingfast.io:443"

def stream_reasonable_range():
    """Stream from a range where we know Polymarket is active"""
    print("🎯 Streaming Polymarket data from active range...")
    
    # Use a range where we know Polymarket is active (around block 50M-60M)
    start_block = 50000000
    end_block = 60000000
    chunk_size = 1000  # 1000 blocks per chunk
    
    print(f"📊 Range: {start_block:,} to {end_block:,}")
    print(f"📦 Chunk size: {chunk_size:,} blocks")
    print(f"🔢 Total chunks: {(end_block - start_block) // chunk_size}")
    
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
    total_chunks = (end_block - start_block) // chunk_size
    
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
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=120)
            
            if result.returncode == 0:
                print(f"   ✅ Chunk {chunk_num} completed")
                
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
                    print(f"   ⚠️  JSON parsing error: {e}")
                    continue
                    
            else:
                print(f"   ❌ Chunk {chunk_num} failed: {result.stderr}")
                continue
                
        except subprocess.TimeoutExpired:
            print(f"   ⏰ Chunk {chunk_num} timed out")
            continue
        except Exception as e:
            print(f"   ❌ Chunk {chunk_num} error: {e}")
            continue
    
    return all_data

def save_results(data, filename="polymarket_reasonable_data.json"):
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
    print("🎯 Polymarket Reasonable Range Streaming")
    print("=" * 50)
    
    # Stream data from reasonable range
    data = stream_reasonable_range()
    
    # Save results
    save_results(data)
    
    print(f"\n🎉 Streaming completed!")
    print(f"   Data saved to: polymarket_reasonable_data.json")
    print(f"   Ready for analysis!")

if __name__ == "__main__":
    main()
