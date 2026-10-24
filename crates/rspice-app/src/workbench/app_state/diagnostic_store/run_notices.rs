//! What a run raised and went on past, as records beside its failures.
//!
//! The engine states each advisory in the Console as it happens, and the
//! Console keeps that prose: it is the run's transcript. These records are
//! the other statement, the one an engineer signs off against: a clamped
//! parameter, a convergence aid, a held timestep, time points accepted past
//! the truncation-error tolerance, a measurement that never triggered. Each
//! is one record per condition for the whole run, however
//! many analyses, sweep points or Monte Carlo trials raised it, and it says
//! where it was raised.
//!
//! The records stand in the run's own batch, so they live and die with its
//! failures: the next run of the design supersedes them, and an edit to the
//! design the run was dispatched against makes them stale rather than gone.

use std::collections::HashSet;
use std::hash::Hash;

use rspice_core::ContinuationAid;
use rspice_core::notice::{
    BoundarySupplyKind, EngineNotice, ParameterReplacement, ParameterScope, RecordedNotice,
};
use rspice_results::convergence_quality::TransientConvergenceEvidence;
use rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility;

use crate::quantity::engineering::{EngineeringPrecision, format_engineering_display_with};
use crate::state::PreflightRemediation;
use crate::state::{CellViewRef, InstancePath};

use super::super::model::{Model, SlotFacts};
use super::{
    DiagnosticInputs, DiagnosticLevel, DiagnosticRecord, DiagnosticRoute, DiagnosticSubject,
    FindingKey, mint_canonical,
};

/// A model replaced a parameter it could not evaluate.
const MODEL_PRODUCER: &str = "rspice.engine.model";
/// An operating-point solve needed an aid. The same producer the run's
/// convergence failures are reported under.
const CONVERGENCE_PRODUCER: &str = "rspice.engine.convergence";
/// The transient stepper.
const TIMESTEP_PRODUCER: &str = "rspice.engine.timestep";
/// The run's `.MEASURE` evaluation.
const MEASURE_PRODUCER: &str = "rspice.engine.measure";
/// The converters the build inserts on mixed boundaries.
const BOUNDARY_PRODUCER: &str = "rspice.engine.boundary";

pub const PARAMETER_CODE: &str = "MODEL-PARAMETER-CLAMPED";
pub const AID_CODE: &str = "RUN-CONVERGENCE-AID";
const TIMESTEP_CODE: &str = "RUN-TIMESTEP-FLOOR";
pub const TRUNCATION_CODE: &str = "RUN-TRUNCATION-EXCEEDED";
pub const MEASURE_CODE: &str = "RUN-MEASURE-FAILED";
const SUPPLY_INFERRED_CODE: &str = "RUN-BOUNDARY-SUPPLY-INFERRED";
pub const SUPPLY_DEFAULTED_CODE: &str = "RUN-BOUNDARY-SUPPLY-DEFAULTED";
const SUPPLY_TEMPLATE_CODE: &str = "RUN-BOUNDARY-SUPPLY-TEMPLATE-STATIC";

/// Every rule code a notice record carries, which is what tells a notice
/// from a failure in the batch they share.
const NOTICE_CODES: [&str; 8] = [
    PARAMETER_CODE,
    AID_CODE,
    TIMESTEP_CODE,
    TRUNCATION_CODE,
    MEASURE_CODE,
    SUPPLY_INFERRED_CODE,
    SUPPLY_DEFAULTED_CODE,
    SUPPLY_TEMPLATE_CODE,
];

/// The analyses, or the sweep points, one record names. Past this many, it
/// counts.
const MAX_NAMED: usize = 8;

/// One thing an analysis of a run raised, as the run's batch keeps it.
#[derive(Debug, Clone)]
pub(crate) enum RunNotice {
    Engine(RecordedNotice),
    /// Transient time points the stepper accepted although they failed the
    /// local truncation-error test, and the first time it did.
    TruncationExceeded {
        points: u64,
        first_time: Option<f64>,
    },
    /// A `.MEASURE` that produced no value, in the engine's words.
    MeasureFailed {
        name: String,
        reason: String,
    },
}

/// One thing an analysis of a run raised, and where the run's log states it.
#[derive(Debug, Clone)]
pub(super) struct RunNoticeFact {
    analysis: String,
    notice: RunNotice,
    /// The line of the run's log that states it, by its position there, where
    /// one does.
    line: Option<usize>,
}

