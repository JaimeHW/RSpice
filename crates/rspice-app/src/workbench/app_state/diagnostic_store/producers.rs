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
    DrawnObject, ModelsPage, PreflightAdvisory, PreflightIssue, PreflightRemediation,
    PreflightReport,
};

use super::super::AppState;
use super::{
    DiagnosticInputs, DiagnosticLevel, DiagnosticRecord, DiagnosticRoute, DiagnosticSubject,
    FindingKey, mint_canonical,
};

/// The document a run's failures are about: the run, not any file.
const RUN_DOCUMENT: &str = "simulation-run";
/// The document a dispatch finding is about.
const DISPATCH_DOCUMENT: &str = "simulation-dispatch";
/// The rule code every dispatch advisory carries.
const ADVISORY_CODE: &str = "RUN-ADVISORY";

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
/// The engine's own build of a dispatched deck, for what it refuses there.
const ENGINE_PRODUCER: &str = "rspice.engine.elaboration";

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
        // Keyed by the analysis that failed, in the place the run stands in:
        // a plain run, or one member of a campaign. Not by the run's
        // sequence, which is recorded as the revision — the next run's
        // failure of the same analysis is the same condition, still standing.
        let key = FindingKey::new(producer, code, run_scope(self, run_sequence), analysis);
        let inputs = DiagnosticInputs::Run { run_sequence };
        // Every failure in one run stands together. Each analysis that
        // failed is its own record with its own identity; a later failure in
        // the same run is added to the batch rather than replacing it. Both
        // are decided against the design as it was dispatched.
        let opened = self.diagnostics.batch(&inputs);
        let dispatched = opened
            .map(|batch| batch.input_digest)
            .or_else(|| self.diagnostics.run_baselines.get(&run_sequence).copied());
        let mut records: Vec<DiagnosticRecord> = opened
            .map(|batch| batch.records.clone())
            .unwrap_or_default();
        let mut keys: Vec<FindingKey> = records.iter().map(|record| record.key.clone()).collect();
        keys.push(key.clone());
        let Some(canonical) = mint_canonical(
            &keys,
            run_sequence,
            &format!("run:{run_sequence}:{analysis}"),
        )
        .pop() else {
            return;
        };
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
        .keyed(key)
        .with_details(attribution.map_or_else(String::new, ConvergenceAttribution::summary))
        .with_route(route);
        records.push(record);
        let published = match dispatched {
            Some(digest) => self.diagnostics.insert(digest, inputs, records),
            None => self.publish_diagnostics(inputs, records),
        };
        self.state_refusal(published);
    }

    /// One record for a campaign member that never became a run.
    ///
    /// Filed as the members that ran file their failures — scoped to the
    /// member's place in the campaign and named by the plan it was to run —
    /// so the Problems page lists it beside them, and a later run supersedes
    /// it as it supersedes theirs. It used to reach only the Console, which
    /// left a campaign with a refused member reading as clean on the page.
    ///
    /// A refusal is an attempt to measure the design all the same, so it
    /// supersedes what earlier runs and campaigns reported before it is
    /// filed: a re-run whose member is refused again replaces the last
    /// refusal rather than colliding with its identity.
    pub(crate) fn publish_campaign_member_refusal(
        &mut self,
        campaign: crate::product::SimulationCampaignId,
        member_index: u32,
        member: &str,
        refusal: crate::simulation::controller::CampaignMemberRefusal,
        error: &str,
    ) {
        use crate::simulation::controller::CampaignMemberRefusal;

        let (code, message) = match refusal {
            CampaignMemberRefusal::Authorization => (
                "RUN-MEMBER-UNAUTHORIZED",
                format!("The member could not be authorized: {error}"),
            ),
            CampaignMemberRefusal::Start => (
                "RUN-MEMBER-NOT-STARTED",
                format!("The member could not be started: {error}"),
            ),
        };
        let key = FindingKey::new(RUN_PRODUCER, code, member_scope(member_index), member);
        let revision = self.workspace.content.project.revision().get();
        let Some(canonical) = mint_canonical(
            std::slice::from_ref(&key),
            revision,
            &format!("campaign:{campaign}:member:{member_index}"),
        )
        .pop() else {
            return;
        };
        let record = DiagnosticRecord::new(
            canonical,
            DiagnosticLevel::Error,
            DiagnosticSubject::new(
                self.workspace.content.project.id(),
                format!("{member} · campaign member {member_index}"),
            ),
            message,
        )
        .keyed(key);
        self.supersede_run_outcomes(None, Some(campaign));
        self.publish_or_state_refusal(
            DiagnosticInputs::CampaignMember {
                campaign,
                member_index,
            },
            vec![record],
        );
    }

    /// Take the digest a dispatched run's failures will be decided against.
    ///
    /// Taken here because this is the design and setup the run solves. Taken
    /// when a failure arrives instead, it was whatever the reader had edited
    /// while the run was solving, and a failure about the design as
    /// dispatched read as current against an edit made after it. Nothing is
    /// published: a run that fails nothing has nothing to list, and an empty
    /// batch would tell the page that checks had run.
    pub(crate) fn open_run_outcome(&mut self, run_sequence: u64) {
        match (DiagnosticInputs::Run { run_sequence }).digest(&super::DigestScope::new(self)) {
            Ok(digest) => {
                let retained = &self.simulation.retained.runs;
                self.diagnostics
                    .run_baselines
                    .retain(|run, _| retained.iter().any(|kept| kept.id == *run));
                self.diagnostics.run_baselines.insert(run_sequence, digest);
            }
            Err(error) => self.state_refusal(Err(error)),
        }
    }

    /// Everything the dispatch checks found, as records.
    ///
    /// A blocker is an error whose route is the destination it named for
    /// itself; an advisory is a warning the run proceeds through. Both come
    /// from one report, so both expire together when the design, the plan or
    /// the bindings move — which is exactly what the report's own currency
    /// rule already said and what no surface but the report could state.
    pub(in crate::workbench) fn publish_dispatch_diagnostics(&mut self, report: &PreflightReport) {
        let records = self.dispatch_records(report);
        self.publish_or_state_refusal(DiagnosticInputs::Dispatch, records);
    }

    /// The same records, decided against the dispatch inputs as they stood
    /// when the pass that found them began (`Self::dispatch_inputs_at_press`),
    /// so an edit made while the engine built the decks leaves them stale.
    pub(in crate::workbench) fn publish_dispatch_diagnostics_against(
        &mut self,
        report: &PreflightReport,
        inputs_at_press: crate::product::ContentDigest,
    ) {
        let records = self.dispatch_records(report);
        let published =
            self.diagnostics
                .insert(inputs_at_press, DiagnosticInputs::Dispatch, records);
        self.state_refusal(published);
    }

    fn dispatch_records(&self, report: &PreflightReport) -> Vec<DiagnosticRecord> {
        let project = self.workspace.content.project.id();
        let revision = report.project_revision;
        // Keyed by what each finding names as the thing to change, which is
        // the nearest a dispatch check comes to an object. Two findings that
        // name the same destination are told apart by the order the pass
        // reported them in, and by nothing else about the pass.
        let blocker_keys = report.blockers.iter().map(|issue| {
            let (producer, family) = remediation_producer(&issue.remediation);
            FindingKey::new(
                producer,
                screaming_code(family, &issue.check),
                DISPATCH_DOCUMENT,
                remediation_key(&issue.remediation),
            )
        });
        let advisory_keys = report.advisories.iter().map(|advisory| {
            let remediation = advisory.remediation.as_ref();
            FindingKey::new(
                remediation.map_or(PREFLIGHT_PRODUCER, |remediation| {
                    remediation_producer(remediation).0
                }),
                ADVISORY_CODE,
                DISPATCH_DOCUMENT,
                remediation.map_or_else(String::new, remediation_key),
            )
        });
        let keys: Vec<FindingKey> = blocker_keys.chain(advisory_keys).collect();
        let mut canonicals =
            mint_canonical(&keys, revision, &format!("dispatch:{revision}")).into_iter();
        let mut keys = keys.into_iter();
        let mut records = Vec::with_capacity(keys.len());
        for issue in &report.blockers {
            if let (Some(key), Some(canonical)) = (keys.next(), canonicals.next()) {
                records.push(blocker_record(project, key, canonical, issue));
            }
        }
        for advisory in &report.advisories {
            if let (Some(key), Some(canonical)) = (keys.next(), canonicals.next()) {
                records.push(advisory_record(project, key, canonical, advisory));
            }
        }
        records
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
        let published = self.publish_diagnostics(inputs, records);
        self.state_refusal(published);
    }

    fn state_refusal(&mut self, published: Result<(), String>) {
        if let Err(error) = published {
            self.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                "Findings could not be recorded: {error}"
            )));
        }
    }
}

