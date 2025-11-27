// Color constants - ACTUALLY READABLE
use eframe::egui::Color32;

// Background colors - LIGHT
pub const BG_MAIN: Color32 = Color32::from_rgb(245, 245, 245);  // Light gray
pub const BG_PANEL: Color32 = Color32::from_rgb(255, 255, 255); // White
pub const BG_INPUT: Color32 = Color32::from_rgb(250, 250, 250); // Off-white

// Text colors - DARK (readable on light backgrounds)
pub const TEXT_MAIN: Color32 = Color32::from_rgb(30, 30, 30);     // Almost black
pub const TEXT_LABEL: Color32 = Color32::from_rgb(60, 60, 60);    // Dark gray
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(100, 100, 100); // Medium gray

// Accent colors
pub const ACCENT: Color32 = Color32::from_rgb(218, 165, 32);      // Gold
pub const ACCENT_HOVER: Color32 = Color32::from_rgb(240, 185, 50); // Light gold
pub const ACCENT_ACTIVE: Color32 = Color32::from_rgb(180, 130, 20); // Dark gold

// Button colors
pub const BUTTON_PRIMARY: Color32 = Color32::from_rgb(34, 139, 34);  // Green
pub const BUTTON_PRIMARY_HOVER: Color32 = Color32::from_rgb(50, 165, 50); // Light green
pub const BUTTON_TEXT: Color32 = Color32::WHITE;

// Status colors
pub const SUCCESS: Color32 = Color32::from_rgb(34, 139, 34);   // Green
pub const WARNING: Color32 = Color32::from_rgb(255, 140, 0);   // Orange
pub const ERROR: Color32 = Color32::from_rgb(220, 20, 60);     // Red

// Border colors
pub const BORDER_LIGHT: Color32 = Color32::from_rgb(200, 200, 200); // Light gray
pub const BORDER_ACCENT: Color32 = Color32::from_rgb(218, 165, 32); // Gold