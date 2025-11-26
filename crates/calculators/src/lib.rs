// Calculator implementations for Mazerion - All 39 calculators.

// Basic (3)
pub mod abv;
pub mod brix_to_sg;
pub mod sg_correction;

// Basic Operations (5)
pub mod dilution;
pub mod blending;
pub mod plato_to_sg;
pub mod hydrometer_correction;
pub mod volume_adjustment;

// Advanced Analysis (3)
pub mod refractometer;
pub mod attenuation;
pub mod gravity_from_ingredients;

// Fermentation Management (5)
pub mod nutrition;
pub mod yeast_pitch;
pub mod yeast_starter;
pub mod alcohol_tolerance;
pub mod fermentation_timeline;

// Carbonation & Packaging (4)
pub mod carbonation;
pub mod priming_alternatives;
pub mod bottling;
pub mod stabilization;

// Finishing (3)
pub mod acid_addition;
pub mod sulfite;
pub mod backsweetening;

// Mead Styles (8)
pub mod melomel;
pub mod bochet;
pub mod cyser;
pub mod metheglin;
pub mod acerglyn;
pub mod braggot;
pub mod capsicumel;
pub mod hydromel;

// Advanced Mead (2)
pub mod sack;
pub mod great_mead;

// Beer-specific (2)
pub mod ibu;
pub mod srm;

// Additional Utilities (4)
pub mod bench_trials;
pub mod cost_calculator;
pub mod water_chemistry;
pub mod tannin;

/// Initialize all calculators.
pub fn init() {
    let _ = &*crate::abv::AbvCalculator::ID;
    let _ = &*crate::brix_to_sg::BrixToSgCalculator::ID;
    let _ = &*crate::sg_correction::SgCorrectionCalculator::ID;
    let _ = &*crate::dilution::DilutionCalculator::ID;
    let _ = &*crate::blending::BlendingCalculator::ID;
    let _ = &*crate::plato_to_sg::PlatoToSgCalculator::ID;
    let _ = &*crate::hydrometer_correction::HydrometerCorrectionCalculator::ID;
    let _ = &*crate::volume_adjustment::VolumeAdjustmentCalculator::ID;
    let _ = &*crate::refractometer::RefractometerCalculator::ID;
    let _ = &*crate::attenuation::AttenuationCalculator::ID;
    let _ = &*crate::gravity_from_ingredients::GravityFromIngredientsCalculator::ID;
    let _ = &*crate::nutrition::NutritionCalculator::ID;
    let _ = &*crate::yeast_pitch::YeastPitchCalculator::ID;
    let _ = &*crate::yeast_starter::YeastStarterCalculator::ID;
    let _ = &*crate::alcohol_tolerance::AlcoholToleranceCalculator::ID;
    let _ = &*crate::fermentation_timeline::FermentationTimelineCalculator::ID;
    let _ = &*crate::carbonation::CarbonationCalculator::ID;
    let _ = &*crate::priming_alternatives::PrimingAlternativesCalculator::ID;
    let _ = &*crate::bottling::BottlingCalculator::ID;
    let _ = &*crate::stabilization::StabilizationCalculator::ID;
    let _ = &*crate::acid_addition::AcidAdditionCalculator::ID;
    let _ = &*crate::sulfite::SulfiteCalculator::ID;
    let _ = &*crate::backsweetening::BacksweeteningCalculator::ID;
    let _ = &*crate::melomel::MelomelsCalculator::ID;
    let _ = &*crate::bochet::BochetCalculator::ID;
    let _ = &*crate::cyser::CyserCalculator::ID;
    let _ = &*crate::metheglin::MetheglinCalculator::ID;
    let _ = &*crate::acerglyn::AcerglynCalculator::ID;
    let _ = &*crate::braggot::BraggotCalculator::ID;
    let _ = &*crate::capsicumel::CapsicumelCalculator::ID;
    let _ = &*crate::hydromel::HydromelCalculator::ID;
    let _ = &*crate::sack::SackCalculator::ID;
    let _ = &*crate::great_mead::GreatMeadCalculator::ID;
    let _ = &*crate::ibu::IbuCalculator::ID;
    let _ = &*crate::srm::SrmCalculator::ID;
    let _ = &*crate::bench_trials::BenchTrialsCalculator::ID;
    // TODO: Fix cost_calculator ID reference once implementation is complete
    // let _ = &*crate::cost_calculator::Calculator::ID;
    let _ = &*crate::water_chemistry::WaterChemistryCalculator::ID;
    let _ = &*crate::tannin::TanninCalculator::ID;
}
