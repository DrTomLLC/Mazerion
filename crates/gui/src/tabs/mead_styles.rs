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

    ui.heading(RichText::new("🍯 Comprehensive Mead Styles Encyclopedia")
        .size(26.0)
        .color(c.honey_gold));
    ui.add_space(10.0);

    ui.label(RichText::new("Complete reference for all mead categories, styles, and subcategories")
        .size(14.0)
        .color(c.dark_text));

    ui.add_space(20.0);

    // CATEGORY 1: TRADITIONAL MEADS
    egui::CollapsingHeader::new(RichText::new("📜 Traditional Meads (Show Meads)")
        .size(22.0)
        .strong()
        .color(c.forest_green))
        .default_open(true)
        .show(ui, |ui| {
            ui.add_space(10.0);

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
                food_pairings: "Cheese: Aged Comté, Gruyère, sharp aged cheddar, Manchego. Mild chèvre if dry.\nProtein: Roasted chicken with herbs, pork tenderloin with apple glaze, pan-seared scallops, grilled white fish.\nDessert: Honey panna cotta, lemon tart, almond biscotti, crème brûlée.\nCuisine: Mediterranean mezze, French bistro fare, refined poultry dishes.\nSpecialty: Honeycomb, marcona almonds, dried apricots, charcuterie with honey mustard.",
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
                food_pairings: "Cheese: Stilton, Roquefort, aged Gouda, triple-crème Brie.\nDessert: Dark chocolate torte, baklava, pecan pie, tiramisu, crème caramel.\nProtein: Foie gras, duck confit, braised short ribs.\nNuts: Candied walnuts, honey-roasted pecans, pralines.\nSpecialty: After-dinner digestif. Pairs with cigars. Drizzle over vanilla ice cream.",
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
                food_pairings: "Cheese: Fresh mozzarella, burrata, mild feta, young chèvre.\nProtein: Grilled shrimp, chicken satay, fish tacos, turkey sandwiches.\nLight fare: Summer salads, vegetable crudités, hummus, bruschetta.\nCuisine: Picnic foods, tapas, light Asian cuisine, Mediterranean appetizers.\nSpecialty: Excellent session pairing for long meals, BBQ events, outdoor gatherings.",
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
                food_pairings: "Cheese: Aged Parmigiano-Reggiano, vintage Gouda, aged Mimolette, cave-aged Gruyère.\nProtein: Dry-aged beef, wild game (venison, duck), bone-in ribeye.\nCuisine: Fine dining, contemplative sipping experience.\nSpecialty: Pair like vintage port or fine Cognac. Nuts, dark chocolate, contemplative evening.\nOccasion: Special celebrations, cigar pairing, after-dinner sophistication.",
            });
        });

    ui.add_space(15.0);

    // CATEGORY 2: FRUIT MEADS (MELOMELS)
    egui::CollapsingHeader::new(RichText::new("🍓 Fruit Meads (Melomels)")
        .size(22.0)
        .strong()
        .color(c.forest_green))
        .default_open(false)
        .show(ui, |ui| {
            ui.add_space(10.0);

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
                food_pairings: "Cheese: Brie, Camembert, mild blue cheese, fontina.\nProtein: Pork chops with fruit reduction, duck breast, roasted turkey.\nDessert: Fruit tarts, berry crumble, sorbet.\nCuisine: Varies by fruit - generally versatile with poultry and pork.\nSpecialty: Charcuterie with fruit preserves, glazed ham.",
            });

            // Berry Melomels subcategory
            egui::CollapsingHeader::new(RichText::new("  🫐 Berry Melomels")
                .size(18.0)
                .color(c.honey_gold))
                .default_open(false)
                .show(ui, |ui| {
                    ui.add_space(5.0);

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Raspberry Melomel",
                        notes: "Tart, bright, aromatic. Use 3-4 lbs/gal. Backsweeten to 1.012-1.015.",
                        tips: "Seeds add tannin - strain or leave based on preference. Combines well with vanilla.",
                        food_pairings: "Lemon cheesecake, white chocolate mousse, duck breast with berry reduction, soft-ripened cheeses (Camembert, Brie), dark chocolate truffles, roasted lamb.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Blueberry Melomel",
                        notes: "Subtle fruit, deep color. Use 4-5 lbs/gal for strong flavor.",
                        tips: "Freeze berries first to break cell walls. Color can fade - add at bottling for best hue.",
                        food_pairings: "Lemon pound cake, blueberry pie, pork tenderloin, mild blue cheese, breakfast pastries, French toast, pancakes with syrup.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Blackberry Melomel",
                        notes: "Rich, complex, tannic. Use 3-4 lbs/gal. Needs 12+ months aging.",
                        tips: "High tannin content - balance with sweetness. Excellent with oak aging.",
                        food_pairings: "Dark chocolate desserts, game meats (venison, wild boar), aged cheddar, blackberry cobbler, grilled lamb chops, strong blue cheeses.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Strawberry Melomel",
                        notes: "Delicate, aromatic. Use 4-5 lbs/gal. Drink young (6-12 months).",
                        tips: "Flavor fades quickly - don't age long. Freeze-thaw cycle enhances juice extraction.",
                        food_pairings: "Strawberry shortcake, white chocolate, fresh berries with cream, mild chèvre, angel food cake, delicate pastries, brunch dishes.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Elderberry Melomel",
                        notes: "Deep, complex, wine-like. Use 2-3 lbs/gal. High tannin.",
                        tips: "MUST cook elderberries first (raw are toxic). Rich, aged character. Popular medieval style.",
                        food_pairings: "Wild game, roasted duck, aged hard cheeses, dark berry desserts, hearty stews, mushroom dishes, rustic European cuisine.",
                    });
                });

            // Stone Fruit Melomels subcategory
            egui::CollapsingHeader::new(RichText::new("  🍑 Stone Fruit Melomels")
                .size(18.0)
                .color(c.honey_gold))
                .default_open(false)
                .show(ui, |ui| {
                    ui.add_space(5.0);

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Peach Melomel",
                        notes: "Aromatic, sweet, summer fruit. Use 3-4 lbs/gal.",
                        tips: "Remove pits (cyanide compounds). Pairs well with vanilla, cinnamon. Drink young-ish.",
                        food_pairings: "Peach cobbler, vanilla ice cream, BBQ pulled pork, grilled chicken, bourbon-glazed ham, mascarpone, Southern cuisine, summer cookouts.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Cherry Melomel",
                        notes: "Tart (sour) or sweet (bing). Use 3-4 lbs/gal. Versatile.",
                        tips: "Tart cherries (Montmorency) most popular. Pits add almond notes if left in. Beautiful color.",
                        food_pairings: "Black Forest cake, duck à l'orange, dark chocolate, aged Gouda, cherry pie, duck breast with cherry sauce, pork chops, German cuisine.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Plum Melomel",
                        notes: "Rich, complex, varied by plum variety. Use 3-4 lbs/gal.",
                        tips: "Italian prune plums excellent. Can be sweet or tart depending on variety.",
                        food_pairings: "Roasted pork loin, plum tart, Asian five-spice duck, aged manchego, Chinese cuisine (Peking duck), caramelized onion dishes.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Apricot Melomel",
                        notes: "Delicate, aromatic, peachy. Use 3-4 lbs/gal.",
                        tips: "Flavor can be subtle - use generous amounts. Pairs with vanilla, almond.",
                        food_pairings: "Apricot tart, chicken tagine, Middle Eastern cuisine, almond desserts, mild curry dishes, Moroccan lamb, frangipane.",
                    });
                });

            // Tropical Fruit Melomels
            egui::CollapsingHeader::new(RichText::new("  🥭 Tropical Fruit Melomels")
                .size(18.0)
                .color(c.honey_gold))
                .default_open(false)
                .show(ui, |ui| {
                    ui.add_space(5.0);

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Mango Melomel",
                        notes: "Sweet, aromatic, tropical. Use 3-4 lbs/gal. Popular with peppers.",
                        tips: "Combines excellently with habanero for sweet heat. Freezes well for year-round brewing.",
                        food_pairings: "Thai curry, jerk chicken, mango sticky rice, coconut desserts, ceviche, fish tacos, Caribbean cuisine, spicy foods (heat balance).",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Pineapple Melomel",
                        notes: "Bright, acidic, tropical. Use juice or fresh (3-4 lbs/gal).",
                        tips: "High acidity - needs sweetness balance. Pairs with jalapeño, coconut. Ferments quickly.",
                        food_pairings: "Hawaiian pizza, teriyaki salmon, grilled pork with pineapple, coconut shrimp, sweet and sour dishes, island cuisine, poke bowls.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Passion Fruit Melomel",
                        notes: "Intensely aromatic, tart, exotic. Use 2-3 lbs/gal (potent).",
                        tips: "Very strong flavor - use less than other fruits. Seeds add bitterness if left too long.",
                        food_pairings: "Tropical fruit salad, key lime pie, grilled fish, ceviche, crème brûlée, delicate seafood, South American cuisine, exotic desserts.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Guava Melomel",
                        notes: "Tropical, aromatic, pink color. Use 3-4 lbs/gal.",
                        tips: "Pink or white guava both work. Strong aroma carries through aging.",
                        food_pairings: "Cuban sandwich, queso fresco, guava paste with cream cheese, Latin cuisine, flan, tres leches cake, empanadas.",
                    });
                });

            ui.add_space(15.0);

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
                food_pairings: "Cheese: Aged cheddar, smoked Gouda, sharp white cheddar, aged Gruyère.\nProtein: Roast pork loin, roasted turkey, baked ham, pork chops with apple chutney.\nDessert: Apple pie, apple crisp, caramel apple tart, cinnamon rolls.\nCuisine: Thanksgiving dinner, autumn harvest meals, German/Belgian cuisine.\nSpecialty: Perfect Thanksgiving pairing, cider donuts, roasted root vegetables.",
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
                food_pairings: "Cheese: Aged Parmigiano, Pecorino Romano, aged Manchego, Gruyère.\nProtein: Osso buco, braised lamb shanks, beef bourguignon, coq au vin.\nCuisine: Italian, French, Mediterranean fine dining.\nSpecialty: Wine-food pairings apply. Pasta with red sauce, risotto, beef dishes.\nOccasion: Elegant dinners, wine-replacement for special meals.",
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
                food_pairings: "Cheese: Aged cheddar, blue cheese, aged Gouda.\nProtein: Game birds, roasted duck, lamb.\nDessert: Berry tarts, dark chocolate desserts.\nCuisine: Medieval feasts, historical recreation dinners.\nSpecialty: Historically accurate pairings, rustic European fare.",
            });
        });

    ui.add_space(15.0);

    // CATEGORY 3: SPICED & HERBAL MEADS
    egui::CollapsingHeader::new(RichText::new("🌿 Spiced & Herbal Meads (Metheglins)")
        .size(22.0)
        .strong()
        .color(c.forest_green))
        .default_open(false)
        .show(ui, |ui| {
            ui.add_space(10.0);

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
                food_pairings: "Cheese: Varies by spice - generally aged hard cheeses, spiced cheese varieties.\nProtein: Spiced roasted meats, curry dishes, Moroccan tagines.\nDessert: Spice cakes, gingerbread, chai-spiced desserts.\nCuisine: Indian, Middle Eastern, North African, medieval European.\nSpecialty: Holiday meals, mulled mead-style warming drinks.",
            });

            // Warming Spice Metheglins
            egui::CollapsingHeader::new(RichText::new("  🔥 Warming Spice Metheglins")
                .size(18.0)
                .color(c.honey_gold))
                .default_open(false)
                .show(ui, |ui| {
                    ui.add_space(5.0);

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Cinnamon Metheglin",
                        notes: "Most popular spice. Warming, sweet aromatic. Use 2-4 sticks per gallon.",
                        tips: "Ceylon cinnamon more delicate than Cassia. Add in secondary. Pairs with vanilla, apple.",
                        food_pairings: "Apple pie, snickerdoodles, churros, Mexican hot chocolate, cinnamon rolls, pumpkin pie, sweet potato casserole, holiday desserts.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Ginger Metheglin",
                        notes: "Spicy, warming, slightly hot. Use 1-3 oz fresh per gallon.",
                        tips: "Fresh ginger stronger than dried. Can be very spicy - start light. Excellent carbonated.",
                        food_pairings: "Asian stir-fry, sushi, gingerbread, crystallized ginger, Thai cuisine, Indian curry (mild to medium), carrot cake, pumpkin dishes.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Vanilla Metheglin",
                        notes: "Sweet, aromatic, dessert-like. Use 1-2 beans per gallon.",
                        tips: "Add in secondary. Split beans lengthwise. Madagascar vanilla most popular. Combines with everything.",
                        food_pairings: "Crème brûlée, vanilla bean ice cream, pound cake, white chocolate, panna cotta, delicate seafood, cream-based sauces.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Clove Metheglin",
                        notes: "Potent, warming, medicinal if overdone. Use 5-10 whole cloves per gallon.",
                        tips: "VERY easy to overdo. Start with 5 cloves. Can add more, never remove. Medieval favorite.",
                        food_pairings: "Ham with clove glaze, spice cakes, mulled wine accompaniments, holiday roasts, gingerbread, medieval-spiced dishes.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Nutmeg Metheglin",
                        notes: "Warming, sweet spice. Use 1-2 whole nuts per gallon, grated.",
                        tips: "Fresh-grated nutmeg far superior to pre-ground. Pairs with cinnamon, vanilla.",
                        food_pairings: "Eggnog, custards, bread pudding, béchamel dishes, spinach and ricotta, creamy pasta, pumpkin pie, holiday baking.",
                    });
                });

            // Floral & Herbal Metheglins
            egui::CollapsingHeader::new(RichText::new("  🌸 Floral & Herbal Metheglins")
                .size(18.0)
                .color(c.honey_gold))
                .default_open(false)
                .show(ui, |ui| {
                    ui.add_space(5.0);

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Lavender Metheglin",
                        notes: "Floral, perfume-like, calming. Use 1-2 oz dried per 5 gal.",
                        tips: "Very easy to overdo - tastes like soap if too much. Culinary lavender only. Pairs with lemon.",
                        food_pairings: "Lemon shortbread, honey cookies, roasted chicken with herbs de Provence, goat cheese, crème brûlée, French cuisine, delicate fish.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Chamomile Metheglin",
                        notes: "Delicate, floral, slightly apple-like. Use 2-4 oz per 5 gal.",
                        tips: "Roman or German chamomile both work. Calming properties. Makes excellent session mead.",
                        food_pairings: "Light salads, delicate white fish, lemon desserts, shortbread, mild cheeses, breakfast pastries, afternoon tea accompaniments.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Elderflower Metheglin",
                        notes: "Delicate, floral, slightly muscat-like. Use 3-5 oz dried per 5 gal.",
                        tips: "Very delicate flavor. Pairs excellently with lemon, grapefruit. Popular in UK.",
                        food_pairings: "Lemon curd tarts, gooseberry fool, British desserts, delicate seafood, cucumber sandwiches, asparagus, spring vegetables.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Hibiscus Metheglin",
                        notes: "Tart, cranberry-like, bright red color. Use 2-4 oz per 5 gal.",
                        tips: "Adds beautiful red color and tartness. Often combined with fruit. Very popular lately.",
                        food_pairings: "Ceviche, fish tacos, cranberry sauce, turkey, tart desserts, tropical fruit salads, light Mexican cuisine, summer salads.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Mint Metheglin",
                        notes: "Cooling, refreshing, herbal. Use 2-4 oz dried per 5 gal.",
                        tips: "Peppermint or spearmint. Best served cold or carbonated. Pairs with chocolate, lime.",
                        food_pairings: "Lamb with mint jelly, chocolate mint desserts, Middle Eastern lamb, tabbouleh, Greek salads, Mediterranean cuisine, after-dinner digestif.",
                    });
                });

            // Tea Metheglins
            egui::CollapsingHeader::new(RichText::new("  🍵 Tea Metheglins")
                .size(18.0)
                .color(c.honey_gold))
                .default_open(false)
                .show(ui, |ui| {
                    ui.add_space(5.0);

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Earl Grey Metheglin",
                        notes: "Bergamot-forward, sophisticated. Use 5-10 tea bags per gallon.",
                        tips: "Black tea base with bergamot oil. Pairs excellently with lemon. Increasingly popular.",
                        food_pairings: "Scones with clotted cream, lemon curd, shortbread, cucumber sandwiches, smoked salmon, afternoon tea service, British fare.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Green Tea Metheglin",
                        notes: "Delicate, grassy, antioxidant-rich. Use 5-10 tea bags per gallon.",
                        tips: "Jasmine green tea popular variant. Light, refreshing. Don't over-steep (bitter).",
                        food_pairings: "Sushi, sashimi, edamame, light Asian cuisine, delicate fish, steamed vegetables, Japanese desserts, mochi.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Chai Metheglin",
                        notes: "Complex spice blend. Cinnamon, cardamom, ginger, clove.",
                        tips: "Use chai tea bags or make your own spice blend. Warming, complex. Excellent with milk (braggot variant).",
                        food_pairings: "Indian cuisine, samosas, naan bread, butter chicken, tikka masala, spiced cookies, carrot cake, pumpkin desserts.",
                    });
                });

            ui.add_space(15.0);

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
                food_pairings: "Cheese: Fresh chèvre, feta, mascarpone.\nProtein: Delicate white fish, scallops, lobster.\nDessert: Rose water Turkish delight, Persian love cake, macarons, rosewater panna cotta.\nCuisine: Middle Eastern, Persian, romantic dinners.\nSpecialty: Wedding toasts, Valentine's Day, rose-flavored desserts.",
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
                food_pairings: "Cheese: Pepper jack, aged cheddar, queso fresco.\nProtein: Spicy wings, jerk chicken, BBQ ribs, spicy shrimp, fajitas.\nCuisine: Mexican, Caribbean, Southwestern, BBQ, spicy Asian.\nSpecialty: Heat balance - sweetness counters spice. Excellent with hot sauces.\nPairing principle: Sweet-heat balance for spicy foods.",
            });
        });

    ui.add_space(15.0);

    // CATEGORY 4: SPECIALTY & HYBRID MEADS
    egui::CollapsingHeader::new(RichText::new("⚡ Specialty & Hybrid Meads")
        .size(22.0)
        .strong()
        .color(c.forest_green))
        .default_open(false)
        .show(ui, |ui| {
            ui.add_space(10.0);

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
                food_pairings: "Cheese: Beer cheese, aged cheddar, smoked Gouda.\nProtein: Burgers, sausages, grilled meats, pub fare.\nFood: Fish and chips, shepherd's pie, bangers and mash, hearty stews.\nCuisine: British pub food, German beer hall fare, BBQ.\nSpecialty: Pretzels, beer brats, comfort food, casual dining.",
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
                food_pairings: "Cheese: Aged cheddar, smoked Gouda, Oka.\nProtein: Maple-glazed salmon, bacon, pork belly, duck breast.\nDessert: Maple walnut pie, maple fudge, butter tarts, pancakes.\nCuisine: Canadian, New England, autumn harvest meals.\nSpecialty: Breakfast foods, brunch, French toast, waffles, pecan pie.",
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
                food_pairings: "Cheese: Aged Gouda, Comté, aged cheddar, blue cheese.\nDessert: Caramel flan, crème brûlée, toffee pudding, butterscotch desserts, praline.\nProtein: Braised short ribs, caramelized pork belly, duck confit.\nSpecialty: Salted caramels, toffee, dulce de leche, bananas foster.\nOccasion: Dessert pairing, after-dinner sipping.",
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
                food_pairings: "Cheese: Aged Manchego, Parmigiano-Reggiano, spiced cheese.\nProtein: Roasted game, lamb, beef tenderloin, duck.\nCuisine: Medieval feasts, Renaissance faires, European holiday meals.\nSpecialty: Mulled wine companion dishes, roasted chestnuts, holiday ham.\nOccasion: Christmas dinner, winter solstice, historical reenactments.",
            });

            // Other Specialty Meads
            egui::CollapsingHeader::new(RichText::new("  🎯 Other Specialty Meads")
                .size(18.0)
                .color(c.honey_gold))
                .default_open(false)
                .show(ui, |ui| {
                    ui.add_space(5.0);

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Oxymel (Vinegar Mead)",
                        notes: "Ancient medicinal mead. Honey + vinegar + herbs. Historical tonic.",
                        tips: "Add vinegar after fermentation. Herbal additions common. Used as digestif or health tonic.",
                        food_pairings: "Digestif after heavy meals, grilled vegetables, pickled foods, salads with vinaigrette, palate cleanser.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Chouchenn (Breton Mead)",
                        notes: "Traditional French mead. Dry, sparkling, champagne-method.",
                        tips: "Brittany regional specialty. Light, dry, effervescent. Similar to hydromel but traditional method.",
                        food_pairings: "Oysters, mussels, crêpes, Breton galettes, seafood, French coastal cuisine.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Tej (Ethiopian Honey Wine)",
                        notes: "African honey wine with gesho (buckthorn). Smoky, hoppy, unique.",
                        tips: "Traditional Ethiopian drink. Uses gesho for bittering (hop-like). Often cloudy, rustic.",
                        food_pairings: "Ethiopian injera with wat (stews), doro wat, kitfo, African cuisine, spiced lentils.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Medovina (Slavic Mead)",
                        notes: "Eastern European mead. Often spiced, can be hot or cold.",
                        tips: "Czech, Slovak, Polish traditions. Varies by region. Often served warm in winter.",
                        food_pairings: "Pierogi, kielbasa, Eastern European stews, roasted meats, hearty rye bread.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Short Mead",
                        notes: "Quick-turnaround traditional. Ready in 2-4 weeks. Ale yeast.",
                        tips: "Lower gravity (1.050-1.070). Ale yeast for quick ferment. Drink young and fresh.",
                        food_pairings: "Casual pub fare, lighter meals, appetizers, session drinking with various foods.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Dwójniak (Polish Mead)",
                        notes: "Two-parts mead: 1 part honey, 1 part water. Medium strength.",
                        tips: "Traditional Polish style. Moderate gravity. Between półtorak and trójniak in strength.",
                        food_pairings: "Polish sausage, rye bread, pickled vegetables, bigos (hunter's stew), Easter ham.",
                    });
                });
        });

    ui.add_space(15.0);

    // CATEGORY 5: ADVANCED & EXPERIMENTAL
    egui::CollapsingHeader::new(RichText::new("🔬 Advanced & Experimental Styles")
        .size(22.0)
        .strong()
        .color(c.forest_green))
        .default_open(false)
        .show(ui, |ui| {
            ui.add_space(10.0);

            // Barrel-Aged Meads
            egui::CollapsingHeader::new(RichText::new("  🛢️ Barrel-Aged & Oak-Aged Meads")
                .size(18.0)
                .color(c.honey_gold))
                .default_open(false)
                .show(ui, |ui| {
                    ui.add_space(5.0);

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Bourbon Barrel Traditional",
                        notes: "Aged in bourbon barrels. Vanilla, oak, caramel notes.",
                        tips: "Use spent bourbon barrels or spirals. 3-12 months oak contact. Adds complexity, color.",
                        food_pairings: "Bourbon-glazed salmon, smoked brisket, aged steaks, dark chocolate, bourbon balls, pecan pie, BBQ.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Wine Barrel Pyment",
                        notes: "Pyment aged in wine barrels. Enhanced wine character.",
                        tips: "Red or white wine barrels. Adds tannin, oxidative notes. Expensive but worth it.",
                        food_pairings: "As pyment but elevated - osso buco, coq au vin, beef Wellington, aged Parmigiano-Reggiano.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Oak-Aged Bochet",
                        notes: "Bochet with oak aging. Amplifies caramel complexity.",
                        tips: "Medium or heavy toast oak. 2-6 months. Complements caramelization perfectly.",
                        food_pairings: "Crème brûlée, toffee pudding, aged cheeses, braised meats, rich desserts.",
                    });
                });

            // Sour/Funky Meads
            egui::CollapsingHeader::new(RichText::new("  🦠 Sour & Funky Meads")
                .size(18.0)
                .color(c.honey_gold))
                .default_open(false)
                .show(ui, |ui| {
                    ui.add_space(5.0);

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Brett Mead",
                        notes: "Brettanomyces fermentation. Funky, earthy, complex.",
                        tips: "Use Brett yeast or mixed culture. Long fermentation (6-12 months). Develops barnyard, tropical notes.",
                        food_pairings: "Farmhouse cheeses, aged hard cheeses, charcuterie, funky washed-rind cheeses (Époisses), wild game.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Lactomel (Sour Mead)",
                        notes: "Lactobacillus souring. Tart, clean acidity.",
                        tips: "Pre-acidify with lactobacillus. Combines well with fruit. Refreshing, tart character.",
                        food_pairings: "Ceviche, oysters, tartare, goat cheese, salads, palate cleanser, tart fruit desserts.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Wild Fermented Mead",
                        notes: "Spontaneous fermentation. Terroir-driven, unpredictable.",
                        tips: "No added yeast. Local microflora. Risky but rewarding. Long fermentation.",
                        food_pairings: "Artisanal cheeses, wild game, foraged mushrooms, rustic bread, farmhouse cuisine.",
                    });
                });

            // Carbonated/Sparkling Meads
            egui::CollapsingHeader::new(RichText::new("  🫧 Carbonated & Sparkling Meads")
                .size(18.0)
                .color(c.honey_gold))
                .default_open(false)
                .show(ui, |ui| {
                    ui.add_space(5.0);

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Sparkling Traditional",
                        notes: "Champagne-method traditional mead. Elegant bubbles.",
                        tips: "Bottle condition with priming sugar. Use champagne yeast. Needs champagne bottles. 3-3.5 volumes CO₂.",
                        food_pairings: "Champagne pairings apply - oysters, caviar, lobster, delicate fish, celebration foods.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Carbonated Cyser",
                        notes: "Sparkling apple-honey. Like hard cider meets mead.",
                        tips: "Very popular style. 2.5-3 volumes CO₂. Refreshing, approachable. Great gateway mead.",
                        food_pairings: "Fried chicken, fish and chips, apple-based appetizers, casual foods, brunch.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Sparkling Melomel",
                        notes: "Carbonated fruit mead. Bright, refreshing.",
                        tips: "Berry melomels work great. Citrus excellent. Tropical fruits popular. Session-strength best.",
                        food_pairings: "Mimosa-style brunch, fruit-based appetizers, light salads, celebration cakes.",
                    });
                });

            // Coffee & Chocolate Meads
            egui::CollapsingHeader::new(RichText::new("  ☕ Coffee & Chocolate Meads")
                .size(18.0)
                .color(c.honey_gold))
                .default_open(false)
                .show(ui, |ui| {
                    ui.add_space(5.0);

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Coffee Mead",
                        notes: "Cold brew coffee addition. Roasty, rich.",
                        tips: "Add cold brew in secondary. 8-16 oz per 5 gal. Pairs with vanilla, chocolate, oak.",
                        food_pairings: "Tiramisu, coffee cake, chocolate espresso desserts, crème brûlée, biscotti, morning pastries.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Chocolate Mead",
                        notes: "Cacao nibs or chocolate. Rich, dessert-like.",
                        tips: "Use cacao nibs (2-8 oz per 5 gal) in secondary. Pairs with vanilla, coffee, peppers, cherry.",
                        food_pairings: "Chocolate lava cake, triple chocolate brownies, chocolate mousse, mole dishes, chocolate-covered strawberries.",
                    });

                    render_substyle(ui, c, SubStyleInfo {
                        name: "Mocha Mead",
                        notes: "Coffee + chocolate combination. Decadent.",
                        tips: "Combine coffee and cacao nibs. Finish sweet. Excellent with vanilla, oak. Dessert mead.",
                        food_pairings: "Mocha torte, chocolate coffee desserts, tiramisu, dark chocolate truffles, decadent desserts.",
                    });
                });
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
    food_pairings: &'static str,
}

