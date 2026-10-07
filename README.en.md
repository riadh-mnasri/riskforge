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
| 4 | Structs, enums | Domain model: Trade, NettingSet | upcoming |
| 5 | Error handling | CSV loading without panics | upcoming |
| 6 | Traits and generics | GBM, Hull-White diffusion models | upcoming |
| 7 | Closures and iterators | EE, 97.5% PFE, EPE | upcoming |
| 8 | Tests and benchmarks | criterion, comparison with Kotlin | upcoming |
| 9 | Concurrency | rayon | upcoming |
| 10 | FFI | Called from Kotlin, JVM versus Rust benchmark | upcoming |

## Requirements

Stable Rust via [rustup](https://rustup.rs) (2024 edition). No external dependency yet, no environment variable.

## License

© 2026 Riadh MNASRI. All rights reserved.
