// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

//! Étape 5 du fil rouge : un portefeuille multi-contreparties.
//!
//! Les trades sont rangés par contrepartie, chacune avec son netting set.
//! Implémentation de référence, limitée aux notions vues jusqu'au module 5.

use std::collections::{BTreeMap, HashMap};

use crate::model::{NettingSet, Trade};

#[derive(Debug, Default)]
pub struct Portfolio {
    netting_sets: HashMap<String, NettingSet>,
}

impl Portfolio {
    pub fn new() -> Self {
        Self::default()
    }

    /// Range le trade dans le netting set de sa contrepartie (créé au besoin).
    pub fn add(&mut self, counterparty: &str, trade: Trade) {
        self.netting_sets
            .entry(counterparty.to_string())
            .or_insert(NettingSet::new(counterparty))
            .add(trade);
    }

    /// Le netting set d'une contrepartie, s'il existe.
    pub fn netting_set(&self, counterparty: &str) -> Option<&NettingSet> {
        self.netting_sets.get(counterparty)
    }

    /// Noms des contreparties, triés par ordre alphabétique.
    pub fn counterparties(&self) -> Vec<&str> {
        let mut names = Vec::with_capacity(self.netting_sets.len());
        for name in self.netting_sets.keys() {
            names.push(name.as_str());
        }
        names.sort();
        names
    }

    /// Nombre total de trades, toutes contreparties confondues.
    pub fn trade_count(&self) -> usize {
        let mut count = 0;
        for set in self.netting_sets.values() {
            count += set.trades().len();
        }
        count
    }

    /// Exposition nette par contrepartie, triée par nom.
    pub fn exposures(&self, spot: f64) -> BTreeMap<String, f64> {
        let mut result = BTreeMap::new();
        for (name, set) in &self.netting_sets {
            result.insert(name.clone(), set.net_exposure(spot));
        }
        result
    }

    /// Somme des expositions nettes (pas de netting entre contreparties).
    pub fn total_exposure(&self, spot: f64) -> f64 {
        let mut total = 0.0;
        for set in self.netting_sets.values() {
            total += set.net_exposure(spot);
        }
        total
    }
}
