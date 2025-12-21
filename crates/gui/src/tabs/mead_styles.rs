use crate::MazerionApp;
use eframe::egui::{self, Color32, RichText};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeadStyle {
    Traditional,
    Melomel,
    Metheglin,
    Cyser,
    Pyment,
    Braggot,
    Bochet,
    Acerglyn,
    Capsicumel,
    Rhodomel,
    Hippocras,
    Morat,
    SackMead,
    Hydromel,
    GreatMead,
}

impl Default for MeadStyle {
    fn default() -> Self {
        Self::Traditional
    }
}

impl MeadStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Traditional => "Traditional",
            Self::Melomel => "Melomel",
            Self::Metheglin => "Metheglin",
            Self::Cyser => "Cyser",
            Self::Pyment => "Pyment",
            Self::Braggot => "Braggot",
            Self::Bochet => "Bochet",
            Self::Acerglyn => "Acerglyn",
            Self::Capsicumel => "Capsicumel",
            Self::Rhodomel => "Rhodomel",
            Self::Hippocras => "Hippocras",
            Self::Morat => "Morat",
            Self::SackMead => "Sack Mead",
            Self::Hydromel => "Hydromel",
            Self::GreatMead => "Great Mead",
        }
    }
}

pub fn render(app: &mut MazerionApp, ui: &mut egui::Ui) {
    let c = &app.state.custom_colors;

    ui.heading(RichText::new("🍯 Comprehensive Mead Styles Guide")
        .size(26.0)
        .color(c.honey_gold));
    ui.add_space(10.0);

    ui.label(RichText::new("Complete reference for all major mead categories and styles")
        .size(14.0)
        .color(c.dark_text));

    ui.add_space(20.0);

    // Traditional Meads Category
    render_category_header(ui, c, "Traditional Meads (Show Meads)");

    render_style(ui, c, StyleInfo {
        name: "Traditional Mead (Show Mead)",
        fg_range: "0.996 - 1.012",
        abv_range: "10-15%",
        og_range: "1.080 - 1.120",
        ingredients: "Honey, Water, Yeast only",
        description: "The purest expression of honey. No fruit, spices, or adjuncts. Showcases honey varietal characteristics. Can be dry, semi-sweet, or sweet. Aging reveals complex floral, herbal, and sometimes vinous notes.",
        characteristics: "Color: Pale straw to deep gold depending on honey\nClarity: Brilliant clarity expected\nAroma: Honey-forward with varietal notes (floral, herbal, fruity)\nFlavor: Clean honey character, balanced sweetness\nMouthfeel: Light to medium body, smooth",
        aging: "12-24 months minimum. Benefits from extended aging 3-5+ years. Great meads improve for decades.",
        tips: "• Use high-quality varietal honey for best results\n• Wildflower, orange blossom, clover, buckwheat each shine\n• Proper nutrient schedule (TOSNA, Fermaid O) critical\n• Temperature control 60-68°F for clean fermentation\n• Avoid oxidation during aging\n• This is the benchmark style for competitions",
    });

    render_style(ui, c, StyleInfo {
        name: "Sack Mead (Dessert Mead)",
        fg_range: "1.025 - 1.050",
        abv_range: "14-20%",
        og_range: "1.120 - 1.170",
        ingredients: "High amounts of honey, Water, Alcohol-tolerant yeast",
        description: "High-gravity dessert mead. Very sweet, very strong. Rich, unctuous, honey-bomb. Comparable to vintage port or Sauternes. Sipping mead served in small portions.",
        characteristics: "Color: Deep gold to amber\nClarity: Brilliant\nAroma: Intense honey, dried fruit, alcohol warmth\nFlavor: Rich honey sweetness, complex aged notes\nMouthfeel: Full, syrupy, warming",
        aging: "24-48 months minimum. Peak at 5-10 years. Can age indefinitely.",
        tips: "• Use 4-6 lbs honey per gallon (OG 1.120-1.160)\n• Requires high alcohol-tolerant yeast (EC-1118, K1V-1116, 71B)\n• Extended nutrient additions over 10-14 days\n• Fermentation can take 6-12 months\n• May need to step-feed honey for extreme gravity\n• Serve at room temperature in small pours (2-3 oz)",
    });

    render_style(ui, c, StyleInfo {
        name: "Hydromel (Session Mead)",
        fg_range: "0.998 - 1.008",
        abv_range: "4-8%",
        og_range: "1.030 - 1.055",
        ingredients: "Lower honey amounts, Water, Yeast",
        description: "Light, refreshing, sessionable mead. Low alcohol makes it perfect for warm weather or extended drinking. Often carbonated. Delicate honey character. Quick turn-around style.",
        characteristics: "Color: Pale straw to light gold\nClarity: Brilliant if still, can be hazy if carbonated\nAroma: Subtle honey, light and fresh\nFlavor: Clean, refreshing, light honey notes\nMouthfeel: Light, crisp, often effervescent",
        aging: "3-6 months. Drink young and fresh.",
        tips: "• Use 1.5-2 lbs honey per gallon\n• Lower nutrient requirements\n• Finish dry to off-dry (1.000-1.008)\n• Perfect for carbonation (2.5-3 volumes CO₂)\n• Can add light fruit for variety\n• Popular in Europe, gaining traction in US craft scene",
    });

    render_style(ui, c, StyleInfo {
        name: "Great Mead (Aged Traditional)",
        fg_range: "1.000 - 1.010",
        abv_range: "12-16%",
        og_range: "1.090 - 1.130",
        ingredients: "Premium honey, Water, Yeast, Time (3-10+ years)",
        description: "Traditional mead aged extensively for complexity. Develops sherry-like oxidative notes, smoothness, and integrated flavors. Comparable to fine aged wine. Investment style requiring patience.",
        characteristics: "Color: Deep gold to amber with age\nClarity: Brilliant\nAroma: Complex honey, sherry notes, dried fruit, nuts\nFlavor: Smooth, integrated, oxidatively complex\nMouthfeel: Silky, refined, balanced",
        aging: "Minimum 3 years, often 5-10+. No upper limit with proper cellaring.",
        tips: "• Start with quality traditional mead\n• Oak barrels or spirals add complexity\n• Slight oxidation over time is desirable\n• Store in cool, dark cellar (55-60°F)\n• Can develop solera system for perpetual aging\n• Dry to off-dry most common\n• Each year brings new complexity",
    });

    ui.add_space(25.0);

    // Fruit Meads Category
    render_category_header(ui, c, "Fruit Meads (Melomels)");

    render_style(ui, c, StyleInfo {
        name: "Melomel (General Fruit Mead)",
        fg_range: "1.006 - 1.018",
        abv_range: "10-16%",
        og_range: "1.080 - 1.130",
        ingredients: "Honey, Fruit (fresh, frozen, juice, puree), Water, Yeast",
        description: "Fruit-forward mead with honey in supporting role. Balance between fruit character and honey sweetness. Any fruit or combination. Color and flavor vary by fruit selection.",
        characteristics: "Color: Pale gold to deep purple (fruit-dependent)\nClarity: Can be hazy with fruit solids, brilliant if aged/filtered\nAroma: Fruit-forward with honey background\nFlavor: Fresh fruit balanced with honey sweetness\nMouthfeel: Medium body, fruit tannins",
        aging: "6-18 months. Delicate fruits shorter, dark fruits longer.",
        tips: "• Use 2-4 lbs fruit per gallon for strong flavor\n• Add fruit in secondary for best control and freshness\n• Backsweeten to 1.010-1.015 to balance fruit acidity\n• Popular fruits: raspberry, blueberry, cherry, strawberry, peach\n• Can blend multiple fruits\n• Pectic enzyme recommended for clarity",
    });

    render_style(ui, c, StyleInfo {
        name: "Cyser (Apple Mead)",
        fg_range: "1.008 - 1.018",
        abv_range: "8-14%",
        og_range: "1.070 - 1.110",
        ingredients: "Honey, Apple juice or cider, Yeast",
        description: "Marriage of apple and honey. Crisp apple notes with honey complexity. Ranges from champagne-dry to dessert-sweet. Apple tartness balanced by honey. Fall seasonal favorite.",
        characteristics: "Color: Pale gold to amber\nClarity: Brilliant\nAroma: Fresh apple, honey, sometimes spice\nFlavor: Apple-forward with honey sweetness\nMouthfeel: Crisp, refreshing, medium body",
        aging: "6-12 months. Can drink young or age for complexity.",
        tips: "• Use fresh-pressed cider or quality juice (NO preservatives)\n• Typical ratio: 60-70% apple juice, 30-40% honey must\n• Semi-sweet (1.010-1.015) most popular\n• Consider apple variety: tart (Granny Smith) vs sweet (Fuji)\n• Excellent with fall spices: cinnamon, nutmeg, allspice\n• Carbonation popular (still or sparkling both work)",
    });

    render_style(ui, c, StyleInfo {
        name: "Pyment (Grape Mead)",
        fg_range: "0.996 - 1.012",
        abv_range: "11-16%",
        og_range: "1.085 - 1.125",
        ingredients: "Honey, Grape juice or wine, Yeast",
        description: "Wine-mead hybrid. Honey adds body, smoothness, and aromatics to wine character. Can use white or red grapes. Elegant and refined. Most wine-like of all mead styles.",
        characteristics: "Color: Pale gold (white) to deep ruby (red)\nClarity: Brilliant\nAroma: Grape/wine character with honey complexity\nFlavor: Wine-like with added honey dimension\nMouthfeel: Medium to full body, wine tannins",
        aging: "12-24 months. Red pyments need longer. Oak aging common.",
        tips: "• Use quality grape juice or blend finished wine with mead\n• White pyment: Chardonnay, Riesling, Gewürztraminer\n• Red pyment: Merlot, Cabernet, Pinot Noir\n• Typically dry to off-dry (wine-like finish)\n• Wine yeast (RC-212, D-47) for authentic character\n• Oak aging adds complexity\n• Acid balance critical",
    });

    render_style(ui, c, StyleInfo {
        name: "Morat (Mulberry Mead)",
        fg_range: "1.008 - 1.018",
        abv_range: "10-14%",
        og_range: "1.080 - 1.110",
        ingredients: "Honey, Mulberries (fresh or frozen), Water, Yeast",
        description: "Historical medieval style. Deep purple color. Delicate berry flavor with honey sweetness. Mulberries provide light tannin structure. Elegant and refined. Similar to blackberry but more delicate.",
        characteristics: "Color: Deep purple to ruby\nClarity: Can be hazy, brilliance with aging\nAroma: Delicate berry, honey\nFlavor: Subtle mulberry, balanced honey\nMouthfeel: Medium body, light tannins",
        aging: "8-16 months for full color and flavor integration.",
        tips: "• Traditional medieval recipe (12th century)\n• Use fresh or frozen mulberries (3-4 lbs/gal)\n• Color is stunning - deep purple to ruby\n• Semi-sweet finish (1.010-1.015) showcases delicate fruit\n• Mulberries hard to find - can use mulberry juice\n• Similar to blackberry melomel but more refined\n• Historical recreation popular in SCA",
    });

    ui.add_space(25.0);

    // Spiced & Herbal Category
    render_category_header(ui, c, "Spiced & Herbal Meads (Metheglins)");

    render_style(ui, c, StyleInfo {
        name: "Metheglin (Spice/Herb Mead)",
        fg_range: "0.996 - 1.015",
        abv_range: "10-15%",
        og_range: "1.080 - 1.120",
        ingredients: "Honey, Spices and/or herbs, Water, Yeast",
        description: "Spice or herb-forward mead. Wide range from warming spices (cinnamon, ginger) to cooling herbs (mint, chamomile). Can be medicinal, culinary, or aromatic. Ancient style with therapeutic origins.",
        characteristics: "Color: Pale gold to amber (spice-dependent)\nClarity: Brilliant\nAroma: Spice/herb forward with honey\nFlavor: Balanced spice and honey\nMouthfeel: Variable by spices used",
        aging: "8-18 months. Strong spices mellow with time. Delicate herbs shorter.",
        tips: "• START LIGHT - easy to over-spice, impossible to remove\n• Add spices in secondary for better control\n• Traditional spices: cinnamon, clove, ginger, vanilla, nutmeg\n• Herbs: chamomile, lavender, rose hips, elderflower, mint\n• Tea-based metheglins increasingly popular\n• Can combine multiple spices but keep it simple",
    });

    render_style(ui, c, StyleInfo {
        name: "Rhodomel (Rose Petal Mead)",
        fg_range: "1.000 - 1.012",
        abv_range: "10-13%",
        og_range: "1.080 - 1.105",
        ingredients: "Honey, Rose petals or rose water, Water, Yeast",
        description: "Delicate floral mead. Rose provides perfume-like aromatics without heavy flavor. Light, elegant, romantic. Ancient style dating to classical period. Requires restraint - easily overdone.",
        characteristics: "Color: Pale gold to light pink\nClarity: Brilliant\nAroma: Floral, rose perfume, honey\nFlavor: Delicate rose, balanced honey\nMouthfeel: Light to medium, smooth",
        aging: "6-12 months. Delicate flavors don't need extended aging.",
        tips: "• Use organic rose petals (MUST be pesticide-free)\n• OR use rose water (easier to control, add in secondary 1-2 tsp/gal)\n• Very easy to over-do - start with HALF what you think\n• Dry to off-dry finish (1.000-1.008)\n• Pair with light honey (orange blossom, acacia)\n• Popular for weddings, anniversaries, gifts\n• Can add hibiscus for color",
    });

    render_style(ui, c, StyleInfo {
        name: "Capsicumel (Pepper Mead)",
        fg_range: "1.008 - 1.015",
        abv_range: "10-14%",
        og_range: "1.080 - 1.110",
        ingredients: "Honey, Peppers (jalapeño to ghost), Water, Yeast",
        description: "Pepper heat balanced by honey sweetness. Ranges from mild warmth to extreme fire. Unique and challenging style. Sweetness critical to temper heat. Often combined with fruit.",
        characteristics: "Color: Pale gold (depends on pepper)\nClarity: Brilliant\nAroma: Honey with pepper heat (capsaicin aroma)\nFlavor: Sweet honey upfront, building pepper heat\nMouthfeel: Medium body, burning sensation",
        aging: "6-12 months. Heat mellows slightly with age.",
        tips: "• Start with ONE pepper type for first batch\n• Add peppers in secondary for heat control\n• MUST finish sweet: 1.010-1.015 to balance heat\n• Popular peppers: jalapeño (mild), habanero (hot), ghost (extreme)\n• Remove seeds/ribs for less heat, keep for maximum\n• Pair with fruit: mango-habanero, pineapple-jalapeño\n• Wear gloves when handling hot peppers",
    });

    ui.add_space(25.0);

    // Specialty & Hybrid Category
    render_category_header(ui, c, "Specialty & Hybrid Meads");

    render_style(ui, c, StyleInfo {
        name: "Braggot (Bracket)",
        fg_range: "1.008 - 1.020",
        abv_range: "7-14%",
        og_range: "1.070 - 1.120",
        ingredients: "Honey, Malted grains, Hops (optional), Yeast",
        description: "Honey-beer hybrid. Malt provides beer-like body and character. Honey adds smoothness and complexity. Can be hoppy or hop-free. Ancient Anglo-Saxon style with wide variation.",
        characteristics: "Color: Pale gold to dark brown (malt-dependent)\nClarity: Can be hazy (unfiltered) or brilliant\nAroma: Malt and honey, possible hop notes\nFlavor: Balance of malt sweetness and honey\nMouthfeel: Medium to full body",
        aging: "4-12 months. Light versions drink younger, high-gravity longer.",
        tips: "• Ratio varies: 50/50 honey/malt typical, but 25/75 to 75/25 all work\n• Base malt: 2-row pale malt\n• Specialty malts: crystal, chocolate, roasted for character\n• Hops optional - balance honey sweetness if used\n• Ferment with ale yeast (beer-like) or wine yeast (mead-like)\n• Can be still or carbonated\n• Ancient style, many interpretations",
    });

    render_style(ui, c, StyleInfo {
        name: "Acerglyn (Maple Mead)",
        fg_range: "1.010 - 1.022",
        abv_range: "10-15%",
        og_range: "1.085 - 1.125",
        ingredients: "Honey, Maple syrup (real), Water, Yeast",
        description: "Maple and honey combination. Maple adds earthy, woody sweetness. Honey provides floral complexity. Fall/winter seasonal favorite. Medium to full body. Often carbonated.",
        characteristics: "Color: Light amber to dark amber\nClarity: Brilliant\nAroma: Maple, honey, subtle wood notes\nFlavor: Maple sweetness with honey complexity\nMouthfeel: Medium to full body, smooth",
        aging: "8-16 months. Maple and honey flavors integrate over time.",
        tips: "• Use REAL maple syrup (Grade A Dark or Grade B)\n• Never use maple-flavored corn syrup\n• Typical ratio: 60-70% honey, 30-40% maple syrup\n• Needs semi-sweet to sweet finish (1.012-1.020)\n• Pairs well with vanilla, cinnamon, pecans\n• Often carbonated for champagne-style\n• Fall/winter seasonal release",
    });

    render_style(ui, c, StyleInfo {
        name: "Bochet (Caramelized Honey Mead)",
        fg_range: "1.015 - 1.030",
        abv_range: "12-18%",
        og_range: "1.095 - 1.140",
        ingredients: "Caramelized honey, Water, Yeast",
        description: "Caramelized honey creates toffee, butterscotch, burnt sugar complexity. Dark amber to mahogany. Dessert mead. Ancient technique creating unique character. Requires sweet finish to balance caramel bitterness.",
        characteristics: "Color: Dark amber to mahogany\nClarity: Can be hazy from caramelization\nAroma: Toffee, butterscotch, burnt sugar, dark honey\nFlavor: Rich caramel, complex, hint of bitterness\nMouthfeel: Full body, rich",
        aging: "12-36 months. Harsh caramel notes mellow and integrate with time.",
        tips: "• Caramelize 50-100% of honey (reserve some fresh for balance)\n• Light bochet: cook to amber (mild caramel notes)\n• Medium bochet: cook to dark brown (strong toffee)\n• Dark bochet: cook until nearly burnt (intense, bitter)\n• DANGEROUS: molten honey is 300°F+ - heavy pot, watch constantly\n• MUST finish very sweet (1.020+) to balance bitterness\n• Pairs with vanilla, oak aging",
    });

    render_style(ui, c, StyleInfo {
        name: "Hippocras (Spiced Pyment)",
        fg_range: "1.006 - 1.015",
        abv_range: "11-15%",
        og_range: "1.085 - 1.120",
        ingredients: "Honey, Grape juice, Medieval spices, Yeast",
        description: "Historical spiced wine-mead. Medieval European recipe combining pyment base with warming spices. Cinnamon, ginger, grains of paradise traditional. Mulled wine character with honey complexity.",
        characteristics: "Color: Deep red to ruby\nClarity: Brilliant\nAroma: Spiced wine, honey, warming aromatics\nFlavor: Red wine base with spice and honey\nMouthfeel: Medium to full body",
        aging: "10-18 months. Spices integrate and mellow.",
        tips: "• Historical recipe (12th-15th century Europe)\n• Base: red grape juice + honey (pyment)\n• Traditional spices: cinnamon, ginger, grains of paradise, long pepper, galangal\n• Semi-sweet finish (1.010-1.015)\n• Can be served warm like mulled wine\n• Holiday seasonal favorite\n• Popular at Renaissance faires and medieval events",
    });

    ui.add_space(25.0);

    // Quick reference section
    egui::Frame::default()
        .fill(c.light_cream)
        .stroke(egui::Stroke::new(2.0, c.forest_green))
        .corner_radius(egui::CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            ui.label(RichText::new("📊 Quick Reference: Sweetness Levels")
                .size(18.0)
                .strong()
                .color(c.forest_green));
            ui.add_space(10.0);

            let sweetness = [
                ("Bone Dry", "0.990-0.996", "Crisp, wine-like, tart finish"),
                ("Dry", "0.996-1.006", "Clean, subtle honey sweetness"),
                ("Semi-Sweet", "1.006-1.015", "Balanced, most popular range"),
                ("Sweet", "1.015-1.025", "Dessert mead, honey-forward"),
                ("Very Sweet", "1.025-1.040+", "Sack mead, sipping strength"),
            ];

            for (name, fg, desc) in sweetness {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(name).strong().size(14.0).color(c.honey_gold));
                    ui.label(RichText::new(fg).size(14.0).color(c.dark_text));
                    ui.label(RichText::new(format!("- {}", desc)).size(13.0).color(c.dark_text));
                });
            }
        });
}

