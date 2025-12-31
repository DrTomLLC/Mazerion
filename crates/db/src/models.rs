// Database models with validation

use mazerion_core::{Error, Result};
use rust_decimal::Decimal;
use std::str::FromStr;

// ══════════════════════════════════════════════════════════════════════════════
// BATCH MODELS
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct Batch {
    pub id: Option<i64>,
    pub name: String,
    pub recipe_id: Option<i64>,
    pub category: String,
    pub batch_size_l: Decimal,
    pub brew_date: String,
    pub target_og: Option<Decimal>,
    pub target_fg: Option<Decimal>,
    pub target_abv: Option<Decimal>,
    pub status: BatchStatus,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Batch {
    pub fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() || self.name.len() > 200 {
            return Err(Error::Validation("Invalid batch name".into()));
        }

        if self.category.trim().is_empty() || self.category.len() > 50 {
            return Err(Error::Validation("Invalid category".into()));
        }

        if self.batch_size_l <= Decimal::ZERO || self.batch_size_l > Decimal::from(10000) {
            return Err(Error::Validation("Batch size must be between 0 and 10000L".into()));
        }

        if let Some(ref og) = self.target_og {
            let min = Decimal::from_str_exact("0.900").map_err(|_| Error::Validation("Invalid OG min".into()))?;
            let max = Decimal::from_str_exact("2.000").map_err(|_| Error::Validation("Invalid OG max".into()))?;
            if *og < min || *og > max {
                return Err(Error::Validation("OG must be between 0.900 and 2.000".into()));
            }
        }

        if let Some(ref fg) = self.target_fg {
            let min = Decimal::from_str_exact("0.900").map_err(|_| Error::Validation("Invalid FG min".into()))?;
            let max = Decimal::from_str_exact("2.000").map_err(|_| Error::Validation("Invalid FG max".into()))?;
            if *fg < min || *fg > max {
                return Err(Error::Validation("FG must be between 0.900 and 2.000".into()));
            }
        }

        if let Some(ref abv) = self.target_abv {
            if *abv < Decimal::ZERO || *abv > Decimal::from(30) {
                return Err(Error::Validation("ABV must be between 0% and 30%".into()));
            }
        }

        if let Some(ref notes) = self.notes
            && notes.len() > 5000
        {
            return Err(Error::Validation("Notes too long (max 5000 chars)".into()));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchStatus {
    Planning,
    Brewing,
    Fermenting,
    Conditioning,
    Bottled,
    Kegged,
    Complete,
    Archived,
}

impl BatchStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Planning => "planning",
            Self::Brewing => "brewing",
            Self::Fermenting => "fermenting",
            Self::Conditioning => "conditioning",
            Self::Bottled => "bottled",
            Self::Kegged => "kegged",
            Self::Complete => "complete",
            Self::Archived => "archived",
        }
    }
}

