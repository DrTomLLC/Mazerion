//! Finishing tab - PRODUCTION READY WITH COMPREHENSIVE SWEETNESS GUIDE
//! All 6 calculators fully functional + professional sweetness reference

use crate::{MazerionApp, state::colors};
use eframe::egui::{self, RichText, Color32, Stroke, CornerRadius, ScrollArea};
use mazerion_core::{CalcInput, Measurement};
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::state::FinishingCalculator;

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Select Calculator:").strong());
        egui::ComboBox::from_id_salt("finishing_calc")
            .selected_text(get_calc_name(app.state.finishing_calc))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.state.finishing_calc, FinishingCalculator::Backsweetening, "Backsweetening");
                ui.selectable_value(&mut app.state.finishing_calc, FinishingCalculator::Stabilization, "Stabilization");
                ui.selectable_value(&mut app.state.finishing_calc, FinishingCalculator::Sulfite, "Sulfite (Oxidation)");
                ui.selectable_value(&mut app.state.finishing_calc, FinishingCalculator::AcidAddition, "Acid Addition");
                ui.selectable_value(&mut app.state.finishing_calc, FinishingCalculator::Pasteurization, "Pasteurization");
                ui.selectable_value(&mut app.state.finishing_calc, FinishingCalculator::SweetnessChart, "Sweetness Guide");
            });
    });

    ui.add_space(10.0);

    egui::Frame::new()
        .fill(colors::LIGHT_CREAM)
        .stroke(Stroke::new(1.5, colors::HONEY_GOLD))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            match app.state.finishing_calc {
                FinishingCalculator::Backsweetening => render_backsweetening(app, ui),
                FinishingCalculator::Stabilization => render_stabilization(app, ui),
                FinishingCalculator::Sulfite => render_sulfite(app, ui),
                FinishingCalculator::AcidAddition => render_acid(app, ui),
                FinishingCalculator::Pasteurization => render_pasteurization(app, ui),
                FinishingCalculator::SweetnessChart => render_sweetness_guide_comprehensive(ui),
            }
        });
}

fn get_calc_name(calc: FinishingCalculator) -> &'static str {
    match calc {
        FinishingCalculator::Backsweetening => "Backsweetening",
        FinishingCalculator::Stabilization => "Stabilization",
        FinishingCalculator::Sulfite => "Sulfite (Oxidation)",
        FinishingCalculator::AcidAddition => "Acid Addition",
        FinishingCalculator::Pasteurization => "Pasteurization",
        FinishingCalculator::SweetnessChart => "Sweetness Guide",
    }
}

fn render_backsweetening(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍯 Backsweetening Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate sweetener additions to reach target sweetness");
    ui.label(RichText::new("⚠️ MUST stabilize before backsweetening!").color(colors::DARK_ORANGE).strong());
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "L" } else { "gal" };

    crate::input_field(ui, &format!("Volume ({}):", vol_unit), &mut app.sweet_vol, "Total volume to sweeten");
    crate::input_field(ui, "Current SG:", &mut app.current_sg, "Current specific gravity (e.g., 0.998)");
    crate::input_field(ui, "Target SG:", &mut app.target_sg, "Desired final gravity (e.g., 1.015)");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Sweetener:").strong());
        egui::ComboBox::from_id_salt("sweetener")
            .selected_text(&app.sweetener)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.sweetener, "honey".to_string(), "Honey (82% sugar)");
                ui.selectable_value(&mut app.sweetener, "table_sugar".to_string(), "Table Sugar (100% sugar)");
                ui.selectable_value(&mut app.sweetener, "agave".to_string(), "Agave Nectar (76% sugar)");
                ui.selectable_value(&mut app.sweetener, "maple_syrup".to_string(), "Maple Syrup (67% sugar)");
            });
    });

    ui.add_space(5.0);
    ui.label(RichText::new("💡 Tip: Use Stabilization calculator first, wait 24 hours, then backsweeten").size(12.0));

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Sweetener Amount") {
        calc_backsweetening(app);
    }
}

fn render_stabilization(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🛡️ Stabilization Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate K-meta + K-sorbate for preventing re-fermentation");
    ui.label(RichText::new("⚠️ Required before backsweetening!").color(colors::DARK_ORANGE).strong());
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "L" } else { "gal" };

    crate::input_field(ui, &format!("Volume ({}):", vol_unit), &mut app.sulfite_vol, "Total volume to stabilize");
    crate::input_field(ui, "pH:", &mut app.ph, "Current pH (affects sorbate effectiveness)");

    ui.add_space(5.0);
    ui.label(RichText::new("💡 Two-step process:").size(12.0).strong());
    ui.label(RichText::new("   1. Add K-meta, wait 24 hours").size(12.0));
    ui.label(RichText::new("   2. Add K-sorbate, wait 3-7 days before backsweetening").size(12.0));

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Stabilization Doses") {
        calc_stabilization(app);
    }
}

fn render_sulfite(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🔒 Sulfite Calculator (Oxidation Prevention)").color(colors::SADDLE_BROWN));
    ui.label("Calculate K-meta for antioxidant protection only");
    ui.label(RichText::new("ℹ️ This is NOT for stabilization - use Stabilization calculator instead").size(12.0).weak());
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "L" } else { "gal" };

    crate::input_field(ui, &format!("Volume ({}):", vol_unit), &mut app.sulfite_vol, "Total volume to treat");
    crate::input_field(ui, "pH:", &mut app.ph, "Current pH (critical for SO₂ effectiveness!)");
    crate::input_field(ui, "Target Free SO₂ (ppm):", &mut app.target_so2, "Desired free SO₂ level (20-50 ppm typical)");

    ui.add_space(5.0);
    ui.label(RichText::new("💡 Free SO₂ protects against oxidation and spoilage microbes").size(12.0));
    ui.label(RichText::new("   Lower pH = more effective (need less K-meta)").size(12.0));

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate K-meta Dose") {
        calc_sulfite(app);
    }
}

