// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

//! Étape 7 du fil rouge : les modèles de diffusion derrière un trait commun.
//!
//! Implémentation de référence, limitée aux notions vues jusqu'au module 7.

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
        let sigma = self.volatility;
        current * ((self.drift - 0.5 * sigma * sigma) * dt + sigma * dt.sqrt() * z).exp()
    }

    fn name(&self) -> String {
        format!("GBM(mu={}, sigma={})", self.drift, self.volatility)
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
        if self.speed == 0.0 {
            return current + self.volatility * dt.sqrt() * z;
        }
        let decay = (-self.speed * dt).exp();
        let std_dev = self.volatility * ((1.0 - decay * decay) / (2.0 * self.speed)).sqrt();
        self.mean + (current - self.mean) * decay + std_dev * z
    }

    fn name(&self) -> String {
        format!(
            "OU(speed={}, mean={}, sigma={})",
            self.speed, self.mean, self.volatility
        )
    }
}
