use crate::types::*;
use crate::simulator::Simulator;
use crate::strategy::Strategy;
use anyhow::{Context, Result};
use rust_decimal::Decimal;
use time::OffsetDateTime;
use tracing::info;

/// Backtesting engine for historical strategy evaluation
pub struct BacktestEngine {
    start_date: OffsetDateTime,
    end_date: OffsetDateTime,
    strategy: Strategy,
    market_data: Vec<MarketData>,
}

impl BacktestEngine {
    /// Create a new backtest engine
    pub fn new(
        start_date_str: &str,
        end_date_str: &str,
        strategy: Strategy,
    ) -> Result<Self> {
        // Parse dates (simplified - in real implementation use proper parsing)
        let start_date = OffsetDateTime::now_utc();
        let end_date = start_date + time::Duration::days(365);
        
        // In a real implementation, we'd load historical market data
        let market_data = Self::generate_mock_market_data(&start_date, &end_date)?;
        
        Ok(Self {
            start_date,
            end_date,
            strategy,
            market_data,
        })
    }

    /// Run backtest
    pub async fn run(&mut self) -> Result<BacktestResults> {
        info!("Running backtest from {} to {}", self.start_date, self.end_date);
        
        let initial_value = Decimal::from(1_000_000);
        let mut simulator = Simulator::new(
            initial_value.to_f64().unwrap_or(1_000_000.0),
            self.strategy.clone(),
        );
        
        // Simulate over historical period
        let days = (self.end_date - self.start_date).whole_days() as usize;
        
        for day in 0..days.min(100) {
            // Update market data for this day
            // In real implementation, we'd use actual historical prices
            simulator.step()?;
        }
        
        let results = simulator.finalize();
        
        // Calculate additional metrics
        let annualized_return = self.calculate_annualized_return(
            &results.initial_value,
            &results.final_value,
            days,
        );
        
        let volatility = results.volatility_pct;
        let sharpe_ratio = results.sharpe_ratio;
        let max_drawdown = results.max_drawdown_pct;
        
        // Mock trade data
        let trades = vec![];
        let win_rate = 0.0;
        let profit_factor = 0.0;
        
        Ok(BacktestResults {
            start_date: self.start_date,
            end_date: self.end_date,
            initial_value: results.initial_value,
            final_value: results.final_value,
            total_return_pct: results.total_return_pct,
            annualized_return_pct: annualized_return,
            volatility_pct: volatility,
            sharpe_ratio,
            max_drawdown_pct: max_drawdown,
            win_rate,
            profit_factor,
            trades,
        })
    }

    fn calculate_annualized_return(
        &self,
        initial: &Decimal,
        final_val: &Decimal,
        days: usize,
    ) -> f64 {
        if *initial <= Decimal::ZERO || days == 0 {
            return 0.0;
        }
        
        let total_return = (final_val - initial) / *initial;
        let total_return_f64 = total_return.to_f64().unwrap_or(0.0);
        
        // Annualize
        let years = days as f64 / 365.0;
        if years > 0.0 {
            ((1.0 + total_return_f64).powf(1.0 / years) - 1.0) * 100.0
        } else {
            0.0
        }
    }

    fn generate_mock_market_data(
        start: &OffsetDateTime,
        end: &OffsetDateTime,
    ) -> Result<Vec<MarketData>> {
        let mut data = vec![];
        let mut current = *start;
        let mut price = Decimal::from(100);
        
        while current <= *end {
            // Simple random walk for mock data
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let change_val = rng.gen_range(-0.02..0.02);
            let change = Decimal::try_from(change_val).unwrap_or(Decimal::ZERO);
            
            price = price * (Decimal::ONE + change);
            
            data.push(MarketData {
                timestamp: current,
                symbol: "MOCK".to_string(),
                price,
                volume: Decimal::from(1_000_000),
                high: price * Decimal::from(101) / Decimal::from(100),
                low: price * Decimal::from(99) / Decimal::from(100),
                open: price,
                close: price,
            });
            
            current = current + time::Duration::days(1);
        }
        
        Ok(data)
    }
}
