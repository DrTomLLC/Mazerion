pub use mazerion_core::{CalcInput, CalcResult, Error as CoreError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Calculator not found: {0}")]
    CalculatorNotFound(String),

    #[error("Calculation error: {0}")]
    CalculationError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequest {
    pub calculator_id: String,
    pub params: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub value: String,
    pub unit: String,
    pub warnings: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl From<CoreError> for ApiError {
    fn from(err: CoreError) -> Self {
        match err {
            CoreError::Validation(msg) => ApiError::InvalidInput(msg),
            CoreError::OutOfRange(msg) => ApiError::InvalidInput(msg),
            CoreError::MissingInput(msg) => ApiError::InvalidInput(msg),
            CoreError::Calculation(msg) => ApiError::CalculationError(msg),
            CoreError::Parse(msg) => ApiError::InvalidInput(msg),
            CoreError::Io(msg) => ApiError::CalculationError(format!("IO error: {}", msg)),
            CoreError::Config(msg) => ApiError::CalculationError(format!("Config error: {}", msg)),
            CoreError::DatabaseError(msg) => {
                ApiError::CalculationError(format!("Database error: {}", msg))
            }
        }
    }
}

pub fn execute_calculation(request: ApiRequest) -> Result<ApiResponse, ApiError> {
    let calculator = mazerion_core::traits::get_calculator(&request.calculator_id)
        .ok_or_else(|| ApiError::CalculatorNotFound(request.calculator_id.clone()))?;

    let mut input = CalcInput::new();
    for (key, value) in request.params {
        input = input.add_param(key, value);
    }

    let result: CalcResult = calculator.calculate(input)?;

    let metadata: HashMap<String, String> = result.metadata.into_iter().collect();

    Ok(ApiResponse {
        value: result.output.value.to_string(),
        unit: result.output.unit.to_string(),
        warnings: result.warnings,
        metadata,
    })
}

pub fn list_calculators() -> Vec<CalculatorInfo> {
    mazerion_core::traits::get_all_calculators()
        .iter()
        .map(|calc| CalculatorInfo {
            id: calc.id().to_string(),
            name: calc.name().to_string(),
            description: calc.description().to_string(),
            category: calc.category().to_string(),
        })
        .collect()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculatorInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
}
