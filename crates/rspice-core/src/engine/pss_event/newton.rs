//! The period map over images and the Newton iteration on it.
//!
//! A traversal writes `z` into the boundary image, runs the transient engine
//! over the locked mesh for one period and reads `z` back at the end. The
//! inner Newton iteration solves `Π(z) − z = 0` with the discrete state held,
//! from `Φ = ∂Π/∂z`: the variational recurrence of the accepted steps
//! ([`super::variational`]) where it is exact, finite-difference columns
//! elsewhere. Every trial keeps the
//! orbit's event signature (the ordered list of the discrete changes a
//! period makes), and a step that changes it is halved, up to eight times.

use super::Refusal;
use super::basis::{DiscreteFingerprint, ShootingBasis};
use super::boundary::{self, ObservedEvent};
use crate::abort_signal::AbortSignal;
use crate::analysis::PssConfig;
use crate::engine::transient::state_capture::{
    AcceptedPointObserver, AcceptedStep, CapturedTransientState,
};
use crate::engine::{Engine, SimulationError, TransientResult};
use crate::{CircuitData, Netlist, Value};

/// The finite-difference step, relative to each coordinate's scale: well
/// above what a traversal's Newton tolerance leaves in its end state, and
/// small enough that the map's curvature stays below 1e-4 of a column.
pub(super) const FD_STEP: Value = 1.0e-4;
/// How many times a Newton step that changes the event signature is halved.
const SIGNATURE_HALVINGS: usize = 8;
/// How many Newton steps in a row may be cut by the signature before the
/// orbit's event structure is declared unreachable from the boundary.
const SIGNATURE_CUT_STREAK: usize = 4;

/// What one period from an image produced.
#[derive(Clone, Debug)]
pub(super) struct Traversal {
    pub(super) end: CapturedTransientState,
    pub(super) z_end: Vec<Value>,
    pub(super) result: TransientResult,
    pub(super) events: Vec<ObservedEvent>,
    /// The cross detectors' expression values at every accepted point.
    pub(super) crossings: Vec<(Value, Vec<Value>)>,
}

/// A converged inner iteration.
#[derive(Clone, Debug)]
pub(super) struct Solved {
    pub(super) z: Vec<Value>,
    pub(super) traversal: Traversal,
    /// `Φ`, row-major, at `z`.
    pub(super) jacobian: Vec<Vec<Value>>,
    pub(super) iterations: usize,
    /// The largest residual in units of its coordinate's tolerance.
    pub(super) residual: Value,
}

/// Everything a traversal needs that does not change between them.
pub(super) struct Orbit<'a> {
    pub(super) engine: &'a Engine,
    pub(super) netlist: &'a Netlist,
    pub(super) basis: &'a ShootingBasis,
    pub(super) config: &'a PssConfig,
    pub(super) max_step: Value,
    pub(super) guard: Value,
    /// The relative and absolute tolerances of a node voltage an orbit's
    /// mesh is certified against, by the rule native shooting reads
    /// ([`crate::engine::Engine::periodic_refinement_tolerances`]).
    pub(super) mesh_tolerance: (Value, Value),
    pub(super) abort: &'a dyn AbortSignal,
}

/// The ordered discrete changes of one period.
pub(super) fn signature(events: &[ObservedEvent]) -> Vec<(usize, u64)> {
    events
        .iter()
        .map(|event| (event.quantity, event.value))
        .collect()
}

