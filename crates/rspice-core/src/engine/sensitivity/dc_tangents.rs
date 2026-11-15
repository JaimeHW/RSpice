//! The operating-point tangents a DC sensitivity row is read from, taken at
//! each converged point of a study.
//!
//! A row is the derivative of the output along `J(x*)^-1 d`, where `x*` is
//! the converged solution, `J(x*)` the Jacobian the operating point solved
//! and `d` the excitation the parameter moves: an independent source's own
//! row for its DC value, or for a device or model parameter the change of the
//! residual `r = b - J x`, assembled at the fixed `x*`, on the circuit rebuilt
//! with the parameter moved. Every row of a point, for every objective, shares
//! one assembly of `J(x*)`, as Xyce's direct sensitivities reuse the factors
//! of the converged point for each parameter and read every objective from
//! the same `dx/dp` (N_NLS_Sensitivity.C, `Sensitivity::solveDirect`).
//!
//! The rebuilt circuits belong to the deck, not to the point: nothing in them
//! depends on the solution, and an independent source's DC value reaches the
//! residual only through its own branch row, identically in the circuit as
//! built and in each rebuilt one, so it cancels from every difference. A
//! source sweep therefore builds them once for all its points, and each point
//! assembles its residuals on fresh copies, which see the circuits exactly as
//! their builds left them.

use super::{
    AcSensitivityLocation, AcSensitivityTarget, Engine, SimulationError, sensitivity_trial,
};
use crate::abort_signal::AbortSignal;
use crate::analysis::sensitivity::AcSensitivityOutput;
use crate::netlist::ElementKind;
use crate::solver::{SimulationResult, StaticMatrix};
use crate::{CircuitData, Netlist, Value};
use std::collections::HashMap;

/// Relative agreement two one-sided residual derivatives of a row need for
/// the row to be taken as differentiable at its nominal value without
/// looking further. A smooth model's sides differ by the step times the
/// curvature, `δ·|y''/y'|`: 1e-3 of the derivative per unit of the
/// parameter's own curvature scale `|p·y''/y'|`. Sides further apart than
/// this are a kink only if the gap does not halve with the step.
pub(super) const ONE_SIDED_AGREEMENT: Value = 0.1;

/// The output change over one step, relative to the nominal output, below
/// which one-sided responses are not resolvable. The residual is assembled
/// from linearized stamps whose companion currents round at `ε·|g·x|`, and a
/// high-gain loop carries that to the output at about 1e-10 of its value.
pub(super) const ONE_SIDED_RESOLUTION: Value = 1.0e-9;

/// The unknowns, summed over the rebuilt circuits of one deck, up to which a
/// sweep keeps those circuits for all the points the deck solves. A deck
/// whose circuits exceed it rebuilds them at each point, as a single
/// operating point does, so a study never holds more than this at once.
const RETAINED_PROBE_UNKNOWNS: usize = 1 << 20;

/// One side of a residual-tangent row at one step: the output's change along
/// `J(x*)^-1` times the residual difference over the step, not divided by it.
type ResidualSide = Option<Value>;

/// One device or model DC row's operating-point tangents, read on one output.
///
/// Each side is the output's change along `J(x*)^-1 (r(p + δ) - r(p))` or
/// `J(x*)^-1 (r(p) - r(p - δ))`, the residual `r = b - J x` assembled at the
/// FIXED converged solution `x*` for the circuit rebuilt at each parameter
/// value, at the step `δ` and at `δ/2`. `None` is a side whose circuit could
/// not be built or assembled at `x*` (a parameter domain, or a value that
/// changes the unknowns).
pub(super) struct ResidualTangent {
    step: Value,
    /// `[forward, backward]` at `δ`, then at `δ/2`.
    sides: [[ResidualSide; 2]; 2],
    /// The side's residual at `δ` equals the nominal residual bit for bit:
    /// the model ignores the parameter on that side.
    flat: [bool; 2],
}

