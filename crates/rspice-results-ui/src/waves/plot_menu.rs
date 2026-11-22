//! The menu a right click on a strip's plot opens, as ViVA's graph menu
//! and ADS's plot menu offer theirs: a cursor, a marker or a note at the
//! point clicked, a fit, the values the readout reads there, the curve
//! nearest the point for the calculator, and a plot of the strip's curves
//! against something else. Shift+F10 opens it from the keyboard, acting on
//! the middle of the window.

use egui::Ui;
use rspice_results::visualization_document::Abscissa;
use rspice_ui_kit::plot::CursorId;

use super::StripModel;
use super::header::{AbscissaMenu, abscissa_choices};

/// Where the menu acts, as the right click that opened it left it.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct MenuPlace {
    /// The X clicked.
    pub x: f64,
    /// The value of the strip's first axis at the height clicked.
    pub y: f64,
    /// The curve nearest the click, by its index in the strip model.
    pub trace: Option<usize>,
    /// What the hover readout read at `x`: each row's name and value.
    pub values: Vec<(String, String)>,
}

/// What the reader chose from the menu.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PlotChoice {
    /// Put this cursor at the X clicked.
    Cursor(CursorId),
    /// Put a marker on this curve at the X clicked.
    Marker(usize),
    /// Add a note at the point clicked, or on this curve at the X clicked.
    Note(Option<usize>),
    /// Fit every axis of the strip, and the shared X, to the curves again.
    Fit,
    /// Copy what the readout reads at the X clicked.
    CopyValues,
    /// Hand this curve to the calculator.
    Calculator(usize),
    /// Plot the strip's curves against this.
    PlotAgainst(Abscissa),
}

/// What the menu offers beyond its fixed entries.
pub(crate) struct MenuOffers<'a> {
    /// Whether the host takes a note added at a point.
    pub notes: bool,
    /// What the strip's curves may be plotted against, with the strip's
    /// unit and the id the menu keeps a typed expression under.
    pub against: Option<(&'a AbscissaMenu, &'a str, egui::Id)>,
}

/// Draw the menu's entries for a right click at `place` on a strip of
/// `model`, and return the entry chosen.
pub(crate) fn show(
    ui: &mut Ui,
    model: &StripModel,
    place: &MenuPlace,
    offers: &MenuOffers<'_>,
) -> Option<PlotChoice> {
    let mut chosen = None;
    let nearest = place.trace.and_then(|index| model.traces.get(index));
    let response = ui.button("Place cursor A here");
    pick(ui, &response, PlotChoice::Cursor(CursorId::A), &mut chosen);
    let response = ui.button("Place cursor B here");
    pick(ui, &response, PlotChoice::Cursor(CursorId::B), &mut chosen);
    let no_curve = "No curve is near the point clicked";
    let marker = ui
        .add_enabled(nearest.is_some(), egui::Button::new("Add marker here"))
        .on_disabled_hover_text(no_curve);
    if let Some(index) = place.trace {
        pick(ui, &marker, PlotChoice::Marker(index), &mut chosen);
    }
    if offers.notes {
        let response = ui
            .button("Add note here")
            .on_hover_text("Write a note at this point of the axes");
        pick(ui, &response, PlotChoice::Note(None), &mut chosen);
        let on_curve = ui
            .add_enabled(nearest.is_some(), egui::Button::new("Add note on curve"))
            .on_disabled_hover_text(no_curve);
        if let (Some(index), Some(trace)) = (place.trace, nearest) {
            let on_curve = on_curve.on_hover_text(format!(
                "Write a note on {} at the X clicked, which follows the curve",
                trace.name
            ));
            pick(ui, &on_curve, PlotChoice::Note(Some(index)), &mut chosen);
        }
    }
    ui.separator();
    let response = ui
        .button("Fit")
        .on_hover_text("Fit the shared X and every axis of the strip to its curves");
    pick(ui, &response, PlotChoice::Fit, &mut chosen);
    let copy = ui
        .add_enabled(!place.values.is_empty(), egui::Button::new("Copy values"))
        .on_hover_text("Copy what the readout reads at the point clicked")
        .on_disabled_hover_text("The readout reads nothing here");
    pick(ui, &copy, PlotChoice::CopyValues, &mut chosen);
    let calculator = ui
        .add_enabled(nearest.is_some(), egui::Button::new("Send to calculator"))
        .on_disabled_hover_text(no_curve);
    if let (Some(index), Some(trace)) = (place.trace, nearest) {
        let calculator = calculator.on_hover_text(format!("Send {} to the calculator", trace.name));
        pick(ui, &calculator, PlotChoice::Calculator(index), &mut chosen);
    }
    if let Some((menu, unit, draft_id)) = offers.against {
        rspice_ui_kit::widgets::menu_button(ui, "Plot against", |ui| {
            if let Some(abscissa) =
                abscissa_choices(ui, unit, menu, &|name: &str| name.to_owned(), draft_id)
            {
                chosen = Some(PlotChoice::PlotAgainst(abscissa));
                ui.close();
            }
        });
    }
    chosen
}

/// Take `choice` when `response` was clicked, and close the menu.
fn pick(ui: &Ui, response: &egui::Response, choice: PlotChoice, chosen: &mut Option<PlotChoice>) {
    if response.clicked() {
        *chosen = Some(choice);
        ui.close();
    }
}

/// The lines "Copy values" puts on the clipboard: each readout row's name
/// and value, a tab between them.
pub(crate) fn copied_values(values: &[(String, String)]) -> String {
    values
        .iter()
        .map(|(name, value)| format!("{name}\t{value}"))
        .collect::<Vec<_>>()
        .join("\n")
}
