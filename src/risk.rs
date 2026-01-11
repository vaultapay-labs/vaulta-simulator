use crate::types::*;
use rust_decimal::Decimal;

/// Risk calculation utilities
pub struct RiskCalculator;

impl RiskCalculator {
    /// Calculate Value at Risk (VaR) for a portfolio
    pub fn value_at_risk(
        portfolio: &Portfolio,
        confidence: f64,
        time_horizon_days: usize,
    ) -> Decimal {
        // Simplified VaR calculation
        // In full implementation, we'd use historical simulation or parametric methods
        let portfolio_risk = portfolio.total_value * Decimal::try_from(0.05).unwrap();
        
        // Adjust for time horizon
        let time_factor = (time_horizon_days as f64 / 252.0).sqrt();
        portfolio_risk * Decimal::try_from(time_factor).unwrap_or(Decimal::ONE)
    }
    
    /// Calculate Conditional VaR (Expected Shortfall)
    pub fn conditional_var(
        portfolio: &Portfolio,
        confidence: f64,
        time_horizon_days: usize,
    ) -> Decimal {
        // CVaR is typically 1.2-1.5x VaR
        let var = Self::value_at_risk(portfolio, confidence, time_horizon_days);
        var * Decimal::try_from(1.3).unwrap()
    }
    
    /// Calculate maximum drawdown from portfolio history
    pub fn max_drawdown(history: &[PortfolioSnapshot]) -> f64 {
        if history.len() < 2 {
            return 0.0;
        }
        
        let values: Vec<f64> = history
            .iter()
            .map(|s| s.total_value.to_f64().unwrap_or(0.0))
            .collect();
        
        let mut peak = values[0];
        let mut max_dd = 0.0;
        
        for &value in &values {
            if value > peak {
                peak = value;
            }
            let drawdown = (peak - value) / peak * 100.0;
            if drawdown > max_dd {
                max_dd = drawdown;
            }
        }
        
        max_dd
    }
    
    /// Calculate Sharpe ratio
    pub fn sharpe_ratio(returns: &[f64], risk_free_rate: f64) -> f64 {
        if returns.is_empty() {
            return 0.0;
        }
        
        let mean_return = returns.iter().sum::<f64>() / returns.len() as f64;
        let excess_return = mean_return - risk_free_rate;
        
        let variance = returns.iter()
            .map(|r| (r - mean_return).powi(2))
            .sum::<f64>() / returns.len() as f64;
        let std_dev = variance.sqrt();
        
        if std_dev > 0.0 {
            excess_return / std_dev * (252.0_f64).sqrt() // Annualized
        } else {
            0.0
        }
    }
}
