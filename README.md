# riskforge

Moteur Monte Carlo de risque de contrepartie écrit en Rust, construit étape par étape en suivant le parcours [RustPrimer](https://rust-primer.vercel.app). À terme : profils d'exposition (EE, PFE, EPE), CVA, parallélisation avec rayon et appel depuis Kotlin via l'API FFM.

*Read this in English: [README.en.md](README.en.md)*

## Comment l'utiliser

Ce dépôt est un cahier d'exercices :

- la branche `main` contient les tests et des fonctions à compléter (`todo!()`) ;
- la branche `solutions` contient une implémentation de référence, à consulter une fois ton code écrit.

```bash
cargo test          # rouge au départ : remplace les todo!() jusqu'au vert
cargo run           # affiche le contexte et les premiers indicateurs
cargo clippy        # conseils idiomatiques
git diff main solutions -- src/   # comparer avec la solution
```

## Étapes

| # | Module RustPrimer | Contenu | État |
|---|---|---|---|
| 1 | Prise en main | Crate Cargo, contexte de simulation | ✅ |
| 2 | Les bases du langage | `exposure`, `expected_exposure`, `max_exposure` | ✅ |
| 3 | Ownership et emprunts | `shift` (&mut [f64]), `exposure_profile` | ✅ |
| 4 | Structs, enums | Modèle métier : Trade, NettingSet | à venir |
| 5 | Gestion des erreurs | Chargement CSV sans panic | à venir |
| 6 | Traits et génériques | Modèles de diffusion GBM, Hull-White | à venir |
| 7 | Closures et itérateurs | EE, PFE 97,5 %, EPE | à venir |
| 8 | Tests et benchmarks | criterion, comparaison avec Kotlin | à venir |
| 9 | Concurrence | rayon | à venir |
| 10 | FFI | Appel depuis Kotlin, benchmark JVM contre Rust | à venir |

## Prérequis

Rust stable via [rustup](https://rustup.rs) (édition 2024). Aucune dépendance externe pour l'instant, aucune variable d'environnement.

## Licence

© 2026 Riadh MNASRI. Tous droits réservés.