/// Where a run stands, for keying its failures: a plain run, or one member of
/// the campaign it belongs to.
///
/// A campaign's members are dispatched one after another and their outcomes
/// stand together, so each member is its own place; a plain run's outcome is
/// superseded by the next run's, so every plain run is the same place.
pub(super) fn run_scope(state: &AppState, run_sequence: u64) -> String {
    state
        .simulation
        .retained
        .run_by_sequence(run_sequence)
        .and_then(|run| run.campaign_membership())
        .map_or_else(
            || RUN_DOCUMENT.to_owned(),
            |membership| member_scope(membership.member_index()),
        )
}

/// The place one campaign member stands in, whether it ran or was refused.
fn member_scope(member_index: u32) -> String {
    format!("{RUN_DOCUMENT}/member-{member_index}")
}

/// A stable key of the destination a dispatch finding names.
///
/// A deck line is left out: it is a position in a generated deck, which moves
/// with every edit upstream of it, and the finding is about the deck. An
/// analysis is kept: its instance identity survives every edit to it, and two
/// analyses that fail the same check are two findings, not one told apart by
/// the order they were reported in.
fn remediation_key(remediation: &PreflightRemediation) -> String {
    match remediation {
        PreflightRemediation::DesignChecks => "design-checks".to_owned(),
        PreflightRemediation::SimulationPlan { analysis: None } => "simulation-plan".to_owned(),
        PreflightRemediation::SimulationPlan {
            analysis: Some(analysis),
        } => format!("simulation-plan:{analysis}"),
        PreflightRemediation::SolverOptions => "solver-options".to_owned(),
        PreflightRemediation::ProjectTechnology => "project-technology".to_owned(),
        PreflightRemediation::Models {
            page,
            library,
            corner,
        } => format!(
            "models:{page:?}:{}:{}",
            library.as_deref().unwrap_or_default(),
            corner.as_deref().unwrap_or_default()
        ),
        PreflightRemediation::NetlistSource { .. } => "netlist-source".to_owned(),
        PreflightRemediation::RuleSetup => "rule-setup".to_owned(),
        // The drawn object, in the view that draws it: two engine refusals
        // about one net are one place, wherever the deck spelled it.
        PreflightRemediation::Drawing { cell_view, object } => match object {
            DrawnObject::Component { id, .. } => {
                format!("drawing:{}:component:{id}", cell_view.key())
            }
            DrawnObject::Net { name, .. } => format!(
                "drawing:{}:net:{}",
                cell_view.key(),
                name.to_ascii_lowercase()
            ),
        },
    }
}

