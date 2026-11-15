//! Operating-point tangents with respect to several design parameters, all
//! taken at one solved operating point.
//!
//! A search over design parameters needs every parameter's derivative at the
//! same candidate. Re-solving the operating point with each parameter moved
//! costs two Newton solves per parameter and carries their convergence error.
//! Here the operating point is solved once; each parameter's excitation
//! `dr/dp` is assembled at the converged solution `x*`, and its tangent
//! `J(x*)^-1 dr/dp` is one more solve with the converged Jacobian
//! ([`Engine::dc_operating_point_tangents`]), as a `.SENS` source row is.
//!
//! The excitation is exact on a deck every element of which is a qualified
//! linear element: the parser's captured expression derivatives contracted
//! with the owning stamps, as the `.SENS` design-parameter rows take them.
//! Elsewhere it is the residual difference the `.SENS` device and model rows
//! use: `r = b - J x` assembled at the fixed `x*` for the deck replayed with
//! the parameter at `p ± h`. The replay moves everything defined from the
//! parameter (elements, model cards, options), so the excitation is the
//! parameter's total effect, and no Newton iteration enters it.
//!
//! A replay re-elaborates the deck, which on a small deck costs as much as a
//! Newton solve, so a search takes one central level where `.SENS` takes a
//! Richardson pair: `h` is a tenth of the `.SENS` step `δ`, which keeps the
//! truncation error near `1e-8` of the slope, the size of the error of the
//! finite differences it replaces, with half the replays.

use super::parameter::add_parameter_forcing;
use super::*;
use crate::netlist::expr::ComplexDirection;

/// How a design parameter's operating-point tangent was taken.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesignTangentMethod {
    /// Captured expression derivatives on a deck of qualified linear
    /// elements: exact.
    Exact,
    /// A central difference of the operating-point residual at the fixed
    /// converged solution.
    ResidualDifference,
}

/// The residual step of a search as a fraction of the `.SENS` step.
const SEARCH_STEP_FRACTION: Value = 0.1;

/// A direction of the operating point, in the layout the operating point's
/// [`SimulationResult`] publishes.
#[derive(Debug, Clone, PartialEq)]
pub struct OperatingPointDirection {
    /// Indexed like [`SimulationResult::node_voltages`]; ground is 0.
    pub node_voltages: Vec<Value>,
    /// Indexed like [`SimulationResult::branch_currents`].
    pub branch_currents: Vec<Value>,
}

#[derive(Debug, Clone)]
enum TangentShape {
    Exact(OperatingPointDirection),
    /// `J(x*)^-1 (r(p + h) - r(p))` and `J(x*)^-1 (r(p) - r(p - h))`, not
    /// divided by `h`. A side is `None` when its deck could not be
    /// assembled at `x*`; `flat` marks a side whose residual equals the
    /// nominal one bit for bit.
    Residual {
        step: Value,
        sides: [Option<OperatingPointDirection>; 2],
        flat: [bool; 2],
    },
}

/// One design parameter's operating-point tangent.
#[derive(Debug, Clone)]
pub struct DesignParameterTangent {
    shape: TangentShape,
}

impl DesignParameterTangent {
    pub fn method(&self) -> DesignTangentMethod {
        match self.shape {
            TangentShape::Exact(_) => DesignTangentMethod::Exact,
            TangentShape::Residual { .. } => DesignTangentMethod::ResidualDifference,
        }
    }

