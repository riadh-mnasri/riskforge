# riskforge

A Monte Carlo counterparty risk engine written in Rust, built step by step while following the [RustPrimer](https://rust-primer.vercel.app) course. Eventually: exposure profiles (EE, PFE, EPE), CVA, parallelization with rayon, and calls from Kotlin through the FFM API.

*Lire en français : [README.md](README.md)*

## How to use it

This repository is a workbook:

- the `main` branch holds the tests and functions to complete (`todo!()`);
- the `solutions` branch holds a reference implementation, to look at once your own code is written.

```bash
cargo test          # red at first: replace the todo!() until green
cargo run           # prints the context and the first metrics
cargo clippy        # idiomatic advice
git diff main solutions -- src/   # compare with the solution
```

## Steps

| # | RustPrimer module | Content | Status |
|---|---|---|---|
| 1 | Getting started | Cargo crate, simulation context | ✅ |
| 2 | Language basics | `exposure`, `expected_exposure`, `max_exposure` | ✅ |
| 3 | Ownership and borrowing | `shift` (&mut [f64]), `exposure_profile` | ✅ |
| 4 | Structs, enums | `Trade`, `Instrument`, `NettingSet` (net and gross) | ✅ |
| 5 | Collections | `Portfolio`: HashMap of netting sets, sorted report | ✅ |
| 6 | Error handling | `loader`: CSV to `Portfolio`, typed and line-numbered errors | ✅ |
| 7 | Traits and generics | `DiffusionModel` trait (GBM, Ornstein-Uhlenbeck), Monte Carlo simulation, EE profile | ✅ |
| 8 | Closures and iterators | EE, 97.5% PFE, EPE | upcoming |
| 9 | Tests and benchmarks | criterion, comparison with Kotlin | upcoming |
| 10 | Concurrency | rayon | upcoming |
| 11 | FFI | Called from Kotlin, JVM versus Rust benchmark | upcoming |

## Provided, not to code

`src/rng.rs`: a deterministic pseudo-random generator (SplitMix64 + Box-Muller). A fixed seed makes simulations reproducible, and therefore testable.

## Requirements

Stable Rust via [rustup](https://rustup.rs) (2024 edition). No external dependency yet, no environment variable.

## License

© 2026 Riadh MNASRI. All rights reserved.
