use rust_decimal::Decimal;

/// Utility functions for the simulator

/// Convert f64 to Decimal safely
pub fn f64_to_decimal(value: f64) -> Decimal {
    Decimal::try_from(value).unwrap_or(Decimal::ZERO)
}

/// Calculate percentage change
pub fn percentage_change(old: Decimal, new: Decimal) -> f64 {
    if old > Decimal::ZERO {
        ((new - old) / old * Decimal::from(100)).to_f64().unwrap_or(0.0)
    } else {
        0.0
    }
}

/// Format currency value
pub fn format_currency(value: Decimal) -> String {
    format!("${:.2}", value.to_f64().unwrap_or(0.0))
}

/// Format percentage
pub fn format_percentage(value: f64) -> String {
    format!("{:.2}%", value)
}
