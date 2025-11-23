use mazerion_core::{find_calculator, CalcInput};
use rust_decimal::Decimal;

pub fn calc(calc_id: &str, params: Vec<(&str, &str)>) -> String {
    let calculator = match find_calculator(calc_id) {
        Some(c) => c,
        None => return format!("Calculator '{}' not found", calc_id),
    };

    let mut input = CalcInput::new();
    for (key, value) in params {
        if let Ok(decimal) = value.parse::<Decimal>() {
            input = input.with_decimal(key, decimal);
        } else {
            input = input.with_string(key, value);
        }
    }

    match calculator.calculate(input) {
        Ok(result) => format_result(result),
        Err(e) => format!("Error: {}", e),
    }
}

fn format_result(result: mazerion_core::CalcResult) -> String {
    let mut output = format!("{} {}", result.primary.value, result.primary.unit);

    for secondary in result.secondary {
        output.push_str(&format!("\n{} {}", secondary.value, secondary.unit));
    }

    for warning in result.warnings {
        output.push_str(&format!("\n⚠️  {}", warning));
    }

    for note in result.notes {
        output.push_str(&format!("\nℹ️  {}", note));
    }

    output
}
