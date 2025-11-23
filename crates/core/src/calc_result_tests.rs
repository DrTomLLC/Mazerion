#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use std::str::FromStr;
    use crate::{CalcResult, Measurement, Unit};

    #[test]
    fn test_calc_result_new() {
        if let Ok(val) = Decimal::from_str("42.5") {
            let result = CalcResult::new(Measurement::new(val, Unit::Percent));
            assert_eq!(result.output.value, val);
            assert_eq!(result.output.unit, Unit::Percent);
            assert!(result.warnings.is_empty());
            assert!(result.metadata.is_empty());
        }
    }

    #[test]
    fn test_with_warning() {
        if let Ok(val) = Decimal::from_str("12.5") {
            let result = CalcResult::new(Measurement::new(val, Unit::Abv))
                .with_warning("High ABV");
            
            assert_eq!(result.warnings.len(), 1);
            assert_eq!(result.warnings[0], "High ABV");
        }
    }

    #[test]
    fn test_with_meta() {
        if let Ok(val) = Decimal::from_str("1.050") {
            if let Ok(sg_meas) = Measurement::sg(val) {
                let result = CalcResult::new(sg_meas)
                    .with_meta("og", "1.100")
                    .with_meta("fg", "1.010");
                
                assert_eq!(result.metadata.len(), 2);
                assert_eq!(result.metadata[0].0, "og");
                assert_eq!(result.metadata[0].1, "1.100");
                assert_eq!(result.metadata[1].0, "fg");
                assert_eq!(result.metadata[1].1, "1.010");
            }
        }
    }

    #[test]
    fn test_multiple_warnings() {
        let result = CalcResult::new(Measurement::new(Decimal::from(20), Unit::Abv))
            .with_warning("Warning 1")
            .with_warning("Warning 2");
        
        assert_eq!(result.warnings.len(), 2);
    }

    #[test]
    fn test_chaining() {
        let result = CalcResult::new(Measurement::new(Decimal::from(10), Unit::Abv))
            .with_warning("test warning")
            .with_meta("key", "value");
        
        assert_eq!(result.warnings.len(), 1);
        assert_eq!(result.metadata.len(), 1);
    }

    #[test]
    fn test_empty_result() {
        let result = CalcResult::new(Measurement::new(Decimal::ZERO, Unit::Percent));
        assert!(result.warnings.is_empty());
        assert!(result.metadata.is_empty());
    }
}
