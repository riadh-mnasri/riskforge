// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

use std::error::Error;
use std::fs;

use riskforge::loader::load_portfolio;

const PORTFOLIO_FILE: &str = "data/portfolio.csv";
const SPOT: f64 = 110.0;

fn main() -> Result<(), Box<dyn Error>> {
    let csv = fs::read_to_string(PORTFOLIO_FILE)?;
    let portfolio = load_portfolio(&csv)?;

    println!("riskforge");
    println!(
        "{} trades, {} counterparties, spot = {SPOT}",
        portfolio.trade_count(),
        portfolio.counterparties().len()
    );
    for (counterparty, exposure) in portfolio.exposures(SPOT) {
        println!("  {counterparty:<10} {exposure:>12.2}");
    }
    println!("Total exposure: {:.2}", portfolio.total_exposure(SPOT));
    Ok(())
}
