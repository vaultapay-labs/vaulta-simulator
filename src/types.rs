use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;

/// Represents a financial asset in the simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub symbol: String,
    pub name: String,
    pub asset_type: AssetType,
    pub current_price: Decimal,
    pub volatility: Decimal,
    pub yield_rate: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    Crypto,
    DeFiPool,
    RWABond,
    RWACredit,
    Stablecoin,
    Other,
}

/// Represents a position in a portfolio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub asset: Asset,
    pub quantity: Decimal,
    pub entry_price: Decimal,
    pub current_value: Decimal,
}

impl Position {
    pub fn new(asset: Asset, quantity: Decimal, entry_price: Decimal) -> Self {
        let current_value = quantity * asset.current_price;
        Self {
            asset,
            quantity,
            entry_price,
            current_value,
        }
    }

    pub fn update_price(&mut self, new_price: Decimal) {
        self.asset.current_price = new_price;
        self.current_value = self.quantity * new_price;
    }

    pub fn unrealized_pnl(&self) -> Decimal {
        (self.asset.current_price - self.entry_price) * self.quantity
    }

    pub fn unrealized_pnl_pct(&self) -> Decimal {
        if self.entry_price > Decimal::ZERO {
            (self.asset.current_price - self.entry_price) / self.entry_price * Decimal::from(100)
        } else {
            Decimal::ZERO
        }
    }
}

/// Portfolio state at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub positions: HashMap<String, Position>,
    pub cash: Decimal,
    pub total_value: Decimal,
    pub timestamp: OffsetDateTime,
}

impl Portfolio {
    pub fn new(initial_cash: Decimal) -> Self {
        Self {
            positions: HashMap::new(),
            cash: initial_cash,
            total_value: initial_cash,
            timestamp: OffsetDateTime::now_utc(),
        }
    }

    pub fn add_position(&mut self, position: Position) {
        let symbol = position.asset.symbol.clone();
        self.cash -= position.current_value;
        self.positions.insert(symbol, position);
        self.update_total_value();
    }

    pub fn remove_position(&mut self, symbol: &str) -> Option<Position> {
        if let Some(position) = self.positions.remove(symbol) {
            self.cash += position.current_value;
            self.update_total_value();
            Some(position)
        } else {
            None
        }
    }

    pub fn update_total_value(&mut self) {
        let positions_value: Decimal = self
            .positions
            .values()
            .map(|p| p.current_value)
            .sum();
        self.total_value = self.cash + positions_value;
    }

    pub fn update_prices(&mut self, price_updates: &HashMap<String, Decimal>) {
        for (symbol, new_price) in price_updates {
            if let Some(position) = self.positions.get_mut(symbol) {
                position.update_price(*new_price);
            }
        }
        self.update_total_value();
    }
}

/// Simulation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResults {
    pub initial_value: Decimal,
    pub final_value: Decimal,
    pub total_return: Decimal,
    pub total_return_pct: f64,
    pub sharpe_ratio: f64,
    pub max_drawdown_pct: f64,
    pub volatility_pct: f64,
    pub value_at_risk: Decimal,
    pub conditional_var: Decimal,
    pub portfolio_history: Vec<PortfolioSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioSnapshot {
    pub timestamp: OffsetDateTime,
    pub total_value: Decimal,
    pub cash: Decimal,
    pub positions_value: Decimal,
    pub positions_count: usize,
}

/// Monte Carlo simulation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonteCarloResults {
    pub iterations: usize,
    pub expected_value: Decimal,
    pub value_at_risk: Decimal,
    pub conditional_var: Decimal,
    pub max_drawdown_pct: f64,
    pub confidence_level: f64,
    pub distribution: Vec<f64>,
    pub percentiles: HashMap<u8, Decimal>,
}

/// Backtest results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestResults {
    pub start_date: OffsetDateTime,
    pub end_date: OffsetDateTime,
    pub initial_value: Decimal,
    pub final_value: Decimal,
    pub total_return_pct: f64,
    pub annualized_return_pct: f64,
    pub volatility_pct: f64,
    pub sharpe_ratio: f64,
    pub max_drawdown_pct: f64,
    pub win_rate: f64,
    pub profit_factor: f64,
    pub trades: Vec<Trade>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub entry_time: OffsetDateTime,
    pub exit_time: Option<OffsetDateTime>,
    pub asset: String,
    pub quantity: Decimal,
    pub entry_price: Decimal,
    pub exit_price: Option<Decimal>,
    pub pnl: Option<Decimal>,
    pub pnl_pct: Option<f64>,
}

/// Market data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    pub timestamp: OffsetDateTime,
    pub symbol: String,
    pub price: Decimal,
    pub volume: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub open: Decimal,
    pub close: Decimal,
}

/// Risk parameters for a strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskParameters {
    pub max_position_size_pct: f64,
    pub max_leverage: f64,
    pub stop_loss_pct: f64,
    pub take_profit_pct: f64,
    pub max_drawdown_pct: f64,
    pub correlation_limit: f64,
}

impl Default for RiskParameters {
    fn default() -> Self {
        Self {
            max_position_size_pct: 20.0,
            max_leverage: 1.0,
            stop_loss_pct: 5.0,
            take_profit_pct: 10.0,
            max_drawdown_pct: 15.0,
            correlation_limit: 0.7,
        }
    }
}

/// Capital routing decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingDecision {
    pub timestamp: OffsetDateTime,
    pub source_asset: String,
    pub target_asset: String,
    pub amount: Decimal,
    pub expected_yield: Decimal,
    pub risk_score: f64,
    pub execution_cost: Decimal,
}

/// Strategy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfig {
    pub name: String,
    pub risk_parameters: RiskParameters,
    pub rebalance_frequency_days: u32,
    pub min_yield_threshold: f64,
    pub max_slippage_pct: f64,
    pub preferred_asset_types: Vec<AssetType>,
}
