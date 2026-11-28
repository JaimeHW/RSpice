//! The selected analysis's card: the directive it adds to the deck, its
//! prerequisites and their repair, and the datasets earlier runs left.

use super::*;

/// Gap between the instance contract's two columns.
const CONTRACT_COLUMN_GAP: f32 = 10.0;
/// The narrowest instance contract that can carry its two columns side by side.
///
/// The mockup collapses this block and the analysis form's grid together at
/// `@container (width <= 560px)`
/// (`styles/30-simulation/090-simulation-cockpit.css`). A CSS grid child
/// shrinks to whatever its track gives it and stays readable doing it; the
/// participation row — a select, a point picker and a stat laid out beside a
/// label column — shrinks by taking the room out of the select alone, and below
/// [`participation::CONTROL_ROW_MIN_WIDTH`] what is left of it no longer shows
/// which mode is chosen. So the breakpoint here is that floor doubled rather
/// than the mockup's number, which is the same rule resolved against the widget
/// set that actually draws it.
pub(super) const CONTRACT_SPLIT_MIN_WIDTH: f32 =
    2.0 * participation::CONTROL_ROW_MIN_WIDTH + CONTRACT_COLUMN_GAP;

#[derive(Clone)]
pub(super) struct DependencyCandidate {
    pub(super) id: AnalysisInstanceId,
    pub(super) label: String,
}

/// What the contract card says about prior datasets, and the way out to one.
///
/// The summary sentence and the hop are passed together because they are one
/// claim: a card that names Run 7 and a control that opens something else
/// would be two answers to the same question.
pub(super) struct ContractDatasets<'a> {
    pub(super) summary: &'a str,
    /// The display sequence of the run the summary names, when it names one
    /// that retained a dataset.
    pub(super) sequence: Option<u64>,
    pub(super) open: &'a mut bool,
}

/// The engine directive this draft contributes to the deck.
///
/// Read through [`crate::simulation::SimulationController::analysis_draft_directive`],
/// which is the same path the directive-parse ratchet proves parses. What the
/// operator reads here is therefore the statement the engine is actually
/// offered, not a re-spelling of it that happens to look like one.
///
/// The builders read the legacy singleton slots rather than the draft, so the
/// draft has to be projected into the state first. Projected in place and
/// restored exactly, rather than onto a cloned model: the model carries the
/// design, the datasets and the model libraries, and cloning that
/// once a frame to read one line of text would be the most expensive thing on
/// this route.
///
/// The draft is projected onto a copy of the setup, minus its plan catalog.
/// Every inactive plan is held there in full — drafts, receipts, tombstones —
/// and a whole copy carried all of them on every frame the Analyses page drew,
/// to read one directive line. The copy stands in for the call and the setup
/// itself is put back as it was, stamp and all, so a frame that draws the line
/// leaves the project as unedited as it found it; nothing on the directive
/// route reads the catalog
/// (`analysis_draft_directive` builds its spec from the legacy analysis slots
/// and the options block, and the only reader of `inactive_plans` in
/// `crate::simulation` is the campaign activator).
///
/// Takes the two things it needs rather than the whole application: it has no
/// business reaching the schematic, the workspace or the documents, and saying
/// so in the signature is what keeps that true.
pub fn plan_statement_for(
    model: &mut Model,
    controller: &SimulationController,
    draft: &AnalysisDraft,
) -> Result<String, String> {
    let mut projected = SimSetupState {
        setup: model.sim_setup.setup.clone_without_inactive_plans(),
        session: model.sim_setup.session.clone(),
    };
    projected.apply_analysis_draft_projection(draft);
    let setup = std::mem::replace(&mut model.sim_setup, projected);
    let statement = controller.analysis_draft_directive(model, draft);
    model.sim_setup = setup;
    statement
}

/// How far a re-parsed engineering spelling may sit from the number it was
/// made from before the statement keeps the number as written. The same
/// relative tolerance the netlist value hover and the models manager use.
const STATEMENT_ROUND_TRIP_TOLERANCE: f64 = 1e-12;