impl ResidualTangent {
    /// The row's derivative and whether it is one-sided, or `None` when
    /// neither side resolved.
    ///
    /// Two sides that agree give the central difference, Richardson-
    /// extrapolated from `δ` and `δ/2` so the truncation error is fourth
    /// order. Sides that do not agree (one is flat and the other is not, or
    /// they differ beyond [`ONE_SIDED_AGREEMENT`] and the resolution floor by
    /// a gap that does not halve with the step) mark a model that is not
    /// differentiable at the nominal value, and the row is the forward side,
    /// as SPICE's forward perturbation reports it. A row with one resolvable
    /// side is that side. A one-sided value is extrapolated from both steps
    /// too, which leaves a second-order error.
    pub(super) fn derivative(&self, nominal_output: Value) -> Option<(Value, bool)> {
        let read = |side: ResidualSide, step: Value| side.map(|change| change / step);
        let [[forward, backward], [forward_half, backward_half]] = self.sides;
        let (forward, backward) = (read(forward, self.step), read(backward, self.step));
        let half = 0.5 * self.step;
        let (forward_half, backward_half) = (read(forward_half, half), read(backward_half, half));
        let one_sided = |coarse: Value, fine: Option<Value>| match fine {
            Some(fine) => 2.0 * fine - coarse,
            None => coarse,
        };
        match (forward, backward) {
            (Some(forward), Some(backward)) => {
                let floor = ONE_SIDED_RESOLUTION * (nominal_output.abs() + ONE_SIDED_RESOLUTION)
                    / self.step;
                // A smooth row's sides differ by the step times its curvature,
                // which halves with the step; a kink's differ by the change of
                // slope, which does not. A strongly curved smooth row (C5's
                // NMOS `VFB`, whose sides differ by a fifth at `δ`) is told
                // from a kink by the half step.
                let gap = forward - backward;
                let kink = self.flat[0] != self.flat[1]
                    || (gap.abs()
                        > ONE_SIDED_AGREEMENT * forward.abs().max(backward.abs()) + floor
                        && match (forward_half, backward_half) {
                            (Some(forward), Some(backward)) => {
                                (forward - backward).abs() > 0.75 * gap.abs()
                            }
                            _ => true,
                        });
                Some(if kink {
                    (one_sided(forward, forward_half), true)
                } else {
                    let coarse = 0.5 * (forward + backward);
                    let central = match (forward_half, backward_half) {
                        (Some(forward), Some(backward)) => {
                            (4.0 * (0.5 * (forward + backward)) - coarse) / 3.0
                        }
                        _ => coarse,
                    };
                    (central, false)
                })
            }
            (Some(forward), None) => Some((one_sided(forward, forward_half), true)),
            (None, Some(backward)) => Some((one_sided(backward, backward_half), true)),
            (None, None) => None,
        }
    }
}

/// The rows of one objective a converged point's tangents answer: each
/// source row's output derivative, and each residual row's sides, keyed by
/// row.
#[derive(Default)]
pub(super) struct ObjectiveTangents {
    pub(super) sources: HashMap<usize, Value>,
    pub(super) residuals: HashMap<usize, ResidualTangent>,
}

/// The unknowns of a converged circuit, which a rebuilt circuit must share
/// for its residual to be assembled at that circuit's solution.
#[derive(PartialEq, Eq)]
pub(super) struct UnknownLayout {
    size: usize,
    nodes: Vec<String>,
    branches: Vec<String>,
}

impl UnknownLayout {
    pub(super) fn of(circuit: &CircuitData) -> Self {
        Self {
            size: circuit.matrix_size(),
            nodes: circuit.node_names_sorted(),
            branches: circuit.branch_names_sorted(),
        }
    }
}

/// A circuit built and prepared from one deck, on which the operating-point
/// residual is assembled at a solution another circuit converged to.
struct ResidualProbe {
    /// As built and prepared. It is never assembled itself: every residual is
    /// assembled on a copy.
    circuit: CircuitData,
    matrix: StaticMatrix,
}

impl ResidualProbe {
    /// `deck` built and prepared afresh, or `None` when the build is
    /// recoverably refused (a parameter domain) or its unknowns are not the
    /// converged `layout`.
    fn build(
        engine: &Engine,
        deck: &Netlist,
        layout: &UnknownLayout,
        abort: &dyn AbortSignal,
    ) -> Result<Option<Self>, SimulationError> {
        let Ok((circuit, matrix)) =
            sensitivity_trial(engine.prepared_dc_probe_circuit(deck, abort))?
        else {
            return Ok(None);
        };
        Ok((UnknownLayout::of(&circuit) == *layout).then_some(Self { circuit, matrix }))
    }

