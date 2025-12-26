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

    /// Category for organization (must be one of VALID_CATEGORIES).
    fn category(&self) -> &'static str;

    /// Perform calculation.
    fn calculate(&self, input: CalcInput) -> Result<CalcResult>;

    /// Validate inputs before calculation.
    fn validate(&self, input: &CalcInput) -> Result<()> {
        if input.measurements.is_empty() && input.params.is_empty() {
            return Err(Error::MissingInput(
                "No measurements or parameters provided".into(),
            ));
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

/// Inventory of all calculators (linkme distributed slice).
#[linkme::distributed_slice]
pub static CALCULATORS: [CalculatorEntry];

/// Get calculator by ID.
pub fn get_calculator(id: &str) -> Option<Box<dyn Calculator>> {
    CALCULATORS
        .iter()
        .find(|e| e.id == id)
        .map(|e| (e.factory)())
}

/// Get all registered calculators.
pub fn get_all_calculators() -> Vec<Box<dyn Calculator>> {
    CALCULATORS.iter().map(|e| (e.factory)()).collect()
}

/// List all calculator IDs.
pub fn list_calculator_ids() -> Vec<&'static str> {
    CALCULATORS.iter().map(|e| e.id).collect()
}

/// Get total count of registered calculators.
pub fn calculator_count() -> usize {
    CALCULATORS.len()
}

/// Macro to register a calculator at compile-time.
///
/// # Example
///
/// ```
/// use mazerion_core::{Calculator, CalcInput, CalcResult, Result, register_calculator};
///
/// #[derive(Default)]
/// struct MyCalculator;
///
/// impl MyCalculator {
///     pub const ID: &'static str = "my_calc";
/// }
///
/// impl Calculator for MyCalculator {
///     fn id(&self) -> &'static str { Self::ID }
///     fn name(&self) -> &'static str { "My Calculator" }
///     fn description(&self) -> &'static str { "Does something" }
///     fn category(&self) -> &'static str { "Basic" }
///     fn calculate(&self, _input: CalcInput) -> Result<CalcResult> {
///         unimplemented!()
///     }
/// }
///
/// register_calculator!(MyCalculator);
/// ```
#[macro_export]
macro_rules! register_calculator {
    ($calc:ty) => {
        #[::linkme::distributed_slice($crate::traits::CALCULATORS)]
        static ENTRY: $crate::traits::CalculatorEntry =
            $crate::traits::CalculatorEntry::new(<$calc>::ID, || Box::new(<$calc>::default()));
    };
}