fn render_acid(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍋 Acid Addition Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate acid additions to adjust pH - different acids have different strengths");
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "L" } else { "gal" };

    crate::input_field(ui, &format!("Volume ({}):", vol_unit), &mut app.acid_vol, "Total volume to treat");
    crate::input_field(ui, "Current pH:", &mut app.current_ph, "Current pH measurement");
    crate::input_field(ui, "Target pH:", &mut app.target_ph_acid, "Desired pH (must be lower than current)");

    ui.horizontal(|ui| {
        ui.label(RichText::new("Acid Type:").strong());
        egui::ComboBox::from_id_salt("acid_type")
            .selected_text(&app.acid_type)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.acid_type, "tartaric".to_string(), "Tartaric (strongest, wine standard)");
                ui.selectable_value(&mut app.acid_type, "citric".to_string(), "Citric (bright, fruity)");
                ui.selectable_value(&mut app.acid_type, "malic".to_string(), "Malic (soft, apple-like)");
                ui.selectable_value(&mut app.acid_type, "lactic".to_string(), "Lactic (smooth, creamy)");
            });
    });

    ui.add_space(5.0);
    ui.label(RichText::new("💡 Different acids need different amounts for same pH drop").size(12.0).strong());
    ui.label(RichText::new("   Tartaric: Strongest | Citric: Medium | Malic: Weaker | Lactic: Weakest").size(12.0));

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Acid Addition") {
        calc_acid_addition(app);
    }
}

fn render_pasteurization(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🌡️ Pasteurization Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate time/temperature for safe pasteurization");
    ui.label(RichText::new("⚠️ Alternative to chemical stabilization").color(colors::DARK_ORANGE).strong());
    ui.add_space(10.0);

    let temp_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "°C" } else { "°F" };

    crate::input_field(ui, &format!("Temperature ({}):", temp_unit), &mut app.pasteurization_temp, "Target pasteurization temperature");

    ui.add_space(5.0);
    ui.label(RichText::new("💡 Standard ranges:").size(12.0).strong());
    ui.label(RichText::new("   60-63°C (140-145°F): 60-90 min - best for delicate flavors").size(12.0));
    ui.label(RichText::new("   65°C (149°F): 25-35 min - standard temperature").size(12.0));
    ui.label(RichText::new("   68-70°C (154-158°F): 12-20 min - efficient balance").size(12.0));
    ui.label(RichText::new("   72-75°C (162-167°F): 5-10 min - quick but risks flavor damage").size(12.0));

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Pasteurization Time") {
        calc_pasteurization(app);
    }
}

