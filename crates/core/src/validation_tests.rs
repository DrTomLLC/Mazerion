#[cfg(test)]
mod tests {
    use crate::{CalcInput, Measurement, Unit, Validator};
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn test_sg_validation() {
        assert!(Validator::sg(Decimal::new(1050, 3)).is_ok());
        assert!(Validator::sg(Decimal::new(5, 1)).is_err());
        assert!(Validator::sg(Decimal::new(25, 1)).is_err());
    }

    #[test]
    fn test_ph_validation() {
        assert!(Validator::ph(Decimal::new(35, 1)).is_ok());
        assert!(Validator::ph(Decimal::ONE).is_err());
        assert!(Validator::ph(Decimal::new(90, 1)).is_err());
    }

    #[test]
    fn test_brix_validation() {
        assert!(Validator::brix(Decimal::from(12)).is_ok());
        assert!(Validator::brix(Decimal::from(50)).is_ok());
        assert!(Validator::brix_warning(Decimal::from(50)).is_some());
        assert!(Validator::brix_warning(Decimal::from(30)).is_none());
    }

    #[test]
    fn test_temp_validation() {
        assert!(Validator::temp_c(Decimal::from(20)).is_ok());
        assert!(Validator::temp_c(Decimal::from(-10)).is_err());
        assert!(Validator::temp_c(Decimal::from(150)).is_err());
    }

    #[test]
    fn test_measurement_creation() {
        let sg = Measurement::sg(Decimal::new(1050, 3));
        assert!(sg.is_ok());

        let ph = Measurement::ph(Decimal::new(35, 1));
        assert!(ph.is_ok());

        let invalid_sg = Measurement::sg(Decimal::new(5, 1));
        assert!(invalid_sg.is_err());
    }

    #[test]
    fn test_calc_input() {
        let sg = Measurement::sg(Decimal::new(1050, 3));
        assert!(sg.is_ok());

        if let Ok(measurement) = sg {
            let input = CalcInput::new()
                .add_measurement(measurement)
                .add_param("test", "value");

            assert!(input.get_measurement(Unit::SpecificGravity).is_ok());
            assert!(input.get_param("test").is_some());
            assert_eq!(input.get_param("test"), Some("value"));
        }
    }
}