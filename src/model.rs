// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

//! Étape 4 du fil rouge : le modèle métier (Trade, Instrument, NettingSet).
//!
//! Les types sont déjà déclarés ; remplace chaque `todo!()` puis lance
//! `cargo test --test model_test` jusqu'à ce que tout passe.

// Les paramètres restent inutilisés tant que les `todo!()` ne sont pas remplacés.
#![allow(unused_variables)]

/// Sens du trade, vu de notre côté.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Long,
    Short,
}

impl Direction {
    /// +1 pour un achat, -1 pour une vente.
    pub fn sign(self) -> f64 {
        todo!("étape 4 : un match sur self")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptionKind {
    Call,
    Put,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instrument {
    Forward { strike: f64 },
    EuropeanOption { kind: OptionKind, strike: f64 },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Trade {
    pub id: u32,
    pub instrument: Instrument,
    pub direction: Direction,
    pub quantity: f64,
}

impl Trade {
    pub fn new(id: u32, instrument: Instrument, direction: Direction, quantity: f64) -> Self {
        Self { id, instrument, direction, quantity }
    }

    /// MtM du trade pour un prix spot donné.
    ///
    /// Simplification assumée à ce stade : une option vaut sa valeur intrinsèque.
    pub fn mtm(&self, spot: f64) -> f64 {
        todo!("étape 4 : valeur unitaire selon l'instrument, x quantité x sens")
    }
}

/// Ensemble des trades couverts par un même accord de compensation.
#[derive(Debug, Clone, PartialEq)]
pub struct NettingSet {
    pub counterparty: String,
    trades: Vec<Trade>,
}

impl NettingSet {
    pub fn new(counterparty: &str) -> Self {
        Self { counterparty: counterparty.to_string(), trades: Vec::new() }
    }

    pub fn add(&mut self, trade: Trade) {
        self.trades.push(trade);
    }

    pub fn trades(&self) -> &[Trade] {
        &self.trades
    }

    /// Somme des MtM de tous les trades.
    pub fn net_mtm(&self, spot: f64) -> f64 {
        todo!("étape 4 : somme des MtM")
    }

    /// Exposition avec netting : max(somme des MtM, 0).
    pub fn net_exposure(&self, spot: f64) -> f64 {
        todo!("étape 4 : plancher à zéro sur le total")
    }

    /// Exposition sans netting : somme des expositions trade par trade.
    pub fn gross_exposure(&self, spot: f64) -> f64 {
        todo!("étape 4 : plancher à zéro sur chaque trade")
    }
}
