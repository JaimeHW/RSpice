//! Where each subject of the project stands, one function per subject.
//!
//! The Overview's band, the inspector and the status bar each state some
//! of these: the configuration runs use, the technology, the model files
//! the project reads, its deck, the checks of the cell view runs start
//! from, how the latest run went, what Verify judged of it, and the work a
//! save would write. Every one of them reads its words and its mark from here, so no
//! two places can disagree about a subject.
//!
//! One rule for the mark: a subject that has not been run yet is neutral, a
//! result the design has moved on from is the caution, and only what stops
//! a run or fails a limit is the failure. A new project opens with nothing
//! marked.
//!
//! Nothing here walks the design or a catalogue on a frame: what has to be
//! derived is kept in the frame context beside the identity it was derived
//! from, as the Project pages keep theirs.

use crate::state::SimulationRunLifecycle;
use crate::ui::accessibility::{counted, finding_counts_and_notes};
use crate::workbench::app_state::{DesignCheckStatus, Model, PlatformState, SlotFacts};
use crate::workbench::lifecycle::project_lifecycle::{
    LifecycleView, ProjectDocumentId, dirty_documents,
};
use crate::workbench::project::dependency_status::{shown_path, technology_status};
use crate::workbench::surfaces::project::derived::derived;
use crate::workbench::surfaces::project::overview::Tone;

/// Where one subject stands: the mark it earns, its state in a word or a
/// count, and the one line the reader needs beside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubjectStatus {
    pub tone: Tone,
    pub state: String,
    pub detail: String,
}

impl SubjectStatus {
    fn new(tone: Tone, state: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            tone,
            state: state.into(),
            detail: detail.into(),
        }
    }

    fn neutral(state: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(Tone::Neutral, state, detail)
    }
}

/// The checks of the cell view runs start from: the configuration's root
/// when one is used for runs, else the project's top schematic.
pub fn schematic_checks(model: &Model, facts: SlotFacts) -> SubjectStatus {
    let root = model
        .workspace
        .content
        .simulation_root_reference()
        .display_path();
    match model.simulation_root_design_check_status(facts) {
        DesignCheckStatus::Current(receipt) => {
            let summary = receipt.result.summary();
            let errors = summary.critical + summary.errors;
            let state = if errors + summary.warnings + summary.info == 0 {
                "Passed".to_owned()
            } else {
                finding_counts_and_notes(errors, summary.warnings, summary.info)
            };
            let tone = if errors > 0 {
                Tone::Error
            } else {
                Tone::Neutral
            };
            SubjectStatus::new(tone, state, "")
        }
        DesignCheckStatus::NotRun => SubjectStatus::neutral("Not run", ""),
        DesignCheckStatus::Stale(_) => SubjectStatus::new(
            Tone::Warn,
            "Out of date",
            format!("{root} changed after it was checked"),
        ),
        DesignCheckStatus::Unavailable { reason, .. } => {
            SubjectStatus::new(Tone::Error, "Unavailable", reason)
        }
    }
}

/// The name a netlist-first project's deck goes by: its file's name.
pub fn deck_name(model: &Model) -> Option<String> {
    model
        .workspace
        .content
        .decks
        .first()
        .map(|deck| deck.name())
}

/// The project's deck: its name, and how many files its include lines name
/// where its text is at hand (`None` where it is not). A netlist-first
/// project with no deck has nothing to run.
pub fn source_deck(model: &Model, includes: Option<usize>) -> SubjectStatus {
    let Some(name) = deck_name(model) else {
        return SubjectStatus::new(Tone::Error, "Unavailable", "The project has no deck");
    };
    SubjectStatus::neutral(
        name,
        match includes {
            None => String::new(),
            Some(0) => "no includes".to_owned(),
            Some(includes) => counted(includes, "include", "includes"),
        },
    )
}

/// The configuration the next run uses: the one the active plan names, else
/// the one the project marks for runs, as the Analyses page's select and
/// the run's receipt name it; and whether a run can be netlisted through
/// it. Proving every configuration's digest is done once per edit of the
/// catalogue, not on every frame.
pub fn configuration(ctx: &egui::Context, model: &Model) -> SubjectStatus {
    let catalog = &model.workspace.content.configuration_sets;
    let validity = derived(
        ctx,
        "health.configuration validity",
        catalog.stamp(),
        || catalog.validate().map_err(|error| error.to_string()),
    );
    if let Err(error) = validity {
        return SubjectStatus::new(
            Tone::Error,
            catalog
                .active()
                .map_or("Default", |configuration| configuration.name()),
            format!("The configurations cannot be read: {error}"),
        );
    }
    let plan = match model
        .workspace
        .plan_configuration_sets(model.sim_setup.design_configuration)
    {
        Ok(plan) => plan,
        Err(error) => {
            return SubjectStatus::new(
                Tone::Error,
                "Deleted configuration",
                format!("Runs are blocked: {error}"),
            );
        }
    };
    let Some(active) = plan.active() else {
        // With none named or used for runs the plan runs Default: the
        // project's own top cell through the built-in view list.
        return SubjectStatus::neutral("Default", "Runs use the built-in view list");
    };
    let projection = model.plan_execution_projection();
    match projection {
        // The Testbench row names the root it binds from.
        Ok(projection) => SubjectStatus::neutral(
            active.name(),
            counted(
                projection.plan().bindings().len(),
                "instance bound",
                "instances bound",
            ),
        ),
        // A library that could not be read is named as what is missing,
        // in the one sentence every place says it in.
        Err(rspice_design::projection::ConfigurationExecutionPlanError::LibraryUnavailable(
            sentence,
        )) => SubjectStatus::new(Tone::Error, active.name(), sentence),
        Err(error) => SubjectStatus::new(
            Tone::Error,
            active.name(),
            format!("Runs are blocked: {error}"),
        ),
    }
}

