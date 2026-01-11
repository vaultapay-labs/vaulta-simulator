# Vaulta Simulator

<div align="center">

**High-fidelity capital routing simulator with Monte Carlo stress testing, backtesting engine, and strategy optimization for Vaulta Protocol**

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

</div>

---

## 🚀 Overview

**Vaulta Simulator** is a comprehensive simulation engine designed to test capital routing strategies for the Vaulta Protocol without risking real funds. It provides:

- **Monte Carlo Simulation**: Stress test strategies with thousands of scenarios
- **Backtesting Engine**: Test strategies on historical market data
- **Strategy Optimization**: Find optimal parameters using evolutionary algorithms
- **Risk Metrics**: Calculate VaR, CVaR, Sharpe ratio, max drawdown, and more
- **Real-time Simulation**: Step-by-step capital routing simulation

## ✨ Features

### Core Capabilities

- **Multi-Strategy Support**: Conservative, Balanced, Aggressive, Yield Maximizer, Risk Parity
- **Portfolio Management**: Track positions, cash, and total value over time
- **Market Simulation**: Realistic price evolution using Geometric Brownian Motion
- **Risk Analysis**: Comprehensive risk metrics and portfolio analytics
- **Parallel Processing**: Efficient Monte Carlo simulations using Rayon
- **Historical Backtesting**: Test strategies against historical market conditions

### Risk Metrics

- Value at Risk (VaR)
- Conditional Value at Risk (CVaR)
- Sharpe Ratio
- Maximum Drawdown
- Volatility (annualized)
- Portfolio Diversification Score

## 📦 Installation

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/vaultapay/vaulta-simulator.git
cd vaulta-simulator

# Build the project
make build

# Or use cargo directly
cargo build --release
```

### Install as Cargo Binary

```bash
make install
# Or
cargo install --path .
```

## 🎯 Quick Start

### Basic Simulation

Run a simple simulation with default parameters:

```bash
vaulta-simulator simulate --capital 1000000 --steps 100 --strategy balanced
```

### Monte Carlo Stress Test

Run Monte Carlo analysis with 10,000 iterations:

```bash
vaulta-simulator monte-carlo --iterations 10000 --scenarios 100 --confidence 0.95
```

### Backtesting

Test a strategy on historical data:

```bash
vaulta-simulator backtest \
  --start-date 2024-01-01 \
  --end-date 2024-12-31 \
  --strategy aggressive
```

### List Available Strategies

```bash
vaulta-simulator strategies
```

## 📚 Usage

### Using the Library

Add to your `Cargo.toml`:

```toml
[dependencies]
vaulta-simulator = { path = "../vaulta-simulator" }
```

### Example: Basic Simulation

```rust
use vaulta_simulator::{Simulator, Strategy};

// Create a simulator with $1M initial capital and balanced strategy
let strategy = Strategy::balanced();
let mut simulator = Simulator::new(1_000_000.0, strategy);

// Run 100 simulation steps
for step in 0..100 {
    simulator.step()?;
    if step % 10 == 0 {
        println!("Step {}: Portfolio value = {:.2}", 
                 step, simulator.portfolio_value());
    }
}

// Get final results
let results = simulator.finalize();
println!("Final value: ${:.2}", results.final_value);
println!("Total return: {:.2}%", results.total_return_pct);
println!("Sharpe ratio: {:.4}", results.sharpe_ratio);
```

### Example: Monte Carlo Analysis

```rust
use vaulta_simulator::monte_carlo::MonteCarloEngine;

let mut engine = MonteCarloEngine::new(10_000, 100);
let results = engine.run_stress_test(0.95).await?;

println!("Expected value: ${:.2}", results.expected_value);
println!("VaR (95%): ${:.2}", results.value_at_risk);
println!("CVaR: ${:.2}", results.conditional_var);
```

### Example: Backtesting

```rust
use vaulta_simulator::backtest::BacktestEngine;
use vaulta_simulator::Strategy;

let strategy = Strategy::conservative();
let mut engine = BacktestEngine::new(
    "2024-01-01",
    "2024-12-31",
    strategy
)?;

