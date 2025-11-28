//! Modern, beautiful GUI with Settings that WORK and Conversions tab

mod state;
mod tabs;
mod unit_helpers;

use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};
use state::{colors, AppState, TabView};

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
        // Apply theme and font size settings
        self.apply_settings(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            self.render_header(ui);
            ui.add_space(10.0);

            // Tab selection
            self.render_tabs(ui);
            ui.add_space(10.0);

            // Tab content with scroll
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    match self.state.current_tab {
                        TabView::Basic => tabs::render_basic(ui, &mut self.state),
                        TabView::Advanced => tabs::render_advanced(ui, &mut self.state),
                        TabView::Brewing => tabs::render_brewing(ui, &mut self.state),
                        TabView::Finishing => tabs::render_finishing(ui, &mut self.state),
                        TabView::Conversions => tabs::render_conversions(ui, &mut self.state),
                        TabView::Settings => tabs::render_settings(ui, &mut self.state),
                    }

                    ui.add_space(10.0);

                    // Results section
                    self.render_results(ui);
                });
        });
    }
}

impl MazerionApp {
    fn apply_settings(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();

        // Apply theme
        let (bg_main, bg_panel, accent, tab_active) = match self.state.theme.as_str() {
            "light" => (
                colors::LIGHT_BG_MAIN,
                colors::LIGHT_BG_PANEL,
                colors::LIGHT_ACCENT,
                colors::LIGHT_ACCENT,
            ),
            "cream" => (
                colors::CREAM_BG_MAIN,
                colors::CREAM_BG_PANEL,
                colors::CREAM_ACCENT,
                colors::CREAM_ACCENT,
            ),
            _ => (
                // "soft_blue" or default
                colors::BG_MAIN,
                colors::BG_PANEL,
                colors::ACCENT,
                colors::TAB_ACTIVE,
            ),
        };

        style.visuals.window_fill = bg_main;
        style.visuals.panel_fill = bg_main;
        style.visuals.extreme_bg_color = bg_panel;
        style.visuals.widgets.noninteractive.bg_fill = bg_panel;
        style.visuals.widgets.inactive.bg_fill = bg_panel;
        style.visuals.widgets.hovered.bg_fill = accent;
        style.visuals.widgets.active.bg_fill = tab_active;

        // HIGHLY VISIBLE SCROLLBAR
        style.visuals.widgets.inactive.fg_stroke = Stroke::new(12.0, colors::SCROLLBAR);
        style.visuals.widgets.hovered.fg_stroke =
            Stroke::new(14.0, Color32::from_rgb(65, 105, 225));
        style.visuals.widgets.active.fg_stroke =
            Stroke::new(14.0, Color32::from_rgb(30, 75, 180));

        // Apply font size safely (no unwrap)
        let text_size = match self.state.font_size.as_str() {
            "small" => 12.0,
            "large" => 16.0,
            _ => 14.0, // medium
        };

        if let Some(body) = style.text_styles.get_mut(&egui::TextStyle::Body) {
            body.size = text_size;
        }

        if let Some(button) = style.text_styles.get_mut(&egui::TextStyle::Button) {
            button.size = text_size;
        }

        if let Some(small) = style.text_styles.get_mut(&egui::TextStyle::Small) {
            small.size = text_size - 2.0;
        }

        ctx.set_style(style);
    }

    fn render_header(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading(
                RichText::new("🍯 Mazerion")
                    .size(32.0)
                    .color(colors::ACCENT)
                    .strong(),
            );
            ui.label(
                RichText::new("Professional Beverage Calculator Suite")
                    .size(16.0)
                    .color(colors::TEXT_MAIN),
            );
        });
    }

    fn render_tabs(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 5.0;

            if self
                .tab_button(ui, "📊 Basic", self.state.current_tab == TabView::Basic)
                .clicked()
            {
                self.state.current_tab = TabView::Basic;
                self.clear_results();
            }

            if self
                .tab_button(
                    ui,
                    "🔬 Advanced",
                    self.state.current_tab == TabView::Advanced,
                )
                .clicked()
            {
                self.state.current_tab = TabView::Advanced;
                self.clear_results();
            }

            if self
                .tab_button(
                    ui,
                    "🍺 Brewing",
                    self.state.current_tab == TabView::Brewing,
                )
                .clicked()
            {
                self.state.current_tab = TabView::Brewing;
                self.clear_results();
            }

            if self
                .tab_button(
                    ui,
                    "✨ Finishing",
                    self.state.current_tab == TabView::Finishing,
                )
                .clicked()
            {
                self.state.current_tab = TabView::Finishing;
                self.clear_results();
            }

            if self
                .tab_button(
                    ui,
                    "📐 Conversions",
                    self.state.current_tab == TabView::Conversions,
                )
                .clicked()
            {
                self.state.current_tab = TabView::Conversions;
                self.clear_results();
            }

            if self
                .tab_button(
                    ui,
                    "⚙️ Settings",
                    self.state.current_tab == TabView::Settings,
                )
                .clicked()
            {
                self.state.current_tab = TabView::Settings;
                self.clear_results();
            }
        });
    }

    fn tab_button(&self, ui: &mut egui::Ui, text: &str, active: bool) -> egui::Response {
        let (bg_color, text_color) = if active {
            (colors::TAB_ACTIVE, Color32::WHITE)
        } else {
            (colors::TAB_INACTIVE, colors::TEXT_MAIN)
        };

        let button = egui::Button::new(RichText::new(text).color(text_color).size(14.0))
            .fill(bg_color)
            .rounding(Rounding::same(5.0))
            .min_size(Vec2::new(115.0, 35.0));

        ui.add(button)
    }

    fn clear_results(&mut self) {
        self.state.result = None;
        self.state.warnings.clear();
        self.state.metadata.clear();
        self.state.conv_result = None;
    }

    fn render_results(&self, ui: &mut egui::Ui) {
        if self.state.result.is_none() && self.state.warnings.is_empty() {
            return;
        }

        ui.separator();
        ui.add_space(5.0);

        egui::Frame::none()
            .fill(colors::BG_PANEL)
            .stroke(Stroke::new(2.0, colors::ACCENT))
            .rounding(Rounding::same(10.0))
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.heading(RichText::new("📋 Results").color(colors::ACCENT));
                ui.add_space(5.0);

                if let Some(ref result) = self.state.result {
                    ui.label(
                        RichText::new(result)
                            .size(18.0)
                            .color(colors::TEXT_SUCCESS)
                            .strong(),
                    );
                }

                // Only show warnings if setting is enabled
                if self.state.show_warnings && !self.state.warnings.is_empty() {
                    ui.add_space(8.0);
                    for warning in &self.state.warnings {
                        ui.label(
                            RichText::new(format!("⚠️ {}", warning))
                                .size(14.0)
                                .color(colors::TEXT_WARNING),
                        );
                    }
                }

                // Only show metadata if setting is enabled
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
            .with_inner_size([950.0, 750.0])
            .with_min_inner_size([850.0, 650.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Mazerion - Professional Beverage Calculator",
        options,
        Box::new(|_cc| Ok(Box::new(MazerionApp::default()))),
    )
}
