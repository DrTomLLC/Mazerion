use std::str::FromStr;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Honey varietal encyclopedia entry
///
/// Stores comprehensive information about honey varieties including floral source,
/// flavor profiles, sugar composition, and usage in brewing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Honey {
    /// Unique identifier (never reused, AUTOINCREMENT)
    pub id: i64,

    /// Honey varietal name (e.g., "Orange Blossom", "Wildflower", "Buckwheat")
    pub name: String,

    /// Floral source (e.g., "Citrus sinensis", "Mixed wildflowers")
    pub floral_source: String,

    /// Geographic origin/region (e.g., "California", "New Zealand", "Italian Alps")
    pub origin: Option<String>,

    /// Honey color: "extra white", "white", "extra light amber", "light amber",
    /// "amber", "dark amber", "dark"
    pub color: String,

    /// Moisture content percentage (typical 15.0-20.0)
    pub moisture_content: Option<Decimal>,

    /// Average fructose percentage (typically 35-45%)
    pub fructose_percentage: Option<Decimal>,

    /// Average glucose percentage (typically 25-35%)
    pub glucose_percentage: Option<Decimal>,

    /// Other sugars percentage (sucrose, maltose, etc.)
    pub other_sugars_percentage: Option<Decimal>,

    /// Specific gravity (typically 1.410-1.450)
    pub specific_gravity: Option<Decimal>,

    /// pH level (typically 3.5-4.5)
    pub ph: Option<Decimal>,

    /// Flavor intensity: "delicate", "mild", "moderate", "strong", "robust"
    pub flavor_intensity: String,

    /// Flavor profile (professional sommelier vocabulary)
    /// JSON array of flavor descriptors
    pub flavor_profile: Option<String>,

    /// Aroma profile (professional sommelier vocabulary)
    /// JSON array of aroma descriptors
    pub aroma_profile: Option<String>,

    /// Crystallization tendency: "rapid", "moderate", "slow", "very slow"
    pub crystallization_tendency: Option<String>,

    /// Best suited beverage styles (JSON array)
    pub best_suited_styles: Option<String>,

    /// Usage notes and recommendations
    pub usage_notes: Option<String>,

    /// Professional sensory notes (Master Sommelier level)
    pub sensory_notes: Option<String>,

    /// Typical harvest season (e.g., "Spring", "Summer", "Fall")
    pub harvest_season: Option<String>,

    /// Monofloral or multifloral
    pub is_monofloral: bool,

    /// Raw/unpasteurized or processed
    pub is_raw: Option<bool>,

    /// Compatible yeast types (JSON array)
    pub compatible_yeasts: Option<String>,

    /// Record creation timestamp
    pub created_at: String,

    /// Record last update timestamp
    pub updated_at: String,
}

impl Honey {
    /// Validate honey data
    ///
    /// Ensures all fields meet professional brewing standards and safety requirements.
    pub fn validate(&self) -> Result<(), String> {
        // Name validation
        if self.name.trim().is_empty() {
            return Err("Honey name cannot be empty".to_string());
        }
        if self.name.len() > 200 {
            return Err("Honey name too long (max 200 characters)".to_string());
        }

        // Floral source validation
        if self.floral_source.trim().is_empty() {
            return Err("Floral source cannot be empty".to_string());
        }
        if self.floral_source.len() > 200 {
            return Err("Floral source too long (max 200 characters)".to_string());
        }

        // Origin validation
        if let Some(ref origin) = self.origin {
            if origin.len() > 100 {
                return Err("Origin too long (max 100 characters)".to_string());
            }
        }

        // Color validation
        let valid_colors = [
            "extra white", "white", "extra light amber", "light amber",
            "amber", "dark amber", "dark"
        ];
        if !valid_colors.contains(&self.color.as_str()) {
            return Err(format!(
                "Invalid honey color '{}', must be one of: {}",
                self.color,
                valid_colors.join(", ")
            ));
        }

        // Moisture content validation
        if let Some(moisture) = self.moisture_content {
            if moisture < Decimal::ZERO || moisture > Decimal::from(25) {
                return Err("Moisture content must be between 0% and 25%".to_string());
            }
        }

        // Fructose validation
        if let Some(fructose) = self.fructose_percentage {
            if fructose < Decimal::ZERO || fructose > Decimal::from(100) {
                return Err("Fructose percentage must be between 0% and 100%".to_string());
            }
        }

        // Glucose validation
        if let Some(glucose) = self.glucose_percentage {
            if glucose < Decimal::ZERO || glucose > Decimal::from(100) {
                return Err("Glucose percentage must be between 0% and 100%".to_string());
            }
        }

        // Other sugars validation
        if let Some(other) = self.other_sugars_percentage {
            if other < Decimal::ZERO || other > Decimal::from(100) {
                return Err("Other sugars percentage must be between 0% and 100%".to_string());
            }
        }

        // Sugar totals validation (if all present)
        if let (Some(fructose), Some(glucose), Some(other)) = (
            self.fructose_percentage,
            self.glucose_percentage,
            self.other_sugars_percentage,
        ) {
            let total = fructose + glucose + other;
            if total > Decimal::from(100) {
                return Err("Total sugar percentages cannot exceed 100%".to_string());
            }
        }

        // Specific gravity validation
        if let Some(sg) = self.specific_gravity {
            if sg < Decimal::from_str("1.000").unwrap() || sg > Decimal::from_str("1.600").unwrap() {
                return Err("Specific gravity must be between 1.000 and 1.600".to_string());
            }
        }

        // pH validation
        if let Some(ph) = self.ph {
            if ph < Decimal::from(2) || ph > Decimal::from(7) {
                return Err("pH must be between 2.0 and 7.0".to_string());
            }
        }

        // Flavor intensity validation
        let valid_intensities = ["delicate", "mild", "moderate", "strong", "robust"];
        if !valid_intensities.contains(&self.flavor_intensity.as_str()) {
            return Err(format!(
                "Invalid flavor intensity '{}', must be one of: {}",
                self.flavor_intensity,
                valid_intensities.join(", ")
            ));
        }

        // Crystallization tendency validation
        if let Some(ref cryst) = self.crystallization_tendency {
            let valid_cryst = ["rapid", "moderate", "slow", "very slow"];
            if !valid_cryst.contains(&cryst.as_str()) {
                return Err(format!(
                    "Invalid crystallization tendency '{}', must be one of: {}",
                    cryst,
                    valid_cryst.join(", ")
                ));
            }
        }

        // Harvest season validation
        if let Some(ref season) = self.harvest_season {
            if season.len() > 50 {
                return Err("Harvest season too long (max 50 characters)".to_string());
            }
        }

        Ok(())
    }
}