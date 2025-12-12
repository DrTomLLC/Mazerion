//! Beer calculators tab with comprehensive style guides
//! v0.11.1 - EXPANDED VERSION with 20+ styles

use crate::{MazerionApp, state::BeerCalculator};
use eframe::egui::{self, RichText, Color32, Stroke, CornerRadius};
use mazerion_core::{CalcInput, Measurement, Unit};
use rust_decimal::Decimal;
use std::str::FromStr;

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    let c = app.state.custom_colors;  // Copy instead of borrow

    ui.heading(RichText::new("🍺 Beer Brewing").color(c.saddle_brown).size(24.0));
    ui.add_space(10.0);

    ui.horizontal(|ui| {
        if ui.selectable_label(app.beer_calc == BeerCalculator::Ibu, "IBU").clicked() {
            app.beer_calc = BeerCalculator::Ibu;
        }
        if ui.selectable_label(app.beer_calc == BeerCalculator::Srm, "SRM").clicked() {
            app.beer_calc = BeerCalculator::Srm;
        }
        if ui.selectable_label(app.beer_calc == BeerCalculator::Mash, "Mash").clicked() {
            app.beer_calc = BeerCalculator::Mash;
        }
        if ui.selectable_label(app.beer_calc == BeerCalculator::Efficiency, "Efficiency").clicked() {
            app.beer_calc = BeerCalculator::Efficiency;
        }
        if ui.selectable_label(app.beer_calc == BeerCalculator::StyleGuide, "📚 Style Guide").clicked() {
            app.beer_calc = BeerCalculator::StyleGuide;
        }
    });

    ui.add_space(15.0);
    ui.separator();
    ui.add_space(15.0);

    egui::Frame::new()
        .fill(c.light_cream)
        .stroke(Stroke::new(1.5, c.honey_gold))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            match app.beer_calc {
                BeerCalculator::Ibu => render_ibu(app, ui, c),
                BeerCalculator::Srm => render_srm(app, ui, c),
                BeerCalculator::Mash => render_mash(app, ui, c),
                BeerCalculator::Efficiency => render_efficiency(app, ui, c),
                BeerCalculator::StyleGuide => render_style_guide(ui, c),
            }
        });
}

fn render_ibu(app: &mut MazerionApp, ui: &mut egui::Ui, c: crate::state::CustomColors) {
    ui.heading(RichText::new("🌿 IBU Calculator (Tinseth)").color(c.saddle_brown));
    ui.label(RichText::new("Calculate International Bitterness Units using Tinseth formula").color(c.dark_text));
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "L" } else { "gal" };
    let weight_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "g" } else { "oz" };

    crate::input_field(ui, &format!("Hop Weight ({}):", weight_unit), &mut app.hop_weight, "Weight of hops");
    crate::input_field(ui, "Alpha Acid %:", &mut app.alpha_acid, "Alpha acid percentage (e.g., 12.5)");
    crate::input_field(ui, "Boil Time (min):", &mut app.boil_time, "Boil duration in minutes");
    crate::input_field(ui, &format!("Beer Volume ({}):", vol_unit), &mut app.beer_volume, "Final volume");
    crate::input_field(ui, "Boil Gravity:", &mut app.boil_gravity, "Specific gravity during boil (e.g., 1.050)");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate IBU") {
        calc_ibu(app);
    }
}

fn render_srm(app: &mut MazerionApp, ui: &mut egui::Ui, c: crate::state::CustomColors) {
    ui.heading(RichText::new("🎨 SRM Color Calculator").color(c.saddle_brown));
    ui.label(RichText::new("Calculate beer color in Standard Reference Method").color(c.dark_text));
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "L" } else { "gal" };
    let weight_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "kg" } else { "lb" };

    crate::input_field(ui, &format!("Grain Weight ({}):", weight_unit), &mut app.grain_weight, "Weight of grain");
    crate::input_field(ui, "Grain Lovibond:", &mut app.grain_lovibond, "Color rating (e.g., 2.5 for pale malt)");
    crate::input_field(ui, &format!("Beer Volume ({}):", vol_unit), &mut app.beer_volume, "Final volume");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate SRM") {
        calc_srm(app);
    }
}

