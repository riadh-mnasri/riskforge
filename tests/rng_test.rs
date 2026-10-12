// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

use riskforge::rng::Rng;

#[test]
fn same_seed_gives_the_same_sequence() {
    // Given
    let mut a = Rng::new(42);
    let mut b = Rng::new(42);
    // When / Then
    for _ in 0..1_000 {
        assert_eq!(a.next_u64(), b.next_u64());
    }
}

#[test]
fn uniform_stays_strictly_between_zero_and_one() {
    // Given
    let mut rng = Rng::new(1);
    // When / Then
    for _ in 0..100_000 {
        let u = rng.uniform();
        assert!(u > 0.0 && u < 1.0, "hors de ]0, 1[ : {u}");
    }
}

#[test]
fn normal_draws_have_zero_mean_and_unit_variance() {
    // Given
    let mut rng = Rng::new(7);
    let n = 200_000;
    // When
    let mut sum = 0.0;
    let mut sum_sq = 0.0;
    for _ in 0..n {
        let z = rng.normal();
        sum += z;
        sum_sq += z * z;
    }
    let mean = sum / n as f64;
    let variance = sum_sq / n as f64 - mean * mean;
    // Then
    assert!(mean.abs() < 0.01, "moyenne {mean}");
    assert!((variance - 1.0).abs() < 0.01, "variance {variance}");
}
