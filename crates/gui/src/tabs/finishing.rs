//! Finishing tab - COMPREHENSIVE SWEETNESS GUIDE - ALL 12 TYPES

use crate::{MazerionApp, state::{colors, FinishingCalculator}};
use eframe::egui::{self, RichText, Color32, Stroke, CornerRadius};
use mazerion_core::{CalcInput, Measurement};
use rust_decimal::Decimal;
use std::str::FromStr;

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("✨ Finishing").color(colors::SADDLE_BROWN).size(24.0));
    ui.add_space(10.0);

    ui.horizontal(|ui| {
        for calc in FinishingCalculator::all() {
            if ui.selectable_label(
                app.state.finishing_calc == calc,
                calc.name()
            ).clicked() {
                app.state.finishing_calc = calc;
            }
        }
    });

    ui.add_space(15.0);
    ui.separator();
    ui.add_space(15.0);

    // Wrap everything in proper background frame LIKE UTILITIES TAB DOES
    egui::Frame::new()
        .fill(colors::LIGHT_CREAM)
        .stroke(Stroke::new(1.5, colors::HONEY_GOLD))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            match app.state.finishing_calc {
                FinishingCalculator::Backsweetening => render_backsweetening(app, ui),
                FinishingCalculator::Sulfite => render_sulfite(app, ui),
                FinishingCalculator::AcidAddition => render_acid(app, ui),
                FinishingCalculator::SweetnessChart => render_sweetness_guide(ui),
            }
        });
}