fn render_sweetness_guide_comprehensive(ui: &mut egui::Ui) {
    ui.heading(RichText::new("📊 COMPREHENSIVE SWEETNESS & FINISH GUIDE").color(colors::SADDLE_BROWN).size(20.0));
    ui.label(RichText::new("Professional reference for all fermented beverages - from novice to sommelier").size(14.0));
    ui.add_space(20.0);

    ScrollArea::vertical().show(ui, |ui| {

        // ========== TRADITIONAL MEAD (HONEY-ONLY) ==========
        section_header(ui, "🍯 TRADITIONAL MEAD (Honey-Only)", "Pure honey fermentation showcasing terroir and varietals");

        sweetness_entry(ui, "Bone Dry", "0.990-0.996", "Crisp & Ethereal", &[
            "TASTE PROFILE: No perceptible sweetness. Bone-dry finish like a crisp white wine. Dominant acidity cuts through with laser precision. Honey character is subtle and ephemeral - you taste the floral/herbal notes of the varietal without sweetness. Finish is clean, bright, refreshing.",
            "MOUTHFEEL: Light-bodied, almost water-like. No viscosity. Sharp, clean texture. High perceived acidity makes mouth water.",
            "ALCOHOL PERCEPTION: ABV feels prominent - nothing to mask it. Best 8-12% ABV range.",
            "FOOD PAIRING: Oysters, sushi, ceviche, fresh goat cheese, green salads, steamed fish, light Asian cuisine. Anything delicate that would be overwhelmed by sweetness.",
            "SERVING: 45-50°F in white wine glasses. Serve well-chilled to emphasize crispness.",
            "YEAST: High attenuators - 71B-1122 (very dry), EC-1118 (desert dry), DV10 (bone dry)",
            "AGING: 6-12 months. Develops into very wine-like character. Acidity softens slightly with age.",
            "PROFESSIONAL NOTES: Look for balance between honey origin character and acidity. Assess cleanness of fermentation. Check for appropriate tannin structure if oak-aged. Evaluate complexity - should show terroir.",
            "BEGINNER TIPS: If you like dry white wines (Sauvignon Blanc, Albariño), start here. Don't expect sweetness - this is for wine lovers who appreciate acidity and minerality.",
            "COMMON MISTAKES: Under-acidifying (tastes flabby/dull), over-oaking (masks honey), serving too warm (loses crispness)",
        ]);

        sweetness_entry(ui, "Dry", "0.996-1.006", "Elegant & Balanced", &[
            "TASTE PROFILE: Barely perceptible sweetness on the finish. Honey character emerges more clearly than bone dry. Floral, herbal, or fruity notes shine through. Still quite wine-like, but with a subtle honeyed kiss at the end. Balanced acidity keeps it refreshing.",
            "MOUTHFEEL: Light to light-medium body. Slight roundness developing. Smooth texture with minimal viscosity.",
            "ALCOHOL PERCEPTION: ABV 10-14% feels well-integrated. Sweetness provides slight buffer.",
            "FOOD PAIRING: Poultry, pork chops, grilled fish with herbs, mild curry, mushroom dishes, brie, camembert, fruit-forward salads, Thai cuisine. Most versatile pairing range.",
            "SERVING: 50-55°F in white wine or ISO tasting glasses. Slightly warmer than bone dry to let honey aromatics develop.",
            "YEAST: Moderate attenuators - D47 (leaves 3-5g/L residual), BA11 (clean, leaves gentle sweetness), QA23 (enhances aromatics)",
            "AGING: 9-18 months. Honey character integrates beautifully. Best balance point for many varietals.",
            "PROFESSIONAL NOTES: This is the 'sweet spot' for traditional meads. Assess honey varietal expression vs. sweetness balance. Look for layered complexity - primary (honey), secondary (fermentation esters), tertiary (aging). Tannin structure should support without dominating.",
            "BEGINNER TIPS: Perfect entry point. Familiar wine-like qualities with unique honey character. Not sweet like dessert, but not aggressively dry. 'Goldilocks zone' for most palates.",
            "COMMERCIAL EXAMPLES: Superstition 'Marion', Schramm's 'The Heart', B. Nektar 'Zombie Killer' (dry version)",
            "VARIETAL SHOWCASE: Orange blossom (citrus notes), wildflower (complex herbaceous), clover (gentle vanilla), buckwheat (earthy/malty)",
        ]);

        sweetness_entry(ui, "Semi-Dry", "1.006-1.015", "Approachable & Honeyed", &[
            "TASTE PROFILE: Noticeable honey sweetness balanced by good acidity. This is where 'mead' character really shines - distinctly honeyed but not dessert-sweet. Floral, fruity notes are prominent. Finish is smooth with lingering honey warmth. Very food-friendly.",
            "MOUTHFEEL: Medium body. Definite viscosity developing. Coating texture without being heavy. Smooth, round mouthfeel.",
            "ALCOHOL PERCEPTION: ABV 12-15% well-masked by sweetness. Feels warming but not hot.",
            "FOOD PAIRING: Rich fish (salmon, tuna), duck, pork belly, aged cheeses (aged gouda, manchego), spicy Asian/Indian dishes, roasted root vegetables, fruit tarts, mild blue cheese.",
            "SERVING: 55-60°F in Bordeaux-style wine glasses. Warmer serving brings out honey aromatics and complexity.",
            "YEAST: Lower attenuators - 71B-1122 cold-stopped early, Red Star Premier Blanc, Lalvin D47 nutrient-starved, sweet mead yeasts",
            "AGING: 12-24 months minimum. Sweetness integrates, honey character deepens, complexity develops. Many improve 3-5 years.",
            "PROFESSIONAL NOTES: Evaluate sweetness-to-acid ratio carefully. Should taste balanced, not cloying. Assess honey quality and expression - this level showcases expensive varietals well. Look for length of finish - quality meads linger. Check for appropriate body relative to sweetness.",
            "BEGINNER TIPS: This tastes like 'mead' to most people. Sweet but not syrupy. Great for holiday meals. If you like off-dry Riesling or Gewürztraminer, you'll enjoy this level.",
            "STABILIZATION: CRITICAL - must stabilize (K-meta + sorbate) before backsweetening or risk bottle bombs. Alternatively, pasteurize.",
            "TROUBLESHOOTING: If tastes too sweet - add acid blend or tannin. If too thin - let age longer. If cloying - needs more acidity.",
        ]);

        sweetness_entry(ui, "Semi-Sweet", "1.015-1.025", "Rich & Luxurious", &[
            "TASTE PROFILE: Clear, present sweetness. Rich honey character dominates. This is 'liquid honey' territory. Floral/fruity notes are lush and full. Still has balancing acidity preventing cloyingness. Finish is long, sweet, warming. Syrupy without being heavy.",
            "MOUTHFEEL: Medium-full body. Noticeable viscosity. Coating, unctuous texture. Weight on the tongue.",
            "ALCOHOL PERCEPTION: ABV 13-16%. Sweetness masks alcohol well - dangerous! Can be deceptively strong.",
            "FOOD PAIRING: Foie gras, duck confit, strong blue cheese (Stilton, Roquefort), sharp aged cheddar, nut-based desserts, caramel desserts, apple pie, pecan pie. After-dinner sipper.",
            "SERVING: 58-62°F in Port glasses or small Bordeaux glasses. Warmer serving emphasizes richness. Small pours (2-3 oz).",
            "YEAST: Very low attenuators or cold-crashed early - Uvaferm 43, Wyeast 4184 (sweet mead), arrested fermentation with ice bath",
            "AGING: 18-36 months minimum. These need time. Sweetness integrates, alcohol mellows, complexity deepens. 5-10 year aging potential.",
            "PROFESSIONAL NOTES: This is dessert mead territory. Assess balance carefully - needs significant acidity and/or tannin to prevent cloying. Evaluate honey quality critically - cheap honey = sweet syrup, good honey = liquid gold. Look for viscosity 'legs' on glass. Check for appropriate alcohol warmth (not burning). Assess finish length - should linger 30+ seconds.",
            "BEGINNER TIPS: This is SWEET. Not wine-sweet, dessert-sweet. Small servings, after dinner. Pairs with dessert or IS dessert. If you like Port, Sauternes, or Ice Wine, try this level.",
            "COMMERCIAL EXAMPLES: Moonlight Meadery 'Desire', Redstone Meadery 'Traditional Mountain Honey Wine', Oliver Winery 'Camelot Mead'",
            "CELLAR-WORTHY: These age beautifully. Buy extra to cellar 5-10 years. Sweetness integrates, honey caramelizes slightly, complexity develops.",
        ]);

        sweetness_entry(ui, "Sweet", "1.025-1.040", "Decadent Dessert", &[
            "TASTE PROFILE: Pronounced sweetness. Very rich, dessert wine character. Honey is liquid and luscious. Intense floral/fruity/caramel notes. Needs strong acidity to balance - without it, becomes cloying. Finish is sweet, long, warming, coating.",
            "MOUTHFEEL: Full body. Very viscous, syrupy texture. Heavy coating feel. Almost liqueur-like.",
            "ALCOHOL PERCEPTION: ABV 14-18%. Well-masked but present as warmth. Can be quite strong.",
            "FOOD PAIRING: Dark chocolate torte, crème brûlée, bread pudding, strong cheese plates, nuts (candied walnuts), dried fruits, cigar pairing. Best alone as dessert.",
            "SERVING: 60-65°F in Port or sherry glasses. 1.5-2 oz pours. Room temperature is fine - emphasizes complexity.",
            "YEAST: Cold-crashed or stabilized around 1.025-1.040. EC-1118 stopped early, champagne yeasts arrested, or deliberate stuck fermentation.",
            "AGING: 2-5 years minimum. These are marathon meads. Can age 10-20+ years. Develop incredible complexity - dried fruit, caramel, oxidative notes (in a good way).",
            "PROFESSIONAL NOTES: Evaluate balance intensely. This sweetness level absolutely requires balancing elements: high acidity (pH 3.0-3.3), tannin structure (oak, tea, skin contact), and/or complementary bitterness. Assess for hotness vs. warmth. Look for oxidative notes (should be minimal in young mead, developed in aged). Check mouthfeel - should be viscous but not cloying. Evaluate finish complexity - cheap sweet mead = simple sugar finish, quality = layers of honey, fruit, caramel, spice.",
            "BEGINNER TIPS: This is very sweet. Like Sauternes, Tokaji, or Ice Wine. Tiny servings. Expensive to make well (lots of honey). Save for special occasions. Not for daily drinking.",
            "PRODUCTION NOTES: Requires high-quality honey - cheap honey just tastes like sugar syrup at this level. Consider oak aging (French oak adds vanilla/structure). Acid additions almost always needed (2-4 g/L tartaric).",
        ]);

        sweetness_entry(ui, "Dessert / Sack", "1.040-1.060+", "Liquid Gold Elixir", &[
            "TASTE PROFILE: Extremely sweet. Liqueur-level sweetness. Intense, concentrated honey. This is sipping mead - like fine Cognac or aged Port. Overwhelming sweetness must be balanced by equally intense acidity, tannins, or both. Complex notes: dried fruit, toffee, caramel, oxidative sherry notes.",
            "MOUTHFEEL: Very full, syrupy, viscous. Coats mouth and throat. Almost sticky. Heavy weight.",
            "ALCOHOL PERCEPTION: ABV 15-20%+. Hot if not aged properly. Needs years to integrate.",
            "FOOD PAIRING: Very dark chocolate (85%+), blue cheese (intense Roquefort), nuts, digestif. Usually served INSTEAD of dessert. Cigar pairing. Post-prandial sipper.",
            "SERVING: Room temperature (65-70°F) in snifters or Port glasses. 1 oz pours. Sip slowly. Contemplate.",
            "YEAST: High alcohol tolerance yeast stopped at 1.040+ OR fortified. EC-1118, K1-V1116, or champagne yeasts arrested. Some traditional sack meads are fortified with brandy.",
            "AGING: 3-10 years MINIMUM. These are decade meads. Can age 20-50+ years. Legendary meads are in this category.",
            "PROFESSIONAL NOTES: This is advanced meadmaking. Evaluate: (1) Balance - must have intense acidity or tannin structure, (2) Complexity - should show layers upon layers, (3) Integration - sweetness/alcohol/acid must be harmonious, (4) Finish - should linger minutes, (5) Evolution - how does it change in glass over 30 minutes? Must show development. Look for oxidative notes (desirable here) - walnut, dried fruit, caramel. Assess viscosity - should form thick legs. Check for crystallization (can happen with high honey content).",
            "BEGINNER TIPS: This is NOT for beginners. Very advanced, very expensive, very sweet. Think 50-year-old Tawny Port, Tokaji Eszencia, or ancient Madeira. Small sips. Meditative drinking. Ages forever. Investment-worthy bottles.",
            "PRODUCTION NOTES: Requires premium honey (often 3-5 lbs per gallon). Long fermentation (6-12 months). Extended aging (5-10 years minimum). Often oak-aged in small barrels. Some producers fortify with brandy. High acid additions (3-5 g/L). Tannin additions (FT Rouge, oak). Production cost $30-100+ per bottle.",
            "HISTORICAL CONTEXT: Medieval 'Sack Mead' was this style - strong, sweet, prestigious. Vikings' ceremonial meads. Polish 'Półtorak' tradition. Revered for special occasions, royal tables, ceremonial use.",
        ]);

        ui.add_space(30.0);
        ui.separator();
        ui.add_space(20.0);

        // ========== WINE (GRAPE) ==========
        section_header(ui, "🍷 WINE (Grape-Based)", "Classic wine sweetness levels - universal reference points");

        sweetness_entry(ui, "Bone Dry Wine", "0.990-0.995", "Classic Dry Table Wine", &[
            "RESIDUAL SUGAR: <4 g/L. Imperceptible sweetness.",
            "TASTE: Crisp, clean, dry finish. No sugar detected. Acid and tannin define structure.",
            "EXAMPLES: Sauvignon Blanc (Sancerre), Pinot Grigio, Chablis, dry Champagne (Brut Nature)",
            "ALCOHOL: 11-13.5% typical",
            "FOOD: Oysters, sushi, salads, shellfish, fresh cheeses",
            "PROFESSIONAL CONTEXT: VDP.Grosse Lage Riesling, Muscadet sur lie, Fino sherry, dry rosé",
        ]);

        sweetness_entry(ui, "Dry Wine", "0.995-1.000", "Standard Table Wine", &[
            "RESIDUAL SUGAR: 4-9 g/L. Just perceptible sweetness on finish.",
            "TASTE: 'Dry' to most palates but slight roundness. Fruit-forward without sugar perception.",
            "EXAMPLES: Chardonnay, Pinot Noir, Cabernet Sauvignon, most red wines",
            "ALCOHOL: 12-14.5% typical",
            "FOOD: Extremely versatile - poultry, red meat, pasta, hard cheeses",
            "PROFESSIONAL CONTEXT: This is most table wine. Bourgogne, Bordeaux, Napa Valley reds, most Italian wines",
        ]);

        sweetness_entry(ui, "Off-Dry Wine", "1.000-1.010", "Hint of Sweetness", &[
            "RESIDUAL SUGAR: 10-30 g/L. Noticeable but balanced sweetness.",
            "TASTE: Perceptible sweetness balanced by acidity. Fruit character prominent. Smooth, round mouthfeel.",
            "EXAMPLES: Riesling (Kabinett, many German styles), Gewürztraminer, Chenin Blanc, Prosecco, Moscato d'Asti",
            "ALCOHOL: 9-12% typical",
            "FOOD: Asian cuisine (Thai, Chinese), spicy dishes, pork, shellfish, mild cheeses",
            "PROFESSIONAL CONTEXT: German Prädikatswein (Kabinett/Spätlese), Vouvray demi-sec, many crowd-pleasing whites",
            "MARKET NOTES: Very popular style - balances 'wine drinkers' and 'sweet wine lovers'",
        ]);

        sweetness_entry(ui, "Medium-Sweet Wine", "1.010-1.025", "Clearly Sweet", &[
            "RESIDUAL SUGAR: 30-50 g/L. Obvious sweetness.",
            "TASTE: Sweet but not dessert-level. Fruit is lush, ripe. Needs good acidity to balance.",
            "EXAMPLES: Late Harvest Riesling, Moscato, some Lambrusco, rosé d'Anjou",
            "ALCOHOL: 8-11% typical (lower) OR 13-15% (late harvest)",
            "FOOD: Fruit desserts, mild blue cheese, foie gras, Asian desserts",
            "PROFESSIONAL CONTEXT: Spätlese/Auslese level sweetness, vendange tardive",
        ]);

        sweetness_entry(ui, "Sweet / Dessert Wine", "1.025-1.050", "Dessert Territory", &[
            "RESIDUAL SUGAR: 50-150 g/L. Very sweet.",
            "TASTE: Intensely sweet. Concentrated fruit, honey, caramel notes. Thick, viscous.",
            "EXAMPLES: Sauternes, Tokaji Aszú (3-5 puttonyos), Ice Wine, Vin Santo, PX Sherry",
            "ALCOHOL: 10-13% (botrytis) OR 14-16% (late harvest)",
            "FOOD: Blue cheese, foie gras, nut desserts, serve as dessert",
            "SERVING: Small portions (2-3 oz), chilled to cold",
            "AGING: Decades. These are cellar-worthy.",
            "PROFESSIONAL CONTEXT: Noble rot (Botrytis cinerea) concentration, frozen grape concentration, raisin concentration",
        ]);

        sweetness_entry(ui, "Very Sweet / Liqueur Wine", "1.050-1.100+", "Liquid Gold", &[
            "RESIDUAL SUGAR: 150-450+ g/L. Extraordinarily sweet.",
            "TASTE: Syrupy, concentrated, intense. Oxidative notes, dried fruit, caramel, toffee.",
            "EXAMPLES: Tokaji Eszencia, PX Sherry, Commandaria, ancient Madeira, Recioto",
            "ALCOHOL: Variable - 9-11% (Tokaji) OR 15-20% (fortified)",
            "SERVING: Tiny portions (1 oz), room temperature, contemplate",
            "AGING: 20-100+ years. Legendary bottles.",
            "PRODUCTION: Extreme concentration methods - botrytis, raisining, fortification",
            "PROFESSIONAL CONTEXT: World's most expensive wines often here - Château d'Yquem, Tokaji Eszencia, ancient Madeira",
        ]);

        ui.add_space(30.0);
        ui.separator();
        ui.add_space(20.0);

        // ========== BEER & CIDER ==========
        section_header(ui, "🍺 BEER & CIDER", "Malt and apple-based fermentations");

        ui.label(RichText::new("BEER SWEETNESS LEVELS").size(16.0).strong().color(colors::SADDLE_BROWN));
        ui.add_space(10.0);

        sweetness_entry(ui, "Dry Beer", "1.008-1.012", "Clean & Crisp", &[
            "TASTE: Dry, clean finish. No residual sweetness. Malt provides flavor but not sweetness.",
            "EXAMPLES: Pilsner, lager, IPA, pale ale, most session beers",
            "MOUTHFEEL: Light, crisp, refreshing. High carbonation enhances dryness.",
            "ABV: 4-7% typical",
            "FOOD: Versatile - pizza, burgers, fried foods, pub fare",
        ]);

        sweetness_entry(ui, "Medium Beer", "1.012-1.018", "Balanced & Malty", &[
            "TASTE: Slight malt sweetness. Balanced between sweet and dry. Smooth, round.",
            "EXAMPLES: Brown ale, Scottish ale, some stouts, Vienna lager, Märzen",
            "MOUTHFEEL: Medium body. Slight sweetness balances hop bitterness.",
            "ABV: 5-7%",
            "FOOD: Roasted meats, stews, aged cheddar, grilled foods",
        ]);

        sweetness_entry(ui, "Sweet Beer", "1.018-1.030+", "Dessert & Specialty", &[
            "TASTE: Noticeable to pronounced sweetness. Malty, full, rich.",
            "EXAMPLES: Milk stout (lactose), sweet stout, imperial stout, barleywine, pastry stouts",
            "MOUTHFEEL: Full-bodied, coating, smooth. Lower carbonation.",
            "ABV: 7-14%",
            "FOOD: Desserts, chocolate, coffee, vanilla ice cream, strong cheese",
            "PRODUCTION: Lactose (unfermentable), high mash temps (more dextrins), specialty malts",
        ]);

        ui.add_space(15.0);
        ui.label(RichText::new("CIDER SWEETNESS LEVELS").size(16.0).strong().color(colors::SADDLE_BROWN));
        ui.add_space(10.0);

        sweetness_entry(ui, "Dry Cider", "0.995-1.005", "Crisp & Tart", &[
            "TASTE: Bone dry to dry. Tart apple character. No residual sweetness. Wine-like.",
            "EXAMPLES: Traditional English cider, French cidre brut, farmhouse cider",
            "APPLES: Bittersweet and bittersharps add tannin structure",
            "ABV: 6-8.5%",
            "FOOD: Pork, chicken, aged cheese, savory dishes",
            "PROFESSIONAL: This is traditional cider. High tannin, high acid, bone dry.",
        ]);

        sweetness_entry(ui, "Medium Cider", "1.005-1.015", "Balanced Apple", &[
            "TASTE: Noticeable apple sweetness. Balanced acid. Recognizable as 'cider' to most.",
            "EXAMPLES: Most commercial American ciders, Strongbow, Angry Orchard",
            "ABV: 4.5-6.5%",
            "FOOD: Very versatile - BBQ, casual dining, picnics",
            "MARKET: Most popular commercial level - balances traditional and sweet preferences",
        ]);

        sweetness_entry(ui, "Sweet Cider", "1.015-1.025+", "Apple Juice-Like", &[
            "TASTE: Clearly sweet. Like alcoholic apple juice. Minimal tannin.",
            "EXAMPLES: Woodchuck, Redd's Apple Ale, sweet ciders",
            "ABV: 4-5%",
            "SERVING: Cold. Entry-level for beer drinkers.",
            "PRODUCTION: Backsweetened with juice, arrested fermentation, or unfermentable sweeteners",
        ]);

        ui.add_space(30.0);
        ui.separator();
        ui.add_space(20.0);

        // ========== SAKE ==========
        section_header(ui, "🌸 SAKE (Rice Wine)", "Japanese rice wine sweetness terminology");

        sweetness_entry(ui, "Karakuchi (Dry)", "1.000-1.005", "Crisp Sake", &[
            "SAKE METER VALUE (SMV): +3 to +10 (positive = dry)",
            "TASTE: Clean, dry, crisp. Prominent rice umami. Refreshing finish.",
            "EXAMPLES: Most junmai, honjozo, many daiginjo",
            "SERVING: Chilled to room temp depending on grade",
            "FOOD: Sushi, sashimi, grilled fish, Japanese cuisine",
        ]);

        sweetness_entry(ui, "Futsu (Medium)", "1.005-1.015", "Balanced Sake", &[
            "SMV: -1.5 to +3 (around neutral)",
            "TASTE: Balanced sweetness. Smooth, approachable. Rice character clear.",
            "EXAMPLES: Many ginjo, some junmai",
            "SERVING: Versatile temperature range",
            "FOOD: Wide range - yakitori, tempura, teriyaki",
        ]);

        sweetness_entry(ui, "Amakuchi (Sweet)", "1.015-1.030", "Sweet Sake", &[
            "SMV: -6 to -1.5 (negative = sweet)",
            "TASTE: Noticeably sweet. Rich rice character. Almost creamy.",
            "EXAMPLES: Some nigori (cloudy sake), dessert sake",
            "SERVING: Well-chilled",
            "FOOD: Spicy foods, desserts, fruit",
        ]);

        sweetness_entry(ui, "Kijoshu", "1.030-1.050+", "Noble Sweet Sake", &[
            "PRODUCTION: Made with sake instead of water - extremely concentrated",
            "TASTE: Very sweet, rich, thick. Honey-like. Complex umami.",
            "AGING: Often aged years to decades",
            "SERVING: Small portions, room temperature, digestif",
            "COMPARISON: Like Sauternes or sherry - meditation beverage",
        ]);

        ui.add_space(30.0);
        ui.separator();
        ui.add_space(20.0);

        // ========== PROFESSIONAL GUIDANCE ==========
        ui.label(RichText::new("🎓 PROFESSIONAL GUIDANCE").size(18.0).strong().color(colors::SADDLE_BROWN));
        ui.add_space(15.0);

        ui.label(RichText::new("📊 SWEETNESS PERCEPTION FACTORS:").size(14.0).strong());
        ui.label("• ALCOHOL: Higher ABV masks sweetness - 15% wine at 1.020 tastes drier than 10% wine at 1.015");
        ui.label("• ACIDITY: High acid cuts sweetness - pH 3.0 wine at 1.020 tastes like pH 3.5 wine at 1.010");
        ui.label("• TANNIN: Astringency counterpoints sweetness - makes sweet beverages less cloying");
        ui.label("• TEMPERATURE: Warmer = sweeter, colder = drier perception");
        ui.label("• CARBONATION: CO₂ creates carbonic acid, tastes drier and more refreshing");
        ui.label("• BITTERNESS: Hops, cacao, tannins provide balance");

        ui.add_space(15.0);

        ui.label(RichText::new("🎯 ACHIEVING TARGET SWEETNESS:").size(14.0).strong());
        ui.label("OPTION 1: Stabilization + Backsweetening");
        ui.label("  • Ferment to dry, stabilize (K-meta + sorbate), wait, backsweeten gradually");
        ui.label("OPTION 2: Pasteurization");
        ui.label("  • Ferment to target, bottle, heat to 65°C for 30 min OR 72°C for 10 min");
        ui.label("OPTION 3: Controlled Fermentation");
        ui.label("  • Low-attenuating yeast, strict temperature control, cold crash at target");

        ui.add_space(15.0);

        ui.label(RichText::new("⚠️ COMMON MISTAKES:").size(14.0).strong().color(colors::DARK_ORANGE));
        ui.label("1. Under-acidifying → Cloying, flat taste. Add acid blend for pH 3.2-3.6");
        ui.label("2. No stabilization → Bottle bombs. ALWAYS stabilize or pasteurize first");
        ui.label("3. Too much at once → Add sweetener gradually, taste between");
        ui.label("4. Serving too cold → Sweet meads taste flat. Serve 55-65°F");
        ui.label("5. Cheap honey in sweet mead → Use premium honey for sweet styles");
        ui.label("6. No aging → Sweet meads need 12-24 months minimum");

        ui.add_space(15.0);

        ui.label(RichText::new("🍇 YEAST SELECTION BY TARGET:").size(14.0).strong());
        ui.label("BONE DRY (0.990-0.996): EC-1118, DV10, Premier Cuvée");
        ui.label("DRY (0.996-1.006): 71B-1122, D47, QA23");
        ui.label("SEMI-SWEET (1.010-1.020): Cold crash + stabilization");
        ui.label("SWEET (1.020+): Ferment dry then backsweeten (most reliable)");

        ui.add_space(30.0);

        ui.label(RichText::new("💡 FINAL TIPS").size(16.0).strong().color(colors::SADDLE_BROWN));
        ui.label("1. Trust your palate - numbers are guidelines, taste is truth");
        ui.label("2. Age sweet beverages - they need time to integrate");
        ui.label("3. Balance is key - sweetness needs acidity/tannin");
        ui.label("4. Quality ingredients = quality results");
        ui.label("5. Take notes - document what works");
        ui.label("6. Experiment and have fun!");
    });
}

