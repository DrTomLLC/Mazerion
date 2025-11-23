#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use std::str::FromStr;
    use crate::Validator;

    #[test]
    fn test_sg_valid() {
        if let Ok(v) = Decimal::from_str("1.000") { assert!(Validator::sg(v).is_ok()); }
        if let Ok(v) = Decimal::from_str("1.100") { assert!(Validator::sg(v).is_ok()); }
        if let Ok(v) = Decimal::from_str("0.990") { assert!(Validator::sg(v).is_ok()); }
        if let Ok(v) = Decimal::from_str("0.980") { assert!(Validator::sg(v).is_ok()); }
        if let Ok(v) = Decimal::from_str("0.960") { assert!(Validator::sg(v).is_ok()); }
    }

    #[test]
    fn test_brix_valid() {
        assert!(Validator::brix(Decimal::ZERO).is_ok());
        assert!(Validator::brix(Decimal::from(25)).is_ok());
        assert!(Validator::brix(Decimal::from(70)).is_ok());
    }

    #[test]
    fn test_ph_valid() {
        assert!(Validator::ph(Decimal::from(3)).is_ok());
        assert!(Validator::ph(Decimal::from(4)).is_ok());
    }

    #[test]
    fn test_temp_valid() {
        assert!(Validator::temp_c(Decimal::from(20)).is_ok());
        assert!(Validator::temp_c(Decimal::ZERO).is_ok());
        assert!(Validator::temp_c(Decimal::from(100)).is_ok());
    }

    #[test]
    fn test_percent_valid() {
        assert!(Validator::percent(Decimal::from(50)).is_ok());
        assert!(Validator::percent(Decimal::ZERO).is_ok());
        assert!(Validator::percent(Decimal::from(100)).is_ok());
    }

    #[test]
    fn test_brix_warning() {
        assert!(Validator::brix_warning(Decimal::from(30)).is_none());
        assert!(Validator::brix_warning(Decimal::from(50)).is_some());
    }

    #[test]
    fn test_sg_boundaries() {
        if let Ok(v) = Decimal::from_str("0.6000") { assert!(Validator::sg(v).is_ok()); }
        if let Ok(v) = Decimal::from_str("2.0000") { assert!(Validator::sg(v).is_ok()); }
        if let Ok(v) = Decimal::from_str("0.5999") { assert!(Validator::sg(v).is_err()); }
        if let Ok(v) = Decimal::from_str("2.0001") { assert!(Validator::sg(v).is_err()); }
    }

    #[test]
    fn test_brix_boundaries() {
        assert!(Validator::brix(Decimal::ZERO).is_ok());
        assert!(Validator::brix(Decimal::from(70)).is_ok());
        if let Ok(v) = Decimal::from_str("-0.001") { assert!(Validator::brix(v).is_err()); }
        if let Ok(v) = Decimal::from_str("70.001") { assert!(Validator::brix(v).is_err()); }
    }

    #[test]
    fn test_ph_boundaries() {
        if let Ok(v) = Decimal::from_str("1.50") { assert!(Validator::ph(v).is_ok()); }
        if let Ok(v) = Decimal::from_str("8.50") { assert!(Validator::ph(v).is_ok()); }
        if let Ok(v) = Decimal::from_str("1.49") { assert!(Validator::ph(v).is_err()); }
        if let Ok(v) = Decimal::from_str("8.51") { assert!(Validator::ph(v).is_err()); }
    }

    #[test]
    fn test_temp_boundaries() {
        assert!(Validator::temp_c(Decimal::from(-5)).is_ok());
        assert!(Validator::temp_c(Decimal::from(100)).is_ok());
        if let Ok(v) = Decimal::from_str("-5.1") { assert!(Validator::temp_c(v).is_err()); }
        if let Ok(v) = Decimal::from_str("100.1") { assert!(Validator::temp_c(v).is_err()); }
    }

    #[test]
    fn test_percent_boundaries() {
        assert!(Validator::percent(Decimal::ZERO).is_ok());
        assert!(Validator::percent(Decimal::from(100)).is_ok());
        if let Ok(v) = Decimal::from_str("-0.001") { assert!(Validator::percent(v).is_err()); }
        if let Ok(v) = Decimal::from_str("100.001") { assert!(Validator::percent(v).is_err()); }
    }
}
