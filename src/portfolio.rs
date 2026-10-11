// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

//! Étape 5 du fil rouge : un portefeuille multi-contreparties.
//!
//! Les trades sont rangés par contrepartie, chacune avec son netting set.
//! Remplace chaque `todo!()` puis lance `cargo test --test portfolio_test`.

// Les paramètres restent inutilisés tant que les `todo!()` ne sont pas remplacés.
#![allow(unused_variables, dead_code)]

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
        todo!("étape 5 : l'API entry de HashMap")
    }

    /// Le netting set d'une contrepartie, s'il existe.
    pub fn netting_set(&self, counterparty: &str) -> Option<&NettingSet> {
        todo!("étape 5 : get renvoie déjà une Option")
    }

    /// Noms des contreparties, triés par ordre alphabétique.
    pub fn counterparties(&self) -> Vec<&str> {
        todo!("étape 5 : collecter les clés puis trier")
    }

    /// Nombre total de trades, toutes contreparties confondues.
    pub fn trade_count(&self) -> usize {
        todo!("étape 5 : additionner la taille de chaque netting set")
    }

    /// Exposition nette par contrepartie, triée par nom.
    pub fn exposures(&self, spot: f64) -> BTreeMap<String, f64> {
        todo!("étape 5 : une entrée par netting set")
    }

    /// Somme des expositions nettes (pas de netting entre contreparties).
    pub fn total_exposure(&self, spot: f64) -> f64 {
        todo!("étape 5 : somme des net_exposure")
    }
}
