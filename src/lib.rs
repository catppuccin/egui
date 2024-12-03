//! Soothing pastel theme for [egui](egui).
//!
//! To use, call [`set_theme`](crate::set_theme) with the egui context
//! and a [`Theme`](crate::Theme).
//!
//! # Example
//!
//! ```rust
//! # use eframe::egui;
//! struct App;
//! impl eframe::App for App {
//!     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//!         catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO);
//!         egui::CentralPanel::default().show(&ctx, |ui| {
//!             ui.label("Hello, world!");
//!         });
//!     }
//! }
//! ```
//!
//! You can also customize your own theme:
//!
//! ```rust
//! # use eframe::egui;
//! use catppuccin_egui::{Theme, MOCHA};
//! const MY_MOCHA: Theme = Theme {
//!     red: egui::Color32::from_rgb(255, 0, 0),
//!     ..MOCHA
//! };
//! ```
//!

#[cfg(not(any(
    feature = "egui26",
    feature = "egui27",
    feature = "egui28",
    feature = "egui29"
)))]
compile_error!("at least one egui version must be enabled");

#[cfg(feature = "egui26")]
use egui26 as egui;
#[cfg(feature = "egui27")]
use egui27 as egui;
#[cfg(feature = "egui28")]
use egui28 as egui;
#[cfg(feature = "egui29")]
use egui29 as egui;

use egui::{epaint, style};

mod themes;
pub use themes::*;

/// Apply the given theme to a [`Context`](egui::Context).
pub fn set_theme(ctx: &egui::Context, theme: Theme) {
    let old = ctx.style().visuals.clone();
    ctx.set_visuals(theme.visuals(old));
}

/// Apply the given theme to a [`Style`](egui::Style).
///
/// # Example
///
/// ```rust
/// # use eframe::egui;
/// # use egui::__run_test_ctx;
/// # __run_test_ctx(|ctx| {
/// let mut style = (*ctx.style()).clone();
/// catppuccin_egui::set_style_theme(&mut style, catppuccin_egui::MACCHIATO);
/// ctx.set_style(style);
/// # });
/// ```
pub fn set_style_theme(style: &mut egui::Style, theme: Theme) {
    let old = style.visuals.clone();
    style.visuals = theme.visuals(old);
}

fn make_widget_visual(
    old: style::WidgetVisuals,
    theme: &Theme,
    bg_fill: egui::Color32,
) -> style::WidgetVisuals {
    style::WidgetVisuals {
        bg_fill,
        weak_bg_fill: bg_fill,
        bg_stroke: egui::Stroke {
            color: bg_fill,
            ..old.bg_stroke
        },
        fg_stroke: egui::Stroke {
            color: theme.text,
            ..old.fg_stroke
        },
        ..old
    }
}

impl Theme {
    fn visuals(&self, old: egui::Visuals) -> egui::Visuals {
        let is_latte = *self == LATTE;
        let shadow_color = if is_latte {
            egui::Color32::from_black_alpha(25)
        } else {
            egui::Color32::from_black_alpha(96)
        };

        egui::Visuals {
            override_text_color: Some(self.text),
            hyperlink_color: self.rosewater,
            faint_bg_color: self.surface0,
            extreme_bg_color: self.crust,
            code_bg_color: self.mantle,
            warn_fg_color: self.peach,
            error_fg_color: self.maroon,
            window_fill: self.base,
            panel_fill: self.base,
            window_stroke: egui::Stroke {
                color: self.overlay1,
                ..old.window_stroke
            },
            widgets: style::Widgets {
                noninteractive: make_widget_visual(old.widgets.noninteractive, self, self.base),
                inactive: make_widget_visual(old.widgets.inactive, self, self.surface0),
                hovered: make_widget_visual(old.widgets.hovered, self, self.surface2),
                active: make_widget_visual(old.widgets.active, self, self.surface1),
                open: make_widget_visual(old.widgets.open, self, self.surface0),
            },
            selection: style::Selection {
                bg_fill: self.blue.linear_multiply(if is_latte { 0.4 } else { 0.2 }),
                stroke: egui::Stroke {
                    color: self.overlay1,
                    ..old.selection.stroke
                },
            },

            window_shadow: epaint::Shadow {
                color: shadow_color,
                ..old.window_shadow
            },
            popup_shadow: epaint::Shadow {
                color: shadow_color,
                ..old.popup_shadow
            },
            dark_mode: !is_latte,
            ..old
        }
    }
}
