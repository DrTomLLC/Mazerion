// Tab implementations
pub mod basic;
pub mod advanced;
pub mod brewing;
pub mod finishing;

pub use basic::render as render_basic;
pub use advanced::render as render_advanced;
pub use brewing::render as render_brewing;
pub use finishing::render as render_finishing;