struct SubStyleInfo {
    name: &'static str,
    notes: &'static str,
    tips: &'static str,
    food_pairings: &'static str,
}

fn render_style(ui: &mut egui::Ui, c: &crate::state::CustomColors, style: StyleInfo) {
    egui::Frame::default()
        .fill(Color32::WHITE)
        .stroke(egui::Stroke::new(2.0, c.honey_gold))
        .corner_radius(egui::CornerRadius::same(8))
        .inner_margin(15.0)
        .show(ui, |ui| {
            ui.label(RichText::new(style.name)
                .size(20.0)
                .strong()
                .color(c.forest_green));
            ui.add_space(8.0);

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
            ui.label(RichText::new(style.description).size(14.0).color(c.dark_text));
            ui.add_space(10.0);

            ui.label(RichText::new("Characteristics:")
                .size(14.0)
                .strong()
                .color(c.forest_green));
            for line in style.characteristics.lines() {
                ui.label(RichText::new(line).size(13.0).color(c.dark_text));
            }

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("⏱ Aging:").size(14.0).strong().color(c.sunset_orange));
                ui.label(RichText::new(style.aging).size(13.0).color(c.dark_text));
            });

            ui.add_space(10.0);
            ui.label(RichText::new("💡 Brewing Tips:")
                .size(14.0)
                .strong()
                .color(c.honey_gold));
            ui.add_space(5.0);
            for line in style.tips.lines() {
                ui.label(RichText::new(line).size(13.0).color(c.dark_text));
            }

            ui.add_space(10.0);
            ui.label(RichText::new("🍽️ Food Pairings:")
                .size(14.0)
                .strong()
                .color(c.sunset_orange));
            ui.add_space(5.0);
            for line in style.food_pairings.lines() {
                ui.label(RichText::new(line).size(13.0).color(c.dark_text));
            }
        });

    ui.add_space(15.0);
}

fn render_substyle(ui: &mut egui::Ui, c: &crate::state::CustomColors, substyle: SubStyleInfo) {
    egui::Frame::default()
        .fill(c.light_cream)
        .stroke(egui::Stroke::new(1.0, c.honey_gold))
        .corner_radius(egui::CornerRadius::same(5))
        .inner_margin(10.0)
        .show(ui, |ui| {
            ui.label(RichText::new(substyle.name)
                .size(16.0)
                .strong()
                .color(c.forest_green));
            ui.add_space(5.0);
            ui.label(RichText::new(substyle.notes).size(13.0).color(c.dark_text));
            ui.add_space(5.0);
            ui.label(RichText::new(format!("💡 {}", substyle.tips))
                .size(12.0)
                .color(c.dark_text));
            ui.add_space(5.0);
            ui.label(RichText::new(format!("🍽️ {}", substyle.food_pairings))
                .size(12.0)
                .color(c.sunset_orange));
        });
    ui.add_space(8.0);
}