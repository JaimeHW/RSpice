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

use rspice_core::ContinuationAid;
use rspice_core::notice::{EngineNotice, ParameterReplacement, ParameterScope, RecordedNotice};
use rspice_results::convergence_quality::TransientConvergenceEvidence;

use crate::diagnostics::LogAnchor;
use crate::quantity::engineering::{EngineeringPrecision, format_engineering_display_with};
use crate::state::{CellViewRef, InstancePath, Point};
use crate::workbench::state::PreflightRemediation;

use super::super::AppState;
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

const PARAMETER_CODE: &str = "MODEL-PARAMETER-CLAMPED";
const AID_CODE: &str = "RUN-CONVERGENCE-AID";
const TIMESTEP_CODE: &str = "RUN-TIMESTEP-FLOOR";
const TRUNCATION_CODE: &str = "RUN-TRUNCATION-EXCEEDED";
const MEASURE_CODE: &str = "RUN-MEASURE-FAILED";

/// Every rule code a notice record carries, which is what tells a notice
/// from a failure in the batch they share.
const NOTICE_CODES: [&str; 5] = [
    PARAMETER_CODE,
    AID_CODE,
    TIMESTEP_CODE,
    TRUNCATION_CODE,
    MEASURE_CODE,
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

/// Everything one run's analyses raised, in the order they reported it.
pub(super) type RunNoticeFacts = Vec<(String, RunNotice)>;

/// Where a drawn component the engine named is: the occurrence it was solved
/// in, the cell view that draws it, the component, and its place on that
/// sheet.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::workbench) struct DrawnElement {
    pub(in crate::workbench) occurrence: InstancePath,
    pub(in crate::workbench) cell_view: CellViewRef,
    pub(in crate::workbench) component: u64,
    pub(in crate::workbench) name: String,
    pub(in crate::workbench) position: Point,
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
    analyses: Named,
    points: Named,
}

/// A short list of names and how many more there were.
#[derive(Default)]
struct Named {
    names: Vec<String>,
    more: usize,
}

impl Named {
    fn add(&mut self, name: &str) {
        if self.names.iter().any(|named| named == name) {
            return;
        }
        if self.names.len() < MAX_NAMED {
            self.names.push(name.to_owned());
        } else {
            self.more = self.more.saturating_add(1);
        }
    }

    fn sentence(&self) -> String {
        let mut text = self.names.join(", ");
        if self.more > 0 {
            text.push_str(&format!(" and {} more", self.more));
        }
        text
    }
}

impl AppState {
    /// Record what one analysis of a run raised and went on past.
    ///
    /// Called for every analysis the run reports, completed or failed: a
    /// clamp is part of the account of a failure too. The run's batch is then
    /// rebuilt from everything its analyses have raised so far, so a clamp
    /// the second analysis raises again is counted on the first one's record
    /// rather than listed twice.
    ///
    /// `transient` is the convergence evidence of the transient this analysis
    /// solved, where it solved one; `failed_measurements` its `.MEASURE`
    /// results, of which only those that produced no value are kept.
    pub(crate) fn publish_run_notices(
        &mut self,
        analysis: &str,
        notices: Vec<RecordedNotice>,
        transient: Option<&TransientConvergenceEvidence>,
        failed_measurements: &[rspice_core::MeasureResult],
        run_sequence: u64,
    ) {
        let raised: Vec<RunNotice> = notices
            .into_iter()
            .map(RunNotice::Engine)
            .chain(transient.and_then(truncation_exceeded))
            .chain(failed_measurements.iter().filter_map(|measure| {
                measure
                    .error
                    .as_ref()
                    .map(|reason| RunNotice::MeasureFailed {
                        name: measure.name.clone(),
                        reason: reason.clone(),
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
            .extend(
                raised
                    .into_iter()
                    .map(|notice| (analysis.to_owned(), notice)),
            );
        self.republish_run_notices(run_sequence);
    }

    fn republish_run_notices(&mut self, run_sequence: u64) {
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
                self.notice_record(project, condition, canonical, manual_deck)
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
            None => self.publish_diagnostics(inputs, records),
        };
        if let Err(error) = published {
            self.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                "Findings could not be recorded: {error}"
            )));
        }
    }

    fn notice_record(
        &self,
        project: crate::product::ProjectId,
        condition: &Gathered<'_>,
        canonical: rspice_app_types::diagnostics::CanonicalDiagnosticMetadata,
        manual_deck: bool,
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
        let route = located
            .map(|drawn| {
                DiagnosticRoute::Anchor(LogAnchor::Schematic {
                    cell_view: drawn.cell_view,
                    x: drawn.position.x,
                    y: drawn.position.y,
                    component: Some(drawn.component),
                    wire: None,
                })
            })
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
    pub(in crate::workbench) fn locate_engine_element(
        &self,
        element: &str,
    ) -> Option<DrawnElement> {
        let element = element.trim();
        let scope = rspice_app_types::hierarchy_path::ProbeTarget::parse(element)
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
            position: component.pos,
        })
    }
}

