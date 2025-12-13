//! Beer calculators tab - IBU, SRM, Mash, Efficiency, Style Guide
//! SAFETY-CRITICAL: Zero panics, production-ready

pub(crate) use crate::state::BeerCalculator;
use crate::{state::colors, MazerionApp};
use eframe::egui::{self, CornerRadius, RichText, Color32, ScrollArea};

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Select Calculator:").strong());
        egui::ComboBox::from_id_salt("beer_calc")
            .selected_text(get_calc_name(app.state.beer_calc))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.state.beer_calc, BeerCalculator::Ibu, "IBU Calculator");
                ui.selectable_value(&mut app.state.beer_calc, BeerCalculator::Srm, "SRM Color Calculator");
                ui.selectable_value(&mut app.state.beer_calc, BeerCalculator::Mash, "Mash Water Calculator");
                ui.selectable_value(&mut app.state.beer_calc, BeerCalculator::Efficiency, "Brewhouse Efficiency");
                ui.selectable_value(&mut app.state.beer_calc, BeerCalculator::StyleGuide, "Beer Style Guide");
            });
    });

    ui.add_space(10.0);

    egui::Frame::default()
        .fill(colors::LIGHT_CREAM)
        .stroke(egui::Stroke::new(1.5, colors::HONEY_GOLD))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            match app.state.beer_calc {
                BeerCalculator::Ibu => render_ibu(app, ui),
                BeerCalculator::Srm => render_srm(ui),
                BeerCalculator::Mash => render_mash(ui),
                BeerCalculator::Efficiency => render_efficiency(ui),
                BeerCalculator::StyleGuide => render_style_guide(ui),
            }
        });
}

fn get_calc_name(calc: BeerCalculator) -> &'static str {
    match calc {
        BeerCalculator::Ibu => "IBU Calculator",
        BeerCalculator::Srm => "SRM Color Calculator",
        BeerCalculator::Mash => "Mash Water Calculator",
        BeerCalculator::Efficiency => "Brewhouse Efficiency",
        BeerCalculator::StyleGuide => "Beer Style Guide",
    }
}

