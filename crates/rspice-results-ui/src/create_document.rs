//! The New result document form: a name, the run and analysis it reads, the
//! sheet its panes draw, how many panes, and the signals they start with.

use egui::{Context, Id};
use rspice_results::document_creation::ResultDocumentLayout;
use rspice_ui_kit::widgets::table::{Cell, Column, DataTable};
use rspice_ui_kit::widgets::{CommandForm, DialogChoice, band_header};
use rspice_vocabulary::product::DatasetId;

use crate::document_dialogs::labels_and_selection;

/// Signal rows the list shows before it scrolls by itself.
const SIGNAL_ROWS: f32 = 8.0;

/// The draft the form edits, by identity: the run by its dataset, the
/// analysis by its place in the run, the sheet by its viewer document. The
/// host resolves every choice again against the retained runs before it
/// commits.
#[derive(Debug, Clone, PartialEq)]
pub struct CreateResultDocumentDialogState {
    pub open: bool,
    pub name: String,
    /// The reader typed the name, so a change of run or analysis keeps it.
    pub name_touched: bool,
    pub dataset_id: Option<DatasetId>,
    pub analysis: Option<usize>,
    pub viewer_id: String,
    pub layout: ResultDocumentLayout,
    /// The signals the panes plot, by name.
    pub signals: Vec<String>,
    /// Why the last Create failed, until the draft changes.
    pub validation_error: Option<String>,
}

impl Default for CreateResultDocumentDialogState {
    fn default() -> Self {
        Self {
            open: false,
            name: String::new(),
            name_touched: false,
            dataset_id: None,
            analysis: None,
            viewer_id: String::new(),
            layout: ResultDocumentLayout::SinglePane,
            signals: Vec::new(),
            validation_error: None,
        }
    }
}

/// What the form offers, resolved by the host against the retained runs.
#[derive(Debug, Clone, Default)]
pub struct CreateDocumentChoices {
    /// Finished runs, named as the data browser names them.
    pub runs: Vec<(DatasetId, String)>,
    /// The chosen run's analyses, by place, named as the data browser names
    /// them.
    pub analyses: Vec<(usize, String)>,
    /// The sheets that draw the chosen analysis, by viewer document.
    pub viewers: Vec<(&'static str, String)>,
    /// The chosen analysis's signals, when the chosen sheet plots signals.
    pub signals: Vec<String>,
    /// Where each ticked signal lands, beside `signals`, when the layout has
    /// more than one pane.
    pub places: Vec<Option<&'static str>>,
    /// Why Create is refused, when it is.
    pub refusal: Option<String>,
}

pub fn show(
    ctx: &Context,
    draft: &mut CreateResultDocumentDialogState,
    choices: &CreateDocumentChoices,
) -> DialogChoice {
    let name_id = Id::new("rspice.create-result-document.name");
    let refusal = draft
        .validation_error
        .clone()
        .or_else(|| choices.refusal.clone());
    let mut ticked = std::mem::take(&mut draft.signals);
    let held = ticked.clone();
    let form = CommandForm::new("New result document", "Create")
        .describe("Create a result document over one analysis of a run.")
        .first_field(name_id)
        .primary_enabled(refusal.is_none())
        .status(refusal.as_deref().map_or(Ok(None), Err));
    let rows = |rows: &mut rspice_ui_kit::widgets::FormRows<'_>| {
        if rows
            .text_sans("Name", name_id, &mut draft.name, "")
            .changed()
        {
            draft.name_touched = true;
            draft.validation_error = None;
        }
        let (labels, current) = labels_and_selection(&choices.runs, draft.dataset_id);
        if let Some(index) = rows.select(
            "Run",
            "rspice.create-result-document.run",
            &current,
            &labels,
        ) {
            draft.dataset_id = Some(choices.runs[index].0);
            draft.validation_error = None;
        }
        let (labels, current) = labels_and_selection(&choices.analyses, draft.analysis);
        if let Some(index) = rows.select(
            "Analysis",
            "rspice.create-result-document.analysis",
            &current,
            &labels,
        ) {
            draft.analysis = Some(choices.analyses[index].0);
            draft.validation_error = None;
        }
        let viewer = choices
            .viewers
            .iter()
            .position(|(id, _)| *id == draft.viewer_id);
        let (labels, current) = labels_and_selection(
            &choices.viewers,
            viewer.map(|index| choices.viewers[index].0),
        );
        if let Some(index) = rows.select(
            "Sheet",
            "rspice.create-result-document.sheet",
            &current,
            &labels,
        ) {
            choices.viewers[index].0.clone_into(&mut draft.viewer_id);
            draft.validation_error = None;
        }
        let layouts: Vec<String> = ResultDocumentLayout::ALL
            .iter()
            .map(|layout| layout.label().to_owned())
            .collect();
        if let Some(index) = rows.select(
            "Layout",
            "rspice.create-result-document.layout",
            draft.layout.label(),
            &layouts,
        ) {
            draft.layout = ResultDocumentLayout::ALL[index];
            draft.validation_error = None;
        }
        Some(name_id)
    };
    if choices.signals.is_empty() {
        let choice = form.show(ctx, rows);
        draft.signals = ticked;
        return choice;
    }
    let signals = &choices.signals;
    let choice = form.show_over(ctx, rows, |ui| {
        let count = format!(
            "{} of {}",
            signals
                .iter()
                .filter(|signal| ticked.contains(signal))
                .count(),
            signals.len()
        );
        band_header(ui, "Signals", Some(&count));
        let columns = [Column::name("Signal", 120.0)];
        let row_h = rspice_ui_kit::widgets::table::LineHeights::list(
            &rspice_ui_kit::tokens::Tokens::get(ui.ctx()),
        )
        .row;
        let response = DataTable::new(
            "rspice.create-result-document.signals",
            &columns,
            signals.len(),
        )
        .list()
        .max_height(row_h * SIGNAL_ROWS)
        .show(ui, |row| {
            let signal = &signals[row];
            let cell = Cell::text(signal.as_str()).check(ticked.contains(signal));
            // The pane a ticked signal lands in, when there is a choice of
            // pane: the panes take the signals by unit.
            vec![match choices.places.get(row).copied().flatten() {
                Some(place) => cell.state(place, None),
                None => cell,
            }]
        });
        if let Some((row, on)) = response.checked {
            let signal = &signals[row];
            ticked.retain(|kept| kept != signal);
            if on {
                ticked.push(signal.clone());
            }
            // The run's order, so the panes plot them as the run lists them.
            ticked.sort_by_key(|kept| signals.iter().position(|signal| signal == kept));
            // The panes the ticks land in follow on the next frame.
            ui.ctx().request_repaint();
        }
    });
    if ticked != held {
        draft.validation_error = None;
    }
    draft.signals = ticked;
    choice
}
