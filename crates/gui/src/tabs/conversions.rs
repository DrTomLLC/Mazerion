//! Conversion Reference Guide - COMPLETE AND CORRECT
//! Professional reference tables for all unit conversions

use eframe::egui::{self, RichText, Color32, Stroke, Rounding};

const BG_PANEL: Color32 = Color32::from_rgb(250, 248, 240);
const BORDER: Color32 = Color32::from_rgb(184, 134, 11);
const TEXT_ACCENT: Color32 = Color32::from_rgb(139, 69, 19);

pub fn render(ui: &mut egui::Ui) {
    ui.heading(RichText::new("📏 Unit Conversion Reference").size(20.0).color(TEXT_ACCENT));
    ui.label("Quick reference for brewing and fermentation conversions");
    ui.add_space(15.0);

    // Volume and Weight conversions
    ui.columns(2, |columns| {
        section(&mut columns[0], "💧 Volume Conversions", |ui| {
            table(ui, &[
                ("1 Liter (L)", "0.2642 Gallons (US)"),
                ("1 Liter", "1.0567 Quarts (US)"),
                ("1 Liter", "2.1134 Pints (US)"),
                ("1 Liter", "4.2268 Cups (US)"),
                ("1 Liter", "33.814 Fluid Ounces (US)"),
                ("1 Liter", "1000 Milliliters (mL)"),
                ("", ""),
                ("1 Gallon (US)", "3.7854 Liters"),
                ("1 Gallon", "4 Quarts"),
                ("1 Gallon", "8 Pints"),
                ("1 Gallon", "16 Cups"),
                ("1 Gallon", "128 Fluid Ounces"),
                ("", ""),
                ("1 Quart (US)", "0.9464 Liters"),
                ("1 Pint (US)", "0.4732 Liters"),
                ("1 Cup (US)", "236.6 mL"),
                ("1 Fl Oz (US)", "29.574 mL"),
            ]);
        });

        section(&mut columns[1], "⚖️ Weight/Mass Conversions", |ui| {
            table(ui, &[
                ("1 Kilogram (kg)", "2.2046 Pounds (lb)"),
                ("1 Kilogram", "35.274 Ounces (oz)"),
                ("1 Kilogram", "1000 Grams (g)"),
                ("", ""),
                ("1 Pound (lb)", "0.4536 Kilograms"),
                ("1 Pound", "16 Ounces (oz)"),
                ("1 Pound", "453.6 Grams (g)"),
                ("", ""),
                ("1 Ounce (oz)", "28.35 Grams"),
                ("1 Gram (g)", "0.0353 Ounces"),
                ("", ""),
                ("1 Metric Ton", "1000 Kilograms"),
                ("1 US Ton", "2000 Pounds"),
                ("1 US Ton", "907.2 Kilograms"),
            ]);
        });
    });

    ui.add_space(10.0);

    // Temperature and Gravity conversions
    ui.columns(2, |columns| {
        section(&mut columns[0], "🌡️ Temperature Conversions", |ui| {
            table(ui, &[
                ("Formula", "Result"),
                ("°C → °F", "°F = (°C × 9/5) + 32"),
                ("°F → °C", "°C = (°F - 32) × 5/9"),
                ("°C → K", "K = °C + 273.15"),
                ("", ""),
                ("Common Temps:", ""),
                ("0°C", "32°F (Water freezes)"),
                ("20°C", "68°F (Room temp)"),
                ("100°C", "212°F (Water boils)"),
                ("", ""),
                ("Brewing Temps:", ""),
                ("10-15°C", "50-59°F (Lager)"),
                ("18-22°C", "64-72°F (Ale)"),
                ("22-30°C", "72-86°F (Wine/Mead)"),
            ]);
        });

        section(&mut columns[1], "📊 Gravity & Sugar Conversions", |ui| {
            table(ui, &[
                ("Specific Gravity", "Brix / Plato"),
                ("1.000 SG", "0.0° Bx"),
                ("1.020 SG", "~5.1° Bx"),
                ("1.040 SG", "~10.0° Bx"),
                ("1.060 SG", "~14.7° Bx"),
                ("1.080 SG", "~19.3° Bx"),
                ("1.100 SG", "~23.7° Bx"),
                ("1.120 SG", "~28.0° Bx"),
                ("", ""),
                ("Formula (approx):", ""),
                ("Brix → SG", "SG ≈ 1 + (Bx × 0.004)"),
                ("SG → Brix", "Bx ≈ (SG - 1) × 250"),
                ("", ""),
                ("Note:", "Brix ≈ Plato"),
            ]);
        });
    });

    ui.add_space(10.0);

    // Brewing-specific conversions
    section(ui, "🍺 Brewing-Specific Conversions", |ui| {
        ui.columns(2, |columns| {
            columns[0].group(|ui| {
                ui.label(RichText::new("Common Batch Sizes:").strong());
                ui.label("5 gallons (US) = 18.93 L");
                ui.label("6 gallons (US) = 22.71 L");
                ui.label("10 gallons (US) = 37.85 L");
                ui.label("20 gallons (US) = 75.71 L");
                ui.label("1 barrel (US) = 31 gallons = 117.3 L");
                ui.add_space(8.0);
                ui.label(RichText::new("Extract/Grain:").strong());
                ui.label("1 lb DME ≈ 1.22 lb LME");
                ui.label("1 lb extract ≈ 0.7 lb grain");
                ui.label("Efficiency: typically 70-80%");
            });

            columns[1].group(|ui| {
                ui.label(RichText::new("Sugar/Honey Conversions:").strong());
                ui.label("1 cup honey ≈ 340g ≈ 12 oz");
                ui.label("1 cup sugar ≈ 200g ≈ 7 oz");
                ui.label("1 tbsp honey ≈ 21g");
                ui.label("1 tsp honey ≈ 7g");
                ui.label("Honey density ≈ 1.42 g/mL");
                ui.add_space(8.0);
                ui.label(RichText::new("ABV Approximation:").strong());
                ui.label("ABV ≈ (OG - FG) × 131.25");
                ui.label("Each 0.001 SG ≈ 0.13% ABV");
            });
        });
    });

    ui.add_space(10.0);

    // Carbonation reference
    section(ui, "🫧 Carbonation Reference", |ui| {
        ui.columns(2, |columns| {
            columns[0].group(|ui| {
                ui.label(RichText::new("CO₂ Volumes by Style:").strong());
                ui.label("English Ale: 1.5-2.0 volumes");
                ui.label("American Ale: 2.2-2.7 volumes");
                ui.label("Lager: 2.5-2.8 volumes");
                ui.label("Wheat Beer: 2.8-4.5 volumes");
                ui.label("Mead/Wine: 0.0-2.5 volumes");
                ui.label("Cider: 2.5-4.0 volumes");
            });

            columns[1].group(|ui| {
                ui.label(RichText::new("Priming Sugar (5 gal):").strong());
                ui.label("Low (1.5 vol): 2.0 oz corn sugar");
                ui.label("Med (2.5 vol): 3.8 oz corn sugar");
                ui.label("High (3.5 vol): 5.3 oz corn sugar");
                ui.add_space(5.0);
                ui.label(RichText::new("Keg PSI at 38°F:").strong());
                ui.label("2.0 volumes ≈ 7 PSI");
                ui.label("2.5 volumes ≈ 10 PSI");
                ui.label("3.0 volumes ≈ 13 PSI");
            });
        });
    });

    ui.add_space(10.0);

    // Mead/Wine specific
    section(ui, "🍯 Mead & Wine Conversions", |ui| {
        ui.columns(2, |columns| {
            columns[0].group(|ui| {
                ui.label(RichText::new("Honey to Water Ratios:").strong());
                ui.label("Hydromel: ~1.5 lb/gal (0.18 kg/L)");
                ui.label("Standard: ~3.0 lb/gal (0.36 kg/L)");
                ui.label("Sack: ~5.0 lb/gal (0.60 kg/L)");
                ui.add_space(5.0);
                ui.label(RichText::new("Honey Properties:").strong());
                ui.label("~82% fermentable sugars");
                ui.label("~35 gravity points/lb/gal");
                ui.label("pH typically 3.5-4.5");
            });

            columns[1].group(|ui| {
                ui.label(RichText::new("Common ABV Targets:").strong());
                ui.label("Hydromel (Session): 3.5-7.5%");
                ui.label("Standard Mead: 8-14%");
                ui.label("Sack Mead: 14-18%+");
                ui.add_space(5.0);
                ui.label(RichText::new("Aging Guidelines:").strong());
                ui.label("Dry mead: 6-12 months");
                ui.label("Sweet mead: 12-24 months");
                ui.label("Sack mead: 24-60 months");
            });
        });
    });

    ui.add_space(10.0);

    // Chemical additives
    section(ui, "🧪 Chemical Additions Reference", |ui| {
        ui.columns(2, |columns| {
            columns[0].group(|ui| {
                ui.label(RichText::new("Stabilization (per gal):").strong());
                ui.label("K-meta: 0.5 g/L (1.9 g/gal)");
                ui.label("K-sorbate: 0.75 g/L (2.8 g/gal)");
                ui.add_space(5.0);
                ui.label(RichText::new("Acid Additions (g/L per 0.1 pH):").strong());
                ui.label("Tartaric: 0.15 g/L (strongest)");
                ui.label("Citric: 0.17 g/L");
                ui.label("Malic: 0.19 g/L");
                ui.label("Lactic: 0.22 g/L (weakest)");
            });

            columns[1].group(|ui| {
                ui.label(RichText::new("Nutrient Dosing:").strong());
                ui.label("Fermaid-O: 25-40 g/hL");
                ui.label("DAP: 50-200 ppm nitrogen");
                ui.add_space(5.0);
                ui.label(RichText::new("Enzyme Usage:").strong());
                ui.label("Pectic enzyme: 1-2 tsp/5 gal");
                ui.label("Amylase: per package directions");
                ui.add_space(5.0);
                ui.label(RichText::new("Tannin:").strong());
                ui.label("FT Rouge: 0.5-2 g/L");
            });
        });
    });
}

fn section(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(BG_PANEL)
        .stroke(Stroke::new(1.5, BORDER))
        .rounding(Rounding::same(8.0 as u8))
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.label(RichText::new(title).color(TEXT_ACCENT).size(16.0).strong());
            ui.add_space(6.0);
            content(ui);
        });
}

fn table(ui: &mut egui::Ui, rows: &[(&str, &str)]) {
    for (left, right) in rows {
        if left.is_empty() {
            ui.add_space(4.0);
        } else {
            ui.horizontal(|ui| {
                ui.label(RichText::new(*left).size(12.0));
                ui.label(RichText::new("→").weak());
                ui.label(RichText::new(*right).size(12.0).color(Color32::from_rgb(34, 139, 34)));
            });
        }
    }
}