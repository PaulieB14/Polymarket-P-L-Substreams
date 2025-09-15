package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/streamingfast/substreams-sink"
)

// Configuration
const (
	PACKAGE_NAME = "polymarket-pnl@v0.3.1"
	MODULE_NAME  = "map_enhanced_pnl_with_neg_risk"
	START_BLOCK  = 60000000
)

// ArbitrageMonitor handles streaming and processing of arbitrage opportunities
type ArbitrageMonitor struct {
	totalOpportunities int
	totalUsers         int
	startTime          time.Time
}

// NewArbitrageMonitor creates a new monitor instance
func NewArbitrageMonitor() *ArbitrageMonitor {
	return &ArbitrageMonitor{
		startTime: time.Now(),
	}
}

// log prints a formatted log message with timestamp
func (m *ArbitrageMonitor) log(message string) {
	timestamp := time.Now().Format("2006-01-02T15:04:05Z")
	fmt.Printf("[%s] %s\n", timestamp, message)
}

// formatArbitrage formats arbitrage opportunity data
func (m *ArbitrageMonitor) formatArbitrage(arbitrage map[string]interface{}) {
	marketId := arbitrage["marketId"]
	totalCost := arbitrage["totalNoCost"]
	guaranteedPayout := arbitrage["guaranteedPayout"]
	profit := arbitrage["profit"]
	profitPercentage := arbitrage["profitPercentage"]
	noOutcomes := arbitrage["noOutcomes"]
	blockNumber := arbitrage["blockNumber"]

	fmt.Printf("  💰 Arbitrage Opportunity:\n")
	fmt.Printf("     Market: %v\n", marketId)
	fmt.Printf("     Cost: $%v → Payout: $%v\n", totalCost, guaranteedPayout)
	fmt.Printf("     Profit: $%v (%v)\n", profit, profitPercentage)
	fmt.Printf("     NO Outcomes: %v\n", noOutcomes)
	fmt.Printf("     Block: %v\n", blockNumber)
}

// formatUserPnL formats user P&L data
func (m *ArbitrageMonitor) formatUserPnL(user map[string]interface{}) {
	address := user["userAddress"]
	totalTrades := user["totalTrades"]
	totalVolume := user["totalVolume"]
	totalPnl := user["totalPnl"]
	arbitrageProfit := user["totalArbitrageProfit"]
	riskScore := user["riskScore"]

	if arbitrageProfit != "0" {
		fmt.Printf("  🎯 User %v... - Arbitrage Profit: $%v\n", 
			fmt.Sprintf("%v", address)[:8], arbitrageProfit)
	}
}

// processBlock processes a single block of data
func (m *ArbitrageMonitor) processBlock(data map[string]interface{}) {
	blockNumber := data["blockNumber"]
	timestamp := data["blockTimestamp"]

	fmt.Printf("\n📦 Block #%v\n", blockNumber)
	fmt.Printf("⏰ Time: %v\n", timestamp)

	// Process arbitrage opportunities
	if arbitrageOpportunities, ok := data["arbitrageOpportunities"].([]interface{}); ok && len(arbitrageOpportunities) > 0 {
		fmt.Printf("🎯 Found %d arbitrage opportunities!\n", len(arbitrageOpportunities))
		
		for i, arbitrage := range arbitrageOpportunities {
			if arbMap, ok := arbitrage.(map[string]interface{}); ok {
				fmt.Printf("  💰 Opportunity #%d:\n", i+1)
				m.formatArbitrage(arbMap)
				m.totalOpportunities++
			}
		}
	}

	// Process user P&L data
	if userPnls, ok := data["userPnls"].([]interface{}); ok && len(userPnls) > 0 {
		fmt.Printf("👥 Processing %d users\n", len(userPnls))
		
		for _, user := range userPnls {
			if userMap, ok := user.(map[string]interface{}); ok {
				m.formatUserPnL(userMap)
				m.totalUsers++
			}
		}
	}

	// Process global P&L
	if globalPnls, ok := data["globalPnls"].([]interface{}); ok && len(globalPnls) > 0 {
		if global, ok := globalPnls[0].(map[string]interface{}); ok {
			activeUsers := global["activeUsers"]
			totalTrades := global["totalTrades"]
			fmt.Printf("📊 Global Stats: %v users, %v trades\n", activeUsers, totalTrades)
		}
	}

	fmt.Println("─" + string(make([]byte, 80)) + "─")
}

// printSummary prints the final summary
func (m *ArbitrageMonitor) printSummary() {
	duration := time.Since(m.startTime).Seconds()
	fmt.Printf("\n📈 STREAMING SUMMARY\n")
	fmt.Printf("─" + string(make([]byte, 50)) + "─\n")
	fmt.Printf("⏱️  Duration: %.2f seconds\n", duration)
	fmt.Printf("🎯 Arbitrage Opportunities: %d\n", m.totalOpportunities)
	fmt.Printf("👥 Total Users Processed: %d\n", m.totalUsers)
	fmt.Printf("📦 Package: %s\n", PACKAGE_NAME)
	fmt.Printf("─" + string(make([]byte, 50)) + "─\n")
}

// startStreaming begins the streaming process
func (m *ArbitrageMonitor) startStreaming() {
	m.log("🚀 Starting Polymarket P&L Arbitrage Monitor...")
	fmt.Printf("📦 Package: %s\n", PACKAGE_NAME)
	fmt.Printf("📊 Module: %s\n", MODULE_NAME)
	fmt.Printf("🔢 Start Block: %d\n", START_BLOCK)
	fmt.Println("─" + string(make([]byte, 80)) + "─")

	// Create context with cancellation
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Handle graceful shutdown
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)
	go func() {
		<-sigChan
		m.log("👋 Shutting down gracefully...")
		cancel()
	}()

	// Create Substreams client
	client := substreams.NewClient(PACKAGE_NAME)
	
	// Create stream options
	options := &substreams.StreamOptions{
		StartBlock: START_BLOCK,
	}

	// Start streaming
	stream := client.Stream(MODULE_NAME, options)
	
	for {
		select {
		case data, ok := <-stream:
			if !ok {
				m.log("✅ Stream ended")
				m.printSummary()
				return
			}
			m.processBlock(data)
			
		case <-ctx.Done():
			m.log("👋 Shutting down gracefully...")
			m.printSummary()
			return
		}
	}
}

func main() {
	monitor := NewArbitrageMonitor()
	monitor.startStreaming()
}