/// A directive in the notation a designer writes it in: `.tran 10n 1m`, not
/// `.tran 0.00000001 0.001`.
///
/// The card writer spells every number with `f64`'s own `Display`, so this
/// respells each numeric token — bare or after `KEY=` — with the deck spelling
/// [`crate::quantity::format_engineering_value`] gives (`Meg`, `u`), which is
/// what a SPICE parser reads back. A spelling that does not re-parse to the
/// same number keeps the digits as written. Integers below a million stay as
/// written too: those are counts (`.ac dec 10`, `.mc 1000`) as often as
/// quantities, and a count respelled `1k` reads as a quantity.
pub fn engineering_statement(statement: &str) -> String {
    statement
        .split(' ')
        .map(|token| match token.split_once('=') {
            Some((key, value)) => format!("{key}={}", engineering_token(value)),
            None => engineering_token(token),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn engineering_token(token: &str) -> String {
    let Ok(value) = token.parse::<f64>() else {
        return token.to_owned();
    };
    if !value.is_finite() || (value.fract() == 0.0 && value.abs() < 1e6) {
        return token.to_owned();
    }
    let spelled = crate::quantity::format_engineering_value(value);
    let round_trips = crate::quantity::parse_engineering_value(&spelled)
        .is_ok_and(|parsed| (parsed - value).abs() <= value.abs() * STATEMENT_ROUND_TRIP_TOLERANCE);
    if round_trips {
        spelled
    } else {
        token.to_owned()
    }
}

pub(super) fn analysis_contract(
    ui: &mut Ui,
    selected: &SelectedAnalysis,
    plan_statement: &Result<String, String>,
    datasets: ContractDatasets<'_>,
    resolved: &participation::PlanParticipation,
    participation_action: &mut Option<participation::ParticipationAction>,
    stacked: bool,
    action: &mut Option<AnalysisAction>,
) {
    let t = Tokens::get(ui.ctx());
    let content_width = (ui.available_width() - 16.0).max(1.0);
    // Its own width decides this, not only the viewport's. The block's left
    // column carries control rows with measured floors, and half of a 690-point
    // editor is not enough for the participation row's — which is how the
    // Analyses route came to paint the STB and Noise forms' right-hand column
    // outside the pane at the 1000-point gate.
    let stacked = stacked || content_width < CONTRACT_SPLIT_MIN_WIDTH;
    let response = egui::Frame::new()
        .fill(t.color.bg_inset)
        .inner_margin(egui::Margin::same(8))
        .show(ui, |ui| {
            ui.set_width(content_width);
            ui.spacing_mut().item_spacing.x = CONTRACT_COLUMN_GAP;
            ui.spacing_mut().item_spacing.y = 0.0;
            let mut property_action = None;
            let mut open_dataset = false;
            let mut properties = |ui: &mut Ui| {
                property_action = analysis_form::prerequisite_rows(ui, selected);
                if let Some(asked) = participation::participation_row(ui, resolved, selected.id) {
                    *participation_action = Some(asked);
                }
                // What this instance actually contributes to the deck. Every
                // other row here describes the analysis; this one is the
                // analysis, in the dialect the engine reads.
                match plan_statement {
                    Ok(statement) => {
                        property_row(ui, "Plan statement", &engineering_statement(statement))
                            .on_hover_text("The directive this instance contributes to the deck.")
                    }
                    Err(error) => {
                        property_row_problem(ui, "Plan statement", error, Some(Problem::Warning))
                            .on_hover_text(
                                "This analysis emits no directive until its configuration \
                                 resolves, so it would contribute nothing to the deck.",
                            )
                    }
                };
                // Words ("No prior datasets", "Run 3 finished"), so the body
                // face; the statement above is deck text and stays mono.
                property_row_wrapped(ui, "Prior datasets", datasets.summary);
                // The row above names the run; this opens it. Kept as a
                // separate control rather than making the row clickable,
                // because every other property row here is read-only and one
                // secretly-live row is worse than an explicit button.
                if let Some(sequence) = datasets.sequence {
                    ui.add_space(4.0);
                    let label = format!("Open Run {sequence} in Results");
                    open_dataset |= Button::new(&label)
                        .show(ui)
                        .on_hover_text(
                            "Open this run in Results without changing the plan",
                        )
                        .clicked();
                }
            };
            // The dependency graph speaks only when something in it is wrong: a
            // valid graph is the normal state and says nothing.
            let mut evidence = |ui: &mut Ui| {
                let problem = if let Some(reason) = &selected.contextual_dependency_error {
                    Some((page_kit::Tone::Error, "Cannot run", reason.clone()))
                } else if selected.repair_label.is_some() || selected.configure_pss_label.is_some() {
                    Some((
                        page_kit::Tone::Warn,
                        "Not ready to run",
                        "Choose what it uses before enabling it.".to_owned(),
                    ))
                } else {
                    None
                };
                if let Some((tone, title, detail)) = problem {
                    page_kit::note_line(ui, title, tone);
                    page_kit::note_line(ui, &detail, page_kit::Tone::Dim);
                }
                if let Some(configure_label) = selected.configure_pss_label.as_deref() {
                    ui.add_space(7.0);
                    if Button::new(configure_label)
                        .icon(Icon::Add)
                        .show(ui)
                        .on_hover_text(
                            "Open the autonomous PSS this analysis can use, adding one, switched off, when the test has none.",
                        )
                        .clicked()
                    {
                        *action = Some(AnalysisAction::PrepareAutonomousPss);
                    }
                }
            };
            if stacked {
                properties(ui);
                ui.add_space(8.0);
                evidence(ui);
            } else {
                ui.columns(2, |columns| {
                    properties(&mut columns[0]);
                    evidence(&mut columns[1]);
                });
            }
            if property_action.is_some() {
                *action = property_action;
            }
            *datasets.open = open_dataset;
        })
        .response;
    ui.painter().hline(
        response.rect.x_range(),
        response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}
