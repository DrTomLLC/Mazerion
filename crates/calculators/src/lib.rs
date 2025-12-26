//! Calculator implementations for Mazerion MCL

use mazerion_core::Result;

pub fn init() -> Result<()> {
    Ok(())
}

// BASIC CALCULATORS
pub mod abv;
pub mod brix_to_sg;
pub mod dilution;
pub mod gravity_from_ingredients;
pub mod hydrometer_correction;
pub mod plato_to_sg;
pub mod sg_correction;
pub mod sg_to_brix;

pub use abv::AbvCalculator;
pub use brix_to_sg::BrixToSgCalculator;
pub use dilution::DilutionCalculator;
pub use gravity_from_ingredients::GravityFromIngredientsCalculator;
pub use hydrometer_correction::HydrometerCorrectionCalculator;
pub use plato_to_sg::PlatoToSgCalculator;
pub use sg_correction::SgCorrectionCalculator;
pub use sg_to_brix::SgToBrixCalculator;

// ADVANCED CALCULATORS
pub mod alcohol_tolerance;
pub mod attenuation;
pub mod bench_trials;
pub mod blending;
pub mod refractometer;
pub mod volume_adjustment;

pub use alcohol_tolerance::AlcoholToleranceCalculator;
pub use attenuation::AttenuationCalculator;
pub use bench_trials::BenchTrialsCalculator;
pub use blending::BlendingCalculator;
pub use refractometer::RefractometerCalculator;
pub use volume_adjustment::VolumeAdjustmentCalculator;

// BREWING CALCULATORS
pub mod carbonation;
pub mod nutrition;
pub mod yeast_pitch;
pub mod yeast_starter;

pub use carbonation::CarbonationCalculator;
pub use nutrition::NutritionCalculator;
pub use yeast_pitch::YeastPitchCalculator;
pub use yeast_starter::YeastStarterCalculator;

// BEER CALCULATORS
pub mod efficiency;
pub mod ibu;
pub mod mash;
pub mod srm;

pub use efficiency::EfficiencyCalculator;
pub use ibu::IbuCalculator;
pub use mash::MashCalculator;
pub use srm::SrmCalculator;

// FINISHING CALCULATORS
pub mod acid_addition;
pub mod backsweetening;
pub mod bottling;
pub mod stabilization;
pub mod sulfite;
pub mod tannin;

pub use acid_addition::AcidAdditionCalculator;
pub use backsweetening::BacksweeteningCalculator;
pub use bottling::BottlingCalculator;
pub use pasteurization::PasteurizationCalculator;
pub use stabilization::StabilizationCalculator;
pub use sulfite::SulfiteCalculator;
pub use tannin::TanninCalculator;

// MEAD STYLE CALCULATORS
pub mod acerglyn;
pub mod bochet;
pub mod braggot;
pub mod capsicumel;
pub mod cyser;
pub mod great_mead;
pub mod hydromel;
pub mod melomel;
pub mod metheglin;
pub mod sack;

pub use acerglyn::AcerglynCalculator;
pub use bochet::BochetCalculator;
pub use braggot::BraggotCalculator;
pub use capsicumel::CapsicumelCalculator;
pub use cyser::CyserCalculator;
pub use great_mead::GreatMeadCalculator;
pub use hydromel::HydromelCalculator;
pub use melomel::MelomelCalculator;
pub use metheglin::MetheglinCalculator;
pub use pyment::PymentCalculator;
pub use sack::SackCalculator;
pub mod pasteurization;

pub use lactomel::LactomelCalculator;
pub use oxymel::OxymelCalculator;
// UTILITIES CALCULATORS
pub mod batch_cost;
mod gallons_to_bottles;
mod gallons_to_bottles_with_losses;
mod lactomel;
mod oxymel;
pub mod priming_alternatives;
mod pyment;
pub mod upscaling;
mod waste;
pub mod water_chemistry;

pub use upscaling::UpscalingCalculator;

pub use batch_cost::BatchCostCalculator;
pub use priming_alternatives::PrimingAlternativesCalculator;
pub use waste::WasteCalculator;
pub use water_chemistry::WaterChemistryCalculator;