impl Orbit<'_> {
    /// One period of the map from `image` with its coordinates set to `z`.
    pub(super) fn traverse(
        &self,
        image: &CapturedTransientState,
        z: &[Value],
        mesh: &[Value],
    ) -> Result<Traversal, SimulationError> {
        if self.abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let start = self.basis.write(image, z)?;
        let probes = std::cell::RefCell::new(Vec::with_capacity(mesh.len() + 8));
        let observer = |time: Value,
                        circuit: &CircuitData,
                        _solution: &[Value],
                        _step: Option<AcceptedStep>| {
            probes
                .borrow_mut()
                .push(super::basis::probe(time, circuit)?);
            Ok(())
        };
        let observer: &AcceptedPointObserver<'_> = &observer;
        let (result, end) = self.engine.run_tran_segment_from_image(
            self.netlist,
            &start,
            (mesh, None),
            self.max_step,
            Some(observer),
            self.abort,
        )?;
        let z_end = self.basis.read(&end)?;
        let probes = probes.into_inner();
        let events = boundary::events(self.basis, &probes);
        let crossings = probes
            .iter()
            .map(|probe| {
                (
                    probe.time,
                    probe.crossings.iter().map(|&(value, _)| value).collect(),
                )
            })
            .collect();
        Ok(Traversal {
            end,
            z_end,
            result,
            events,
            crossings,
        })
    }

    /// Each coordinate's periodicity tolerance: relative to its value, or
    /// the configured absolute floor.
    fn tolerance(&self, value: Value, end: Value) -> Value {
        (self.config.tolerance * value.abs().max(end.abs())).max(self.config.abstol)
    }

    /// The largest residual in units of its coordinate's tolerance; one or
    /// less is periodic.
    pub(super) fn scaled_residual(&self, z: &[Value], z_end: &[Value]) -> Value {
        z.iter()
            .zip(z_end)
            .map(|(start, end)| (end - start).abs() / self.tolerance(*start, *end))
            .fold(0.0, Value::max)
    }

    /// `Φ` at `z`: the variational recurrence of the accepted steps where it
    /// is the map's exact derivative, and forward differences elsewhere.
    pub(super) fn jacobian(
        &self,
        image: &CapturedTransientState,
        z: &[Value],
        base: &Traversal,
        mesh: &[Value],
    ) -> Result<Vec<Vec<Value>>, SimulationError> {
        match self.variational_jacobian(image, z, mesh)? {
            Some(phi) => Ok(phi),
            None => self.finite_difference_jacobian(image, z, base, mesh),
        }
    }

    /// `Φ` at `z` by forward differences, each column taken on a trial that
    /// keeps `base`'s event signature.
    pub(super) fn finite_difference_jacobian(
        &self,
        image: &CapturedTransientState,
        z: &[Value],
        base: &Traversal,
        mesh: &[Value],
    ) -> Result<Vec<Vec<Value>>, SimulationError> {
        let n = z.len();
        let wanted = signature(&base.events);
        let mut phi = vec![vec![0.0; n]; n];
        for column in 0..n {
            let mut step = FD_STEP * self.basis.unit(column).scale(z[column]);
            let mut taken = None;
            'attempts: for _ in 0..4 {
                for sign in [1.0, -1.0] {
                    let mut trial = z.to_vec();
                    trial[column] += sign * step;
                    let traversal = self.traverse(image, &trial, mesh)?;
                    if signature(&traversal.events) == wanted {
                        taken = Some((traversal.z_end, sign * step));
                        break 'attempts;
                    }
                }
                step *= 0.1;
            }
            let Some((end, delta)) = taken else {
                return Err(Refusal::grazing_under_perturbation(self.basis.name(column)));
            };
            for (row, value) in phi.iter_mut().enumerate() {
                value[column] = (end[row] - base.z_end[row]) / delta;
            }
        }
        Ok(phi)
    }

    /// The Newton correction `Δ` with `(Φ − I) Δ = −r`.
    ///
    /// A coordinate the map leaves untouched (its row of `Φ` is the unit row)
    /// is periodic at any value, so it is held and its equation dropped.
    fn newton_step(phi: &[Vec<Value>], residual: &[Value]) -> Result<Vec<Value>, SimulationError> {
        let n = residual.len();
        let active: Vec<usize> = (0..n)
            .filter(|&row| {
                !phi[row].iter().enumerate().all(|(column, value)| {
                    let unit = if row == column { 1.0 } else { 0.0 };
                    (value - unit).abs() <= 1.0e-12
                })
            })
            .collect();
        let mut delta = vec![0.0; n];
        if active.is_empty() {
            return Ok(delta);
        }
        let matrix: Vec<Vec<Value>> = active
            .iter()
            .map(|&row| {
                active
                    .iter()
                    .map(|&column| phi[row][column] - if row == column { 1.0 } else { 0.0 })
                    .collect()
            })
            .collect();
        let rhs: Vec<Value> = active.iter().map(|&row| -residual[row]).collect();
        let lu = crate::engine::pss::PssDenseLu::factor(&matrix).ok_or_else(|| {
            SimulationError::Circuit(
                "PSS event shooting: the Newton system Φ − I is singular at the boundary; the \
                 orbit has a neutral direction no event fixes"
                    .to_string(),
            )
        })?;
        for (value, &index) in lu.solve(&rhs).into_iter().zip(&active) {
            delta[index] = value;
        }
        Ok(delta)
    }

    /// The inner iteration from `image` with its discrete state held.
    pub(super) fn solve(
        &self,
        image: &CapturedTransientState,
        mesh: &[Value],
        reuse: Option<Vec<Vec<Value>>>,
    ) -> Result<Solved, SimulationError> {
        let mut z = self.basis.read(image)?;
        let mut traversal = self.traverse(image, &z, mesh)?;
        let mut phi = reuse;
        let mut fresh = false;
        let mut iterations = 0;
        let mut cut_streak = 0;
        loop {
            let residual: Vec<Value> = traversal
                .z_end
                .iter()
                .zip(&z)
                .map(|(end, start)| end - start)
                .collect();
            let scaled = self.scaled_residual(&z, &traversal.z_end);
            if scaled <= 1.0 {
                let jacobian = match phi {
                    Some(phi) if fresh => phi,
                    _ => self.jacobian(image, &z, &traversal, mesh)?,
                };
                return Ok(Solved {
                    z,
                    traversal,
                    jacobian,
                    iterations,
                    residual: scaled,
                });
            }
            if iterations >= self.config.max_iterations {
                return Err(SimulationError::Circuit(format!(
                    "PSS event shooting did not close the period within {} Newton corrections; \
                     the largest residual is {scaled:.3e} times its tolerance",
                    self.config.max_iterations
                )));
            }
            if phi.is_none() {
                phi = Some(self.jacobian(image, &z, &traversal, mesh)?);
                fresh = true;
            }
            let delta = Self::newton_step(phi.as_ref().expect("jacobian"), &residual)?;
            let wanted = signature(&traversal.events);
            let mut lambda = 1.0;
            let mut accepted = None;
            let mut changed = None;
            for _ in 0..=SIGNATURE_HALVINGS {
                let trial: Vec<Value> = z
                    .iter()
                    .zip(&delta)
                    .map(|(value, step)| value + lambda * step)
                    .collect();
                let candidate = self.traverse(image, &trial, mesh)?;
                let kept = signature(&candidate.events) == wanted;
                if kept && self.scaled_residual(&trial, &candidate.z_end) < scaled {
                    accepted = Some((trial, candidate));
                    break;
                }
                if !kept && changed.is_none() {
                    changed = Some(candidate.events.clone());
                }
                lambda *= 0.5;
            }
            iterations += 1;
            match accepted {
                Some((trial, candidate)) => {
                    let contraction = self.scaled_residual(&trial, &candidate.z_end) / scaled;
                    cut_streak = if changed.is_some() { cut_streak + 1 } else { 0 };
                    if cut_streak >= SIGNATURE_CUT_STREAK {
                        return Err(self
                            .refuse_unstable(&traversal, changed.as_deref().unwrap_or_default()));
                    }
                    // A slow contraction asks for a fresh Jacobian; a fast
                    // one keeps it for the next correction.
                    if contraction > 0.1 {
                        phi = None;
                    }
                    fresh = false;
                    z = trial;
                    traversal = candidate;
                }
                None if !fresh => phi = None,
                None => {
                    return Err(
                        self.refuse_unstable(&traversal, changed.as_deref().unwrap_or_default())
                    );
                }
            }
        }
    }

    /// The refusal for an iteration that cannot keep the held orbit's events.
    /// A held orbit that grazes a threshold, or holds two events whose order
    /// is not physical, is refused by that name: the event structure moves
    /// because of it.
    pub(super) fn refuse_unstable(
        &self,
        held: &Traversal,
        changed: &[ObservedEvent],
    ) -> SimulationError {
        if let Err(specific) = self.certify_margins(held) {
            return specific;
        }
        let difference = held
            .events
            .iter()
            .zip(changed)
            .find(|(a, b)| (a.quantity, a.value) != (b.quantity, b.value))
            .map(|(a, _)| a)
            .or_else(|| {
                if held.events.len() > changed.len() {
                    held.events.get(changed.len())
                } else {
                    changed.get(held.events.len())
                }
            });
        let named = difference.map_or_else(
            || "an event".to_string(),
            |event| {
                format!(
                    "{} at t={:.6e} s",
                    self.basis.discrete_name(event.quantity),
                    event.time
                )
            },
        );
        Refusal::unstable(&named, held.events.len(), changed.len())
    }

    /// The discrete state at an image.
    pub(super) fn discrete_state(
        &self,
        image: &CapturedTransientState,
    ) -> Result<DiscreteFingerprint, SimulationError> {
        let probe = super::basis::probe(image.time, &image.circuit)?;
        Ok(self.basis.fingerprint(&probe))
    }
}
