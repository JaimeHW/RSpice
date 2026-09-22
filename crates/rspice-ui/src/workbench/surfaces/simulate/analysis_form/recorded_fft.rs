//! The recorded-FFT form: one control per field of the engine's `.fft` card.
//!
//! The two choosers exist so an unrepresentable request cannot be typed. The
//! engine's rule for `NP` is a power of two in `4 ..= 1048576`, so the lengths
//! are offered rather than parsed; the window list is the engine's own enum,
//! spelled with the card keyword, so the control and the card are the same
//! word. Gaussian and Kaiser windows expose the engine's `ALFA` parameter.

use egui::Ui;

use crate::simulation::config::{FFT_WINDOWS, fft_point_counts, window_keyword};
use crate::simulation::plan::FftDraft;

use crate::quantity::{QuantityInputKind, QuantityPresentationPolicy, UiNumberLocale};

use super::{
    clear_pending_cell, field_advisory, field_note, hinted_quantity_input_row, input_row,
    sub_header,
};

/// The three `FORMAT=` positions. Index 0 writes no keyword at all, which is
/// the engine's own `Option<FftFormat>`: the mode decides.
const FORMATS: [&str; 3] = ["Mode default", "NORM", "UNORM"];

fn format_index(draft: &FftDraft) -> usize {
    match draft.format.trim() {
        "NORM" => 1,
        "UNORM" => 2,
        _ => 0,
    }
}

/// Render the recorded-FFT fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut FftDraft,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    input_row(ui, "Output", &mut setup.output);
    hinted_quantity_input_row(
        ui,
        "From",
        "0",
        &mut setup.start,
        QuantityInputKind::Time,
        policy,
        locale,
    );
    hinted_quantity_input_row(
        ui,
        "To",
        "transient stop",
        &mut setup.stop,
        QuantityInputKind::Time,
        policy,
        locale,
    );

    let counts = fft_point_counts();
    let labels = counts
        .iter()
        .map(|points| points.to_string())
        .collect::<Vec<_>>();
    let options = labels.iter().map(String::as_str).collect::<Vec<_>>();
    let mut selected = counts
        .iter()
        .position(|points| *points == setup.points)
        .unwrap_or(0);
    // "Samples", not "Points": the product bars a bare `Points` row on a
    // form with no graded sweep (`every_graded_sweep_form_names_what_a_point_is`),
    // because in two sweep modes out of three it is a false unit. This is the
    // engine's `NP`, a count of uniform time samples, and says so.
    if super::choice_row(ui, "Samples", &options, &mut selected) {
        // The number, never the index: a chooser position is not a request.
        setup.points = counts[selected];
    }

    let window_labels = FFT_WINDOWS.map(window_keyword);
    let mut window = FFT_WINDOWS
        .iter()
        .position(|candidate| window_keyword(*candidate) == setup.window)
        .unwrap_or(0);
    if super::choice_row(ui, "Window", &window_labels, &mut window) {
        setup.window = window_keyword(FFT_WINDOWS[window]).to_owned();
    }

    if matches!(setup.window.as_str(), "GAUSS" | "KAISER") {
        input_row(ui, "ALFA", &mut setup.alfa);
        field_note(ui, "Gaussian/Kaiser shape parameter (1..=20; default 3)");
    }

    let mut format = format_index(setup);
    if super::choice_row(ui, "Format", &FORMATS, &mut format) {
        setup.format = match format {
            1 => "NORM".to_owned(),
            2 => "UNORM".to_owned(),
            _ => String::new(),
        };
    }

    sub_header(ui, "Metric band");
    hinted_quantity_input_row(
        ui,
        "Fundamental",
        "bin 1",
        &mut setup.fundamental,
        QuantityInputKind::Frequency,
        policy,
        locale,
    );
    hinted_quantity_input_row(
        ui,
        "Band from",
        "bin 1",
        &mut setup.fmin,
        QuantityInputKind::Frequency,
        policy,
        locale,
    );
    hinted_quantity_input_row(
        ui,
        "Band to",
        "Nyquist",
        &mut setup.fmax,
        QuantityInputKind::Frequency,
        policy,
        locale,
    );
    match setup.to_request() {
        // The card this form writes, so what the run will carry is readable
        // without opening the deck.
        Ok(request) => field_note(ui, &request.to_card()),
        // Refused in the engine's own words, where the field is typed.
        Err(error) => field_advisory(ui, &error),
    }
    clear_pending_cell(ui);
}
