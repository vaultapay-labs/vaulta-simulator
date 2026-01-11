use crate::types::*;
use anyhow::Result;
use rust_decimal::Decimal;
use std::collections::HashMap;

/// Market data provider interface
pub trait MarketDataProvider {
    fn get_current_price(&self, symbol: &str) -> Result<Decimal>;
    fn get_historical_prices(&self, symbol: &str, days: usize) -> Result<Vec<Decimal>>;
    fn get_volatility(&self, symbol: &str) -> Result<Decimal>;
    fn get_yield_rate(&self, symbol: &str) -> Result<Decimal>;
}

/// Mock market data provider for testing
pub struct MockMarketDataProvider {
    prices: HashMap<String, Decimal>,
    volatilities: HashMap<String, Decimal>,
    yields: HashMap<String, Decimal>,
}

impl MockMarketDataProvider {
    pub fn new() -> Self {
        let mut prices = HashMap::new();
        let mut volatilities = HashMap::new();
        let mut yields = HashMap::new();
        
        // Initialize with mock data
        prices.insert("USDC".to_string(), Decimal::from(1));
        prices.insert("ETH".to_string(), Decimal::from(2000));
        prices.insert("BTC".to_string(), Decimal::from(40000));
        prices.insert("SOL".to_string(), Decimal::from(100));
        
        volatilities.insert("USDC".to_string(), Decimal::try_from(0.001).unwrap());
        volatilities.insert("ETH".to_string(), Decimal::try_from(0.05).unwrap());
        volatilities.insert("BTC".to_string(), Decimal::try_from(0.04).unwrap());
        volatilities.insert("SOL".to_string(), Decimal::try_from(0.06).unwrap());
        
        yields.insert("USDC".to_string(), Decimal::try_from(0.05).unwrap());
        yields.insert("ETH".to_string(), Decimal::try_from(0.08).unwrap());
        yields.insert("BTC".to_string(), Decimal::try_from(0.06).unwrap());
        yields.insert("SOL".to_string(), Decimal::try_from(0.10).unwrap());
        
        Self {
            prices,
            volatilities,
            yields,
        }
    }
}

impl MarketDataProvider for MockMarketDataProvider {
    fn get_current_price(&self, symbol: &str) -> Result<Decimal> {
        self.prices
            .get(symbol)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("Price not found for {}", symbol))
    }

    fn get_historical_prices(&self, symbol: &str, days: usize) -> Result<Vec<Decimal>> {
        let base_price = self.get_current_price(symbol)?;
        let mut prices = vec![base_price];
        
        // Generate historical prices with random walk
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for _ in 1..days {
            let change_val = rng.gen_range(-0.02..0.02);
            let change = Decimal::try_from(change_val).unwrap_or(Decimal::ZERO);
            let new_price = prices.last().unwrap() * (Decimal::ONE + change);
            prices.push(new_price);
        }
        
        prices.reverse(); // Oldest first
        Ok(prices)
    }

    fn get_volatility(&self, symbol: &str) -> Result<Decimal> {
        self.volatilities
            .get(symbol)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("Volatility not found for {}", symbol))
    }

    fn get_yield_rate(&self, symbol: &str) -> Result<Decimal> {
        self.yields
            .get(symbol)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("Yield not found for {}", symbol))
    }
}
