//! The format question "Export dataset…" asks before it writes anything.
//!
//! The format used to be a Preferences setting and nothing else, so the only
//! way to export a TSV after a CSV was to leave the export, open Preferences,
//! change a default and come back. The picker asks at the point of export,
//! offers the format used last time, and remembers the one chosen.

use egui::Context;

use crate::ui::widgets::{CommandForm, DialogChoice};
use crate::workbench::ChoicePreference;
use crate::workbench::documents::result_document::ResultsReads;
use crate::workbench::preferences::UserPreferences;

/// The open "Export dataset…" format picker: the format chosen so far, by
/// its stored index, and whether the view it exports shows more than one
/// analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DatasetExportPicker {
    pub format: usize,
    pub several_analyses: bool,
}

/// Every format the export writes, in the order the stored choice numbers
/// them, and whether it can hold only one analysis.
const FORMATS: [(&str, bool); 12] = [
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
    ("SPICE raw \u{00b7} binary, for ngspice and LTspice", true),
    ("SPICE raw \u{00b7} ASCII values", true),
];

/// Why a one-analysis format is unavailable on a view that shows several.
const ONE_ANALYSIS_REASON: &str = "Exports one analysis. Maximize one strip to use it.";

/// The picker for the view that is showing, open on the format used last, or
/// on CSV when that one cannot export the view. The error says why no view
/// can be exported.
pub fn dataset_export_picker(state: ResultsReads<'_>) -> Result<DatasetExportPicker, String> {
    let displayed =
        crate::workbench::documents::result_document::view_context::resolve_displayed_result_view(
            state,
        )?;
    let several_analyses = displayed.analysis_indices.len() > 1;
    let last = state
        .model
        .preferences
        .choice(ChoicePreference::EngineeringExport)
        .min(FORMATS.len() - 1);
    let format = if several_analyses && FORMATS[last].1 {
        0
    } else {
        last
    };
    Ok(DatasetExportPicker {
        format,
        several_analyses,
    })
}

/// Show the picker while it is open. Returns `true` on the frame the reader
/// chooses Export, with the chosen format already stored as the one the
/// export writes and the one the picker offers next time.
pub fn show_dataset_export_picker(
    ctx: &Context,
    dataset_export: &mut Option<DatasetExportPicker>,
    preferences: &mut UserPreferences,
) -> bool {
    let Some(mut picker) = *dataset_export else {
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
    // Format stands on the label/value grid every form uses, its list across
    // the control column. A line under it said the same of every format.
    let choice = CommandForm::new("Export dataset", "Export\u{2026}")
        .wide()
        .describe("Choose the file format for the displayed result.")
        .show(ctx, |rows| {
            let format = rows.select_with_disabled(
                "Format",
                "results.export-dataset.format",
                FORMATS[picker.format].0,
                &labels,
                &disabled,
            );
            if let Some(index) = format.picked {
                picker.format = index;
            }
            // The first field takes the focus, as Export generated netlist's
            // does; the form opened with nothing focused.
            Some(format.response.id)
        });
    match choice {
        DialogChoice::Primary => {
            *dataset_export = None;
            preferences
                .set_choice(ChoicePreference::EngineeringExport, picker.format)
                .expect("the picker offers only formats the export writes");
            true
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            *dataset_export = None;
            false
        }
        DialogChoice::Secondary | DialogChoice::None => {
            *dataset_export = Some(picker);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::EngineeringExportFormat;
    use crate::workbench::documents::result_document::test_host::ResultsTestHost;

    /// The picker's list is the stored choice's numbering, and its "one
    /// analysis" flags agree with the formats the export refuses on a
    /// multi-analysis view.
    #[test]
    fn the_picker_offers_every_format_the_export_writes_in_stored_order() {
        let mut state = ResultsTestHost::default();
        for (index, (_, one_analysis)) in FORMATS.iter().enumerate() {
            state
                .model
                .preferences
                .set_choice(ChoicePreference::EngineeringExport, index)
                .expect("every listed format is a stored choice");
            let format = state
                .model
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
                .model
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
        let mut state = ResultsTestHost::default();
        state.results.dataset_export = Some(DatasetExportPicker {
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
            exported |= show_dataset_export_picker(
                ctx,
                &mut state.results.dataset_export,
                &mut state.model.preferences,
            );
        });
        assert!(
            state.results.dataset_export.is_some(),
            "the picker stays open until it is answered"
        );
        let _ = ctx.run_ui(escape, |ctx| {
            exported |= show_dataset_export_picker(
                ctx,
                &mut state.results.dataset_export,
                &mut state.model.preferences,
            );
        });
        assert!(!exported);
        assert!(state.results.dataset_export.is_none());
        assert_eq!(
            state
                .model
                .preferences
                .choice(ChoicePreference::EngineeringExport),
            0,
            "a cancelled picker leaves the remembered format alone"
        );
    }

    /// Format stands in the label column with its list beside it, and the
    /// list ends where the footer's actions end, as on every form.
    #[test]
    fn the_format_list_stands_on_the_form_grid() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut state = ResultsTestHost::default();
        state.results.dataset_export = Some(DatasetExportPicker {
            format: 0,
            several_analyses: false,
        });
        let mut nodes = std::collections::HashMap::new();
        for _ in 0..3 {
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1440.0, 900.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    show_dataset_export_picker(
                        ctx,
                        &mut state.results.dataset_export,
                        &mut state.model.preferences,
                    );
                },
            );
            nodes.extend(
                output
                    .platform_output
                    .accesskit_update
                    .expect("AccessKit tree update")
                    .nodes,
            );
        }
        let bounds = |label: &str, role: egui::accesskit::Role| {
            nodes
                .values()
                .find(|node| node.role() == role && node.label() == Some(label))
                .and_then(egui::accesskit::Node::bounds)
                .unwrap_or_else(|| panic!("{label} is published"))
        };
        let format = bounds("Format", egui::accesskit::Role::ComboBox);
        let export = bounds("Export\u{2026}", egui::accesskit::Role::Button);
        let title = nodes
            .values()
            .find(|node| {
                node.role() == egui::accesskit::Role::Label
                    && (node.label() == Some("Export dataset")
                        || node.value() == Some("Export dataset"))
            })
            .and_then(egui::accesskit::Node::bounds)
            .expect("the title is published");
        assert!(
            format.x0 > title.x0 + 60.0 && (format.x1 - export.x1).abs() < 1.0,
            "Format {format:?}, title {title:?}, Export {export:?}"
        );
    }
}
