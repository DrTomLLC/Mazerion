use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;

const MAX_NAME_LEN: usize = 100;
const MAX_ORIGIN_LEN: usize = 100;
const MAX_JSON_LEN: usize = 1000;
const MAX_NOTES_LEN: usize = 5000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hop {
    pub id: i64,
    pub name: String,
    pub origin: String,
    pub hop_type: String,
    pub alpha_acid: Option<Decimal>,
    pub beta_acid: Option<Decimal>,
    pub cohumulone: Option<Decimal>,
    pub total_oil: Option<Decimal>,
    pub myrcene: Option<Decimal>,
    pub humulene: Option<Decimal>,
    pub caryophyllene: Option<Decimal>,
    pub farnesene: Option<Decimal>,
    pub flavor_profile: Option<String>,
    pub aroma_profile: Option<String>,
    pub substitutes: Option<String>,
    pub best_suited_styles: Option<String>,
    pub usage_notes: Option<String>,
    pub sensory_notes: Option<String>,
    pub typical_usage: Option<String>,
    pub storage_stability: Option<String>,
    pub compatible_styles: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Hop {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if self.name.len() > MAX_NAME_LEN {
            return Err(format!("Name exceeds {} characters", MAX_NAME_LEN));
        }

        if self.origin.is_empty() {
            return Err("Origin cannot be empty".to_string());
        }
        if self.origin.len() > MAX_ORIGIN_LEN {
            return Err(format!("Origin exceeds {} characters", MAX_ORIGIN_LEN));
        }

        if !matches!(self.hop_type.as_str(), "bittering" | "aroma" | "dual-purpose") {
            return Err("Invalid hop type".to_string());
        }

        if let Some(alpha) = self.alpha_acid {
            if alpha < Decimal::ZERO || alpha > Decimal::from(25) {
                return Err("Alpha acid must be 0-25%".to_string());
            }
        }
        if let Some(beta) = self.beta_acid {
            if beta < Decimal::ZERO || beta > Decimal::from(15) {
                return Err("Beta acid must be 0-15%".to_string());
            }
        }
        if let Some(cohum) = self.cohumulone {
            if cohum < Decimal::ZERO || cohum > Decimal::from(100) {
                return Err("Cohumulone must be 0-100%".to_string());
            }
        }

        if let Some(total) = self.total_oil {
            if total < Decimal::ZERO || total > Decimal::from(5) {
                return Err("Total oil must be 0-5 mL/100g".to_string());
            }
        }

        let mut oil_total = Decimal::ZERO;
        if let Some(m) = self.myrcene {
            if m < Decimal::ZERO || m > Decimal::from(100) {
                return Err("Myrcene must be 0-100%".to_string());
            }
            oil_total += m;
        }
        if let Some(h) = self.humulene {
            if h < Decimal::ZERO || h > Decimal::from(100) {
                return Err("Humulene must be 0-100%".to_string());
            }
            oil_total += h;
        }
        if let Some(c) = self.caryophyllene {
            if c < Decimal::ZERO || c > Decimal::from(100) {
                return Err("Caryophyllene must be 0-100%".to_string());
            }
            oil_total += c;
        }
        if let Some(f) = self.farnesene {
            if f < Decimal::ZERO || f > Decimal::from(100) {
                return Err("Farnesene must be 0-100%".to_string());
            }
            oil_total += f;
        }

        if oil_total > Decimal::from(100) {
            return Err("Sum of oil percentages cannot exceed 100%".to_string());
        }

        if let Some(ref s) = self.flavor_profile {
            if s.len() > MAX_JSON_LEN {
                return Err(format!("Flavor profile exceeds {} characters", MAX_JSON_LEN));
            }
        }
        if let Some(ref s) = self.aroma_profile {
            if s.len() > MAX_JSON_LEN {
                return Err(format!("Aroma profile exceeds {} characters", MAX_JSON_LEN));
            }
        }
        if let Some(ref s) = self.substitutes {
            if s.len() > MAX_JSON_LEN {
                return Err(format!("Substitutes exceeds {} characters", MAX_JSON_LEN));
            }
        }
        if let Some(ref s) = self.best_suited_styles {
            if s.len() > MAX_JSON_LEN {
                return Err(format!("Best suited styles exceeds {} characters", MAX_JSON_LEN));
            }
        }
        if let Some(ref s) = self.compatible_styles {
            if s.len() > MAX_JSON_LEN {
                return Err(format!("Compatible styles exceeds {} characters", MAX_JSON_LEN));
            }
        }
        if let Some(ref s) = self.usage_notes {
            if s.len() > MAX_NOTES_LEN {
                return Err(format!("Usage notes exceed {} characters", MAX_NOTES_LEN));
            }
        }
        if let Some(ref s) = self.sensory_notes {
            if s.len() > MAX_NOTES_LEN {
                return Err(format!("Sensory notes exceed {} characters", MAX_NOTES_LEN));
            }
        }

        if let Some(ref usage) = self.typical_usage {
            if !matches!(usage.as_str(), "bittering" | "aroma" | "dry hop" | "first wort" | "whirlpool") {
                return Err("Invalid typical usage value".to_string());
            }
        }

        if let Some(ref stability) = self.storage_stability {
            if !matches!(stability.as_str(), "excellent" | "good" | "fair" | "poor") {
                return Err("Invalid storage stability value".to_string());
            }
        }

        Ok(())
    }
}

impl fmt::Display for Hop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}, {}) - Alpha: {:.1}%, Type: {}",
            self.name,
            self.origin,
            self.hop_type,
            self.alpha_acid.unwrap_or(Decimal::ZERO),
            self.hop_type
        )
    }
}