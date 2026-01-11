//! # Vaulta Simulator
//!
//! High-fidelity capital routing simulator with Monte Carlo stress testing,
//! backtesting engine, and strategy optimization for Vaulta Protocol.
//!
//! ## Features
//!
//! - **Monte Carlo Simulation**: Stress test strategies with thousands of scenarios
//! - **Backtesting Engine**: Test strategies on historical market data
//! - **Strategy Optimization**: Find optimal parameters using genetic algorithms
//! - **Risk Metrics**: Calculate VaR, CVaR, Sharpe ratio, and more
//! - **Real-time Simulation**: Step-by-step capital routing simulation
//!
//! ## Example
//!
//! ```rust,no_run
//! use vaulta_simulator::{Simulator, Strategy, types::*};
//!
//! let strategy = Strategy::conservative();
//! let mut simulator = Simulator::new(1_000_000.0, strategy);
//!
//! for _ in 0..100 {
//!     simulator.step()?;
//! }
//!
//! let results = simulator.finalize();
//! println!("Final value: {}", results.final_value);
//! ```

pub mod backtest;
pub mod market;
pub mod monte_carlo;
pub mod optimizer;
pub mod portfolio;
pub mod risk;
pub mod simulator;
pub mod strategy;
pub mod types;
pub mod utils;

pub use simulator::Simulator;
pub use strategy::Strategy;
pub use types::*;