/// A day, in milliseconds: inside one an age is told, past one a date.
const DAY_MS: u64 = 86_400_000;

/// How far ahead of the clock a time may read and still be now: a run
/// started this instant is stamped to the nearest millisecond, which can
/// round past the clock read after it.
const AHEAD_MS: u64 = 1_000;

/// A time as a row says it: how long ago inside a day, the date and time
/// past one or ahead of the clock.
pub fn told(at_ms: u64, now_ms: u64) -> String {
    match now_ms.checked_sub(at_ms) {
        Some(ago) if ago < DAY_MS => {
            crate::time_compat::relative_age(std::time::Duration::from_millis(ago))
        }
        None if at_ms - now_ms <= AHEAD_MS => {
            crate::time_compat::relative_age(std::time::Duration::ZERO)
        }
        _ => crate::time_compat::local_stamp(at_ms),
    }
}

/// A run's start in seconds since the epoch as milliseconds; `None` for a
/// time no clock could have read.
pub fn unix_seconds_to_millis(seconds: f64) -> Option<u64> {
    if !seconds.is_finite() || seconds < 0.0 || seconds > u64::MAX as f64 / 1_000.0 {
        return None;
    }
    Some((seconds * 1_000.0).round() as u64)
}

/// A run as a list names it: "Run 12", with its name where it was given
/// one, and without the start time its label ends with, which is stated
/// beside it.
pub fn run_title(sequence: u64, label: &str) -> String {
    let prefix = format!("Run {sequence}");
    let name = without_start_stamp(label);
    if name.starts_with(&prefix) {
        name.to_owned()
    } else {
        format!("{prefix} \u{b7} {name}")
    }
}

/// `label` without the start time a run's label ends with: " (1:04:01 PM)",
/// or " (21 Nov 2026, 1:04:01 PM)" with its date.
fn without_start_stamp(label: &str) -> &str {
    let Some((name, stamp)) = label
        .strip_suffix(')')
        .and_then(|rest| rest.rsplit_once(" ("))
    else {
        return label;
    };
    let Some(stamp) = stamp
        .strip_suffix(" AM")
        .or_else(|| stamp.strip_suffix(" PM"))
    else {
        return label;
    };
    let (date, time) = match stamp.rsplit_once(", ") {
        Some((date, time)) => (Some(date), time),
        None => (None, stamp),
    };
    let digits = |text: &str| !text.is_empty() && text.chars().all(|c| c.is_ascii_digit());
    let clock = !time.is_empty() && time.split(':').all(digits);
    let dated = date.is_none_or(|date| {
        let parts = date.split(' ').collect::<Vec<_>>();
        matches!(parts.as_slice(), [day, month, year]
            if digits(day) && digits(year) && month.chars().all(|c| c.is_ascii_alphabetic()))
    });
    if clock && dated { name } else { label }
}

/// What a run's lifecycle reads as.
pub const fn lifecycle_label(lifecycle: SimulationRunLifecycle, success: bool) -> &'static str {
    match lifecycle {
        SimulationRunLifecycle::LegacyUnknown => "outcome not recorded",
        SimulationRunLifecycle::Preparing => "preparing",
        SimulationRunLifecycle::Running => "running",
        SimulationRunLifecycle::Cancelling => "cancelling",
        SimulationRunLifecycle::Completed if success => "completed",
        SimulationRunLifecycle::Completed => "completed with failures",
        SimulationRunLifecycle::Failed => "failed",
        SimulationRunLifecycle::Aborted => "aborted",
        SimulationRunLifecycle::Interrupted => "interrupted",
    }
}

/// The mark a run's lifecycle earns: a run stopped short is the caution, a
/// run that failed is the failure.
pub const fn lifecycle_tone(lifecycle: SimulationRunLifecycle, success: bool) -> Tone {
    match lifecycle {
        SimulationRunLifecycle::Completed if success => Tone::Neutral,
        SimulationRunLifecycle::Preparing
        | SimulationRunLifecycle::Running
        | SimulationRunLifecycle::LegacyUnknown => Tone::Neutral,
        SimulationRunLifecycle::Cancelling | SimulationRunLifecycle::Aborted => Tone::Warn,
        SimulationRunLifecycle::Completed
        | SimulationRunLifecycle::Failed
        | SimulationRunLifecycle::Interrupted => Tone::Error,
    }
}

