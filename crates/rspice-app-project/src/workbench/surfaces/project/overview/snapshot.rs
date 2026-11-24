//! What the Overview says, read once per frame from the project, its
//! design, the checks, the run history, Verify's judge and the catalogues
//! before anything is drawn.
//!
//! # Derived on change, not on a frame
//!
//! What has to walk the design, a catalogue or a text to be known — the
//! device census, the dependency statuses, the model files, the deck's
//! include lines — is derived once and kept in the frame context beside the
//! identity of what it was derived from. A frame in which none of those
//! moved derives nothing, so the page costs the same on a project of eight
//! thousand instances as on an empty one.
//!
//! Each fact's words and mark are its subject's, from
//! [`crate::workbench::project::health`], the dependency ledger, the
//! checks' receipt and Verify's sheet: nothing here judges anything again.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher as _};
use std::sync::Arc;

use rspice_app_state::results::{RowStatus, difference_text, format_value, limit_text};

use crate::state::{AnalysisResult, AnalysisResultFamilyMetadata, AnalysisType, SimulationRun};
use crate::ui::accessibility::counted;
use crate::workbench::app_state::{DesignCheckStatus, Model};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::{CodePort, Cx};
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectCommand;
use crate::workbench::project::dependency_status::{DependencyKind, dependency_rows};
use crate::workbench::project::health::{self, SubjectStatus, worse};
use crate::workbench::project::workspace_state::ProjectPage;
use crate::workbench::state::Workspace;

use super::super::derived::derived;
use super::census::design_census;
use super::{OverviewIntent, Tone, command_gate};

/// What a linked value, a row or an empty table's button does: its words,
/// what it asks for, and whether it can be taken now.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
    pub label: &'static str,
    pub intent: OverviewIntent,
    pub enabled: bool,
    pub disabled_reason: Option<&'static str>,
}

impl Action {
    fn new(
        label: &'static str,
        intent: OverviewIntent,
        (enabled, disabled_reason): (bool, Option<&'static str>),
    ) -> Self {
        Self {
            label,
            intent,
            enabled,
            disabled_reason,
        }
    }
}

/// One fact of the summary band: what it is, its value, the mark the
/// reader has to see, and where its value leads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SummaryFact {
    pub label: &'static str,
    pub value: String,
    pub tone: Tone,
    pub link: Option<Action>,
}

impl SummaryFact {
    fn new(label: &'static str, value: impl Into<String>) -> Self {
        Self {
            label,
            value: value.into(),
            tone: Tone::Neutral,
            link: None,
        }
    }

    /// A subject's state and the one line beside it, as one value.
    fn of(label: &'static str, status: SubjectStatus) -> Self {
        let value = if status.detail.is_empty() {
            status.state
        } else {
            format!("{} \u{b7} {}", status.state, status.detail)
        };
        Self {
            tone: status.tone,
            ..Self::new(label, value)
        }
    }

    const fn toned(mut self, tone: Tone) -> Self {
        self.tone = tone;
        self
    }

    fn linked(mut self, link: Option<Action>) -> Self {
        self.link = link;
        self
    }
}

/// One specification Verify judged of the run: its output, its value or
/// the spread of its values, its limit, the margin to it, and the verdict.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecificationLine {
    pub output: String,
    /// The measurement behind the output, when it is named apart from it.
    pub measurement: Option<String>,
    /// The worst value it took: the one value of a run of one point.
    pub value: Option<String>,
    /// The least and greatest values across the run's points or trials.
    pub min: Option<String>,
    pub max: Option<String>,
    /// Where the worst value was: the corner, the trial or the point.
    pub worst_at: Option<String>,
    pub limit: Option<String>,
    pub margin: Option<String>,
    pub verdict: Option<(&'static str, Tone)>,
    pub action: Action,
}

/// One measurement of a run that no limit judges.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasurementLine {
    pub output: String,
    pub value: String,
    pub action: Action,
}

/// The table under the band: the latest run's outputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outputs {
    /// Verify's verdicts. A run of several points or trials is read by its
    /// spread, `spread` heading where the worst was ("Worst corner"); a run
    /// of one point by its one value.
    Judged {
        spread: Option<&'static str>,
        rows: Vec<SpecificationLine>,
    },
    /// What the run measured, where no limit judges it.
    Measured(Vec<MeasurementLine>),
    /// No run yet: what a run does, and what runs one.
    NoRun(String, Action),
    /// A run that measured nothing: where outputs are added.
    Unmeasured(Action),
}

