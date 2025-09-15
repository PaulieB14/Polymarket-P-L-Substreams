#!/usr/bin/env node

/**
 * Polymarket P&L Substreams - JavaScript Streaming Example
 * 
 * This example demonstrates how to stream arbitrage opportunities
 * from the polymarket-pnl@v0.3.1 Substreams package in real-time.
 * 
 * Usage: node stream-arbitrage.js
 */

const { createSubstreamsClient } = require('@substreams/stream');

// Configuration
const PACKAGE_NAME = 'polymarket-pnl@v0.3.1';
const MODULE_NAME = 'map_enhanced_pnl_with_neg_risk';
const START_BLOCK = 60000000;

// Colors for console output
const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  magenta: '\x1b[35m',
  cyan: '\x1b[36m'
};

class ArbitrageMonitor {
  constructor() {
    this.totalOpportunities = 0;
    this.totalUsers = 0;
    this.startTime = Date.now();
  }

  log(message, color = 'reset') {
    const timestamp = new Date().toISOString();
    console.log(`${colors[color]}[${timestamp}] ${message}${colors.reset}`);
  }

  formatArbitrage(arbitrage) {
    return {
      marketId: arbitrage.marketId,
      totalCost: arbitrage.totalNoCost,
      guaranteedPayout: arbitrage.guaranteedPayout,
      profit: arbitrage.profit,
      profitPercentage: arbitrage.profitPercentage,
      noOutcomes: arbitrage.noOutcomes,
      blockNumber: arbitrage.blockNumber
    };
  }

  formatUserPnL(user) {
    return {
      address: user.userAddress,
      totalTrades: user.totalTrades,
      totalVolume: user.totalVolume,
      totalPnl: user.totalPnl,
      arbitrageProfit: user.totalArbitrageProfit,
      riskScore: user.riskScore
    };
  }

  async startStreaming() {
    this.log('🚀 Starting Polymarket P&L Arbitrage Monitor...', 'cyan');
    this.log(`📦 Package: ${PACKAGE_NAME}`, 'blue');
    this.log(`📊 Module: ${MODULE_NAME}`, 'blue');
    this.log(`🔢 Start Block: ${START_BLOCK}`, 'blue');
    this.log('─'.repeat(80), 'yellow');

    try {
      const client = createSubstreamsClient(PACKAGE_NAME);
      const stream = client.stream(MODULE_NAME, {
        startBlock: START_BLOCK
      });

      stream.on('data', (data) => {
        this.processBlock(data);
      });

      stream.on('error', (error) => {
        this.log(`❌ Stream error: ${error.message}`, 'red');
      });

      stream.on('end', () => {
        this.log('✅ Stream ended', 'green');
        this.printSummary();
      });

    } catch (error) {
      this.log(`❌ Failed to start streaming: ${error.message}`, 'red');
      process.exit(1);
    }
  }

  processBlock(data) {
    const blockNumber = data.blockNumber;
    const timestamp = data.blockTimestamp;
    
    this.log(`\n📦 Block #${blockNumber}`, 'bright');
    this.log(`⏰ Time: ${timestamp}`, 'blue');

    // Process arbitrage opportunities
    if (data.arbitrageOpportunities && data.arbitrageOpportunities.length > 0) {
      this.log(`🎯 Found ${data.arbitrageOpportunities.length} arbitrage opportunities!`, 'green');
      
      data.arbitrageOpportunities.forEach((arbitrage, index) => {
        const formatted = this.formatArbitrage(arbitrage);
        this.log(`  💰 Opportunity #${index + 1}:`, 'yellow');
        this.log(`     Market: ${formatted.marketId}`, 'cyan');
        this.log(`     Cost: $${formatted.totalCost} → Payout: $${formatted.guaranteedPayout}`, 'cyan');
        this.log(`     Profit: $${formatted.profit} (${formatted.profitPercentage})`, 'green');
        this.log(`     NO Outcomes: ${formatted.noOutcomes.join(', ')}`, 'cyan');
        this.log(`     Block: ${formatted.blockNumber}`, 'blue');
        this.totalOpportunities++;
      });
    }

    // Process user P&L data
    if (data.userPnls && data.userPnls.length > 0) {
      this.log(`👥 Processing ${data.userPnls.length} users`, 'blue');
      
      data.userPnls.forEach((user, index) => {
        const formatted = this.formatUserPnL(user);
        if (formatted.arbitrageProfit !== '0') {
          this.log(`  🎯 User ${formatted.address.slice(0, 8)}... - Arbitrage Profit: $${formatted.arbitrageProfit}`, 'green');
        }
        this.totalUsers++;
      });
    }

    // Process global P&L
    if (data.globalPnls && data.globalPnls.length > 0) {
      const global = data.globalPnls[0];
      this.log(`📊 Global Stats: ${global.activeUsers} users, ${global.totalTrades} trades`, 'magenta');
    }

    this.log('─'.repeat(80), 'yellow');
  }

  printSummary() {
    const duration = (Date.now() - this.startTime) / 1000;
    this.log('\n📈 STREAMING SUMMARY', 'bright');
    this.log('─'.repeat(50), 'yellow');
    this.log(`⏱️  Duration: ${duration.toFixed(2)} seconds`, 'blue');
    this.log(`🎯 Arbitrage Opportunities: ${this.totalOpportunities}`, 'green');
    this.log(`👥 Total Users Processed: ${this.totalUsers}`, 'blue');
    this.log(`📦 Package: ${PACKAGE_NAME}`, 'cyan');
    this.log('─'.repeat(50), 'yellow');
  }
}

// Handle graceful shutdown
process.on('SIGINT', () => {
  console.log('\n👋 Shutting down gracefully...');
  process.exit(0);
});

// Start the monitor
const monitor = new ArbitrageMonitor();
monitor.startStreaming();
