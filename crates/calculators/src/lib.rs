//! Calculator implementations for Mazerion MCL

// ═══════════════════════════════════════════════════════════════════════
// BASIC CALCULATORS (7)
// ═══════════════════════════════════════════════════════════════════════
pub mod abv;
pub mod brix_to_sg;
pub mod sg_correction;
pub mod plato_to_sg;
pub mod hydrometer_correction;
pub mod gravity_from_ingredients;

pub use abv::AbvCalculator;
pub use brix_to_sg::BrixToSgCalculator;
pub use sg_correction::SgCorrectionCalculator;
pub use plato_to_sg::PlatoToSgCalculator;
pub use hydrometer_correction::HydrometerCorrectionCalculator;
pub use gravity_from_ingredients::GravityFromIngredientsCalculator;

// ═══════════════════════════════════════════════════════════════════════
// ADVANCED CALCULATORS (7)
// ═══════════════════════════════════════════════════════════════════════
pub mod dilution;
pub mod blending;
pub mod refractometer;
pub mod attenuation;
pub mod volume_adjustment;
pub mod bench_trials;
pub mod alcohol_tolerance;

pub use dilution::DilutionCalculator;
pub use blending::BlendingCalculator;
pub use refractometer::RefractometerCalculator;
pub use attenuation::AttenuationCalculator;
pub use volume_adjustment::VolumeAdjustmentCalculator;
pub use bench_trials::BenchTrialsCalculator;
pub use alcohol_tolerance::AlcoholToleranceCalculator;

// ═══════════════════════════════════════════════════════════════════════
// BREWING CALCULATORS (5)
// ═══════════════════════════════════════════════════════════════════════
pub mod nutrition;
pub mod yeast_pitch;
pub mod yeast_starter;
pub mod fermentation_timeline;
pub mod carbonation;

pub use nutrition::NutritionCalculator;
pub use yeast_pitch::YeastPitchCalculator;
pub use yeast_starter::YeastStarterCalculator;
pub use fermentation_timeline::FermentationTimelineCalculator;
pub use carbonation::CarbonationCalculator;

// ═══════════════════════════════════════════════════════════════════════
// BEER CALCULATORS (1)
// ═══════════════════════════════════════════════════════════════════════
pub mod ibu;

pub use ibu::IbuCalculator;

// ═══════════════════════════════════════════════════════════════════════
// FINISHING CALCULATORS (6)
// ═══════════════════════════════════════════════════════════════════════
pub mod sulfite;
pub mod acid_addition;
pub mod backsweetening;
pub mod stabilization;
pub mod tannin;
pub mod bottling;

pub use sulfite::SulfiteCalculator;
pub use acid_addition::AcidAdditionCalculator;
pub use backsweetening::BacksweeteningCalculator;
pub use stabilization::StabilizationCalculator;
pub use tannin::TanninCalculator;
pub use bottling::BottlingCalculator;

// ═══════════════════════════════════════════════════════════════════════
// MEAD STYLE CALCULATORS (10)
// ═══════════════════════════════════════════════════════════════════════
pub mod cyser;
pub mod acerglyn;
pub mod bochet;
pub mod braggot;
pub mod capsicumel;
pub mod great_mead;
pub mod hydromel;
pub mod melomel;
pub mod metheglin;
pub mod sack;

pub use cyser::CyserCalculator;
pub use acerglyn::AcerglynCalculator;
pub use bochet::BochetCalculator;
pub use braggot::BraggotCalculator;
pub use capsicumel::CapsicumelCalculator;
pub use great_mead::GreatMeadCalculator;
pub use hydromel::HydromelCalculator;
pub use melomel::MelomelCalculator;
pub use metheglin::MetheglinCalculator;
pub use sack::SackCalculator;

// ═══════════════════════════════════════════════════════════════════════
// UTILITY CALCULATORS (4)
// ═══════════════════════════════════════════════════════════════════════
pub mod cost_calculator;
pub mod priming_alternatives;
pub mod water_chemistry;

pub use cost_calculator::CostCalculator;
pub use priming_alternatives::PrimingAlternativesCalculator;
pub use water_chemistry::WaterChemistryCalculator;

// ═══════════════════════════════════════════════════════════════════════
// FORCE LINKER TO INCLUDE ALL CALCULATORS
// ═══════════════════════════════════════════════════════════════════════

#[used]
static FORCE_LINK: fn() = force_calculator_linking;

fn force_calculator_linking() {
    // Basic
    let _ = AbvCalculator::default();
    let _ = BrixToSgCalculator::default();
    let _ = SgCorrectionCalculator::default();
    let _ = PlatoToSgCalculator::default();
    let _ = HydrometerCorrectionCalculator::default();
    let _ = GravityFromIngredientsCalculator::default();

    // Advanced
    let _ = DilutionCalculator::default();
    let _ = BlendingCalculator::default();
    let _ = RefractometerCalculator::default();
    let _ = AttenuationCalculator::default();
    let _ = VolumeAdjustmentCalculator::default();
    let _ = BenchTrialsCalculator::default();
    let _ = AlcoholToleranceCalculator::default();

    // Brewing
    let _ = NutritionCalculator::default();
    let _ = YeastPitchCalculator::default();
    let _ = YeastStarterCalculator::default();
    let _ = FermentationTimelineCalculator::default();
    let _ = CarbonationCalculator::default();

    // Beer
    let _ = IbuCalculator::default();

    // Finishing
    let _ = SulfiteCalculator::default();
    let _ = AcidAdditionCalculator::default();
    let _ = BacksweeteningCalculator::default();
    let _ = StabilizationCalculator::default();
    let _ = TanninCalculator::default();
    let _ = BottlingCalculator::default();

    // Mead Styles
    let _ = CyserCalculator::default();
    let _ = AcerglynCalculator::default();
    let _ = BochetCalculator::default();
    let _ = BraggotCalculator::default();
    let _ = CapsicumelCalculator::default();
    let _ = GreatMeadCalculator::default();
    let _ = HydromelCalculator::default();
    let _ = MelomelCalculator::default();
    let _ = MetheglinCalculator::default();
    let _ = SackCalculator::default();

    // Utilities
    let _ = CostCalculator::default();
    let _ = PrimingAlternativesCalculator::default();
    let _ = WaterChemistryCalculator::default();
}

pub fn init() -> Result<(), &'static str> {
    force_calculator_linking();
    let count = mazerion_core::traits::calculator_count();
    if count == 0 {
        return Err("No calculators registered! Check linkme configuration.");
    }
    println!("✓ Loaded {} calculators", count);
    Ok(())
}