struct StyleInfo {
    name: &'static str,
    fg_range: &'static str,
    abv_range: &'static str,
    og_range: &'static str,
    ingredients: &'static str,
    description: &'static str,
    characteristics: &'static str,
    aging: &'static str,
    tips: &'static str,
}

fn render_category_header(ui: &mut egui::Ui, c: &crate::state::CustomColors, category_name: &str) {
    ui.label(RichText::new(category_name)
        .size(22.0)
        .strong()
        .color(c.forest_green));
    ui.separator();
    ui.add_space(15.0);
}

fn render_style(ui: &mut egui::Ui, c: &crate::state::CustomColors, style: StyleInfo) {
    egui::Frame::default()
        .fill(Color32::WHITE)
        .stroke(egui::Stroke::new(2.0, c.honey_gold))
        .corner_radius(egui::CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            // Style name
            ui.label(RichText::new(style.name)
                .size(20.0)
                .strong()
                .color(c.forest_green));
            ui.add_space(8.0);

            // Stats box
            egui::Frame::default()
                .fill(c.light_cream)
                .inner_margin(10.0)
                .corner_radius(egui::CornerRadius::same(5))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("FG:").strong().color(c.dark_text));
                        ui.label(RichText::new(style.fg_range).color(c.honey_gold));
                        ui.label(" | ");
                        ui.label(RichText::new("ABV:").strong().color(c.dark_text));
                        ui.label(RichText::new(style.abv_range).color(c.honey_gold));
                        ui.label(" | ");
                        ui.label(RichText::new("OG:").strong().color(c.dark_text));
                        ui.label(RichText::new(style.og_range).color(c.honey_gold));
                    });
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Ingredients:").strong().color(c.dark_text));
                        ui.label(RichText::new(style.ingredients).color(c.dark_text));
                    });
                });

            ui.add_space(10.0);

            // Description
            ui.label(RichText::new(style.description)
                .size(14.0)
                .color(c.dark_text));

            ui.add_space(10.0);

            // Characteristics
            ui.label(RichText::new("Characteristics:")
                .size(14.0)
                .strong()
                .color(c.forest_green));
            for line in style.characteristics.lines() {
                ui.label(RichText::new(line).size(13.0).color(c.dark_text));
            }

            ui.add_space(10.0);

            // Aging
            ui.horizontal(|ui| {
                ui.label(RichText::new("⏱ Aging:").size(14.0).strong().color(c.sunset_orange));
                ui.label(RichText::new(style.aging).size(13.0).color(c.dark_text));
            });

            ui.add_space(10.0);

            // Tips
            ui.label(RichText::new("💡 Brewing Tips:")
                .size(14.0)
                .strong()
                .color(c.honey_gold));
            ui.add_space(5.0);
            for line in style.tips.lines() {
                ui.label(RichText::new(line).size(13.0).color(c.dark_text));
            }
        });

    ui.add_space(15.0);
}