fn render_sweetness_guide(ui: &mut egui::Ui) {
    ui.heading(RichText::new("📊 COMPREHENSIVE SWEETNESS GUIDE").color(colors::SADDLE_BROWN).size(18.0));
    ui.label(RichText::new("Final Gravity Reference for All Fermentable Beverages").color(colors::DARK_TEXT));
    ui.add_space(15.0);

    // NO ScrollArea here - lib.rs already has one at parent level
    // TRADITIONAL MEAD
    section_header(ui, "🍯 TRADITIONAL MEAD (Honey-Only)", "Pure honey fermentation showcasing terroir", colors::HONEY_GOLD);
    sweetness_grid(ui, &[
        ("Bone Dry", "0.990-0.996", "Crisp & Refreshing", "Clean, bright acidity with no residual sweetness. Honey character is subtle. Bone-dry finish. Wine-like with pronounced acidity. Pairs with oysters, sushi, fresh salads. Serve chilled 45-50°F."),
        ("Dry", "0.996-1.006", "Elegant & Balanced", "Light honey sweetness on finish with dominant floral notes. Still quite wine-like. You taste the honey variety more than sugar. Clean, refreshing. Pairs with grilled fish, chicken, soft cheeses. Serve 50-55°F."),
        ("Semi-Dry", "1.006-1.015", "Versatile & Food-Friendly", "Noticeable honey sweetness balanced by acidity. Rounded mouthfeel, floral honey forward. Not dessert-sweet but clearly honeyed. Medium body, smooth. Pairs with pork, duck, aged cheddar, fruit desserts. Serve 55-60°F."),
        ("Semi-Sweet", "1.015-1.025", "Rich & Honey-Forward", "Clear sweetness with viscous mouthfeel. Honey character dominates. Still has acidity to avoid cloying. Rich, satisfying, full-bodied. Pairs with blue cheese, nuts, dried fruits, milk chocolate. Serve 55-60°F."),
        ("Sweet", "1.025-1.040", "Dessert-Style", "Dessert-level sweetness with thick, coating mouthfeel. Pure honey flavor like honeycomb. Needs high ABV (14%+) to balance. Luscious, decadent. Pairs with foie gras, crème brûlée, dark chocolate. Serve 55-60°F."),
        ("Very Sweet", "1.040-1.060+", "Extremely Rich Sack", "Intensely sweet, liqueur-like. Very thick, syrupy, coating. Pure concentrated honey. Requires 15%+ ABV. Sip slowly. After-dinner sipper, dessert replacement. Small pours, 60-65°F."),
    ]);

    ui.add_space(20.0);

    // MELOMEL
    section_header(ui, "🍓 MELOMEL (Fruit Mead)", "Honey and fruit - balance both components", colors::DARK_RED);
    sweetness_grid(ui, &[
        ("Bone Dry", "0.990-0.996", "Tart & Bright", "Fruit acidity dominates with zero sweetness. Sharp, refreshing, clean. Fruit is acidic/tart, not sweet. Honey provides body. Very wine-like. Pairs with shellfish, goat cheese. Best with tart fruits like cranberry, cherry."),
        ("Dry", "0.996-1.006", "Fruit-Forward", "Light fruit sweetness with pronounced fruit character. Acidity prominent. Fresh, bright, fruit-driven. Like dry fruit wine. Pairs with salmon, turkey, brie. Works with most fruits - berry, stone fruit, tropical."),
        ("Semi-Dry", "1.006-1.015", "Balanced Honey-Fruit", "Noticeable sweetness from both. Round, smooth, harmonious. Neither dominates - perfect balance. Medium body, very drinkable. Universal crowd-pleaser. Pairs with BBQ, glazed ham, cheddar, fruit tarts."),
        ("Semi-Sweet", "1.015-1.025", "Fruit Dessert", "Clear sweetness with fruit shining through honey base. Like ripe fruit with honey drizzle. Fuller body, coating. Fruit and honey meld. Pairs with panna cotta, cheesecake, vanilla ice cream. Best with sweet fruits like peach, mango."),
        ("Sweet", "1.025-1.040", "Fruit Syrup", "Very sweet - like fruit preserves. Thick, luscious, intense. Fruit remains but sugar dominates. Needs high ABV or strong acidity. Pairs with chocolate torte, pound cake, blue cheese. Small servings only."),
        ("Very Sweet", "1.040-1.060+", "Fruit Liqueur", "Extremely sweet, cordial-like. Thick, syrupy, intense fruit-honey fusion. Sipping beverage. Can cloy without careful acid/tannin balance. Rare and special. After-dinner digestif. Tiny portions, savor slowly."),
    ]);

    ui.add_space(20.0);

    // CYSER
    section_header(ui, "🍎 CYSER (Apple-Honey)", "Apple juice and honey - best of both worlds", colors::FOREST_GREEN);
    sweetness_grid(ui, &[
        ("Bone Dry", "0.990-0.998", "Crisp Apple-Wine", "Tart, crisp, refreshing like dry cider with honey aromatics. Apple acidity dominates. Honey adds floral notes and body but no sweetness. Think dry champagne with apple notes. Pairs with oysters, pork chops, arugula. Excellent aperitif."),
        ("Dry", "0.998-1.008", "Classic Dry Cyser", "Light apple sweetness with honey undertones. Still quite dry with good acidity. Apple and honey play together. Smooth, balanced, sophisticated. Like premium dry hard cider. Pairs with roast chicken, grilled fish, gouda. Perfect with apple-smoked meats."),
        ("Semi-Dry", "1.008-1.018", "Traditional Cyser", "Noticeable sweetness, apple-honey balance perfect. Medium body, round mouthfeel. You taste both clearly. Not too sweet, not too dry - goldilocks zone. Most popular style. Pairs with pork tenderloin, cheddar, apple pie. Thanksgiving turkey traditional pairing."),
        ("Semi-Sweet", "1.018-1.028", "Sweet Cyser", "Clear sweetness, like fresh apple cider with honey. Fuller body, coating texture. Apple and honey sweetness reinforce each other. Rich, satisfying. Pairs with baked brie, caramel desserts, spice cake. Fall/winter seasonal favorite."),
        ("Sweet", "1.028-1.040", "Dessert Cyser", "Very sweet - like apple butter with honey. Thick, luscious, full-bodied. Needs strong apple character and acidity. Almost like drinking honeyed applesauce. Pairs with apple crisp, vanilla ice cream, cinnamon rolls. Holiday dessert beverage."),
    ]);

    ui.add_space(20.0);

    // PYMENT
    section_header(ui, "🍇 PYMENT (Grape-Honey)", "Grape wine meets mead - ancient and noble", Color32::from_rgb(102, 51, 102));
    sweetness_grid(ui, &[
        ("Bone Dry", "0.990-0.996", "Dry Wine", "Like bone-dry white wine with honey aromatics. Crisp, acidic, no residual sugar. Grape tannins provide structure, honey adds complexity. Very wine-like with mead elegance. Pairs with seafood, fresh cheeses, Mediterranean. Serve chilled."),
        ("Dry", "0.996-1.006", "Table Wine Style", "Light sweetness, wine-forward with honey nuance. Good acidity, structured tannins. Grape dominates but honey rounds edges. Sophisticated, food-friendly. Pairs with grilled meats, pasta, aged cheeses. Most versatile pyment style."),
        ("Semi-Dry", "1.006-1.016", "Port-Like Richness", "Noticeable sweetness with grape-honey fusion. Medium-full body, smooth. More honey-forward. Rich, warming, satisfying. Like light port or sweet red. Pairs with lamb, game meats, strong cheeses. Excellent with chocolate."),
        ("Semi-Sweet", "1.016-1.025", "Dessert Wine", "Clear sweetness, grape-honey blend apparent. Full body, viscous. Both components shine - grape tannins balance honey sweetness. Rich, complex, layered. Pairs with fruit tarts, nut desserts, blue cheese. After-dinner sipper."),
        ("Sweet", "1.025-1.040+", "Noble Rot Style", "Very sweet, like Sauternes or late harvest with honey. Thick, luscious, intensely sweet. Grape complexity prevents cloying. Rare, special-occasion. Pairs with foie gras, crème brûlée, aged stilton. Tiny portions, savor slowly."),
    ]);

    ui.add_space(20.0);

    // BRAGGOT
    section_header(ui, "🍺🍯 BRAGGOT (Mead-Beer)", "Beer meets mead - honey with malt backbone", colors::SADDLE_BROWN);
    sweetness_grid(ui, &[
        ("Dry", "1.008-1.014", "Session Braggot", "Light honey sweetness with beer-like dryness. Malt and hops balance honey. Carbonated, refreshing, very drinkable. Honey adds complexity without sweetness. Think amber ale with honey notes. Pairs with burgers, pizza, wings. Excellent session beer alternative."),
        ("Semi-Dry", "1.014-1.020", "Traditional Braggot", "Balanced honey-malt sweetness. Medium body, smooth. Honey and malt play equally. Slightly sweet but still beer-like. Most authentic historical style. Pairs with roast meats, stews, hearty breads. Medieval feast beverage."),
        ("Semi-Sweet", "1.020-1.028", "Strong Braggot", "Noticeable honey sweetness with malt richness. Fuller body, warming. Honey-forward but malt backbone keeps grounded. Like barleywine with honey. Complex, sipping beverage. Pairs with aged cheddar, BBQ ribs, apple pie. Winter warmer style."),
        ("Sweet", "1.028-1.040+", "Dessert Braggot", "Very sweet - honey dominates with malt support. Thick, rich, dessert-like. Needs high ABV (10%+) to balance. Rare style - more mead than beer. Sip slowly. Pairs with dessert only - chocolate cake, bread pudding. Small servings."),
    ]);

    ui.add_space(20.0);

    // HYDROMEL
    section_header(ui, "💧🍯 HYDROMEL (Session Mead)", "Low ABV, light-bodied, highly drinkable mead", Color32::from_rgb(135, 206, 235));
    sweetness_grid(ui, &[
        ("Bone Dry", "0.994-1.000", "Ultra-Light", "Barely any sweetness, very light body. Honey flavor is delicate, aromatic. Refreshing, quenching, highly carbonated. Think hard seltzer with honey hints. Pairs with light salads, sushi, summer foods. Ultimate session drink."),
        ("Dry", "1.000-1.006", "Classic Session", "Light honey character, minimal sweetness. Low ABV (3-7%), very drinkable. Refreshing, light-bodied, can drink several. Perfect for hot weather, outdoor events. Pairs with grilled vegetables, white fish, fresh fruits. Day-drinking friendly."),
        ("Semi-Dry", "1.006-1.012", "Smooth Session", "Light sweetness, still very drinkable. Honey flavor more apparent. Smooth, easy-drinking, sessionable. Most popular hydromel sweetness level. Pairs with chicken, mild cheeses, sandwiches. Backyard BBQ favorite."),
        ("Semi-Sweet", "1.012-1.020", "Sweet Session", "Noticeable sweetness but still light-bodied. Honey-forward, pleasant, not cloying despite sweetness because of low ABV. Refreshing despite sugar. Pairs with fruit salads, light desserts, brunch foods. Crowd-pleaser."),
    ]);

    ui.add_space(20.0);

    // BEER
    section_header(ui, "🍺 BEER / ALE", "Malt-derived sweetness balanced by hops", colors::SADDLE_BROWN);
    sweetness_grid(ui, &[
        ("Very Dry", "1.006-1.010", "Crisp & Clean", "Very little residual sugar. Crisp, clean finish with pronounced hop bitterness. Light body, refreshing. Malt provides body but minimal sweetness. Finishes bone-dry. Examples: Light lagers, pilsners, dry-hopped IPAs, saisons. Perfect thirst quenchers."),
        ("Dry", "1.010-1.014", "Balanced", "Low residual sugar with good hop balance. Clean, crisp finish with slight malt sweetness. Medium-light body. Most common beer finish. Examples: Most ales, amber ales, English bitters. Universal appeal, highly drinkable."),
        ("Medium", "1.014-1.018", "Malty", "Noticeable malt sweetness balanced by hops. Medium body, smooth. Neither too sweet nor too dry. Malt flavor comes through without cloying. Examples: ESB, brown ales, malty IPAs. Food-friendly, versatile, satisfying."),
        ("Sweet", "1.018-1.024", "Rich & Malty", "Clear malt sweetness with fuller body. Hops take back seat to malt. Rich, smooth, coating texture. Sweet but not dessert-like. Sipping beer. Examples: Milk stouts, porters, oatmeal stouts, Scottish ales. Dessert-friendly."),
        ("Very Sweet", "1.024-1.030+", "Dessert", "Pronounced sweetness, very full body. Malt-forward with minimal hop balance. Thick, rich, almost syrupy. Needs high ABV or becomes cloying. Rare styles. Examples: Sweet stouts, barleywine, imperial stouts. Small pours, sip slowly."),
    ]);

    ui.add_space(20.0);

    // WINE
    section_header(ui, "🍷 WINE (Grape)", "Grape-derived sweetness with varietal character", colors::DARK_RED);
    sweetness_grid(ui, &[
        ("Bone Dry", "0.990-0.995", "Brut", "Zero perceptible sweetness. Crisp, acidic, clean. Grape tannins and acidity dominate completely. Refreshing, austere, food-friendly. Very wine-forward. Examples: Brut champagne, dry whites (Chablis, Albariño). Pairs with oysters, sushi."),
        ("Dry", "0.995-0.998", "Table Wine", "Minimal residual sugar, still quite dry. Light fruit sweetness balanced by acidity. Crisp finish, medium acidity. Most common wine style. Examples: Chardonnay, Cabernet, Pinot Noir. Universal food pairing wine."),
        ("Off-Dry", "0.998-1.005", "Hint of Sweet", "Noticeable fruit sweetness without being sweet. Round mouthfeel, smooth. Sugar balances high acidity. Approachable, crowd-pleasing. Examples: Riesling, Gewürztraminer, Moscato d'Asti. Pairs with spicy foods."),
        ("Medium-Sweet", "1.005-1.020", "Dessert Wine", "Clear sweetness with preserved acidity. Fuller body, viscous. Fruit character intense and concentrated. Sweet but not cloying due to acid. Examples: Late harvest Riesling, Sauternes, Vin Santo. With fruit desserts, foie gras."),
        ("Sweet", "1.020-1.040", "Noble Dessert", "Very sweet - concentrated grape sugars. Thick, luscious, full-bodied. High acidity required to balance. Complex, layered, intense. Special occasion wines. Examples: Tokaji, Eiswein, PX sherry. Tiny portions with dessert or cheese."),
        ("Very Sweet", "1.040+", "Ultra-Dessert", "Extremely sweet, syrupy texture. Intense concentrated grape character. Almost liqueur-like. Rare, expensive, age-worthy. Sip in small quantities. Examples: Ice wine, vintage port, PX sherry. After-dinner digestif, dessert replacement."),
    ]);

    ui.add_space(20.0);

    // CIDER
    section_header(ui, "🍎 CIDER", "Apple-derived sweetness - orchard to glass", colors::FOREST_GREEN);
    sweetness_grid(ui, &[
        ("Dry", "1.000-1.004", "Traditional English", "Zero residual sugar, tart apple acidity. Crisp, clean, refreshing. Pronounced apple tannins give structure. Slightly astringent finish. True farmhouse style. Pairs with pork, fish & chips, cheddar. Traditional pub cider - acquired taste."),
        ("Medium-Dry", "1.004-1.009", "Continental", "Light apple sweetness with good acidity. Balanced, approachable, still quite dry. Apple character shines through. Smooth, crisp, highly drinkable. Most versatile style. Pairs with roast chicken, salads, soft cheeses. Crowd-pleaser, universal appeal."),
        ("Medium", "1.009-1.015", "Commercial", "Noticeable sweetness, round mouthfeel. Clear apple flavor without tartness dominating. Smooth, easy-drinking, popular. Most common commercial cider sweetness. Pairs with BBQ, burgers, apple pie. Everyday drinking, very approachable."),
        ("Medium-Sweet", "1.015-1.020", "New World", "Clear sweetness, apple-forward. Fuller body, smooth texture. Sweet but not dessert-like. Like biting into a sweet apple. Popular in North America. Pairs with spicy foods, pork tenderloin, brie. Balances heat, friendly to sweet-tooths."),
        ("Sweet", "1.020-1.030+", "Dessert", "Very sweet - like apple juice with alcohol. Thick, full-bodied, intense apple flavor. Needs carbonation to avoid cloying. Dessert beverage or ice cider style. Pairs with apple desserts, cheddar, caramel. After-dinner drink, serve chilled."),
    ]);

    ui.add_space(20.0);

    // PERRY
    section_header(ui, "🍐 PERRY (Pear)", "Pear-based - delicate, aromatic, refined", Color32::from_rgb(210, 180, 140));
    sweetness_grid(ui, &[
        ("Dry", "1.000-1.006", "Traditional", "Zero sweetness, crisp pear acidity. Delicate, refined, complex. Pear tannins provide structure. Very dry finish. More complex than dry cider. Pairs with seafood, goat cheese, delicate salads. Traditional English style."),
        ("Medium-Dry", "1.006-1.012", "Balanced", "Light pear sweetness, still quite dry. Elegant, aromatic, sophisticated. Pear character is subtle, floral. Smooth, refined, beautiful. Most popular perry style. Pairs with chicken, fish, soft cheeses. Sophisticated beverage for wine drinkers."),
        ("Medium", "1.012-1.018", "Modern", "Noticeable sweetness, pear-forward. Round, smooth, approachable. Clear pear flavor without tartness. Fuller body than dry styles. Crowd-pleasing. Pairs with pork, Asian cuisine, fruit desserts. Accessible, very drinkable."),
        ("Medium-Sweet", "1.018-1.025", "Sweet Perry", "Clear sweetness like fresh pear juice. Aromatic, floral, intensely fruity. Fuller body, smooth texture. Sweet but maintains elegance. Dessert-friendly. Pairs with blue cheese, pear tart, vanilla desserts. Special occasion beverage."),
        ("Sweet", "1.025-1.035+", "Dessert", "Very sweet - concentrated pear flavor. Thick, aromatic, intensely fruity. Rare style - more common in ice perry. Sip slowly, small portions. Pairs with poached pears, gorgonzola, dessert only. After-dinner digestif."),
    ]);

    ui.add_space(20.0);

    // SAKE
    section_header(ui, "🍶 SAKE (Rice)", "Rice-based fermentation - umami-rich and nuanced", Color32::from_rgb(240, 240, 240));
    sweetness_grid(ui, &[
        ("Karakuchi Dry", "0.995-1.005", "Bone Dry", "Extremely dry, crisp, clean finish. High acidity, pronounced umami character. Very little residual sugar. Refreshing, light-bodied, food-friendly. Think extra-dry white wine. Pairs with sushi, sashimi, tempura, light seafood. Serve chilled 40-50°F."),
        ("Dry", "1.005-1.015", "Balanced", "Light sweetness with good acidity. Balanced umami, smooth texture. Rice character evident but not sweet. Clean, refined, elegant. Most versatile sake style. Pairs with grilled fish, yakitori, vegetable dishes. Room temp or lightly chilled."),
        ("Medium Amakuchi", "1.015-1.025", "Semi-Sweet", "Noticeable sweetness with rich umami. Fuller body, round mouthfeel. Rice sweetness more apparent. Smooth, rich, satisfying. Popular with sake beginners. Pairs with teriyaki, richer fish, miso-glazed dishes. Serve 55-60°F."),
        ("Sweet", "1.025-1.040", "Dessert", "Clear sweetness, rich rice character. Full-bodied, almost creamy texture. Sweet but maintains complexity and umami. Dessert-friendly, sipping sake. Pairs with mochi, fruit desserts, mild blue cheese. Small cups, savor slowly."),
        ("Kijoshu Sweet", "1.040+", "Noble", "Extremely sweet - made with sake instead of water. Thick, rich, liqueur-like. Intense concentrated flavors. Very rare, special occasion. Like rice sherry. Pairs with dessert only - chocolate, caramel, aged cheeses. Tiny portions."),
    ]);

    ui.add_space(30.0);
    ui.separator();
    ui.add_space(20.0);

    // PRO TIPS
    ui.heading(RichText::new("💡 PROFESSIONAL GUIDANCE").size(16.0).color(colors::SADDLE_BROWN));
    ui.add_space(10.0);

    ui.label(RichText::new("🎯 Sweetness Perception Factors:").strong().color(colors::SADDLE_BROWN));
    ui.label("• ABV masks sweetness - higher alcohol makes the same FG taste drier");
    ui.label("• Acidity cuts sweetness - high-acid beverages taste drier at same FG");
    ui.label("• Tannins add structure - make sweet beverages less cloying");
    ui.label("• Temperature affects perception - warm = sweeter, cold = drier");
    ui.label("• Carbonation lightens mouthfeel - makes sweet beverages more refreshing");

    ui.add_space(10.0);

    ui.label(RichText::new("⚗️ Achieving Target Sweetness:").strong().color(colors::SADDLE_BROWN));
    ui.label("• ALWAYS stabilize first (potassium sorbate + sulfite) before backsweetening");
    ui.label("• Taste as you sweeten - it's easy to add, impossible to remove");
    ui.label("• Let backsweetened beverages rest 1-2 weeks before final judgment");
    ui.label("• Consider acid balance - sometimes you need acid adjustment, not sugar");
    ui.label("• Cold crash before sweetening to drop sediment for clearer results");

    ui.add_space(10.0);

    ui.label(RichText::new("📊 Reading Hydrometer vs Taste:").strong().color(colors::SADDLE_BROWN));
    ui.label("• Use these ranges as guidelines, not absolutes");
    ui.label("• Two beverages at 1.015 FG can taste completely different");
    ui.label("• Trust your palate over the numbers");
    ui.label("• Different fermentables leave different residual flavors");
    ui.label("• Aging changes perception - what's sweet now may taste balanced later");

    ui.add_space(10.0);

    ui.label(RichText::new("🔬 Advanced Techniques:").strong().color(colors::SADDLE_BROWN));
    ui.label("• Blend batches - mix dry and sweet to hit exact sweetness");
    ui.label("• Step-feed honey during fermentation for controlled sweetness");
    ui.label("• Use arrested fermentation (cold crash + sorbate) to stop at target FG");
    ui.label("• Consider non-fermentable sugars (lactose, maltodextrin) for body without sweetness");
    ui.label("• Experiment with different sweeteners - each has unique character");
}

