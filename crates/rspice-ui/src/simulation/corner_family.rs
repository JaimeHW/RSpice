//! The plotting family a corner declaration produces, as a view over the
//! points it declared.
//!
//! A corner run expands into one task per declared PVT point, and each of those
//! results keeps its own waveforms, its own `.MEAS` evaluation and the point it
//! was solved at. The family a corner plot reads — one scalar per node against
//! a shared corner axis — is a reduction of exactly those results.
//!
//! The declaration keeps its own task, so the run's authenticated receipt still
//! has an entry for it and the retained results still line up with that receipt
//! one for one. What changed is what its turn costs: it assembles the family
//! from the points instead of solving the whole declared space again.

use std::collections::HashMap;

use rspice_core::NoAbort;

use crate::product::AnalysisInstanceId;
use crate::services::simulation_runner::{
    CornerBaseMode, CornerPoint, SweepPointResult, map_corner_results,
};
use crate::simulation::SimulationResult;
use crate::simulation::execution::AuthorizedTaskDispatch;
use crate::state::{AnalysisResult, AnalysisResultPayload, SimulationRun};

/// Ground, which the corner mappers require at index 0 and never plot.
const GROUND_NODE: &str = "0";

/// What a corner point's task cannot state about the declaration it belongs to.
///
/// Frozen onto every point task during preparation, because the declaration's
/// own task never reaches an executor and there is nothing else left to ask for
/// its base analysis or how many points it declared.
#[derive(Debug, Clone)]
pub(in crate::simulation) struct CornerRunPoint {
    base_mode: CornerBaseMode,
    declared_points: usize,
    /// Position in the declared space. The family's axis, labels and per-node
    /// samples are all positional, so pairing a result with its point is done
    /// on this index and never on the order results happen to arrive in.
    index: usize,
    point: CornerPoint,
}

impl CornerRunPoint {
    pub(in crate::simulation) fn new(
        base_mode: CornerBaseMode,
        declared_points: usize,
        index: usize,
        point: CornerPoint,
    ) -> Self {
        Self {
            base_mode,
            declared_points,
            index,
            point,
        }
    }
}

/// Every corner declaration in one authorized run, and the tasks that solve
/// their points.
#[derive(Debug, Default)]
pub(in crate::simulation) struct CornerFamilyRegistry {
    declarations: Vec<CornerDeclaration>,
}

#[derive(Debug)]
struct CornerDeclaration {
    authored_instance_id: AnalysisInstanceId,
    base_mode: CornerBaseMode,
    declared_points: usize,
    points: Vec<(usize, CornerPoint, AnalysisInstanceId)>,
}

impl CornerFamilyRegistry {
    /// Record a dispatched task that solves one point of a corner declaration.
    /// Every other task is ignored, including the declaration's own.
    pub(in crate::simulation) fn register(&mut self, task: &AuthorizedTaskDispatch) {
        let Some(corner_point) = task.corner_point() else {
            return;
        };
        let authored = task.authored_instance_id();
        let position = self
            .declarations
            .iter()
            .position(|declaration| declaration.authored_instance_id == authored);
        let position = match position {
            Some(position) => position,
            None => {
                self.declarations.push(CornerDeclaration {
                    authored_instance_id: authored,
                    base_mode: corner_point.base_mode.clone(),
                    declared_points: corner_point.declared_points,
                    points: Vec::new(),
                });
                self.declarations.len().saturating_sub(1)
            }
        };
        self.declarations[position].points.push((
            corner_point.index,
            corner_point.point,
            task.instance_id(),
        ));
    }

    pub(in crate::simulation) fn clear(&mut self) {
        self.declarations.clear();
    }

    /// The family a corner declaration's turn produces, read off the point
    /// results already retained in the run.
    ///
    /// `Err` where the corner executor would have failed the whole analysis:
    /// a declaration none of whose points converged is a failed result, not a
    /// missing one, because a corner plot reporting nothing is indistinguishable
    /// from a corner run nobody asked for.
    pub(in crate::simulation) fn family_for(
        &self,
        declaration: AnalysisInstanceId,
        run: &SimulationRun,
    ) -> Result<SimulationResult, String> {
        let Some(declaration) = self
            .declarations
            .iter()
            .find(|candidate| candidate.authored_instance_id == declaration)
        else {
            return Err("Corner declaration expanded into no points to assemble".to_owned());
        };
        declaration.family(run)
    }
}

