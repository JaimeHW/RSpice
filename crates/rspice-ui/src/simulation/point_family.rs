//! The plotting family a PVT declaration produces, as a view over the points
//! it declared.
//!
//! A corner run and a temperature step are the same shape with different axes:
//! a base analysis solved once at every point of a declared space. Each
//! declaration expands into one task per point, and each of those results keeps
//! its own waveforms, its own `.MEAS` evaluation and the point it was solved
//! at. The family a plot reads — one scalar per node against the declaration's
//! axis — is a reduction of those solved results, captured before their
//! authored output projection and admitted only for retained successful points.
//!
//! Each declaration keeps its own task, so the run's authenticated receipt
//! still has an entry for it and the retained results still line up with that
//! receipt one for one. What changed is what its turn costs: it assembles the
//! family from the points instead of solving the whole declared space again.
//!
//! The reduction is shared and the assembly is not. Reducing a point result to
//! one scalar per node is a question about the base analysis, which both axes
//! spell the same way; what the scalars are laid against is the axis itself,
//! and a corner index is not a temperature.

use std::collections::HashMap;

use rspice_core::{NoAbort, Value};

use crate::product::AnalysisInstanceId;
use crate::services::simulation_runner::{
    CornerBaseMode, CornerPoint, SweepPointResult, map_corner_results, map_temperature_results,
};
use crate::simulation::SimulationResult;
use crate::simulation::execution::AuthorizedTaskDispatch;
use crate::simulation::results::WaveformData;
use crate::state::{AnalysisResult, AnalysisResultPayload, SimulationRun};

/// Ground, which the sweep mappers require at index 0 and never plot.
const GROUND_NODE: &str = "0";

/// What a temperature family names its axis. The deck-driven parametric runner
/// spells the same target for a `.STEP TEMP`, so a plot cannot tell a
/// temperature step apart by where it came from.
const TEMPERATURE_TARGET: &str = "TEMP";

/// Where one point sits in the space its declaration declared.
///
/// A corner point states the whole PVT triple because its family's axis, its
/// labels and its own temperature column are all read back off it. A
/// temperature point states only the temperature, because that is the entire
/// condition a temperature step varies; naming a supply it never scaled would
/// be attributing the result to a corner it did not solve.
#[derive(Debug, Clone, Copy)]
pub(in crate::simulation) enum DeclaredAxisPoint {
    Corner(CornerPoint),
    Temperature(Value),
}

/// What a point's task cannot state about the declaration it belongs to.
///
/// Frozen onto every point task during preparation, because the declaration's
/// own task never reaches an executor and there is nothing else left to ask for
/// its base analysis or how many points it declared.
#[derive(Debug, Clone)]
pub(in crate::simulation) struct DeclaredRunPoint {
    base_mode: CornerBaseMode,
    declared_points: usize,
    /// Position in the declared space. The family's axis, labels and per-node
    /// samples are all positional, so pairing a result with its point is done
    /// on this index and never on the order results happen to arrive in.
    index: usize,
    point: DeclaredAxisPoint,
}

impl DeclaredRunPoint {
    pub(in crate::simulation) fn new(
        base_mode: CornerBaseMode,
        declared_points: usize,
        index: usize,
        point: DeclaredAxisPoint,
    ) -> Self {
        Self {
            base_mode,
            declared_points,
            index,
            point,
        }
    }
}

/// Every PVT declaration in one authorized run, and the tasks that solve their
/// points.
#[derive(Debug, Default)]
pub(in crate::simulation) struct PointFamilyRegistry {
    declarations: Vec<PointDeclaration>,
    reductions: HashMap<AnalysisInstanceId, PointReduction>,
}

/// Only terminal scalars survive output projection here. A retained successful
/// point must still answer its authorized task before its scalars enter a family.
#[derive(Debug)]
struct PointReduction {
    declaration: usize,
    result: Option<Result<SweepPointResult, String>>,
}

#[derive(Debug)]
struct PointDeclaration {
    authored_instance_id: AnalysisInstanceId,
    base_mode: CornerBaseMode,
    declared_points: usize,
    space: DeclaredSpace,
}