fn section_header(ui: &mut egui::Ui, title: &str, subtitle: &str) {
    egui::Frame::default()
        .fill(colors::HONEY_GOLD.linear_multiply(0.3))
        .stroke(Stroke::new(2.0, colors::SADDLE_BROWN))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.label(RichText::new(title).size(18.0).strong().color(colors::SADDLE_BROWN));
            ui.label(RichText::new(subtitle).size(12.0).color(colors::DARK_TEXT));
        });
    ui.add_space(15.0);
}

fn sweetness_entry(ui: &mut egui::Ui, level: &str, gravity: &str, character: &str, details: &[&str]) {
    egui::Frame::default()
        .fill(Color32::WHITE)
        .stroke(Stroke::new(1.5, colors::HONEY_GOLD))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(level).size(15.0).strong().color(colors::SADDLE_BROWN));
                ui.label(RichText::new("|").color(colors::HONEY_GOLD));
                ui.label(RichText::new(gravity).size(14.0).strong().color(colors::HONEY_GOLD));
                ui.label(RichText::new("|").color(colors::HONEY_GOLD));
                ui.label(RichText::new(character).size(13.0).color(colors::FOREST_GREEN));
            });

            ui.add_space(8.0);

            for detail in details {
                ui.label(RichText::new(*detail).size(12.0));
                ui.add_space(4.0);
            }
        });
    ui.add_space(10.0);
}

