// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

use riskforge::diffusion::{DiffusionModel, Gbm, MeanReverting};

const EPSILON: f64 = 1e-9;

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < EPSILON,
        "attendu {expected}, obtenu {actual}"
    );
}

#[test]
fn gbm_without_volatility_grows_at_the_drift_rate() {
    // Given
    let model = Gbm {
        drift: 0.05,
        volatility: 0.0,
    };
    // When
    let next = model.step(100.0, 1.0, 1.7);
    // Then
    assert_close(next, 100.0 * 0.05_f64.exp());
}

#[test]
fn gbm_applies_the_ito_correction() {
    // Given
    let model = Gbm {
        drift: 0.0,
        volatility: 0.2,
    };
    // When
    let next = model.step(100.0, 1.0, 1.0);
    // Then : exp(-0.02 + 0.2)
    assert_close(next, 100.0 * 0.18_f64.exp());
}

#[test]
fn gbm_scales_the_shock_with_the_square_root_of_time() {
    // Given
    let model = Gbm {
        drift: 0.0,
        volatility: 0.2,
    };
    // When
    let next = model.step(100.0, 0.25, -1.0);
    // Then : exp(-0.005 - 0.1)
    assert_close(next, 100.0 * (-0.105_f64).exp());
}

#[test]
fn mean_reverting_without_volatility_closes_the_gap_exponentially() {
    // Given
    let model = MeanReverting {
        speed: 2.0,
        mean: 100.0,
        volatility: 0.0,
    };
    // When
    let next = model.step(130.0, 0.5, 0.9);
    // Then
    assert_close(next, 100.0 + 30.0 * (-1.0_f64).exp());
}

#[test]
fn mean_reverting_uses_the_exact_variance() {
    // Given
    let model = MeanReverting {
        speed: 1.0,
        mean: 0.0,
        volatility: 1.0,
    };
    // When
    let next = model.step(0.0, 1.0, 1.0);
    // Then
    assert_close(next, ((1.0 - (-2.0_f64).exp()) / 2.0).sqrt());
}

#[test]
fn mean_reverting_without_speed_is_a_plain_random_walk() {
    // Given
    let model = MeanReverting {
        speed: 0.0,
        mean: 50.0,
        volatility: 5.0,
    };
    // When
    let next = model.step(100.0, 0.25, 2.0);
    // Then : 100 + 5 x 0.5 x 2
    assert_close(next, 105.0);
}

#[test]
fn each_model_has_a_descriptive_name() {
    // Given
    let gbm = Gbm {
        drift: 0.05,
        volatility: 0.2,
    };
    let ou = MeanReverting {
        speed: 2.0,
        mean: 100.0,
        volatility: 5.0,
    };
    // When / Then
    assert!(gbm.name().contains("GBM"), "{}", gbm.name());
    assert!(ou.name().contains("OU"), "{}", ou.name());
}

struct Frozen;

impl DiffusionModel for Frozen {
    fn step(&self, current: f64, _dt: f64, _z: f64) -> f64 {
        current
    }
}

#[test]
fn a_new_model_only_needs_step_and_gets_a_default_name() {
    // Given
    let model = Frozen;
    // When / Then
    assert_close(model.step(42.0, 1.0, 3.0), 42.0);
    assert_eq!(model.name(), "model");
}

#[test]
fn models_can_be_mixed_behind_trait_objects() {
    // Given
    let models: Vec<Box<dyn DiffusionModel>> = vec![
        Box::new(Gbm {
            drift: 0.0,
            volatility: 0.0,
        }),
        Box::new(MeanReverting {
            speed: 0.0,
            mean: 0.0,
            volatility: 0.0,
        }),
        Box::new(Frozen),
    ];
    // When / Then : aucun choc, aucune tendance : la valeur ne bouge pas
    for model in &models {
        assert_close(model.step(100.0, 1.0, 0.0), 100.0);
    }
}
