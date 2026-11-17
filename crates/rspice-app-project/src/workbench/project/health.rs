//! Where each subject of the project stands, one function per subject.
//!
//! The Overview's status rows, the health chips over the other Project
//! pages, the inspector and the status bar each state some of these: the
//! checks of the cell view runs start from, the deck a netlist-first project
//! runs, the configuration runs use, the latest run, what Verify judged of
//! it, the model sources, the libraries the project depends on, and the work
//! not yet saved. Every one of them reads its words and its mark from here,
//! so no two places can disagree about a subject.
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
use crate::ui::accessibility::{counted, finding_counts};
use crate::workbench::app::dialogs::project::ProjectCheckpointRecoveryState;
use crate::workbench::app_state::{DesignCheckStatus, Model, PlatformState, SlotFacts};
use crate::workbench::project::dependency_status::{
    dependency_rows, shown_path, technology_status,
};
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

/// The subject that says whether the design itself is sound: the schematic
/// checks of the cell view runs start from, or the deck a netlist-first
/// project runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesignSubject {
    SchematicChecks,
    SourceDeck,
}

impl DesignSubject {
    pub fn of(model: &Model) -> Self {
        if model.is_netlist_first_without_schematic() {
            Self::SourceDeck
        } else {
            Self::SchematicChecks
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::SchematicChecks => "Schematic checks",
            Self::SourceDeck => "Source deck",
        }
    }

    /// The word a chip puts in front of the state.
    pub const fn chip_label(self) -> &'static str {
        match self {
            Self::SchematicChecks => "Checks",
            Self::SourceDeck => "Deck",
        }
    }
}

/// The design subject the project has, and where it stands.
pub fn design(model: &Model, facts: SlotFacts) -> (DesignSubject, SubjectStatus) {
    let subject = DesignSubject::of(model);
    let status = match subject {
        DesignSubject::SchematicChecks => schematic_checks(model, facts),
        DesignSubject::SourceDeck => source_deck(model),
    };
    (subject, status)
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
            let advisories = summary.warnings + summary.info;
            let state = if errors + advisories == 0 {
                "Passed".to_owned()
            } else {
                finding_counts(errors, advisories)
            };
            let tone = if errors > 0 {
                Tone::Error
            } else {
                Tone::Neutral
            };
            SubjectStatus::new(tone, state, root)
        }
        DesignCheckStatus::NotRun => SubjectStatus::neutral("Not run", root),
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

/// The name a netlist-first project's deck goes by.
pub fn deck_name(model: &Model) -> Option<String> {
    let document = model.workspace.content.netlist_document.as_ref()?;
    Some(
        document
            .provenance()
            .imported()
            .map(|imported| imported.origin().display_name().to_owned())
            .or_else(|| {
                document
                    .save_acknowledgement()
                    .map(|saved| saved.origin().display_name().to_owned())
            })
            .or_else(|| {
                model
                    .workspace
                    .content
                    .netlist_source_path
                    .as_ref()
                    .and_then(|path| path.file_name())
                    .map(|name| name.to_string_lossy().into_owned())
            })
            .unwrap_or_else(|| "Untitled SPICE deck".to_owned()),
    )
}

/// The deck a netlist-first project runs, as its last validation found it.
pub fn source_deck(model: &Model) -> SubjectStatus {
    let (Some(document), Some(name)) = (
        model.workspace.content.netlist_document.as_ref(),
        deck_name(model),
    ) else {
        return SubjectStatus::new(Tone::Error, "Unavailable", "The project has no SPICE deck");
    };
    match document.validation() {
        None => SubjectStatus::neutral("Not validated", name),
        Some(report) if report.content_digest() != document.content_digest() => SubjectStatus::new(
            Tone::Warn,
            "Out of date",
            format!("{name} changed after it was validated"),
        ),
        Some(report) => {
            let errors = report.error_count();
            let advisories = report.diagnostics().len().saturating_sub(errors);
            let state = if errors + advisories == 0 {
                "Validated".to_owned()
            } else {
                finding_counts(errors, advisories)
            };
            let tone = if errors > 0 {
                Tone::Error
            } else {
                Tone::Neutral
            };
            SubjectStatus::new(tone, state, name)
        }
    }
}

/// The configuration runs use, and whether a run can be netlisted through
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
    let active = catalog.active();
    if let Err(error) = validity {
        return SubjectStatus::new(
            Tone::Error,
            active.map_or("Default", |configuration| configuration.name()),
            format!("The configurations cannot be read: {error}"),
        );
    }
    let Some(active) = active else {
        // With none used for runs the project runs Default: its own top cell
        // through the built-in view list.
        return SubjectStatus::neutral("Default", "Runs use the built-in view list");
    };
    let projection = model.workspace.configuration_execution_projection(
        &model.library_manager,
        &model.workspace.content.active_view,
        &model.schematic,
    );
    match projection {
        Ok(projection) => SubjectStatus::neutral(
            active.name(),
            format!(
                "{} \u{b7} {}",
                active.root().display_path(),
                counted(
                    projection.plan().bindings().len(),
                    "instance bound",
                    "instances bound"
                )
            ),
        ),
        Err(error) => SubjectStatus::new(
            Tone::Error,
            active.name(),
            format!("Runs are blocked: {error}"),
        ),
    }
}

