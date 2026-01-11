use clap::{Parser, Subcommand};
use tracing::{info, error};
use vaulta_simulator::{
    backtest::BacktestEngine,
    monte_carlo::MonteCarloEngine,
    simulator::Simulator,
    strategy::Strategy,
    types::*,
};

#[derive(Parser)]
#[command(name = "vaulta-simulator")]
#[command(about = "High-fidelity capital routing simulator for Vaulta Protocol", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a single simulation with specified parameters
    Simulate {
        /// Initial capital amount
        #[arg(short, long, default_value = "1000000.0")]
        capital: f64,
        /// Number of time steps
        #[arg(short, long, default_value = "100")]
        steps: usize,
        /// Strategy name to use
        #[arg(short, long, default_value = "conservative")]
        strategy: String,
    },
    /// Run Monte Carlo stress testing
    MonteCarlo {
        /// Number of Monte Carlo iterations
        #[arg(short, long, default_value = "10000")]
        iterations: usize,
        /// Number of scenarios to generate
        #[arg(short, long, default_value = "100")]
        scenarios: usize,
        /// Confidence level (0.0 to 1.0)
        #[arg(short, long, default_value = "0.95")]
        confidence: f64,
    },
    /// Run backtesting on historical data
    Backtest {
        /// Start date (YYYY-MM-DD)
        #[arg(short, long)]
        start_date: String,
        /// End date (YYYY-MM-DD)
        #[arg(short, long)]
        end_date: String,
        /// Strategy name
        #[arg(short, long, default_value = "balanced")]
        strategy: String,
    },
    /// List available strategies
    Strategies,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "vaulta_simulator=info".into()),
        )
        .init();

    info!("🚀 Vaulta Simulator v{}", env!("CARGO_PKG_VERSION"));
    info!("Starting simulation engine...");

    let cli = Cli::parse();

    match cli.command {
        Commands::Simulate {
            capital,
            steps,
            strategy,
        } => {
            info!("Running simulation with capital: {}, steps: {}, strategy: {}", 
                  capital, steps, strategy);
            
            let strategy = Strategy::from_name(&strategy)?;
            let mut simulator = Simulator::new(capital, strategy);
            
            for step in 0..steps {
                simulator.step()?;
                if step % 10 == 0 {
                    info!("Step {}: Portfolio value = {:.2}", 
                          step, simulator.portfolio_value());
                }
            }
            
            let results = simulator.finalize();
            info!("Simulation complete!");
            info!("Final portfolio value: {:.2}", results.final_value);
            info!("Total return: {:.2}%", results.total_return_pct);
            info!("Sharpe ratio: {:.4}", results.sharpe_ratio);
        }
        
        Commands::MonteCarlo {
            iterations,
            scenarios,
            confidence,
        } => {
            info!("Running Monte Carlo stress test...");
            info!("Iterations: {}, Scenarios: {}, Confidence: {}", 
                  iterations, scenarios, confidence);
            
            let mut engine = MonteCarloEngine::new(iterations, scenarios);
            let results = engine.run_stress_test(confidence).await?;
            
            info!("Monte Carlo analysis complete!");
            info!("Expected value: {:.2}", results.expected_value);
            info!("Value at Risk ({}%): {:.2}", 
                  confidence * 100.0, results.value_at_risk);
            info!("Conditional VaR: {:.2}", results.conditional_var);
            info!("Max drawdown: {:.2}%", results.max_drawdown_pct);
        }
        
        Commands::Backtest {
            start_date,
            end_date,
            strategy,
        } => {
            info!("Running backtest from {} to {} with strategy: {}", 
                  start_date, end_date, strategy);
            
            let strategy = Strategy::from_name(&strategy)?;
            let mut engine = BacktestEngine::new(&start_date, &end_date, strategy)?;
            
            let results = engine.run().await?;
            
            info!("Backtest complete!");
            info!("Total return: {:.2}%", results.total_return_pct);
            info!("Annualized return: {:.2}%", results.annualized_return_pct);
            info!("Volatility: {:.2}%", results.volatility_pct);
            info!("Sharpe ratio: {:.4}", results.sharpe_ratio);
            info!("Max drawdown: {:.2}%", results.max_drawdown_pct);
        }
        
        Commands::Strategies => {
            println!("Available strategies:");
            for strategy in Strategy::list_all() {
                println!("  - {}", strategy);
            }
        }
    }

    Ok(())
}