    /// The derivative of one observable of the operating point, which must
    /// be linear in the direction it reads (a node-voltage difference, a
    /// branch current); `nominal` is its value at the operating point.
    ///
    /// A residual row whose two sides disagree is refused rather than
    /// averaged, with the `.SENS` residual rows' agreement test: the sides
    /// of a smooth row differ by the step times its curvature, a kink's by
    /// the change of slope. A search needs the derivative itself, not the
    /// one-sided value `.SENS` reports at a kink, so a kink is refused, and
    /// so is a row with only one side.
    pub fn derivative(
        &self,
        nominal: Value,
        observe: impl Fn(&OperatingPointDirection) -> Option<Value>,
    ) -> Result<Value, &'static str> {
        let unobservable = "the observable is not part of the operating point";
        let value = match &self.shape {
            TangentShape::Exact(direction) => observe(direction).ok_or(unobservable)?,
            TangentShape::Residual {
                step,
                sides: [Some(forward), Some(backward)],
                flat,
            } => {
                let forward = observe(forward).ok_or(unobservable)? / step;
                let backward = observe(backward).ok_or(unobservable)? / step;
                let floor = ONE_SIDED_RESOLUTION * (nominal.abs() + ONE_SIDED_RESOLUTION) / step;
                if flat[0] != flat[1]
                    || (forward - backward).abs()
                        > ONE_SIDED_AGREEMENT * forward.abs().max(backward.abs()) + floor
                {
                    return Err(
                        "the residual differences on either side disagree: the deck is not differentiable in the parameter here",
                    );
                }
                0.5 * (forward + backward)
            }
            TangentShape::Residual { .. } => {
                return Err("the deck cannot be assembled at the operating point on one side");
            }
        };
        if value.is_finite() {
            Ok(value)
        } else {
            Err("the derivative is not finite")
        }
    }
}

/// An operating point and, per requested design parameter, its tangent or
/// the reason no tangent could be taken at the solved point.
#[derive(Debug, Clone)]
pub struct DesignParameterTangents {
    pub operating_point: SimulationResult,
    /// One entry per requested parameter, in order; or, as `Err`, the reason
    /// this deck gives no parameter a tangent (it retains no source to
    /// replay, or its replay does not reproduce the solved circuit), which
    /// holds for every candidate of the same deck and environment.
    pub tangents: Result<Vec<Result<DesignParameterTangent, String>>, String>,
}

/// A tangent in the solver's complete layout, before publication.
enum SolvedTangent {
    Exact(Vec<Value>),
    Residual {
        step: Value,
        sides: [Option<Vec<Value>>; 2],
        flat: [bool; 2],
    },
}

/// Where a parameter's excitations sit in the batch solved together.
enum PendingTangent {
    Exact(usize),
    Residual {
        step: Value,
        sides: [Option<usize>; 2],
        flat: [bool; 2],
    },
}

/// Every parameter's tangent, or why the deck gives none.
type SolvedTangents = Result<Vec<Result<SolvedTangent, String>>, String>;

