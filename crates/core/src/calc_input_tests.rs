#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use std::str::FromStr;
    use crate::{CalcInput, Measurement, Unit};

    #[test]
    fn test_calc_input_new() {
        let input = CalcInput::new();
        assert!(input.measurements.is_empty());
        assert!(input.params.is_empty());
    }

    #[test]
    fn test_add_measurement() {
        if let Ok(val) = Decimal::from_str("1.050") {
            if let Ok(sg_meas) = Measurement::sg(val) {
                let input = CalcInput::new().add_measurement(sg_meas);
                assert_eq!(input.measurements.len(), 1);
                assert_eq!(input.measurements[0].unit, Unit::SpecificGravity);
            }
        }
    }

    #[test]
    fn test_add_param() {
        let input = CalcInput::new().add_param("og", "1.050");
        assert_eq!(input.params.len(), 1);
        assert_eq!(input.get_param("og"), Some("1.050"));
    }

    #[test]
    fn test_get_measurement() {
        let sg_result = Decimal::from_str("1.050").ok().and_then(|v| Measurement::sg(v).ok());
        let ph_result = Decimal::from_str("3.5").ok().and_then(|v| Measurement::ph(v).ok());
        
        if let (Some(sg_meas), Some(ph_meas)) = (sg_result, ph_result) {
            let input = CalcInput::new()
                .add_measurement(sg_meas)
                .add_measurement(ph_meas);
            
            assert!(input.get_measurement(Unit::SpecificGravity).is_ok());
            assert!(input.get_measurement(Unit::Ph).is_ok());
        }
    }

    #[test]
    fn test_get_missing_measurement() {
        let input = CalcInput::new();
        assert!(input.get_measurement(Unit::SpecificGravity).is_err());
    }

    #[test]
    fn test_get_param() {
        let input = CalcInput::new()
            .add_param("og", "1.050")
            .add_param("fg", "1.010");
        
        assert_eq!(input.get_param("og"), Some("1.050"));
        assert_eq!(input.get_param("fg"), Some("1.010"));
        assert_eq!(input.get_param("missing"), None);
    }

    #[test]
    fn test_multiple_measurements() {
        let sg = Decimal::from_str("1.050").ok().and_then(|v| Measurement::sg(v).ok());
        let temp = Measurement::celsius(Decimal::from(20)).ok();
        let brix = Decimal::from_str("12.5").ok().and_then(|v| Measurement::brix(v).ok());
        
        if let (Some(s), Some(t), Some(b)) = (sg, temp, brix) {
            let input = CalcInput::new()
                .add_measurement(s)
                .add_measurement(t)
                .add_measurement(b);
            
            assert_eq!(input.measurements.len(), 3);
        }
    }

    #[test]
    fn test_multiple_params() {
        let input = CalcInput::new()
            .add_param("a", "1")
            .add_param("b", "2")
            .add_param("c", "3");
        
        assert_eq!(input.params.len(), 3);
    }

    #[test]
    fn test_chaining() {
        if let Ok(val) = Decimal::from_str("1.050") {
            if let Ok(sg_meas) = Measurement::sg(val) {
                let input = CalcInput::new()
                    .add_param("test", "value")
                    .add_measurement(sg_meas);
                
                assert_eq!(input.params.len(), 1);
                assert_eq!(input.measurements.len(), 1);
            }
        }
    }
}
