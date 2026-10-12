// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

use riskforge::diffusion::{DiffusionModel, Gbm, MeanReverting};
use riskforge::model::{Direction, Instrument, NettingSet, Trade};
use riskforge::simulation::{SimulationConfig, expected_exposure_profile, simulate};

const EPSILON: f64 = 1e-9;

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < EPSILON,
        "attendu {expected}, obtenu {actual}"
    );
}

fn config(paths: usize, seed: u64) -> SimulationConfig {
    SimulationConfig {
        spot: 100.0,
        horizon_years: 1.0,
        steps: 12,
        paths,
        seed,
    }
}

fn gbm(drift: f64, volatility: f64) -> Gbm {
    Gbm { drift, volatility }
}

#[test]
fn grid_has_one_row_per_date_and_one_value_per_path() {
    // Given / When
    let grid = simulate(&gbm(0.05, 0.2), &config(50, 1));
    // Then
    assert_eq!(grid.len(), 13);
    for row in &grid {
        assert_eq!(row.len(), 50);
    }
}

#[test]
fn first_date_holds_the_initial_spot_for_every_path() {
    // Given / When
    let grid = simulate(&gbm(0.05, 0.2), &config(20, 1));
    // Then
    for &value in &grid[0] {
        assert_close(value, 100.0);
    }
}

#[test]
fn same_seed_gives_the_same_grid() {
    // Given
    let model = gbm(0.05, 0.2);
    // When
    let a = simulate(&model, &config(100, 9));
    let b = simulate(&model, &config(100, 9));
    let c = simulate(&model, &config(100, 10));
    // Then
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn deterministic_model_follows_the_analytic_path() {
    // Given
    let cfg = config(3, 1);
    // When
    let grid = simulate(&gbm(0.05, 0.0), &cfg);
    // Then
    for (date, row) in grid.iter().enumerate() {
        let expected = 100.0 * (0.05 * date as f64 * cfg.dt()).exp();
        for &value in row {
            assert!((value - expected).abs() < 1e-9);
        }
    }
}

#[test]
fn simulated_gbm_mean_matches_theory() {
    // Given
    let model = gbm(0.05, 0.2);
    // When
    let grid = simulate(&model, &config(20_000, 2026));
    let terminal = &grid[12];
    let mean = terminal.iter().sum::<f64>() / terminal.len() as f64;
    // Then : E[S(T)] = S0 exp(mu T), à 1 % près
    let theory = 100.0 * 0.05_f64.exp();
    assert!(
        (mean / theory - 1.0).abs() < 0.01,
        "moyenne {mean}, théorie {theory}"
    );
}

#[test]
fn simulation_accepts_a_trait_object() {
    // Given
    let model: Box<dyn DiffusionModel> = Box::new(MeanReverting {
        speed: 1.0,
        mean: 100.0,
        volatility: 0.0,
    });
    // When
    let grid = simulate(model.as_ref(), &config(4, 1));
    // Then : partie de 100 = la moyenne, sans volatilité on y reste
    for row in &grid {
        for &value in row {
            assert_close(value, 100.0);
        }
    }
}

fn long_forward_set(strike: f64, quantity: f64) -> NettingSet {
    let mut set = NettingSet::new("ACME");
    set.add(Trade::new(
        1,
        Instrument::Forward { strike },
        Direction::Long,
        quantity,
    ));
    set
}

#[test]
fn exposure_profile_is_flat_when_the_market_does_not_move() {
    // Given
    let set = long_forward_set(90.0, 10.0);
    let grid = simulate(&gbm(0.0, 0.0), &config(5, 1));
    // When
    let profile = expected_exposure_profile(&set, &grid);
    // Then : (100 - 90) x 10 à chaque date
    assert_eq!(profile.len(), 13);
    for ee in profile {
        assert_close(ee, 100.0);
    }
}

#[test]
fn exposure_profile_starts_at_todays_exposure_and_stays_non_negative() {
    // Given : forward à la monnaie, exposition nulle aujourd'hui
    let set = long_forward_set(100.0, 10.0);
    let grid = simulate(&gbm(0.0, 0.3), &config(5_000, 3));
    // When
    let profile = expected_exposure_profile(&set, &grid);
    // Then
    assert_close(profile[0], 0.0);
    for &ee in &profile {
        assert!(ee >= 0.0);
    }
    assert!(
        profile[12] > profile[1],
        "l'EE doit croître avec l'incertitude"
    );
}
