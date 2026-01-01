use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;

const MAX_TYPE_LEN: usize = 50;
const MAX_NAME_LEN: usize = 200;
const MAX_UNIT_LEN: usize = 20;
const MAX_LOCATION_LEN: usize = 200;
const MAX_NOTES_LEN: usize = 5000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: i64,
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
    pub fn validate(&self) -> Result<(), String> {
        if self.item_type.is_empty() {
            return Err("Item type cannot be empty".to_string());
        }
        if self.item_type.len() > MAX_TYPE_LEN {
            return Err(format!("Item type exceeds {} characters", MAX_TYPE_LEN));
        }

        if self.item_name.is_empty() {
            return Err("Item name cannot be empty".to_string());
        }
        if self.item_name.len() > MAX_NAME_LEN {
            return Err(format!("Item name exceeds {} characters", MAX_NAME_LEN));
        }

        if self.quantity < Decimal::ZERO {
            return Err("Quantity cannot be negative".to_string());
        }
        if self.quantity > Decimal::from(1000000) {
            return Err("Quantity exceeds maximum (1000000)".to_string());
        }

        if self.unit.is_empty() {
            return Err("Unit cannot be empty".to_string());
        }
        if self.unit.len() > MAX_UNIT_LEN {
            return Err(format!("Unit exceeds {} characters", MAX_UNIT_LEN));
        }

        if let Some(ref loc) = self.location {
            if loc.len() > MAX_LOCATION_LEN {
                return Err(format!("Location exceeds {} characters", MAX_LOCATION_LEN));
            }
        }

        if let Some(cost) = self.cost {
            if cost < Decimal::ZERO {
                return Err("Cost cannot be negative".to_string());
            }
            if cost > Decimal::from(1000000) {
                return Err("Cost exceeds maximum (1000000)".to_string());
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

impl fmt::Display for InventoryItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {} {} ({})",
            self.item_name,
            self.quantity,
            self.unit,
            self.item_type
        )
    }
}