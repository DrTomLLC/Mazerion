# QA Summary

- Repo: C:\Users\DrTom\RustroverProjects\Mazerion
- Run:  20251224_190213
- PASS: 3
- FAIL: 14
- SKIP: 

| Status | Step | Exit | Seconds |
|---|---|---:|---:|
| PASS | cargo --version | 0 | 0.23 |
| PASS | rustc --version | 0 | 0.13 |
| PASS | rustup show | 0 | 0.1 |
| FAIL | cargo metadata | 101 | 0.13 |
| FAIL | cargo fmt --check | 101 | 0.09 |
| FAIL | cargo check | 101 | 0.08 |
| FAIL | cargo clippy -D warnings | 101 | 0.09 |
| FAIL | cargo build (debug) | 101 | 0.09 |
| FAIL | cargo test (debug, --no-fail-fast) | 101 | 0.08 |
| FAIL | cargo test (release, --no-fail-fast) | 101 | 0.08 |
| FAIL | cargo nextest run | 101 | 0.09 |
| FAIL | cargo llvm-cov (lcov) | 101 | 0.08 |
| FAIL | cargo audit | 101 | 0.08 |
| FAIL | cargo deny check | 101 | 0.08 |
| FAIL | cargo machete | 101 | 0.09 |
| FAIL | cargo geiger | 101 | 0.08 |
| SKIP | PROP HAMMER | 0 | 0 |
| FAIL | LOOM PASS (release) | 101 | 0.08 |
