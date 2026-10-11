// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

use riskforge::loader::{
    LoadError, ParseError, load_portfolio, parse_direction, parse_id, parse_instrument,
    parse_number, parse_trade,
};
use riskforge::model::{Direction, Instrument, OptionKind, Trade};

const HEADER: &str = "id,counterparty,instrument,strike,direction,quantity";

#[test]
fn parse_number_accepts_a_decimal_value() {
    // Given / When / Then
    assert_eq!(parse_number("strike", "101.5"), Ok(101.5));
}

#[test]
fn parse_number_names_the_faulty_field() {
    // Given / When
    let result = parse_number("strike", "abc");
    // Then
    assert_eq!(
        result,
        Err(ParseError::InvalidNumber {
            field: "strike",
            value: "abc".to_string()
        })
    );
}

#[test]
fn parse_id_rejects_a_negative_identifier() {
    // Given / When
    let result = parse_id("-3");
    // Then
    assert_eq!(
        result,
        Err(ParseError::InvalidNumber {
            field: "id",
            value: "-3".to_string()
        })
    );
}

#[test]
fn parse_instrument_ignores_case() {
    // Given / When / Then
    assert_eq!(
        parse_instrument("Forward", 100.0),
        Ok(Instrument::Forward { strike: 100.0 })
    );
    assert_eq!(
        parse_instrument("CALL", 120.0),
        Ok(Instrument::EuropeanOption {
            kind: OptionKind::Call,
            strike: 120.0
        })
    );
    assert_eq!(
        parse_instrument("put", 95.0),
        Ok(Instrument::EuropeanOption {
            kind: OptionKind::Put,
            strike: 95.0
        })
    );
}

#[test]
fn parse_instrument_rejects_an_unknown_type() {
    // Given / When / Then
    assert_eq!(
        parse_instrument("swap", 100.0),
        Err(ParseError::UnknownInstrument("swap".to_string()))
    );
}

#[test]
fn parse_direction_reads_long_and_short() {
    // Given / When / Then
    assert_eq!(parse_direction("LONG"), Ok(Direction::Long));
    assert_eq!(parse_direction("short"), Ok(Direction::Short));
    assert_eq!(
        parse_direction("buy"),
        Err(ParseError::UnknownDirection("buy".to_string()))
    );
}

#[test]
fn parse_trade_builds_a_trade_and_trims_fields() {
    // Given
    let line = " 3 , Globex , call , 120 , long , 200 ";
    // When
    let result = parse_trade(line);
    // Then
    let expected = Trade::new(
        3,
        Instrument::EuropeanOption {
            kind: OptionKind::Call,
            strike: 120.0,
        },
        Direction::Long,
        200.0,
    );
    assert_eq!(result, Ok(("Globex".to_string(), expected)));
}

#[test]
fn parse_trade_checks_the_field_count() {
    // Given / When / Then
    assert_eq!(
        parse_trade("1,ACME,forward,100,long"),
        Err(ParseError::WrongFieldCount {
            expected: 6,
            found: 5
        })
    );
}

#[test]
fn parse_trade_rejects_an_empty_counterparty() {
    // Given / When / Then
    assert_eq!(
        parse_trade("1,  ,forward,100,long,1000"),
        Err(ParseError::EmptyCounterparty)
    );
}

#[test]
fn load_portfolio_skips_header_blank_lines_and_comments() {
    // Given
    let csv = format!(
        "{HEADER}\n1,ACME,forward,100,long,1000\n\n# commentaire\n2,ACME,forward,105,short,800\n3,Globex,call,120,long,200\n"
    );
    // When
    let portfolio = load_portfolio(&csv).expect("le CSV est valide");
    // Then
    assert_eq!(portfolio.trade_count(), 3);
    assert_eq!(portfolio.counterparties(), vec!["ACME", "Globex"]);
}

#[test]
fn load_portfolio_reports_the_line_number_of_the_first_error() {
    // Given : l'erreur est en ligne 4 (en-tête et commentaire comptent)
    let csv = format!(
        "{HEADER}\n1,ACME,forward,100,long,1000\n# commentaire\n2,ACME,swap,105,short,800\n3,Globex,call,abc,long,200\n"
    );
    // When
    let result = load_portfolio(&csv);
    // Then
    assert_eq!(
        result.unwrap_err(),
        LoadError {
            line: 4,
            error: ParseError::UnknownInstrument("swap".to_string())
        }
    );
}

#[test]
fn load_error_message_is_readable() {
    // Given
    let error = LoadError {
        line: 4,
        error: ParseError::UnknownInstrument("swap".to_string()),
    };
    // When / Then
    assert_eq!(error.to_string(), "ligne 4 : instrument inconnu : swap");
}

#[test]
fn bundled_portfolio_file_loads_without_error() {
    // Given
    let csv = std::fs::read_to_string("data/portfolio.csv").expect("fichier livré avec le dépôt");
    // When
    let portfolio = load_portfolio(&csv).expect("le fichier d'exemple est valide");
    // Then
    assert_eq!(portfolio.trade_count(), 5);
    assert_eq!(
        portfolio.counterparties(),
        vec!["ACME", "Globex", "Initech"]
    );
}
