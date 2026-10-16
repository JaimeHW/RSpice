//! Adapters for the producers that cannot publish for themselves.
//!
//! The schematic rule checks publish beside their own receipt, and the two
//! source editors keep canonical collections the store reads in place. These
//! two cannot: the simulation controller sits below the shell and must not
//! reach up into it for a record type, and the dispatch checks are a workflow
//! that ran and then finished. Both hand the facts to the session, which is
//! where the records are built.

use rspice_app_types::diagnostics::CanonicalDiagnosticMetadata;

use crate::diagnostics::LogAnchor;
use crate::state::ConvergenceAttribution;
use crate::workbench::state::{
    ModelsPage, PreflightAdvisory, PreflightIssue, PreflightRemediation, PreflightReport,
};

use super::super::AppState;
use super::{
    DiagnosticInputs, DiagnosticLevel, DiagnosticRecord, DiagnosticRoute, DiagnosticSubject,
};

/// The engine, when it could say which objects a solve gave up on.
const CONVERGENCE_PRODUCER: &str = "rspice.engine.convergence";
/// The run itself, for a failure the engine attributed to nothing.
const RUN_PRODUCER: &str = "rspice.simulation.run";
/// The checks a run has to pass before it can be dispatched.
const PREFLIGHT_PRODUCER: &str = "rspice.simulation.preflight";
/// Where a model or technology binding failure is repaired.
const MODEL_BINDING_PRODUCER: &str = "rspice.models.binding";
/// The deck the run is generated from.
const NETLIST_PRODUCER: &str = "rspice.netlist.generator";

impl AppState {
    /// One record for a failed analysis, attributed to the objects the engine
    /// named where it named any.
    ///
    /// The Console still records the failure, because a run that failed is an
    /// event in the session and the Console is the session's log. This is the
    /// other statement: the design does not simulate, which is a condition,
    /// and it stands until the inputs it was measured against move.
    pub(crate) fn publish_run_failure(
        &mut self,
        analysis: &str,
        message: &str,
        attribution: Option<&ConvergenceAttribution>,
        run_sequence: u64,
    ) {
        let attribution = attribution.filter(|attribution| attribution.describes(message));
        let code = attribution.map_or_else(
            || "RUN-FAILED".to_owned(),
            |attribution| screaming_code("RUN", attribution.class.headline()),
        );
        let producer = if attribution.is_some() {
            CONVERGENCE_PRODUCER
        } else {
            RUN_PRODUCER
        };
        let canonical = CanonicalDiagnosticMetadata::current(
            producer,
            code,
            format!("run/{run_sequence}"),
            None,
            run_sequence,
            format!("run:{run_sequence}:{analysis}"),
            message,
        );
        let route = attribution.and_then(|attribution| {
            let nets: Vec<String> = attribution.nets().map(str::to_owned).collect();
            let devices: Vec<String> = attribution.devices().map(str::to_owned).collect();
            (!nets.is_empty() || !devices.is_empty()).then_some(DiagnosticRoute::Anchor(
                LogAnchor::Simulation { nets, devices },
            ))
        });
        let record = DiagnosticRecord::new(
            canonical,
            DiagnosticLevel::Error,
            DiagnosticSubject::new(
                self.workspace.content.project.id(),
                format!("{analysis} · run {run_sequence}"),
            ),
            message,
        )
        .with_details(attribution.map_or_else(String::new, ConvergenceAttribution::summary))
        .with_route(route);
        self.publish_or_state_refusal(DiagnosticInputs::Run { run_sequence }, vec![record]);
    }

    /// Everything the dispatch checks found, as records.
    ///
    /// A blocker is an error whose route is the destination it named for
    /// itself; an advisory is a warning the run proceeds through. Both come
    /// from one report, so both expire together when the design, the plan or
    /// the bindings move — which is exactly what the report's own currency
    /// rule already said and what no surface but the report could state.
    pub(in crate::workbench) fn publish_dispatch_diagnostics(&mut self, report: &PreflightReport) {
        let project = self.workspace.content.project.id();
        let revision = report.project_revision;
        let mut records = Vec::with_capacity(report.blockers.len() + report.advisories.len());
        for (ordinal, issue) in report.blockers.iter().enumerate() {
            records.push(blocker_record(project, revision, ordinal, issue));
        }
        for (ordinal, advisory) in report.advisories.iter().enumerate() {
            records.push(advisory_record(project, revision, ordinal, advisory));
        }
        self.publish_or_state_refusal(DiagnosticInputs::Dispatch, records);
    }

