//! The format question "Export dataset…" asks before it writes anything.
//!
//! The format used to be a Preferences setting and nothing else, so the only
//! way to export a TSV after a CSV was to leave the export, open Preferences,
//! change a default and come back. The picker asks at the point of export,
//! offers the format used last time, and remembers the one chosen.

use egui::Context;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, select_with_disabled};
use crate::workbench::ChoicePreference;
use crate::workbench::app::DatasetExportPicker;
use crate::workbench::app_state::AppState;

/// Every format the export writes, in the order the stored choice numbers
/// them, and whether it can hold only one analysis.
const FORMATS: [(&str, bool); 10] = [
    ("CSV", false),
    ("Touchstone where compatible", false),
    ("TSV", false),
    ("RSpice result bundle", true),
    ("RSpice dataset bundle", true),
    ("VCD \u{00b7} event timelines", true),
    ("NumPy .npy \u{00b7} one array, column order stated", true),
    ("NumPy .npz \u{00b7} one named array per signal", true),
    ("HDF5 \u{00b7} names and units carried", true),
    ("MATLAB v5 \u{00b7} column vectors, complex for AC", true),
];

/// Why a one-analysis format is unavailable on a view that shows several.
const ONE_ANALYSIS_REASON: &str = "Exports one analysis. Maximize one strip to use it.";

/// Open the picker on the format used last, or on CSV when that one cannot
/// export the view that is showing.
pub(crate) fn open_dataset_export_picker(state: &mut AppState) {
    let displayed =
        match crate::workbench::documents::result_document::view_context::resolve_displayed_result_view(
            state,
        ) {
            Ok(displayed) => displayed,
            Err(message) => {
                state.prompt(message);
                return;
            }
        };
    let several_analyses = displayed.analysis_indices.len() > 1;
    let last = state
        .ui
        .preferences
        .choice(ChoicePreference::EngineeringExport)
        .min(FORMATS.len() - 1);
    let format = if several_analyses && FORMATS[last].1 {
        0
    } else {
        last
    };
    state.dialogs.dataset_export = Some(DatasetExportPicker {
        format,
        several_analyses,
    });
}

/// Show the picker while it is open. Returns `true` on the frame the reader
/// chooses Export, with the chosen format already stored as the one the
/// export writes and the one the picker offers next time.
pub(crate) fn show_dataset_export_picker(ctx: &Context, state: &mut AppState) -> bool {
    let Some(mut picker) = state.dialogs.dataset_export else {
        return false;
    };
    let labels = FORMATS
        .iter()
        .map(|(label, _)| (*label).to_owned())
        .collect::<Vec<_>>();
    let disabled = if picker.several_analyses {
        FORMATS
            .iter()
            .enumerate()
            .filter(|(_, (_, one_analysis))| *one_analysis)
            .map(|(index, _)| (index, ONE_ANALYSIS_REASON))
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let choice = Dialog::new("Export dataset", "Export\u{2026}")
        .description("Choose the file format for the displayed result.")
        .size(DialogSize::Confirmation)
        .ghost("Cancel")
        .show(ctx, |ui| {
            let t = Tokens::get(ui.ctx());
            ui.label(
                egui::RichText::new("Format")
                    .font(theme::sans(tokens::FS_1, FontWeight::Medium))
                    .color(t.color.text),
            );
            ui.add_space(4.0);
            if let Some(index) = select_with_disabled(
                ui,
                "results.export-dataset.format",
                "Format",
                FORMATS[picker.format].0,
                &labels,
                &disabled,
                ui.available_width(),
            ) {
                picker.format = index;
            }
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new("Every format keeps full stored precision.")
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
        });
    match choice {
        DialogChoice::Primary => {
            state.dialogs.dataset_export = None;
            state
                .ui
                .preferences
                .set_choice(ChoicePreference::EngineeringExport, picker.format)
                .expect("the picker offers only formats the export writes");
            true
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            state.dialogs.dataset_export = None;
            false
        }
        DialogChoice::Secondary | DialogChoice::None => {
            state.dialogs.dataset_export = Some(picker);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::EngineeringExportFormat;

    /// The picker's list is the stored choice's numbering, and its "one
    /// analysis" flags agree with the formats the export refuses on a
    /// multi-analysis view.
    #[test]
    fn the_picker_offers_every_format_the_export_writes_in_stored_order() {
        let mut state = AppState::default();
        for (index, (_, one_analysis)) in FORMATS.iter().enumerate() {
            state
                .ui
                .preferences
                .set_choice(ChoicePreference::EngineeringExport, index)
                .expect("every listed format is a stored choice");
            let format = state
                .ui
                .preferences
                .result_presentation_policy()
                .engineering_export();
            let tabular = matches!(
                format,
                EngineeringExportFormat::Csv
                    | EngineeringExportFormat::Tsv
                    | EngineeringExportFormat::TouchstoneWhereCompatible
            );
            assert_eq!(*one_analysis, !tabular, "format {index} disagrees");
        }
        assert!(
            state
                .ui
                .preferences
                .set_choice(ChoicePreference::EngineeringExport, FORMATS.len())
                .is_err(),
            "the list covers the whole stored domain"
        );
    }

    /// Export stores the chosen format; Cancel stores nothing.
    #[test]
    fn export_remembers_the_format_and_cancel_does_not() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut state = AppState::default();
        state.dialogs.dataset_export = Some(DatasetExportPicker {
            format: 2,
            several_analyses: false,
        });
        let escape = egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::Escape,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..egui::RawInput::default()
        };
        let mut exported = false;
        let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
            exported |= show_dataset_export_picker(ctx, &mut state);
        });
        assert!(
            state.dialogs.dataset_export.is_some(),
            "the picker stays open until it is answered"
        );
        let _ = ctx.run_ui(escape, |ctx| {
            exported |= show_dataset_export_picker(ctx, &mut state);
        });
        assert!(!exported);
        assert!(state.dialogs.dataset_export.is_none());
        assert_eq!(
            state
                .ui
                .preferences
                .choice(ChoicePreference::EngineeringExport),
            0,
            "a cancelled picker leaves the remembered format alone"
        );
    }
}
