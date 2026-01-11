use crate::types::*;
use anyhow::{Context, Result};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;
use time::OffsetDateTime;

/// Main simulator engine for capital routing
pub struct Simulator {
    portfolio: Portfolio,
    strategy: crate::strategy::Strategy,
    step_count: usize,
    portfolio_history: Vec<PortfolioSnapshot>,
    market_state: HashMap<String, Decimal>,
}

impl Simulator {
    /// Create a new simulator with initial capital and strategy
    pub fn new(initial_capital: f64, strategy: crate::strategy::Strategy) -> Self {
        let portfolio = Portfolio::new(
            Decimal::try_from(initial_capital).unwrap_or(Decimal::ZERO)
        );
        
        Self {
            portfolio,
            strategy,
            step_count: 0,
            portfolio_history: vec![],
            market_state: HashMap::new(),
        }
    }

    /// Execute one simulation step
    pub fn step(&mut self) -> Result<()> {
        self.step_count += 1;
        
        // Update market prices (simulated)
        self.update_market_prices()?;
        
        // Get routing decisions from strategy
        let decisions = self.strategy.generate_routing_decisions(
            &self.portfolio,
            &self.market_state,
        )?;
        
        // Execute routing decisions
        for decision in decisions {
            self.execute_routing(decision)?;
        }
        
        // Update portfolio value
        self.portfolio.update_total_value();
        
        // Record snapshot
        self.record_snapshot();
        
        Ok(())
    }

    /// Update market prices based on volatility and random walk
    fn update_market_prices(&mut self) -> Result<()> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for (symbol, position) in &mut self.portfolio.positions {
            let current_price = position.asset.current_price;
            let volatility = position.asset.volatility;
            
            // Geometric Brownian Motion for price evolution
            let dt = 1.0 / 365.0; // Daily time step
            let drift = position.asset.yield_rate;
            let random_shock = rng.gen::<f64>() - 0.5; // Random walk component
            
            let dt_decimal = Decimal::try_from(dt).unwrap_or(Decimal::ZERO);
            let drift_term = drift * dt_decimal;
            let shock_term = Decimal::try_from(random_shock * dt.sqrt()).unwrap_or(Decimal::ZERO) * volatility;
            let price_change = drift_term + shock_term;
            
            let new_price = current_price * (Decimal::ONE + price_change);
            position.update_price(new_price);
            
            self.market_state.insert(symbol.clone(), new_price);
        }
        