/// Everything one run's analyses raised, in the order they reported it.
pub(super) type RunNoticeFacts = Vec<RunNoticeFact>;

/// Which drawn component the engine named: the occurrence it was solved in,
/// the cell view that draws it, and the component.
///
/// Not where it is: the component was found in the projected design, which
/// lays each sheet a stride along, so its position there is not where any
/// sheet draws it. The route asks the drawing itself.
#[derive(Debug, Clone, PartialEq)]
pub struct DrawnElement {
    pub occurrence: InstancePath,
    pub cell_view: CellViewRef,
    pub component: u64,
    pub name: String,
}

impl DrawnElement {
    /// The component as the reader finds it in the design: `M1` at the root,
    /// `/amp/M1` inside an occurrence. A master placed twice draws one `M1`
    /// that is solved twice, and the two are different conditions.
    fn path(&self) -> String {
        if self.occurrence.is_root() {
            self.name.clone()
        } else {
            format!("{}/{}", self.occurrence, self.name)
        }
    }
}

/// One condition, gathered over every analysis and point that raised it.
struct Gathered<'a> {
    key: FindingKey,
    first: &'a RunNotice,
    occurrences: usize,
    /// By name.
    analyses: Named<String>,
    /// By the identity each analysis's record gives a point.
    points: Named<u64>,
    /// The line of the run's log that states it: the first that did.
    line: Option<usize>,
}

/// A short list of names and how many more there were.
///
/// Every distinct one is held by what identifies it, so one that two
/// analyses raised is counted once, whether both name it, one names it and
/// the other only counts it, or both only count it.
struct Named<K> {
    names: Vec<String>,
    /// Every one, the named among them.
    every: HashSet<K>,
}

impl<K> Default for Named<K> {
    fn default() -> Self {
        Self {
            names: Vec::new(),
            every: HashSet::new(),
        }
    }
}

impl<K: Eq + Hash> Named<K> {
    /// Name `name` if it is among the first few.
    fn name(&mut self, name: &str) {
        if self.names.len() < MAX_NAMED && !self.names.iter().any(|named| named == name) {
            self.names.push(name.to_owned());
        }
    }

    fn sentence(&self) -> String {
        let mut text = self.names.join(", ");
        let more = self.every.len().saturating_sub(self.names.len());
        if more > 0 {
            text.push_str(&format!(" and {more} more"));
        }
        text
    }
}

impl Named<String> {
    fn add(&mut self, name: &str) {
        self.name(name);
        self.every.insert(name.to_owned());
    }
}

impl Named<u64> {
    /// The points one analysis's record names, and every point it was
    /// raised at, the named among them.
    fn add(&mut self, named: &[String], identities: &[u64]) {
        for name in named {
            self.name(name);
        }
        self.every.extend(identities);
    }
}

impl Model {
    /// Record what one analysis of a run raised and went on past.
    ///
    /// Called for every analysis the run reports, completed or failed: a
    /// clamp is part of the account of a failure too. The run's batch is then
    /// rebuilt from everything its analyses have raised so far, so a clamp
    /// the second analysis raises again is counted on the first one's record
    /// rather than listed twice.
    ///
    /// Each notice comes with the position in the run's log of the line
    /// that stated it, where the log kept one. `transient` is the convergence
    /// evidence of the transient this analysis solved, where it solved one,
    /// with the run's line that says it accepted points past its truncation
    /// tolerance; `failed_measurements` its `.MEASURE` results, of which only
    /// those that produced no value are kept. A measurement's failure is
    /// stated in no line of the log.
    pub fn publish_run_notices(
        &mut self,
        analysis: &str,
        notices: Vec<(RecordedNotice, Option<usize>)>,
        transient: Option<(&TransientConvergenceEvidence, Option<usize>)>,
        failed_measurements: &[rspice_core::MeasureResult],
        run_sequence: u64,
        facts: SlotFacts,
    ) {
        let raised: Vec<(RunNotice, Option<usize>)> = notices
            .into_iter()
            .map(|(notice, line)| (RunNotice::Engine(notice), line))
            .chain(transient.and_then(|(evidence, line)| {
                truncation_exceeded(evidence).map(|notice| (notice, line))
            }))
            .chain(failed_measurements.iter().filter_map(|measure| {
                measure.error.as_ref().map(|reason| {
                    let failed = RunNotice::MeasureFailed {
                        name: measure.name.clone(),
                        reason: reason.clone(),
                    };
                    (failed, None)
                })
            }))
            .collect();
        if raised.is_empty() {
            return;
        }
        let retained = &self.simulation.retained.runs;
        self.diagnostics
            .run_notices
            .retain(|run, _| *run == run_sequence || retained.iter().any(|kept| kept.id == *run));
        self.diagnostics
            .run_notices
            .entry(run_sequence)
            .or_default()
            .extend(raised.into_iter().map(|(notice, line)| RunNoticeFact {
                analysis: analysis.to_owned(),
                notice,
                line,
            }));
        self.republish_run_notices(run_sequence, facts);
    }

