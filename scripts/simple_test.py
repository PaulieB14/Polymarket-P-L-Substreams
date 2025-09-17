#!/usr/bin/env python3
"""
Simple test to see what's happening with Substreams
"""

import subprocess
import sys
import time
import os

def test_substreams():
    """Test Substreams with a very simple command"""
    print("🧪 TESTING SUBSTREAMS")
    print("=" * 50)
    
    # Set the token
    os.environ['SUBSTREAMS_API_TOKEN'] = "eyJhbGciOiJLTVNFUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3OTQwNjA3ODcsImp0aSI6ImMwMWY3NDc3LWVjOTAtNGI0Ni1hYWVhLWVlYzk4ZDQ5ODE2YyIsImlhdCI6MTc1ODA2MDc4NywiaXNzIjoiZGZ1c2UuaW8iLCJzdWIiOiIwYm9qaTQ5NTUyMjg5MjIwYzVkYjciLCJ2IjoyLCJha2kiOiIzNjJiNDU5NGI1NmFkYWE0YzIxZWNhYzE3M2M4MTEyZDM3OGMyMWY1MjM1MDUzZWYwYmJkYjVlZjJkZWY2NDViIiwidWlkIjoiMGJvamk0OTU1MjI4OTIyMGM1ZGI3Iiwic3Vic3RyZWFtc19wbGFuX3RpZXIiOiJGUkVFIiwiY2ZnIjp7IlNVQlNUUkVBTVNfTUFYX1JFUVVFU1RTIjoiMiIsIlNVQlNUUkVBTVNfUEFSQUxMRUxfSk9CUyI6IjUiLCJTVUJTVFJFQU1TX1BBUkFMTEVMX1dPUktFUlMiOiI1In19.wMr3wAPfLuS8OpgEpb_D64VMPlow3D7JG3pfXhu5ptuMdJ1jBD45GXZFNlV8xitvzURV1SRN_T2-pPDj_j1ZEg"
    
    print("🔑 Token set")
    print("🌐 Testing Polygon endpoint...")
    
    # Try a very simple command first
    cmd = [
        "substreams", "run",
        "-e", "polygon.streamingfast.io:443",
        "substreams.yaml",
        "map_pure_dune_pnl",
        "-s", "879000000",
        "-t", "879000001",  # Just 1 block
        "-o", "json"
    ]
    
    print(f"⏳ Running: {' '.join(cmd)}")
    print("⏰ This will timeout after 30 seconds...")
    
    try:
        # Run with timeout
        result = subprocess.run(
            cmd, 
            capture_output=True, 
            text=True, 
            timeout=30
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
        print("⏰ Command timed out after 30 seconds")
        print("This suggests the endpoint might be slow or there's a connection issue")
        return False
    except Exception as e:
        print(f"❌ Error: {e}")
        return False

def test_endpoint_connectivity():
    """Test if we can reach the endpoint"""
    print("\n🌐 TESTING ENDPOINT CONNECTIVITY")
    print("=" * 50)
    
    try:
        # Try to ping the endpoint
        result = subprocess.run(
            ["ping", "-c", "3", "polygon.streamingfast.io"],
            capture_output=True,
            text=True,
            timeout=10
        )
        
        if result.returncode == 0:
            print("✅ Endpoint is reachable")
            return True
        else:
            print("❌ Endpoint ping failed")
            print(result.stderr)
            return False
            
    except Exception as e:
        print(f"❌ Ping error: {e}")
        return False

def main():
    """Main function"""
    print("🎯 SUBSTREAMS DIAGNOSTIC")
    print("=" * 50)
    
    # Test connectivity first
    if not test_endpoint_connectivity():
        print("\n❌ Cannot reach endpoint - check your internet connection")
        return
    
    # Test Substreams
    if test_substreams():
        print("\n🎉 SUBSTREAMS IS WORKING!")
        print("The issue might be with the specific block range or data")
    else:
        print("\n❌ SUBSTREAMS IS NOT WORKING")
        print("Possible issues:")
        print("1. Authentication token expired")
        print("2. Endpoint is slow/overloaded")
        print("3. Block range has no data")
        print("4. Network connectivity issues")

if __name__ == "__main__":
    main()
