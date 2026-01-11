use crate::types::*;
use rust_decimal::Decimal;

/// Portfolio analysis and optimization utilities
pub struct PortfolioAnalyzer;

impl PortfolioAnalyzer {
    /// Calculate portfolio diversification score (0-1)
    pub fn diversification_score(portfolio: &Portfolio) -> f64 {
        if portfolio.positions.is_empty() {
            return 0.0;
        }
        
        let position_count = portfolio.positions.len();
        let max_score = 1.0 - (1.0 / position_count as f64);
        
        // Check position size distribution
        let position_sizes: Vec<f64> = portfolio.positions
            .values()
            .map(|p| p.current_value.to_f64().unwrap_or(0.0))
            .collect();
        
        let total_positions_value: f64 = position_sizes.iter().sum();
        if total_positions_value == 0.0 {
            return 0.0;
        }
        
        // Calculate Herfindahl index (lower is more diversified)
        let herfindahl: f64 = position_sizes
            .iter()
            .map(|&size| {
                let share = size / total_positions_value;
                share * share
            })
            .sum();
        
        // Convert to diversification score (inverse of Herfindahl)
        1.0 - herfindahl
    }
    
    /// Calculate portfolio yield
    pub fn portfolio_yield(portfolio: &Portfolio) -> Decimal {
        if portfolio.positions.is_empty() {
            return Decimal::ZERO;
        }
        
        let total_value = portfolio.total_value;
        if total_value <= Decimal::ZERO {
            return Decimal::ZERO;
        }
        
        let weighted_yield: Decimal = portfolio.positions
            .values()
            .map(|p| {
                let weight = p.current_value / total_value;
                weight * p.asset.yield_rate
            })
            .sum();
        
        weighted_yield
    }
    
    /// Calculate portfolio risk (weighted volatility)
    pub fn portfolio_risk(portfolio: &Portfolio) -> Decimal {
        if portfolio.positions.is_empty() {
            return Decimal::ZERO;
        }
        
        let total_value = portfolio.total_value;
        if total_value <= Decimal::ZERO {
            return Decimal::ZERO;
        }
        
        // Simplified: weighted average volatility
        // In full implementation, we'd calculate correlation-adjusted risk
        let weighted_volatility: Decimal = portfolio.positions
            .values()
            .map(|p| {
                let weight = p.current_value / total_value;
                weight * p.asset.volatility
            })
            .sum();
        
        weighted_volatility
    }
}
