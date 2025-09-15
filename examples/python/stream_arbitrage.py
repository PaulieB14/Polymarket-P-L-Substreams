#!/usr/bin/env python3

"""
Polymarket P&L Substreams - Python Streaming Example

This example demonstrates how to stream arbitrage opportunities
from the polymarket-pnl@v0.3.1 Substreams package in real-time.

Usage: python stream_arbitrage.py
"""

import asyncio
import json
import sys
import time
from datetime import datetime
from typing import Dict, List, Any

try:
    from substreams import SubstreamsClient
except ImportError:
    print("❌ Error: substreams package not found. Install with: pip install substreams")
    sys.exit(1)

# Configuration
PACKAGE_NAME = "polymarket-pnl@v0.3.1"
MODULE_NAME = "map_enhanced_pnl_with_neg_risk"
START_BLOCK = 60000000

# Colors for console output
class Colors:
    RESET = '\033[0m'
    BRIGHT = '\033[1m'
    RED = '\033[31m'
    GREEN = '\033[32m'
    YELLOW = '\033[33m'
    BLUE = '\033[34m'
    MAGENTA = '\033[35m'
    CYAN = '\033[36m'

class ArbitrageMonitor:
    def __init__(self):
        self.total_opportunities = 0
        self.total_users = 0
        self.start_time = time.time()

    def log(self, message: str, color: str = Colors.RESET):
        timestamp = datetime.now().isoformat()
        print(f"{color}[{timestamp}] {message}{Colors.RESET}")

    def format_arbitrage(self, arbitrage: Dict[str, Any]) -> Dict[str, Any]:
        return {
            'market_id': arbitrage.get('marketId', ''),
            'total_cost': arbitrage.get('totalNoCost', ''),
            'guaranteed_payout': arbitrage.get('guaranteedPayout', ''),
            'profit': arbitrage.get('profit', ''),
            'profit_percentage': arbitrage.get('profitPercentage', ''),
            'no_outcomes': arbitrage.get('noOutcomes', []),
            'block_number': arbitrage.get('blockNumber', 0)
        }

    def format_user_pnl(self, user: Dict[str, Any]) -> Dict[str, Any]:
        return {
            'address': user.get('userAddress', ''),
            'total_trades': user.get('totalTrades', ''),
            'total_volume': user.get('totalVolume', ''),
            'total_pnl': user.get('totalPnl', ''),
            'arbitrage_profit': user.get('totalArbitrageProfit', ''),
            'risk_score': user.get('riskScore', '')
        }

    async def start_streaming(self):
        self.log("🚀 Starting Polymarket P&L Arbitrage Monitor...", Colors.CYAN)
        self.log(f"📦 Package: {PACKAGE_NAME}", Colors.BLUE)
        self.log(f"📊 Module: {MODULE_NAME}", Colors.BLUE)
        self.log(f"🔢 Start Block: {START_BLOCK}", Colors.BLUE)
        self.log("─" * 80, Colors.YELLOW)

        try:
            # Create Substreams client
            client = SubstreamsClient(PACKAGE_NAME)
            
            # Start streaming
            stream = client.stream(MODULE_NAME, start_block=START_BLOCK)
            
            async for data in stream:
                await self.process_block(data)
                
        except Exception as error:
            self.log(f"❌ Failed to start streaming: {str(error)}", Colors.RED)
            sys.exit(1)

    async def process_block(self, data: Dict[str, Any]):
        block_number = data.get('blockNumber', 0)
        timestamp = data.get('blockTimestamp', '')
        
        self.log(f"\n📦 Block #{block_number}", Colors.BRIGHT)
        self.log(f"⏰ Time: {timestamp}", Colors.BLUE)

        # Process arbitrage opportunities
        arbitrage_opportunities = data.get('arbitrageOpportunities', [])
        if arbitrage_opportunities:
            self.log(f"🎯 Found {len(arbitrage_opportunities)} arbitrage opportunities!", Colors.GREEN)
            
            for i, arbitrage in enumerate(arbitrage_opportunities):
                formatted = self.format_arbitrage(arbitrage)
                self.log(f"  💰 Opportunity #{i + 1}:", Colors.YELLOW)
                self.log(f"     Market: {formatted['market_id']}", Colors.CYAN)
                self.log(f"     Cost: ${formatted['total_cost']} → Payout: ${formatted['guaranteed_payout']}", Colors.CYAN)
                self.log(f"     Profit: ${formatted['profit']} ({formatted['profit_percentage']})", Colors.GREEN)
                self.log(f"     NO Outcomes: {', '.join(formatted['no_outcomes'])}", Colors.CYAN)
                self.log(f"     Block: {formatted['block_number']}", Colors.BLUE)
                self.total_opportunities += 1

        # Process user P&L data
        user_pnls = data.get('userPnls', [])
        if user_pnls:
            self.log(f"👥 Processing {len(user_pnls)} users", Colors.BLUE)
            
            for user in user_pnls:
                formatted = self.format_user_pnl(user)
                if formatted['arbitrage_profit'] != '0':
                    self.log(f"  🎯 User {formatted['address'][:8]}... - Arbitrage Profit: ${formatted['arbitrage_profit']}", Colors.GREEN)
                self.total_users += 1

        # Process global P&L
        global_pnls = data.get('globalPnls', [])
        if global_pnls:
            global_data = global_pnls[0]
            active_users = global_data.get('activeUsers', '0')
            total_trades = global_data.get('totalTrades', '0')
            self.log(f"📊 Global Stats: {active_users} users, {total_trades} trades", Colors.MAGENTA)

        self.log("─" * 80, Colors.YELLOW)

    def print_summary(self):
        duration = time.time() - self.start_time
        self.log("\n📈 STREAMING SUMMARY", Colors.BRIGHT)
        self.log("─" * 50, Colors.YELLOW)
        self.log(f"⏱️  Duration: {duration:.2f} seconds", Colors.BLUE)
        self.log(f"🎯 Arbitrage Opportunities: {self.total_opportunities}", Colors.GREEN)
        self.log(f"👥 Total Users Processed: {self.total_users}", Colors.BLUE)
        self.log(f"📦 Package: {PACKAGE_NAME}", Colors.CYAN)
        self.log("─" * 50, Colors.YELLOW)

async def main():
    monitor = ArbitrageMonitor()
    
    try:
        await monitor.start_streaming()
    except KeyboardInterrupt:
        monitor.log("\n👋 Shutting down gracefully...", Colors.YELLOW)
        monitor.print_summary()
    except Exception as e:
        monitor.log(f"❌ Error: {str(e)}", Colors.RED)
        sys.exit(1)

if __name__ == "__main__":
    asyncio.run(main())