fn render_style_guide(ui: &mut egui::Ui) {
    ui.heading(RichText::new("📖 BJCP Beer Style Guide").color(colors::SADDLE_BROWN).size(20.0));
    ui.label("Complete specifications for 29 classic beer styles");
    ui.add_space(15.0);

    ScrollArea::vertical().show(ui, |ui| {
        style_section(ui, "🍺 PALE ALES", &[
            ("American Pale Ale", "1.045-1.060", "1.010-1.015", "30-50", "5-10", "4.5-6.2%", "Moderate hop aroma with citrus, pine, or floral notes. Clean malt backbone. Medium body. Classic American hops shine. Sierra Nevada Pale Ale, Dale's Pale Ale, Stone Pale Ale."),
            ("English Pale Ale", "1.040-1.054", "1.008-1.014", "20-40", "5-14", "3.8-5.4%", "Earthy, herbal hop character. Biscuity malt. Medium-light body. Traditional English hops dominate. Fuller's ESB, Bass Pale Ale, Samuel Smith's."),
            ("IPA (American)", "1.056-1.075", "1.010-1.018", "40-70", "6-14", "5.5-7.5%", "Intense hop aroma: citrus, tropical fruit, pine. Strong bitterness balanced by malt. Medium-full body. The craft beer revolution. Pliny the Elder, Bell's Two Hearted, Dogfish Head 60 Minute."),
            ("English IPA", "1.050-1.070", "1.010-1.015", "40-60", "6-14", "5.0-7.5%", "Earthy, floral hops. More malt-forward than American. Medium body. Traditional British interpretation. Greene King IPA, Goose Island IPA, Samuel Smith's India Ale."),
        ]);

        ui.add_space(10.0);

        style_section(ui, "🥃 AMBER & BROWN ALES", &[
            ("American Amber", "1.045-1.060", "1.010-1.015", "25-40", "10-17", "4.5-6.2%", "Caramel malt sweetness, moderate hops. Balanced, smooth. Medium body. Malt and hops in harmony. Fat Tire, Anderson Valley Boont Amber, Bell's Amber."),
            ("English Brown Ale", "1.040-1.052", "1.008-1.013", "20-30", "12-22", "4.2-5.4%", "Nutty, toffee, caramel notes. Low hops. Medium-light body. Gentle, sessionable. Newcastle Brown, Samuel Smith's Nut Brown, Brooklyn Brown."),
            ("American Brown", "1.045-1.060", "1.010-1.016", "20-40", "18-35", "4.3-6.2%", "Chocolate, caramel malt. More hop presence than English. Medium body. American twist on tradition. Avery Ellie's Brown, Brooklyn Brown, Lost Coast Downtown Brown."),
        ]);

        ui.add_space(10.0);

        style_section(ui, "⚫ STOUTS & PORTERS", &[
            ("Dry Irish Stout", "1.036-1.044", "1.007-1.011", "25-45", "25-40", "4.0-5.0%", "Roasted barley dominates. Dry, coffee-like finish. Medium-light body. Creamy nitro pour. Guinness, Murphy's, Beamish."),
            ("Sweet Stout", "1.044-1.060", "1.012-1.024", "20-40", "30-40", "4.0-6.0%", "Sweet, milk chocolate character. Lactose adds sweetness and body. Full body. Dessert-like. Left Hand Milk Stout, Mackeson's, Samuel Adams Cream Stout."),
            ("Oatmeal Stout", "1.048-1.065", "1.010-1.018", "25-40", "22-40", "4.2-5.9%", "Smooth, silky from oats. Coffee, chocolate notes. Medium-full body. Velvety mouthfeel. Samuel Smith's Oatmeal, Rogue Shakespeare, Anderson Valley Barney Flats."),
            ("American Stout", "1.050-1.075", "1.010-1.022", "35-75", "30-40", "5.0-7.0%", "Bold roast character, strong hop presence. Full body. American hops meet dark malt. Rogue Shakespeare, Sierra Nevada Stout, North Coast Old Rasputin."),
            ("Russian Imperial Stout", "1.075-1.115", "1.018-1.030", "50-90", "30-40", "8.0-12.0%", "Intense roasted malt, dark fruit complexity. Very full body. Rich, warming, complex. Old Rasputin, Founders KBS, Three Floyds Dark Lord."),
            ("Robust Porter", "1.048-1.065", "1.012-1.016", "25-50", "22-35", "4.8-6.5%", "Roasted malt, chocolate, coffee. Balanced bitterness. Medium-full body. Dark but approachable. Anchor Porter, Deschutes Black Butte, Sierra Nevada Porter."),
        ]);

        ui.add_space(10.0);

        style_section(ui, "🍯 WHEAT BEERS", &[
            ("American Wheat", "1.040-1.055", "1.008-1.013", "15-30", "2-6", "4.0-5.5%", "Clean wheat character, citrus hop notes. Light, crisp. Light-medium body. Refreshing summer beer. Bell's Oberon, Boulevard Unfiltered Wheat, Goose Island 312."),
            ("Hefeweizen", "1.044-1.052", "1.010-1.014", "8-15", "2-6", "4.3-5.6%", "Banana and clove from yeast. Cloudy, unfiltered. Medium-light body. Bavarian classic. Weihenstephaner, Paulaner, Franziskaner."),
            ("Witbier", "1.044-1.052", "1.008-1.012", "10-20", "2-4", "4.5-5.5%", "Coriander, orange peel spicing. Cloudy, refreshing. Light-medium body. Belgian summer classic. Hoegaarden, Allagash White, Blue Moon."),
        ]);

        ui.add_space(10.0);

        style_section(ui, "🏆 LAGERS", &[
            ("American Lager", "1.040-1.050", "1.004-1.010", "8-18", "2-4", "4.2-5.3%", "Crisp, clean, neutral. Very light body. Highly carbonated. Mass-market appeal. Budweiser, Miller, Coors."),
            ("Pilsner (German)", "1.044-1.050", "1.008-1.013", "25-45", "2-5", "4.4-5.2%", "Noble hop aroma, crisp bitterness. Light body. Elegant, refined. Original pilsner. Pilsner Urquell, Bitburger, König."),
            ("Pilsner (Czech)", "1.044-1.056", "1.013-1.017", "30-45", "3.5-6", "4.2-5.4%", "Rich malt, spicy Saaz hops. Medium body. More malt-forward than German. Pilsner Urquell, Staropramen, Czechvar."),
            ("Märzen/Oktoberfest", "1.054-1.060", "1.010-1.014", "18-24", "8-17", "5.8-6.3%", "Toasty malt, clean finish. Medium body. Amber lager perfection. Seasonal favorite. Paulaner, Spaten, Sam Adams Octoberfest."),
            ("Bock", "1.064-1.072", "1.013-1.019", "20-27", "14-22", "6.3-7.2%", "Malty, slightly sweet. Full body. Minimal hop presence. Malt showcase. Einbecker, Shiner Bock, Ayinger Celebrator."),
            ("Doppelbock", "1.072-1.112", "1.016-1.024", "16-26", "16-26", "7.0-10.0%", "Rich, intense malt. Very full body. Dark fruit complexity. Strong lager. Paulaner Salvator, Ayinger Celebrator, Spaten Optimator."),
        ]);

        ui.add_space(10.0);

        style_section(ui, "🇧🇪 BELGIAN STYLES", &[
            ("Belgian Blonde", "1.062-1.075", "1.008-1.018", "15-30", "4-7", "6.0-7.5%", "Fruity esters, spicy phenols. Medium body. Complex but approachable. Elegant Belgian ale. Leffe Blonde, Affligem, La Chouffe."),
            ("Belgian Dubbel", "1.062-1.075", "1.008-1.018", "15-25", "10-17", "6.0-7.6%", "Dark fruit, caramel, spice. Medium-full body. Malty complexity. Abbey-style classic. Westmalle Dubbel, Chimay Red, La Trappe."),
            ("Belgian Tripel", "1.075-1.085", "1.008-1.014", "20-40", "4.5-7", "7.5-9.5%", "Fruity, spicy, strong. Medium-light body despite ABV. Deceptively drinkable. Belgian masterpiece. Westmalle Tripel, Chimay White, La Fin du Monde."),
            ("Saison", "1.048-1.065", "1.002-1.012", "20-35", "5-14", "5.0-7.0%", "Fruity, spicy, peppery. Light-medium body. Highly carbonated. Farmhouse refreshment. Saison Dupont, Boulevard Tank 7, Ommegang Hennepin."),
        ]);

        ui.add_space(10.0);

        style_section(ui, "🔥 STRONG ALES", &[
            ("Barleywine (American)", "1.080-1.120", "1.016-1.030", "50-100", "10-19", "8.0-12.0%", "Intense malt, strong hops. Very full body. Rich, complex, age-worthy. Beer for sipping. Sierra Nevada Bigfoot, Anchor Old Foghorn, Stone Old Guardian."),
            ("Barleywine (English)", "1.080-1.120", "1.018-1.030", "35-70", "8-22", "8.0-12.0%", "Fruity, malty, complex. Very full body. Sherry-like with age. British strong ale tradition. J.W. Lees Harvest Ale, Fuller's Golden Pride, Young's Old Nick."),
            ("Scotch Ale (Wee Heavy)", "1.070-1.130", "1.018-1.056", "17-35", "14-25", "6.5-10.0%", "Rich, malty, sweet. Full body. Caramel dominance. Scottish strength. Founders Dirty Bastard, Oskar Blues Old Chub, Traquair House."),
        ]);
    });
}