fn render_mash(app: &mut MazerionApp, ui: &mut egui::Ui, c: crate::state::CustomColors) {
    ui.heading(RichText::new("🌡️ Mash Water Calculator").color(c.saddle_brown));
    ui.label(RichText::new("Calculate strike water temperature and volume").color(c.dark_text));
    ui.add_space(10.0);

    let temp_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "°C" } else { "°F" };
    let weight_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "kg" } else { "lb" };

    crate::input_field(ui, &format!("Target Mash Temp ({}):", temp_unit), &mut app.mash_target_temp, "Desired mash temperature");
    crate::input_field(ui, &format!("Grain Temp ({}):", temp_unit), &mut app.grain_temp, "Initial grain temperature");
    crate::input_field(ui, &format!("Grain Weight ({}):", weight_unit), &mut app.grain_weight, "Total grain weight");
    crate::input_field(ui, "Mash Ratio (L/kg or qt/lb):", &mut app.mash_ratio, "Water to grain ratio");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Strike Water") {
        calc_mash(app);
    }
}

fn render_efficiency(app: &mut MazerionApp, ui: &mut egui::Ui, c: crate::state::CustomColors) {
    ui.heading(RichText::new("⚡ Brewhouse Efficiency").color(c.saddle_brown));
    ui.label(RichText::new("Calculate actual efficiency from grain bill").color(c.dark_text));
    ui.add_space(10.0);

    let weight_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "kg" } else { "lb" };

    crate::input_field(ui, &format!("Grain Weight ({}):", weight_unit), &mut app.grain_weight, "Total grain weight");
    crate::input_field(ui, "Grain PPG:", &mut app.grain_ppg, "Points per pound per gallon (e.g., 37 for 2-row)");
    crate::input_field(ui, "Measured Gravity:", &mut app.measured_gravity, "Actual measured OG");
    crate::input_field(ui, &format!("Beer Volume ({}):", if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "L" } else { "gal" }), &mut app.beer_volume, "Final volume");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Efficiency") {
        calc_efficiency(app);
    }
}

