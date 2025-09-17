#!/usr/bin/env python3
"""
Test with more recent blocks to see if we can get any data
"""

import subprocess
import sys
import time
import os
import json

def get_current_block():
    """Estimate current block number"""
    current_time = int(time.time())
    # Polygon produces ~1 block every 2 seconds
    current_block = current_time // 2
    return current_block

def test_recent_blocks():
    """Test with very recent blocks"""
    print("🧪 TESTING RECENT BLOCKS")
    print("=" * 50)
    
    # Set the token
    os.environ['SUBSTREAMS_API_TOKEN'] = "eyJhbGciOiJLTVNFUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3OTQwNjA3ODcsImp0aSI6ImMwMWY3NDc3LWVjOTAtNGI0Ni1hYWVhLWVlYzk4ZDQ5ODE2YyIsImlhdCI6MTc1ODA2MDc4NywiaXNzIjoiZGZ1c2UuaW8iLCJzdWIiOiIwYm9qaTQ5NTUyMjg5MjIwYzVkYjciLCJ2IjoyLCJha2kiOiIzNjJiNDU5NGI1NmFkYWE0YzIxZWNhYzE3M2M4MTEyZDM3OGMyMWY1MjM1MDUzZWYwYmJkYjVlZjJkZWY2NDViIiwidWlkIjoiMGJvamk0OTU1MjI4OTIyMGM1ZGI3Iiwic3Vic3RyZWFtc19wbGFuX3RpZXIiOiJGUkVFIiwiY2ZnIjp7IlNVQlNUUkVBTVNfTUFYX1JFUVVFU1RTIjoiMiIsIlNVQlNUUkVBTVNfUEFSQUxMRUxfSk9CUyI6IjUiLCJTVUJTVFJFQU1TX1BBUkFMTEVMX1dPUktFUlMiOiI1In19.wMr3wAPfLuS8OpgEpb_D64VMPlow3D7JG3pfXhu5ptuMdJ1jBD45GXZFNlV8xitvzURV1SRN_T2-pPDj_j1ZEg"
    
    current_block = get_current_block()
    start_block = current_block - 100  # 100 blocks ago (~3 minutes)
    end_block = current_block - 50     # 50 blocks ago (~1.5 minutes)
    
    print(f"📊 Testing blocks: {start_block:,} to {end_block:,}")
    print(f"⏰ This is very recent data (last few minutes)")
    
    # Try the command
    cmd = [
        "substreams", "run",
        "-e", "polygon.streamingfast.io:443",
        "substreams.yaml",
        "map_pure_dune_pnl",
        "-s", str(start_block),
        "-t", str(end_block),
        "-o", "json"
    ]
    
    print(f"⏳ Running: {' '.join(cmd)}")
    print("⏰ This will timeout after 60 seconds...")
    
    try:
        result = subprocess.run(
            cmd, 
            capture_output=True, 
            text=True, 
            timeout=60
        )
        
        print(f"\n✅ Command completed!")
        print(f"Return code: {result.returncode}")
        print(f"STDOUT length: {len(result.stdout)}")
        print(f"STDERR length: {len(result.stderr)}")
        
        if result.stdout:
            print(f"\n📊 OUTPUT:")
            # Try to parse as JSON
            try:
                lines = result.stdout.strip().split('\n')
                for line in lines:
                    if line.strip():
                        data = json.loads(line)
                        print(f"Block {data.get('@block', {}).get('number', 'unknown')}: {len(data.get('@data', {}).get('userPnls', []))} users")
            except json.JSONDecodeError:
                print("Raw output (first 1000 chars):")
                print(result.stdout[:1000])
        
        if result.stderr:
            print(f"\n❌ ERRORS:")
            print(result.stderr)
            
        return result.returncode == 0
        
    except subprocess.TimeoutExpired:
        print("⏰ Command timed out after 60 seconds")
        print("This suggests the endpoint is very slow or overloaded")
        return False
    except Exception as e:
        print(f"❌ Error: {e}")
        return False

def test_alternative_endpoint():
    """Try a different approach - maybe the issue is with the specific endpoint"""
    print("\n🔄 TRYING ALTERNATIVE APPROACH")
    print("=" * 50)
    
    # Try using the package directly from registry
    cmd = [
        "substreams", "run",
        "-e", "polygon.streamingfast.io:443",
        "polymarket-dune-pure@v0.4.0",
        "map_pure_dune_pnl",
        "-s", "879000000",
        "-t", "879000001",
        "-o", "json"
    ]
    
    print(f"⏳ Trying package from registry: {' '.join(cmd)}")
    
    try:
        result = subprocess.run(
            cmd, 
            capture_output=True, 
            text=True, 
            timeout=30
        )
        
        print(f"Return code: {result.returncode}")
        if result.stdout:
            print("✅ Got output from registry package!")
            print(result.stdout[:500])
            return True
        else:
            print("❌ No output from registry package")
            if result.stderr:
                print(result.stderr)
            return False
            
    except subprocess.TimeoutExpired:
        print("⏰ Registry package also timed out")
        return False
    except Exception as e:
        print(f"❌ Registry error: {e}")
        return False

def main():
    """Main function"""
    print("🎯 RECENT BLOCKS TEST")
    print("=" * 50)
    
    # Test recent blocks first
    if test_recent_blocks():
        print("\n🎉 RECENT BLOCKS WORK!")
        print("The issue was with the old block range")
    else:
        print("\n❌ RECENT BLOCKS ALSO FAILED")
        print("Trying alternative approach...")
        
        if test_alternative_endpoint():
            print("\n🎉 ALTERNATIVE APPROACH WORKS!")
        else:
            print("\n❌ ALL APPROACHES FAILED")
            print("Possible solutions:")
            print("1. Check if your JWT token is still valid")
            print("2. Try a different time of day (endpoint might be overloaded)")
            print("3. Contact Substreams support")
            print("4. Try a different endpoint or approach")

if __name__ == "__main__":
    main()
