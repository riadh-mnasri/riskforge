// Copyright (c) 2026 Riadh MNASRI. Tous droits réservés.

use riskforge::exposure::{expected_exposure, max_exposure, shift};

const STEPS_PER_YEAR: u32 = 12;

fn main() {
    // Étape 1 : le contexte de la simulation.
    let counterparty = "ACME Bank";
    let scenarios = 10_000;
    let horizon_years = 5;
    let time_steps = horizon_years * STEPS_PER_YEAR;

    println!("riskforge");
    println!("Counterparty: {counterparty}");
    println!("Simulating {scenarios} scenarios over {horizon_years} years");
    println!("Time grid: {time_steps} dates");

    // Étapes 2 et 3 : premiers indicateurs sur quelques scénarios codés en dur.
    let mut mtms = vec![1_200.0, -300.0, 450.0, 0.0, -800.0];
    println!("EE = {:.2}", expected_exposure(&mtms));
    println!("max = {:.2}", max_exposure(&mtms));

    shift(&mut mtms, 200.0);
    println!("EE after +200 shock = {:.2}", expected_exposure(&mtms));
}