fn render_style_guide(ui: &mut egui::Ui, c: crate::state::CustomColors) {
    ui.heading(RichText::new("📚 COMPREHENSIVE BEER STYLE GUIDE").color(c.saddle_brown).size(18.0));
    ui.label(RichText::new("Complete reference for 20+ beer styles with specifications and brewing notes").color(c.dark_text));
    ui.add_space(15.0);

    // AMERICAN LAGERS
    section_header(ui, "🇺🇸 AMERICAN LAGERS", "Clean, crisp, highly drinkable", c.honey_gold, c);
    style_grid(ui, &[
        ("American Light Lager", "OG: 1.028-1.040 | FG: 1.004-1.010 | ABV: 2.8-4.2%", "IBU: 8-12 | SRM: 2-3", "Ultra-light body, minimal malt character, very subtle hop aroma. Clean fermentation, highly carbonated. Examples: Bud Light, Coors Light, Miller Lite.", "Brewing: Use 20-40% adjuncts (rice/corn), minimal hops, lager at 32-35°F for 3+ weeks. Water: soft, low mineral. Mash: 148-150°F for max attenuation."),
        ("American Lager", "OG: 1.040-1.050 | FG: 1.006-1.010 | ABV: 4.2-5.3%", "IBU: 8-15 | SRM: 2-4", "Light-bodied, crisp, clean. Subtle malt sweetness balanced by light hop bitterness. Highly carbonated, refreshing. Examples: Budweiser, Coors, Miller High Life.", "Brewing: 20-40% adjuncts (flaked corn/rice), noble hops for light bitterness, cold ferment (48-54°F), extended lagering (4-6 weeks) for crispness."),
        ("Premium Lager", "OG: 1.046-1.056 | FG: 1.008-1.012 | ABV: 4.6-6.0%", "IBU: 15-25 | SRM: 2-6", "More substantial than light lagers. Grainy-sweet malt, light hop flavor. Crisp, clean finish. Examples: Michelob, Corona Extra, Heineken.", "Brewing: All-malt or minimal adjuncts (<15%), longer lagering period (6-8 weeks) for smoothness. Mash 150-152°F for body."),
    ], c);

    ui.add_space(20.0);

    // PILSNERS
    section_header(ui, "🍺 PILSNERS", "Golden, hoppy lagers with noble hop character", Color32::from_rgb(255, 215, 0), c);
    style_grid(ui, &[
        ("German Pilsner", "OG: 1.044-1.050 | FG: 1.008-1.013 | ABV: 4.4-5.2%", "IBU: 25-45 | SRM: 2-5", "Light gold, crisp, dry. Spicy, floral noble hops. Clean malt character. Examples: Bitburger, König Pilsener, Jever.", "Brewing: 100% Pilsner malt, decoction or step mash optional. Saazer/Hallertau hops. Soft water (<100 ppm), pH 5.2-5.4. Lager 4-6 weeks."),
        ("Czech Pilsner", "OG: 1.044-1.060 | FG: 1.013-1.017 | ABV: 4.2-5.8%", "IBU: 30-45 | SRM: 3.5-6", "Rich, complex malt. Spicy Saaz hops. Full-bodied for a lager. Diacetyl acceptable. Examples: Pilsner Urquell, Budvar.", "Brewing: Czech Pilsner malt, decoction mash traditional. Saaz hops throughout boil. Soft water. Diacetyl rest at 55-58°F. Lager 6-8 weeks."),
    ], c);

    ui.add_space(20.0);

    // PALE ALES & IPAs
    section_header(ui, "🌾 PALE ALES & IPAs", "Hop-forward, balanced to aggressive bitterness", c.forest_green, c);
    style_grid(ui, &[
        ("American Pale Ale", "OG: 1.045-1.060 | FG: 1.010-1.015 | ABV: 4.5-6.2%", "IBU: 30-50 | SRM: 5-10", "Medium-light to medium body. Citrus, floral, piney hops. Moderate maltiness supports hop character. Examples: Sierra Nevada Pale Ale, Dale's Pale Ale.", "Brewing: American 2-row (90-95%) + crystal (5-10%). Cascade/Centennial hops. Ferment 65-68°F. Dry hop 3-5 days. Water: moderate sulfate (150-200 ppm)."),
        ("India Pale Ale", "OG: 1.056-1.070 | FG: 1.010-1.015 | ABV: 5.5-7.5%", "IBU: 40-70 | SRM: 6-14", "Medium-bodied, hoppy, bitter. Citrus, pine, tropical fruit hops. Dry finish. Examples: Lagunitas IPA, Stone IPA, Bell's Two Hearted.", "Brewing: High hopping rates (1-2 oz/gal). Late additions + dry hop. Attenuative yeast. Mash 150-152°F. Water: sulfate-forward (2:1 sulfate:chloride)."),
        ("Double IPA", "OG: 1.065-1.085 | FG: 1.010-1.020 | ABV: 7.5-10.0%", "IBU: 60-120 | SRM: 6-14", "Full-bodied, intensely hoppy. Huge hop aroma and flavor. Warming alcohol, dry finish. Examples: Pliny the Elder, Heady Topper, Hopslam.", "Brewing: Massive hop additions (2-4 oz/gal). Multiple dry hop charges. High OG with sugar addition (10-15%). Extended boil (90 min). Very high attenuation."),
        ("New England IPA", "OG: 1.060-1.085 | FG: 1.010-1.015 | ABV: 6.0-9.0%", "IBU: 25-60 | SRM: 3-7", "Hazy, juicy, soft. Tropical fruit, citrus, low perceived bitterness. Smooth, pillowy mouthfeel. Examples: Hazy Little Thing, Julius, Heady Topper.", "Brewing: Wheat/oats (20-30%), chloride-heavy water (2:1 chloride:sulfate). Biotransform hops during fermentation. Low bitterness (<30 IBU from boil). Heavy dry hop (3+ oz/gal)."),
        ("Session IPA", "OG: 1.040-1.050 | FG: 1.008-1.012 | ABV: 3.5-5.0%", "IBU: 40-60 | SRM: 4-7", "Light-bodied, hoppy, sessionable. Hop-forward despite low ABV. Dry, crisp finish. Examples: Founders All Day IPA, Stone Go To.", "Brewing: High hop rates relative to gravity. Dry hop for aroma. Mash low (148°F) for attenuation. Water: sulfate-forward. Keep IBU:OG ratio >1."),
    ], c);

    ui.add_space(20.0);

    // BROWN & AMBER ALES
    section_header(ui, "🟤 BROWN & AMBER ALES", "Malty, nutty, balanced ales", Color32::from_rgb(139, 90, 43), c);
    style_grid(ui, &[
        ("American Brown", "OG: 1.045-1.060 | FG: 1.010-1.016 | ABV: 4.3-6.2%", "IBU: 20-40 | SRM: 18-35", "Malty, caramel, chocolate, nutty. More hop character than English. Examples: Brooklyn Brown, Avery Ellie's Brown.", "Brewing: Crystal malts (10-15%), chocolate malt (3-5%). American hops for balance. Ferment 65-68°F. Clean fermentation profile."),
        ("English Brown", "OG: 1.040-1.052 | FG: 1.008-1.013 | ABV: 4.2-5.4%", "IBU: 20-30 | SRM: 12-22", "Malty, caramel, nutty, low esters. Light chocolate. Examples: Newcastle Brown, Sam Smith Nut Brown.", "Brewing: English pale + crystal + chocolate. English hops (Fuggles, EK Goldings). English ale yeast. Moderate hardness water."),
    ], c);

    ui.add_space(20.0);

    // DARK ALES - PORTERS & STOUTS
    section_header(ui, "🖤 PORTERS & STOUTS", "Roasty, rich, complex dark ales", Color32::from_rgb(0, 0, 0), c);
    style_grid(ui, &[
        ("Robust Porter", "OG: 1.048-1.065 | FG: 1.012-1.016 | ABV: 4.8-6.5%", "IBU: 25-50 | SRM: 22-35", "Dark brown to black, roasty. Coffee, dark chocolate, caramel. Medium to full body. Examples: Founder's Porter, Deschutes Black Butte.", "Brewing: Pale base (75%) + crystal (10%) + chocolate/black (10-15%). Roasted barley optional. Moderate hops. Ferment 65-68°F."),
        ("Baltic Porter", "OG: 1.060-1.090 | FG: 1.016-1.024 | ABV: 6.5-9.5%", "IBU: 20-40 | SRM: 17-30", "Smooth, complex, malty. Dark fruit, caramel, roast. Lager yeast. Examples: Sinebrychoff Porter, Okocim Porter.", "Brewing: Munich/Vienna base + caramel + roasted. Lager yeast fermented warm (60-65°F) then lagered. Low hop bitterness. Decoction mash optional."),
        ("Dry Stout", "OG: 1.036-1.050 | FG: 1.007-1.011 | ABV: 4.0-5.0%", "IBU: 25-45 | SRM: 25-40", "Black, roasted, dry. Coffee, roasted barley, low sweetness. Creamy head. Examples: Guinness Draught, Murphy's Stout.", "Brewing: Pale malt base (70-80%) + roasted barley (8-10%) + flaked barley (10%). High carbonate water. Nitrogen or high CO₂. Dry finish."),
        ("Sweet/Milk Stout", "OG: 1.044-1.060 | FG: 1.012-1.024 | ABV: 4.0-6.0%", "IBU: 20-40 | SRM: 30-40", "Black, sweet, creamy. Chocolate, coffee, lactose sweetness. Full body. Examples: Left Hand Milk Stout, Mackeson's.", "Brewing: Similar to dry stout + lactose (8-12 oz/5 gal) at end of boil. Lower attenuation. Smooth, sweet finish."),
        ("Imperial Stout", "OG: 1.075-1.115 | FG: 1.018-1.030 | ABV: 8.0-12.0%", "IBU: 50-90 | SRM: 30-40", "Intense, complex, warming. Dark fruit, chocolate, espresso, roast. Full-bodied, velvety. Age-worthy. Examples: North Coast Old Rasputin, Bell's Expedition.", "Brewing: High gravity, complex malt bill. Extended boil (90-120 min). Strong yeast pitch. Age 3-12 months. Oak/bourbon barrel optional."),
    ], c);

    ui.add_space(20.0);

    // BELGIAN ALES
    section_header(ui, "🇧🇪 BELGIAN ALES", "Fruity esters, spicy phenolics, complex yeast", Color32::from_rgb(184, 134, 11), c);
    style_grid(ui, &[
        ("Belgian Blonde", "OG: 1.062-1.075 | FG: 1.008-1.018 | ABV: 6.0-7.5%", "IBU: 15-30 | SRM: 4-7", "Golden, smooth, deceptively strong. Fruity, spicy yeast character. Dry finish. Examples: Leffe Blonde, La Chouffe.", "Brewing: Pilsner malt base (85-90%), Belgian candi sugar (5-15%). Belgian ale yeast. Ferment 68-78°F for esters. High attenuation."),
        ("Dubbel", "OG: 1.062-1.075 | FG: 1.008-1.018 | ABV: 6.0-7.6%", "IBU: 15-25 | SRM: 10-17", "Rich, malty, complex. Dark fruit (raisin, plum), caramel, spice. Moderate body. Examples: Westmalle Dubbel, Chimay Red.", "Brewing: Dark Belgian candi sugar (10-15%), Munich/Special B/Cara malts. Low hops. High fermentation temp (70-78°F) for esters/phenols."),
        ("Tripel", "OG: 1.075-1.085 | FG: 1.008-1.014 | ABV: 7.5-9.5%", "IBU: 20-40 | SRM: 4.5-7", "Strong, pale, complex. Spicy, fruity, peppery phenolics. Highly attenuated. Examples: Westmalle Tripel, La Fin du Monde.", "Brewing: Pilsner malt, clear Belgian candi sugar (15-20%). Aggressive Belgian yeast. Warm fermentation (72-78°F). High carbonation (3+ vol)."),
        ("Quadrupel", "OG: 1.075-1.110 | FG: 1.010-1.024 | ABV: 8.0-12.0%", "IBU: 20-40 | SRM: 12-20", "Dark, rich, complex. Dark fruit, caramel, spice, warming alcohol. Full-bodied. Age-worthy. Examples: St. Bernardus Abt 12, Rochefort 10.", "Brewing: Dark candi sugar/syrup (15-25%), complex malt bill. Extended fermentation. Warm fermentation temps. Age 6+ months for complexity."),
        ("Saison", "OG: 1.048-1.065 | FG: 1.002-1.008 | ABV: 5.0-7.0%", "IBU: 20-35 | SRM: 5-14", "Dry, effervescent, spicy/fruity. Peppery, citrus, herbal. Highly attenuated. Examples: Saison Dupont, Boulevard Tank 7.", "Brewing: Pilsner base, wheat (10-20%). Saison yeast. High attenuation (<1.005). Warm fermentation (75-85°F). High carbonation (3.5+ vol)."),
    ], c);

    ui.add_space(20.0);

    // WHEAT BEERS
    section_header(ui, "🌾 WHEAT BEERS", "Wheat malt forward, often hazy and refreshing", Color32::from_rgb(255, 235, 205), c);
    style_grid(ui, &[
        ("American Wheat", "OG: 1.040-1.055 | FG: 1.008-1.013 | ABV: 4.0-5.5%", "IBU: 15-30 | SRM: 3-6", "Clean wheat beer. Light hop character, no banana/clove. Refreshing. Examples: Bell's Oberon, Boulevard Wheat.", "Brewing: 30-50% wheat malt, American ale yeast. Light hopping. Bright, clean fermentation. May clarify or serve hazy."),
        ("Hefeweizen", "OG: 1.044-1.052 | FG: 1.010-1.014 | ABV: 4.3-5.6%", "IBU: 8-15 | SRM: 2-8", "Bavarian wheat beer. Banana, clove, bubblegum esters. Hazy, effervescent. Examples: Weihenstephaner, Paulaner.", "Brewing: 50-70% wheat malt, German weizen yeast. Low hops. Warm fermentation (64-70°F) for phenols/esters. Bottle condition. Never filter."),
        ("Witbier", "OG: 1.044-1.052 | FG: 1.008-1.012 | ABV: 4.5-5.5%", "IBU: 10-20 | SRM: 2-4", "Belgian wheat with spices. Orange peel, coriander. Tart, refreshing. Hazy. Examples: Hoegaarden, Allagash White.", "Brewing: 40-50% unmalted wheat, oats (5-10%). Orange peel + coriander in last 5 min. Belgian wit yeast. Low attenuation for body. Hazy."),
    ], c);

    ui.add_space(20.0);

    // STRONG ALES
    section_header(ui, "💪 STRONG & SPECIALTY", "High gravity, complex, warming", c.dark_red, c);
    style_grid(ui, &[
        ("Barleywine", "OG: 1.080-1.120 | FG: 1.018-1.030 | ABV: 8.0-12.0%", "IBU: 50-100 | SRM: 10-19", "Rich, complex, strong. Caramel, toffee, fruit, warming. Full-bodied, age-worthy. Examples: Sierra Nevada Bigfoot, Anchor Old Foghorn.", "Brewing: Massive grain bill (20-25 lb/5 gal). Extended boil (2+ hrs). High hopping. Strong yeast starter. Age 6-12 months minimum."),
        ("Scotch Ale", "OG: 1.070-1.130 | FG: 1.018-1.056 | ABV: 6.5-10.0%", "IBU: 17-35 | SRM: 14-25", "Malty, rich, sweet. Caramel, toffee. Full-bodied, low hops. Examples: Founders Dirty Bastard, Traquair House.", "Brewing: Crystal/cara malts heavy (20-30%). Low hopping. Long boil for caramelization. Scottish ale yeast. Mash high (158°F)."),
        ("English Barleywine", "OG: 1.080-1.120 | FG: 1.018-1.030 | ABV: 8.0-12.0%", "IBU: 35-70 | SRM: 8-22", "Malty, fruity, complex. Toffee, bread, dark fruit. Balanced hops. Examples: J.W. Lees Harvest Ale, Fuller's Golden Pride.", "Brewing: English pale/Maris Otter base. Crystal malts. English hops (Target, Challenger). English ale yeast. Age 6+ months."),
    ], c);

    ui.add_space(20.0);

    // GERMAN LAGERS
    section_header(ui, "🇩🇪 GERMAN LAGERS", "Traditional lagers with malt focus", Color32::from_rgb(205, 133, 63), c);
    style_grid(ui, &[
        ("Märzen/Oktoberfest", "OG: 1.054-1.060 | FG: 1.010-1.014 | ABV: 5.8-6.3%", "IBU: 18-25 | SRM: 8-17", "Amber, malty, smooth. Toasted bread, caramel. Clean lager. Examples: Ayinger Oktoberfest, Paulaner Oktoberfest.", "Brewing: Vienna/Munich malts (50-100%). Decoction mash traditional. German noble hops. Lager 6-8 weeks. Soft to moderate hardness water."),
        ("Bock", "OG: 1.064-1.072 | FG: 1.013-1.019 | ABV: 6.3-7.2%", "IBU: 20-27 | SRM: 14-22", "Malty, toasty, sweet. Rich malt character, minimal hops. Examples: Ayinger Celebrator, Einbecker Ur-Bock.", "Brewing: Munich malt base (50%+). Decoction mash. Low hopping. Lager 8-10 weeks. Mash 154-156°F for body."),
        ("Doppelbock", "OG: 1.072-1.112 | FG: 1.016-1.024 | ABV: 7.0-10.0%", "IBU: 16-26 | SRM: 6-25", "Strong, malty, rich. Bread, toast, caramel. Full-bodied. Examples: Paulaner Salvator, Ayinger Celebrator.", "Brewing: Heavy Munich malt base (75%+). Extended boil. Low hops. Strong lager yeast starter. Lager 10-12 weeks."),
        ("Schwarzbier", "OG: 1.046-1.052 | FG: 1.010-1.016 | ABV: 4.4-5.4%", "IBU: 20-30 | SRM: 17-30", "Black lager. Roasty, clean, smooth. Coffee, chocolate, no harshness. Examples: Köstritzer Schwarzbier.", "Brewing: Pilsner base + roasted malts (5-10%). Smooth roast character. German hops. Lager 6-8 weeks. Avoid astringency."),
    ], c);

    ui.add_space(30.0);
    ui.separator();
    ui.add_space(20.0);

    // PROFESSIONAL TIPS
    ui.heading(RichText::new("💡 PROFESSIONAL BREWING GUIDANCE").size(16.0).color(c.saddle_brown));
    ui.add_space(10.0);

    ui.label(RichText::new("🎯 IBU Management:").strong().color(c.saddle_brown));
    ui.label("• Perceived bitterness affected by residual sweetness - higher OG needs more IBU for balance");
    ui.label("• Late additions (<15 min) add flavor/aroma with minimal bitterness contribution");
    ui.label("• Dry hopping adds zero IBU - only aroma and flavor compounds");
    ui.label("• Whirlpool/hop stand at 175-185°F: maximum flavor, minimal bitterness extraction");
    ui.label("• First wort hopping (add hops before sparging): smooth, integrated bitterness");

    ui.add_space(10.0);

    ui.label(RichText::new("🎨 SRM Color Guide:").strong().color(c.saddle_brown));
    ui.label("• SRM 2-4: Pale straw (light lagers, wheat beers)");
    ui.label("• SRM 5-10: Gold to light amber (pale ales, IPAs, pilsners)");
    ui.label("• SRM 11-20: Amber to copper (brown ales, bocks, märzen)");
    ui.label("• SRM 21-30: Brown to very dark brown (porters, robust stouts)");
    ui.label("• SRM 30+: Black, opaque (imperial stouts, schwarzbier)");
    ui.label("• Roasted malts add color faster than flavor - use restraint (<10% of grist)");

    ui.add_space(10.0);

    ui.label(RichText::new("🌡️ Mash Temperature Guide:").strong().color(c.saddle_brown));
    ui.label("• 148-153°F (64-67°C): High fermentability, dry finish, light body (IPAs, saisons)");
    ui.label("• 154-158°F (68-70°C): Balanced fermentability, moderate body and sweetness (most ales)");
    ui.label("• 159-162°F (71-72°C): Low fermentability, sweet finish, full body (stouts, scotch ales)");
    ui.label("• Single infusion: Hold target temp 60-90 minutes for complete conversion");
    ui.label("• Step mash: Multiple rests optimize for delicate beers (pilsners, lagers)");
    ui.label("• Decoction: Boil portion of mash, traditional for German beers (depth, melanoidins)");

    ui.add_space(10.0);

    ui.label(RichText::new("💧 Water Chemistry by Style:").strong().color(c.saddle_brown));
    ui.label("• Pilsner Profile: Soft water (50-100 ppm total dissolved solids), low sulfate, low chloride");
    ui.label("• IPA Profile: High sulfate (200-400 ppm), moderate chloride (50-100 ppm), accentuates hop bitterness");
    ui.label("• Stout Profile: Moderate hardness (200-300 ppm), higher chloride for smoothness, carbonate for roast");
    ui.label("• Sulfate:Chloride Ratio: >2:1 for hop-forward (IPA), <1:2 for malt-forward (brown, porter)");
    ui.label("• Mash pH Target: 5.2-5.6 optimal for enzyme activity and flavor");
    ui.label("• Finished Beer pH: 4.0-4.5 for stability and flavor");

    ui.add_space(10.0);

    ui.label(RichText::new("⚗️ Yeast & Fermentation:").strong().color(c.saddle_brown));
    ui.label("• Pitch Rate: 0.75M cells/mL/°P for ales, 1.5M cells/mL/°P for lagers (use calculator)");
    ui.label("• Temperature Control: ±2°F critical for clean fermentation, prevents off-flavors");
    ui.label("• Ale Fermentation: 65-72°F (18-22°C), higher temps = more esters/phenols");
    ui.label("• Lager Fermentation: 48-58°F (9-14°C), extended lagering for smoothness");
    ui.label("• Oxygenation: 8-12 ppm dissolved oxygen essential for healthy yeast growth");
    ui.label("• Diacetyl Rest (lagers): Raise to 65°F for last 2 days before lagering");

    ui.add_space(10.0);

    ui.label(RichText::new("🔬 Advanced Techniques:").strong().color(c.saddle_brown));
    ui.label("• Parti-Gyle: Split single mash into multiple beers (strong + session from one brew day)");
    ui.label("• Decoction Mash: Boil portion of mash, adds depth and color (traditional German lagers)");
    ui.label("• Hop Bursting: Concentrate all hops late in boil (last 20 min) for flavor without harsh bitterness");
    ui.label("• Kveik Yeast: Norwegian farmhouse yeast, ferments hot (85-100°F), extremely fast turnaround");
    ui.label("• Pressure Fermentation: 10-15 PSI reduces esters, cleaner fermentation, faster conditioning");
    ui.label("• Biotransformation: Add hops during active fermentation for enzyme-driven flavor enhancement");
}

