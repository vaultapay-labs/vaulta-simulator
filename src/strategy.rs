use crate::types::*;
use anyhow::Result;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;
use time::OffsetDateTime;

/// Strategy trait for capital routing decisions
pub trait RoutingStrategy {
    fn generate_routing_decisions(
        &self,
        portfolio: &Portfolio,
        market_state: &HashMap<String, Decimal>,
    ) -> Result<Vec<RoutingDecision>>;
    
    fn name(&self) -> &str;
}

/// Built-in strategy implementations
#[derive(Debug, Clone)]
pub enum Strategy {
    Conservative(ConservativeStrategy),
    Balanced(BalancedStrategy),
    Aggressive(AggressiveStrategy),
    YieldMaximizer(YieldMaximizerStrategy),
    RiskParity(RiskParityStrategy),
}

impl Strategy {
    pub fn conservative() -> Self {
        Self::Conservative(ConservativeStrategy::new())
    }
    
    pub fn balanced() -> Self {
        Self::Balanced(BalancedStrategy::new())
    }
    
    pub fn aggressive() -> Self {
        Self::Aggressive(AggressiveStrategy::new())
    }
    
    pub fn yield_maximizer() -> Self {
        Self::YieldMaximizer(YieldMaximizerStrategy::new())
    }
    
    pub fn risk_parity() -> Self {
        Self::RiskParity(RiskParityStrategy::new())
    }
    
    pub fn from_name(name: &str) -> Result<Self> {
        match name.to_lowercase().as_str() {
            "conservative" => Ok(Self::conservative()),
            "balanced" => Ok(Self::balanced()),
            "aggressive" => Ok(Self::aggressive()),
            "yield_maximizer" | "yield" => Ok(Self::yield_maximizer()),
            "risk_parity" | "risk" => Ok(Self::risk_parity()),
            _ => Err(anyhow::anyhow!("Unknown strategy: {}", name)),
        }
    }
    
    pub fn list_all() -> Vec<&'static str> {
        vec!["conservative", "balanced", "aggressive", "yield_maximizer", "risk_parity"]
    }
}

impl RoutingStrategy for Strategy {
    fn generate_routing_decisions(
        &self,
        portfolio: &Portfolio,
        market_state: &HashMap<String, Decimal>,
    ) -> Result<Vec<RoutingDecision>> {
        match self {
            Self::Conservative(s) => s.generate_routing_decisions(portfolio, market_state),
            Self::Balanced(s) => s.generate_routing_decisions(portfolio, market_state),
            Self::Aggressive(s) => s.generate_routing_decisions(portfolio, market_state),
            Self::YieldMaximizer(s) => s.generate_routing_decisions(portfolio, market_state),
            Self::RiskParity(s) => s.generate_routing_decisions(portfolio, market_state),
        }
    }
    
    fn name(&self) -> &str {
        match self {
            Self::Conservative(s) => s.name(),
            Self::Balanced(s) => s.name(),
            Self::Aggressive(s) => s.name(),
            Self::YieldMaximizer(s) => s.name(),
            Self::RiskParity(s) => s.name(),
        }
    }
}

/// Conservative strategy: Low risk, stable assets
pub struct ConservativeStrategy {
    max_position_size: f64,
    min_yield: f64,
}

impl ConservativeStrategy {
    pub fn new() -> Self {
        Self {
            max_position_size: 0.15, // 15% max per position
            min_yield: 0.03, // 3% minimum yield
        }
    }
}

impl RoutingStrategy for ConservativeStrategy {
    fn generate_routing_decisions(
        &self,
        portfolio: &Portfolio,
        _market_state: &HashMap<String, Decimal>,
    ) -> Result<Vec<RoutingDecision>> {
        let mut decisions = vec![];
        
        // Only allocate if we have significant cash
        if portfolio.cash < portfolio.total_value * dec!(0.1) {
            return Ok(decisions);
        }
        
        // Conservative allocation to stable assets
        let allocation_amount = portfolio.cash * dec!(0.3); // 30% of cash
        
        if allocation_amount > dec!(1000) {
            decisions.push(RoutingDecision {
                timestamp: OffsetDateTime::now_utc(),
                source_asset: "USD".to_string(),
                target_asset: "USDC".to_string(),
                amount: allocation_amount,
                expected_yield: dec!(0.05), // 5% APY
                risk_score: 0.1,
                execution_cost: allocation_amount * dec!(0.001), // 0.1% fee
            });
        }
        
        Ok(decisions)
    }
    
    fn name(&self) -> &str {
        "conservative"
    }
}

/// Balanced strategy: Diversified allocation
pub struct BalancedStrategy {
    max_position_size: f64,
    target_positions: usize,
}

impl BalancedStrategy {
    pub fn new() -> Self {
        Self {
            max_position_size: 0.25, // 25% max per position
            target_positions: 5,
        }
    }
}