    /// Publish, and say so in the Console if the store refuses.
    ///
    /// A refusal here is a defect in the producer — an identity minted twice,
    /// or inputs that cannot be digested — and it costs the reader the
    /// findings. Swallowing it would leave an empty Problems page as the only
    /// symptom of a bug in the thing that fills it.
    fn publish_or_state_refusal(
        &mut self,
        inputs: DiagnosticInputs,
        records: Vec<DiagnosticRecord>,
    ) {
        if let Err(error) = self.publish_diagnostics(inputs, records) {
            self.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                "Findings could not be recorded: {error}"
            )));
        }
    }
}

fn blocker_record(
    project: crate::product::ProjectId,
    revision: u64,
    ordinal: usize,
    issue: &PreflightIssue,
) -> DiagnosticRecord {
    let (producer, family) = remediation_producer(&issue.remediation);
    let canonical = CanonicalDiagnosticMetadata::current(
        producer,
        screaming_code(family, &issue.check),
        "simulation-dispatch",
        None,
        revision,
        format!("dispatch:{revision}:blocker:{ordinal}"),
        &issue.observed,
    );
    DiagnosticRecord::new(
        canonical,
        DiagnosticLevel::Error,
        DiagnosticSubject::new(project, remediation_subject(&issue.remediation)),
        issue.observed.clone(),
    )
    .with_details(format!("{} · required: {}", issue.check, issue.required))
    // A dispatch check states what it requires of every run, which is the
    // nearest thing it has to rule prose and is not the same sentence as what
    // it observed about this one.
    .with_rule(issue.required.clone())
    .with_route(Some(DiagnosticRoute::Repair(issue.remediation.clone())))
}

fn advisory_record(
    project: crate::product::ProjectId,
    revision: u64,
    ordinal: usize,
    advisory: &PreflightAdvisory,
) -> DiagnosticRecord {
    let remediation = advisory.remediation.as_ref();
    let producer = remediation.map_or(PREFLIGHT_PRODUCER, |remediation| {
        remediation_producer(remediation).0
    });
    let canonical = CanonicalDiagnosticMetadata::current(
        producer,
        "RUN-ADVISORY",
        "simulation-dispatch",
        None,
        revision,
        format!("dispatch:{revision}:advisory:{ordinal}"),
        &advisory.message,
    );
    DiagnosticRecord::new(
        canonical,
        DiagnosticLevel::Warning,
        DiagnosticSubject::new(
            project,
            remediation.map_or_else(|| "Simulation dispatch".to_owned(), remediation_subject),
        ),
        advisory.message.clone(),
    )
    .with_route(remediation.map(|remediation| DiagnosticRoute::Repair(remediation.clone())))
}

/// Who reported a dispatch finding, and the family its rule code belongs to.
///
/// A dispatch blocker is not always the dispatch checks' own finding: a
/// corner with no section is the model bindings' finding and a deck that
/// will not parse is the generator's. Naming the real producer is what lets
/// the grid group by it and the rule deck address it.
const fn remediation_producer(remediation: &PreflightRemediation) -> (&'static str, &'static str) {
    match remediation {
        PreflightRemediation::Models { .. } => (MODEL_BINDING_PRODUCER, "MODEL"),
        PreflightRemediation::ProjectTechnology => (MODEL_BINDING_PRODUCER, "TECH"),
        PreflightRemediation::NetlistSource { .. } => (NETLIST_PRODUCER, "NETLIST"),
        PreflightRemediation::DesignChecks | PreflightRemediation::SimulationPlan => {
            (PREFLIGHT_PRODUCER, "RUN")
        }
    }
}

/// The object a dispatch finding is about, which for a dispatch finding is
/// the thing that has to change.
fn remediation_subject(remediation: &PreflightRemediation) -> String {
    match remediation {
        PreflightRemediation::DesignChecks => "Design checks".to_owned(),
        PreflightRemediation::SimulationPlan => "Simulation plan".to_owned(),
        PreflightRemediation::ProjectTechnology => "Project technology".to_owned(),
        PreflightRemediation::Models {
            library: Some(library),
            corner: Some(corner),
            ..
        } => format!("{library} · {corner}"),
        PreflightRemediation::Models {
            library: Some(library),
            ..
        } => library.clone(),
        PreflightRemediation::Models {
            page: ModelsPage::Corners,
            ..
        } => "Corners & sections".to_owned(),
        PreflightRemediation::Models { .. } => "Model libraries".to_owned(),
        PreflightRemediation::NetlistSource { line: Some(line) } => {
            format!("Generated netlist · line {line}")
        }
        PreflightRemediation::NetlistSource { line: None } => "Generated netlist".to_owned(),
    }
}

