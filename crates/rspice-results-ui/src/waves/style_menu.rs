//! A trace's Style submenu: the width and dash of its line, the symbol on
//! its samples and how they are joined, each "Automatic" until the reader
//! picks one, as Virtuoso's trace properties offer them.

use egui::Ui;
use rspice_results::visualization_document::{
    TraceDash, TraceDraw, TraceStyle, TraceSymbol, TraceWidth,
};

/// Draw the Style entries for a trace drawn in `current`, and return the
/// style the reader picked.
pub fn style_menu(ui: &mut Ui, current: TraceStyle) -> Option<TraceStyle> {
    let mut chosen = None;
    rspice_ui_kit::widgets::menu_button(ui, "Width", |ui| {
        if let Some(width) = choices(
            ui,
            current.width,
            TraceWidth::ALL.map(|width| (Some(width), width.label())),
        ) {
            chosen = Some(TraceStyle { width, ..current });
        }
    });
    rspice_ui_kit::widgets::menu_button(ui, "Dash", |ui| {
        if let Some(dash) = choices(
            ui,
            current.dash,
            TraceDash::ALL.map(|dash| (Some(dash), dash.label())),
        ) {
            chosen = Some(TraceStyle { dash, ..current });
        }
    });
    rspice_ui_kit::widgets::menu_button(ui, "Symbols", |ui| {
        if let Some(symbol) = choices(
            ui,
            current.symbol,
            TraceSymbol::ALL.map(|symbol| (Some(symbol), symbol.label())),
        ) {
            chosen = Some(TraceStyle { symbol, ..current });
        }
    });
    rspice_ui_kit::widgets::menu_button(ui, "Draw as", |ui| {
        for draw in TraceDraw::ALL {
            if ui
                .add(egui::Button::selectable(current.draw == draw, draw.label()))
                .clicked()
            {
                chosen = Some(TraceStyle { draw, ..current });
                ui.close();
            }
        }
    });
    ui.separator();
    if ui
        .add_enabled(!current.is_default(), egui::Button::new("Reset style"))
        .clicked()
    {
        chosen = Some(TraceStyle::default());
        ui.close();
    }
    chosen
}

/// "Automatic", then each of `options`, the one `current` names selected;
/// returns the one picked.
fn choices<T: Copy + PartialEq, const N: usize>(
    ui: &mut Ui,
    current: Option<T>,
    options: [(Option<T>, &'static str); N],
) -> Option<Option<T>> {
    let mut picked = None;
    for (value, label) in std::iter::once((None, "Automatic")).chain(options) {
        if ui
            .add(egui::Button::selectable(current == value, label))
            .clicked()
        {
            picked = Some(value);
            ui.close();
        }
    }
    picked
}
