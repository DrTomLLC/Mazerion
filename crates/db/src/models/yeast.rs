use rust_decimal::Decimal;

#[derive(Debug, Clone)]
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
    pub nutrient_requirements: String,
    pub flocculation: String,
    pub flavor_profile: String,
    pub best_suited_styles: String,
    pub usage_notes: String,
    pub aroma_profile: String,
    pub lag_time_hours: Option<Decimal>,
    pub sensory_notes: String,
    pub requires_rehydration: i64,
    pub compatible_ingredients: String,
    pub created_at: String,
    pub updated_at: String,
    pub fermentation_duration_days: Option<Decimal>,
    pub notes: String,
}

impl Yeast {
    pub fn new(name: String, laboratory: String, product_code: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: None,
            name,
            laboratory,
            product_code,
            yeast_type: String::new(),
            yeast_form: String::new(),
            alcohol_tolerance: None,
            temperature_range_min: None,
            temperature_range_max: None,
            attenuation: None,
            nutrient_requirements: String::new(),
            flocculation: String::new(),
            flavor_profile: String::new(),
            best_suited_styles: String::new(),
            usage_notes: String::new(),
            aroma_profile: String::new(),
            lag_time_hours: None,
            sensory_notes: String::new(),
            requires_rehydration: 0,
            compatible_ingredients: String::new(),
            created_at: now.clone(),
            updated_at: now,
            fermentation_duration_days: None,
            notes: String::new(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        Ok(())
    }
}