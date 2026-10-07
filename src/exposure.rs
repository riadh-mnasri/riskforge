// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

//! Étapes 2 et 3 du fil rouge : exposition, exposition attendue et profil.
//!
//! Implémentation de référence, volontairement limitée aux notions vues dans
//! les modules 1 à 3 (boucles `for`, références, slices). Les itérateurs
//! viendront la simplifier au module « Closures et itérateurs ».

/// Exposition d'un scénario : la partie positive du MtM.
pub fn exposure(mtm: f64) -> f64 {
    mtm.max(0.0)
}

/// Exposition attendue (EE) : moyenne des expositions sur tous les scénarios.
pub fn expected_exposure(mtms: &[f64]) -> f64 {
    let mut total = 0.0;
    for &mtm in mtms {
        total += exposure(mtm);
    }
    total / mtms.len() as f64
}

/// Exposition maximale observée parmi les scénarios.
pub fn max_exposure(mtms: &[f64]) -> f64 {
    let mut max = 0.0;
    for &mtm in mtms {
        let e = exposure(mtm);
        if e > max {
            max = e;
        }
    }
    max
}

/// Choc de marché appliqué sur place : chaque MtM bouge du même montant.
pub fn shift(mtms: &mut [f64], amount: f64) {
    for mtm in mtms.iter_mut() {
        *mtm += amount;
    }
}

/// Profil d'exposition : l'EE de chaque date de la grille de temps.
pub fn exposure_profile(dates: &[Vec<f64>]) -> Vec<f64> {
    let mut profile = Vec::with_capacity(dates.len());
    for scenarios in dates {
        profile.push(expected_exposure(scenarios));
    }
    profile
}