/// The points of one declaration, kept in the shape its own family assembler
/// reads. A declaration expands one space, so its points are all of one kind
/// and the two vectors are never both populated.
#[derive(Debug)]
enum DeclaredSpace {
    Corner(Vec<(usize, CornerPoint, AnalysisInstanceId)>),
    Temperature(Vec<(usize, Value, AnalysisInstanceId)>),
}

impl DeclaredSpace {
    fn empty_like(point: DeclaredAxisPoint) -> Self {
        match point {
            DeclaredAxisPoint::Corner(_) => Self::Corner(Vec::new()),
            DeclaredAxisPoint::Temperature(_) => Self::Temperature(Vec::new()),
        }
    }

    fn push(&mut self, index: usize, point: DeclaredAxisPoint, instance: AnalysisInstanceId) {
        match (self, point) {
            (Self::Corner(points), DeclaredAxisPoint::Corner(point)) => {
                points.push((index, point, instance));
            }
            (Self::Temperature(points), DeclaredAxisPoint::Temperature(point)) => {
                points.push((index, point, instance));
            }
            // One expansion produces one kind of point, so the declaration's
            // space and its points cannot disagree. Dropping the point rather
            // than panicking keeps a preparation fault out of the run loop; it
            // surfaces as a family short of a point.
            _ => {}
        }
    }
}

impl PointFamilyRegistry {
    /// Record a dispatched task that solves one point of a declaration. Every
    /// other task is ignored, including the declaration's own.
    pub(in crate::simulation) fn register(&mut self, task: &AuthorizedTaskDispatch) {
        let Some(declared) = task.declared_point() else {
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
                self.declarations.push(PointDeclaration {
                    authored_instance_id: authored,
                    base_mode: declared.base_mode.clone(),
                    declared_points: declared.declared_points,
                    space: DeclaredSpace::empty_like(declared.point),
                });
                self.declarations.len().saturating_sub(1)
            }
        };
        self.declarations[position]
            .space
            .push(declared.index, declared.point, task.instance_id());
        self.reductions.insert(
            task.instance_id(),
            PointReduction {
                declaration: position,
                result: None,
            },
        );
    }

    /// Capture before saved-output aliases, sampling, or retention discard the
    /// engine basis. No full waveform arrays are retained or cloned here.
    pub(in crate::simulation) fn capture_result(
        &mut self,
        instance: AnalysisInstanceId,
        analysis: &AnalysisResult,
    ) {
        let Some(reduction) = self.reductions.get_mut(&instance) else {
            return;
        };
        reduction.result = analysis.success.then(|| {
            let values = point_node_values(
                analysis,
                &self.declarations[reduction.declaration].base_mode,
            )?
            .ok_or_else(|| "Successful PVT point has no terminal node values".to_owned())?;
            let (node_names, node_values) = std::iter::once((GROUND_NODE.to_owned(), 0.0))
                .chain(values)
                .unzip();
            Ok(SweepPointResult {
                node_names,
                node_values,
            })
        });
    }

    /// Whether this task's turn assembles a family rather than reaching the
    /// engine. True exactly for a declaration whose points were expanded, which
    /// is the one thing that makes its own solve redundant.
    pub(in crate::simulation) fn declares(&self, declaration: AnalysisInstanceId) -> bool {
        self.declarations
            .iter()
            .any(|candidate| candidate.authored_instance_id == declaration)
    }

    pub(in crate::simulation) fn clear(&mut self) {
        self.declarations.clear();
        self.reductions.clear();
    }

    /// The family a declaration's turn produces, read off the point results
    /// already retained in the run.
    ///
    /// `Err` where the collapsing executor would have failed the whole
    /// analysis: a declaration none of whose points converged is a failed
    /// result, not a missing one, because a plot reporting nothing is
    /// indistinguishable from a sweep nobody asked for.
    pub(in crate::simulation) fn family_for(
        &mut self,
        declaration: AnalysisInstanceId,
        run: &SimulationRun,
    ) -> Result<SimulationResult, String> {
        let Some(declaration) = self
            .declarations
            .iter()
            .find(|candidate| candidate.authored_instance_id == declaration)
        else {
            return Err("PVT declaration expanded into no points to assemble".to_owned());
        };
        declaration.family(run, &mut self.reductions)
    }
}