        Ok(())
    }

    /// Execute a capital routing decision
    fn execute_routing(&mut self, decision: RoutingDecision) -> Result<()> {
        // Check if we have enough capital
        if decision.amount > self.portfolio.cash {
            return Err(anyhow::anyhow!("Insufficient cash for routing decision"));
        }
        
        // Check if target asset exists in portfolio
        if let Some(position) = self.portfolio.positions.get_mut(&decision.target_asset) {
            // Add to existing position
            let additional_quantity = decision.amount / position.asset.current_price;
            position.quantity += additional_quantity;
            position.current_value += decision.amount;
        } else {
            // Create new position
            // In a real implementation, we'd fetch asset data from market
            let asset = Asset {
                symbol: decision.target_asset.clone(),
                name: format!("Asset {}", decision.target_asset),
                asset_type: crate::types::AssetType::Crypto,
                current_price: self.market_state
                    .get(&decision.target_asset)
                    .copied()
                    .unwrap_or(dec!(1.0)),
                volatility: dec!(0.02),
                yield_rate: decision.expected_yield,
            };
            
            let quantity = decision.amount / asset.current_price;
            let position = Position::new(asset, quantity, asset.current_price);
            self.portfolio.add_position(position);
        }
        
        // Deduct execution cost
        self.portfolio.cash -= decision.execution_cost;
        
        Ok(())
    }

    /// Record current portfolio state
    fn record_snapshot(&mut self) {
        let positions_value: Decimal = self
            .portfolio
            .positions
            .values()
            .map(|p| p.current_value)
            .sum();
        
        let snapshot = PortfolioSnapshot {
            timestamp: OffsetDateTime::now_utc(),
            total_value: self.portfolio.total_value,
            cash: self.portfolio.cash,
            positions_value,
            positions_count: self.portfolio.positions.len(),
        };
        
        self.portfolio_history.push(snapshot);
    }

    /// Get current portfolio value
    pub fn portfolio_value(&self) -> f64 {
        self.portfolio.total_value.to_f64().unwrap_or(0.0)
    }

    /// Finalize simulation and return results
    pub fn finalize(mut self) -> SimulationResults {
        self.portfolio.update_total_value();
        
        let initial_value = self.portfolio_history
            .first()
            .map(|s| s.total_value)
            .unwrap_or(Decimal::ZERO);
        let final_value = self.portfolio.total_value;
        
        let total_return = final_value - initial_value;
        let total_return_pct = if initial_value > Decimal::ZERO {
            (total_return / initial_value * Decimal::from(100)).to_f64().unwrap_or(0.0)
        } else {
            0.0
        };
        
        // Calculate Sharpe ratio
        let sharpe_ratio = self.calculate_sharpe_ratio();
        
        // Calculate max drawdown
        let max_drawdown_pct = self.calculate_max_drawdown();
        
        // Calculate volatility
        let volatility_pct = self.calculate_volatility();
        
        // Calculate VaR (simplified)
        let value_at_risk = self.calculate_var(0.95);
        let conditional_var = self.calculate_cvar(0.95);
        
        SimulationResults {
            initial_value,
            final_value,
            total_return,
            total_return_pct,
            sharpe_ratio,
            max_drawdown_pct,
            volatility_pct,
            value_at_risk,
            conditional_var,
            portfolio_history: self.portfolio_history,
        }
    }

    fn calculate_sharpe_ratio(&self) -> f64 {
        if self.portfolio_history.len() < 2 {
            return 0.0;
        }
        
        let returns: Vec<f64> = self.portfolio_history
            .windows(2)
            .map(|w| {
                let prev = w[0].total_value.to_f64().unwrap_or(0.0);
                let curr = w[1].total_value.to_f64().unwrap_or(0.0);
                if prev > 0.0 {
                    (curr - prev) / prev
                } else {
                    0.0
                }
            })
            .collect();
        
        if returns.is_empty() {
            return 0.0;
        }
        
        let mean = returns.iter().sum::<f64>() / returns.len() as f64;
        let variance = returns.iter()
            .map(|r| (r - mean).powi(2))
            .sum::<f64>() / returns.len() as f64;
        let std_dev = variance.sqrt();
        
        if std_dev > 0.0 {
            mean / std_dev * (252.0_f64).sqrt() // Annualized Sharpe
        } else {
            0.0
        }
    }

    fn calculate_max_drawdown(&self) -> f64 {
        if self.portfolio_history.is_empty() {
            return 0.0;
        }
        
        let values: Vec<f64> = self.portfolio_history
            .iter()
            .map(|s| s.total_value.to_f64().unwrap_or(0.0))
            .collect();
        
        let mut max_value = values[0];
        let mut max_drawdown = 0.0;
        
        for &value in &values {
            if value > max_value {
                max_value = value;
            }
            let drawdown = (max_value - value) / max_value * 100.0;
            if drawdown > max_drawdown {
                max_drawdown = drawdown;
            }
        }
        
        max_drawdown
    }

    fn calculate_volatility(&self) -> f64 {
        if self.portfolio_history.len() < 2 {
            return 0.0;
        }
        
        let returns: Vec<f64> = self.portfolio_history
            .windows(2)
            .map(|w| {
                let prev = w[0].total_value.to_f64().unwrap_or(0.0);
                let curr = w[1].total_value.to_f64().unwrap_or(0.0);
                if prev > 0.0 {
                    (curr - prev) / prev
                } else {
                    0.0
                }
            })
            .collect();
        
        if returns.is_empty() {
            return 0.0;
        }
        
        let mean = returns.iter().sum::<f64>() / returns.len() as f64;
        let variance = returns.iter()
            .map(|r| (r - mean).powi(2))
            .sum::<f64>() / returns.len() as f64;
        
        variance.sqrt() * (252.0_f64).sqrt() * 100.0 // Annualized volatility in %
    }

    fn calculate_var(&self, confidence: f64) -> Decimal {
        if self.portfolio_history.is_empty() {
            return Decimal::ZERO;
        }
        
        let returns: Vec<f64> = self.portfolio_history
            .windows(2)
            .map(|w| {
                let prev = w[0].total_value.to_f64().unwrap_or(0.0);
                let curr = w[1].total_value.to_f64().unwrap_or(0.0);
                if prev > 0.0 {
                    (curr - prev) / prev
                } else {
                    0.0
                }
            })
            .collect();
        
        if returns.is_empty() {
            return Decimal::ZERO;
        }
        
        let mut sorted_returns = returns.clone();
        sorted_returns.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let index = ((1.0 - confidence) * sorted_returns.len() as f64) as usize;
        let var_return = sorted_returns.get(index).copied().unwrap_or(0.0);
        
        let current_value = self.portfolio.total_value;
        current_value * Decimal::from_f64_retain(var_return.abs()).unwrap()
    }

    fn calculate_cvar(&self, confidence: f64) -> Decimal {
        if self.portfolio_history.is_empty() {
            return Decimal::ZERO;
        }
        
        let returns: Vec<f64> = self.portfolio_history
            .windows(2)
            .map(|w| {
                let prev = w[0].total_value.to_f64().unwrap_or(0.0);
                let curr = w[1].total_value.to_f64().unwrap_or(0.0);
                if prev > 0.0 {
                    (curr - prev) / prev
                } else {
                    0.0
                }
            })
            .collect();
        
        if returns.is_empty() {
            return Decimal::ZERO;
        }
        
        let mut sorted_returns = returns.clone();
        sorted_returns.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let var_index = ((1.0 - confidence) * sorted_returns.len() as f64) as usize;
        let tail_returns: Vec<f64> = sorted_returns[..var_index].to_vec();
        
        if tail_returns.is_empty() {
            return Decimal::ZERO;
        }
        
        let avg_tail_loss = tail_returns.iter().sum::<f64>() / tail_returns.len() as f64;
        let current_value = self.portfolio.total_value;
        current_value * Decimal::from_f64_retain(avg_tail_loss.abs()).unwrap()
    }
}