// === CALCULATION FUNCTIONS ===

fn calc_backsweetening(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("backsweetening") {
        Some(c) => c,
        None => {
            app.result = Some("❌ Calculator not found".to_string());
            return;
        }
    };

    let is_metric = matches!(app.state.unit_system, crate::state::UnitSystem::Metric);

    let current_sg_val = match Decimal::from_str(&app.current_sg) {
        Ok(v) => v,
        _ => {
            app.result = Some("❌ Invalid current SG".to_string());
            return;
        }
    };

    let sg_meas = match Measurement::sg(current_sg_val) {
        Ok(m) => m,
        Err(e) => {
            app.result = Some(format!("❌ {}", e));
            return;
        }
    };

    let volume_val = match Decimal::from_str(&app.sweet_vol) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid volume".to_string());
            return;
        }
    };

    let volume_liters = if is_metric {
        volume_val
    } else {
        volume_val * Decimal::new(378541, 5)
    };

    let input = CalcInput::new()
        .add_measurement(sg_meas)
        .add_param("volume", &volume_liters.to_string())
        .add_param("target_sg", &app.target_sg)
        .add_param("sweetener", &app.sweetener);

    match calc.calculate(input) {
        Ok(res) => {
            let (amount, weight_unit) = if is_metric {
                let g = res.output.value;
                if g >= Decimal::from(1000) {
                    (g / Decimal::from(1000), "kg")
                } else {
                    (g, "g")
                }
            } else {
                let oz = res.output.value / Decimal::new(2835, 2);
                if oz >= Decimal::from(16) {
                    (oz / Decimal::from(16), "lb")
                } else {
                    (oz, "oz")
                }
            };

            app.result = Some(format!("Add {:.1} {} of {}", amount, weight_unit, app.sweetener.replace('_', " ")));
            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => {
            app.result = Some(format!("❌ {}", e));
            app.warnings.clear();
            app.metadata.clear();
        }
    }
}

