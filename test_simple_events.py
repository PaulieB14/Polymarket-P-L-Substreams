#!/usr/bin/env python3
import json
import sys

def analyze_substreams_output():
    """Analyze raw Substreams output to understand what's happening"""
    
    try:
        # Read all input
        data = sys.stdin.read()
        print(f"📊 Raw data length: {len(data)} characters")
        
        # Try to parse as JSON
        try:
            parsed = json.loads(data)
            print(f"✅ Successfully parsed JSON")
            print(f"📋 Top-level keys: {list(parsed.keys()) if isinstance(parsed, dict) else 'Not a dict'}")
            
            if isinstance(parsed, dict) and 'data' in parsed:
                data_section = parsed['data']
                print(f"📊 Data section keys: {list(data_section.keys()) if isinstance(data_section, dict) else 'Not a dict'}")
                
                if isinstance(data_section, dict):
                    for key, value in data_section.items():
                        if isinstance(value, list):
                            print(f"   {key}: {len(value)} items")
                        elif isinstance(value, dict):
                            print(f"   {key}: dict with {len(value)} keys")
                        else:
                            print(f"   {key}: {type(value).__name__} = {value}")
            
        except json.JSONDecodeError as e:
            print(f"❌ JSON parse error: {e}")
            print(f"📄 First 500 chars: {data[:500]}")
            
    except Exception as e:
        print(f"❌ Error: {e}")

if __name__ == "__main__":
    analyze_substreams_output()