    /// The operating-point residual at `solution`, or `None` when it cannot
    /// be assembled there.
    fn residual(
        &mut self,
        engine: &Engine,
        solution: &[Value],
    ) -> Result<Option<Vec<Value>>, SimulationError> {
        let mut circuit = self.circuit.clone();
        // This circuit never iterated to `solution`; a BJT must not answer
        // for it with the first Newton load it still owes.
        circuit.update_bjt_static_linearizations(solution);
        Ok(sensitivity_trial(engine.dc_operating_point_residual(
            &mut circuit,
            &mut self.matrix,
            solution,
        ))?
        .ok())
    }
}

impl Engine {
    /// The operating-point residual at `solution` of `netlist` built and
    /// prepared afresh, or `None` when the build is recoverably refused (a
    /// parameter domain) or its unknowns are not the converged `layout`.
    pub(super) fn dc_rebuilt_residual(
        engine: &Engine,
        netlist: &Netlist,
        layout: &UnknownLayout,
        solution: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Option<Vec<Value>>, SimulationError> {
        match ResidualProbe::build(engine, netlist, layout, abort)? {
            Some(mut probe) => probe.residual(engine, solution),
            None => Ok(None),
        }
    }
}

/// The circuits a deck's residual rows are assembled on: the deck as built,
/// and each row's deck rebuilt at `p ± δ` and `p ± δ/2`.
struct ResidualProbes {
    /// The deck as built; `None` when it is refused or its unknowns are not
    /// the converged ones, which leaves every residual row without a tangent.
    nominal: Option<ResidualProbe>,
    /// Per residual row, `[upper, lower]` at `δ`, then at `δ/2`.
    sides: Vec<[[Option<ResidualProbe>; 2]; 2]>,
}

impl ResidualProbes {
    fn build(
        engine: &Engine,
        flat: &Netlist,
        rows: &[(usize, &AcSensitivityTarget)],
        layout: &UnknownLayout,
        progress: &dyn Fn(usize) -> Result<(), SimulationError>,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        let Some(nominal) = ResidualProbe::build(engine, flat, layout, abort)? else {
            return Ok(Self {
                nominal: None,
                sides: Vec::new(),
            });
        };
        let mut sides = Vec::with_capacity(rows.len());
        for (count, &(_, target)) in rows.iter().enumerate() {
            progress(count)?;
            let step = Engine::residual_sensitivity_step(target.nominal_value);
            let side = |value: Value| -> Result<Option<ResidualProbe>, SimulationError> {
                if !value.is_finite() {
                    return Ok(None);
                }
                let mut perturbed = flat.clone();
                if sensitivity_trial(Engine::apply_ac_sensitivity_target(
                    &mut perturbed,
                    target,
                    value,
                ))?
                .is_err()
                {
                    return Ok(None);
                }
                ResidualProbe::build(engine, &perturbed, layout, abort)
            };
            let mut levels: [[Option<ResidualProbe>; 2]; 2] = Default::default();
            for (level, offset) in [step, 0.5 * step].into_iter().enumerate() {
                levels[level] = [
                    side(target.nominal_value + offset)?,
                    side(target.nominal_value - offset)?,
                ];
            }
            sides.push(levels);
        }
        Ok(Self {
            nominal: Some(nominal),
            sides,
        })
    }
}

/// What a DC sensitivity study differentiates at one deck: the rows any of
/// its objectives selects, each objective's selection, and the circuits the
/// residual rows are assembled on.
pub(super) struct DcSensitivityDeck {
    /// The flattened deck every row perturbs.
    pub(super) flat: Netlist,
    pub(super) rows: Vec<AcSensitivityTarget>,
    /// Per objective, its rows in result order, as indices into `rows`.
    pub(super) selections: Vec<Vec<usize>>,
}

/// The rebuilt circuits of the deck a study is solving: built at the deck's
/// first point, and kept for its later points when they fit
/// [`RETAINED_PROBE_UNKNOWNS`]. A new deck starts a new cache.
#[derive(Default)]
pub(super) struct ResidualProbeCache(Option<ResidualProbes>);

impl DcSensitivityDeck {
    pub(super) fn new(
        flat: Netlist,
        rows: Vec<AcSensitivityTarget>,
        selections: Vec<Vec<usize>>,
    ) -> Self {
        Self {
            flat,
            rows,
            selections,
        }
    }

    /// Whether some row is read from an operating-point tangent.
    pub(super) fn has_tangent_rows(&self) -> bool {
        self.rows.iter().any(|target| {
            !matches!(
                target.location,
                AcSensitivityLocation::DesignParameter { .. }
            )
        })
    }

