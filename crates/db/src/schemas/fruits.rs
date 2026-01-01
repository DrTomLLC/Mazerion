pub const FRUIT_SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS fruits (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    scientific_name TEXT,
    fruit_type TEXT NOT NULL,
    origin TEXT,
    typical_sugar_content TEXT,
    ph_level TEXT,
    color_contribution TEXT,
    flavor_profile TEXT,
    aroma_profile TEXT,
    best_suited_styles TEXT,
    usage_notes TEXT,
    sensory_notes TEXT,
    pounds_per_gallon TEXT,
    preparation_method TEXT,
    compatible_styles TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    CHECK(fruit_type IN ('berry', 'stone fruit', 'citrus', 'pome', 'tropical', 'melon', 'other'))
);
CREATE INDEX IF NOT EXISTS idx_fruits_name ON fruits(name);
CREATE INDEX IF NOT EXISTS idx_fruits_type ON fruits(fruit_type);
";