fn section_header(ui: &mut egui::Ui, title: &str, desc: &str, color: Color32) {
    ui.heading(RichText::new(title).size(16.0).color(color).strong());
    ui.label(RichText::new(desc).size(12.0).weak());
    ui.add_space(8.0);
}

fn sweetness_grid(ui: &mut egui::Ui, data: &[(&str, &str, &str, &str)]) {
    egui::Grid::new(ui.next_auto_id())
        .num_columns(4)
        .spacing([10.0, 6.0])
        .striped(true)
        .show(ui, |ui| {
            // Header
            ui.label(RichText::new("Level").strong().color(colors::SADDLE_BROWN));
            ui.label(RichText::new("Gravity").strong().color(colors::SADDLE_BROWN));
            ui.label(RichText::new("Character").strong().color(colors::SADDLE_BROWN));
            ui.label(RichText::new("Taste Profile & Pairings").strong().color(colors::SADDLE_BROWN));
            ui.end_row();

            // Data rows
            for (level, gravity, character, profile) in data {
                ui.label(RichText::new(*level).strong().color(colors::SADDLE_BROWN));
                ui.label(RichText::new(*gravity).color(colors::HONEY_GOLD));
                ui.label(RichText::new(*character).color(colors::FOREST_GREEN));
                // Enable wrapping for long text - dynamically adjusts to window size
                ui.add(egui::Label::new(RichText::new(*profile).size(11.0)).wrap());
                ui.end_row();
            }
        });
}

