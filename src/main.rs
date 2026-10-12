// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

use std::error::Error;
use std::fs;

use riskforge::diffusion::{DiffusionModel, Gbm, MeanReverting};
use riskforge::loader::load_portfolio;
use riskforge::simulation::{SimulationConfig, expected_exposure_profile, simulate};

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

    // Étape 7 : profil d'EE simulé pour ACME, avec deux modèles interchangeables.
    let Some(acme) = portfolio.netting_set("ACME") else {
        return Ok(());
    };
    let config = SimulationConfig {
        spot: SPOT,
        horizon_years: 5.0,
        steps: 60,
        paths: 10_000,
        seed: 2026,
    };
    let models: Vec<Box<dyn DiffusionModel>> = vec![
        Box::new(Gbm {
            drift: 0.02,
            volatility: 0.25,
        }),
        Box::new(MeanReverting {
            speed: 1.5,
            mean: 100.0,
            volatility: 20.0,
        }),
    ];

    println!("\nACME expected exposure, {} paths", config.paths);
    for model in &models {
        let grid = simulate(model.as_ref(), &config);
        let profile = expected_exposure_profile(acme, &grid);
        println!(
            "  {:<32} 1y {:>9.2}   2y {:>9.2}   5y {:>9.2}",
            model.name(),
            profile[12],
            profile[24],
            profile[60]
        );
    }
    Ok(())
}