    /// Every tangent row at one converged point, read on each objective's
    /// output: `circuit`, `matrix` and `solution` are the converged
    /// operating point, whose state is left as it was.
    ///
    /// An independent source's DC row is the exact tangent along its
    /// excitation (see [`Engine::dc_source_direction`]); every device-instance
    /// and model-parameter row is the tangent along its residual difference.
    /// A rebuilt nominal circuit whose unknowns differ from the converged
    /// ones yields no residual tangents, and those rows keep the re-solved
    /// finite difference.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn point_tangents(
        &self,
        cache: &mut ResidualProbeCache,
        engine: &Engine,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        outputs: &[AcSensitivityOutput],
        progress: &dyn Fn(usize) -> Result<(), SimulationError>,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<ObjectiveTangents>, SimulationError> {
        let mut directions = Vec::new();
        let mut sources = Vec::new();
        for (row, target) in self.rows.iter().enumerate() {
            if let AcSensitivityLocation::SourceDc { element_index } = target.location
                && let Some(direction) =
                    Engine::dc_source_direction(circuit, &self.flat.elements[element_index])
            {
                sources.push((row, directions.len()));
                directions.push(direction);
            }
        }

        let residual_rows = self
            .rows
            .iter()
            .enumerate()
            .filter(|(_, target)| {
                !matches!(
                    target.location,
                    AcSensitivityLocation::SourceDc { .. }
                        | AcSensitivityLocation::DesignParameter { .. }
                )
            })
            .collect::<Vec<_>>();
        let mut residuals = Vec::new();
        if !residual_rows.is_empty() {
            let mut probes = match cache.0.take() {
                Some(probes) => probes,
                None => {
                    let layout = UnknownLayout::of(circuit);
                    ResidualProbes::build(
                        engine,
                        &self.flat,
                        &residual_rows,
                        &layout,
                        progress,
                        abort,
                    )?
                }
            };
            let retain = probes
                .sides
                .len()
                .saturating_mul(4)
                .saturating_add(1)
                .saturating_mul(circuit.matrix_size())
                <= RETAINED_PROBE_UNKNOWNS;
            if let Some(nominal) = probes.nominal.as_mut()
                && let Some(nominal) = nominal.residual(engine, solution)?
            {
                let difference = |left: &[Value], right: &[Value]| {
                    left.iter()
                        .zip(right)
                        .map(|(left, right)| left - right)
                        .collect::<Vec<_>>()
                };
                for (&(row, target), levels) in residual_rows.iter().zip(probes.sides.iter_mut()) {
                    let step = Engine::residual_sensitivity_step(target.nominal_value);
                    let mut flat = [false; 2];
                    let mut sides = [[None, None], [None, None]];
                    for (level, [upper, lower]) in levels.iter_mut().enumerate() {
                        let upper = match upper {
                            Some(probe) => probe.residual(engine, solution)?,
                            None => None,
                        };
                        let lower = match lower {
                            Some(probe) => probe.residual(engine, solution)?,
                            None => None,
                        };
                        if level == 0 {
                            flat = [
                                upper.as_deref() == Some(nominal.as_slice()),
                                lower.as_deref() == Some(nominal.as_slice()),
                            ];
                        }
                        for (slot, change) in sides[level].iter_mut().zip([
                            upper.map(|upper| difference(&upper, &nominal)),
                            lower.map(|lower| difference(&nominal, &lower)),
                        ]) {
                            *slot = change.map(|change| {
                                directions.push(change);
                                directions.len() - 1
                            });
                        }
                    }
                    residuals.push((row, step, sides, flat));
                }
            }
            if retain {
                cache.0 = Some(probes);
            }
        }

        let tangents = if directions.is_empty() {
            Vec::new()
        } else {
            engine.dc_operating_point_tangents(circuit, matrix, solution, &directions)?
        };
        // The tangents are in the solution's layout, which the published
        // result shares: node voltages, then public branch currents
        // (`populate_public_dc_solution`).
        let mut layout = SimulationResult::new(circuit.num_nodes(), circuit.num_branches());
        layout.branch_names = circuit.branch_names_sorted();
        outputs
            .iter()
            .map(|output| {
                let read = |index: usize| {
                    Engine::dc_tangent_output_value(&layout, &tangents[index], output)
                };
                let sources = sources
                    .iter()
                    .map(|&(row, index)| Ok((row, read(index)?)))
                    .collect::<Result<_, SimulationError>>()?;
                let residuals = residuals
                    .iter()
                    .map(|&(row, step, sides, flat)| {
                        let side = |index: Option<usize>| index.map(read).transpose();
                        let [[forward, backward], [forward_half, backward_half]] = sides;
                        Ok((
                            row,
                            ResidualTangent {
                                step,
                                sides: [
                                    [side(forward)?, side(backward)?],
                                    [side(forward_half)?, side(backward_half)?],
                                ],
                                flat,
                            },
                        ))
                    })
                    .collect::<Result<_, SimulationError>>()?;
                Ok(ObjectiveTangents { sources, residuals })
            })
            .collect()
    }
}

/// A rebuilt sweep point: the engine that solves it when it is not the
/// study's own, the point's deck and its flattened copy.
type SweptPointDeck = (Option<Engine>, Netlist, Netlist);

/// The deck one point of a study solved, for the rows that re-solve it.
pub(super) enum DcSensitivityPointDeck<'a> {
    /// The deck the study itself solved, with its flattened copy.
    Solved {
        engine: &'a Engine,
        netlist: &'a Netlist,
        flat: &'a Netlist,
    },
    /// A sweep point, rebuilt on first use: most points re-solve nothing.
    Swept {
        base: &'a Engine,
        build: &'a dyn Fn(&dyn AbortSignal) -> Result<SweptPointDeck, SimulationError>,
        built: Option<Box<SweptPointDeck>>,
    },
}

