// Flavor, fermentation, and aging profile schemas

use rusqlite::Connection;
use mazerion_core::Result;

pub fn create_profile_tables(conn: &Connection) -> Result<()> {
    // ══════════════════════════════════════════════════════════════════════
    // FLAVOR PROFILES - Master sensory database for prediction
    // ══════════════════════════════════════════════════════════════════════
    conn.execute(
        "CREATE TABLE IF NOT EXISTS flavor_profiles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,

            -- Source Reference
            source_type TEXT NOT NULL,
            source_id INTEGER NOT NULL,
            source_name TEXT NOT NULL,

            -- Primary Taste Categories (0-10 scale)
            sweetness TEXT,
            sourness TEXT,
            bitterness TEXT,
            saltiness TEXT,
            umami TEXT,
            astringency TEXT,
            heat TEXT,

            -- Aroma Categories (0-10 scale)
            fruity TEXT,
            floral TEXT,
            spicy TEXT,
            herbal TEXT,
            earthy TEXT,
            nutty TEXT,
            caramel TEXT,
            roasted TEXT,
            smoky TEXT,
            woody TEXT,
            resinous TEXT,
            citrus TEXT,
            tropical TEXT,
            berry TEXT,
            stone_fruit TEXT,
            tree_fruit TEXT,
            melon TEXT,

            -- Detailed Flavor Descriptors (0-10 scale)
            vanilla TEXT,
            chocolate TEXT,
            coffee TEXT,
            toffee TEXT,
            honey TEXT,
            maple TEXT,
            molasses TEXT,
            bread TEXT,
            toast TEXT,
            biscuit TEXT,
            cracker TEXT,
            grain TEXT,
            malt TEXT,

            -- Mouthfeel (0-10 scale)
            body TEXT,
            viscosity TEXT,
            carbonation_feel TEXT,
            warmth TEXT,
            cooling TEXT,
            drying TEXT,
            coating TEXT,
            creamy TEXT,
            smooth TEXT,
            rough TEXT,
            tannic TEXT,

            -- Intensity Metrics (0-10 scale)
            overall_intensity TEXT,
            aroma_intensity TEXT,
            flavor_intensity TEXT,
            finish_length TEXT,
            complexity TEXT,

            -- Flavor Vector (JSON for ML/AI prediction)
            flavor_vector TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP,

            UNIQUE(source_type, source_id)
        )",
        [],
    ).map_err(|e| mazerion_core::Error::DatabaseError(format!("Create flavor_profiles: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_flavor_source ON flavor_profiles(source_type, source_id)",
        [],
    ).map_err(|e| mazerion_core::Error::DatabaseError(format!("{}", e)))?;

    // ══════════════════════════════════════════════════════════════════════
    // INGREDIENT INTERACTIONS - Prediction engine
    // ══════════════════════════════════════════════════════════════════════
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ingredient_interactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,

            -- Ingredient Pair
            ingredient_a_type TEXT NOT NULL,
            ingredient_a_id INTEGER NOT NULL,
            ingredient_a_name TEXT NOT NULL,
            ingredient_b_type TEXT NOT NULL,
            ingredient_b_id INTEGER NOT NULL,
            ingredient_b_name TEXT NOT NULL,

            -- Interaction Type
            interaction_type TEXT NOT NULL,
            interaction_strength TEXT,
            synergy_type TEXT,

            -- Synergy Effects
            flavor_synergy TEXT,
            aroma_synergy TEXT,
            creates_new_flavors TEXT,
            amplifies_existing TEXT,
            masks_undesirable TEXT,
            complementary_notes TEXT,

            -- Chemical Interactions
            chemical_reaction TEXT,
            ph_effect TEXT,
            oxidation_effect TEXT,
            enzymatic_effect TEXT,
            polymerization TEXT,

            -- Sensory Prediction
            predicted_flavor_change TEXT,
            predicted_aroma_change TEXT,
            predicted_mouthfeel_change TEXT,
            predicted_color_change TEXT,
            predicted_complexity_change TEXT,

            -- Optimal Ratios
            optimal_ratio_a_to_b TEXT,
            ratio_range_min TEXT,
            ratio_range_max TEXT,
            ratio_notes TEXT,

            -- Timing
            interaction_timing TEXT,
            interaction_duration TEXT,
            time_to_peak_synergy TEXT,

            -- Professional Experience
            chef_recommendations TEXT,
            sommelier_notes TEXT,
            brewmaster_notes TEXT,
            mazer_notes TEXT,

            -- Classic Combinations
            classic_examples TEXT,
            regional_traditions TEXT,
            award_winning_recipes TEXT,

            -- Confidence Score
            data_confidence TEXT,
            number_of_examples TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP,

            UNIQUE(ingredient_a_type, ingredient_a_id, ingredient_b_type, ingredient_b_id)
        )",
        [],
    ).map_err(|e| mazerion_core::Error::DatabaseError(format!("Create ingredient_interactions: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_interaction_a ON ingredient_interactions(ingredient_a_type, ingredient_a_id)",
        [],
    ).map_err(|e| mazerion_core::Error::DatabaseError(format!("{}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_interaction_b ON ingredient_interactions(ingredient_b_type, ingredient_b_id)",
        [],
    ).map_err(|e| mazerion_core::Error::DatabaseError(format!("{}", e)))?;

    // ══════════════════════════════════════════════════════════════════════
    // FERMENTATION PROFILES - Timeline & flavor development prediction
    // ══════════════════════════════════════════════════════════════════════
    conn.execute(
        "CREATE TABLE IF NOT EXISTS fermentation_profiles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,

            -- Base Parameters
            yeast_id INTEGER NOT NULL,
            yeast_name TEXT NOT NULL,
            og_min TEXT,
            og_max TEXT,
            temp_c TEXT,
            nutrient_protocol TEXT,

            -- Timeline Predictions (hours/days)
            lag_phase_hours TEXT,
            primary_fermentation_days TEXT,
            secondary_fermentation_days TEXT,
            total_fermentation_days TEXT,
            peak_activity_day TEXT,

            -- Flavor Development Timeline
            day_1_flavors TEXT,
            day_3_flavors TEXT,
            day_7_flavors TEXT,
            day_14_flavors TEXT,
            day_30_flavors TEXT,
            day_60_flavors TEXT,
            day_90_flavors TEXT,

            -- Chemical Production Timeline
            ester_development_timeline TEXT,
            phenol_development_timeline TEXT,
            fusel_alcohol_timeline TEXT,
            diacetyl_timeline TEXT,
            acetaldehyde_timeline TEXT,

            -- Sensory Evolution
            early_fermentation_aroma TEXT,
            mid_fermentation_aroma TEXT,
            late_fermentation_aroma TEXT,
            post_fermentation_aroma TEXT,
            young_flavor_profile TEXT,
            mature_flavor_profile TEXT,

            -- Gravity Progression
            expected_gravity_day_1 TEXT,
            expected_gravity_day_3 TEXT,
            expected_gravity_day_7 TEXT,
            expected_gravity_day_14 TEXT,
            expected_gravity_final TEXT,

            -- Optimal Timing
            optimal_racking_day TEXT,
            optimal_nutrient_additions TEXT,
            optimal_degassing_day TEXT,
            optimal_cold_crash_day TEXT,
            optimal_packaging_day TEXT,

            -- Temperature Adjustments
            temperature_schedule TEXT,
            diacetyl_rest_timing TEXT,
            cold_conditioning_timing TEXT,

            -- Professional Notes
            brewmaster_timeline_notes TEXT,
            mazer_timeline_notes TEXT,
            common_issues TEXT,
            optimization_tips TEXT,
            troubleshooting TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| mazerion_core::Error::DatabaseError(format!("Create fermentation_profiles: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_ferment_yeast ON fermentation_profiles(yeast_id)",
        [],
    ).map_err(|e| mazerion_core::Error::DatabaseError(format!("{}", e)))?;

    // ══════════════════════════════════════════════════════════════════════
    // AGING PROFILES - Aging prediction engine
    // ══════════════════════════════════════════════════════════════════════
    conn.execute(
        "CREATE TABLE IF NOT EXISTS aging_profiles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,

            -- Beverage Type
            beverage_type TEXT NOT NULL,
            style TEXT,
            base_ingredients TEXT,
            abv_range TEXT,

            -- Aging Potential
            minimum_aging_weeks TEXT,
            optimal_aging_months TEXT,
            peak_aging_months TEXT,
            maximum_aging_years TEXT,
            decline_begins_years TEXT,

            -- Aging Conditions
            optimal_temp_c TEXT,
            temp_range_c TEXT,
            optimal_humidity TEXT,
            light_sensitivity TEXT,
            oxygen_exposure TEXT,
            bottle_position TEXT,

            -- Flavor Evolution Timeline
            month_1_profile TEXT,
            month_3_profile TEXT,
            month_6_profile TEXT,
            month_12_profile TEXT,
            year_2_profile TEXT,
            year_5_profile TEXT,
            year_10_profile TEXT,
            year_20_profile TEXT,

            -- Chemical Changes Over Time
            oxidation_effects TEXT,
            ester_evolution TEXT,
            tannin_polymerization TEXT,
            acid_softening TEXT,
            alcohol_integration TEXT,
            color_changes TEXT,
            clarity_changes TEXT,

            -- Sensory Prediction by Age
            young_characteristics TEXT,
            developing_characteristics TEXT,
            peak_characteristics TEXT,
            mature_characteristics TEXT,
            over_aged_characteristics TEXT,

            -- Value & Collectibility
            value_appreciation TEXT,
            collectibility_rating TEXT,
            auction_potential TEXT,

            -- Professional Guidance
            sommelier_aging_notes TEXT,
            cellar_master_notes TEXT,
            when_to_drink TEXT,
            when_to_sell TEXT,
            decanting_recommendations TEXT,
            serving_temperature TEXT,

            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| mazerion_core::Error::DatabaseError(format!("Create aging_profiles: {}", e)))?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_aging_beverage ON aging_profiles(beverage_type)",
        [],
    ).map_err(|e| mazerion_core::Error::DatabaseError(format!("{}", e)))?;

    Ok(())
}