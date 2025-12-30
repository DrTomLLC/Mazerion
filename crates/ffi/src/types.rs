// crates/ffi/src/types.rs
// FFI-safe types for UniFFI boundary

#[derive(uniffi::Record, Debug)]
pub struct CalculatorInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
}

#[derive(uniffi::Record, Clone, Debug)]
pub struct CalcParam {
    pub key: String,
    pub value: String,
}

#[derive(uniffi::Record, Debug)]
pub struct CalcResult {
    pub value: String,
    pub unit: String,
    pub display_text: String,
    pub warnings: Vec<String>,
    pub metadata: MetadataMap,
}

impl CalcResult {
    pub fn from_core_result(result: mazerion_core::CalcResult) -> Self {
        let metadata = MetadataMap {
            entries: result
                .metadata
                .into_iter()
                .map(|(key, value)| MetadataEntry { key, value })
                .collect(),
        };

        Self {
            value: result.output.value.to_string(),
            unit: result.output.unit.to_string(),
            display_text: format!("{}", result.output),
            warnings: result.warnings,
            metadata,
        }
    }
}

#[derive(uniffi::Record, Clone, Debug)]
pub struct BatchCalculatorRequest {
    pub calculator_id: String,
    pub params: Vec<CalcParam>,
}

#[derive(uniffi::Record, Debug)]
pub struct BatchCalculatorResult {
    pub calculator_id: String,
    pub result: Option<CalcResult>,
    pub error: Option<String>,
}

#[derive(uniffi::Record, Debug)]
pub struct CategoryMap {
    pub entries: Vec<CategoryEntry>,
}

#[derive(uniffi::Record, Debug)]
pub struct CategoryEntry {
    pub category: String,
    pub count: u32,
}

#[derive(uniffi::Record, Debug)]
pub struct MetadataMap {
    pub entries: Vec<MetadataEntry>,
}

#[derive(uniffi::Record, Debug)]
pub struct MetadataEntry {
    pub key: String,
    pub value: String,
}