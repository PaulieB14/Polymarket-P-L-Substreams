#!/usr/bin/env python3
"""
Stream with proper cursor handling to avoid getting stuck
Based on: https://docs.substreams.dev/reference-material/never-miss-data
"""

import subprocess
import sys
import time
import json
import os
from typing import Dict, Any, Optional

def stream_with_cursor(start_block: int, end_block: int, cursor: Optional[str] = None):
    """Stream with proper cursor handling"""
    print(f"🚀 STREAMING WITH CURSOR HANDLING")
    print("=" * 50)
    print(f"📊 Block range: {start_block:,} to {end_block:,}")
    if cursor:
        print(f"📍 Starting from cursor: {cursor}")
    else:
        print("📍 Starting from beginning")
    
    # Set up environment
    os.environ['SUBSTREAMS_API_TOKEN'] = "eyJhbGciOiJLTVNFUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3OTQwNjA3ODcsImp0aSI6ImMwMWY3NDc3LWVjOTAtNGI0Ni1hYWVhLWVlYzk4ZDQ5ODE2YyIsImlhdCI6MTc1ODA2MDc4NywiaXNzIjoiZGZ1c2UuaW8iLCJzdWIiOiIwYm9qaTQ5NTUyMjg5MjIwYzVkYjciLCJ2IjoyLCJha2kiOiIzNjJiNDU5NGI1NmFkYWE0YzIxZWNhYzE3M2M4MTEyZDM3OGMyMWY1MjM1MDUzZWYwYmJkYjVlZjJkZWY2NDViIiwidWlkIjoiMGJvamk0OTU1MjI4OTIyMGM1ZGI3Iiwic3Vic3RyZWFtc19wbGFuX3RpZXIiOiJGUkVFIiwiY2ZnIjp7IlNVQlNUUkVBTVNfTUFYX1JFUVVFU1RTIjoiMiIsIlNVQlNUUkVBTVNfUEFSQUxMRUxfSk9CUyI6IjUiLCJTVUJTVFJFQU1TX1BBUkFMTEVMX1dPUktFUlMiOiI1In19.wMr3wAPfLuS8OpgEpb_D64VMPlow3D7JG3pfXhu5ptuMdJ1jBD45GXZFNlV8xitvzURV1SRN_T2-pPDj_j1ZEg"
    
    # Build command
    cmd = [
        "substreams", "run",
        "-e", "polygon.streamingfast.io:443",
        "substreams.yaml",
        "map_pure_dune_pnl",
        "-s", str(start_block),
        "-t", str(end_block),
        "-o", "json"
    ]
    
    if cursor:
        cmd.extend(["-c", cursor])
    
    print(f"⏳ Running: {' '.join(cmd)}")
    
    try:
        # Run with timeout and proper error handling
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=120  # 2 minutes timeout
        )
        
        print(f"\n✅ Command completed!")
        print(f"Return code: {result.returncode}")
        
        if result.returncode == 0:
            print("📊 Processing output...")
            return process_output(result.stdout)
        else:
            print(f"❌ Error: {result.stderr}")
            return None
            
    except subprocess.TimeoutExpired:
        print("⏰ Command timed out - this might be normal for large ranges")
        return None
    except Exception as e:
        print(f"❌ Unexpected error: {e}")
        return None