fn calc_stabilization(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("stabilization") {
        Some(c) => c,
        None => {
            app.result = Some("❌ Calculator not found".to_string());
            return;
        }
    };

    let is_metric = matches!(app.state.unit_system, crate::state::UnitSystem::Metric);

    let volume_val = match Decimal::from_str(&app.sulfite_vol) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid volume".to_string());
            return;
        }
    };

    let volume_liters = if is_metric {
        volume_val
    } else {
        volume_val * Decimal::new(378541, 5)
    };

    let ph_val = match Decimal::from_str(&app.ph) {
        Ok(v) => v,
        _ => Decimal::new(35, 1),
    };

    let ph_meas = match Measurement::ph(ph_val) {
        Ok(m) => m,
        _ => {
            app.result = Some("❌ Invalid pH".to_string());
            return;
        }
    };

    let input = CalcInput::new()
        .add_measurement(ph_meas)
        .add_param("volume", &volume_liters.to_string());

    match calc.calculate(input) {
        Ok(res) => {
            let kmeta_g = res.metadata.iter()
                .find(|(k, _)| k == "kmeta_g")
                .and_then(|(_, v)| v.split_whitespace().next())
                .and_then(|s| Decimal::from_str(s).ok())
                .unwrap_or(Decimal::ZERO);

            let sorbate_g = res.metadata.iter()
                .find(|(k, _)| k == "sorbate_g")
                .and_then(|(_, v)| v.split_whitespace().next())
                .and_then(|s| Decimal::from_str(s).ok())
                .unwrap_or(Decimal::ZERO);

            let (kmeta_display, sorbate_display) = if is_metric {
                (format!("{:.1}g", kmeta_g), format!("{:.1}g", sorbate_g))
            } else {
                let kmeta_oz = kmeta_g / Decimal::new(2835, 2);
                let sorbate_oz = sorbate_g / Decimal::new(2835, 2);
                (format!("{:.2}oz", kmeta_oz), format!("{:.2}oz", sorbate_oz))
            };

            app.result = Some(format!("K-meta: {} | K-sorbate: {}", kmeta_display, sorbate_display));
            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => {
            app.result = Some(format!("❌ {}", e));
            app.warnings.clear();
            app.metadata.clear();
        }
    }
}