let results = engine.run().await?;
println!("Annualized return: {:.2}%", results.annualized_return_pct);
println!("Sharpe ratio: {:.4}", results.sharpe_ratio);
```

## 🏗️ Architecture

### Core Components

```
vaulta-simulator/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library exports
│   ├── simulator.rs         # Main simulation engine
│   ├── strategy.rs          # Strategy implementations
│   ├── monte_carlo.rs       # Monte Carlo engine
│   ├── backtest.rs          # Backtesting engine
│   ├── portfolio.rs         # Portfolio management
│   ├── risk.rs              # Risk calculations
│   ├── market.rs            # Market data providers
│   ├── optimizer.rs         # Strategy optimization
│   ├── types.rs             # Core data structures
│   └── utils.rs             # Utility functions
├── Cargo.toml
├── Makefile
└── README.md
```

### Key Types

- **`Simulator`**: Main simulation engine
- **`Strategy`**: Capital routing strategy implementations
- **`Portfolio`**: Portfolio state and management
- **`MonteCarloEngine`**: Monte Carlo stress testing
- **`BacktestEngine`**: Historical backtesting
- **`RiskCalculator`**: Risk metric calculations

## 🎨 Available Strategies

### Conservative
- **Risk Level**: Low
- **Max Position Size**: 15%
- **Min Yield**: 3% APY
- **Best For**: Capital preservation, stable returns

### Balanced
- **Risk Level**: Medium
- **Max Position Size**: 25%
- **Target Positions**: 5
- **Best For**: Diversified growth

### Aggressive
- **Risk Level**: High
- **Max Position Size**: 40%
- **Min Yield**: 15% APY
- **Best For**: Maximum returns, high risk tolerance

### Yield Maximizer
- **Risk Level**: Medium-High
- **Allocation**: 90% of available capital
- **Target**: Highest yield opportunities
- **Best For**: Yield-focused strategies

### Risk Parity
- **Risk Level**: Medium
- **Target Volatility**: 10%
- **Approach**: Equal risk contribution
- **Best For**: Risk-balanced portfolios

## 📊 Output and Results

### Simulation Results

```rust
SimulationResults {
    initial_value: Decimal,
    final_value: Decimal,
    total_return: Decimal,
    total_return_pct: f64,
    sharpe_ratio: f64,
    max_drawdown_pct: f64,
    volatility_pct: f64,
    value_at_risk: Decimal,
    conditional_var: Decimal,
    portfolio_history: Vec<PortfolioSnapshot>,
}
```

### Monte Carlo Results

```rust
MonteCarloResults {
    iterations: usize,
    expected_value: Decimal,
    value_at_risk: Decimal,
    conditional_var: Decimal,
    max_drawdown_pct: f64,
    confidence_level: f64,
    distribution: Vec<f64>,
    percentiles: HashMap<u8, Decimal>,
}
```

### Backtest Results

```rust
BacktestResults {
    start_date: OffsetDateTime,
    end_date: OffsetDateTime,
    total_return_pct: f64,
    annualized_return_pct: f64,
    volatility_pct: f64,
    sharpe_ratio: f64,
    max_drawdown_pct: f64,
    win_rate: f64,
    profit_factor: f64,
    trades: Vec<Trade>,
}
```

## 🛠️ Development

### Build Commands

```bash
make build          # Build in release mode
make build-dev      # Build in dev mode
make test           # Run tests
make bench          # Run benchmarks
make fmt            # Format code
make clippy         # Run linter
make check          # Run fmt, clippy, and test
make docs           # Generate documentation
make clean          # Clean build artifacts
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run integration tests
cargo test --test '*'
```

### Benchmarking

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench monte_carlo_bench
```

## 📈 Performance

The simulator is optimized for performance:

- **Parallel Monte Carlo**: Uses Rayon for parallel processing
- **Efficient Data Structures**: Uses `Decimal` for precise financial calculations
- **Memory Efficient**: Streaming portfolio history when possible
- **Fast Execution**: Release builds with LTO and optimizations

Typical performance:
- Single simulation (100 steps): ~10ms
- Monte Carlo (10,000 iterations): ~30 seconds
- Backtest (1 year): ~100ms

## 🔧 Configuration

### Environment Variables

```bash
# Logging level
RUST_LOG=vaulta_simulator=info

# Number of threads for parallel processing
RAYON_NUM_THREADS=8
```

### Strategy Configuration

Strategies can be customized by modifying parameters:

```rust
let strategy = Strategy::conservative();
// Modify risk parameters, allocation rules, etc.
```

## 🤝 Contributing

Contributions are welcome! Please see our contributing guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style

- Follow Rust conventions
- Run `make fmt` and `make clippy` before committing
- Add tests for new features
- Update documentation

## 📝 License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

