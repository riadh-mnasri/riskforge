// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

//! Étape 7 du fil rouge : simulation Monte Carlo et profil d'exposition simulé.
//!
//! Remplace chaque `todo!()` puis lance `cargo test --test simulation_test`.

// Les paramètres restent inutilisés tant que les `todo!()` ne sont pas remplacés.
#![allow(unused_variables, unused_imports)]

use crate::diffusion::DiffusionModel;
use crate::exposure::expected_exposure;
use crate::model::NettingSet;
use crate::rng::Rng;

/// Paramètres d'une simulation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimulationConfig {
    pub spot: f64,
    pub horizon_years: f64,
    pub steps: usize,
    pub paths: usize,
    pub seed: u64,
}

impl SimulationConfig {
    /// Pas de temps en années.
    pub fn dt(&self) -> f64 {
        self.horizon_years / self.steps as f64
    }
}

/// Simule `paths` trajectoires et renvoie la grille `grid[date][scénario]`.
///
/// La grille a `steps + 1` dates : la date 0 contient le spot initial pour tous
/// les scénarios. Les tirages sont consommés trajectoire par trajectoire (tous
/// les pas de la trajectoire 0, puis ceux de la 1...), avec un `Rng` créé à
/// partir de `config.seed`. Le `?Sized` autorise aussi un `&dyn DiffusionModel`.
pub fn simulate<M: DiffusionModel + ?Sized>(model: &M, config: &SimulationConfig) -> Vec<Vec<f64>> {
    todo!("étape 7 : une boucle sur les trajectoires, une boucle sur les pas")
}

/// EE du netting set à chaque date de la grille : moyenne des expositions nettes.
pub fn expected_exposure_profile(set: &NettingSet, grid: &[Vec<f64>]) -> Vec<f64> {
    todo!("étape 7 : pour chaque date, net_exposure de chaque scénario, puis la moyenne")
}
