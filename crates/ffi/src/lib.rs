// path: crates/ffi/src/lib.rs

//! JSON-oriented boundary for external callers (Android/KMP/WebAssembly).
//!
//! Design goals:
//! - No `unwrap`, no `expect`, no panics in library code.
//! - All failures are reported via `FfiError`.
//! - Wire format is JSON-in / JSON-out for maximum language interoperability.
//!
//! Typical usage from another language (via JNI, C, etc.):
//!
//! 1. Call `list_calculators_json()` to discover available calculators.
//! 2. Build a JSON request matching `RunCalculatorRequest`.
//! 3. Call `run_calculator_json(request_json)` to get a JSON response.
//!
//! This crate is intentionally thin and delegates all real work to
//! `mazerion-api` + the existing calculator registry.

use mazerion_api::{ApiEngine, ApiError, CalcInput, CalcResult};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Public error type for JSON-facing APIs.
#[derive(Debug, Error)]
pub enum FfiError {
    /// Error originating from the higher-level API.
    #[error("API error: {0}")]
    Api(String),

    /// Error originating from JSON (de)serialization.
    #[error("serialization error: {0}")]
    Serialization(String),
}

pub type FfiResult<T> = Result<T, FfiError>;

/// JSON payload for "list calculators".
///
/// Minimal on purpose: external callers typically only need IDs and maybe
/// user-facing labels. If you need richer metadata later, you can extend this.
#[derive(Debug, Clone, Serialize)]
pub struct ListCalculatorsResponse {
    pub calculators: Vec<String>,
}

/// JSON payload for "run calculator".
///
/// External callers must send this shape as JSON text to `run_calculator_json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunCalculatorRequest {
    pub calculator_id: String,
    pub input: CalcInput,
}

/// JSON payload for "run calculator" response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunCalculatorResponse {
    pub result: CalcResult,
}

/// Ensure the calculator registry is fully initialised.
///
/// It is safe to call this multiple times.
pub fn initialize() {
    // This function is idempotent: calling it multiple times is harmless.
    mazerion_calculators::init();
}

/// Internal helper to construct an engine after initialising calculators.
///
/// We keep this private so that external callers always go through the
/// JSON-oriented functions defined below.
fn engine() -> ApiEngine {
    initialize();
    ApiEngine::new()
}

/// List calculators as a JSON string.
///
/// The returned JSON has the shape:
///
/// ```json
/// {
///   "calculators": ["abv", "dilution", "backsweetening", ...]
/// }
/// ```
pub fn list_calculators_json() -> FfiResult<String> {
    let api = engine();
    let ids = api.list_calculators();

    let payload = ListCalculatorsResponse { calculators: ids };

    serde_json::to_string(&payload).map_err(|e| FfiError::Serialization(e.to_string()))
}

/// Run a calculator given a JSON request, returning a JSON response.
///
/// The `request_json` string must match `RunCalculatorRequest`, e.g.:
///
/// ```json
/// {
///   "calculator_id": "abv",
///   "input": {
///     "params": {
///       "og": "1.100",
///       "fg": "1.010"
///     },
///     "measurements": []
///   }
/// }
/// ```
///
/// On success, the returned JSON matches `RunCalculatorResponse`.
pub fn run_calculator_json(request_json: &str) -> FfiResult<String> {
    let request: RunCalculatorRequest =
        match serde_json::from_str(request_json) {
            Ok(req) => req,
            Err(err) => return Err(FfiError::Serialization(err.to_string())),
        };

    let api = engine();

    let result = match api.run_calculator(&request.calculator_id, request.input) {
        Ok(res) => res,
        Err(err) => return Err(FfiError::Api(format_api_error(&err))),
    };

    let response = RunCalculatorResponse { result };

    serde_json::to_string(&response).map_err(|e| FfiError::Serialization(e.to_string()))
}

/// Convert `ApiError` into a stable string.
///
/// We do not expose the enum variant structure here so that the FFI surface
/// remains simple and language-agnostic.
fn format_api_error(err: &ApiError) -> String {
    err.to_string()
}
