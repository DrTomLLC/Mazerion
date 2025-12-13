//! Calculator implementations for Mazerion MCL

use mazerion_core::Result;

pub fn init() -> Result<()> {
    Ok(())
}

// BASIC CALCULATORS
pub mod abv;
pub mod brix_to_sg;
pub mod sg_to_brix;
pub mod sg_correction;
pub mod plato_to_sg;
pub mod hydrometer_correction;
pub mod gravity_from_ingredients;
pub mod dilution;

pub use abv::AbvCalculator;
pub use brix_to_sg::BrixToSgCalculator;
pub use sg_to_brix::SgToBrixCalculator;
pub use sg_correction::SgCorrectionCalculator;
pub use plato_to_sg::PlatoToSgCalculator;
pub use hydrometer_correction::HydrometerCorrectionCalculator;
pub use gravity_from_ingredients::GravityFromIngredientsCalculator;
pub use dilution::DilutionCalculator;

// ADVANCED CALCULATORS
pub mod blending;
pub mod refractometer;
pub mod attenuation;
pub mod volume_adjustment;
pub mod bench_trials;
pub mod alcohol_tolerance;

pub use blending::BlendingCalculator;
pub use refractometer::RefractometerCalculator;
pub use attenuation::AttenuationCalculator;
pub use volume_adjustment::VolumeAdjustmentCalculator;
pub use bench_trials::BenchTrialsCalculator;
pub use alcohol_tolerance::AlcoholToleranceCalculator;

// BREWING CALCULATORS
pub mod nutrition;
pub mod carbonation;
pub mod yeast_pitch;
pub mod yeast_starter;

pub use nutrition::NutritionCalculator;
pub use carbonation::CarbonationCalculator;
pub use yeast_pitch::YeastPitchCalculator;
pub use yeast_starter::YeastStarterCalculator;

// BEER CALCULATORS
pub mod ibu;
pub mod srm;
pub mod mash;
pub mod efficiency;

pub use ibu::IbuCalculator;
pub use srm::SrmCalculator;
pub use mash::MashCalculator;
pub use efficiency::EfficiencyCalculator;

// FINISHING CALCULATORS
pub mod backsweetening;
pub mod sulfite;
pub mod acid_addition;
pub mod stabilization;
pub mod tannin;
pub mod bottling;

pub use backsweetening::BacksweeteningCalculator;
pub use sulfite::SulfiteCalculator;
pub use acid_addition::AcidAdditionCalculator;
pub use stabilization::StabilizationCalculator;
pub use tannin::TanninCalculator;
pub use bottling::BottlingCalculator;

// MEAD STYLE CALCULATORS
pub mod great_mead;
pub mod hydromel;
pub mod sack;
pub mod melomel;
pub mod cyser;
pub mod bochet;
pub mod braggot;
pub mod metheglin;
pub mod acerglyn;
pub mod capsicumel;

pub use great_mead::GreatMeadCalculator;
pub use hydromel::HydromelCalculator;
pub use sack::SackCalculator;
pub use melomel::MelomelCalculator;
pub use cyser::CyserCalculator;
pub use bochet::BochetCalculator;
pub use braggot::BraggotCalculator;
pub use metheglin::MetheglinCalculator;
pub use acerglyn::AcerglynCalculator;
pub use capsicumel::CapsicumelCalculator;

// UTILITIES CALCULATORS
pub mod batch_cost;
pub mod water_chemistry;
pub mod priming_alternatives;
pub mod upscaling;
pub use upscaling::UpscalingCalculator;

pub use batch_cost::BatchCostCalculator;
pub use water_chemistry::WaterChemistryCalculator;
pub use priming_alternatives::PrimingAlternativesCalculator;