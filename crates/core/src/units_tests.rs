#[cfg(test)]
mod tests {
    use crate::Unit;

    #[test]
    fn test_unit_symbols() {
        assert_eq!(Unit::SpecificGravity.symbol(), "SG");
        assert_eq!(Unit::Ph.symbol(), "pH");
        assert_eq!(Unit::Brix.symbol(), "°Bx");
        assert_eq!(Unit::Plato.symbol(), "°P");
        assert_eq!(Unit::Celsius.symbol(), "°C");
        assert_eq!(Unit::Fahrenheit.symbol(), "°F");
        assert_eq!(Unit::Percent.symbol(), "%");
        assert_eq!(Unit::Abv.symbol(), "% ABV");
        assert_eq!(Unit::Grams.symbol(), "g");
        assert_eq!(Unit::Liters.symbol(), "L");
        assert_eq!(Unit::Milliliters.symbol(), "mL");
    }

    #[test]
    fn test_unit_precision() {
        assert_eq!(Unit::SpecificGravity.precision(), 4);
        assert_eq!(Unit::Ph.precision(), 3);
        assert_eq!(Unit::Brix.precision(), 2);
        assert_eq!(Unit::Plato.precision(), 2);
        assert_eq!(Unit::Celsius.precision(), 1);
        assert_eq!(Unit::Fahrenheit.precision(), 1);
        assert_eq!(Unit::Percent.precision(), 2);
        assert_eq!(Unit::Abv.precision(), 2);
        assert_eq!(Unit::Grams.precision(), 2);
        assert_eq!(Unit::Liters.precision(), 2);
        assert_eq!(Unit::Milliliters.precision(), 2);
    }

    #[test]
    fn test_unit_display() {
        assert_eq!(format!("{}", Unit::Abv), "% ABV");
        assert_eq!(format!("{}", Unit::SpecificGravity), "SG");
        assert_eq!(format!("{}", Unit::Ph), "pH");
    }

    #[test]
    fn test_unit_equality() {
        assert_eq!(Unit::Abv, Unit::Abv);
        assert_ne!(Unit::Abv, Unit::Percent);
        assert_ne!(Unit::Ph, Unit::SpecificGravity);
    }

    #[test]
    fn test_unit_clone() {
        let unit = Unit::SpecificGravity;
        let cloned = unit;
        assert_eq!(unit, cloned);
    }

    #[test]
    fn test_unit_copy() {
        let unit1 = Unit::Abv;
        let unit2 = unit1;
        assert_eq!(unit1, unit2);
    }
}