fn render_backsweetening(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍯 Backsweetening Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate sweetener additions to reach target sweetness");
    ui.label(RichText::new("⚠️ MUST stabilize before backsweetening!").color(colors::DARK_ORANGE).strong());
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "L" } else { "gal" };

    crate::input_field(ui, &format!("Volume ({}):", vol_unit), &mut app.sweet_vol, "Total volume to sweeten");
    crate::input_field(ui, "Current SG:", &mut app.current_sg, "Current specific gravity");
    crate::input_field(ui, "Target SG:", &mut app.target_sg, "Desired final gravity");

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

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Sweetener Amount") {
        calc_backsweetening(app);
    }
}

fn render_sulfite(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🛡️ Sulfite Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate K-meta additions with pH-dependent effectiveness");
    ui.add_space(10.0);

    let vol_unit = if matches!(app.state.unit_system, crate::state::UnitSystem::Metric) { "L" } else { "gal" };

    crate::input_field(ui, &format!("Volume ({}):", vol_unit), &mut app.sulfite_vol, "Total volume to treat");
    crate::input_field(ui, "pH:", &mut app.ph, "Current pH (critical for effectiveness!)");
    crate::input_field(ui, "Target Free SO₂ (ppm):", &mut app.target_so2, "Desired free SO₂ level (20-50 ppm typical)");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Sulfite Addition") {
        calc_sulfite(app);
    }
}

