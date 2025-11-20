#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_sg_validation() {
        assert!(Validator::sg(dec!(1.050)).is_ok());
        assert!(Validator::sg(dec!(0.5)).is_err());
        assert!(Validator::sg(dec!(2.5)).is_err());
    }

    #[test]
    fn test_ph_validation() {
        assert!(Validator::ph(dec!(3.5)).is_ok());
        assert!(Validator::ph(dec!(1.0)).is_err());
        assert!(Validator::ph(dec!(9.0)).is_err());
    }

    #[test]
    fn test_brix_validation() {
        assert!(Validator::brix(dec!(12.0)).is_ok());
        assert!(Validator::brix(dec!(50.0)).is_ok());
        assert!(Validator::brix_warning(dec!(50.0)).is_some());
        assert!(Validator::brix_warning(dec!(30.0)).is_none());
    }

    #[test]
    fn test_temp_validation() {
        assert!(Validator::temp_c(dec!(20.0)).is_ok());
        assert!(Validator::temp_c(dec!(-10.0)).is_err());
        assert!(Validator::temp_c(dec!(150.0)).is_err());
    }

    #[test]
    fn test_measurement_creation() {
        let sg = Measurement::sg(dec!(1.050));
        assert!(sg.is_ok());

        let ph = Measurement::ph(dec!(3.5));
        assert!(ph.is_ok());

        let invalid_sg = Measurement::sg(dec!(0.5));
        assert!(invalid_sg.is_err());
    }

    #[test]
    fn test_calc_input() {
        let input = CalcInput::new()
            .add_measurement(Measurement::sg(dec!(1.050)).unwrap())
            .add_param("test", "value");

        assert!(input.get_measurement(Unit::SpecificGravity).is_ok());
        assert!(input.get_param("test").is_some());
        assert_eq!(input.get_param("test"), Some("value"));
    }
}
