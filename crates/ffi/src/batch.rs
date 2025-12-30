// crates/ffi/src/batch.rs
// Coarse-grained batch operations for battery efficiency

use crate::error::MazerionError;
use crate::types::{BatchCalculatorRequest, BatchCalculatorResult, CalculatorInfo};
use crate::validation;

#[uniffi::export]
pub fn execute_batch(
    requests: Vec<BatchCalculatorRequest>,
) -> Result<Vec<BatchCalculatorResult>, MazerionError> {
    validation::validate_batch_size(requests.len())?;

    let mut results = Vec::with_capacity(requests.len());

    for request in requests {
        let result = match execute_single_in_batch(request.clone()) {
            Ok(calc_result) => BatchCalculatorResult {
                calculator_id: request.calculator_id,
                result: Some(calc_result),
                error: None,
            },
            Err(e) => BatchCalculatorResult {
                calculator_id: request.calculator_id,
                result: None,
                error: Some(e.to_string()),
            },
        };

        results.push(result);
    }

    Ok(results)
}

fn execute_single_in_batch(
    request: BatchCalculatorRequest,
) -> Result<crate::types::CalcResult, MazerionError> {
    validation::validate_calculator_id(&request.calculator_id)?;
    validation::validate_params(&request.params)?;

    let calculator = mazerion_core::traits::get_calculator(&request.calculator_id)
        .ok_or(MazerionError::CalculatorNotFound)?;

    let mut input = mazerion_core::CalcInput::new();
    for param in request.params {
        input = input.add_param(param.key, param.value);
    }

    let result = calculator
        .calculate(input)
        .map_err(MazerionError::from_core_error)?;

    Ok(crate::types::CalcResult::from_core_result(result))
}

#[uniffi::export]
pub fn get_calculators_by_category(
    category: String,
) -> Result<Vec<CalculatorInfo>, MazerionError> {
    validation::check_system_ready()?;

    if category.len() > 50 {
        return Err(MazerionError::InputTooLarge);
    }

    let calculators = mazerion_core::traits::get_all_calculators();

    let filtered: Vec<CalculatorInfo> = calculators
        .iter()
        .filter(|calc| {
            let c: &dyn mazerion_core::traits::Calculator = calc.as_ref();
            c.category() == category
        })
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

    Ok(filtered)
}