// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

//! Étape 6 du fil rouge : charger un portefeuille depuis un CSV, sans panic.
//!
//! Format attendu (en-tête obligatoire en première ligne) :
//! `id,counterparty,instrument,strike,direction,quantity`
//! avec `instrument` parmi forward, call, put et `direction` parmi long, short.
//! Les lignes vides et les lignes commençant par `#` sont ignorées.
//!
//! Les types d'erreurs et leurs messages sont fournis ; remplace chaque
//! `todo!()` puis lance `cargo test --test loader_test`.

// Les paramètres restent inutilisés tant que les `todo!()` ne sont pas remplacés.
#![allow(unused_variables, unused_imports, dead_code)]

use std::fmt;

use crate::model::{Direction, Instrument, OptionKind, Trade};
use crate::portfolio::Portfolio;

pub const FIELD_COUNT: usize = 6;

/// Ce qui ne va pas dans une ligne, indépendamment de sa position dans le fichier.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    WrongFieldCount { expected: usize, found: usize },
    InvalidNumber { field: &'static str, value: String },
    UnknownInstrument(String),
    UnknownDirection(String),
    EmptyCounterparty,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::WrongFieldCount { expected, found } => {
                write!(f, "{expected} champs attendus, {found} trouvés")
            }
            ParseError::InvalidNumber { field, value } => {
                write!(f, "champ {field} : '{value}' n'est pas un nombre valide")
            }
            ParseError::UnknownInstrument(value) => write!(f, "instrument inconnu : {value}"),
            ParseError::UnknownDirection(value) => write!(f, "sens inconnu : {value}"),
            ParseError::EmptyCounterparty => write!(f, "contrepartie vide"),
        }
    }
}

impl std::error::Error for ParseError {}

/// Une erreur de chargement : la ligne fautive (numérotée à partir de 1) et la cause.
#[derive(Debug, Clone, PartialEq)]
pub struct LoadError {
    pub line: usize,
    pub error: ParseError,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ligne {} : {}", self.line, self.error)
    }
}

impl std::error::Error for LoadError {}

/// Convertit un champ texte en f64, ou renvoie `InvalidNumber` avec le nom du champ.
pub fn parse_number(field: &'static str, value: &str) -> Result<f64, ParseError> {
    todo!("étape 6 : parse puis map_err")
}

/// Convertit l'identifiant en u32, ou renvoie `InvalidNumber { field: "id", .. }`.
pub fn parse_id(value: &str) -> Result<u32, ParseError> {
    todo!("étape 6 : comme parse_number, mais vers un u32")
}

/// "forward", "call" ou "put" (casse ignorée), avec le strike déjà converti.
pub fn parse_instrument(value: &str, strike: f64) -> Result<Instrument, ParseError> {
    todo!("étape 6 : un match sur la valeur en minuscules")
}

/// "long" ou "short" (casse ignorée).
pub fn parse_direction(value: &str) -> Result<Direction, ParseError> {
    todo!("étape 6 : un match sur la valeur en minuscules")
}

/// Une ligne CSV devient (contrepartie, trade). Les champs sont nettoyés avec `trim`.
pub fn parse_trade(line: &str) -> Result<(String, Trade), ParseError> {
    todo!("étape 6 : découper, vérifier le nombre de champs, puis ? sur chaque champ")
}

/// Charge un CSV complet ; la première erreur rencontrée est renvoyée avec son numéro de ligne.
pub fn load_portfolio(csv: &str) -> Result<Portfolio, LoadError> {
    todo!("étape 6 : ignorer en-tête, lignes vides et commentaires, puis map_err + ?")
}