impl RoutingStrategy for BalancedStrategy {
    fn generate_routing_decisions(
        &self,
        portfolio: &Portfolio,
        market_state: &HashMap<String, Decimal>,
    ) -> Result<Vec<RoutingDecision>> {
        let mut decisions = vec![];
        
        let available_cash = portfolio.cash;
        if available_cash < dec!(1000) {
            return Ok(decisions);
        }
        
        // Allocate to multiple assets
        let allocation_per_asset = available_cash * dec!(0.2); // 20% per asset
        
        let target_assets = vec!["USDC", "ETH", "BTC", "SOL", "MATIC"];
        
        for asset in target_assets {
            if !portfolio.positions.contains_key(asset) && allocation_per_asset > dec!(500) {
                decisions.push(RoutingDecision {
                    timestamp: OffsetDateTime::now_utc(),
                    source_asset: "USD".to_string(),
                    target_asset: asset.to_string(),
                    amount: allocation_per_asset,
                    expected_yield: dec!(0.08), // 8% expected yield
                    risk_score: 0.5,
                    execution_cost: allocation_per_asset * dec!(0.002), // 0.2% fee
                });
            }
        }
        
        Ok(decisions)
    }
    
    fn name(&self) -> &str {
        "balanced"
    }
}

/// Aggressive strategy: High risk, high reward
pub struct AggressiveStrategy {
    max_position_size: f64,
    min_yield: f64,
}

impl AggressiveStrategy {
    pub fn new() -> Self {
        Self {
            max_position_size: 0.4, // 40% max per position
            min_yield: 0.15, // 15% minimum yield
        }
    }
}

impl RoutingStrategy for AggressiveStrategy {
    fn generate_routing_decisions(
        &self,
        portfolio: &Portfolio,
        _market_state: &HashMap<String, Decimal>,
    ) -> Result<Vec<RoutingDecision>> {
        let mut decisions = vec![];
        
        let available_cash = portfolio.cash;
        if available_cash < dec!(1000) {
            return Ok(decisions);
        }
        
        // Aggressive allocation to high-yield assets
        let allocation_amount = available_cash * dec!(0.6); // 60% of cash
        
        decisions.push(RoutingDecision {
            timestamp: OffsetDateTime::now_utc(),
            source_asset: "USD".to_string(),
            target_asset: "HIGH_YIELD_POOL".to_string(),
            amount: allocation_amount,
            expected_yield: dec!(0.20), // 20% APY
            risk_score: 0.8,
            execution_cost: allocation_amount * dec!(0.005), // 0.5% fee
        });
        
        Ok(decisions)
    }
    
    fn name(&self) -> &str {
        "aggressive"
    }
}

/// Yield maximizer: Always route to highest yield
pub struct YieldMaximizerStrategy {
    rebalance_threshold: f64,
}

impl YieldMaximizerStrategy {
    pub fn new() -> Self {
        Self {
            rebalance_threshold: 0.02, // 2% yield difference triggers rebalance
        }
    }
}

impl RoutingStrategy for YieldMaximizerStrategy {
    fn generate_routing_decisions(
        &self,
        portfolio: &Portfolio,
        _market_state: &HashMap<String, Decimal>,
    ) -> Result<Vec<RoutingDecision>> {
        let mut decisions = vec![];
        
        // Find highest yield opportunity
        let available_cash = portfolio.cash;
        if available_cash > dec!(1000) {
            decisions.push(RoutingDecision {
                timestamp: OffsetDateTime::now_utc(),
                source_asset: "USD".to_string(),
                target_asset: "MAX_YIELD".to_string(),
                amount: available_cash * dec!(0.9), // 90% allocation
                expected_yield: dec!(0.25), // 25% APY
                risk_score: 0.7,
                execution_cost: available_cash * dec!(0.003), // 0.3% fee
            });
        }
        
        Ok(decisions)
    }
    
    fn name(&self) -> &str {
        "yield_maximizer"
    }
}

/// Risk parity: Equal risk contribution from each position
pub struct RiskParityStrategy {
    target_volatility: f64,
}

impl RiskParityStrategy {
    pub fn new() -> Self {
        Self {
            target_volatility: 0.10, // 10% target volatility
        }
    }
}

impl RoutingStrategy for RiskParityStrategy {
    fn generate_routing_decisions(
        &self,
        portfolio: &Portfolio,
        _market_state: &HashMap<String, Decimal>,
    ) -> Result<Vec<RoutingDecision>> {
        let mut decisions = vec![];
        
        let available_cash = portfolio.cash;
        if available_cash < dec!(1000) {
            return Ok(decisions);
        }
        
        // Allocate equally across uncorrelated assets
        let assets = vec!["USDC", "ETH", "BTC", "SOL"];
        let allocation_per_asset = available_cash / Decimal::from(assets.len());
        
        for asset in assets {
            if !portfolio.positions.contains_key(asset) && allocation_per_asset > dec!(500) {
                decisions.push(RoutingDecision {
                    timestamp: OffsetDateTime::now_utc(),
                    source_asset: "USD".to_string(),
                    target_asset: asset.to_string(),
                    amount: allocation_per_asset,
                    expected_yield: dec!(0.10), // 10% expected yield
                    risk_score: 0.4,
                    execution_cost: allocation_per_asset * dec!(0.002), // 0.2% fee
                });
            }
        }
        
        Ok(decisions)
    }
    
    fn name(&self) -> &str {
        "risk_parity"
    }
}