/// The Overview, as it stands this frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverviewSnapshot {
    /// The project runs a deck and has no drawing of its own yet.
    pub netlist_first: bool,
    /// The summary band's facts, in reading order.
    pub facts: Vec<SummaryFact>,
    /// What the reader has to know about the verdicts before reading them:
    /// that they are of another run, or why Verify cannot judge.
    pub note: Option<String>,
    pub outputs: Outputs,
}

impl OverviewSnapshot {
    /// Read the Overview from `cx`, deriving on `ctx` only what changed.
    pub fn capture(ctx: &egui::Context, cx: &Cx<'_, ProjectFeature>) -> Self {
        let model = &*cx.model;
        let netlist_first = model.is_netlist_first_without_schematic();
        let deck = (netlist_first || !model.workspace.content.decks.is_empty())
            .then(|| deck_summary(ctx, model, cx.peers.code()))
            .flatten();
        // A deck-driven project runs its deck as written, not the plan.
        let mut facts = vec![if netlist_first {
            SummaryFact::new(
                "Runs",
                deck.as_deref()
                    .map_or_else(|| "The deck".to_owned(), DeckSummary::runs),
            )
        } else {
            SummaryFact::new("Plan", model.sim_setup.active_plan_name().as_str())
        }];
        // The inspector names the configuration the project marks for runs;
        // the page says which one runs only where that is not it, or where
        // it cannot run.
        if !netlist_first {
            let status = health::configuration(ctx, model);
            let marked = model
                .workspace
                .content
                .configuration_sets
                .active()
                .map_or("Default", |configuration| configuration.name());
            if status.tone != Tone::Neutral || status.state != marked {
                facts.push(SummaryFact::of("Configuration", status));
            }
        }
        // A deck-driven project keeps its top cell pristine: it has no
        // hierarchy or drawing to check until it has a schematic.
        if !netlist_first && let Some(census) = design_census(ctx, model) {
            facts.push(SummaryFact::new("Hierarchy", census.hierarchy_text()));
            facts.push(SummaryFact::new("Devices", census.devices_text()));
        }
        if netlist_first || !model.workspace.content.decks.is_empty() {
            let includes = deck.as_ref().map(|deck| deck.includes.len());
            let deck = command_gate(cx, ids::open_workspace(Workspace::Netlist));
            facts.push(
                SummaryFact::of("Source deck", health::source_deck(model, includes)).linked(Some(
                    Action::new("Open deck", OverviewIntent::OpenProjectDeck, deck),
                )),
            );
        }
        facts.push(dependencies(ctx, cx));
        if !netlist_first {
            facts.push(checks(cx));
        }
        facts.extend(latest_run(cx));
        let (note, outputs) = outputs(cx, netlist_first.then_some(deck.as_deref()));
        Self {
            netlist_first,
            facts,
            note,
            outputs,
        }
    }

    /// The band's fact under `label`.
    pub fn fact(&self, label: &str) -> Option<&SummaryFact> {
        self.facts.iter().find(|fact| fact.label == label)
    }
}

/// Whether a project is open: what checking its cells needs.
fn open_gate(cx: &Cx<'_, ProjectFeature>) -> (bool, Option<&'static str>) {
    let open = cx.platform.project_lifecycle.is_open();
    (open, (!open).then_some("no project is open"))
}

