use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

const MAX_NAME_LEN: usize = 200;
const MAX_CATEGORY_LEN: usize = 50;
const MAX_NOTES_LEN: usize = 5000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BatchStatus {
    Planning,
    Brewing,
    Fermenting,
    Conditioning,
    Aging,
    Bottling,
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
            Self::Aging => "aging",
            Self::Bottling => "bottling",
            Self::Complete => "complete",
            Self::Archived => "archived",
        }
    }
}

impl FromStr for BatchStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "planning" => Ok(Self::Planning),
            "brewing" => Ok(Self::Brewing),
            "fermenting" => Ok(Self::Fermenting),
            "conditioning" => Ok(Self::Conditioning),
            "aging" => Ok(Self::Aging),
            "bottling" => Ok(Self::Bottling),
            "complete" => Ok(Self::Complete),
            "archived" => Ok(Self::Archived),
            _ => Err(format!("Invalid batch status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Batch {
    pub id: i64,
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
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if self.name.len() > MAX_NAME_LEN {
            return Err(format!("Name exceeds {} characters", MAX_NAME_LEN));
        }
        if self.category.is_empty() {
            return Err("Category cannot be empty".to_string());
        }
        if self.category.len() > MAX_CATEGORY_LEN {
            return Err(format!("Category exceeds {} characters", MAX_CATEGORY_LEN));
        }
        if self.batch_size_l <= Decimal::ZERO {
            return Err("Batch size must be positive".to_string());
        }
        if self.batch_size_l > Decimal::from(10000) {
            return Err("Batch size exceeds maximum (10000L)".to_string());
        }

        if let Some(og) = self.target_og {
            if og < Decimal::new(960, 3) || og > Decimal::new(1200, 3) {
                return Err("OG must be between 0.960 and 1.200".to_string());
            }
        }
        if let Some(fg) = self.target_fg {
            if fg < Decimal::new(960, 3) || fg > Decimal::new(1050, 3) {
                return Err("FG must be between 0.960 and 1.050".to_string());
            }
        }
        if let Some(abv) = self.target_abv {
            if abv < Decimal::ZERO || abv > Decimal::from(25) {
                return Err("ABV must be between 0% and 25%".to_string());
            }
        }

        if let Some(ref notes) = self.notes {
            if notes.len() > MAX_NOTES_LEN {
                return Err(format!("Notes exceed {} characters", MAX_NOTES_LEN));
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchReading {
    pub id: i64,
    pub batch_id: i64,
    pub reading_date: String,
    pub gravity: Decimal,
    pub temperature_c: Option<Decimal>,
    pub ph: Option<Decimal>,
    pub notes: Option<String>,
    pub source: String,
}

impl BatchReading {
    pub fn validate(&self) -> Result<(), String> {
        if self.batch_id <= 0 {
            return Err("Invalid batch_id".to_string());
        }
        if self.gravity < Decimal::new(960, 3) || self.gravity > Decimal::new(1200, 3) {
            return Err("Gravity must be between 0.960 and 1.200".to_string());
        }

        if let Some(temp) = self.temperature_c {
            if temp < Decimal::from(-20) || temp > Decimal::from(50) {
                return Err("Temperature must be between -20°C and 50°C".to_string());
            }
        }

        if let Some(ph) = self.ph {
            if ph < Decimal::ZERO || ph > Decimal::from(14) {
                return Err("pH must be between 0 and 14".to_string());
            }
        }

        if let Some(ref notes) = self.notes {
            if notes.len() > MAX_NOTES_LEN {
                return Err(format!("Notes exceed {} characters", MAX_NOTES_LEN));
            }
        }

        if !matches!(self.source.as_str(), "manual" | "tilt" | "ispindel" | "plaato" | "other") {
            return Err("Invalid reading source".to_string());
        }

        Ok(())
    }
}

impl fmt::Display for Batch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}) - {}L, Status: {}",
            self.name,
            self.category,
            self.batch_size_l,
            self.status.as_str()
        )
    }
}