/// Whether a record in a run's batch is one of its notices.
fn is_notice(record: &DiagnosticRecord) -> bool {
    NOTICE_CODES.contains(&record.canonical.code.as_ref())
}

/// The run's notices, one per condition, in the order first raised.
fn gather<'a>(facts: &'a [(String, RunNotice)], scope: &str) -> Vec<Gathered<'a>> {
    let mut gathered: Vec<Gathered<'a>> = Vec::new();
    for (analysis, notice) in facts {
        let key = notice_key(notice, scope);
        let (occurrences, points, unnamed) = match notice {
            RunNotice::Engine(recorded) => (
                recorded.occurrences,
                recorded.points.as_slice(),
                recorded.unnamed_points,
            ),
            RunNotice::TruncationExceeded { points, .. } => {
                (usize::try_from(*points).unwrap_or(usize::MAX), &[][..], 0)
            }
            RunNotice::MeasureFailed { .. } => (1, &[][..], 0),
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
                });
                gathered.len() - 1
            }
        };
        let condition = &mut gathered[index];
        condition.occurrences = condition.occurrences.saturating_add(occurrences);
        condition.analyses.add(analysis);
        for point in points {
            condition.points.add(point);
        }
        condition.points.more = condition.points.more.saturating_add(unnamed);
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
fn replacement_message(replacement: &ParameterReplacement) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Cell, Component, ComponentType, Library, LibraryCellInstance, SchematicState, View,
        ViewType,
    };
    use crate::workbench::app_state::diagnostic_store::{BatchStanding, listing};

    fn open_state(runs: &[u64]) -> AppState {
        let mut state = AppState::default();
        state.project_lifecycle.authority.open_session();
        state.simulation.retained.runs = runs
            .iter()
            .map(|run| crate::state::SimulationRun::new(*run))
            .collect::<Vec<_>>()
            .into();
        state
    }

    /// `parameter` of `element` replaced at each of `points`.
    fn clamp(element: &str, parameter: &str, points: &[&str]) -> RecordedNotice {
        RecordedNotice {
            notice: EngineNotice::ParameterReplaced(ParameterReplacement {
                family: "BSIM4".to_owned(),
                model: Some("nch".to_owned()),
                element: Some(element.to_owned()),
                scope: ParameterScope::Instance,
                parameter: parameter.to_owned(),
                requested: -5.0,
                applied: 0.0,
                limit: Some(0.0),
            }),
            occurrences: points.len().max(1),
            points: points.iter().map(|point| (*point).to_owned()).collect(),
            unnamed_points: 0,
        }
    }

    /// The notice records the run's batch holds, as (code, subject, details).
    fn notices(state: &AppState) -> Vec<(String, String, String)> {
        state
            .diagnostics
            .batches()
            .flat_map(|batch| batch.records.iter())
            .filter(|record| is_notice(record))
            .map(|record| {
                (
                    record.canonical.code.to_string(),
                    record.subject.object.clone(),
                    record.details.clone(),
                )
            })
            .collect()
    }

    /// The next run of the design is a new measurement of it: what the last
    /// run raised stops standing when it is dispatched, and the new run's
    /// notices stand in its place.
    #[test]
    fn a_rerun_replaces_the_runs_warnings() {
        let mut state = open_state(&[1, 2]);
        state.open_run_outcome(1);
        state.publish_run_notices("Transient", vec![clamp("M1", "rdsw", &[])], None, &[], 1);
        assert_eq!(notices(&state).len(), 1);

        state.supersede_run_diagnostics(2);
        state.open_run_outcome(2);
        state.publish_run_notices("Transient", vec![clamp("M2", "rdsw", &[])], None, &[], 2);

        let standing = notices(&state);
        assert_eq!(standing.len(), 1, "{standing:?}");
        assert_eq!(standing[0].1, "M2", "the rerun's own warning stands");
    }

    /// An edit to the design the run was dispatched against does not take
    /// its warnings away: they say the run was measured on another design.
    #[test]
    fn an_edit_stales_the_runs_warnings_rather_than_withdrawing_them() {
        let mut state = open_state(&[1]);
        state.open_run_outcome(1);
        state.publish_run_notices("Transient", vec![clamp("M1", "rdsw", &[])], None, &[], 1);
        let batch = state
            .diagnostics
            .batch(&DiagnosticInputs::Run { run_sequence: 1 })
            .expect("the run's batch")
            .clone();
        assert!(state.diagnostic_batch_standing(&batch).is_current());

        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(0, 0));

        assert_eq!(
            state.diagnostic_batch_standing(&batch),
            BatchStanding::Stale
        );
        assert_eq!(notices(&state).len(), 1, "the warning is kept, stale");
        let listed = listing::listing(&state);
        assert!(listed.counts().stale > 0, "{:?}", listed.counts());
    }

    /// A clamp raised at every point of a sweep and in every analysis of the
    /// run is one record that counts them and names where, not a row per
    /// point; a second parameter is a second condition.
    #[test]
    fn a_sweep_raises_one_record_per_condition_with_the_points_it_occurred_at() {
        let mut state = open_state(&[1]);
        state.open_run_outcome(1);
        let trials: Vec<String> = (1..=10).map(|trial| format!("trial {trial}")).collect();
        let trials: Vec<&str> = trials.iter().map(String::as_str).collect();
        state.publish_run_notices(
            "Monte Carlo",
            vec![
                clamp("M1", "rdsw", &trials),
                RecordedNotice {
                    notice: EngineNotice::ConvergenceAid(ContinuationAid::GminStepping),
                    occurrences: 3,
                    points: Vec::new(),
                    unnamed_points: 0,
                },
            ],
            None,
            &[],
            1,
        );
        state.publish_run_notices(
            "Transient",
            vec![clamp("m1", "rdsw", &[]), clamp("M1", "prwg", &[])],
            None,
            &[],
            1,
        );

        let standing = notices(&state);
        assert_eq!(standing.len(), 3, "{standing:#?}");
        let (code, subject, details) = &standing[0];
        assert_eq!(code, PARAMETER_CODE);
        assert_eq!(subject, "M1");
        assert_eq!(
            details,
            "Raised 11 times in Monte Carlo, Transient · at trial 1, trial 2, trial 3, trial 4, \
             trial 5, trial 6, trial 7, trial 8 and 2 more · engine element M1"
        );
        assert_eq!(standing[1].0, AID_CODE);
        assert_eq!(standing[2].0, PARAMETER_CODE);
        let listed = listing::listing(&state);
        assert_eq!(listed.len(), 3);
        assert_eq!(listed.counts().warnings, 2, "{:?}", listed.counts());
    }

    /// A `.MEASURE` that produced no value is a warning about the run, in
    /// the engine's words; one that produced a value is not.
    #[test]
    fn a_measurement_that_never_triggered_is_a_warning() {
        let mut state = open_state(&[1]);
        state.open_run_outcome(1);
        let failed = rspice_core::MeasureResult::failed("tdelay", "TRIG never crossed 0.5");
        let mut measured = rspice_core::MeasureResult::failed("tpeak", "");
        measured.error = None;
        measured.value = Some(1.0);
        state.publish_run_notices("Transient", Vec::new(), None, &[failed, measured], 1);

        let standing = notices(&state);
        assert_eq!(standing.len(), 1, "{standing:?}");
        assert_eq!(standing[0].0, MEASURE_CODE);
        assert_eq!(standing[0].1, "tdelay");
    }

    /// A clamp states the bound the value crossed; a reset with no bound, A1
    /// zeroed because A2 was out of range, says only that the value was
    /// replaced, because its own value was not out of anything.
    #[test]
    fn a_replacement_states_its_bound_only_where_the_model_states_one() {
        let replacement = |parameter: &str, requested: f64, applied: f64, limit: Option<f64>| {
            replacement_message(&ParameterReplacement {
                family: "BSIM3".to_owned(),
                model: Some("nch".to_owned()),
                element: Some("M1".to_owned()),
                scope: ParameterScope::Instance,
                parameter: parameter.to_owned(),
                requested,
                applied,
                limit,
            })
        };
        assert_eq!(
            replacement("a2", 2.0, 1.0, Some(1.0)),
            "BSIM3 a2 = 2 is above 1; the model uses 1"
        );
        assert_eq!(
            replacement("a1", 0.5, 0.0, None),
            "BSIM3 a1 = 500m is replaced; the model uses 0"
        );
    }

    /// The convergence evidence of a transient whose stepper accepted the
    /// points at `accepted` (ascending indices into a 1 ns grid) past the
    /// truncation-error test.
    fn transient_accepting(accepted: &[usize]) -> TransientConvergenceEvidence {
        let mut quality = rspice_core::diagnostics::ConvergenceQuality::default();
        for &index in accepted {
            quality.record_force_accept(index);
        }
        let time: Vec<f64> = (0..10).map(|step| f64::from(step) * 1e-9).collect();
        TransientConvergenceEvidence::capture(quality, &time, &rspice_core::abort_signal::NoAbort)
            .expect("the fixture's evidence captures")
    }

    /// Points a transient accepted past its truncation-error tolerance are a
    /// warning about the run's accuracy, counted over every transient that
    /// accepted any and dated by the first; a transient that accepted none
    /// raises nothing.
    #[test]
    fn points_accepted_past_the_truncation_tolerance_are_one_warning_for_the_run() {
        let mut state = open_state(&[1]);
        state.open_run_outcome(1);
        state.publish_run_notices(
            "Transient",
            Vec::new(),
            Some(&transient_accepting(&[])),
            &[],
            1,
        );
        assert!(notices(&state).is_empty(), "nothing was accepted past it");

        state.publish_run_notices(
            "Transient",
            Vec::new(),
            Some(&transient_accepting(&[3, 7])),
            &[],
            1,
        );
        state.publish_run_notices(
            "Transient 2",
            Vec::new(),
            Some(&transient_accepting(&[5])),
            &[],
            1,
        );

        let records: Vec<&DiagnosticRecord> = state
            .diagnostics
            .batches()
            .flat_map(|batch| batch.records.iter())
            .filter(|record| is_notice(record))
            .collect();
        assert_eq!(records.len(), 1, "{records:#?}");
        let record = records[0];
        assert_eq!(record.canonical.code.as_ref(), TRUNCATION_CODE);
        assert_eq!(record.level, DiagnosticLevel::Warning);
        assert_eq!(record.subject.object, "Transient, Transient 2");
        assert_eq!(
            record.message,
            "Time points were accepted past the local truncation-error tolerance, the first near \
             t = 3ns"
        );
        assert_eq!(record.details, "Raised 3 times in Transient, Transient 2");
    }

    /// A root drawing that places `amp` twice, as `amp` and `amp2`, and a
    /// memristor, over an `amp` master that draws `M1` and a memristor of its
    /// own.
    fn hierarchical_design() -> AppState {
        let mut state = open_state(&[1]);
        let mut amp = SchematicState::default();
        amp.document_mut_for_test().components.extend([
            Component::new(10, ComponentType::Nmos, Point::new(40, 60))
                .with_name_value("M1", "nch"),
            Component::new(11, ComponentType::Memristor, Point::new(80, 60))
                .with_name_value("MR1", "1"),
        ]);
        let mut work = Library::new("work");
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        work.add_cell(cell);
        state.library_manager.add_library(work);
        state
            .workspace
            .insert_schematic_editor(CellViewRef::new("work", "amp", "schematic").key(), amp);
        let active = state.workspace.content.active_view.clone();
        if state.library_manager.get_library(&active.library).is_none() {
            let mut top = Library::new(&active.library);
            let mut top_cell = Cell::new(&active.cell);
            top_cell.add_view(View::new(&active.view, ViewType::Schematic));
            top.add_cell(top_cell);
            state.library_manager.add_library(top);
        }
        let placed = |id: u64, name: &str, x: i32| {
            let mut instance = Component::new(id, ComponentType::CellInstance, Point::new(x, 0))
                .with_library_cell(LibraryCellInstance::new("work", "amp", "schematic"));
            instance.name = name.to_owned();
            instance
        };
        state.schematic.document_mut_for_test().components.extend([
            placed(1, "amp", 0),
            placed(3, "amp2", 400),
            Component::new(2, ComponentType::Memristor, Point::new(200, 60))
                .with_name_value("MR1", "1"),
        ]);
        state.sync_active_schematic_to_workspace();
        state
    }

    /// The engine names an element inside a sub-cell by the written names
    /// above it; the record finds the component the `amp` master draws, and
    /// routes to it in that master rather than in the root. The master is
    /// placed twice, so each occurrence's clamp is its own record, named by
    /// its occurrence, and both lead to the one `M1` the master draws.
    #[test]
    fn an_engine_name_in_a_sub_cell_locates_the_drawn_instance_under_its_occurrence() {
        let mut state = hierarchical_design();
        let amp = CellViewRef::new("work", "amp", "schematic");

        let located = state
            .locate_engine_element("xamp.m1")
            .expect("XAMP.M1 is the M1 that /amp draws");
        assert_eq!(located.occurrence.to_string(), "/amp");
        assert_eq!(located.cell_view.key(), amp.key());
        assert_eq!(located.component, 10);
        assert_eq!(located.name, "M1");
        assert_eq!(
            state
                .locate_engine_element("XAMP2.M1")
                .map(|located| (located.occurrence.to_string(), located.component)),
            Some(("/amp2".to_owned(), 10))
        );
        assert_eq!(state.locate_engine_element("m1"), None, "no M1 at the root");

        state.open_run_outcome(1);
        state.publish_run_notices(
            "Operating point",
            vec![
                clamp("XAMP.M1", "rdsw", &[]),
                clamp("XAMP2.M1", "rdsw", &[]),
            ],
            None,
            &[],
            1,
        );
        let records: Vec<&DiagnosticRecord> = state
            .diagnostics
            .batches()
            .flat_map(|batch| batch.records.iter())
            .filter(|record| is_notice(record))
            .collect();
        let subjects: Vec<&str> = records
            .iter()
            .map(|record| record.subject.object.as_str())
            .collect();
        assert_eq!(subjects, ["/amp/M1", "/amp2/M1"]);
        for record in records {
            assert_eq!(
                record.subject.cell_view.as_ref().map(CellViewRef::key),
                Some(amp.key())
            );
            assert!(
                matches!(
                    &record.route,
                    Some(DiagnosticRoute::Anchor(LogAnchor::Schematic {
                        cell_view,
                        component: Some(10),
                        ..
                    })) if cell_view.key() == amp.key()
                ),
                "{:?}",
                record.route
            );
        }
    }

    /// A memristor is known to the engine under its keyword, `YMEMRISTOR!MR1`,
    /// and `MR1` alone would name a MOSFET; the drawn `MR1` is found either
    /// way it is reached.
    #[test]
    fn a_memristor_is_located_by_the_name_the_engine_knows_it_by() {
        let state = hierarchical_design();
        assert_eq!(
            state
                .locate_engine_element("YMEMRISTOR!MR1")
                .map(|located| located.component),
            Some(2)
        );
        assert_eq!(
            state
                .locate_engine_element("xamp.ymemristor!mr1")
                .map(|located| located.component),
            Some(11)
        );
        assert_eq!(state.locate_engine_element("MR1"), None);
    }
}
