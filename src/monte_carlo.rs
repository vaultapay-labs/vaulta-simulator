use crate::types::*;
use crate::simulator::Simulator;
use crate::strategy::Strategy;
use anyhow::Result;
use rust_decimal::Decimal;
use std::collections::HashMap;
use tracing::info;

/// Monte Carlo engine for stress testing strategies
pub struct MonteCarloEngine {
    iterations: usize,
    scenarios: usize,
    rng: rand::rngs::ThreadRng,
}

impl MonteCarloEngine {
    /// Create a new Monte Carlo engine
    pub fn new(iterations: usize, scenarios: usize) -> Self {
        Self {
            iterations,
            scenarios,
            rng: rand::thread_rng(),
        }
    }

    /// Run Monte Carlo stress test
    pub async fn run_stress_test(
        &mut self,
        confidence_level: f64,
    ) -> Result<MonteCarloResults> {
        info!("Starting Monte Carlo simulation with {} iterations", self.iterations);
        
        let mut final_values = Vec::with_capacity(self.iterations);
        
        // Run simulations in parallel batches
        let batch_size = 100;
        let batches = (self.iterations + batch_size - 1) / batch_size;
        
        for batch in 0..batches {
            let start = batch * batch_size;
            let end = (start + batch_size).min(self.iterations);
            
            let batch_results: Vec<f64> = (start..end)
                .map(|i| {
                    let result = self.run_single_simulation(i);
                    result.unwrap_or(0.0)
                })
                .collect();
            
            final_values.extend(batch_results);
            
            if (batch + 1) % 10 == 0 {
                info!("Completed {}/{} batches", batch + 1, batches);
            }
        }
        
        // Calculate statistics
        let expected_value = self.calculate_expected_value(&final_values);
        let value_at_risk = self.calculate_var(&final_values, confidence_level);
        let conditional_var = self.calculate_cvar(&final_values, confidence_level);
        let max_drawdown = self.calculate_max_drawdown_distribution(&final_values);
        
        // Calculate percentiles
        let mut sorted = final_values.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let percentiles = HashMap::from([
            (5, self.percentile(&sorted, 0.05)),
            (25, self.percentile(&sorted, 0.25)),
            (50, self.percentile(&sorted, 0.50)),
            (75, self.percentile(&sorted, 0.75)),
            (95, self.percentile(&sorted, 0.95)),
        ]);
        
        Ok(MonteCarloResults {
            iterations: self.iterations,
            expected_value,
            value_at_risk,
            conditional_var,
            max_drawdown_pct: max_drawdown,
            confidence_level,
            distribution: final_values,
            percentiles,
        })
    }

    /// Run a single simulation iteration
    fn run_single_simulation(&mut self, _seed: usize) -> Result<f64> {
        let initial_capital = 1_000_000.0;
        let strategy = Strategy::balanced();
        let mut simulator = Simulator::new(initial_capital, strategy);
        
        // Run simulation for 100 steps
        for _ in 0..100 {
            simulator.step()?;
        }
        
        let results = simulator.finalize();
        Ok(results.final_value.to_f64().unwrap_or(0.0))
    }

    fn calculate_expected_value(&self, values: &[f64]) -> Decimal {
        if values.is_empty() {
            return Decimal::ZERO;
        }
        
        let sum: f64 = values.iter().sum();
        let mean = sum / values.len() as f64;
        Decimal::try_from(mean).unwrap_or(Decimal::ZERO)
    }

    fn calculate_var(&self, values: &[f64], confidence: f64) -> Decimal {
        if values.is_empty() {
            return Decimal::ZERO;
        }
        
        let mut sorted = values.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let index = ((1.0 - confidence) * sorted.len() as f64) as usize;
        let var_value = sorted.get(index).copied().unwrap_or(0.0);
        
        Decimal::try_from(var_value).unwrap_or(Decimal::ZERO)
    }

    fn calculate_cvar(&self, values: &[f64], confidence: f64) -> Decimal {
        if values.is_empty() {
            return Decimal::ZERO;
        }
        
        let mut sorted = values.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let var_index = ((1.0 - confidence) * sorted.len() as f64) as usize;
        let tail_values: Vec<f64> = sorted[..var_index].to_vec();
        
        if tail_values.is_empty() {
            return Decimal::ZERO;
        }
        
        let avg_tail = tail_values.iter().sum::<f64>() / tail_values.len() as f64;
        Decimal::try_from(avg_tail).unwrap_or(Decimal::ZERO)
    }

    fn calculate_max_drawdown_distribution(&self, values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        
        // For simplicity, calculate average drawdown
        // In a full implementation, we'd track drawdowns per simulation
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let initial = 1_000_000.0;
        
        if initial > 0.0 {
            ((initial - mean) / initial * 100.0).max(0.0)
        } else {
            0.0
        }
    }

    fn percentile(&self, sorted: &[f64], p: f64) -> Decimal {
        if sorted.is_empty() {
            return Decimal::ZERO;
        }
        
        let index = (p * sorted.len() as f64) as usize;
        let value = sorted.get(index.min(sorted.len() - 1)).copied().unwrap_or(0.0);
        Decimal::try_from(value).unwrap_or(Decimal::ZERO)
    }
}
