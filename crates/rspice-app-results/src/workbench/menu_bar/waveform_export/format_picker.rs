//! The form "Export dataset…" asks before it writes anything.
//!
//! The format used to be a Preferences setting and nothing else, so the only
//! way to export a TSV after a CSV was to leave the export, open Preferences,
//! change a default and come back. The form asks at the point of export,
//! offers the format used last time, and remembers the one chosen.
//!
//! It also asks what to write. A view of several analyses offers each of
//! them, the active strip's chosen until the reader picks another, so a
//! one-analysis format no longer sends the reader off to maximize a strip;
//! and the reader chooses between the signals on the plot and every signal
//! the analysis retains.

use egui::Context;

use crate::ui::widgets::{CommandForm, DialogChoice};
use crate::workbench::ChoicePreference;
use crate::workbench::documents::result_document::ResultsReads;
use crate::workbench::preferences::UserPreferences;

/// The open "Export dataset…" form: the format chosen so far, by its stored
/// index, and whether the view it exports shows more than one analysis, so
/// that the form asks which.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DatasetExportPicker {
    pub format: usize,
    pub several_analyses: bool,
}

/// Which of an analysis' signals an export writes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ExportedSignals {
    /// The traces on the plot, and the lanes on it where the format carries
    /// lanes: what the reader is looking at.
    #[default]
    Plotted,
    /// Every signal the analysis retains, on the plot or not.
    All,
}

/// What the form decided beside the format.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DatasetExportChoice {
    /// The run-local index of the one analysis to write, or `None` for what
    /// the view shows.
    pub analysis: Option<usize>,
    pub signals: ExportedSignals,
}

/// One displayed analysis the form offers, with what each signal choice
/// would write of it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct OfferedAnalysis {
    /// Run-local index.
    index: usize,
    label: String,
    traces: usize,
    plotted_traces: usize,
    /// Event nodes of a transient: its digital and real lanes.
    lanes: usize,
    plotted_lanes: usize,
}

/// The form's answers while it is open, and what it offers.
///
/// Built once when the form opens, from the view it exports, so a repaint
/// does not resolve the view again.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DatasetExportForm {
    /// The displayed analyses, in strip order.
    analyses: Vec<OfferedAnalysis>,
    /// Position in `analyses` of the active strip's analysis.
    active: Option<usize>,
    /// The view is a sheet that owns its own table (an operating point, a
    /// spectrum, a report), which every format writes whole.
    owns_table: bool,
    /// Position in `analyses` of the one chosen; `None` for every one.
    analysis: Option<usize>,
    signals: ExportedSignals,
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

/// Stored indices of the formats that have something particular to ask.
const VCD: usize = 5;
const RAW_BINARY: usize = 10;
const RAW_ASCII: usize = 11;

/// The first entry of the analysis list, which a table format writes as one
/// long-form file and a one-analysis format cannot hold.
const EVERY_ANALYSIS: &str = "Every displayed analysis";
const ONE_ANALYSIS_REASON: &str = "This format holds one analysis.";

/// The picker for the view that is showing, open on the format used last.
/// The error says why no view can be exported.
pub fn dataset_export_picker(state: ResultsReads<'_>) -> Result<DatasetExportPicker, String> {
    let displayed =
        crate::workbench::documents::result_document::view_context::resolve_displayed_result_view(
            state,
        )?;
    let format = state
        .model
        .preferences
        .choice(ChoicePreference::EngineeringExport)
        .min(FORMATS.len() - 1);
    Ok(DatasetExportPicker {
        format,
        several_analyses: displayed.analysis_indices.len() > 1,
    })
}

