// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

//! Générateur pseudo-aléatoire déterministe, fourni (pas un exercice).
//!
//! SplitMix64 pour les entiers, Box-Muller pour la loi normale. Une même
//! graine donne toujours la même suite de tirages : indispensable pour tester
//! un code Monte Carlo et comparer deux implémentations sur les mêmes tirages.
//! Ce générateur n'est pas cryptographique.

use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Rng {
    state: u64,
    spare_normal: Option<f64>,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self {
            state: seed,
            spare_normal: None,
        }
    }

    /// SplitMix64 (Steele, Lea et Flood, 2014).
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Tirage uniforme dans l'intervalle ouvert ]0, 1[.
    pub fn uniform(&mut self) -> f64 {
        ((self.next_u64() >> 11) as f64 + 0.5) / (1u64 << 53) as f64
    }

    /// Tirage selon la loi normale centrée réduite (Box-Muller, deux valeurs par appel pair).
    pub fn normal(&mut self) -> f64 {
        if let Some(z) = self.spare_normal.take() {
            return z;
        }
        let radius = (-2.0 * self.uniform().ln()).sqrt();
        let angle = 2.0 * PI * self.uniform();
        self.spare_normal = Some(radius * angle.sin());
        radius * angle.cos()
    }
}