    fn republish_run_notices(&mut self, run_sequence: u64, slot_facts: SlotFacts) {
        let Some(facts) = self.diagnostics.run_notices.get(&run_sequence) else {
            return;
        };
        let scope = super::producers::run_scope(self, run_sequence);
        let gathered = gather(facts, &scope);
        let keys: Vec<FindingKey> = gathered
            .iter()
            .map(|condition| condition.key.clone())
            .collect();
        let canonicals =
            mint_canonical(&keys, run_sequence, &format!("run:{run_sequence}:notices"));
        let manual_deck = self
            .simulation
            .retained
            .run_by_sequence(run_sequence)
            .and_then(|run| run.prepared_receipt())
            .is_some_and(|receipt| {
                receipt.source_domain()
                    == rspice_results::provenance::AnalysisResultSourceDomain::ManualDeck
            });
        let project = self.workspace.content.project.id();
        let notice_records: Vec<DiagnosticRecord> = gathered
            .iter()
            .zip(canonicals)
            .map(|(condition, canonical)| {
                self.notice_record(
                    project,
                    condition,
                    canonical,
                    manual_deck,
                    slot_facts.review_markers,
                )
                .with_log_place(
                    condition
                        .line
                        .map(|line| super::RunLogPlace { run_sequence, line }),
                )
            })
            .collect();

        let inputs = DiagnosticInputs::Run { run_sequence };
        let opened = self.diagnostics.batch(&inputs);
        let dispatched = opened
            .map(|batch| batch.input_digest)
            .or_else(|| self.diagnostics.run_baselines.get(&run_sequence).copied());
        // The run's failures stand as they were reported; only the notices
        // are rebuilt.
        let mut records: Vec<DiagnosticRecord> = opened
            .map(|batch| {
                batch
                    .records
                    .iter()
                    .filter(|record| !is_notice(record))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();
        records.extend(notice_records);
        let published = match dispatched {
            Some(digest) => self.diagnostics.insert(digest, inputs, records, None),
            None => self.publish_diagnostics(inputs, records, slot_facts),
        };
        if let Err(error) = published {
            self.record(
                crate::diagnostics::LogSeverity::Warning,
                crate::diagnostics::LogSource::Studio,
                format!("Findings could not be recorded: {error}"),
            );
        }
    }

    fn notice_record(
        &self,
        project: crate::product::ProjectId,
        condition: &Gathered<'_>,
        canonical: rspice_vocabulary::diagnostics::CanonicalDiagnosticMetadata,
        manual_deck: bool,
        review_markers: SchematicReviewMarkerVisibility,
    ) -> DiagnosticRecord {
        // A manual deck is not the drawing, so nothing it names is looked
        // for there; its notices are repaired in the deck.
        let deck = || DiagnosticRoute::Repair(PreflightRemediation::NetlistSource { line: None });
        let plan = || {
            if manual_deck {
                deck()
            } else {
                DiagnosticRoute::Repair(PreflightRemediation::SimulationPlan { analysis: None })
            }
        };
        let locate = |element: Option<&str>| {
            element
                .filter(|_| !manual_deck)
                .and_then(|element| self.locate_engine_element(element))
        };
        let (level, object, located, message, fallback) = match condition.first {
            RunNotice::Engine(recorded) => match &recorded.notice {
                EngineNotice::ParameterReplaced(replacement) => {
                    let located = locate(replacement.element.as_deref());
                    let object = match replacement.scope {
                        ParameterScope::Model => format!(
                            "model {}",
                            replacement.model.as_deref().unwrap_or(&replacement.family)
                        ),
                        ParameterScope::Instance => located.as_ref().map_or_else(
                            || replacement.element.clone().unwrap_or_default(),
                            DrawnElement::path,
                        ),
                    };
                    let fallback = if manual_deck { Some(deck()) } else { None };
                    (
                        DiagnosticLevel::Warning,
                        object,
                        located,
                        replacement_message(replacement),
                        fallback,
                    )
                }
                EngineNotice::ConvergenceAid(aid) => (
                    DiagnosticLevel::Info,
                    condition.analyses.sentence(),
                    None,
                    format!(
                        "The operating point converged only with {}; the direct solve did not",
                        aid_name(*aid)
                    ),
                    Some(plan()),
                ),
                EngineNotice::TimestepFloor(floor) => {
                    let located = locate(floor.element.as_deref());
                    let object = located.as_ref().map_or_else(
                        || {
                            floor
                                .element
                                .clone()
                                .unwrap_or_else(|| condition.analyses.sentence())
                        },
                        DrawnElement::path,
                    );
                    (
                        DiagnosticLevel::Warning,
                        object,
                        located,
                        format!(
                            "The transient step was held at its minimum, {}s, near t = {}s by \
                             scheduled activations",
                            engineering(floor.floor),
                            engineering(floor.time)
                        ),
                        Some(plan()),
                    )
                }
                EngineNotice::BoundarySupply {
                    boundary,
                    kind,
                    level,
                    from,
                } => (
                    match kind {
                        BoundarySupplyKind::TemplateStatic => DiagnosticLevel::Info,
                        _ => DiagnosticLevel::Warning,
                    },
                    boundary.clone(),
                    None,
                    boundary_supply_message(boundary, *kind, *level, from.as_deref()),
                    Some(plan()),
                ),
            },
            RunNotice::TruncationExceeded { first_time, .. } => (
                DiagnosticLevel::Warning,
                condition.analyses.sentence(),
                None,
                match first_time {
                    Some(time) => format!(
                        "Time points were accepted past the local truncation-error tolerance, \
                         the first near t = {}s",
                        engineering(*time)
                    ),
                    None => "Time points were accepted past the local truncation-error tolerance"
                        .to_owned(),
                },
                Some(plan()),
            ),
            RunNotice::MeasureFailed { name, reason } => (
                DiagnosticLevel::Warning,
                name.clone(),
                None,
                format!("{name} produced no value: {reason}"),
                Some(plan()),
            ),
        };
        let mut subject = DiagnosticSubject::new(project, object);
        if let Some(drawn) = &located {
            subject = subject.in_cell_view(drawn.cell_view.clone());
        }
        // Resolved where the master draws the component, as a rule finding's
        // route is, so a part on a later sheet brings that sheet forward.
        let route = located
            .and_then(|drawn| {
                crate::workbench::app_state::design_checks::model_location_anchor(
                    self,
                    review_markers,
                    &drawn.cell_view,
                    &rspice_design::drc::DrcLocation::Component {
                        id: drawn.component,
                        name: drawn.name,
                    },
                )
            })
            .map(DiagnosticRoute::Anchor)
            .or(fallback);
        DiagnosticRecord::new(canonical, level, subject, message)
            .keyed(condition.key.clone())
            .with_details(occurrence_details(condition))
            .with_route(route)
    }

    /// The component the engine names `element`, drawn in the configured
    /// design, and the cell view it is drawn in.
    ///
    /// The engine's name carries the written names of every instance above
    /// the element, `XAMP.M1`; the occurrence they spell is found among the
    /// ones the configured design binds, and the element among the
    /// components its master draws, by the name each writes into the deck.
    pub fn locate_engine_element(&self, element: &str) -> Option<DrawnElement> {
        let element = element.trim();
        let scope = rspice_vocabulary::hierarchy_path::ProbeTarget::parse(element)
            .map_or_else(|_| InstancePath::root(), |target| target.scope);
        let projection = self
            .workspace
            .design_projection(
                &self.library_manager,
                &self.workspace.content.active_view,
                &self.schematic,
            )
            .ok()?;
        let plan = projection.plan();
        let (occurrence, master) = if scope.is_root() {
            (InstancePath::root(), plan.root().clone())
        } else {
            plan.bindings()
                .map(|binding| (binding.instance_path(), binding.resolved_reference()))
                .find(|(occurrence, _)| occurrence.same_engine_scope(&scope))
                .map(|(occurrence, master)| (occurrence.clone(), master.clone()))?
        };
        let key = master.key();
        let (_, schematic) = projection
            .schematic_buffers()
            .iter()
            .find(|(candidate, _)| candidate.eq_ignore_ascii_case(&key))?;
        let component = rspice_design::schematic::component::component_for_engine_device(
            &schematic.document().components,
            &occurrence,
            element,
        )?;
        Some(DrawnElement {
            occurrence,
            cell_view: master,
            component: component.id,
            name: component.name.clone(),
        })
    }
}

/// Whether a record in a run's batch is one of its notices.
pub fn is_notice(record: &DiagnosticRecord) -> bool {
    NOTICE_CODES.contains(&record.canonical.code.as_ref())
}

/// The run's notices, one per condition, in the order first raised.
fn gather<'a>(facts: &'a [RunNoticeFact], scope: &str) -> Vec<Gathered<'a>> {
    let mut gathered: Vec<Gathered<'a>> = Vec::new();
    for RunNoticeFact {
        analysis,
        notice,
        line,
    } in facts
    {
        let key = notice_key(notice, scope);
        let (occurrences, points, identities) = match notice {
            RunNotice::Engine(recorded) => (
                recorded.occurrences,
                recorded.points.as_slice(),
                recorded.point_identities.as_slice(),
            ),
            RunNotice::TruncationExceeded { points, .. } => (
                usize::try_from(*points).unwrap_or(usize::MAX),
                &[][..],
                &[][..],
            ),
            RunNotice::MeasureFailed { .. } => (1, &[][..], &[][..]),
        };
        let index = match gathered.iter().position(|condition| condition.key == key) {
            Some(index) => index,
            None => {
                gathered.push(Gathered {
                    key,
                    first: notice,
                    occurrences: 0,
                    analyses: Named::default(),
                    points: Named::default(),
                    line: None,
                });
                gathered.len() - 1
            }
        };
        let condition = &mut gathered[index];
        condition.occurrences = condition.occurrences.saturating_add(occurrences);
        condition.analyses.add(analysis);
        condition.points.add(points, identities);
        condition.line = condition.line.or(*line);
    }
    gathered
}

