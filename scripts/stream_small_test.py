#!/usr/bin/env python3
"""
Small Test Script for Polymarket Data Streaming

This script tests with a very small range to verify everything works.
"""

import subprocess
import json
import time
from datetime import datetime

def get_polygon_endpoint():
    """Get the official Polygon endpoint"""
    return "polygon.streamingfast.io:443"

def test_small_range():
    """Test with a very small range that we know works"""
    print("🧪 Testing with small range (blocks 50000000-50000050)...")
    
    cmd = [
        "substreams", "run",
        "-e", get_polygon_endpoint(),
        "substreams.yaml",
        "map_pure_dune_pnl",
        "-s", "50000000",
        "-t", "50000050",
        "-o", "json"
    ]
    
    print(f"Running: {' '.join(cmd)}")
    
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=60)
        
        if result.returncode == 0:
            print("✅ Success!")
            print("Output preview:")
            print(result.stdout[:500] + "..." if len(result.stdout) > 500 else result.stdout)
            
            # Try to parse JSON
            try:
                for line in result.stdout.strip().split('\n'):
                    if line.strip() and line.startswith('{'):
                        data = json.loads(line)
                        print(f"📊 Data structure:")
                        for key, value in data.items():
                            if isinstance(value, list):
                                print(f"   {key}: {len(value)} items")
                            else:
                                print(f"   {key}: {value}")
                        break
            except json.JSONDecodeError as e:
                print(f"⚠️  JSON parsing error: {e}")
                
        else:
            print("❌ Failed!")
            print(f"Error: {result.stderr}")
            
    except subprocess.TimeoutExpired:
        print("⏰ Timeout!")
    except Exception as e:
        print(f"❌ Error: {e}")

def test_recent_small_range():
    """Test with a recent small range"""
    print("\n🧪 Testing with recent small range (blocks 879000000-879000010)...")
    
    cmd = [
        "substreams", "run",
        "-e", get_polygon_endpoint(),
        "substreams.yaml",
        "map_pure_dune_pnl",
        "-s", "879000000",
        "-t", "879000010",
        "-o", "json"
    ]
    
    print(f"Running: {' '.join(cmd)}")
    
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=60)
        
        if result.returncode == 0:
            print("✅ Success!")
            print("Output preview:")
            print(result.stdout[:500] + "..." if len(result.stdout) > 500 else result.stdout)
        else:
            print("❌ Failed!")
            print(f"Error: {result.stderr}")
            
    except subprocess.TimeoutExpired:
        print("⏰ Timeout!")
    except Exception as e:
        print(f"❌ Error: {e}")

def main():
    """Main function"""
    print("🎯 Polymarket Small Range Test")
    print("=" * 40)
    
    # Test with historical range that we know works
    test_small_range()
    
    # Test with recent range
    test_recent_small_range()
    
    print("\n🎉 Testing completed!")

if __name__ == "__main__":
    main()