impl CornerDeclaration {
    fn family(&self, run: &SimulationRun) -> Result<SimulationResult, String> {
        let mut points = self.points.clone();
        points.sort_by_key(|(index, _, _)| *index);
        let solved = points
            .iter()
            .filter_map(|(_, point, instance)| {
                run.analyses
                    .iter()
                    .find(|analysis| {
                        analysis
                            .provenance
                            .as_ref()
                            .is_some_and(|provenance| provenance.source_instance_id() == *instance)
                    })
                    .map(|analysis| (*point, analysis))
            })
            .collect::<Vec<_>>();
        corner_family_of_points(&self.base_mode, self.declared_points, &solved)
    }
}

/// The corner family a set of solved points adds up to.
///
/// `solved` arrives in declaration order and may include points that failed;
/// a point that did not converge contributes nothing to the axis and one to the
/// failure count, which is what the corner executor did when it dropped a
/// point's result.
pub(in crate::simulation) fn corner_family_of_points(
    base_mode: &CornerBaseMode,
    declared_points: usize,
    solved: &[(CornerPoint, &AnalysisResult)],
) -> Result<SimulationResult, String> {
    let converged = converged_point_results(base_mode, solved);
    if converged.is_empty() {
        return Err("Corner analysis produced no converged corner points".to_owned());
    }
    let (x_values, x_label, x_unit, temperatures_c, corner_labels, voltages) =
        map_corner_results(&converged, base_mode.metric_label(), &NoAbort)
            .map_err(|error| error.to_string())?;

    let mut waveforms = HashMap::with_capacity(voltages.len());
    for (name, values) in voltages {
        waveforms.insert(
            name.clone(),
            crate::simulation::results::WaveformData {
                name,
                x_values: x_values.clone(),
                y_values: values,
                y_unit: "V".to_owned(),
                is_complex: false,
                y_imag: None,
            },
        );
    }

    Ok(SimulationResult::Corner {
        x_values,
        x_label,
        x_unit,
        temperatures_c,
        corner_labels,
        waveforms,
        num_failures: declared_points.saturating_sub(converged.len()),
    })
}

/// One scalar per node for every point that converged, all of them stating
/// their nodes in the same order.
///
/// The mappers pair a trace name taken from the first point with a sample
/// taken from every point by index, so a point that listed its nodes in a
/// different order would relabel every trace after it. The order is therefore
/// imposed here — the first converged point's node list, by name — and a later
/// point that is missing one of those nodes reads zero for it rather than
/// shifting the rest.
fn converged_point_results(
    base_mode: &CornerBaseMode,
    solved: &[(CornerPoint, &AnalysisResult)],
) -> Vec<(CornerPoint, SweepPointResult)> {
    let mut collected = Vec::with_capacity(solved.len());
    for (point, analysis) in solved {
        if !analysis.success {
            continue;
        }
        let Some(values) = point_node_values(analysis, base_mode) else {
            continue;
        };
        collected.push((*point, values));
    }

    let Some((_, first)) = collected.first() else {
        return Vec::new();
    };
    let mut node_names = Vec::with_capacity(first.len().saturating_add(1));
    node_names.push(GROUND_NODE.to_owned());
    node_names.extend(first.iter().map(|(name, _)| name.clone()));

    collected
        .into_iter()
        .map(|(point, values)| {
            let by_name = values.into_iter().collect::<HashMap<_, _>>();
            let node_values = node_names
                .iter()
                .enumerate()
                .map(|(index, name)| {
                    if index == 0 {
                        0.0
                    } else {
                        by_name.get(name).copied().unwrap_or(0.0)
                    }
                })
                .collect();
            (
                point,
                SweepPointResult {
                    node_names: node_names.clone(),
                    node_values,
                },
            )
        })
        .collect()
}

/// The scalar each node contributed at one corner, named the way the deck
/// names the node, sorted so every point agrees on an order.
///
/// The reduction is the base analysis's own terminal value: the converged
/// operating point, the last swept point, the last transient sample, or the
/// magnitude at the terminal frequency.
fn point_node_values(
    analysis: &AnalysisResult,
    base_mode: &CornerBaseMode,
) -> Option<Vec<(String, f64)>> {
    let mut values = match base_mode {
        CornerBaseMode::Op => operating_point_node_values(analysis)?,
        CornerBaseMode::Ac { .. } => terminal_ac_magnitudes(analysis),
        CornerBaseMode::DcSweep { .. } | CornerBaseMode::Transient { .. } => {
            terminal_node_samples(analysis)
        }
    };
    if values.is_empty() {
        return None;
    }
    values.sort_by(|left, right| left.0.cmp(&right.0));
    Some(values)
}