// Section helpers
fn section_header(ui: &mut egui::Ui, title: &str, desc: &str, color: Color32, c: crate::state::CustomColors) {
    ui.heading(RichText::new(title).size(16.0).color(color).strong());
    ui.label(RichText::new(desc).size(12.0).color(c.dark_text).weak());
    ui.add_space(8.0);
}

fn style_grid(ui: &mut egui::Ui, data: &[(&str, &str, &str, &str, &str)], c: crate::state::CustomColors) {
    egui::Grid::new(ui.next_auto_id())
        .num_columns(5)
        .spacing([10.0, 6.0])
        .striped(true)
        .show(ui, |ui| {
            // Header
            ui.label(RichText::new("Style").strong().color(c.saddle_brown));
            ui.label(RichText::new("Gravity/ABV").strong().color(c.saddle_brown));
            ui.label(RichText::new("IBU/SRM").strong().color(c.saddle_brown));
            ui.label(RichText::new("Character & Examples").strong().color(c.saddle_brown));
            ui.label(RichText::new("Brewing Notes").strong().color(c.saddle_brown));
            ui.end_row();

            // Data rows with text wrapping and allocated widths
            for (style, gravity, ibu_srm, char, brew) in data {
                ui.label(RichText::new(*style).strong().color(c.saddle_brown).size(11.0));
                ui.label(RichText::new(*gravity).color(c.honey_gold).size(10.0));
                ui.label(RichText::new(*ibu_srm).color(c.honey_gold).size(10.0));

                // Character column - allocate width and wrap
                ui.vertical(|ui| {
                    ui.set_width(250.0);
                    ui.add(egui::Label::new(RichText::new(*char).size(10.0)).wrap());
                });

                // Brewing notes column - allocate width and wrap
                ui.vertical(|ui| {
                    ui.set_width(300.0);
                    ui.add(egui::Label::new(RichText::new(*brew).size(10.0).weak()).wrap());
                });

                ui.end_row();
            }
        });
}

