//! Beer styles data structures and loading
//! SAFETY-CRITICAL: Zero panics, all errors handled

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeerStyle {
    pub name: String,
    pub number: String,
    pub category: String,
    pub categorynumber: String,
    pub overallimpression: String,
    pub aroma: String,
    pub appearance: String,
    pub flavor: String,
    pub mouthfeel: String,
    pub comments: String,
    pub history: String,
    #[serde(rename = "characteristicingredients")]
    pub characteristic_ingredients: String,
    #[serde(rename = "stylecomparison")]
    pub style_comparison: String,
    #[serde(rename = "ibumin")]
    pub ibu_min: String,
    #[serde(rename = "ibumax")]
    pub ibu_max: String,
    #[serde(rename = "ogmin")]
    pub og_min: String,
    #[serde(rename = "ogmax")]
    pub og_max: String,
    #[serde(rename = "fgmin")]
    pub fg_min: String,
    #[serde(rename = "fgmax")]
    pub fg_max: String,
    #[serde(rename = "abvmin")]
    pub abv_min: String,
    #[serde(rename = "abvmax")]
    pub abv_max: String,
    #[serde(rename = "srmmin")]
    pub srm_min: String,
    #[serde(rename = "srmmax")]
    pub srm_max: String,
    #[serde(rename = "commercialexamples")]
    pub commercial_examples: String,
    pub tags: String,
}

impl BeerStyle {
    /// Get vital statistics as formatted string
    pub fn vital_stats(&self) -> String {
        format!(
            "OG: {}-{} | FG: {}-{} | IBU: {}-{} | SRM: {}-{} | ABV: {}-{}%",
            self.og_min, self.og_max,
            self.fg_min, self.fg_max,
            self.ibu_min, self.ibu_max,
            self.srm_min, self.srm_max,
            self.abv_min, self.abv_max
        )
    }

    /// Check if style matches search query
    pub fn matches(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        self.name.to_lowercase().contains(&query_lower) ||
            self.category.to_lowercase().contains(&query_lower) ||
            self.tags.to_lowercase().contains(&query_lower) ||
            self.commercial_examples.to_lowercase().contains(&query_lower)
    }
}

/// Load beer styles from embedded JSON
pub fn load_beer_styles() -> Result<Vec<BeerStyle>, String> {
    const BEER_STYLES_JSON: &str = include_str!("../../../../data/beer_styles.v1.json");

    serde_json::from_str(BEER_STYLES_JSON)
        .map_err(|e| format!("Failed to parse beer styles JSON: {}", e))
}

/// Get all unique categories
pub fn get_categories(styles: &[BeerStyle]) -> Vec<String> {
    let mut cats: Vec<String> = styles.iter()
        .map(|s| s.category.clone())
        .collect();
    cats.sort();
    cats.dedup();
    cats
}

/// Filter styles by category
pub fn filter_by_category(styles: &[BeerStyle], category: &str) -> Vec<BeerStyle> {
    styles.iter()
        .filter(|s| s.category == category)
        .cloned()
        .collect()
}