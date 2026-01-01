use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct Hop {
    pub id: Option<i64>,
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
    pub flavor_profile: String,
    pub aroma_profile: String,
    pub substitutes: String,
    pub best_suited_styles: String,
    pub usage_notes: String,
    pub sensory_notes: String,
    pub typical_usage: String,
    pub storage_stability: String,
    pub compatible_styles: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Hop {
    pub fn new(name: String, origin: String, hop_type: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: None,
            name,
            origin,
            hop_type,
            alpha_acid: None,
            beta_acid: None,
            cohumulone: None,
            total_oil: None,
            myrcene: None,
            humulene: None,
            caryophyllene: None,
            farnesene: None,
            flavor_profile: String::new(),
            aroma_profile: String::new(),
            substitutes: String::new(),
            best_suited_styles: String::new(),
            usage_notes: String::new(),
            sensory_notes: String::new(),
            typical_usage: String::new(),
            storage_stability: String::new(),
            compatible_styles: String::new(),
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        Ok(())
    }
}