/// What the project reads, counted, and what of it is wrong, counted by
/// how it is wrong; its value opens the Dependencies page, on the first
/// thing wrong.
fn dependencies(ctx: &egui::Context, cx: &Cx<'_, ProjectFeature>) -> SummaryFact {
    let model = &*cx.model;
    let rows = dependency_rows(ctx, model);
    // A deck-driven project's own library holds only the empty top cell a
    // drawing would start from: it is not something the project reads.
    let placeholder = model
        .is_netlist_first_without_schematic()
        .then_some(model.workspace.content.project.root_library.as_str());
    let rows = rows
        .iter()
        .filter(|row| {
            row.library.kind != DependencyKind::Design
                || Some(row.library.name.as_str()) != placeholder
        })
        .collect::<Vec<_>>();
    let of_kind = |kind| rows.iter().filter(|row| row.library.kind == kind).count();
    // What the project reads, each kind it reads counted.
    let mut said = [
        (of_kind(DependencyKind::Design), "library", "libraries"),
        (
            health::model_file_count(ctx, model),
            "model file",
            "model files",
        ),
        (
            of_kind(DependencyKind::IncludePath),
            "include path",
            "include paths",
        ),
    ]
    .into_iter()
    .filter(|(count, _, _)| *count > 0)
    .map(|(count, one, many)| counted(count, one, many))
    .collect::<Vec<_>>();
    let wrong = rows
        .iter()
        .filter(|row| row.status.mark != Tone::Neutral)
        .copied()
        .collect::<Vec<_>>();
    let scan_errors = model.pdk_config.scan_errors.len();
    let mut tone = wrong
        .iter()
        .fold(Tone::Neutral, |tone, row| worse(tone, row.status.mark));
    if said.is_empty() && wrong.is_empty() && scan_errors == 0 {
        said.push("None".to_owned());
    } else if wrong.is_empty() && scan_errors == 0 {
        said.push("all found".to_owned());
    }
    // Each way a dependency is wrong, counted, in the order first met.
    let mut ways: Vec<(String, usize)> = Vec::new();
    for row in &wrong {
        let way = row.status.label.to_lowercase();
        match ways.iter_mut().find(|(said, _)| *said == way) {
            Some((_, count)) => *count += 1,
            None => ways.push((way, 1)),
        }
    }
    said.extend(
        ways.into_iter()
            .map(|(way, count)| format!("{count} {way}")),
    );
    if scan_errors > 0 {
        tone = worse(tone, Tone::Warn);
        said.push(counted(
            scan_errors,
            "model file not read",
            "model files not read",
        ));
    }
    let page = command_gate(cx, ProjectCommand::Page(ProjectPage::Dependencies).key());
    let first = wrong.first().map(|row| row.library.clone());
    SummaryFact::new("Dependencies", said.join(" \u{b7} "))
        .toned(tone)
        .linked(Some(Action::new(
            "Open in Dependencies",
            OverviewIntent::OpenDependency(first),
            page,
        )))
}

/// How the last checks of the cell view runs start from went. Checks not
/// run, or run on a drawing changed since, offer to run them; findings
/// open the Problems panel that lists them.
fn checks(cx: &Cx<'_, ProjectFeature>) -> SummaryFact {
    let model = &*cx.model;
    let facts = cx.runs.slot_facts(cx.peers.design().review_markers());
    let status = model.simulation_root_design_check_status(facts);
    let run_checks = Action::new(
        "Run checks",
        OverviewIntent::CheckSimulationRoot,
        open_gate(cx),
    );
    match &status {
        // What the last checks found was of a drawing since changed.
        DesignCheckStatus::Stale(_) => SummaryFact::new("Checks", "Out of date")
            .toned(Tone::Warn)
            .linked(Some(run_checks)),
        _ => {
            let fact = SummaryFact::of("Checks", health::schematic_checks(model, facts));
            let link = match status.last_receipt() {
                None => Some(run_checks),
                Some(receipt) if !receipt.result.violations().is_empty() => Some(Action::new(
                    "Show in Problems",
                    OverviewIntent::Command(ids::OPEN_PROBLEMS),
                    command_gate(cx, ids::OPEN_PROBLEMS),
                )),
                Some(_) => None,
            };
            fact.linked(link)
        }
    }
}