impl PointDeclaration {
    fn family(
        &self,
        run: &SimulationRun,
        reductions: &mut HashMap<AnalysisInstanceId, PointReduction>,
    ) -> Result<SimulationResult, String> {
        match &self.space {
            DeclaredSpace::Corner(points) => corner_family_of_points(
                &self.base_mode,
                self.declared_points,
                &solved_points(run, points, reductions)?,
            ),
            DeclaredSpace::Temperature(points) => temperature_family_of_points(
                &self.base_mode,
                self.declared_points,
                &solved_points(run, points, reductions)?,
            ),
        }
    }
}

/// The retained result of every point that produced one, in declaration order.
///
/// The join is on the point task's own instance identity, so a result belongs
/// to a point because the run says it answers for that task and never because
/// it happened to arrive in the right place.
fn solved_points<A: Copy>(
    run: &SimulationRun,
    points: &[(usize, A, AnalysisInstanceId)],
    reductions: &mut HashMap<AnalysisInstanceId, PointReduction>,
) -> Result<Vec<(A, SweepPointResult)>, String> {
    let mut points = points.to_vec();
    points.sort_by_key(|(index, _, _)| *index);
    let retained = run
        .analyses
        .iter()
        .filter_map(|analysis| {
            analysis
                .provenance
                .as_ref()
                .map(|p| (p.source_instance_id(), analysis.success))
        })
        .collect::<HashMap<_, _>>();
    let mut collected = Vec::with_capacity(points.len());
    let mut failure = None;
    for (index, point, instance) in points {
        let captured = reductions.remove(&instance).and_then(|point| point.result);
        if retained.get(&instance) != Some(&true) {
            continue;
        }
        match captured.unwrap_or_else(|| {
            Err("terminal reduction was not captured before output projection".to_owned())
        }) {
            Ok(values) => collected.push((point, values)),
            Err(error) => {
                failure.get_or_insert_with(|| format!("PVT point {}: {error}", index + 1));
            }
        }
    }
    failure.map_or(Ok(collected), Err)
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
    converged: &[(CornerPoint, SweepPointResult)],
) -> Result<SimulationResult, String> {
    if converged.is_empty() {
        return Err("Corner analysis produced no converged corner points".to_owned());
    }
    let (x_values, x_label, x_unit, temperatures_c, corner_labels, voltages) =
        map_corner_results(converged, base_mode.metric_label(), &NoAbort)
            .map_err(|error| error.to_string())?;

    Ok(SimulationResult::Corner {
        waveforms: waveforms_over(&x_values, voltages),
        x_values,
        x_label,
        x_unit,
        temperatures_c,
        corner_labels,
        num_failures: declared_points.saturating_sub(converged.len()),
        // Deliberately none. Every point of this family is its own retained
        // `AnalysisResult` carrying its own `.MEAS` evaluation and its own PVT
        // attribution, so a limit is already answered over all of them by the
        // ordinary worst-of join. Restating those measurements on the reduction
        // would enter each point into the same verdict twice and report a
        // yield over double the trials the run performed.
        member_measurements: Vec::new(),
    })
}

/// The temperature family a set of solved points adds up to.
///
/// The same reduction as a corner's, laid against the temperatures themselves
/// rather than against a derived corner axis: a temperature step varies one
/// quantity, so the axis is that quantity and needs no derivation.
pub(in crate::simulation) fn temperature_family_of_points(
    base_mode: &CornerBaseMode,
    declared_points: usize,
    converged: &[(Value, SweepPointResult)],
) -> Result<SimulationResult, String> {
    if converged.is_empty() {
        return Err("Parametric analysis produced no converged sweep points".to_owned());
    }
    let (sweep_values, voltages) =
        map_temperature_results(converged, base_mode.metric_label(), &NoAbort)
            .map_err(|error| error.to_string())?;

    Ok(SimulationResult::Parametric {
        target: TEMPERATURE_TARGET.to_owned(),
        waveforms: waveforms_over(&sweep_values, voltages),
        sweep_values,
        num_failures: declared_points.saturating_sub(converged.len()),
        // Deliberately none, for the same reason a corner family reports none:
        // each temperature point is already its own retained result.
        member_measurements: Vec::new(),
    })
}

