// Honey encyclopedia schema

use rusqlite::Connection;
use mazerion_core::{Error, Result};

pub fn create_honey_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS honey_varieties (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,

            -- Provenance & Terroir
            botanical_source TEXT,
            latin_name TEXT,
            origin_region TEXT,
            origin_country TEXT,
            terroir_notes TEXT,
            harvest_elevation TEXT,
            harvest_season TEXT,
            climate_influence TEXT,

            -- Appearance
            color_lovibond TEXT,
            color_description TEXT,
            clarity TEXT,
            viscosity TEXT,
            crystallization_tendency TEXT,
            crystallization_rate TEXT,

            -- Chemical Composition
            moisture_content TEXT,
            fructose_percentage TEXT,
            glucose_percentage TEXT,
            sucrose_percentage TEXT,
            maltose_percentage TEXT,
            ph_range TEXT,
            acidity TEXT,
            enzyme_content TEXT,
            mineral_content TEXT,
            pollen_percentage TEXT,

            -- Sensory Profile (Professional Tasting)
            aroma_intensity TEXT,
            aroma_primary TEXT,
            aroma_secondary TEXT,
            aroma_tertiary TEXT,
            aroma_complexity TEXT,
            flavor_intensity TEXT,
            flavor_primary TEXT,
            flavor_secondary TEXT,
            flavor_tertiary TEXT,
            flavor_complexity TEXT,
            sweetness_level TEXT,
            acidity_perception TEXT,
            bitterness_level TEXT,
            astringency TEXT,
            mouthfeel TEXT,
            finish_length TEXT,
            finish_character TEXT,

            -- Fermentation Properties
            fermentability TEXT,
            typical_gravity_contribution TEXT,
            nutrient_profile TEXT,
            staggered_nutrition_requirements TEXT,

            -- Professional Pairing
            cheese_pairings TEXT,
            meat_pairings TEXT,
            dessert_pairings TEXT,
            wine_pairings TEXT,
            spirit_pairings TEXT,
            beer_pairings TEXT,
            mead_style_pairings TEXT,
            seasonal_pairings TEXT,
            temperature_serving TEXT,

            -- Culinary Applications
            cooking_applications TEXT,
            baking_suitability TEXT,
            marinade_usage TEXT,
            glaze_usage TEXT,
            sauce_usage TEXT,

            -- Quality & Grading
            grade TEXT,
            quality_indicators TEXT,
            adulteration_risks TEXT,
            authenticity_markers TEXT,
            certification TEXT,

            -- Sourcing & Economics
            rarity TEXT,
            price_range TEXT,
            availability TEXT,
            best_producers TEXT,
            sustainable_sourcing TEXT,

            -- Health & Nutrition
            medicinal_properties TEXT,
            antioxidant_content TEXT,
            antibacterial_properties TEXT,
            glycemic_index TEXT,
            nutritional_profile TEXT,
            health_benefits TEXT,
            allergen_info TEXT,

            -- Storage & Aging
            storage_requirements TEXT,
            optimal_storage_temp TEXT,
            aging_potential TEXT,
            aging_changes TEXT,
            shelf_life TEXT,

            -- Professional Notes
            chef_notes TEXT,
            sommelier_notes TEXT,
            mazer_notes TEXT,
            historical_significance TEXT,
            cultural_context TEXT,
            description TEXT,
            tasting_notes TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| Error::DatabaseError(format!("Create honey_varieties: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_honey_origin ON honey_varieties(origin_country)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_honey_botanical ON honey_varieties(botanical_source)",
        [],
    ).map_err(|e| Error::DatabaseError(format!("{}", e)))?;

    Ok(())
}