/// What makes two notices one condition: the kind, the run's place, and the
/// object — the card or the element, and the parameter.
fn notice_key(notice: &RunNotice, scope: &str) -> FindingKey {
    let (producer, code, object) = match notice {
        RunNotice::Engine(recorded) => match &recorded.notice {
            EngineNotice::ParameterReplaced(replacement) => {
                let owner = match replacement.scope {
                    ParameterScope::Model => {
                        format!("model:{}", replacement.model.as_deref().unwrap_or_default())
                    }
                    ParameterScope::Instance => replacement.element.clone().unwrap_or_default(),
                };
                (
                    MODEL_PRODUCER,
                    PARAMETER_CODE,
                    format!(
                        "{}\u{1f}{}\u{1f}{}",
                        replacement.family,
                        owner.to_ascii_uppercase(),
                        replacement.parameter
                    ),
                )
            }
            EngineNotice::ConvergenceAid(aid) => {
                (CONVERGENCE_PRODUCER, AID_CODE, format!("{aid:?}"))
            }
            EngineNotice::TimestepFloor(floor) => (
                TIMESTEP_PRODUCER,
                TIMESTEP_CODE,
                floor
                    .element
                    .as_deref()
                    .unwrap_or_default()
                    .to_ascii_uppercase(),
            ),
            EngineNotice::BoundarySupply { boundary, kind, .. } => (
                BOUNDARY_PRODUCER,
                match kind {
                    BoundarySupplyKind::Inferred => SUPPLY_INFERRED_CODE,
                    BoundarySupplyKind::Defaulted => SUPPLY_DEFAULTED_CODE,
                    BoundarySupplyKind::TemplateStatic => SUPPLY_TEMPLATE_CODE,
                },
                boundary.to_ascii_uppercase(),
            ),
        },
        // One condition of the run's stepping, however many analyses and
        // points it covers: the record counts them.
        RunNotice::TruncationExceeded { .. } => (TIMESTEP_PRODUCER, TRUNCATION_CODE, String::new()),
        RunNotice::MeasureFailed { name, .. } => {
            (MEASURE_PRODUCER, MEASURE_CODE, name.to_ascii_uppercase())
        }
    };
    FindingKey::new(producer, code, scope, object)
}

