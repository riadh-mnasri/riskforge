// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

use riskforge::exposure::{expected_exposure, exposure, exposure_profile, max_exposure, shift};

const EPSILON: f64 = 1e-9;

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < EPSILON,
        "attendu {expected}, obtenu {actual}"
    );
}

#[test]
fn exposure_keeps_a_positive_mtm() {
    // Given
    let mtm = 1_200.0;
    // When
    let result = exposure(mtm);
    // Then
    assert_close(result, 1_200.0);
}

#[test]
fn exposure_floors_a_negative_mtm_at_zero() {
    // Given
    let mtm = -300.0;
    // When
    let result = exposure(mtm);
    // Then
    assert_close(result, 0.0);
}

#[test]
fn expected_exposure_averages_over_all_scenarios_including_negative_ones() {
    // Given
    let mtms = [1_200.0, -300.0, 450.0, 0.0, -800.0];
    // When
    let ee = expected_exposure(&mtms);
    // Then
    assert_close(ee, 330.0);
}

#[test]
fn max_exposure_returns_the_highest_exposure() {
    // Given
    let mtms = [1_200.0, -300.0, 450.0, 0.0, -800.0];
    // When
    let max = max_exposure(&mtms);
    // Then
    assert_close(max, 1_200.0);
}

#[test]
fn max_exposure_is_zero_when_every_mtm_is_negative() {
    // Given
    let mtms = [-10.0, -20.0];
    // When
    let max = max_exposure(&mtms);
    // Then
    assert_close(max, 0.0);
}

#[test]
fn shift_moves_every_mtm_in_place() {
    // Given
    let mut mtms = vec![1_200.0, -300.0, 450.0];
    // When
    shift(&mut mtms, 200.0);
    // Then
    assert_eq!(mtms, vec![1_400.0, -100.0, 650.0]);
}

#[test]
fn shift_works_on_a_sub_slice_only() {
    // Given
    let mut mtms = [1.0, 2.0, 3.0, 4.0];
    // When
    shift(&mut mtms[2..], 10.0);
    // Then
    assert_eq!(mtms, [1.0, 2.0, 13.0, 14.0]);
}

#[test]
fn exposure_profile_gives_one_expected_exposure_per_date() {
    // Given
    let dates = vec![
        vec![100.0, -50.0, 20.0],
        vec![150.0, -80.0, 60.0],
        vec![90.0, 30.0, -200.0],
    ];
    // When
    let profile = exposure_profile(&dates);
    // Then
    assert_eq!(profile.len(), 3);
    assert_close(profile[0], 40.0);
    assert_close(profile[1], 70.0);
    assert_close(profile[2], 40.0);
}