fn render_acid(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍋 Acid Addition Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate acid additions to adjust pH");
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
                ui.selectable_value(&mut app.acid_type, "tartaric".to_string(), "Tartaric (strongest, wine)");
                ui.selectable_value(&mut app.acid_type, "citric".to_string(), "Citric (bright, fruity)");
                ui.selectable_value(&mut app.acid_type, "malic".to_string(), "Malic (soft, apple-like)");
                ui.selectable_value(&mut app.acid_type, "lactic".to_string(), "Lactic (smooth, creamy)");
            });
    });

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate Acid Addition") {
        calc_acid_addition(app);
    }
}

fn calc_backsweetening(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("backsweetening") {
        Some(c) => c,
        None => {
            app.result = Some("Error: Backsweetening calculator not found".to_string());
            return;
        }
    };

    let current_sg_val = match Decimal::from_str(&app.current_sg) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid current SG value".to_string());
            return;
        }
    };

    let sg_meas = match Measurement::sg(current_sg_val) {
        Ok(m) => m,
        Err(e) => {
            app.result = Some(format!("Error: {}", e));
            return;
        }
    };

    let volume_val = match Decimal::from_str(&app.sweet_vol) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid volume".to_string());
            return;
        }
    };

    let is_metric = matches!(app.state.unit_system, crate::state::UnitSystem::Metric);

    let volume_liters = if is_metric {
        volume_val
    } else {
        volume_val * Decimal::new(3785, 3)
    };

    let input = CalcInput::new()
        .add_measurement(sg_meas)
        .add_param("volume", &volume_liters.to_string())
        .add_param("target_sg", &app.target_sg)
        .add_param("sweetener", &app.sweetener);

    match calc.calculate(input) {
        Ok(res) => {
            let (amount, weight_unit) = if is_metric {
                (res.output.value, "g")
            } else {
                let oz = res.output.value / Decimal::new(2835, 2);
                (oz, "oz")
            };

            app.result = Some(format!("Add {:.1} {} of {}", amount, weight_unit, app.sweetener.replace('_', " ")));
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

fn calc_sulfite(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("sulfite") {
        Some(c) => c,
        None => {
            app.result = Some("Error: Sulfite calculator not found".to_string());
            return;
        }
    };

    let ph_val = match Decimal::from_str(&app.ph) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid pH value".to_string());
            return;
        }
    };

    let ph_meas = match Measurement::ph(ph_val) {
        Ok(m) => m,
        Err(e) => {
            app.result = Some(format!("Error: {}", e));
            return;
        }
    };

    let volume_val = match Decimal::from_str(&app.sulfite_vol) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid volume".to_string());
            return;
        }
    };

    let is_metric = matches!(app.state.unit_system, crate::state::UnitSystem::Metric);

    let volume_liters = if is_metric {
        volume_val
    } else {
        volume_val * Decimal::new(3785, 3)
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
            app.result = Some(format!("Error: {}", e));
            app.warnings.clear();
            app.metadata.clear();
        }
    }
}

fn calc_acid_addition(app: &mut MazerionApp) {
    let calc = match mazerion_core::traits::get_calculator("acid_addition") {
        Some(c) => c,
        None => {
            app.result = Some("Error: Acid addition calculator not found".to_string());
            return;
        }
    };

    let current_ph_val = match Decimal::from_str(&app.current_ph) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid current pH".to_string());
            return;
        }
    };

    let current_ph_meas = match Measurement::ph(current_ph_val) {
        Ok(m) => m,
        Err(e) => {
            app.result = Some(format!("Error: {}", e));
            return;
        }
    };

    let volume_val = match Decimal::from_str(&app.acid_vol) {
        Ok(v) => v,
        Err(_) => {
            app.result = Some("Error: Invalid volume".to_string());
            return;
        }
    };

    let is_metric = matches!(app.state.unit_system, crate::state::UnitSystem::Metric);

    let volume_liters = if is_metric {
        volume_val
    } else {
        volume_val * Decimal::new(3785, 3)
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

            app.result = Some(format!("Acid: {:.2} {}", amount, weight_unit));
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