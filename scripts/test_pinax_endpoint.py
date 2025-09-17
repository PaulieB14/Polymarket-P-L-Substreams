#!/usr/bin/env python3
"""
Test Pinax endpoint instead of StreamingFast
Based on: https://docs.substreams.dev/reference-material/chains-and-endpoints
"""

import subprocess
import sys
import time
import json
import os

def test_pinax_endpoint():
    """Test Pinax Polygon endpoint"""
    print("🧪 TESTING PINAX ENDPOINT")
    print("=" * 50)
    print("🌐 Using Pinax Polygon endpoint: polygon.substreams.pinax.network:443")
    
    # Set the token
    os.environ['SUBSTREAMS_API_TOKEN'] = "eyJhbGciOiJLTVNFUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3OTQwNjA3ODcsImp0aSI6ImMwMWY3NDc3LWVjOTAtNGI0Ni1hYWVhLWVlYzk4ZDQ5ODE2YyIsImlhdCI6MTc1ODA2MDc4NywiaXNzIjoiZGZ1c2UuaW8iLCJzdWIiOiIwYm9qaTQ5NTUyMjg5MjIwYzVkYjciLCJ2IjoyLCJha2kiOiIzNjJiNDU5NGI1NmFkYWE0YzIxZWNhYzE3M2M4MTEyZDM3OGMyMWY1MjM1MDUzZWYwYmJkYjVlZjJkZWY2NDViIiwidWlkIjoiMGJvamk0OTU1MjI4OTIyMGM1ZGI3Iiwic3Vic3RyZWFtc19wbGFuX3RpZXIiOiJGUkVFIiwiY2ZnIjp7IlNVQlNUUkVBTVNfTUFYX1JFUVVFU1RTIjoiMiIsIlNVQlNUUkVBTVNfUEFSQUxMRUxfSk9CUyI6IjUiLCJTVUJTVFJFQU1TX1BBUkFMTEVMX1dPUktFUlMiOiI1In19.wMr3wAPfLuS8OpgEpb_D64VMPlow3D7JG3pfXhu5ptuMdJ1jBD45GXZFNlV8xitvzURV1SRN_T2-pPDj_j1ZEg"
    
    # Get current block
    current_time = int(time.time())
    current_block = current_time // 2
    start_block = current_block - 100  # 100 blocks ago
    
    print(f"📊 Testing blocks: {start_block:,} to {start_block + 10:,}")
    
    # Try Pinax endpoint
    cmd = [
        "substreams", "run",
        "-e", "polygon.substreams.pinax.network:443",
        "substreams.yaml",
        "map_pure_dune_pnl",
        "-s", str(start_block),
        "-t", str(start_block + 10),
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
            print(f"\n📊 OUTPUT (first 500 chars):")
            print(result.stdout[:500])
        
        if result.stderr:
            print(f"\n❌ ERRORS:")
            print(result.stderr)
            
        return result.returncode == 0
        
    except subprocess.TimeoutExpired:
        print("⏰ Command timed out after 60 seconds")
        return False
    except Exception as e:
        print(f"❌ Error: {e}")
        return False

def test_different_approach():
    """Try a completely different approach - maybe the issue is with our package"""
    print("\n🔄 TRYING DIFFERENT APPROACH")
    print("=" * 50)
    
    # Try using a simple test with a known working package
    cmd = [
        "substreams", "run",
        "-e", "polygon.substreams.pinax.network:443",
        "https://github.com/streamingfast/substreams-ethereum/releases/download/v0.4.0/substreams-ethereum-v0.4.0.spkg",
        "map_transfers",
        "-s", "879000000",
        "-t", "879000001",
        "-o", "json"
    ]
    
    print(f"⏳ Testing with known working package: {' '.join(cmd)}")
    
    try:
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=30
        )
        
        print(f"Return code: {result.returncode}")
        if result.stdout:
            print("✅ Got output from known package!")
            print(result.stdout[:200])
            return True
        else:
            print("❌ No output from known package")
            if result.stderr:
                print(result.stderr)
            return False
            
    except subprocess.TimeoutExpired:
        print("⏰ Known package also timed out")
        return False
    except Exception as e:
        print(f"❌ Known package error: {e}")
        return False

def main():
    """Main function"""
    print("🎯 ENDPOINT TESTING")
    print("=" * 50)
    print("📚 Testing different endpoints and approaches")
    print()
    
    # Test Pinax endpoint
    if test_pinax_endpoint():
        print("\n🎉 PINAX ENDPOINT WORKS!")
        print("The issue was with the StreamingFast endpoint")
    else:
        print("\n❌ PINAX ENDPOINT ALSO FAILED")
        print("Trying different approach...")
        
        if test_different_approach():
            print("\n🎉 DIFFERENT APPROACH WORKS!")
            print("The issue might be with our specific package")
        else:
            print("\n❌ ALL APPROACHES FAILED")
            print("Possible issues:")
            print("1. Authentication token expired")
            print("2. Network connectivity issues")
            print("3. All endpoints are overloaded")
            print("4. Our package has issues")

if __name__ == "__main__":
    main()