/// The points a transient accepted past its truncation-error tolerance, if
/// it accepted any: its own and its periodic initialization's, and the
/// earliest time on its own axis, which the evidence keeps in source order.
fn truncation_exceeded(evidence: &TransientConvergenceEvidence) -> Option<RunNotice> {
    evidence
        .has_lte_exceptions()
        .then(|| RunNotice::TruncationExceeded {
            points: evidence.transient.force_accepted_points.saturating_add(
                evidence
                    .initialization
                    .as_ref()
                    .map_or(0, |initialization| {
                        initialization.report.force_accepted_points
                    }),
            ),
            first_time: evidence
                .transient
                .time_basis
                .as_ref()
                .and_then(|basis| basis.force_accepted_times_s.first().copied()),
        })
}

/// `BSIM4 pbs = 50m is below 100m; every instance of the card uses 100m`.
///
/// A replacement with no bound is a value the model resets for another's
/// sake (A1, when A2 is out of range) or because it is not positive, so it
/// is said to be replaced and nothing more.
pub fn replacement_message(replacement: &ParameterReplacement) -> String {
    let requested = engineering(replacement.requested);
    let crossing = match replacement.limit {
        Some(limit) if replacement.requested < limit => format!("is below {}", engineering(limit)),
        Some(limit) if replacement.requested > limit => format!("is above {}", engineering(limit)),
        Some(limit) => format!("is at its limit {}", engineering(limit)),
        None => "is replaced".to_owned(),
    };
    let applied = engineering(replacement.applied);
    let parameter = &replacement.parameter;
    match replacement.scope {
        ParameterScope::Model => format!(
            "{} {parameter} = {requested} {crossing}; every instance of the card uses {applied}",
            replacement.family
        ),
        ParameterScope::Instance => format!(
            "{} {parameter} = {requested} {crossing}; the model uses {applied}",
            replacement.family
        ),
    }
}

