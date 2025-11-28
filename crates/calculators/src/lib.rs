//! Calculator implementations for Mazerion - COMPLETE

pub mod abv;
pub mod brix_to_sg;
pub mod sg_correction;
pub mod dilution;
pub mod blending;
pub mod refractometer;
pub mod nutrition;
pub mod carbonation;
pub mod sulfite;
pub mod acid_addition;
pub mod backsweetening;
mod stabilization;

pub use abv::AbvCalculator;
pub use brix_to_sg::BrixToSgCalculator;
pub use sg_correction::SgCorrectionCalculator;
pub use dilution::DilutionCalculator;
pub use blending::BlendingCalculator;
pub use refractometer::RefractometerCalculator;
pub use nutrition::NutritionCalculator;
pub use carbonation::CarbonationCalculator;
pub use sulfite::SulfiteCalculator;
pub use acid_addition::AcidAdditionCalculator;
pub use backsweetening::BacksweeteningCalculator;

/// Initialize all calculators (forces registration)
pub fn init() {
    // Force static initialization
    let _ = &mazerion_core::traits::CALCULATORS;
}