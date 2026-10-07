// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

//! Étapes 2 et 3 du fil rouge : exposition, exposition attendue et profil.
//!
//! Remplace chaque `todo!()` puis lance `cargo test` jusqu'à ce que tout passe.

// Les paramètres restent inutilisés tant que les `todo!()` ne sont pas remplacés.
#![allow(unused_variables)]

/// Exposition d'un scénario : la partie positive du MtM.
pub fn exposure(mtm: f64) -> f64 {
    todo!("étape 2 : renvoyer max(mtm, 0)")
}

/// Exposition attendue (EE) : moyenne des expositions sur tous les scénarios.
pub fn expected_exposure(mtms: &[f64]) -> f64 {
    todo!("étape 2 : moyenne des expositions")
}

/// Exposition maximale observée parmi les scénarios.
pub fn max_exposure(mtms: &[f64]) -> f64 {
    todo!("étape 2 : plus forte exposition")
}

/// Choc de marché appliqué sur place : chaque MtM bouge du même montant.
pub fn shift(mtms: &mut [f64], amount: f64) {
    todo!("étape 3 : modifier les MtM sans copie")
}

/// Profil d'exposition : l'EE de chaque date de la grille de temps.
pub fn exposure_profile(dates: &[Vec<f64>]) -> Vec<f64> {
    todo!("étape 3 : une EE par date")
}