/// What a boundary converts against, where the deck did not say.
fn boundary_supply_message(
    boundary: &str,
    kind: BoundarySupplyKind,
    level: f64,
    from: Option<&str>,
) -> String {
    let level = engineering(level);
    match kind {
        BoundarySupplyKind::Inferred => format!(
            "Boundary net {boundary} converts against {level}V inferred from {}; set .param vcc, \
             or a connect statement's vsup, to state it",
            from.unwrap_or("the one rail that reaches it")
        ),
        BoundarySupplyKind::Defaulted => format!(
            "No supply reaches boundary net {boundary}; it converts against the {level}V \
             default. Set .param vcc, or a connect statement's vsup, to state it"
        ),
        BoundarySupplyKind::TemplateStatic => format!(
            "Boundary net {boundary} converts against the static {level}V of template {}, which \
             does not follow its supply",
            from.unwrap_or("its auto-bridge template")
        ),
    }
}

/// Where and how often the condition was raised, and the engine's own name
/// for the element it was raised on.
fn occurrence_details(condition: &Gathered<'_>) -> String {
    let times = match condition.occurrences {
        0 | 1 => "Raised once".to_owned(),
        count => format!("Raised {count} times"),
    };
    let mut details = format!("{times} in {}", condition.analyses.sentence());
    if !condition.points.names.is_empty() {
        details.push_str(&format!(" · at {}", condition.points.sentence()));
    }
    if let RunNotice::Engine(recorded) = condition.first
        && let EngineNotice::ParameterReplaced(replacement) = &recorded.notice
        && let Some(element) = &replacement.element
    {
        details.push_str(&match replacement.scope {
            ParameterScope::Model => format!(" · card first bound by {element}"),
            ParameterScope::Instance => format!(" · engine element {element}"),
        });
    }
    details
}

fn aid_name(aid: ContinuationAid) -> &'static str {
    match aid {
        ContinuationAid::GminStepping => "gmin stepping",
        ContinuationAid::SourceStepping => "source stepping",
        ContinuationAid::PseudoTransient => "pseudo-transient continuation",
        ContinuationAid::GateGenerationStepping => "gate generation stepping",
        _ => "a convergence aid",
    }
}

fn engineering(value: f64) -> String {
    format_engineering_display_with(value, EngineeringPrecision::Adaptive)
}
