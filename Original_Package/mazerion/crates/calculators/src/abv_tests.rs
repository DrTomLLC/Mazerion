#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_abv_calculation() {
        let calc = AbvCalculator;
        let input = CalcInput::new()
            .add_param("og", "1.050")
            .add_param("fg", "1.010");

        let result = calc.calculate(input);
        assert!(result.is_ok());

        let res = result.unwrap();
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

        let res = result.unwrap();
        assert!(!res.warnings.is_empty());
    }
}