/// What the form offers over the view that is showing: each displayed
/// analysis with its trace and lane counts, and the active strip's. A view
/// that cannot be resolved offers nothing, and Export reports why.
pub fn dataset_export_form(state: ResultsReads<'_>) -> DatasetExportForm {
    let Ok(displayed) =
        crate::workbench::documents::result_document::view_context::resolve_displayed_result_view(
            state,
        )
    else {
        return DatasetExportForm::default();
    };
    let Some(run) = displayed.run(state) else {
        return DatasetExportForm::default();
    };
    let mut analyses = Vec::with_capacity(displayed.analysis_indices.len());
    for &index in &displayed.analysis_indices {
        let Some(analysis) = run.analyses.get(index) else {
            continue;
        };
        let plotted_traces = super::exported_waveforms(
            state,
            displayed.dataset_id,
            analysis,
            ExportedSignals::Plotted,
        )
        .len();
        let nodes = super::event_nodes(analysis);
        let plotted_lanes = nodes
            .iter()
            .filter(|node| {
                super::raw::plotted_event_lane(state, displayed.dataset_id, analysis, node)
            })
            .count();
        let mut label = analysis.label.clone();
        // Two strips of one kind read alike; their place in the run tells
        // them apart.
        if run
            .analyses
            .iter()
            .filter(|other| other.label == label)
            .count()
            > 1
        {
            label = format!("{label} ({})", index + 1);
        }
        analyses.push(OfferedAnalysis {
            index,
            label,
            traces: analysis.waveforms.len(),
            plotted_traces,
            lanes: nodes.len(),
            plotted_lanes,
        });
    }
    let active = displayed
        .primary_analysis_index
        .and_then(|primary| analyses.iter().position(|offered| offered.index == primary));
    // Whether the sheet owns its table is a question about one analysis: a
    // view of several always has the long-form stack to offer.
    let mut single = displayed.clone();
    single.analysis_indices = single.primary_analysis_index.into_iter().collect();
    DatasetExportForm {
        owns_table: super::prepare_displayed_table(state, &single, ExportedSignals::Plotted)
            .is_some(),
        analysis: if analyses.len() > 1 { None } else { active },
        analyses,
        active,
        signals: ExportedSignals::Plotted,
    }
}

impl DatasetExportForm {
    /// The position of the analysis `format` writes: the one chosen, else
    /// the active strip's for a one-analysis format, else every one.
    fn written_position(&self, format: usize) -> Option<usize> {
        self.analysis
            .or(if FORMATS[format].1 { self.active } else { None })
    }

    /// The choice the export reads, for a form showing `format`.
    fn choice(&self, format: usize) -> DatasetExportChoice {
        DatasetExportChoice {
            analysis: self
                .written_position(format)
                .and_then(|position| self.analyses.get(position))
                .map(|offered| offered.index),
            signals: self.signals,
        }
    }

    /// How many signals each choice writes in `format`, plotted and all:
    /// traces, and lanes where the format carries them.
    fn signal_counts(&self, format: usize) -> (usize, usize) {
        let lanes = usize::from(matches!(format, RAW_BINARY | RAW_ASCII));
        let position = self.written_position(format);
        self.analyses
            .iter()
            .enumerate()
            .filter(|(at, _)| position.is_none_or(|chosen| chosen == *at))
            .fold((0, 0), |(plotted, all), (_, offered)| {
                (
                    plotted + offered.plotted_traces + lanes * offered.plotted_lanes,
                    all + offered.traces + lanes * offered.lanes,
                )
            })
    }
}

