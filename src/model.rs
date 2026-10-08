// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

//! Étape 4 du fil rouge : le modèle métier (Trade, Instrument, NettingSet).
//!
//! Implémentation de référence, limitée aux notions vues jusqu'au module 4.

/// Sens du trade, vu de notre côté.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Long,
    Short,
}

impl Direction {
    /// +1 pour un achat, -1 pour une vente.
    pub fn sign(self) -> f64 {
        match self {
            Direction::Long => 1.0,
            Direction::Short => -1.0,
        }
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
        Self {
            id,
            instrument,
            direction,
            quantity,
        }
    }

    /// MtM du trade pour un prix spot donné.
    ///
    /// Simplification assumée à ce stade : une option vaut sa valeur intrinsèque.
    pub fn mtm(&self, spot: f64) -> f64 {
        let unit_value = match self.instrument {
            Instrument::Forward { strike } => spot - strike,
            Instrument::EuropeanOption {
                kind: OptionKind::Call,
                strike,
            } => (spot - strike).max(0.0),
            Instrument::EuropeanOption {
                kind: OptionKind::Put,
                strike,
            } => (strike - spot).max(0.0),
        };
        unit_value * self.quantity * self.direction.sign()
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
        Self {
            counterparty: counterparty.to_string(),
            trades: Vec::new(),
        }
    }

    pub fn add(&mut self, trade: Trade) {
        self.trades.push(trade);
    }

    pub fn trades(&self) -> &[Trade] {
        &self.trades
    }

    /// Somme des MtM de tous les trades.
    pub fn net_mtm(&self, spot: f64) -> f64 {
        let mut total = 0.0;
        for trade in &self.trades {
            total += trade.mtm(spot);
        }
        total
    }

    /// Exposition avec netting : max(somme des MtM, 0).
    pub fn net_exposure(&self, spot: f64) -> f64 {
        self.net_mtm(spot).max(0.0)
    }

    /// Exposition sans netting : somme des expositions trade par trade.
    pub fn gross_exposure(&self, spot: f64) -> f64 {
        let mut total = 0.0;
        for trade in &self.trades {
            total += trade.mtm(spot).max(0.0);
        }
        total
    }
}