/// The newest run: how it went, when and for how long, which opens it in
/// Results; what it analysed; and the engine it ran on. Nothing before the
/// first run.
fn latest_run(cx: &Cx<'_, ProjectFeature>) -> Vec<SummaryFact> {
    let model = &*cx.model;
    let Some(run) = model.simulation.retained.runs.first() else {
        return Vec::new();
    };
    let now_ms = u64::try_from(crate::time_compat::unix_epoch().as_millis()).unwrap_or(u64::MAX);
    let mut ended = health::lifecycle_label(run.lifecycle, run.success).to_owned();
    if let Some(at) = health::unix_seconds_to_millis(run.timestamp) {
        ended = format!("{ended} {}", health::told(at, now_ms));
    }
    let said = [
        health::run_title(run.id, &run.label),
        ended,
        crate::quantity::format_elapsed(run.elapsed_time),
    ];
    let results = (!run.analyses.is_empty()).then(|| {
        Action::new(
            "Open in Results",
            OverviewIntent::OpenResults,
            command_gate(cx, ids::open_workspace(Workspace::Results)),
        )
    });
    let mut facts = vec![
        SummaryFact::new("Latest run", said.join(" \u{b7} "))
            .toned(health::lifecycle_tone(run.lifecycle, run.success))
            .linked(results),
    ];
    if !run.analyses.is_empty() {
        let analyses = run
            .analyses
            .iter()
            .map(analysis_words)
            .collect::<Vec<_>>()
            .join(" \u{b7} ");
        facts.push(SummaryFact::new("Analyses", analyses));
    }
    if let Some(target) = run.execution_target {
        facts.push(SummaryFact::new("Engine", target.label()));
    }
    facts
}

/// An analysis in a few words: its kind, where it swept, and over how many
/// points. "Transient to 1 ms · 1001 points", "Corner sweep · 7 corners".
fn analysis_words(analysis: &AnalysisResult) -> String {
    let mut words = sentence_case(analysis.analysis_type.display_name());
    if let Some(family) = family_words(analysis) {
        return format!("{words} \u{b7} {family}");
    }
    if let Some(sweep) = sweep(analysis) {
        words = format!("{words} {sweep}");
    }
    match points(analysis) {
        Some(points) if points > 1 => format!("{words} \u{b7} {points} points"),
        _ => words,
    }
}