impl FromStr for BatchStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "planning" => Ok(Self::Planning),
            "brewing" => Ok(Self::Brewing),
            "fermenting" => Ok(Self::Fermenting),
            "conditioning" => Ok(Self::Conditioning),
            "bottled" => Ok(Self::Bottled),
            "kegged" => Ok(Self::Kegged),
            "complete" => Ok(Self::Complete),
            "archived" => Ok(Self::Archived),
            _ => Err(Error::Validation(format!("Invalid status: {}", s))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BatchReading {
    pub id: Option<i64>,
    pub batch_id: i64,
    pub reading_date: String,
    pub gravity: Decimal,
    pub temperature_c: Option<Decimal>,
    pub ph: Option<Decimal>,
    pub notes: Option<String>,
    pub source: String,
}

impl BatchReading {
    pub fn validate(&self) -> Result<()> {
        let min = Decimal::from_str_exact("0.900").map_err(|_| Error::Validation("Invalid gravity min".into()))?;
        let max = Decimal::from_str_exact("2.000").map_err(|_| Error::Validation("Invalid gravity max".into()))?;

        if self.gravity < min || self.gravity > max {
            return Err(Error::Validation("Gravity must be between 0.900 and 2.000".into()));
        }

        if let Some(ref temp) = self.temperature_c {
            if *temp < Decimal::from(-20) || *temp > Decimal::from(100) {
                return Err(Error::Validation("Temperature out of range".into()));
            }
        }

        if let Some(ref ph_val) = self.ph {
            if *ph_val < Decimal::ZERO || *ph_val > Decimal::from(14) {
                return Err(Error::Validation("pH must be between 0 and 14".into()));
            }
        }

        Ok(())
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// INVENTORY MODELS
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct InventoryItem {
    pub id: Option<i64>,
    pub item_type: String,
    pub item_name: String,
    pub quantity: Decimal,
    pub unit: String,
    pub location: Option<String>,
    pub purchase_date: Option<String>,
    pub expiration_date: Option<String>,
    pub cost: Option<Decimal>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl InventoryItem {
    pub fn validate(&self) -> Result<()> {
        if self.item_name.trim().is_empty() || self.item_name.len() > 200 {
            return Err(Error::Validation("Invalid item name".into()));
        }

        if self.item_type.trim().is_empty() || self.item_type.len() > 50 {
            return Err(Error::Validation("Invalid item type".into()));
        }

        if self.quantity < Decimal::ZERO || self.quantity > Decimal::from(1000000) {
            return Err(Error::Validation("Quantity out of range".into()));
        }

        if self.unit.trim().is_empty() || self.unit.len() > 20 {
            return Err(Error::Validation("Invalid unit".into()));
        }

        Ok(())
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// RECIPE MODELS
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct Recipe {
    pub id: Option<i64>,
    pub name: String,
    pub category: String,
    pub subcategory: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub source: Option<String>,
    pub difficulty: Option<String>,
    pub batch_size_l: Decimal,
    pub target_og: Option<Decimal>,
    pub target_fg: Option<Decimal>,
    pub target_abv: Option<Decimal>,
    pub created_at: String,
    pub updated_at: String,
}

impl Recipe {
    pub fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() || self.name.len() > 200 {
            return Err(Error::Validation("Invalid recipe name".into()));
        }

        if self.category.trim().is_empty() || self.category.len() > 50 {
            return Err(Error::Validation("Invalid category".into()));
        }

        if self.batch_size_l <= Decimal::ZERO || self.batch_size_l > Decimal::from(10000) {
            return Err(Error::Validation("Batch size must be between 0 and 10000L".into()));
        }

        if let Some(ref og) = self.target_og {
            let min = Decimal::from_str_exact("0.900").map_err(|_| Error::Validation("Invalid OG min".into()))?;
            let max = Decimal::from_str_exact("2.000").map_err(|_| Error::Validation("Invalid OG max".into()))?;
            if *og < min || *og > max {
                return Err(Error::Validation("OG must be between 0.900 and 2.000".into()));
            }
        }

        if let Some(ref fg) = self.target_fg {
            let min = Decimal::from_str_exact("0.900").map_err(|_| Error::Validation("Invalid FG min".into()))?;
            let max = Decimal::from_str_exact("2.000").map_err(|_| Error::Validation("Invalid FG max".into()))?;
            if *fg < min || *fg > max {
                return Err(Error::Validation("FG must be between 0.900 and 2.000".into()));
            }
        }

        if let Some(ref abv) = self.target_abv {
            if *abv < Decimal::ZERO || *abv > Decimal::from(30) {
                return Err(Error::Validation("ABV must be between 0% and 30%".into()));
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RecipeIngredient {
    pub id: Option<i64>,
    pub recipe_id: i64,
    pub ingredient_type: String,
    pub ingredient_name: String,
    pub amount: Decimal,
    pub unit: String,
    pub timing: Option<String>,
    pub notes: Option<String>,
}

impl RecipeIngredient {
    pub fn validate(&self) -> Result<()> {
        if self.ingredient_name.trim().is_empty() || self.ingredient_name.len() > 200 {
            return Err(Error::Validation("Invalid ingredient name".into()));
        }

        if self.amount <= Decimal::ZERO || self.amount > Decimal::from(1000000) {
            return Err(Error::Validation("Amount must be positive and reasonable".into()));
        }

        if self.unit.trim().is_empty() || self.unit.len() > 20 {
            return Err(Error::Validation("Invalid unit".into()));
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RecipeInstruction {
    pub id: Option<i64>,
    pub recipe_id: i64,
    pub step_number: i32,
    pub instruction: String,
    pub duration_minutes: Option<i32>,
    pub temperature_c: Option<Decimal>,
}

impl RecipeInstruction {
    pub fn validate(&self) -> Result<()> {
        if self.instruction.trim().is_empty() || self.instruction.len() > 2000 {
            return Err(Error::Validation("Invalid instruction".into()));
        }

        if self.step_number < 1 {
            return Err(Error::Validation("Step number must be positive".into()));
        }

        if let Some(duration) = self.duration_minutes {
            if duration < 0 || duration > 100000 {
                return Err(Error::Validation("Duration out of range".into()));
            }
        }

        if let Some(ref temp) = self.temperature_c {
            if *temp < Decimal::from(-20) || *temp > Decimal::from(200) {
                return Err(Error::Validation("Temperature out of range".into()));
            }
        }

        Ok(())
    }

}
// ══════════════════════════════════════════════════════════════════════════════
// ENCYCLOPEDIA MODELS
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct YeastStrain {
    pub id: i64,
    pub name: String,
    pub laboratory: Option<String>,
    pub attenuation_min: Option<Decimal>,
    pub attenuation_max: Option<Decimal>,
    pub temp_min_c: Option<Decimal>,
    pub temp_max_c: Option<Decimal>,
    pub alcohol_tolerance: Option<Decimal>,
    pub flocculation: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HopVariety {
    pub id: i64,
    pub name: String,
    pub alpha_acid_min: Option<Decimal>,
    pub alpha_acid_max: Option<Decimal>,
    pub beta_acid_min: Option<Decimal>,
    pub beta_acid_max: Option<Decimal>,
    pub cohumulone: Option<Decimal>,
    pub aroma_profile: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BjcpStyle {
    pub id: i64,
    pub category: String,
    pub subcategory: Option<String>,
    pub style_name: String,
    pub og_min: Option<Decimal>,
    pub og_max: Option<Decimal>,
    pub fg_min: Option<Decimal>,
    pub fg_max: Option<Decimal>,
    pub abv_min: Option<Decimal>,
    pub abv_max: Option<Decimal>,
    pub ibu_min: Option<Decimal>,
    pub ibu_max: Option<Decimal>,
    pub srm_min: Option<Decimal>,
    pub srm_max: Option<Decimal>,
    pub description: Option<String>,
}

// ══════════════════════════════════════════════════════════════════════════════
// CALCULATION LOG MODEL
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct CalculationLog {
    pub id: Option<i64>,
    pub calculator_id: String,
    pub inputs: String,  // JSON
    pub result: String,  // JSON
    pub timestamp: String,
}

impl CalculationLog {
    pub fn validate(&self) -> Result<()> {
        if self.calculator_id.trim().is_empty() || self.calculator_id.len() > 100 {
            return Err(Error::Validation("Invalid calculator ID".into()));
        }

        if self.inputs.len() > 10000 {
            return Err(Error::Validation("Inputs too large".into()));
        }

        if self.result.len() > 10000 {
            return Err(Error::Validation("Result too large".into()));
        }

        Ok(())
    }
}