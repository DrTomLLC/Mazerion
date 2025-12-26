# QA Summary

- Repo: C:\Users\DrTom\RustroverProjects\Mazerion
- Run:  20251223_181627
- PASS: 21
- FAIL: 17
- SKIP: 3

| Status | Step | Exit | Seconds |
|---|---|---:|---:|
| FAIL | Install (winget) tamasfe.taplo | 9999 | 0.07 |
| FAIL | Install (winget) Crate-CI.Typos | 9999 | 0 |
| FAIL | Install (winget) Gitleaks.Gitleaks | 9999 | 0 |
| FAIL | Install (winget) Google.OSVScanner | 9999 | 0 |
| FAIL | Install (winget) Anchore.Syft | 9999 | 0 |
| FAIL | Install (winget) Anchore.Grype | 9999 | 0 |
| FAIL | Install (pip) semgrep | 9999 | 0 |
| FAIL | Install (cargo binstall) cargo-deadlinks | 9999 | 0 |
| FAIL | Install (cargo binstall) cargo-spellcheck | 9999 | 0.01 |
| PASS | cargo --list | 0 | 0.16 |
| PASS | cargo metadata --locked | 0 | 0.15 |
| PASS | cargo fmt --check | 0 | 0.15 |
| PASS | cargo check (workspace all-targets all-features) | 0 | 0.16 |
| PASS | cargo clippy (-D warnings) | 0 | 0.14 |
| PASS | cargo build (debug) | 0 | 0.21 |
| PASS | cargo build (release) | 0 | 0.11 |
| PASS | cargo doc (no-deps) | 0 | 0.14 |
| PASS | cargo test (debug, --no-fail-fast) | 0 | 0.11 |
| PASS | cargo test (release, --no-fail-fast) | 0 | 0.13 |
| PASS | cargo nextest run | 0 | 0.09 |
| PASS | cargo llvm-cov (lcov) | 0 | 0.13 |
| PASS | cargo audit | 0 | 0.12 |
| PASS | cargo deny check | 0 | 0.11 |
| PASS | cargo machete | 0 | 0.14 |
| PASS | cargo geiger | 0 | 0.1 |
| PASS | cargo udeps (best effort) | 0 | 0.09 |
| PASS | cargo msrv verify | 0 | 0.12 |
| PASS | cargo semver-checks | 0 | 0.09 |
| FAIL | Install (cargo binstall) cargo-deadlinks | 9999 | 0 |
| SKIP | cargo deadlinks | 127 | 0 |
| FAIL | Install (cargo binstall) cargo-spellcheck | 9999 | 0 |
| SKIP | cargo spellcheck | 127 | 0 |
| PASS | cargo mutants (slow) | 0 | 0.1 |
| SKIP | PROPTEST/QUICKCHECK HAMMER | 0 | 0 |
| PASS | LOOM PASS (release, test-threads=1, target-dir=target/loom) | 0 | 0.18 |
| FAIL | WSL info | 9999 | 0.01 |
| FAIL | WSL apt deps (root, best effort) | 9999 | 0 |
| FAIL | WSL rustup/toolchains (best effort) | 9999 | 0 |
| FAIL | WSL LOOM PASS (release) | 9999 | 0 |
| FAIL | WSL Kani install + run (best effort) | 9999 | 0 |
| FAIL | WSL ASan test (best effort) | 9999 | 0 |
