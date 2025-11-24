//! Calculator trait and registry system.

use crate::{CalcInput, CalcResult, Error, Result};

/// Calculator trait for all computation modules.
pub trait Calculator: Send + Sync {
    /// Unique identifier.
    fn id(&self) -> &'static str;

    /// Display name.
    fn name(&self) -> &'static str;

    /// Description.
    fn description(&self) -> &'static str;

    /// Category for organization.
    fn category(&self) -> &'static str;

    /// Detailed help text.
    fn help_text(&self) -> &'static str;

    /// Perform calculation.
    fn calculate(&self, input: CalcInput) -> Result<CalcResult>;

    /// Validate inputs before calculation.
    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.measurements.is_empty() {
            return Err(Error::MissingInput("No measurements provided".into()));
        }
        Ok(())
    }
}

/// Calculator registry entry.
pub struct CalculatorEntry {
    pub id: &'static str,
    pub factory: fn() -> Box<dyn Calculator>,
}

impl CalculatorEntry {
    pub const fn new(id: &'static str, factory: fn() -> Box<dyn Calculator>) -> Self {
        Self { id, factory }
    }
}

/// Inventory of all calculators.
#[linkme::distributed_slice]
pub static CALCULATORS: [CalculatorEntry];

/// Get calculator by ID.
pub fn get_calculator(id: &str) -> Option<Box<dyn Calculator>> {
    CALCULATORS
        .iter()
        .find(|e| e.id == id)
        .map(|e| (e.factory)())
}

/// Get all calculators.
pub fn get_all_calculators() -> Vec<Box<dyn Calculator>> {
    CALCULATORS
        .iter()
        .map(|e| (e.factory)())
        .collect()
}

/// List all calculator IDs.
pub fn list_calculators() -> Vec<&'static str> {
    CALCULATORS.iter().map(|e| e.id).collect()
}

/// Macro to register a calculator.
#[macro_export]
macro_rules! register_calculator {
    ($calc:ty) => {
        #[::linkme::distributed_slice($crate::traits::CALCULATORS)]
        static ENTRY: $crate::traits::CalculatorEntry = $crate::traits::CalculatorEntry::new(
            <$calc>::ID,
            || Box::new(<$calc>::default()),
        );
    };
}