/// The newest run the project keeps.
pub fn latest_run(model: &Model) -> SubjectStatus {
    let Some(run) = model.simulation.retained.runs.first() else {
        let detail = if model.is_netlist_first_without_schematic() {
            "Run the deck to produce results"
        } else {
            "Run the active plan to produce results"
        };
        return SubjectStatus::neutral("No runs", detail);
    };
    let measurements: usize = run
        .analyses
        .iter()
        .map(|analysis| analysis.measurements.len())
        .sum();
    SubjectStatus::new(
        lifecycle_tone(run.lifecycle, run.success),
        format!(
            "Run {} \u{b7} {}",
            run.id,
            lifecycle_label(run.lifecycle, run.success)
        ),
        format!(
            "{} \u{b7} {} \u{b7} {}",
            counted(run.analyses.len(), "analysis", "analyses"),
            counted(measurements, "measurement", "measurements"),
            run.execution_target
                .map_or("engine not recorded", |target| target.label()),
        ),
    )
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

/// What Verify judged of the selected run, read from the sheet it builds
/// once per change.
pub fn verification(model: &Model) -> SubjectStatus {
    let sheet = model.simulation.verification_sheet();
    let specifications = sheet.map_or(0, |sheet| sheet.rows.len());
    if specifications == 0 {
        return SubjectStatus::neutral("No specifications", "The active plan sets no limits");
    }
    let Some(words) = verification_words(model) else {
        let detail = sheet.and_then(|sheet| sheet.refusal()).map_or_else(
            || counted(specifications, "specification", "specifications"),
            str::to_owned,
        );
        return SubjectStatus::neutral("Not judged", detail);
    };
    let counts = sheet.map(|sheet| sheet.summary.counts).unwrap_or_default();
    let tone = if counts.fail + counts.error > 0 {
        Tone::Error
    } else {
        Tone::Neutral
    };
    let run = model
        .simulation
        .active_run()
        .map_or_else(String::new, |run| format!("Run {}", run.id));
    SubjectStatus::new(tone, words, run)
}

/// What the model catalogue holds, as its last scan found it. Whether each
/// model library's files are still there is the library's own status, which
/// [`dependencies`] states.
pub fn model_sources(ctx: &egui::Context, model: &Model) -> SubjectStatus {
    let manager = &model.model_library_manager;
    let (libraries, models, sources) =
        derived(ctx, "health.model sources", manager.content_stamp(), || {
            let libraries = manager.libraries_sorted();
            let sources = libraries
                .iter()
                .flat_map(|library| library.source_closure.iter().map(|pin| &pin.path))
                .collect::<std::collections::BTreeSet<_>>()
                .len();
            (
                manager.library_count(),
                manager.total_model_count(),
                sources,
            )
        });
    let scan_errors = &model.pdk_config.scan_errors;
    if let Some(first) = scan_errors.first() {
        return SubjectStatus::new(
            Tone::Warn,
            counted(scan_errors.len(), "file not read", "files not read"),
            first.clone(),
        );
    }
    if libraries == 0 {
        return SubjectStatus::neutral("No model libraries", "Runs read only built-in models");
    }
    SubjectStatus::neutral(
        counted(models, "model", "models"),
        format!(
            "{} \u{b7} {}",
            counted(libraries, "library", "libraries"),
            counted(sources, "source file", "source files")
        ),
    )
}

/// The libraries the project depends on and its technology, each as the
/// Dependencies page states it.
pub fn dependencies(ctx: &egui::Context, model: &Model) -> SubjectStatus {
    let rows = dependency_rows(ctx, model);
    let technology = technology_status(model);
    let mut flagged = rows
        .iter()
        .filter(|row| row.status.mark != Tone::Neutral)
        .map(|row| (row.library.name.as_str(), &row.status))
        .collect::<Vec<_>>();
    if technology.mark != Tone::Neutral {
        flagged.push(("Technology", &technology));
    }
    let tone = flagged
        .iter()
        .map(|(_, status)| status.mark)
        .fold(Tone::Neutral, worse);
    match flagged.as_slice() {
        [] => SubjectStatus::neutral(
            counted(rows.len(), "library", "libraries"),
            model
                .workspace
                .content
                .project
                .technology_binding()
                .map_or_else(
                    || "No technology".to_owned(),
                    |binding| format!("Technology {}", binding.display_label()),
                ),
        ),
        [(name, status)] => SubjectStatus::new(
            tone,
            format!("{name}: {}", status.label.to_lowercase()),
            status.detail.clone().unwrap_or_default(),
        ),
        [(name, status), ..] => SubjectStatus::new(
            tone,
            format!("{} need attention", flagged.len()),
            format!("{name}: {}", status.label.to_lowercase()),
        ),
    }
}

/// The work a save would write, and what the Recovery page can bring back.
/// The checkpoints are counted once the page's catalogue has listed them;
/// a store that could not be listed, or holds damaged checkpoints, is the
/// caution.
pub fn unsaved_work(
    unsaved_documents: usize,
    saved: bool,
    recovery: &ProjectCheckpointRecoveryState,
) -> SubjectStatus {
    let state = if !saved {
        "Not saved".to_owned()
    } else if unsaved_documents == 0 {
        "Saved".to_owned()
    } else {
        counted(unsaved_documents, "unsaved document", "unsaved documents")
    };
    if let Some(error) = &recovery.error {
        return SubjectStatus::new(
            Tone::Warn,
            state,
            format!("Checkpoints cannot be listed: {error}"),
        );
    }
    if !recovery.quarantined.is_empty() {
        return SubjectStatus::new(
            Tone::Warn,
            state,
            counted(
                recovery.quarantined.len(),
                "damaged checkpoint",
                "damaged checkpoints",
            ),
        );
    }
    let detail = if recovery.initialized {
        counted(recovery.checkpoints.len(), "checkpoint", "checkpoints")
    } else {
        String::new()
    };
    SubjectStatus::neutral(state, detail)
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
