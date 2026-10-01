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

use rspice_core::Value;

use crate::execution::AuthorizedTaskDispatch;
use crate::results::SimulationResult;
use crate::sweeps::{
    CornerBaseMode, CornerPoint, SweepPointResult, corner_family_of_points, point_node_values,
    temperature_family_of_points,
};
use rspice_app_types::product::AnalysisInstanceId;
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::run::SimulationRun;
use rspice_results::waveform::RetainedWaveform;

/// Ground, which the sweep mappers require at index 0 and never plot.
const GROUND_NODE: &str = "0";

/// Where one point sits in the space its declaration declared.
///
/// A corner point states the whole PVT triple because its family's axis, its
/// labels and its own temperature column are all read back off it. A
/// temperature point states only the temperature, because that is the entire
/// condition a temperature step varies; naming a supply it never scaled would
/// be attributing the result to a corner it did not solve.
#[derive(Debug, Clone, Copy)]
pub(crate) enum DeclaredAxisPoint {
    Corner(CornerPoint),
    Temperature(Value),
}

/// What a point's task cannot state about the declaration it belongs to.
///
/// Frozen onto every point task during preparation, because the declaration's
/// own task never reaches an executor and there is nothing else left to ask for
/// its base analysis or how many points it declared.
#[derive(Debug, Clone)]
pub(crate) struct DeclaredRunPoint {
    base_mode: CornerBaseMode,
    declared_points: usize,
    /// Position in the declared space. The family's axis, labels and per-node
    /// samples are all positional, so pairing a result with its point is done
    /// on this index and never on the order results happen to arrive in.
    index: usize,
    point: DeclaredAxisPoint,
}

impl DeclaredRunPoint {
    pub(crate) fn new(
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
pub struct PointFamilyRegistry {
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
    pub fn register(&mut self, task: &AuthorizedTaskDispatch) {
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
    pub fn capture_result<W: AsRef<RetainedWaveform>>(
        &mut self,
        instance: AnalysisInstanceId,
        analysis: &AnalysisResult<W>,
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
    pub fn declares(&self, declaration: AnalysisInstanceId) -> bool {
        self.declarations
            .iter()
            .any(|candidate| candidate.authored_instance_id == declaration)
    }

    pub fn clear(&mut self) {
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
    pub fn family_for<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
        &mut self,
        declaration: AnalysisInstanceId,
        run: &SimulationRun<A>,
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
    fn family<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
        &self,
        run: &SimulationRun<A>,
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
fn solved_points<P: Copy, A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
    run: &SimulationRun<A>,
    points: &[(usize, P, AnalysisInstanceId)],
    reductions: &mut HashMap<AnalysisInstanceId, PointReduction>,
) -> Result<Vec<(P, SweepPointResult)>, String> {
    let mut points = points.to_vec();
    points.sort_by_key(|(index, _, _)| *index);
    let retained = run
        .analyses
        .iter()
        .filter_map(|analysis| {
            let analysis = analysis.as_ref();
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

#[cfg(test)]
mod basis_tests;
#[cfg(test)]
mod corner_tests;
#[cfg(test)]
mod temperature_tests;
