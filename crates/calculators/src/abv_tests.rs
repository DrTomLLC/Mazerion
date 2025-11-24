#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use mazerion_core::{CalcInput, Calculator};
    use crate::AbvCalculator;

    #[test]
    fn test_abv_calculation() {
        let calc = AbvCalculator;
        let input = CalcInput::new()
            .add_param("og", "1.050")
            .add_param("fg", "1.010");

        let result = calc.calculate(input);
        assert!(result.is_ok());

        let res = if let Ok(value) = result {
            value
        } else {
            // This should never happen as we've already verified result is Ok
            unreachable!("Result should be Ok as verified by assert");
        };
        assert!(res.output.value > dec!(0));
        assert!(res.output.value < dec!(20));
    }

    #[test]
    fn test_abv_missing_params() {
        let calc = AbvCalculator;
        let input = CalcInput::new().add_param("og", "1.050");

        let result = calc.calculate(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_abv_invalid_og_fg() {
        let calc = AbvCalculator;
        let input = CalcInput::new()
            .add_param("og", "1.010")
            .add_param("fg", "1.050");

        let result = calc.calculate(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_abv_high_warning() {
        let calc = AbvCalculator;
        let input = CalcInput::new()
            .add_param("og", "1.200")
            .add_param("fg", "1.010");

        let result = calc.calculate(input);
        assert!(result.is_ok());

        let res = if let Ok(value) = result {
            value
        } else {
            // This should never happen as we've already verified result is Ok
            unreachable!("Result should be Ok as verified by assert");
        };
        assert!(!res.warnings.is_empty());
    }
}
