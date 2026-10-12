// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

//! Étape 7 du fil rouge : simulation Monte Carlo et profil d'exposition simulé.
//!
//! Implémentation de référence, limitée aux notions vues jusqu'au module 7.

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
    let dt = config.dt();
    let mut rng = Rng::new(config.seed);
    let mut grid = vec![Vec::with_capacity(config.paths); config.steps + 1];
    for _ in 0..config.paths {
        let mut value = config.spot;
        grid[0].push(value);
        for row in grid.iter_mut().skip(1) {
            value = model.step(value, dt, rng.normal());
            row.push(value);
        }
    }
    grid
}

/// EE du netting set à chaque date de la grille : moyenne des expositions nettes.
pub fn expected_exposure_profile(set: &NettingSet, grid: &[Vec<f64>]) -> Vec<f64> {
    let mut profile = Vec::with_capacity(grid.len());
    for scenarios in grid {
        let mut net_mtms = Vec::with_capacity(scenarios.len());
        for &spot in scenarios {
            net_mtms.push(set.net_mtm(spot));
        }
        profile.push(expected_exposure(&net_mtms));
    }
    profile
}