fn waveforms_over(
    x_values: &[Value],
    voltages: Vec<(String, Vec<Value>)>,
) -> HashMap<String, WaveformData> {
    let mut waveforms = HashMap::with_capacity(voltages.len());
    for (name, values) in voltages {
        waveforms.insert(
            name.clone(),
            WaveformData::new_time_domain(name, x_values.to_vec(), values),
        );
    }
    waveforms
}

/// The scalar each node contributed at one point, named the way the deck names
/// the node, sorted so every point agrees on an order.
///
/// The reduction is the base analysis's own terminal value: the converged
/// operating point, the last swept point, the last transient sample, or the
/// magnitude at the terminal frequency.
fn point_node_values(
    analysis: &AnalysisResult,
    base_mode: &CornerBaseMode,
) -> Result<Option<Vec<(String, f64)>>, String> {
    let mut values = match base_mode {
        CornerBaseMode::Op => {
            let Some(values) = operating_point_node_values(analysis) else {
                return Ok(None);
            };
            values
        }
        CornerBaseMode::Ac { .. } => terminal_ac_magnitudes(analysis),
        CornerBaseMode::DcSweep { .. } | CornerBaseMode::DcSweepNested { .. } => {
            terminal_dc_node_samples(analysis, base_mode)?
        }
        CornerBaseMode::Transient { .. } | CornerBaseMode::TransientWindow { .. } => {
            terminal_node_samples(analysis)
        }
    };
    if values.is_empty() {
        return Ok(None);
    }
    values.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(Some(values))
}

fn terminal_dc_node_samples(
    analysis: &AnalysisResult,
    base_mode: &CornerBaseMode,
) -> Result<Vec<(String, f64)>, String> {
    use crate::state::{DcSweepFamily, DcSweepQuantity, DcTraceView};
    let Some(AnalysisResultPayload::DcSweep { evidence }) = &analysis.result_payload else {
        return Err("DC point is missing exact curve identity and traversal evidence; rerun the point before assembling its family".to_owned());
    };
    match (base_mode, &evidence.family) {
        (CornerBaseMode::DcSweep { source_name, .. }, DcSweepFamily::Single)
            if source_name.eq_ignore_ascii_case(&evidence.source) => {}
        (
            CornerBaseMode::DcSweepNested {
                source_name,
                source2,
                ..
            },
            DcSweepFamily::Nested { source, .. },
        ) if source_name.eq_ignore_ascii_case(&evidence.source)
            && source2.eq_ignore_ascii_case(source) => {}
        _ => return Err("DC point evidence does not match its declared sweep family".to_owned()),
    }
    evidence.validate_retained_traces(analysis.waveforms.iter().map(|trace| DcTraceView {
        name: &trace.name,
        unit: trace.unit.as_deref(),
        x: &trace.x,
        sample_count: trace.y.len(),
        complex: trace.complex.is_some(),
    }))?;
    let member = evidence.member_count() - 1;
    let by_name = analysis
        .waveforms
        .iter()
        .map(|trace| (trace.name.as_str(), trace))
        .collect::<HashMap<_, _>>();
    let mut values = Vec::new();
    for curve in evidence
        .curve_indices()
        .filter(|curve| curve.member == member)
    {
        let quantity = &evidence.quantities[curve.quantity];
        let DcSweepQuantity::NodeVoltage(node) = quantity else {
            continue;
        };
        let trace_name = evidence.trace_name(quantity, member);
        let trace = by_name[trace_name.as_str()];
        let index = evidence
            .terminal_sample(member, trace.y.len())
            .ok_or_else(|| "DC point has no terminal sample".to_owned())?;
        values.push((node.clone(), trace.y[index]));
    }
    Ok(values)
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
/// retained complex samples so it is the same quantity the collapsing executor
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

/// A family is one scalar per node, so a branch-current trace beside them is
/// not a node and must not become one.
fn is_branch_current(name: &str) -> bool {
    (name.starts_with("I(") || name.starts_with("i(")) && name.ends_with(')')
}

#[cfg(test)]
mod basis_tests;
#[cfg(test)]
mod corner_tests;
#[cfg(test)]
mod temperature_tests;