// Calculator implementations
fn calc_ibu(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("ibu") {
        Some(c) => c,
        None => {
            app.result = Some("Error: IBU calculator not found".to_string());
            return;
        }
    };

    let hop_weight: f64 = match app.hop_weight.parse() {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid hop weight".to_string());
            return;
        }
    };

    let alpha: f64 = match app.alpha_acid.parse() {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid alpha acid %".to_string());
            return;
        }
    };

    let time: f64 = match app.boil_time.parse() {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid boil time".to_string());
            return;
        }
    };

    let volume: f64 = match app.beer_volume.parse() {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid volume".to_string());
            return;
        }
    };

    let gravity: f64 = match app.boil_gravity.parse() {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid gravity".to_string());
            return;
        }
    };

    let dec_weight = match Decimal::from_str(&format!("{:.3}", hop_weight)) {
        Ok(d) => d,
        Err(_) => {
            app.result = Some("Error: Conversion failed".to_string());
            return;
        }
    };

    let dec_alpha = match Decimal::from_str(&format!("{:.2}", alpha)) {
        Ok(d) => d,
        Err(_) => {
            app.result = Some("Error: Conversion failed".to_string());
            return;
        }
    };

    let dec_volume = match Decimal::from_str(&format!("{:.2}", volume)) {
        Ok(d) => d,
        Err(_) => {
            app.result = Some("Error: Conversion failed".to_string());
            return;
        }
    };

    let dec_gravity = match Decimal::from_str(&format!("{:.4}", gravity)) {
        Ok(d) => d,
        Err(_) => {
            app.result = Some("Error: Conversion failed".to_string());
            return;
        }
    };

    let input = CalcInput::new()
        .add_measurement(Measurement::new(dec_weight, Unit::Grams))
        .add_measurement(Measurement::new(dec_alpha, Unit::Percent))
        .add_param("boil_time", &time.to_string())
        .add_measurement(Measurement::new(dec_volume, Unit::Liters))
        .add_measurement(Measurement::new(dec_gravity, Unit::SpecificGravity));

    match calc.calculate(input) {
        Ok(res) => {
            app.result = Some(format!("IBU: {:.1}", res.output.value));
            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => {
            app.result = Some(format!("Error: {}", e));
            app.warnings.clear();
            app.metadata.clear();
        }
    }
}

fn calc_srm(app: &mut MazerionApp) {
    let _calc = match mazerion_core::traits::get_calculator("srm") {
        Some(c) => c,
        None => {
            app.result = Some("Error: SRM calculator not found".to_string());
            return;
        }
    };

    // Similar implementation to calc_ibu...
    app.result = Some("SRM calculator - implementation pending".to_string());
}

fn calc_mash(app: &mut MazerionApp) {
    let _calc = match mazerion_core::traits::get_calculator("mash_water") {
        Some(c) => c,
        None => {
            app.result = Some("Error: Mash calculator not found".to_string());
            return;
        }
    };

    app.result = Some("Mash calculator - implementation pending".to_string());
}

fn calc_efficiency(app: &mut MazerionApp) {
    let _calc = match mazerion_core::traits::get_calculator("efficiency") {
        Some(c) => c,
        None => {
            app.result = Some("Error: Efficiency calculator not found".to_string());
            return;
        }
    };

    app.result = Some("Efficiency calculator - implementation pending".to_string());
}