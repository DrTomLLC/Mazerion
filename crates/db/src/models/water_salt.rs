use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterSalt {
    pub id: i64,
    pub name: String,
    pub salt_type: String,
    pub chemical_formula: Option<String>,
    pub manufacturer: Option<String>,

    // Salt composition (TEXT for Decimal precision)
    pub calcium_contribution: Option<Decimal>,
    pub magnesium_contribution: Option<Decimal>,
    pub sodium_contribution: Option<Decimal>,
    pub chloride_contribution: Option<Decimal>,
    pub sulfate_contribution: Option<Decimal>,
    pub bicarbonate_contribution: Option<Decimal>,

    // Usage characteristics
    pub typical_dosage_grams_per_gallon: Option<Decimal>,
    pub solubility: Option<String>,

    // Professional guidance
    pub usage_notes: Option<String>,
    pub flavor_impact: Option<String>,
    pub best_suited_styles: Option<String>,
    pub compatible_styles: Option<String>,

    // Metadata
    pub created_at: String,
    pub updated_at: String,
}

impl WaterSalt {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if self.name.len() > 200 {
            return Err("Name too long (max 200 characters)".to_string());
        }

        let valid_types = ["gypsum", "calcium_chloride", "epsom", "baking_soda", "chalk", "table_salt", "other"];
        if !valid_types.contains(&self.salt_type.as_str()) {
            return Err(format!("Invalid salt type '{}', must be one of: {}",
                               self.salt_type, valid_types.join(", ")));
        }

        // Validate mineral contributions (0-500 ppm per gram is reasonable)
        if let Some(ca) = self.calcium_contribution {
            if ca < Decimal::ZERO || ca > Decimal::from(500) {
                return Err("Calcium contribution must be between 0 and 500 ppm/g".to_string());
            }
        }

        if let Some(mg) = self.magnesium_contribution {
            if mg < Decimal::ZERO || mg > Decimal::from(500) {
                return Err("Magnesium contribution must be between 0 and 500 ppm/g".to_string());
            }
        }

        if let Some(na) = self.sodium_contribution {
            if na < Decimal::ZERO || na > Decimal::from(500) {
                return Err("Sodium contribution must be between 0 and 500 ppm/g".to_string());
            }
        }

        if let Some(cl) = self.chloride_contribution {
            if cl < Decimal::ZERO || cl > Decimal::from(500) {
                return Err("Chloride contribution must be between 0 and 500 ppm/g".to_string());
            }
        }

        if let Some(so4) = self.sulfate_contribution {
            if so4 < Decimal::ZERO || so4 > Decimal::from(500) {
                return Err("Sulfate contribution must be between 0 and 500 ppm/g".to_string());
            }
        }

        if let Some(hco3) = self.bicarbonate_contribution {
            if hco3 < Decimal::ZERO || hco3 > Decimal::from(500) {
                return Err("Bicarbonate contribution must be between 0 and 500 ppm/g".to_string());
            }
        }

        // Validate dosage (0-10 g/gal typical)
        if let Some(dose) = self.typical_dosage_grams_per_gallon {
            if dose < Decimal::ZERO || dose > Decimal::from(10) {
                return Err("Typical dosage must be between 0 and 10 g/gal".to_string());
            }
        }

        Ok(())
    }
}