//! Mazerion GUI - BEAUTIFUL & COMPLETE with Settings

mod state;
mod tabs;

use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};
use state::{AppState, TabView};

// SOFT, EASY ON THE EYES COLOR SCHEME
const BG_MAIN: Color32 = Color32::from_rgb(225, 235, 245);       // Soft baby blue
const BG_PANEL: Color32 = Color32::from_rgb(245, 250, 255);      // Very light blue-white
const TEXT_MAIN: Color32 = Color32::from_rgb(30, 30, 30);        // Dark text
const TEXT_SUCCESS: Color32 = Color32::from_rgb(34, 139, 34);    // Green
const TEXT_ERROR: Color32 = Color32::from_rgb(220, 20, 60);      // Red
const TEXT_WARNING: Color32 = Color32::from_rgb(255, 140, 0);    // Orange
const ACCENT: Color32 = Color32::from_rgb(70, 130, 180);         // Steel blue
const TAB_ACTIVE: Color32 = Color32::from_rgb(70, 130, 180);     // Steel blue
const TAB_INACTIVE: Color32 = Color32::from_rgb(200, 220, 240);  // Light blue
const SCROLLBAR: Color32 = Color32::from_rgb(100, 149, 237);     // Cornflower blue (visible!)

pub struct MazerionApp {
    state: AppState,
}

impl Default for MazerionApp {
    fn default() -> Self {
        Self {
            state: AppState::default(),
        }
    }
}

impl eframe::App for MazerionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply beautiful custom style
        let mut style = (*ctx.style()).clone();

        // Main colors
        style.visuals.window_fill = BG_MAIN;
        style.visuals.panel_fill = BG_MAIN;
        style.visuals.extreme_bg_color = BG_PANEL;

        // Widget backgrounds
        style.visuals.widgets.noninteractive.weak_bg_fill = BG_PANEL;
        style.visuals.widgets.inactive.weak_bg_fill = BG_PANEL;
        style.visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(210, 225, 240);
        style.visuals.widgets.active.weak_bg_fill = ACCENT;

        // Scrollbar - HIGHLY VISIBLE!
        style.visuals.widgets.inactive.bg_fill = SCROLLBAR;
        style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(65, 105, 225); // Royal blue
        style.visuals.widgets.active.bg_fill = Color32::from_rgb(30, 75, 180);   // Dark blue

        // Scrollbar handle - THICK AND VISIBLE
        style.visuals.widgets.inactive.fg_stroke = Stroke::new(12.0, SCROLLBAR);
        style.visuals.widgets.hovered.fg_stroke = Stroke::new(14.0, Color32::from_rgb(65, 105, 225));
        style.visuals.widgets.active.fg_stroke = Stroke::new(14.0, Color32::from_rgb(30, 75, 180));

        // Scrollbar background
        style.visuals.extreme_bg_color = Color32::from_rgb(200, 215, 235);

        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            ui.vertical_centered(|ui| {
                ui.heading(
                    RichText::new("🍯 Mazerion")
                        .size(32.0)
                        .color(TEXT_MAIN)
                        .strong(),
                );
                ui.label(
                    RichText::new("Professional Beverage Calculator Suite")
                        .size(16.0)
                        .color(ACCENT),
                );
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // Tab selection
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 5.0;

                if self.tab_button(ui, "📊 Basic", self.state.current_tab == TabView::Basic).clicked() {
                    self.state.current_tab = TabView::Basic;
                    self.state.clear_results();
                }

                if self.tab_button(ui, "🔬 Advanced", self.state.current_tab == TabView::Advanced).clicked() {
                    self.state.current_tab = TabView::Advanced;
                    self.state.clear_results();
                }

                if self.tab_button(ui, "🍺 Brewing", self.state.current_tab == TabView::Brewing).clicked() {
                    self.state.current_tab = TabView::Brewing;
                    self.state.clear_results();
                }

                if self.tab_button(ui, "✨ Finishing", self.state.current_tab == TabView::Finishing).clicked() {
                    self.state.current_tab = TabView::Finishing;
                    self.state.clear_results();
                }

                if self.tab_button(ui, "⚙️ Settings", self.state.current_tab == TabView::Settings).clicked() {
                    self.state.current_tab = TabView::Settings;
                    self.state.clear_results();
                }
            });

            ui.add_space(10.0);

            // Render selected tab
            egui::ScrollArea::vertical()
                .max_height(f32::INFINITY)
                .show(ui, |ui| {
                    match self.state.current_tab {
                        TabView::Basic => tabs::render_basic(ui, &mut self.state),
                        TabView::Advanced => tabs::render_advanced(ui, &mut self.state),
                        TabView::Brewing => tabs::render_brewing(ui, &mut self.state),
                        TabView::Finishing => tabs::render_finishing(ui, &mut self.state),
                        TabView::Settings => tabs::render_settings(ui, &mut self.state),
                    }

                    ui.add_space(10.0);

                    // Results section
                    if self.state.result.is_some() || !self.state.warnings.is_empty() {
                        ui.separator();
                        ui.add_space(5.0);
                        self.render_results(ui);
                    }
                });
        });
    }
}

impl MazerionApp {
    fn tab_button(&self, ui: &mut egui::Ui, text: &str, active: bool) -> egui::Response {
        let (fill, text_color) = if active {
            (TAB_ACTIVE, Color32::WHITE)
        } else {
            (TAB_INACTIVE, TEXT_MAIN)
        };

        let button = egui::Button::new(RichText::new(text).color(text_color).size(14.0).strong())
            .fill(fill)
            .rounding(Rounding::same(5.0))
            .min_size(Vec2::new(110.0, 35.0));

        ui.add(button)
    }

    fn render_results(&self, ui: &mut egui::Ui) {
        egui::Frame::none()
            .fill(BG_PANEL)
            .stroke(Stroke::new(2.0, ACCENT))
            .rounding(Rounding::same(10.0))
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.heading(RichText::new("📋 Results").color(TEXT_MAIN));
                ui.add_space(5.0);

                // Main result
                if let Some(ref result) = self.state.result {
                    let (color, icon) = if result.starts_with('✓') {
                        (TEXT_SUCCESS, "")
                    } else if result.starts_with('❌') {
                        (TEXT_ERROR, "")
                    } else {
                        (TEXT_MAIN, "")
                    };

                    ui.label(
                        RichText::new(format!("{}{}", icon, result))
                            .size(18.0)
                            .color(color)
                            .strong(),
                    );
                }

                // Warnings (only if enabled in settings)
                if self.state.show_warnings && !self.state.warnings.is_empty() {
                    ui.add_space(8.0);
                    for warning in &self.state.warnings {
                        ui.label(
                            RichText::new(format!("⚠️ {}", warning))
                                .size(14.0)
                                .color(TEXT_WARNING),
                        );
                    }
                }

                // Metadata (only if enabled in settings)
                if self.state.show_metadata && !self.state.metadata.is_empty() {
                    ui.add_space(8.0);
                    ui.collapsing("ℹ️ Additional Information", |ui| {
                        for (key, value) in &self.state.metadata {
                            ui.label(format!("  • {}: {}", key, value));
                        }
                    });
                }
            });
    }
}

pub fn run() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Mazerion - Professional Beverage Calculator",
        options,
        Box::new(|_cc| Ok(Box::new(MazerionApp::default()))),
    )
}