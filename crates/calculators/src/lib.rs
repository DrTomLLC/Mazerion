//! Calculator implementations for Mazerion.

pub mod abv;
pub mod brix_to_sg;
pub mod sg_correction;
pub mod dilution;
pub mod blending;
pub mod refractometer;
pub mod nutrition;
pub mod carbonation;
pub mod acid_addition;
pub mod sulfite;
pub mod backsweetening;

pub use abv::AbvCalculator;
pub use brix_to_sg::BrixToSgCalculator;
pub use sg_correction::SgCorrectionCalculator;
pub use dilution::DilutionCalculator;
pub use blending::BlendingCalculator;
pub use refractometer::RefractometerCalculator;
pub use nutrition::NutritionCalculator;
pub use carbonation::CarbonationCalculator;
pub use acid_addition::AcidAdditionCalculator;
pub use sulfite::SulfiteCalculator;
pub use backsweetening::BacksweeteningCalculator;