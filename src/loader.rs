// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

//! Étape 6 du fil rouge : charger un portefeuille depuis un CSV, sans panic.
//!
//! Format attendu (en-tête obligatoire en première ligne) :
//! `id,counterparty,instrument,strike,direction,quantity`
//! avec `instrument` parmi forward, call, put et `direction` parmi long, short.
//! Les lignes vides et les lignes commençant par `#` sont ignorées.
//!
//! Implémentation de référence, limitée aux notions vues jusqu'au module 6.

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
    value.parse().map_err(|_| ParseError::InvalidNumber {
        field,
        value: value.to_string(),
    })
}

/// Convertit l'identifiant en u32, ou renvoie `InvalidNumber { field: "id", .. }`.
pub fn parse_id(value: &str) -> Result<u32, ParseError> {
    value.parse().map_err(|_| ParseError::InvalidNumber {
        field: "id",
        value: value.to_string(),
    })
}

/// "forward", "call" ou "put" (casse ignorée), avec le strike déjà converti.
pub fn parse_instrument(value: &str, strike: f64) -> Result<Instrument, ParseError> {
    match value.to_lowercase().as_str() {
        "forward" => Ok(Instrument::Forward { strike }),
        "call" => Ok(Instrument::EuropeanOption {
            kind: OptionKind::Call,
            strike,
        }),
        "put" => Ok(Instrument::EuropeanOption {
            kind: OptionKind::Put,
            strike,
        }),
        _ => Err(ParseError::UnknownInstrument(value.to_string())),
    }
}

/// "long" ou "short" (casse ignorée).
pub fn parse_direction(value: &str) -> Result<Direction, ParseError> {
    match value.to_lowercase().as_str() {
        "long" => Ok(Direction::Long),
        "short" => Ok(Direction::Short),
        _ => Err(ParseError::UnknownDirection(value.to_string())),
    }
}

/// Une ligne CSV devient (contrepartie, trade). Les champs sont nettoyés avec `trim`.
pub fn parse_trade(line: &str) -> Result<(String, Trade), ParseError> {
    let mut fields = Vec::with_capacity(FIELD_COUNT);
    for field in line.split(',') {
        fields.push(field.trim());
    }
    if fields.len() != FIELD_COUNT {
        return Err(ParseError::WrongFieldCount {
            expected: FIELD_COUNT,
            found: fields.len(),
        });
    }

    let id = parse_id(fields[0])?;
    let counterparty = fields[1];
    if counterparty.is_empty() {
        return Err(ParseError::EmptyCounterparty);
    }
    let strike = parse_number("strike", fields[3])?;
    let instrument = parse_instrument(fields[2], strike)?;
    let direction = parse_direction(fields[4])?;
    let quantity = parse_number("quantity", fields[5])?;

    let trade = Trade::new(id, instrument, direction, quantity);
    Ok((counterparty.to_string(), trade))
}

/// Charge un CSV complet ; la première erreur rencontrée est renvoyée avec son numéro de ligne.
pub fn load_portfolio(csv: &str) -> Result<Portfolio, LoadError> {
    let mut portfolio = Portfolio::new();
    for (index, line) in csv.lines().enumerate() {
        let line_number = index + 1;
        let trimmed = line.trim();
        if line_number == 1 || trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let (counterparty, trade) = parse_trade(trimmed).map_err(|error| LoadError {
            line: line_number,
            error,
        })?;
        portfolio.add(&counterparty, trade);
    }
    Ok(portfolio)
}