impl DcSensitivityPointDeck<'_> {
    /// The engine that solves the point, its deck and the flattened deck.
    pub(super) fn get(
        &mut self,
        abort: &dyn AbortSignal,
    ) -> Result<(&Engine, &Netlist, &Netlist), SimulationError> {
        match self {
            Self::Solved {
                engine,
                netlist,
                flat,
            } => Ok((*engine, *netlist, *flat)),
            Self::Swept { base, build, built } => {
                if built.is_none() {
                    *built = Some(Box::new(build(abort)?));
                }
                let (engine, netlist, flat) = &**built.as_ref().expect("the point was just built");
                Ok((engine.as_ref().unwrap_or(base), netlist, flat))
            }
        }
    }
}

impl Engine {
    /// The excitation direction of an independent source's DC row in
    /// `circuit`, or `None` for a source the elaborated circuit does not hold
    /// under its own name, which keeps the finite-difference row.
    ///
    /// The row's variable is the value the operating point gives the source:
    /// its DC value, or for a waveform without one the `t = 0` value `.op`
    /// takes (PULSE `V1`, SIN `VO + VA sin(PHASE)`, the first PWL point, zero
    /// for SFFM, AM and TRNOISE, and so on). Whatever the waveform kind, that
    /// value enters the DC equations only as the source's excitation: the
    /// branch row of a voltage source, and the two node rows of a current
    /// source (`I` leaves `n+` and enters `n-`). Its tangent is therefore one
    /// solve with the converged Jacobian along that excitation, which holds
    /// any mixed host's discrete state as the operating point left it.
    ///
    /// A finite difference of re-solved operating points cannot stand in for
    /// it: each re-solve carries the Newton iteration's convergence error, and
    /// on a high-gain loop (an LDO's output against its load current) the
    /// response to a resolvable step is buried under it, so the refinement
    /// never reaches agreement.
    fn dc_source_direction(
        circuit: &CircuitData,
        element: &crate::netlist::Element,
    ) -> Option<Vec<Value>> {
        let mut direction = vec![0.0; circuit.matrix_size()];
        match element.kind {
            ElementKind::VoltageSource(_) => {
                let index = circuit
                    .voltage_sources
                    .names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case(&element.name))?;
                let branch =
                    circuit.get_branch_matrix_index(circuit.voltage_sources.branch_indices[index]);
                direction[branch - 1] = 1.0;
            }
            ElementKind::CurrentSource(_) => {
                let index = circuit.current_sources.index_by_name(&element.name)?;
                let positive = circuit.current_sources.node_pos[index];
                let negative = circuit.current_sources.node_neg[index];
                if positive != negative {
                    if let Some(row) = positive.checked_sub(1) {
                        direction[row] -= 1.0;
                    }
                    if let Some(row) = negative.checked_sub(1) {
                        direction[row] += 1.0;
                    }
                }
            }
            _ => return None,
        }
        Some(direction)
    }
}
