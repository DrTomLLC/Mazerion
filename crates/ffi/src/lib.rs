// crates/ffi/src/lib.rs
// UniFFI Bridge for Mazerion v0.30.0
// Zero panics, full Result propagation, coarse-grained batch operations

uniffi::setup_scaffolding!();

mod error;
mod types;
mod validation;
mod batch;

pub use error::MazerionError;
pub use types::*;
pub use batch::*;

// ══════════════════════════════════════════════════════════════════════════════
// SINGLE OPERATIONS
// ══════════════════════════════════════════════════════════════════════════════

#[uniffi::export]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[uniffi::export]
pub fn list_calculators() -> Result<Vec<CalculatorInfo>, MazerionError> {
    validation::check_system_ready()?;

    let calculators = mazerion_core::traits::get_all_calculators();

    let infos: Vec<CalculatorInfo> = calculators
        .iter()
        .map(|calc| {
            let c: &dyn mazerion_core::traits::Calculator = calc.as_ref();
            CalculatorInfo {
                id: c.id().to_string(),
                name: c.name().to_string(),
                description: c.description().to_string(),
                category: c.category().to_string(),
            }
        })
        .collect();

    Ok(infos)
}

#[uniffi::export]
pub fn get_categories() -> Result<CategoryMap, MazerionError> {
    validation::check_system_ready()?;

    let calculators = mazerion_core::traits::get_all_calculators();
    let mut categories: std::collections::HashMap<String, u32> = std::collections::HashMap::new();

    for calc in calculators {
        let c: &dyn mazerion_core::traits::Calculator = calc.as_ref();
        *categories.entry(c.category().to_string()).or_insert(0) += 1;
    }

    let entries = categories
        .into_iter()
        .map(|(category, count)| CategoryEntry { category, count })
        .collect();

    Ok(CategoryMap { entries })
}

#[uniffi::export]
pub fn execute_calculator(
    calculator_id: String,
    params: Vec<CalcParam>,
) -> Result<CalcResult, MazerionError> {
    validation::validate_calculator_id(&calculator_id)?;
    validation::validate_params(&params)?;

    let calculator = mazerion_core::traits::get_calculator(&calculator_id)
        .ok_or(MazerionError::CalculatorNotFound)?;

    let mut input = mazerion_core::CalcInput::new();
    for param in params {
        input = input.add_param(param.key, param.value);
    }

    let result = calculator
        .calculate(input)
        .map_err(MazerionError::from_core_error)?;

    Ok(CalcResult::from_core_result(result))
}