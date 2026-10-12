// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

//! Étape 7 du fil rouge : les modèles de diffusion derrière un trait commun.
//!
//! Remplace chaque `todo!()` puis lance `cargo test --test diffusion_test`.

// Les paramètres restent inutilisés tant que les `todo!()` ne sont pas remplacés.
#![allow(unused_variables)]

/// Fait évoluer une valeur de marché sur un pas de temps.
pub trait DiffusionModel {
    /// Valeur après un pas `dt` (en années), à partir d'un tirage normal centré réduit `z`.
    fn step(&self, current: f64, dt: f64, z: f64) -> f64;

    /// Nom lisible du modèle, pour les rapports.
    fn name(&self) -> String {
        String::from("model")
    }
}

/// Mouvement brownien géométrique : S(t+dt) = S(t) exp((mu - sigma²/2) dt + sigma sqrt(dt) z).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Gbm {
    pub drift: f64,
    pub volatility: f64,
}

impl DiffusionModel for Gbm {
    fn step(&self, current: f64, dt: f64, z: f64) -> f64 {
        todo!("étape 7 : la formule du GBM ci-dessus")
    }

    fn name(&self) -> String {
        todo!("étape 7 : par exemple \"GBM(mu=0.05, sigma=0.2)\"")
    }
}

/// Processus d'Ornstein-Uhlenbeck (retour à la moyenne), discrétisation exacte :
///
/// x(t+dt) = mean + (x(t) - mean) e^(-speed dt)
///           + volatility sqrt((1 - e^(-2 speed dt)) / (2 speed)) z
///
/// Si `speed` vaut 0, il n'y a plus de rappel : x(t+dt) = x(t) + volatility sqrt(dt) z.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeanReverting {
    pub speed: f64,
    pub mean: f64,
    pub volatility: f64,
}

impl DiffusionModel for MeanReverting {
    fn step(&self, current: f64, dt: f64, z: f64) -> f64 {
        todo!("étape 7 : la discrétisation exacte, avec le cas speed == 0")
    }

    fn name(&self) -> String {
        todo!("étape 7 : par exemple \"OU(speed=2, mean=100, sigma=5)\"")
    }
}
