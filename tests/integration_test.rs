use mazerion_core::{get_all_calculators, find_calculator, CalcInput};
use rust_decimal::prelude::*;

#[test]
fn test_all_calculators_registered() {
    mazerion_calculators::init();
    let calculators = get_all_calculators();
    assert_eq!(calculators.len(), 11);
}

#[test]
fn test_find_calculator_by_id() {
    mazerion_calculators::init();

    assert!(find_calculator("abv").is_some());
    assert!(find_calculator("brix_to_sg").is_some());
    assert!(find_calculator("sg_correction").is_some());
    assert!(find_calculator("dilution").is_some());
    assert!(find_calculator("blending").is_some());
    assert!(find_calculator("nutrition").is_some());
    assert!(find_calculator("carbonation").is_some());
    assert!(find_calculator("acid_addition").is_some());
    assert!(find_calculator("sulfite").is_some());
    assert!(find_calculator("backsweetening").is_some());
    assert!(find_calculator("refractometer").is_some());
    assert!(find_calculator("nonexistent").is_none());
}

#[test]
fn test_complete_mead_workflow() {
    mazerion_calculators::init();

    // Step 1: Calculate expected ABV
    let abv_calc = find_calculator("abv").unwrap();
    let abv_input = CalcInput::new()
        .with_decimal("og", dec!(1.120))
        .with_decimal("fg", dec!(1.000));
    let abv_result = abv_calc.calculate(abv_input).unwrap();
    assert!((abv_result.primary.value - dec!(15.75)).abs() < dec!(0.01));

    // Step 2: Calculate nutrition needs
    let nutrition_calc = find_calculator("nutrition").unwrap();
    let nutrition_input = CalcInput::new()
        .with_decimal("volume", dec!(20.0))
        .with_decimal("brix", dec!(25.0));
    let nutrition_result = nutrition_calc.calculate(nutrition_input).unwrap();
    assert!(nutrition_result.primary.value > dec!(0.0));

    // Step 3: Calculate stabilization
    let sulfite_calc = find_calculator("sulfite").unwrap();
    let sulfite_input = CalcInput::new()
        .with_decimal("volume", dec!(20.0))
        .with_decimal("target_so2", dec!(50.0))
        .with_decimal("ph", dec!(3.5));
    let sulfite_result = sulfite_calc.calculate(sulfite_input).unwrap();
    assert!(sulfite_result.primary.value > dec!(0.0));

    // Step 4: Calculate backsweetening
    let backsweeten_calc = find_calculator("backsweetening").unwrap();
    let backsweeten_input = CalcInput::new()
        .with_decimal("volume", dec!(20.0))
        .with_decimal("current_sg", dec!(1.000))
        .with_decimal("target_sg", dec!(1.015))
        .with_string("sugar_type", "honey");
    let backsweeten_result = backsweeten_calc.calculate(backsweeten_input).unwrap();
    assert!(backsweeten_result.primary.value > dec!(0.0));
}

#[test]
fn test_dilution_then_carbonation_workflow() {
    mazerion_calculators::init();

    // Step 1: Dilute high ABV mead
    let dilution_calc = find_calculator("dilution").unwrap();
    let dilution_input = CalcInput::new()
        .with_decimal("volume", dec!(20.0))
        .with_decimal("current_abv", dec!(18.0))
        .with_decimal("target_abv", dec!(14.0));
    let dilution_result = dilution_calc.calculate(dilution_input).unwrap();
    let water_needed = dilution_result.primary.value;
    let new_volume = dilution_result.secondary[0].value;

    // Step 2: Calculate carbonation for new volume
    let carbonation_calc = find_calculator("carbonation").unwrap();
    let carbonation_input = CalcInput::new()
        .with_decimal("volume", new_volume)
        .with_decimal("co2", dec!(2.5))
        .with_string("sugar_type", "table");
    let carbonation_result = carbonation_calc.calculate(carbonation_input).unwrap();
    assert!(carbonation_result.primary.value > dec!(0.0));
}

#[test]
fn test_error_handling_consistency() {
    mazerion_calculators::init();

    for calc in get_all_calculators() {
        // Empty input should error
        let result = calc.calculate(CalcInput::new());
        assert!(result.is_err(), "{} should error on empty input", calc.id());
    }
}