/// A rule code from a label a human wrote: `Model bindings` in the `MODEL`
/// family becomes `MODEL-MODEL-BINDINGS`.
///
/// Derived rather than authored beside each check so a new check cannot ship
/// without a code, and stable because the labels are authored constants.
fn screaming_code(family: &str, label: &str) -> String {
    let mut code = String::with_capacity(family.len() + label.len() + 1);
    code.push_str(family);
    let mut separated = true;
    for character in label.chars() {
        if character.is_ascii_alphanumeric() {
            if separated {
                code.push('-');
                separated = false;
            }
            code.push(character.to_ascii_uppercase());
        } else {
            separated = true;
        }
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::app_state::diagnostic_store::listing;

    fn open_state() -> AppState {
        let mut state = AppState::default();
        state.project_lifecycle.authority.open_session();
        state
    }

    #[test]
    fn a_code_is_one_word_per_label_word() {
        assert_eq!(
            screaming_code("MODEL", "Model bindings"),
            "MODEL-MODEL-BINDINGS"
        );
        assert_eq!(
            screaming_code("RUN", "Corners & sections"),
            "RUN-CORNERS-SECTIONS"
        );
    }

    /// The blocker's own destination is what the row follows. Before this the
    /// only way to reach it was the dialog's Action button.
    #[test]
    fn a_blocker_becomes_a_record_routed_to_its_own_remediation() {
        let mut state = open_state();
        let report = PreflightReport {
            project_revision: 4,
            topology_root: String::new(),
            topology_revision: 0,
            topology_closure: Vec::new(),
            simulation_plan_id: None,
            simulation_plan_revision: None,
            blockers: vec![PreflightIssue {
                check: "Model bindings".to_owned(),
                observed: "Corner tt declares no bulk section".to_owned(),
                required: "Every enabled corner binds each required domain".to_owned(),
                remediation: PreflightRemediation::model_corner("analog", Some("tt".to_owned())),
            }],
            advisories: vec![PreflightAdvisory::from(
                "Technology: not attached".to_owned(),
            )],
            prepared: None,
        };

        state.publish_dispatch_diagnostics(&report);

        let listed = listing::listing(&state);
        assert_eq!(listed.len(), 2);
        let counts = listed.counts();
        assert_eq!(counts.errors, 1);
        assert_eq!(counts.advisories, 1);
        let blocker = listed.row(0).expect("the blocker is listed");
        assert_eq!(blocker.code, "MODEL-MODEL-BINDINGS");
        assert_eq!(blocker.subject, "analog · tt");
        assert_eq!(
            blocker.route,
            Some(DiagnosticRoute::Repair(PreflightRemediation::model_corner(
                "analog",
                Some("tt".to_owned())
            ))),
            "a blocker's route is the destination it named for itself",
        );
        let advisory = listed.row(1).expect("the advisory is listed");
        assert!(
            advisory.route.is_none(),
            "this advisory names no destination"
        );
    }

    /// An attribution recorded for a solve that was later rescued must not be
    /// shown against an unrelated failure.
    #[test]
    fn a_run_failure_takes_the_attribution_that_describes_it() {
        use rspice_results::convergence_attribution::{
            ConvergenceFailureClass, ConvergenceSite, ConvergenceSiteKind,
        };

        let mut state = open_state();
        let attribution = ConvergenceAttribution {
            class: ConvergenceFailureClass::NoDcPathToGround,
            sites: vec![ConvergenceSite {
                name: "vbias".to_owned(),
                kind: ConvergenceSiteKind::Node,
                residual: None,
            }],
            elided_sites: 0,
            failure_message: "no DC path at vbias".to_owned(),
        };

        state.publish_run_failure(
            "Transient",
            "Analysis failed: no DC path at vbias",
            Some(&attribution),
            7,
        );
        let listed = listing::listing(&state);
        let row = listed.row(0).expect("the failure is listed");
        assert_eq!(row.code, "RUN-NO-DC-PATH-TO-GROUND");
        assert_eq!(row.subject, "Transient · run 7");
        assert!(matches!(
            row.route,
            Some(DiagnosticRoute::Anchor(LogAnchor::Simulation { .. }))
        ));

        state.publish_run_failure(
            "Transient",
            "Analysis failed: unrelated refusal",
            Some(&attribution),
            7,
        );
        let listed = listing::listing(&state);
        let row = listed.row(0).expect("the failure is listed");
        assert_eq!(
            row.code, "RUN-FAILED",
            "an attribution for another failure must not be adopted",
        );
        assert!(row.route.is_none());
    }
}
