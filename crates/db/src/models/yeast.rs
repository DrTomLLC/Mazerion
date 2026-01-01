use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

const MAX_NAME_LEN: usize = 200;
const MAX_LAB_LEN: usize = 100;
const MAX_CODE_LEN: usize = 50;
const MAX_JSON_LEN: usize = 1000;
const MAX_NOTES_LEN: usize = 5000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Yeast {
    pub id: Option<i64>,
    pub name: String,
    pub laboratory: String,
    pub product_code: String,
    pub yeast_type: String,
    pub yeast_form: String,
    pub alcohol_tolerance: Option<Decimal>,
    pub temperature_range_min: Option<Decimal>,
    pub temperature_range_max: Option<Decimal>,
    pub attenuation: Option<Decimal>,
    pub flocculation: String,
    pub nutrient_requirements: String,
    pub flavor_profile: String,
    pub aroma_profile: String,
    pub best_suited_styles: String,
    pub usage_notes: String,
    pub lag_time_hours: Option<Decimal>,
    pub fermentation_duration_days: Option<Decimal>,
    pub sensory_notes: String,
    pub notes: String,
    pub requires_rehydration: i32,
    pub compatible_ingredients: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Yeast {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if self.name.len() > MAX_NAME_LEN {
            return Err(format!("Name exceeds {} characters", MAX_NAME_LEN));
        }
        if self.laboratory.is_empty() {
            return Err("Laboratory cannot be empty".to_string());
        }
        if self.laboratory.len() > MAX_LAB_LEN {
            return Err(format!("Laboratory exceeds {} characters", MAX_LAB_LEN));
        }
        if self.product_code.is_empty() {
            return Err("Product code cannot be empty".to_string());
        }
        if self.product_code.len() > MAX_CODE_LEN {
            return Err(format!("Product code exceeds {} characters", MAX_CODE_LEN));
        }
        if !matches!(self.yeast_type.as_str(), "wine" | "beer" | "mead" | "champagne" | "distillers" | "wild") {
            return Err("Invalid yeast type".to_string());
        }
        if !matches!(self.yeast_form.as_str(), "dry" | "liquid" | "slant" | "plate") {
            return Err("Invalid yeast form".to_string());
        }

        if let Some(tol) = self.alcohol_tolerance {
            if tol < Decimal::ZERO || tol > Decimal::from(25) {
                return Err("Alcohol tolerance must be 0-25%".to_string());
            }
        }
        if let Some(temp_min) = self.temperature_range_min {
            if temp_min < Decimal::from(32) || temp_min > Decimal::from(120) {
                return Err("Temperature min must be 32-120°F".to_string());
            }
        }
        if let Some(temp_max) = self.temperature_range_max {
            if temp_max < Decimal::from(32) || temp_max > Decimal::from(120) {
                return Err("Temperature max must be 32-120°F".to_string());
            }
        }
        if let Some(att) = self.attenuation {
            if att < Decimal::ZERO || att > Decimal::from(100) {
                return Err("Attenuation must be 0-100%".to_string());
            }
        }

        if !matches!(self.flocculation.as_str(), "" | "low" | "medium" | "high" | "very high") {
            return Err("Invalid flocculation value".to_string());
        }
        if !matches!(self.nutrient_requirements.as_str(), "" | "low" | "moderate" | "high") {
            return Err("Invalid nutrient requirements value".to_string());
        }

        if self.flavor_profile.len() > MAX_JSON_LEN {
            return Err(format!("Flavor profile exceeds {} characters", MAX_JSON_LEN));
        }
        if self.aroma_profile.len() > MAX_JSON_LEN {
            return Err(format!("Aroma profile exceeds {} characters", MAX_JSON_LEN));
        }
        if self.best_suited_styles.len() > MAX_JSON_LEN {
            return Err(format!("Best suited styles exceeds {} characters", MAX_JSON_LEN));
        }
        if self.compatible_ingredients.len() > MAX_JSON_LEN {
            return Err(format!("Compatible ingredients exceeds {} characters", MAX_JSON_LEN));
        }
        if self.usage_notes.len() > MAX_NOTES_LEN {
            return Err(format!("Usage notes exceed {} characters", MAX_NOTES_LEN));
        }
        if self.sensory_notes.len() > MAX_NOTES_LEN {
            return Err(format!("Sensory notes exceed {} characters", MAX_NOTES_LEN));
        }
        if self.notes.len() > MAX_NOTES_LEN {
            return Err(format!("Notes exceed {} characters", MAX_NOTES_LEN));
        }

        if !matches!(self.requires_rehydration, 0 | 1) {
            return Err("Requires rehydration must be 0 or 1".to_string());
        }

        Ok(())
    }
}