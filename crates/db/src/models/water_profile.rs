use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterProfile {
    pub id: i64,
    pub name: String,
    pub water_type: String,
    pub source: Option<String>,
    pub location: Option<String>,

    // Mineral content (TEXT for Decimal precision) - ppm values
    pub calcium: Option<Decimal>,
    pub magnesium: Option<Decimal>,
    pub sodium: Option<Decimal>,
    pub chloride: Option<Decimal>,
    pub sulfate: Option<Decimal>,
    pub bicarbonate: Option<Decimal>,

    // Water characteristics
    pub ph_level: Option<Decimal>,
    pub total_dissolved_solids: Option<Decimal>,
    pub hardness: Option<Decimal>,

    // Usage recommendations
    pub best_suited_styles: Option<String>,
    pub usage_notes: Option<String>,
    pub flavor_impact: Option<String>,
    pub compatible_styles: Option<String>,

    // Metadata
    pub created_at: String,
    pub updated_at: String,
}

impl WaterProfile {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if self.name.len() > 200 {
            return Err("Name too long (max 200 characters)".to_string());
        }

        let valid_types = ["tap", "distilled", "spring", "RO", "mineral", "other"];
        if !valid_types.contains(&self.water_type.as_str()) {
            return Err(format!("Invalid water type '{}', must be one of: {}",
                               self.water_type, valid_types.join(", ")));
        }

        // Validate mineral ranges (0-1000 ppm is reasonable for brewing)
        if let Some(ca) = self.calcium {
            if ca < Decimal::ZERO || ca > Decimal::from(1000) {
                return Err("Calcium must be between 0 and 1000 ppm".to_string());
            }
        }

        if let Some(mg) = self.magnesium {
            if mg < Decimal::ZERO || mg > Decimal::from(1000) {
                return Err("Magnesium must be between 0 and 1000 ppm".to_string());
            }
        }

        if let Some(na) = self.sodium {
            if na < Decimal::ZERO || na > Decimal::from(1000) {
                return Err("Sodium must be between 0 and 1000 ppm".to_string());
            }
        }

        if let Some(cl) = self.chloride {
            if cl < Decimal::ZERO || cl > Decimal::from(1000) {
                return Err("Chloride must be between 0 and 1000 ppm".to_string());
            }
        }

        if let Some(so4) = self.sulfate {
            if so4 < Decimal::ZERO || so4 > Decimal::from(1000) {
                return Err("Sulfate must be between 0 and 1000 ppm".to_string());
            }
        }

        if let Some(hco3) = self.bicarbonate {
            if hco3 < Decimal::ZERO || hco3 > Decimal::from(1000) {
                return Err("Bicarbonate must be between 0 and 1000 ppm".to_string());
            }
        }

        // Validate pH (brewing range 5.0-9.0)
        if let Some(ph) = self.ph_level {
            if ph < Decimal::from(5) || ph > Decimal::from(9) {
                return Err("pH must be between 5.0 and 9.0".to_string());
            }
        }

        // Validate TDS (0-2000 ppm typical)
        if let Some(tds) = self.total_dissolved_solids {
            if tds < Decimal::ZERO || tds > Decimal::from(2000) {
                return Err("TDS must be between 0 and 2000 ppm".to_string());
            }
        }

        // Validate hardness (0-500 ppm typical)
        if let Some(hard) = self.hardness {
            if hard < Decimal::ZERO || hard > Decimal::from(500) {
                return Err("Hardness must be between 0 and 500 ppm".to_string());
            }
        }

        Ok(())
    }
}