fn style_section(ui: &mut egui::Ui, title: &str, styles: &[(&str, &str, &str, &str, &str, &str, &str)]) {
    ui.label(RichText::new(title).size(18.0).color(colors::SADDLE_BROWN).strong());
    ui.add_space(5.0);

    for (name, og, fg, ibu, srm, abv, desc) in styles {
        egui::Frame::default()
            .fill(Color32::WHITE)
            .stroke(egui::Stroke::new(1.0, colors::HONEY_GOLD))
            .corner_radius(CornerRadius::same(6))
            .inner_margin(10.0)
            .show(ui, |ui| {
                ui.heading(RichText::new(*name).color(colors::SADDLE_BROWN).size(14.0));
                ui.label(RichText::new(format!("OG: {} | FG: {} | IBU: {} | SRM: {} | ABV: {}", og, fg, ibu, srm, abv))
                    .color(Color32::from_rgb(0, 100, 0)).size(11.0).strong());
                ui.add_space(5.0);
                ui.label(*desc);
            });
        ui.add_space(8.0);
    }
}

fn render_ibu(app: &mut MazerionApp, ui: &mut egui::Ui) {
    ui.heading(RichText::new("🍺 IBU Calculator (Tinseth)").color(colors::SADDLE_BROWN));
    ui.label("Calculate International Bitterness Units");
    ui.add_space(10.0);

    crate::input_field(ui, "Hop Weight (oz):", &mut app.hop_weight, "Weight of hops");
    crate::input_field(ui, "Alpha Acid %:", &mut app.alpha_acid, "Alpha acid percentage");
    crate::input_field(ui, "Boil Time (min):", &mut app.boil_time, "Minutes in boil");
    crate::input_field(ui, "Batch Volume (gal):", &mut app.volume, "Final volume");
    crate::input_field(ui, "Wort Gravity (SG):", &mut app.sg, "Pre-boil gravity");

    ui.add_space(10.0);

    if crate::calculate_button(ui, "Calculate IBU") {
        app.result = Some("IBU calculator - pending backend".to_string());
    }
}

fn render_srm(ui: &mut egui::Ui) {
    ui.heading(RichText::new("🎨 SRM Color Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate beer color in SRM");
    ui.add_space(10.0);

    ui.label("SRM calculator - coming soon");
}

fn render_mash(ui: &mut egui::Ui) {
    ui.heading(RichText::new("💧 Mash Water Calculator").color(colors::SADDLE_BROWN));
    ui.label("Calculate water volumes for mashing");
    ui.add_space(10.0);

    ui.label("Mash calculator - coming soon");
}

fn render_efficiency(ui: &mut egui::Ui) {
    ui.heading(RichText::new("📊 Brewhouse Efficiency").color(colors::SADDLE_BROWN));
    ui.label("Calculate extraction efficiency");
    ui.add_space(10.0);

    ui.label("Efficiency calculator - coming soon");
}