/// The operating point reads its retained MNA ordering rather than its node
/// map: the map is unordered, and two points that enumerated it differently
/// would swap their traces.
fn operating_point_node_values(analysis: &AnalysisResult) -> Option<Vec<(String, f64)>> {
    let Some(AnalysisResultPayload::OperatingPoint {
        mna_node_names,
        mna_solution,
        ..
    }) = analysis.result_payload.as_ref()
    else {
        return None;
    };
    Some(
        mna_node_names
            .iter()
            .zip(mna_solution.iter())
            .map(|(name, value)| (name.clone(), *value))
            .collect(),
    )
}

fn terminal_node_samples(analysis: &AnalysisResult) -> Vec<(String, f64)> {
    analysis
        .waveforms
        .iter()
        .filter(|waveform| !is_branch_current(&waveform.name))
        .filter_map(|waveform| {
            waveform
                .y
                .last()
                .map(|value| (waveform.name.clone(), *value))
        })
        .collect()
}

/// AC contributes the magnitude at the terminal frequency, recomputed from the
/// retained complex samples so it is the same quantity the corner executor
/// took from the solver rather than a re-derivation of the display trace.
fn terminal_ac_magnitudes(analysis: &AnalysisResult) -> Vec<(String, f64)> {
    analysis
        .waveforms
        .iter()
        .filter_map(|waveform| {
            let node = waveform
                .name
                .strip_prefix("|V(")
                .and_then(|inner| inner.strip_suffix(")|"))?;
            let complex = waveform.complex.as_ref()?;
            let real = complex.real.last()?;
            let imaginary = complex.imag.last()?;
            Some((node.to_owned(), real.hypot(*imaginary)))
        })
        .collect()
}