/// "3 of 4 specifications pass · 1 fails": the counts of the sheet Verify
/// judged the selected run against the active plan's limits by, with the
/// near ones passing; `None` while it judges no specification.
pub fn verification_words(model: &Model) -> Option<String> {
    let sheet = model.simulation.verification_sheet()?;
    if !sheet.is_judged() || sheet.rows.is_empty() {
        return None;
    }
    let counts = sheet.summary.counts;
    let passing = counts.pass + counts.near;
    let mut parts = vec![match sheet.rows.len() {
        1 => format!("{passing} of 1 specification passes"),
        total => format!("{passing} of {total} specifications pass"),
    }];
    if counts.fail > 0 {
        parts.push(format!(
            "{} {}",
            counts.fail,
            if counts.fail == 1 { "fails" } else { "fail" }
        ));
    }
    if counts.error > 0 {
        parts.push(counted(counts.error, "error", "errors"));
    }
    Some(parts.join(" \u{b7} "))
}

/// The project's technology, as the Dependencies page states it: `None`
/// for a project with none that needs none, which is the ordinary case.
pub fn technology(model: &Model) -> Option<SubjectStatus> {
    let status = technology_status(model);
    let binding = model.workspace.content.project.technology_binding();
    if status.mark != Tone::Neutral {
        return Some(SubjectStatus::new(
            status.mark,
            status.label,
            status.detail.unwrap_or_default(),
        ));
    }
    let binding = binding?;
    Some(SubjectStatus::neutral(
        binding.display_label(),
        binding.pdk_package().map_or_else(
            || binding.model_library().to_owned(),
            |pin| format!("PDK {} {}", pin.package_id(), pin.revision()),
        ),
    ))
}

/// How many model files the model catalogue reads, by their paths, counted
/// once per change of the catalogue.
pub fn model_file_count(ctx: &egui::Context, model: &Model) -> usize {
    let manager = &model.model_library_manager;
    derived(ctx, "health.model files", manager.content_stamp(), || {
        manager
            .libraries_sorted()
            .iter()
            .flat_map(|library| library.source_closure.iter().map(|pin| &pin.path))
            .collect::<std::collections::BTreeSet<_>>()
            .len()
    })
}

/// The documents a save would write, each as the reader knows it: the
/// project file's documents with unsaved changes, and each cell view of a
/// library kept in a folder of its own with edits here, which that file
/// does not hold.
pub fn unsaved_documents(model: &Model, lifecycle: &LifecycleView<'_>) -> Vec<String> {
    let dirty = dirty_documents(lifecycle);
    let mut documents = dirty.iter().map(document_name).collect::<Vec<_>>();
    documents.extend(
        model
            .unsaved_library_cellviews()
            .into_iter()
            .filter(|reference| !dirty.contains(&ProjectDocumentId::CellView(reference.clone())))
            .map(|reference| reference.display_path()),
    );
    documents
}

/// A project document as the reader knows it.
fn document_name(document: &ProjectDocumentId) -> String {
    match document {
        ProjectDocumentId::ProjectConfiguration => "Project settings".to_owned(),
        ProjectDocumentId::CellView(reference) => reference.display_path(),
        ProjectDocumentId::SimulationPlan => "Simulation plans".to_owned(),
        ProjectDocumentId::ResultHistory => "Run history".to_owned(),
        ProjectDocumentId::ModelCatalog => "Model catalog".to_owned(),
        ProjectDocumentId::NetlistSource => "Source deck".to_owned(),
        ProjectDocumentId::StimulusLibrary => "Stimulus library".to_owned(),
    }
}

/// Where the project file is, as the lifecycle bound it: the file a desktop
/// save wrote, or the browser storage a browser save wrote. `None` before
/// the project is first saved. A browser save clears the file path, so the
/// path alone would read "not saved" for a project that is. A desktop path
/// is named as the reader knows it, without the verbatim prefix Windows
/// adds to a canonical one.
pub fn location(model: &Model, platform: &PlatformState) -> Option<String> {
    let Some(binding) = platform.project_lifecycle.binding() else {
        return model
            .workspace
            .content
            .project
            .path
            .as_deref()
            .map(shown_path);
    };

    #[cfg(not(target_arch = "wasm32"))]
    {
        Some(shown_path(&binding.canonical_path))
    }
    #[cfg(target_arch = "wasm32")]
    {
        match binding {
            crate::workbench::lifecycle::project_lifecycle::PersistenceBinding::Browser {
                binding,
                ..
            } => Some(
                if binding.persisted_generation == Some(binding.receipt.accepted_generation) {
                    binding.display_name.clone()
                } else {
                    format!("{} \u{b7} this session only", binding.display_name)
                },
            ),
        }
    }
}

/// The worse of two marks.
pub const fn worse(left: Tone, right: Tone) -> Tone {
    match (left, right) {
        (Tone::Error, _) | (_, Tone::Error) => Tone::Error,
        (Tone::Warn, _) | (_, Tone::Warn) => Tone::Warn,
        (Tone::Neutral, Tone::Neutral) => Tone::Neutral,
    }
}
