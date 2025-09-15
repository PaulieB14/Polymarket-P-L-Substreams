#!/usr/bin/env python3
import json
import sys
from datetime import datetime

def test_wallet_activity():
    """Test Substreams with specific wallet addresses"""
    
    # Test wallets
    test_wallets = [
        "0x6596a3C7C2eA69D04F01F064AA4e914196BbA0a7",  # User's requested wallet
        "0x61ac40d8635f52359f346b33ae1ddec981cdf57d",  # Active wallet from data
        "0x67d76b8aeb25c00ab5dd51d34769107a83e92318",  # Another active wallet
    ]
    
    print("🔍 Testing Substreams with specific wallets...")
    print("=" * 60)
    
    try:
        # Read JSON from stdin
        data = json.load(sys.stdin)
        
        if 'userPnls' not in data:
            print("❌ No user P&L data found")
            return
            
        users = data['userPnls']
        print(f"📊 Total users captured: {len(users)}")
        
        # Test each wallet
        for wallet in test_wallets:
            print(f"\n🎯 Testing wallet: {wallet}")
            print("-" * 40)
            
            # Convert to the format used in data (with padding)
            padded_wallet = "000000000000000000000000" + wallet[2:].lower()
            
            found_users = [u for u in users if u.get('userAddress', '').lower() == padded_wallet]
            
            if found_users:
                for user in found_users:
                    print(f"✅ FOUND: {user['userAddress']}")
                    print(f"   💰 Net USDC: ${user.get('netUsdc', '0')}")
                    print(f"   📈 Share Value: ${user.get('shareValue', '0')}")
                    print(f"   📊 Trading P&L: ${user.get('tradingPnl', '0')}")
                    print(f"   🎁 Liquidity P&L: ${user.get('liqPnl', '0')}")
                    print(f"   💎 Total P&L: ${user.get('totalPnl', '0')}")
                    print(f"   🪙 Holdings: {len(user.get('holdings', []))} tokens")
                    print(f"   ⏰ Last Activity: {user.get('lastActivity', 'Unknown')}")
            else:
                print(f"❌ Wallet not found in current data")
                print(f"   (May not be active in these blocks)")
        
        # Show some sample data
        print(f"\n�� Sample of captured users:")
        print("-" * 40)
        for i, user in enumerate(users[:3], 1):
            addr = user.get('userAddress', 'Unknown')
            # Convert back to normal format
            if addr.startswith('000000000000000000000000'):
                addr = '0x' + addr[24:]
            print(f"#{i} {addr}")
            print(f"   P&L: ${user.get('totalPnl', '0')}")
            print(f"   Holdings: {len(user.get('holdings', []))} tokens")
        
        # Check token transfers
        if 'tokenTransfers' in data:
            transfers = data['tokenTransfers']
            print(f"\n📊 Token transfers captured: {len(transfers)}")
            
            # Look for transfers involving test wallets
            for wallet in test_wallets:
                wallet_transfers = [t for t in transfers if wallet.lower() in t.get('userAddress', '').lower()]
                if wallet_transfers:
                    print(f"✅ Found {len(wallet_transfers)} transfers for {wallet}")
                else:
                    print(f"❌ No transfers found for {wallet}")
        
    except json.JSONDecodeError as e:
        print(f"❌ Error parsing JSON: {e}")
    except Exception as e:
        print(f"❌ Error: {e}")

if __name__ == "__main__":
    test_wallet_activity()
