// path: crates/api/src/lib.rs

//! Stable, front-facing API for Mazerion.
//!
//! This crate provides a small, typed interface around the existing
//! `mazerion-core` calculator registry so that frontends (CLI, TUI, GUI,
//! voice, Android/KMP, WebAssembly, etc.) can depend on this crate and avoid
//! knowledge of internal calculator wiring.

use mazerion_core::{self, Error as CoreError};
use thiserror::Error;

/// Re-export core types that frontends almost always need.
///
/// This lets applications depend on `mazerion-api` only.
pub use mazerion_core::{CalcInput, CalcResult, Measurement, Unit};

/// High-level API errors surfaced to frontends.
///
/// We wrap the core error type so callers can pattern match on it if desired,
/// but also provide a simple `Display` implementation via `thiserror`.
#[derive(Debug, Error, Clone)]
pub enum ApiError {
    /// Calculator with the given ID does not exist.
    #[error("calculator not found: {0}")]
    NotFound(String),

    /// Underlying calculator returned an error.
    #[error(transparent)]
    Core(#[from] CoreError),
}

/// Convenience result type for the API.
pub type ApiResult<T> = Result<T, ApiError>;

/// High-level engine façade.
///
/// Currently this is a unit-like type with no state, but using a struct gives
/// us the flexibility to add configuration later without breaking callers.
#[derive(Debug, Default, Clone, Copy)]
pub struct ApiEngine;

impl ApiEngine {
    /// Create a new API engine.
    ///
    /// NOTE: Callers must ensure that `mazerion_calculators::init()` has been
    /// invoked somewhere in the binary so that the calculator registry is
    /// populated. This mirrors how the existing CLI does it.
    pub fn new() -> Self {
        Self
    }

    /// List all calculators with their IDs as owned `String`s.
    pub fn list_calculators(&self) -> Vec<String> {
        mazerion_core::traits::list_calculators()
            .into_iter()
            .map(|id| id.to_owned())
            .collect()
    }

    /// List calculator IDs as `&'static str` (direct view into the registry).
    pub fn list_calculator_ids(&self) -> Vec<&'static str> {
        mazerion_core::traits::list_calculators()
    }

    /// Run a single calculator by ID with the given input.
    pub fn run_calculator(&self, id: &str, input: CalcInput) -> ApiResult<CalcResult> {
        let calc = mazerion_core::traits::get_calculator(id)
            .ok_or_else(|| ApiError::NotFound(id.to_owned()))?;

        let result = calc.calculate(input)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ApiEngine, CalcInput, Unit};
    use rust_decimal::Decimal;

    #[test]
    fn api_lists_calculators_and_contains_abv() {
        // Ensure registry is initialised
        mazerion_calculators::init();

        let api = ApiEngine::new();
        let ids = api.list_calculator_ids();

        assert!(
            !ids.is_empty(),
            "expected at least one calculator registered"
        );
        assert!(
            ids.contains(&"abv"),
            "expected ABV calculator to be registered"
        );
    }

    #[test]
    fn api_runs_abv_calculator_correctly() {
        mazerion_calculators::init();
        let api = ApiEngine::new();

        // Your CalcInput API: new() + add_param(k, v)
        let input = CalcInput::new()
            .add_param("og", "1.100")
            .add_param("fg", "1.010");

        let result = api
            .run_calculator("abv", input)
            .expect("ABV calculation should succeed");

        // Check that the unit is ABV
        assert_eq!(
            result.output.unit,
            Unit::Abv,
            "expected ABV unit as output"
        );

        // ABV = (OG - FG) * 131.25 = (1.100 - 1.010) * 131.25 = 11.8125
        let val = result.output.value;

        let og = Decimal::new(1100, 3); // 1.100
        let fg = Decimal::new(1010, 3); // 1.010
        let expected = (og - fg) * Decimal::new(13125, 2); // 131.25

        assert_eq!(
            val, expected,
            "unexpected ABV value, got {val}, expected {expected}"
        );
    }
}