fn calc_sulfite(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("sulfite") {
        Some(c) => c,
        None => {
            app.result = Some("❌ Calculator not found".to_string());
            return;
        }
    };

    let is_metric = matches!(app.state.unit_system, crate::state::UnitSystem::Metric);

    let ph_val = match Decimal::from_str(&app.ph) {
        Ok(v) => v,
        _ => {
            app.result = Some("❌ Invalid pH".to_string());
            return;
        }
    };

    let ph_meas = match Measurement::ph(ph_val) {
        Ok(m) => m,
        Err(e) => {
            app.result = Some(format!("❌ {}", e));
            return;
        }
    };

    let volume_val = match Decimal::from_str(&app.sulfite_vol) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid volume".to_string());
            return;
        }
    };

    let volume_liters = if is_metric {
        volume_val
    } else {
        volume_val * Decimal::new(378541, 5)
    };

    let input = CalcInput::new()
        .add_measurement(ph_meas)
        .add_param("volume", &volume_liters.to_string())
        .add_param("target_free_so2", &app.target_so2);

    match calc.calculate(input) {
        Ok(res) => {
            let (amount, weight_unit) = if is_metric {
                (res.output.value, "g")
            } else {
                let oz = res.output.value / Decimal::new(2835, 2);
                (oz, "oz")
            };

            app.result = Some(format!("K-meta: {:.2} {}", amount, weight_unit));
            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => {
            app.result = Some(format!("❌ {}", e));
            app.warnings.clear();
            app.metadata.clear();
        }
    }
}