/// A corner family is one scalar per node, so a branch-current trace beside
/// them is not a node and must not become one.
fn is_branch_current(name: &str) -> bool {
    (name.starts_with("I(") || name.starts_with("i(")) && name.ends_with(')')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::simulation_runner::{
        CornerBaseMode, CornerFrequencySweep, CornerProcess, CornerRunConfig,
    };
    use crate::state::{AnalysisResultFamilyMetadata, AnalysisType};

    /// A 1:1 resistive divider on a supply the corner axis derates by 10%, so
    /// every reduction below has an answer that can be written down: the output
    /// is half the supply its point was solved at.
    const DIVIDER: &str = "corner family\n\
         VDD vdd 0 DC 1.8 AC 1\n\
         R1 vdd out 1k\n\
         R2 out 0 1k\n\
         C1 out 0 1p\n\
         .op\n\
         .end\n";

    fn contract(base_mode: CornerBaseMode) -> CornerRunConfig {
        CornerRunConfig {
            process_corners: vec![CornerProcess::TT],
            voltages: vec![1.8, 1.62],
            temperatures_c: vec![27.0, 85.0],
            full_matrix: true,
            nominal_voltage: Some(1.8),
            base_mode,
            model_bindings: Vec::new(),
            points: Vec::new(),
        }
    }

    /// The declaration's family and its traces, sorted by trace name.
    fn family(
        base_mode: CornerBaseMode,
    ) -> (AnalysisResultFamilyMetadata, Vec<(String, Vec<f64>)>) {
        let run = crate::simulation::runner::pvt_point_evidence::run_corner_declaration(
            DIVIDER,
            contract(base_mode),
            27.0,
        )
        .expect("the corner declaration prepares, authorizes and runs");
        let analysis = run
            .analyses
            .iter()
            .find(|analysis| analysis.analysis_type == AnalysisType::Corner)
            .expect("the declaration produces a corner family");
        assert!(
            analysis.success,
            "corner family failed: {:?}",
            analysis.error_message
        );
        let mut traces = analysis
            .waveforms
            .iter()
            .map(|waveform| (waveform.name.clone(), waveform.y.as_ref().clone()))
            .collect::<Vec<_>>();
        traces.sort_by(|left, right| left.0.cmp(&right.0));
        (
            analysis
                .family_metadata
                .clone()
                .expect("a corner family carries its axis"),
            traces,
        )
    }

    fn assert_close(actual: &[f64], expected: &[f64], what: &str) {
        assert_eq!(actual.len(), expected.len(), "{what}: {actual:?}");
        for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
            assert!(
                (actual - expected).abs() < 1.0e-6,
                "{what} point {index}: expected {expected}, got {actual}"
            );
        }
    }

    fn trace_names(traces: &[(String, Vec<f64>)]) -> Vec<&str> {
        traces.iter().map(|(name, _)| name.as_str()).collect()
    }

    /// The four declared points share a process and differ in supply, so the
    /// axis cannot be temperature and is the corner index instead.
    #[test]
    fn the_axis_is_the_declared_space_itself() {
        let (metadata, _) = family(CornerBaseMode::Op);

        assert_eq!(
            metadata,
            AnalysisResultFamilyMetadata::Corner {
                x_values: vec![0.0, 1.0, 2.0, 3.0],
                x_label: "Corner Index".to_owned(),
                x_unit: String::new(),
                temperatures_c: vec![27.0, 85.0, 27.0, 85.0],
                corner_labels: vec![
                    "TT_1.800000V_27.000000C".to_owned(),
                    "TT_1.800000V_85.000000C".to_owned(),
                    "TT_1.620000V_27.000000C".to_owned(),
                    "TT_1.620000V_85.000000C".to_owned(),
                ],
                failed_corners: 0,
            }
        );
    }

    #[test]
    fn an_operating_point_corner_contributes_its_converged_node_voltages() {
        let (_, traces) = family(CornerBaseMode::Op);

        assert_eq!(trace_names(&traces), vec!["V(OUT)", "V(VDD)"]);
        assert_close(&traces[0].1, &[0.9, 0.9, 0.81, 0.81], "V(OUT)");
        assert_close(&traces[1].1, &[1.8, 1.8, 1.62, 1.62], "V(VDD)");
    }

    /// The sweep drives the same supply the corner derates, so the terminal
    /// swept point overrides the corner value and every point answers alike.
    /// That is exactly the reduction under test: the last swept point.
    #[test]
    fn a_dc_sweep_corner_contributes_its_terminal_swept_point() {
        let (_, traces) = family(CornerBaseMode::DcSweep {
            source_name: "VDD".to_owned(),
            start: 0.0,
            stop: 1.0,
            step: 0.5,
        });

        assert_eq!(trace_names(&traces), vec!["V(OUT)", "V(VDD)"]);
        assert_close(&traces[0].1, &[0.5, 0.5, 0.5, 0.5], "V(OUT)");
        assert_close(&traces[1].1, &[1.0, 1.0, 1.0, 1.0], "V(VDD)");
    }

    /// The divider settles long before 100 ns, so the terminal sample is the
    /// DC answer. Both nodes appear: the corner executor used to drop whichever
    /// node a transient result listed first, because it skipped index zero on
    /// the one base mode whose node vector carries no ground.
    #[test]
    fn a_transient_corner_contributes_its_terminal_sample_for_every_node() {
        let (_, traces) = family(CornerBaseMode::Transient {
            stop_time: 100.0e-9,
            step_time: 1.0e-9,
        });

        assert_eq!(trace_names(&traces), vec!["V(OUT)", "V(VDD)"]);
        assert_close(&traces[0].1, &[0.9, 0.9, 0.81, 0.81], "V(OUT)");
        assert_close(&traces[1].1, &[1.8, 1.8, 1.62, 1.62], "V(VDD)");
    }

    /// AC contributes magnitude at the terminal frequency and names its traces
    /// as magnitudes. The terminal frequency is what makes this assertable: the
    /// divider is loaded by 1 pF against 500 ohm of source resistance, so the
    /// answer at 1 MHz is the divider ratio rolled off by that pole and nothing
    /// like the answer at 1 kHz. The supply corner scales DC values and leaves
    /// the AC drive alone, so every point reads the same.
    #[test]
    fn an_ac_corner_contributes_terminal_frequency_magnitude() {
        let terminal = 2.0 * std::f64::consts::PI * 1.0e6 * 500.0 * 1.0e-12;
        let rolled_off = 0.5 / (1.0 + terminal * terminal).sqrt();

        let (_, traces) = family(CornerBaseMode::Ac {
            start_freq: 1.0e3,
            stop_freq: 1.0e6,
            points_per_unit: 5,
            sweep: CornerFrequencySweep::Decade,
        });

        assert_eq!(trace_names(&traces), vec!["|V(OUT)|", "|V(VDD)|"]);
        assert_close(&traces[0].1, &[rolled_off; 4], "|V(OUT)|");
        assert_close(&traces[1].1, &[1.0, 1.0, 1.0, 1.0], "|V(VDD)|");
        assert!(
            (traces[0].1[0] - 0.5).abs() > 1.0e-9,
            "the terminal frequency must not read like the first one"
        );
    }

    /// Four declared points, four solves, and a family that costs no solve of
    /// its own — it carries no PVT point because it did not solve one.
    #[test]
    fn the_family_is_assembled_from_the_points_rather_than_solved_again() {
        let run = crate::simulation::runner::pvt_point_evidence::run_corner_declaration(
            DIVIDER,
            contract(CornerBaseMode::Op),
            27.0,
        )
        .expect("the corner declaration prepares, authorizes and runs");

        let attributed = run
            .analyses
            .iter()
            .filter(|analysis| {
                analysis
                    .provenance
                    .as_ref()
                    .and_then(crate::state::AnalysisResultProvenance::pvt_point)
                    .is_some()
            })
            .count();
        assert_eq!(attributed, 4, "one solve per declared point, and no more");
        assert_eq!(
            run.analyses.len(),
            5,
            "the four points and the one family they add up to"
        );
        let families = run
            .analyses
            .iter()
            .filter(|analysis| analysis.analysis_type == AnalysisType::Corner)
            .collect::<Vec<_>>();
        assert_eq!(families.len(), 1);
        assert!(
            families[0]
                .provenance
                .as_ref()
                .expect("the family answers for the declaration")
                .pvt_point()
                .is_none(),
            "the family is not a point and must never claim one"
        );
    }

    /// A declaration none of whose points converged is a failed corner result,
    /// not a missing one.
    #[test]
    fn a_declaration_with_no_converged_point_is_a_failure() {
        let outcome = corner_family_of_points(&CornerBaseMode::Op, 3, &[]);

        assert_eq!(
            outcome.expect_err("nothing converged"),
            "Corner analysis produced no converged corner points"
        );
    }

    fn corner_run() -> crate::state::SimulationRun {
        crate::simulation::runner::pvt_point_evidence::run_corner_declaration(
            DIVIDER,
            contract(CornerBaseMode::Op),
            27.0,
        )
        .expect("the corner declaration prepares, authorizes and runs")
    }

    /// A run's retained results have to be an exact ordered prefix of the task
    /// graph its receipt authenticated, and that is checked on project load as a
    /// hard error rather than a warning. The family is a result, so it has to be
    /// a task: while it was assembled outside the task list, a saved corner run
    /// carried one more result than it had authenticated tasks and the project
    /// refused to reopen.
    #[test]
    fn a_corner_run_is_an_authentic_prefix_of_its_receipt_and_survives_a_project_round_trip() {
        use crate::io::project_io::ProjectSimulationResults;

        let run = corner_run();
        run.validate_provenance()
            .expect("every retained result answers for an authenticated task");
        assert_eq!(
            run.prepared_receipt()
                .expect("the run is sealed by its dispatch")
                .tasks()
                .len(),
            run.analyses.len(),
            "four points and the family they add up to, against five authenticated tasks"
        );

        // The exact shape that broke project load: one result more than the
        // receipt authenticates is refused outright, and a family assembled
        // outside the task list is precisely that result.
        let mut surplus = run.clone();
        let duplicate = surplus.analyses[0].clone();
        surplus.analyses.push(duplicate);
        surplus
            .validate_provenance()
            .expect_err("a result with no authenticated task cannot be retained");

        let mut simulation = crate::state::SimulationState::default();
        simulation.next_run_id = run.id;
        simulation.runs = vec![run];

        let persisted = ProjectSimulationResults::from_state(&simulation);
        persisted.validate().expect("a corner run is persistable");

        let mut reloaded = crate::state::SimulationState::default();
        persisted
            .apply_to_state(&mut reloaded)
            .expect("a saved corner run reopens");

        let restored = reloaded.runs.first().expect("the run survives the reload");
        assert_eq!(restored.analyses.len(), simulation.runs[0].analyses.len());
        assert!(
            restored
                .analyses
                .iter()
                .any(|analysis| analysis.analysis_type == AnalysisType::Corner
                    && matches!(
                        analysis.family_metadata,
                        Some(AnalysisResultFamilyMetadata::Corner { .. })
                    )),
            "the corner plot's axis survives the round trip"
        );
        restored
            .validate_provenance()
            .expect("the reloaded run is still an authentic prefix");
    }

    /// A run that stopped part-way keeps the same property. This is why the
    /// assembly is ordered last: every truncation of the result list is still a
    /// prefix of the task list, whereas an assembly ordered first could not
    /// produce its result until its points had and would leave a hole.
    #[test]
    fn a_corner_run_that_stopped_part_way_is_still_an_authentic_prefix() {
        let complete = corner_run();
        assert_eq!(complete.analyses.len(), 5);

        for retained in 0..complete.analyses.len() {
            let mut partial = complete.clone();
            partial.analyses.truncate(retained);
            partial.validate_provenance().unwrap_or_else(|error| {
                panic!("a run that stopped after {retained} result(s) must still validate: {error}")
            });
        }
    }
}