/// Show the form while it is open. Returns the choice on the frame the reader
/// chooses Export, with the chosen format already stored as the one the
/// export writes and the one the form offers next time.
pub fn show_dataset_export_picker(
    ctx: &Context,
    dataset_export: &mut Option<DatasetExportPicker>,
    form: &mut Option<Box<DatasetExportForm>>,
    preferences: &mut UserPreferences,
) -> Option<DatasetExportChoice> {
    let mut picker = (*dataset_export)?;
    let draft = form.get_or_insert_with(Box::default);
    let labels = FORMATS
        .iter()
        .map(|(label, _)| (*label).to_owned())
        .collect::<Vec<_>>();
    let several = picker.several_analyses && draft.analyses.len() > 1;
    let (owns_table, offers_analyses) = (draft.owns_table, !draft.analyses.is_empty());
    // A sheet that owns its table writes it whole, and a dump is the whole
    // event schedule: neither has a choice of signals to make.
    let asks_signals = move |format: usize| !owns_table && format != VCD && offers_analyses;
    let (plotted, all) = draft.signal_counts(picker.format);
    let nothing_plotted = asks_signals(picker.format)
        && draft.signals == ExportedSignals::Plotted
        && plotted == 0
        && all > 0;
    // Each question stands on the label/value grid every form uses, its
    // control across the control column.
    let choice = CommandForm::new("Export dataset", "Export\u{2026}")
        .wide()
        .describe("Choose the file format for the displayed result.")
        .primary_enabled(!nothing_plotted)
        .status(if nothing_plotted {
            Err("Nothing in this analysis is plotted. Choose All to export its signals.")
        } else {
            Ok(None)
        })
        .show(ctx, |rows| {
            let format = rows.select_with_disabled(
                "Format",
                "results.export-dataset.format",
                FORMATS[picker.format].0,
                &labels,
                &[],
            );
            if let Some(index) = format.picked {
                picker.format = index;
            }
            if several {
                show_analysis_row(rows, draft, FORMATS[picker.format].1);
            }
            if asks_signals(picker.format) {
                let (plotted, all) = draft.signal_counts(picker.format);
                let plotted = format!("Plotted ({plotted})");
                let all = format!("All ({all})");
                let mut selected = usize::from(draft.signals == ExportedSignals::All);
                if rows.segmented_columns(
                    "Signals",
                    "results.export-dataset.signals",
                    &[plotted.as_str(), all.as_str()],
                    &mut selected,
                ) {
                    draft.signals = if selected == 1 {
                        ExportedSignals::All
                    } else {
                        ExportedSignals::Plotted
                    };
                }
            }
            // The first field takes the focus, as Export generated netlist's
            // does; the form opened with nothing focused.
            Some(format.response.id)
        });
    match choice {
        DialogChoice::Primary => {
            *dataset_export = None;
            let choice = form
                .take()
                .map(|draft| draft.choice(picker.format))
                .unwrap_or_default();
            preferences
                .set_choice(ChoicePreference::EngineeringExport, picker.format)
                .expect("the picker offers only formats the export writes");
            Some(choice)
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            *dataset_export = None;
            *form = None;
            None
        }
        DialogChoice::Secondary | DialogChoice::None => {
            *dataset_export = Some(picker);
            None
        }
    }
}

/// The analysis question of a view of several. A table format can write
/// every displayed strip as one long-form file; a one-analysis format lists
/// that entry too, unavailable and saying why, and writes the active strip's
/// analysis until the reader picks another.
fn show_analysis_row(
    rows: &mut crate::ui::widgets::FormRows<'_>,
    draft: &mut DatasetExportForm,
    one_analysis: bool,
) {
    if one_analysis && draft.analysis.is_none() {
        draft.analysis = draft.active;
    }
    let options = std::iter::once(EVERY_ANALYSIS.to_owned())
        .chain(draft.analyses.iter().map(|offered| offered.label.clone()))
        .collect::<Vec<_>>();
    let selected = draft
        .analysis
        .map_or(EVERY_ANALYSIS, |position| &draft.analyses[position].label);
    let disabled: &[(usize, &'static str)] = if one_analysis {
        &[(0, ONE_ANALYSIS_REASON)]
    } else {
        &[]
    };
    let picked = rows
        .select_with_disabled(
            "Analysis",
            "results.export-dataset.analysis",
            selected,
            &options,
            disabled,
        )
        .picked;
    if let Some(index) = picked {
        draft.analysis = index.checked_sub(1);
    }
}

#[cfg(test)]
mod tests;