impl Engine {
    /// Solve `netlist`'s operating point once and take its tangent with
    /// respect to each design parameter (a root `.param` name and its value
    /// in this deck) without re-solving.
    ///
    /// A parameter whose tangent cannot be taken at the solved point carries
    /// the reason instead, never a zero: a parameter move the operating
    /// point's unknowns cannot be assembled at, for one; and a deck that does
    /// not replay from source, or whose replay does not reproduce the solved
    /// circuit (an edit made to the netlist after it was elaborated), gives
    /// no parameter a tangent.
    pub fn run_dc_op_with_design_tangents_with_abort(
        &self,
        netlist: &Netlist,
        parameters: &[(String, Value)],
        abort: &dyn AbortSignal,
    ) -> Result<DesignParameterTangents, SimulationError> {
        let limits = self.config.resource_limits;
        let mut solved: SolvedTangents = Err("the operating point has no unknowns".to_owned());
        let operating_point = self.run_dc_op_with_linearization_and_abort(
            netlist,
            &mut |engine, circuit, matrix, solution| {
                solved = Self::design_tangents_at(
                    engine, limits, netlist, parameters, circuit, matrix, solution, abort,
                )?;
                Ok(())
            },
            abort,
        )?;
        let node_count = operating_point.node_voltages.len().saturating_sub(1);
        let branch_count = operating_point.branch_currents.len();
        let publish = |tangent: Vec<Value>| -> Result<OperatingPointDirection, String> {
            if tangent.len() < node_count + branch_count {
                return Err(format!(
                    "the tangent has {} value(s), fewer than the {} public unknown(s)",
                    tangent.len(),
                    node_count + branch_count
                ));
            }
            let mut node_voltages = vec![0.0; node_count + 1];
            node_voltages[1..].copy_from_slice(&tangent[..node_count]);
            Ok(OperatingPointDirection {
                node_voltages,
                branch_currents: tangent[node_count..node_count + branch_count].to_vec(),
            })
        };
        let publish_side = |side: Option<Vec<Value>>| side.map(publish).transpose();
        let tangents = solved.map(|solved| {
            solved
                .into_iter()
                .map(|tangent| {
                    let shape = match tangent? {
                        SolvedTangent::Exact(tangent) => TangentShape::Exact(publish(tangent)?),
                        SolvedTangent::Residual {
                            step,
                            sides: [forward, backward],
                            flat,
                        } => TangentShape::Residual {
                            step,
                            sides: [publish_side(forward)?, publish_side(backward)?],
                            flat,
                        },
                    };
                    Ok(DesignParameterTangent { shape })
                })
                .collect()
        });
        Ok(DesignParameterTangents {
            operating_point,
            tangents,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn design_tangents_at(
        engine: &Engine,
        limits: crate::resource::ResourceLimits,
        netlist: &Netlist,
        parameters: &[(String, Value)],
        circuit: &mut CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        solution: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<SolvedTangents, SimulationError> {
        let refuse_all = |reason: &str| -> Result<SolvedTangents, SimulationError> {
            Ok(Err(reason.to_owned()))
        };
        // A deck without its source cannot replay a parameter: moving one
        // would leave every element it defines at its elaborated value and
        // report a zero.
        if netlist.source_text.is_none() {
            return refuse_all("the deck retains no source to replay with a parameter moved");
        }
        let layout = (
            circuit.matrix_size(),
            circuit.node_names_sorted(),
            circuit.branch_names_sorted(),
        );
        let Some(nominal) = Self::dc_rebuilt_residual(engine, netlist, &layout, solution, abort)?
        else {
            return refuse_all("the solved circuit cannot be reassembled at its operating point");
        };
        // Every excitation below replays the deck from source. A netlist
        // edited after elaboration (a supply corner, a temperature written
        // into its options) replays without the edit, so the replay with no
        // parameter moved must assemble to the very residual the solved
        // circuit does before any difference is taken from replays.
        match sensitivity_trial(Self::replay_parameter_overrides(
            netlist,
            &[],
            None,
            limits,
            abort,
        ))? {
            Ok((replayed, _))
                if Self::dc_rebuilt_residual(engine, &replayed, &layout, solution, abort)?
                    .as_deref()
                    == Some(nominal.as_slice()) => {}
            _ => return refuse_all("replaying the deck does not reproduce the solved circuit"),
        }

        let mut excitations = Vec::new();
        let mut pending = Vec::with_capacity(parameters.len());
        for (name, value) in parameters {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if let Ok(excitation) = Self::exact_design_excitation(
                engine, limits, netlist, name, *value, &layout, solution, abort,
            )? {
                excitations.push(excitation);
                pending.push(Ok(PendingTangent::Exact(excitations.len() - 1)));
                continue;
            }
            let step = SEARCH_STEP_FRACTION * Self::residual_sensitivity_step(*value);
            let side = |candidate: Value| -> Result<Option<Vec<Value>>, SimulationError> {
                if !candidate.is_finite() || candidate == *value {
                    return Ok(None);
                }
                match sensitivity_trial(Self::replay_parameter_overrides(
                    netlist,
                    &[(name.to_ascii_uppercase(), candidate)],
                    None,
                    limits,
                    abort,
                ))? {
                    Ok((perturbed, _)) => {
                        Self::dc_rebuilt_residual(engine, &perturbed, &layout, solution, abort)
                    }
                    Err(_) => Ok(None),
                }
            };
            let difference = |left: &[Value], right: &[Value]| {
                left.iter()
                    .zip(right)
                    .map(|(left, right)| left - right)
                    .collect::<Vec<_>>()
            };
            let (upper, lower) = (side(*value + step)?, side(*value - step)?);
            if upper.is_none() && lower.is_none() {
                pending.push(Err(
                    "the deck cannot be assembled at the operating point with the parameter moved"
                        .to_owned(),
                ));
                continue;
            }
            let flat = [
                upper.as_deref() == Some(nominal.as_slice()),
                lower.as_deref() == Some(nominal.as_slice()),
            ];
            let mut queue = |change: Option<Vec<Value>>| {
                change.map(|change| {
                    excitations.push(change);
                    excitations.len() - 1
                })
            };
            let sides = [
                queue(upper.map(|upper| difference(&upper, &nominal))),
                queue(lower.map(|lower| difference(&nominal, &lower))),
            ];
            pending.push(Ok(PendingTangent::Residual { step, sides, flat }));
        }

        let mut tangents = engine
            .dc_operating_point_tangents(circuit, matrix, solution, &excitations)?
            .into_iter()
            .map(Some)
            .collect::<Vec<_>>();
        let mut take = |index: usize| {
            tangents[index]
                .take()
                .expect("each excitation is read once")
        };
        Ok(Ok(pending
            .into_iter()
            .map(|pending| {
                pending.map(|pending| match pending {
                    PendingTangent::Exact(index) => SolvedTangent::Exact(take(index)),
                    PendingTangent::Residual { step, sides, flat } => SolvedTangent::Residual {
                        step,
                        sides: sides.map(|side| side.map(&mut take)),
                        flat,
                    },
                })
            })
            .collect()))
    }

    /// The exact operating-point excitation `db/dp - dA/dp x*` of a design
    /// parameter on a deck of qualified linear elements, from the directions
    /// the parser captures when the deck is replayed with the parameter
    /// selected, or why this deck does not qualify.
    #[allow(clippy::too_many_arguments)]
    fn exact_design_excitation(
        engine: &Engine,
        limits: crate::resource::ResourceLimits,
        netlist: &Netlist,
        parameter: &str,
        value: Value,
        layout: &(usize, Vec<String>, Vec<String>),
        solution: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Result<Vec<Value>, &'static str>, SimulationError> {
        if let Some(reason) = Self::linear_parameter_refusal(netlist, parameter, false) {
            return Ok(Err(reason));
        }
        let Ok((directed, _)) = sensitivity_trial(Self::replay_parameter_overrides(
            netlist,
            &[(parameter.to_ascii_uppercase(), value)],
            Some(parameter),
            limits,
            abort,
        ))?
        else {
            return Ok(Err(
                "the deck cannot be replayed with the parameter selected",
            ));
        };
        if directed
            .parameter_direction()
            .is_none_or(|capture| capture.has_uncaptured_dependencies)
        {
            return Ok(Err(
                "the parameter reaches a value whose direction is not captured",
            ));
        }
        let engine = engine.resolved_for_netlist(&directed);
        let Ok((mut built, _)) =
            sensitivity_trial(engine.prepared_dc_probe_circuit(&directed, abort))?
        else {
            return Ok(Err(
                "the deck replayed with the parameter selected cannot be built",
            ));
        };
        // An enforced IC owns a DC voltage constraint whose direction is not
        // captured.
        if built
            .capacitors
            .ic_branch_indices
            .iter()
            .any(Option::is_some)
        {
            return Ok(Err("an enforced initial condition holds a node"));
        }
        let Some(capture) = built.parameter_direction.take() else {
            return Ok(Err("the build captured no direction"));
        };
        if !Self::linear_capture_complete(&capture, false) {
            return Ok(Err("some owning element has no captured direction"));
        }
        if (
            built.matrix_size(),
            built.node_names_sorted(),
            built.branch_names_sorted(),
        ) != *layout
        {
            return Ok(Err(
                "the replayed circuit's unknowns differ from the solved ones",
            ));
        }
        let at = solution
            .iter()
            .map(|&value| Complex64::new(value, 0.0))
            .collect::<Vec<_>>();
        let mut forcing = vec![ComplexDirection::zero(); solution.len()];
        if sensitivity_trial(add_parameter_forcing(
            &mut forcing,
            &built,
            &capture,
            directed.options.rshunt,
            &at,
            None,
            abort,
        ))?
        .is_err()
        {
            return Ok(Err("some owning element's derivative is incomplete"));
        }
        // Narrow only now, after every owning factor has been applied.
        Ok(forcing
            .iter()
            .map(|entry| {
                let value = entry.re.binary64();
                (value.is_finite() && (value != 0.0 || entry.re == 0.0)).then_some(value)
            })
            .collect::<Option<Vec<_>>>()
            .ok_or("the excitation exceeds finite representable precision"))
    }
}