def process_output(output: str) -> Dict[str, Any]:
    """Process the Substreams output and extract data"""
    print("📊 Processing Substreams output...")
    
    all_data = {
        "userPnls": [],
        "marketData": [],
        "tokenTransfers": [],
        "orderFills": [],
        "rewardClaims": [],
        "cursors": []
    }
    
    lines = output.strip().split('\n')
    block_count = 0
    
    for line in lines:
        if line.strip():
            try:
                data = json.loads(line)
                
                # Check if this is a progress message
                if "progress" in data:
                    print(f"📈 Progress: {data['progress'].get('total_blocks_processed', 0)} blocks processed")
                    continue
                
                # Check if this is a block scoped data message
                if "block_scoped_data" in data:
                    block_data = data["block_scoped_data"]
                    
                    # Extract cursor
                    if "cursor" in block_data:
                        all_data["cursors"].append(block_data["cursor"])
                    
                    # Extract actual data
                    if "output" in block_data and "map_output" in block_data["output"]:
                        map_output = block_data["output"]["map_output"]
                        if "value" in map_output:
                            # Decode the protobuf data
                            pnl_data = json.loads(map_output["value"])
                            
                            # Extract user PnLs
                            if "userPnls" in pnl_data:
                                all_data["userPnls"].extend(pnl_data["userPnls"])
                            
                            if "marketData" in pnl_data:
                                all_data["marketData"].extend(pnl_data["marketData"])
                            
                            if "tokenTransfers" in pnl_data:
                                all_data["tokenTransfers"].extend(pnl_data["tokenTransfers"])
                            
                            if "orderFills" in pnl_data:
                                all_data["orderFills"].extend(pnl_data["orderFills"])
                            
                            if "rewardClaims" in pnl_data:
                                all_data["rewardClaims"].extend(pnl_data["rewardClaims"])
                            
                            block_count += 1
                            print(f"📦 Processed block {block_count}")
                
            except json.JSONDecodeError:
                # Skip non-JSON lines (like progress messages)
                continue
    
    print(f"📊 Processed {block_count} blocks")
    print(f"👥 Found {len(set(user['userAddress'] for user in all_data['userPnls']))} unique users")
    
    return all_data

def test_small_range():
    """Test with a very small range first"""
    print("🧪 TESTING SMALL RANGE")
    print("=" * 50)
    
    # Test with just 1 block
    start_block = 879000000
    end_block = 879000001
    
    data = stream_with_cursor(start_block, end_block)
    
    if data and data["userPnls"]:
        print(f"\n🎉 SUCCESS!")
        print(f"Found {len(data['userPnls'])} users")
        print(f"Found {len(data['marketData'])} markets")
        print(f"Found {len(data['tokenTransfers'])} transfers")
        return True
    else:
        print(f"\n⚠️  No data found in this range")
        return False

def stream_recent_data():
    """Stream recent data with cursor handling"""
    print("🚀 STREAMING RECENT DATA")
    print("=" * 50)
    
    # Get recent block range
    current_time = int(time.time())
    current_block = current_time // 2
    start_block = current_block - 1000  # 1000 blocks ago (~33 minutes)
    end_block = current_block - 100     # 100 blocks ago (~3 minutes)
    
    print(f"📊 Recent range: {start_block:,} to {end_block:,}")
    
    data = stream_with_cursor(start_block, end_block)
    
    if data:
        print(f"\n📊 RESULTS:")
        print(f"👥 Users: {len(data['userPnls'])}")
        print(f"📈 Markets: {len(data['marketData'])}")
        print(f"🔄 Transfers: {len(data['tokenTransfers'])}")
        print(f"📋 Orders: {len(data['orderFills'])}")
        print(f"🎁 Claims: {len(data['rewardClaims'])}")
        print(f"📍 Cursors: {len(data['cursors'])}")
        
        # Save data
        with open('recent_data.json', 'w') as f:
            json.dump(data, f, indent=2)
        print(f"💾 Data saved to recent_data.json")
        
        return True
    else:
        print("❌ No data received")
        return False

def main():
    """Main function"""
    print("🎯 SUBSTREAMS CURSOR TESTING")
    print("=" * 50)
    print("📚 Based on: https://docs.substreams.dev/reference-material/never-miss-data")
    print()
    
    # Test small range first
    if test_small_range():
        print("\n✅ Small range test passed!")
        print("Trying recent data...")
        
        if stream_recent_data():
            print("\n🎉 CURSOR STREAMING WORKS!")
            print("✅ Ready for larger data streams")
        else:
            print("\n❌ Recent data streaming failed")
    else:
        print("\n❌ Small range test failed")
        print("Check your authentication and endpoint")

if __name__ == "__main__":
    main()
