pub struct CalcContext {
    // config, localization, maybe current datetime, etc.
}

pub trait Calculator {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;

    fn supports(&self, input: &CalcInput) -> bool;
    fn calculate(&self, input: &CalcInput, ctx: &CalcContext) -> CalcResult;
}

// Example shared types; expanded as needed
pub struct CalcInput {
    pub batch_size_l: f64,
    pub og: f64,
    pub fg: f64,
    // …
}

pub struct CalcResult {
    pub abv: f64,
    pub residual_sugar_g_l: f64,
    // …
}
