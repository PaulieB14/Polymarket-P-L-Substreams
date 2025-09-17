#!/usr/bin/env python3
"""
Test chunked streaming with a small range
"""

import json
import subprocess
import sys
import time
from datetime import datetime
from typing import Dict, Any, List

def get_polygon_endpoint():
    """Get the proper Polygon endpoint from documentation"""
    return "polygon.streamingfast.io:443"

def create_chunks(start_block: int, end_block: int, chunk_size: int = 1000) -> List[tuple]:
    """Create chunks of blocks to stay under the 10,000 limit"""
    chunks = []
    current = start_block
    
    while current < end_block:
        chunk_end = min(current + chunk_size, end_block)
        chunks.append((current, chunk_end))
        current = chunk_end
    
    print(f"📦 Created {len(chunks)} chunks of ~{chunk_size:,} blocks each")
    return chunks

def stream_chunk(start_block: int, end_block: int, chunk_num: int, total_chunks: int) -> Dict[str, Any]:
    """Stream a single chunk of data"""
    print(f"\n🔄 Chunk {chunk_num}/{total_chunks}: Blocks {start_block:,} to {end_block:,}")
    
    cmd = [
        "substreams", "run",
        "-e", get_polygon_endpoint(),
        "substreams.yaml",
        "map_pure_dune_pnl",
        "-s", str(start_block),
        "-t", str(end_block),
        "--limit-processed-blocks", "10000"
    ]
    
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=120)  # 2 min per chunk
        
        if result.returncode != 0:
            print(f"❌ Chunk {chunk_num} failed: {result.stderr}")
            return None
        
        print(f"✅ Chunk {chunk_num} completed")
        
        # Extract data from output
        data = extract_data_from_output(result.stdout)
        return data
        
    except subprocess.TimeoutExpired:
        print(f"⏰ Chunk {chunk_num} timed out")
        return None
    except Exception as e:
        print(f"❌ Error in chunk {chunk_num}: {e}")
        return None

def extract_data_from_output(output: str) -> Dict[str, Any]:
    """Extract data from Substreams output"""
    all_data = {
        "userPnls": [],
        "marketData": [],
        "tokenTransfers": [],
        "orderFills": [],
        "rewardClaims": []
    }
    
    lines = output.strip().split('\n')
    
    for line in lines:
        if '"@data"' in line and '"userPnls"' in line:
            try:
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
                        
                        # Merge data
                        all_data["userPnls"].extend(data.get("userPnls", []))
                        all_data["marketData"].extend(data.get("marketData", []))
                        all_data["tokenTransfers"].extend(data.get("tokenTransfers", []))
                        all_data["orderFills"].extend(data.get("orderFills", []))
                        all_data["rewardClaims"].extend(data.get("rewardClaims", []))
                        
            except json.JSONDecodeError:
                continue
    
    return all_data

def main():
    """Main function"""
    print("🧪 TESTING CHUNKED STREAMING")
    print("=" * 50)
    print("📚 Testing with a small range first")
    print()
    
    # Test with a small range
    start_block = 70000000
    end_block = 70001000  # 1000 blocks
    
    print(f"📊 Test range: {start_block:,} to {end_block:,}")
    
    # Create chunks
    chunks = create_chunks(start_block, end_block, chunk_size=500)
    
    print(f"⚠️  This will stream {len(chunks)} chunks of ~500 blocks each")
    
    response = input(f"\nContinue with {len(chunks)}-chunk test? (y/N): ").strip().lower()
    if response != 'y':
        print("❌ Cancelled by user")
        return
    
    # Stream all chunks
    all_chunks = []
    successful_chunks = 0
    
    for i, (chunk_start, chunk_end) in enumerate(chunks, 1):
        chunk_data = stream_chunk(chunk_start, chunk_end, i, len(chunks))
        if chunk_data:
            all_chunks.append(chunk_data)
            successful_chunks += 1
        else:
            print(f"⚠️  Skipping chunk {i} due to error")
        
        # Progress update
        print(f"📊 Progress: {i}/{len(chunks)} chunks completed ({successful_chunks} successful)")
    
    if not all_chunks:
        print("❌ No chunks completed successfully")
        return
    
    print(f"\n✅ Test completed: {successful_chunks}/{len(chunks)} chunks successful")
    
    # Show results
    total_users = sum(len(chunk.get("userPnls", [])) for chunk in all_chunks)
    total_markets = sum(len(chunk.get("marketData", [])) for chunk in all_chunks)
    total_transfers = sum(len(chunk.get("tokenTransfers", [])) for chunk in all_chunks)
    
    print(f"\n📊 TEST RESULTS")
    print("-" * 30)
    print(f"👥 Total Users: {total_users}")
    print(f"📈 Total Markets: {total_markets}")
    print(f"🔄 Total Transfers: {total_transfers}")
    
    if total_users > 0:
        print(f"\n🎉 CHUNKED STREAMING WORKS!")
        print("✅ Ready for 7-day streaming")
        print("💡 Next step: Run stream_7days_chunked.py")
    else:
        print(f"\n⚠️  No user data found in test range")
        print("   This might be a quiet period")

if __name__ == "__main__":
    main()