/// `title` in sentence case: each word after the first lowered, but for a
/// word all in capitals, which is an initialism. "AC Analysis" reads
/// "AC analysis".
fn sentence_case(title: &str) -> String {
    title
        .split(' ')
        .enumerate()
        .map(|(index, word)| {
            let initialism = word.chars().all(|letter| !letter.is_lowercase());
            if index == 0 || initialism {
                word.to_owned()
            } else {
                word.to_lowercase()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// What a family swept, counted: its axis only numbers its members.
fn family_words(analysis: &AnalysisResult) -> Option<String> {
    use AnalysisResultFamilyMetadata as Family;
    Some(match analysis.family_metadata.as_ref()? {
        Family::Corner { corner_labels, .. } => counted(corner_labels.len(), "corner", "corners"),
        Family::MonteCarlo { runs_completed, .. } => counted(*runs_completed, "trial", "trials"),
        Family::Parametric {
            target,
            sweep_values,
            ..
        } => format!(
            "{target} {} \u{2013} {}, {}",
            format_value(*sweep_values.first()?, ""),
            format_value(*sweep_values.last()?, ""),
            counted(sweep_values.len(), "step", "steps")
        ),
        Family::Optimization { iterations, .. } => {
            counted(iterations.len(), "iteration", "iterations")
        }
        _ => return None,
    })
}

/// Where an analysis swept, in its coordinate's unit: from its first
/// coordinate to its last, or where a transient stopped.
fn sweep(analysis: &AnalysisResult) -> Option<String> {
    let (first, last) = analysis.waveforms.first()?.as_ref().x.ends()?;
    let unit = analysis.waveform_coordinate_unit().unwrap_or("");
    Some(if analysis.analysis_type == AnalysisType::Transient {
        format!("to {}", format_value(last, unit))
    } else {
        format!(
            "{} \u{2013} {}",
            format_value(first, unit),
            format_value(last, unit)
        )
    })
}

/// How many points an analysis solved: its sweep's, or the one point an
/// operating point is.
fn points(analysis: &AnalysisResult) -> Option<usize> {
    match analysis.waveforms.first() {
        Some(waveform) => Some(waveform.as_ref().x.len()),
        None => analysis.dc_op.as_ref().map(|_| 1),
    }
}

/// Verify's words for a verdict, and the tone it is inked in: only a fail
/// and an error are the failure, and a near pass the caution.
const fn verdict(status: RowStatus) -> (&'static str, Tone) {
    match status {
        RowStatus::Pass => ("Pass", Tone::Neutral),
        RowStatus::Near => ("Near", Tone::Warn),
        RowStatus::Fail => ("Fail", Tone::Error),
        RowStatus::Error => ("Error", Tone::Error),
        RowStatus::NotMeasured => ("Not measured", Tone::Neutral),
        RowStatus::Waived => ("Waived", Tone::Neutral),
        RowStatus::Tracked => ("Tracked", Tone::Neutral),
    }
}

/// What a spread's worst point is called, by what the run swept: a corner,
/// a Monte Carlo trial, or else a point of the sweep.
fn worst_head(run: Option<&SimulationRun>) -> &'static str {
    use AnalysisResultFamilyMetadata as Family;
    let family = run
        .into_iter()
        .flat_map(|run| &run.analyses)
        .find_map(|analysis| analysis.family_metadata.as_ref());
    match family {
        Some(Family::Corner { .. }) => "Worst corner",
        Some(Family::MonteCarlo { .. }) => "Worst trial",
        _ => "Worst point",
    }
}

/// The latest run's outputs: Verify's verdicts where the plan sets limits,
/// else what the run measured; and what the reader has to know first.
/// `deck` is the deck a deck-driven project runs, where its text is at hand.
fn outputs(
    cx: &Cx<'_, ProjectFeature>,
    deck: Option<Option<&DeckSummary>>,
) -> (Option<String>, Outputs) {
    let model = &*cx.model;
    let verify = command_gate(cx, ids::open_workspace(Workspace::Verify));
    let sheet = model
        .simulation
        .verification_sheet()
        .filter(|sheet| !sheet.rows.is_empty());
    let judged_run = model.simulation.active_run();
    if let Some(sheet) = sheet {
        let note = if !sheet.is_judged() {
            sheet.refusal().map(str::to_owned)
        } else if model.simulation.view.active_run_idx != Some(0) {
            judged_run.map(|run| format!("Judged on Run {}, the run Results has selected", run.id))
        } else {
            None
        };
        let rows = sheet
            .rows
            .iter()
            .map(|row| {
                let unit = row.unit.as_str();
                let worst = row.worst.as_ref();
                let value = worst.and_then(|worst| worst.value);
                let extremes = row.extremes();
                SpecificationLine {
                    output: row.name.clone(),
                    measurement: (!row.name.eq_ignore_ascii_case(&row.measurement))
                        .then(|| row.measurement.clone()),
                    value: value.map(|value| format_value(value, unit)),
                    min: extremes.map(|(least, _)| format_value(least, unit)),
                    max: extremes.map(|(_, most)| format_value(most, unit)),
                    worst_at: worst.and_then(|worst| sheet.worst_at(worst)),
                    limit: limit_text(&row.comparison, unit),
                    margin: value
                        .zip(worst.and_then(|worst| worst.signed_margin))
                        .map(|(value, margin)| difference_text(margin, value, unit)),
                    verdict: row.status.map(verdict),
                    action: Action::new(
                        "Show in Verify",
                        OverviewIntent::OpenVerify(Some(row.measurement.clone())),
                        verify,
                    ),
                }
            })
            .collect();
        // A run of several points, or of Monte Carlo trials, has a spread.
        let swept = sheet.points.len() > 1
            || sheet.rows.iter().any(|row| {
                row.statistics
                    .as_ref()
                    .is_some_and(|statistics| statistics.distribution.count > 1)
            });
        let spread = swept.then(|| worst_head(judged_run));
        return (note, Outputs::Judged { spread, rows });
    }
    let open_verify = Action::new("Open Verify", OverviewIntent::OpenVerify(None), verify);
    let Some(run) = judged_run.or_else(|| model.simulation.retained.runs.first()) else {
        let run = Action::new(
            "Run",
            OverviewIntent::Command(ids::RUN_SIMULATION),
            command_gate(cx, ids::RUN_SIMULATION),
        );
        let does = match deck {
            None => {
                "Run the plan to read its outputs against their specifications here.".to_owned()
            }
            Some(deck) => {
                let name = health::deck_name(model).unwrap_or_else(|| "the deck".to_owned());
                match deck {
                    Some(deck) if !deck.analyses.is_empty() => format!(
                        "Run simulates {name} as written ({}); what it measures is read here.",
                        deck.runs()
                    ),
                    _ => format!("Run simulates {name} as written; what it measures is read here."),
                }
            }
        };
        return (None, Outputs::NoRun(does, run));
    };
    // A plan that sets no limit: the run's measurements, as the console
    // spells them, each opening Verify, where a limit is added.
    let measured = run
        .analyses
        .iter()
        .flat_map(|analysis| &analysis.measurements)
        .map(|measurement| MeasurementLine {
            output: measurement.name.clone(),
            value: rspice_app_workbench::workbench::docks::console::measurement_table_row(
                measurement,
                "",
                None,
            )
            .value,
            action: open_verify.clone(),
        })
        .collect::<Vec<_>>();
    let outputs = if measured.is_empty() {
        Outputs::Unmeasured(open_verify)
    } else {
        Outputs::Measured(measured)
    };
    (None, outputs)
}

/// What the project's deck says of itself: the files its include lines name,
/// in the order it names them, the analyses a run of it runs, and how many
/// measurements it makes.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct DeckSummary {
    pub includes: Vec<String>,
    pub analyses: Vec<String>,
    pub measurements: usize,
}

impl DeckSummary {
    /// What a run of the deck runs, as one line: its analysis cards as
    /// written, then its measurements, counted.
    pub fn runs(&self) -> String {
        let mut said = self.analyses.clone();
        if said.is_empty() {
            said.push("No analysis card".to_owned());
        }
        if self.measurements > 0 {
            said.push(counted(self.measurements, "measurement", "measurements"));
        }
        said.join(" \u{b7} ")
    }
}

/// [`DeckSummary`] of the project's deck, where its text is at hand: a deck
/// the project keeps as text, or a deck file the Netlist workspace has
/// open. `None` where the text is not at hand. Read again only when the
/// text changes.
pub fn deck_summary(
    ctx: &egui::Context,
    model: &Model,
    code: &dyn CodePort,
) -> Option<Arc<DeckSummary>> {
    let deck = model.workspace.content.decks.first()?;
    let text = match &deck.source {
        crate::state::DeckSource::Stored(stored) => DeckIncludesText::Kept(&stored.text),
        crate::state::DeckSource::File { .. } => {
            let open = code.manual_deck()?;
            let rspice_app_runs::simulation::DeckOrigin::File(path) = open.origin else {
                return None;
            };
            let folder = model
                .workspace
                .content
                .project
                .path
                .as_deref()
                .and_then(std::path::Path::parent);
            if !deck.is_file(path, folder) {
                return None;
            }
            DeckIncludesText::Open(open.text)
        }
    };
    let mut identity = DefaultHasher::new();
    text.identity().hash(&mut identity);
    Some(derived(
        ctx,
        "overview.deck summary",
        identity.finish(),
        || {
            let text = text.as_str();
            let index = rspice_design::netlist_document::NetlistSourceIndex::parse(text);
            let entries = index.outline().entries();
            let of = |kind| {
                entries
                    .into_iter()
                    .filter(move |entry| entry.kind() == kind)
            };
            Arc::new(DeckSummary {
                includes: rspice_design::netlist_document::parse_include_directives(text)
                    .into_iter()
                    .map(|include| include.locator().to_owned())
                    .collect(),
                analyses: of(rspice_design::netlist_document::OutlineEntryKind::Analysis)
                    .map(|entry| match entry.detail() {
                        Some(detail) => format!("{} {detail}", entry.name()),
                        None => entry.name().into_owned(),
                    })
                    .collect(),
                measurements: of(rspice_design::netlist_document::OutlineEntryKind::Measurement)
                    .count(),
            })
        },
    ))
}

/// The text [`deck_summary`] reads: the deck the project keeps, or the
/// deck file open in the Netlist workspace. Each is told apart from the
/// text before it by its identity, so a frame reads no text unless the
/// text changed.
#[derive(Clone, Copy)]
enum DeckIncludesText<'a> {
    Kept(&'a crate::state::DeckText),
    Open(&'a crate::state::RevisedText),
}

impl DeckIncludesText<'_> {
    fn identity(self) -> (u8, u64) {
        match self {
            Self::Kept(text) => (0, text.identity()),
            Self::Open(text) => (1, text.revision()),
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Self::Kept(text) => text.as_str(),
            Self::Open(text) => text.as_str(),
        }
    }
}
