// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

use riskforge::model::{Direction, Instrument, Trade};
use riskforge::portfolio::Portfolio;

const EPSILON: f64 = 1e-9;

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < EPSILON,
        "attendu {expected}, obtenu {actual}"
    );
}

fn forward(id: u32, strike: f64, direction: Direction, quantity: f64) -> Trade {
    Trade::new(id, Instrument::Forward { strike }, direction, quantity)
}

/// ACME : +10 000 et -4 000 à 110 ; Globex : -2 000 ; Initech : +1 500.
fn sample_portfolio() -> Portfolio {
    let mut portfolio = Portfolio::new();
    portfolio.add("ACME", forward(1, 100.0, Direction::Long, 1_000.0));
    portfolio.add("Globex", forward(2, 120.0, Direction::Long, 200.0));
    portfolio.add("ACME", forward(3, 105.0, Direction::Short, 800.0));
    portfolio.add("Initech", forward(4, 95.0, Direction::Long, 100.0));
    portfolio
}

#[test]
fn new_portfolio_is_empty() {
    // Given
    let portfolio = Portfolio::new();
    // When / Then
    assert_eq!(portfolio.trade_count(), 0);
    assert!(portfolio.counterparties().is_empty());
    assert_close(portfolio.total_exposure(110.0), 0.0);
}

#[test]
fn trades_of_the_same_counterparty_share_one_netting_set() {
    // Given
    let portfolio = sample_portfolio();
    // When
    let acme = portfolio.netting_set("ACME");
    // Then
    let acme = acme.expect("ACME doit avoir un netting set");
    assert_eq!(acme.counterparty, "ACME");
    assert_eq!(acme.trades().len(), 2);
}

#[test]
fn unknown_counterparty_has_no_netting_set() {
    // Given
    let portfolio = sample_portfolio();
    // When / Then
    assert!(portfolio.netting_set("Umbrella").is_none());
}

#[test]
fn counterparties_are_listed_in_alphabetical_order() {
    // Given
    let portfolio = sample_portfolio();
    // When
    let names = portfolio.counterparties();
    // Then
    assert_eq!(names, vec!["ACME", "Globex", "Initech"]);
}

#[test]
fn trade_count_spans_every_counterparty() {
    // Given / When / Then
    assert_eq!(sample_portfolio().trade_count(), 4);
}

#[test]
fn exposures_are_netted_within_each_counterparty() {
    // Given
    let portfolio = sample_portfolio();
    // When
    let exposures = portfolio.exposures(110.0);
    // Then
    let keys: Vec<&String> = exposures.keys().collect();
    assert_eq!(keys, vec!["ACME", "Globex", "Initech"]);
    assert_close(exposures["ACME"], 6_000.0);
    assert_close(exposures["Globex"], 0.0);
    assert_close(exposures["Initech"], 1_500.0);
}

#[test]
fn total_exposure_never_nets_across_counterparties() {
    // Given
    let portfolio = sample_portfolio();
    // When
    let total = portfolio.total_exposure(110.0);
    // Then : la dette envers Globex ne compense pas la créance sur ACME
    assert_close(total, 7_500.0);
}