fn calc_acid_addition(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("acid_addition") {
        Some(c) => c,
        None => {
            app.result = Some("❌ Calculator not found".to_string());
            return;
        }
    };

    let is_metric = matches!(app.state.unit_system, crate::state::UnitSystem::Metric);

    let current_ph_val = match Decimal::from_str(&app.current_ph) {
        Ok(v) => v,
        _ => {
            app.result = Some("❌ Invalid current pH".to_string());
            return;
        }
    };

    let current_ph_meas = match Measurement::ph(current_ph_val) {
        Ok(m) => m,
        Err(e) => {
            app.result = Some(format!("❌ {}", e));
            return;
        }
    };

    let volume_val = match Decimal::from_str(&app.acid_vol) {
        Ok(v) if v > Decimal::ZERO => v,
        _ => {
            app.result = Some("❌ Invalid volume".to_string());
            return;
        }
    };

    let volume_liters = if is_metric {
        volume_val
    } else {
        volume_val * Decimal::new(378541, 5)
    };

    let input = CalcInput::new()
        .add_measurement(current_ph_meas)
        .add_param("volume", &volume_liters.to_string())
        .add_param("target_ph", &app.target_ph_acid)
        .add_param("acid_type", &app.acid_type);

    match calc.calculate(input) {
        Ok(res) => {
            let (amount, weight_unit) = if is_metric {
                (res.output.value, "g")
            } else {
                let oz = res.output.value / Decimal::new(2835, 2);
                (oz, "oz")
            };

            app.result = Some(format!("{}: {:.2} {}", app.acid_type.replace('_', " "), amount, weight_unit));
            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => {
            app.result = Some(format!("❌ {}", e));
            app.warnings.clear();
            app.metadata.clear();
        }
    }
}

fn calc_pasteurization(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("pasteurization") {
        Some(c) => c,
        None => {
            app.result = Some("❌ Calculator not found".to_string());
            return;
        }
    };

    let is_metric = matches!(app.state.unit_system, crate::state::UnitSystem::Metric);

    let temp_val = match Decimal::from_str(&app.pasteurization_temp) {
        Ok(v) => v,
        _ => {
            app.result = Some("❌ Invalid temperature".to_string());
            return;
        }
    };

    let temp_c = if is_metric {
        temp_val
    } else {
        (temp_val - Decimal::from(32)) * Decimal::new(5, 0) / Decimal::new(9, 0)
    };

    let temp_meas = Measurement::new(temp_c, mazerion_core::Unit::Celsius);

    let input = CalcInput::new().add_measurement(temp_meas);

    match calc.calculate(input) {
        Ok(res) => {
            let time_str = res.metadata.iter()
                .find(|(k, _)| k == "calculated_time_min")
                .map(|(_, v)| v.as_str())
                .unwrap_or("30");

            app.result = Some(format!("Hold at {} for {} minutes",
                                      if is_metric {
                                          format!("{:.1}°C", temp_c)
                                      } else {
                                          format!("{:.1}°F", temp_val)
                                      },
                                      time_str
            ));
            app.warnings = res.warnings;
            app.metadata = res.metadata;
        }
        Err(e) => {
            app.result = Some(format!("❌ {}", e));
            app.warnings.clear();
            app.metadata.clear();
        }
    }
}