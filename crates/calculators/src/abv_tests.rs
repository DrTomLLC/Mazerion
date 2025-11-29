#[cfg(test)]
mod tests {
    use mazerion_core::{CalcInput, Calculator};
    use rust_decimal::Decimal;
    use std::str::FromStr;
    use crate::AbvCalculator;

    #[test]
    fn test_abv_standard() {
        let calc = AbvCalculator::default();
        let input = CalcInput::new()
            .add_param("og", "1.090")
            .add_param("fg", "1.010");

        let result = calc.calculate(input).unwrap();
        let expected = Decimal::from_str("10.5").unwrap();
        assert!((result.output.value - expected).abs() < Decimal::from_str("0.1").unwrap());
    }

    #[test]
    fn test_abv_high() {
        let calc = AbvCalculator::default();
        let input = CalcInput::new()
            .add_param("og", "1.160")
            .add_param("fg", "1.000");

        let result = calc.calculate(input).unwrap();
        assert!(!result.warnings.is_empty());
    }

    #[test]
    fn test_abv_validation_missing_og() {
        let calc = AbvCalculator::default();
        let input = CalcInput::new().add_param("fg", "1.010");

        let result = calc.calculate(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_abv_validation_missing_fg() {
        let calc = AbvCalculator::default();
        let input = CalcInput::new().add_param("og", "1.090");

        let result = calc.calculate(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_abv_validation_fg_greater_than_og() {
        let calc = AbvCalculator::default();
        let input = CalcInput::new()
            .add_param("og", "1.010")
            .add_param("fg", "1.090");

        let result = calc.calculate(input);
        assert!(result.is_err());
    }
}