fn blocker_record(
    project: crate::product::ProjectId,
    key: FindingKey,
    canonical: CanonicalDiagnosticMetadata,
    issue: &PreflightIssue,
) -> DiagnosticRecord {
    let subject = DiagnosticSubject::new(project, remediation_subject(&issue.remediation));
    let subject = match &issue.remediation {
        PreflightRemediation::Drawing { cell_view, .. } => subject.in_cell_view(cell_view.clone()),
        _ => subject,
    };
    DiagnosticRecord::new(
        canonical,
        DiagnosticLevel::Error,
        subject,
        issue.observed.clone(),
    )
    .keyed(key)
    .with_details(format!("{} · required: {}", issue.check, issue.required))
    // A dispatch check states what it requires of every run, which is the
    // nearest thing it has to rule prose and is not the same sentence as what
    // it observed about this one.
    .with_rule(issue.required.clone())
    .with_route(Some(DiagnosticRoute::Repair(issue.remediation.clone())))
}

fn advisory_record(
    project: crate::product::ProjectId,
    key: FindingKey,
    canonical: CanonicalDiagnosticMetadata,
    advisory: &PreflightAdvisory,
) -> DiagnosticRecord {
    let remediation = advisory.remediation.as_ref();
    DiagnosticRecord::new(
        canonical,
        DiagnosticLevel::Warning,
        DiagnosticSubject::new(
            project,
            remediation.map_or_else(|| "Simulation dispatch".to_owned(), remediation_subject),
        ),
        advisory.message.clone(),
    )
    .keyed(key)
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
        // The engine's refusal, whose remedy is one of the plan's options.
        PreflightRemediation::Drawing { .. } | PreflightRemediation::SolverOptions => {
            (ENGINE_PRODUCER, "ENGINE")
        }
        PreflightRemediation::DesignChecks
        | PreflightRemediation::SimulationPlan { .. }
        | PreflightRemediation::RuleSetup => (PREFLIGHT_PRODUCER, "RUN"),
    }
}

/// The object a dispatch finding is about, which for a dispatch finding is
/// the thing that has to change.
fn remediation_subject(remediation: &PreflightRemediation) -> String {
    match remediation {
        PreflightRemediation::DesignChecks => "Design checks".to_owned(),
        PreflightRemediation::SimulationPlan { .. } => "Simulation plan".to_owned(),
        PreflightRemediation::SolverOptions => "Solver & convergence".to_owned(),
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
        PreflightRemediation::RuleSetup => "Rule deck".to_owned(),
        PreflightRemediation::Drawing { object, .. } => object.name().to_owned(),
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
        let row = listed.row(1).expect("the second failure is listed");
        assert_eq!(
            row.code, "RUN-FAILED",
            "an attribution for another failure must not be adopted",
        );
        assert!(row.route.is_none());
    }

    /// Two analyses that fail in one run are two findings. Each failure used
    /// to be published as the whole batch, so the second replaced the first.
    #[test]
    fn every_failure_in_one_run_stands_under_its_own_identity() {
        let mut state = open_state();
        state.publish_run_failure("Operating point", "Analysis failed: OP", None, 3);
        state.publish_run_failure("Transient", "Analysis failed: tran", None, 3);

        let listed = listing::listing(&state);
        assert_eq!(listed.len(), 2, "both failures stand");
        let first = listed.row(0).expect("the first failure is listed");
        let second = listed.row(1).expect("the second failure is listed");
        assert_eq!(first.subject, "Operating point · run 3");
        assert_eq!(second.subject, "Transient · run 3");
        assert_ne!(first.id, second.id);
        assert_eq!(listed.counts().errors, 2);
    }
}
