// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

use riskforge::model::{Direction, Instrument, NettingSet, OptionKind, Trade};

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

fn option(id: u32, kind: OptionKind, strike: f64, direction: Direction, quantity: f64) -> Trade {
    Trade::new(
        id,
        Instrument::EuropeanOption { kind, strike },
        direction,
        quantity,
    )
}

#[test]
fn direction_sign_is_positive_for_long_and_negative_for_short() {
    // Given / When / Then
    assert_close(Direction::Long.sign(), 1.0);
    assert_close(Direction::Short.sign(), -1.0);
}

#[test]
fn long_forward_gains_when_spot_rises_above_strike() {
    // Given
    let trade = forward(1, 100.0, Direction::Long, 1_000.0);
    // When
    let mtm = trade.mtm(110.0);
    // Then
    assert_close(mtm, 10_000.0);
}

#[test]
fn short_forward_has_the_opposite_mtm() {
    // Given
    let trade = forward(1, 100.0, Direction::Short, 1_000.0);
    // When
    let mtm = trade.mtm(110.0);
    // Then
    assert_close(mtm, -10_000.0);
}

#[test]
fn long_call_is_worth_its_intrinsic_value() {
    // Given
    let trade = option(2, OptionKind::Call, 100.0, Direction::Long, 10.0);
    // When / Then
    assert_close(trade.mtm(112.0), 120.0);
    assert_close(trade.mtm(90.0), 0.0);
}

#[test]
fn long_put_pays_when_spot_falls_below_strike() {
    // Given
    let trade = option(3, OptionKind::Put, 100.0, Direction::Long, 10.0);
    // When / Then
    assert_close(trade.mtm(90.0), 100.0);
    assert_close(trade.mtm(112.0), 0.0);
}

#[test]
fn short_call_can_only_lose() {
    // Given
    let trade = option(4, OptionKind::Call, 100.0, Direction::Short, 1_000.0);
    // When
    let mtm = trade.mtm(112.0);
    // Then
    assert_close(mtm, -12_000.0);
}

#[test]
fn netting_set_keeps_the_trades_it_receives() {
    // Given
    let mut set = NettingSet::new("ACME Bank");
    // When
    set.add(forward(1, 100.0, Direction::Long, 1_000.0));
    set.add(forward(2, 105.0, Direction::Short, 800.0));
    // Then
    assert_eq!(set.counterparty, "ACME Bank");
    assert_eq!(set.trades().len(), 2);
    assert_eq!(set.trades()[1].id, 2);
}

#[test]
fn net_mtm_sums_every_trade() {
    // Given
    let mut set = NettingSet::new("ACME Bank");
    set.add(forward(1, 100.0, Direction::Long, 1_000.0)); // +10 000 à 110
    set.add(forward(2, 105.0, Direction::Short, 800.0)); // -4 000 à 110
    // When
    let net = set.net_mtm(110.0);
    // Then
    assert_close(net, 6_000.0);
}

#[test]
fn netting_reduces_exposure_compared_to_gross() {
    // Given
    let mut set = NettingSet::new("ACME Bank");
    set.add(forward(1, 100.0, Direction::Long, 1_000.0));
    set.add(forward(2, 105.0, Direction::Short, 800.0));
    // When
    let net = set.net_exposure(110.0);
    let gross = set.gross_exposure(110.0);
    // Then
    assert_close(net, 6_000.0);
    assert_close(gross, 10_000.0);
}

#[test]
fn net_exposure_is_zero_when_we_owe_the_counterparty() {
    // Given
    let mut set = NettingSet::new("ACME Bank");
    set.add(forward(1, 100.0, Direction::Short, 1_000.0));
    // When
    let net = set.net_exposure(110.0);
    // Then
    assert_close(net, 0.0);
}

#[test]
fn empty_netting_set_has_no_exposure() {
    // Given
    let set = NettingSet::new("ACME Bank");
    // When / Then
    assert_close(set.net_exposure(110.0), 0.0);
    assert_close(set.gross_exposure(110.0), 0.0);
}
