// crates/db/src/schemas/mod.rs
// REPLACE YOUR ENTIRE FILE WITH THIS

pub mod yeasts;
pub mod honeys;
pub mod hops;
pub mod malts;
pub mod fruits;
pub mod vegetables;
pub mod spices;
pub mod herbs;
pub mod extracts;
pub mod syrups;
pub mod adjuncts;
pub mod water_profiles;
pub mod water_salts;
pub mod acids;
pub mod nutrients;
pub mod enzymes;
pub mod bacteria;
pub mod tannins;

use rusqlite::{Connection, Result};

/// Initialize all encyclopedia tables
pub fn init_all_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(yeasts::YEAST_SCHEMA)?;
    conn.execute_batch(honeys::HONEY_SCHEMA)?;
    conn.execute_batch(hops::HOP_SCHEMA)?;
    conn.execute_batch(malts::MALT_SCHEMA)?;
    conn.execute_batch(fruits::FRUIT_SCHEMA)?;
    conn.execute_batch(vegetables::VEGETABLE_SCHEMA)?;
    conn.execute_batch(spices::SPICE_SCHEMA)?;
    conn.execute_batch(herbs::HERB_SCHEMA)?;
    conn.execute_batch(extracts::EXTRACT_SCHEMA)?;
    conn.execute_batch(syrups::SYRUP_SCHEMA)?;
    conn.execute_batch(adjuncts::ADJUNCT_SCHEMA)?;
    conn.execute_batch(water_profiles::WATER_PROFILE_SCHEMA)?;
    conn.execute_batch(water_salts::WATER_SALT_SCHEMA)?;
    conn.execute_batch(acids::ACID_SCHEMA)?;
    conn.execute_batch(nutrients::NUTRIENT_SCHEMA)?;
    conn.execute_batch(enzymes::ENZYME_SCHEMA)?;
    conn.execute_batch(bacteria::BACTERIA_SCHEMA)?;
    conn.execute_batch(tannins::TANNIN_SCHEMA)?;
    Ok(())
}