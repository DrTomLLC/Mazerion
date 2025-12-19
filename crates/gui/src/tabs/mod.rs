//! Tab implementations for Mazerion GUI

pub mod basic;
pub mod advanced;
pub mod brewing;
pub mod beer;
pub mod finishing;
pub mod mead_styles;
pub mod utilities;
pub mod settings;
pub mod beer_styles;
mod conversions;

pub use basic::render as render_basic;
pub use advanced::render as render_advanced;
pub use brewing::render as render_brewing;
pub use beer::render as render_beer;
pub use finishing::render as render_finishing;
pub use mead_styles::{render as render_mead_styles, MeadStyle};
pub use utilities::{render as render_utilities, UtilityCalculator};
pub use settings::render as render_settings;