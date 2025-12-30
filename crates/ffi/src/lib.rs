// crates/ffi/src/lib.rs
// UniFFI Bridge for Mazerion — Version 0.30.0
// This file implements the interface defined in mazerion.udl
// All operations are safe: no panics, full Result propagation

uniffi::setup_scaffolding!();

use mazerion_core::{
    get_calculator_registry,
    traits::Calculator,  // Adjust if your Calculator trait is in a different module
    types::{CalcInput, CalcOutput},  // Adjust if paths differ
};
use std::collections::HashMap;

// ──────────────────────────────────────────────────────────────
// Custom Error Type — Exposed to foreign languages as flat error
// ──────────────────────────────────────────────────────────────
#[derive(Debug, thiserror::Error, uniffi::Error)]
#[uniffi(flat)]  // Important: flat error for simple string variants
pub enum MazerionError {
    #[error("Invalid input provided")]
    InvalidInput,

    #[error("Calculator not found")]
    CalculatorNotFound,

    #[error("Calculation failed")]
    CalculationFailed,

    #[error("Database error")]
    DatabaseError,

    #[error("Internal error")]
    InternalError,
}

// ──────────────────────────────────────────────────────────────
// Records exposed to Kotlin
// ──────────────────────────────────────────────────────────────
#[derive(uniffi::Record)]
pub struct CalculatorInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
}

#[derive(uniffi::Record)]
pub struct CalcParams {
    pub key: String,
    pub value: String,
}

#[derive(uniffi::Record)]
pub struct CalcResult {
    pub display_text: String,
    pub details: HashMap<String, String>,
}

// ──────────────────────────────────────────────────────────────
// Exported Functions (sync — matching current UDL)
// ──────────────────────────────────────────────────────────────

#[uniffi::export]
pub fn version() -> String {
    "0.30.0".to_string()
}

#[uniffi::export]
pub fn list_calculators() -> Result<Vec<CalculatorInfo>, MazerionError> {
    let registry = get_calculator_registry();

    let infos: Vec<CalculatorInfo> = registry
        .iter()  // Assuming registry has .iter() returning (&str, &dyn Calculator)
        .map(|(id, calc)| CalculatorInfo {
            id: id.to_string(),
            name: calc.name().to_string(),
            description: calc.description().to_string(),
            category: calc.category().to_string(),
        })
        .collect();

    Ok(infos)
}

#[uniffi::export]
pub fn execute_calculator(
    calculator_id: String,
    params: Vec<CalcParams>,
) -> Result<CalcResult, MazerionError> {
    let registry = get_calculator_registry();

    let calculator = registry
        .get(&calculator_id)
        .ok_or(MazerionError::CalculatorNotFound)?;

    let mut input_map: HashMap<String, String> = HashMap::with_capacity(params.len());
    for p in params {
        input_map.insert(p.key, p.value);
    }

    let calc_input = calculator
        .parse_input(&input_map)
        .map_err(|_| MazerionError::InvalidInput)?;

    let calc_output = calculator
        .calculate(&calc_input)
        .map_err(|_| MazerionError::CalculationFailed)?;

    Ok(CalcResult {
        display_text: calc_output.display().to_string(),
        details: calc_output.to_details_map(),
    })
}