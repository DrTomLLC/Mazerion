use std::str::FromStr;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Malt/Grain encyclopedia entry
///
/// Stores comprehensive information about malts and grains including
/// color, extract potential, and brewing characteristics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Malt {
    /// Unique identifier (never reused, AUTOINCREMENT)
    pub id: i64,

    /// Malt/grain name (e.g., "Pale Malt 2-Row", "Caramel 60L", "Roasted Barley")
    pub name: String,

    /// Maltster/manufacturer (e.g., "Briess", "Weyermann", "Crisp")
    pub maltster: String,

    /// Country of origin (e.g., "USA", "Germany", "UK")
    pub origin: String,

    /// Grain type: "base", "specialty", "adjunct"
    pub grain_type: String,

    /// Color in degrees Lovibond (typically 1.0-600.0)
    pub color_lovibond: Option<Decimal>,

    /// Maximum percentage in grain bill (0.0-100.0)
    pub max_percentage: Option<Decimal>,

    /// Extract potential as specific gravity (typically 1.032-1.042)
    pub extract_potential: Option<Decimal>,

    /// Diastatic power in degrees Lintner (typically 0-160)
    pub diastatic_power: Option<Decimal>,

    /// Moisture content percentage (typically 3.0-6.0%)
    pub moisture_content: Option<Decimal>,

    /// Protein content percentage (typically 9.0-14.0%)
    pub protein_content: Option<Decimal>,

    /// Flavor profile (professional sommelier vocabulary)
    /// JSON array of flavor descriptors
    pub flavor_profile: Option<String>,

    /// Aroma profile (professional sommelier vocabulary)
    /// JSON array of aroma descriptors
    pub aroma_profile: Option<String>,

    /// Typical usage description
    pub typical_usage: Option<String>,

    /// Substitutes (JSON array of malt names that can substitute)
    pub substitutes: Option<String>,

    /// Best suited beer styles (JSON array)
    pub best_suited_styles: Option<String>,

    /// Usage notes and recommendations
    pub usage_notes: Option<String>,

    /// Professional sensory notes (Master Cicerone level)
    pub sensory_notes: Option<String>,

    /// Requires mashing (true) or can be steeped (false)
    pub requires_mashing: bool,

    /// Compatible beer styles (JSON array)
    pub compatible_styles: Option<String>,

    /// Record creation timestamp
    pub created_at: String,

    /// Record last update timestamp
    pub updated_at: String,
}

impl Malt {
    /// Validate malt data
    pub fn validate(&self) -> Result<(), String> {
        // Name validation
        if self.name.trim().is_empty() {
            return Err("Malt name cannot be empty".to_string());
        }
        if self.name.len() > 100 {
            return Err("Malt name too long (max 100 characters)".to_string());
        }

        // Maltster validation
        if self.maltster.trim().is_empty() {
            return Err("Maltster cannot be empty".to_string());
        }
        if self.maltster.len() > 100 {
            return Err("Maltster too long (max 100 characters)".to_string());
        }

        // Origin validation
        if self.origin.trim().is_empty() {
            return Err("Origin cannot be empty".to_string());
        }
        if self.origin.len() > 100 {
            return Err("Origin too long (max 100 characters)".to_string());
        }

        // Grain type validation
        let valid_types = ["base", "specialty", "adjunct"];
        if !valid_types.contains(&self.grain_type.as_str()) {
            return Err(format!(
                "Invalid grain type '{}', must be one of: {}",
                self.grain_type,
                valid_types.join(", ")
            ));
        }

        // Color validation
        if let Some(color) = self.color_lovibond {
            if color < Decimal::ZERO || color > Decimal::from(700) {
                return Err("Color must be between 0 and 700 Lovibond".to_string());
            }
        }

        // Max percentage validation
        if let Some(max_pct) = self.max_percentage {
            if max_pct < Decimal::ZERO || max_pct > Decimal::from(100) {
                return Err("Max percentage must be between 0% and 100%".to_string());
            }
        }

        // Extract potential validation
        if let Some(extract) = self.extract_potential {
            if extract < Decimal::from_str("1.000").unwrap()
                || extract > Decimal::from_str("1.050").unwrap() {
                return Err("Extract potential must be between 1.000 and 1.050".to_string());
            }
        }

        // Diastatic power validation
        if let Some(dp) = self.diastatic_power {
            if dp < Decimal::ZERO || dp > Decimal::from(200) {
                return Err("Diastatic power must be between 0 and 200 Lintner".to_string());
            }
        }

        // Moisture content validation
        if let Some(moisture) = self.moisture_content {
            if moisture < Decimal::ZERO || moisture > Decimal::from(10) {
                return Err("Moisture content must be between 0% and 10%".to_string());
            }
        }

        // Protein content validation
        if let Some(protein) = self.protein_content {
            if protein < Decimal::ZERO || protein > Decimal::from(20) {
                return Err("Protein content must be between 0% and 20%".to_string());
            }
        }

        Ok(())
    }
}