//! Circuit evaluation adapter for the bounded runtime optimizer.

mod objective;
use super::{ServiceContext, is_ground_like};
#[cfg(test)]
use super::{build_engine_config, parse_runner_netlist_with_abort};
use crate::error::{ServiceRunError, ServiceRunResult, ensure_not_aborted, poll_periodically};
use crate::optimization::{ObjectiveSample, run_optimization_with_gradient_evaluator};
pub use crate::optimization::{
    OptimizationAlgorithmMode, OptimizationData, OptimizationEvaluation, OptimizationGoalMode,
    OptimizationRunConfig, OptimizationVariable, objective_to_cost,
    run_optimization_with_cost_evaluator,
};
use rspice_core::{Value, abort_signal::AbortSignal, engine::Engine};
use std::collections::{HashMap, HashSet};

#[cfg(test)]
thread_local! {
    /// Operating points the optimizer solved on this thread.
    static OPERATING_POINTS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

fn count_operating_point() {
    #[cfg(test)]
    OPERATING_POINTS.with(|count| count.set(count.get() + 1));
}

/// Apply one captured execution policy to materialization and every candidate.
pub(crate) fn run_optimization_analysis_with_context(
    netlist_text: &str,
    config: &OptimizationRunConfig,
    environment: Option<&rspice_core::engine::MonteCarloEnvironment>,
    context: ServiceContext<'_>,
) -> ServiceRunResult<OptimizationData> {
    run_optimization_analysis(netlist_text, config, environment, context, true)
}

/// `exact_gradients: false` keeps every derivative a finite difference: the
/// search exactly as it ran before exact derivatives existed, for comparison.
fn run_optimization_analysis(
    netlist_text: &str,
    config: &OptimizationRunConfig,
    environment: Option<&rspice_core::engine::MonteCarloEnvironment>,
    context: ServiceContext<'_>,
    exact_gradients: bool,
) -> ServiceRunResult<OptimizationData> {
    let abort = context.abort;
    ensure_not_aborted(abort)?;
    config.validate().map_err(ServiceRunError::Failure)?;
    let objective_unit = config.operating_point_objective_unit();
    if !config.objective_unit.trim().is_empty() {
        objective_unit
            .convert_value(0.0, &config.objective_unit)
            .map_err(|error| {
                ServiceRunError::Failure(format!("Optimization objective unit: {error}"))
            })?;
    }
    if let Some(point) = environment
        && (!point.temperature_celsius.is_finite()
            || point.temperature_celsius <= -273.15
            || point.supply_voltage.is_some() != point.nominal_supply_voltage.is_some())
    {
        return Err(ServiceRunError::Failure(
                "Optimization Run Set requires a physical temperature and a complete supply/nominal pair".into(),
            ));
    }
    let netlist = context.parse(netlist_text)?;
    for variable in &config.variables {
        if netlist.params.get(&variable.name).is_none() {
            return Err(ServiceRunError::Failure(format!(
                "Optimization parameter {:?} is not declared in this circuit",
                variable.name,
            )));
        }
    }
    let engine = Engine::new(context.engine_config(&netlist));
    let convert = |value: Value| -> ServiceRunResult<Value> {
        if config.objective_unit.trim().is_empty() {
            Ok(value)
        } else {
            objective_unit
                .convert_value(value, &config.objective_unit)
                .map_err(ServiceRunError::Failure)
        }
    };
    // Exact derivatives exist for a V(node, ref) objective, whose derivative
    // is a difference of two operating-point tangent entries, reported in a
    // unit that is a pure scale of volts. An operating-point expression can
    // read device observables the tangent does not carry, so its variables
    // keep finite differences.
    let unit_scale = convert(1.0)
        .ok()
        .filter(|scale| convert(0.0).ok() == Some(0.0) && convert(2.0).ok() == Some(2.0 * scale));
    let exact_objective = if !exact_gradients {
        None
    } else if config.objective_expression.is_some() {
        log::debug!(
            "[optimization.gradient] objective expression: every variable takes finite differences (the expression has no derivative with respect to the operating point)"
        );
        None
    } else if unit_scale.is_none() {
        log::debug!(
            "[optimization.gradient] objective unit {:?} is not a scale of volts: every variable takes finite differences",
            config.objective_unit
        );
        None
    } else {
        unit_scale
    };
    let mut reported = HashSet::new();
    // A deck that gives no parameter a tangent gives none at any candidate.
    let mut deck_refused = false;
    run_optimization_with_gradient_evaluator(
        config,
        engine.config().resource_limits,
        abort,
        |vars, want_gradient| {
            let candidate = materialize_optimization_candidate(
                &engine,
                &netlist,
                &config.variables,
                vars,
                environment,
                abort,
            )?;
            let (Some(scale), true, false) = (exact_objective, want_gradient, deck_refused) else {
                return Ok(ObjectiveSample {
                    value: convert(evaluate_optimization_objective(
                        &candidate, config, context,
                    )?)?,
                    gradient: Vec::new(),
                });
            };
            let (value, derivatives) =
                evaluate_voltage_objective_with_derivatives(&candidate, config, context, vars)?;
            let gradient = match derivatives {
                Ok(derivatives) => derivatives
                    .into_iter()
                    .zip(&config.variables)
                    .map(|(derivative, variable)| match derivative {
                        Ok(derivative) => Some(derivative * scale),
                        Err(reason) => {
                            if reported.insert((variable.name.clone(), reason.clone())) {
                                log::debug!(
                                    "[optimization.gradient] {}: finite differences ({reason})",
                                    variable.name
                                );
                            }
                            None
                        }
                    })
                    .collect(),
                Err(reason) => {
                    log::debug!(
                        "[optimization.gradient] every variable takes finite differences for the rest of the run ({reason})"
                    );
                    deck_refused = true;
                    Vec::new()
                }
            };
            Ok(ObjectiveSample {
                value: convert(value)?,
                gradient,
            })
        },
    )
}

fn evaluate_optimization_objective(
    netlist: &rspice_core::Netlist,
    config: &OptimizationRunConfig,
    context: ServiceContext<'_>,
) -> ServiceRunResult<Value> {
    let abort = context.abort;
    ensure_not_aborted(abort)?;
    let engine = Engine::new(context.engine_config(netlist));
    count_operating_point();
    let dc = engine
        .run_dc_op_with_abort(netlist, abort)
        .map_err(|error| {
            ServiceRunError::from_core("DC operating point failed during optimization", error)
        })?;

    if let Some(expression) = &config.objective_expression {
        return objective::evaluate_with_context(expression, netlist, &dc, context);
    }
    voltage_objective(netlist, config, &dc, abort).map(|(objective, _)| objective)
}

/// The V(node, ref) objective at one candidate and its derivative with
/// respect to each configured variable, all from one operating-point solve:
/// each derivative is a difference of the operating point's tangent with
/// respect to the variable's `.param`, or the reason that tangent does not
/// exist; or, as `Err`, the reason the deck gives no variable a tangent.
fn evaluate_voltage_objective_with_derivatives(
    netlist: &rspice_core::Netlist,
    config: &OptimizationRunConfig,
    context: ServiceContext<'_>,
    variables: &HashMap<String, Value>,
) -> ServiceRunResult<(Value, Result<Vec<Result<Value, String>>, String>)> {
    let abort = context.abort;
    ensure_not_aborted(abort)?;
    let engine = Engine::new(context.engine_config(netlist));
    let parameters = config
        .variables
        .iter()
        .map(|variable| (variable.name.clone(), variables[&variable.name]))
        .collect::<Vec<_>>();
    count_operating_point();
    let solved = engine
        .run_dc_op_with_design_tangents_with_abort(netlist, &parameters, abort)
        .map_err(|error| {
            ServiceRunError::from_core("DC operating point failed during optimization", error)
        })?;
    let (objective, [node, reference]) =
        voltage_objective(netlist, config, &solved.operating_point, abort)?;
    let derivatives = solved.tangents.map(|tangents| {
        tangents
            .iter()
            .map(|tangent| {
                let tangent = tangent.as_ref().map_err(Clone::clone)?;
                tangent
                    .derivative(objective, |direction| {
                        Some(
                            direction.node_voltages.get(node)?
                                - direction.node_voltages.get(reference)?,
                        )
                    })
                    .map_err(str::to_owned)
            })
            .collect()
    });
    Ok((objective, derivatives))
}

/// The V(node, ref) objective of a solved operating point and the two node
/// indices it reads.
fn voltage_objective(
    netlist: &rspice_core::Netlist,
    config: &OptimizationRunConfig,
    dc: &rspice_core::SimulationResult,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<(Value, [usize; 2])> {
    let node_idx =
        resolve_node_index_case_insensitive(&dc.node_names, &config.objective_node, abort)?
            .ok_or_else(|| {
                ServiceRunError::Failure(format!(
                    "Optimization objective node '{}' not found",
                    config.objective_node
                ))
            })?;
    let ref_idx = if is_ground_like(netlist, &config.objective_ref) {
        Some(0usize)
    } else {
        resolve_node_index_case_insensitive(&dc.node_names, &config.objective_ref, abort)?
    }
    .ok_or_else(|| {
        ServiceRunError::Failure(format!(
            "Optimization objective reference node '{}' not found",
            config.objective_ref
        ))
    })?;

    // `try_voltage` answers `None` both for an index the result does not have
    // and for a net only the event domain resolves; the second is the one an
    // objective can be written against by mistake, so it says so.
    let event_only = |node: usize, role: &str| -> ServiceRunError {
        match dc.event_only_node_kind(node) {
            Some(kind) => ServiceRunError::Failure(
                rspice_core::analysis::transient::event_only_voltage_refusal(
                    dc.node_names.get(node).map_or(role, |name| name.as_str()),
                    kind,
                    rspice_core::analysis::transient::EventTraceSurface::SolvedPoint,
                ),
            ),
            None => {
                ServiceRunError::Failure(format!("Optimization {role} voltage index out of bounds"))
            }
        }
    };
    let node_v = dc
        .try_voltage(node_idx)
        .ok_or_else(|| event_only(node_idx, "node"))?;
    let ref_v = dc
        .try_voltage(ref_idx)
        .ok_or_else(|| event_only(ref_idx, "reference"))?;
    ensure_not_aborted(abort)?;
    let objective = node_v - ref_v;
    if !objective.is_finite() {
        return Err(ServiceRunError::Failure(format!(
            "Optimization objective V({},{}) is non-finite",
            config.objective_node, config.objective_ref
        )));
    }
    Ok((objective, [node_idx, ref_idx]))
}

/// Materialize one bounded coordinate, with all variable and temperature
/// overrides applied before dependent parameters and models are evaluated.
pub(crate) fn materialize_optimization_candidate(
    engine: &Engine,
    circuit: &rspice_core::Netlist,
    configured: &[OptimizationVariable],
    variables: &HashMap<String, Value>,
    environment: Option<&rspice_core::engine::MonteCarloEnvironment>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<rspice_core::Netlist> {
    use rspice_core::netlist::{StepCommand, StepSweep, StepTarget};
    ensure_not_aborted(abort)?;
    let mut steps = configured
        .iter()
        .map(|variable| StepCommand {
            target: StepTarget::Param,
            name: variable.name.clone(),
            param_name: None,
            sweep: StepSweep::List(vec![variables[&variable.name]]),
        })
        .collect::<Vec<_>>();
    if let Some(point) = environment {
        steps.push(StepCommand {
            target: StepTarget::Temp,
            name: "TEMP".into(),
            param_name: None,
            sweep: StepSweep::List(vec![point.temperature_celsius]),
        });
    }
    let plan = engine
        .plan_step_commands_with_abort(
            circuit,
            &steps,
            rspice_core::engine::StepPlanLimits::from_resource_limits(
                engine.config().resource_limits,
            ),
            abort,
        )
        .map_err(|error| ServiceRunError::from_core("Optimization candidate planning", error))?;
    let mut candidate = engine
        .materialize_step_run_with_abort(&plan, 0, abort)
        .map_err(|error| {
            ServiceRunError::from_core("Optimization candidate materialization", error)
        })?
        .into_parts()
        .1;
    if let Some(point) = environment
        && let (Some(supply), Some(nominal)) = (point.supply_voltage, point.nominal_supply_voltage)
    {
        crate::netlist_preparation::apply_voltage_corner(
            &mut candidate,
            supply,
            nominal,
            &point.supply_source_names,
            abort,
        )?;
    }
    Ok(candidate)
}

fn resolve_node_index_case_insensitive(
    node_names: &[String],
    target: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Option<usize>> {
    ensure_not_aborted(abort)?;
    let trimmed = target.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    for (index, name) in node_names.iter().enumerate() {
        poll_periodically(abort, index)?;
        if name.eq_ignore_ascii_case(trimmed) {
            return Ok(Some(index));
        }
    }
    ensure_not_aborted(abort)?;
    Ok(None)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::super::test_entry::ctx;
    use super::*;

    struct AbortOnPoll {
        abort_on: usize,
        polls: AtomicUsize,
    }

    impl AbortOnPoll {
        fn new(abort_on: usize) -> Self {
            Self {
                abort_on,
                polls: AtomicUsize::new(0),
            }
        }
    }

    impl AbortSignal for AbortOnPoll {
        fn is_aborted(&self) -> bool {
            self.polls.fetch_add(1, Ordering::Relaxed) + 1 >= self.abort_on
        }
    }

    const OPTIMIZATION_DECK: &str = "\
Optimization cancellation
.param RLOAD=1k
V1 in 0 1
R1 in out {RLOAD}
R2 out 0 1k
.op
.end
";

    #[test]
    fn optimization_seed_temperature_cooling_and_gradient_threshold_govern_the_search() {
        let run = |config: &OptimizationRunConfig| {
            run_optimization_analysis_with_context(
                OPTIMIZATION_DECK,
                config,
                None,
                ctx(&rspice_core::NoAbort),
            )
            .unwrap()
        };
        let mut config = OptimizationRunConfig {
            algorithm: OptimizationAlgorithmMode::SimulatedAnnealing,
            max_iterations: 25,
            cost_tolerance: 1e-30,
            ..Default::default()
        };
        config.search.random_seed = 42;
        let first = run(&config);
        assert_eq!(first.costs, run(&config).costs);
        config.search.random_seed = 43;
        assert_ne!(first.variable_traces, run(&config).variable_traces);
        config.search.random_seed = 42;
        config.search.sa_initial_temp = 1e-12;
        assert_ne!(first.costs, run(&config).costs);
        config.search.sa_initial_temp = 100.0;
        config.search.sa_cooling_rate = 0.01;
        assert_ne!(first.costs, run(&config).costs);
        config.algorithm = OptimizationAlgorithmMode::GradientDescent;
        config.search.var_tolerance = 1e9;
        let loose = run(&config);
        config.search.var_tolerance = 1e-14;
        let strict = run(&config);
        assert!(strict.iterations.len() > loose.iterations.len());
    }

    #[test]
    fn optimization_honors_early_abort_before_invalid_input() {
        let abort = AbortOnPoll::new(1);
        let result = run_optimization_analysis_with_context(
            "invalid",
            &OptimizationRunConfig::default(),
            None,
            ctx(&abort),
        );

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn optimization_honors_abort_during_objective_trials() {
        let abort = AbortOnPoll::new(12);
        let result = run_optimization_analysis_with_context(
            OPTIMIZATION_DECK,
            &OptimizationRunConfig::default(),
            None,
            ctx(&abort),
        );

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
        assert!(abort.polls.load(Ordering::Relaxed) >= 12);
    }

    /// Under the ngspice dialect only `0` and `GND` are node 0; `GND!` is an
    /// ordinary node, here held 1 V above ground. The objective is measured
    /// against it, so the optimizer has to place V(out) 0.2 V above it, at
    /// 1.2 V, which RLOAD = 2k/3 does. Measured against node 0 instead, the
    /// 0.2 V target sits out of RLOAD's range and the search pins at its bound.
    #[test]
    fn an_objective_reference_the_dialect_does_not_ground_is_measured() {
        const DECK: &str = "\
Floating reference
.param RLOAD=1k
V1 in 0 2
R1 in out {RLOAD}
R2 out 0 1k
VREF GND! 0 1
RREF GND! 0 1k
.op
.end
";
        let config = OptimizationRunConfig {
            objective_unit: "V".into(),
            objective_ref: "GND!".into(),
            target: Some(0.2),
            ..Default::default()
        };
        let data =
            run_optimization_analysis_with_context(DECK, &config, None, ctx(&rspice_core::NoAbort))
                .unwrap();

        let [objective] = data.best_objectives.as_slice() else {
            panic!("one objective: {:?}", data.best_objectives);
        };
        assert!(
            (objective.value - 0.2).abs() < 1e-3,
            "V(out,GND!) = {}",
            objective.value
        );
        let rload = data.best_variables["RLOAD"];
        assert!((rload - 2000.0 / 3.0).abs() < 5.0, "RLOAD = {rload}");
    }
}

#[cfg(test)]
mod variable_domain_tests {
    use super::super::test_entry::ctx;
    use super::*;
    use rspice_core::NoAbort;
    use rspice_simulation_contract::optimization_search::OptimizationVariableDomain as Domain;

    #[test]
    fn optimization_variable_domains_drive_real_circuits_and_report_physical_values() {
        let deck = "Variable domains\n.param X=1\nV1 out 0 {X}\nR1 out 0 1k\n.end\n";
        for (domain, min, max, initial, target, scale, expected) in [
            (Domain::Logarithmic, 1e-12, 1e6, 1.0, 1.0, 1e6, 1e-6),
            (
                Domain::Quantized { step: 2.0 },
                0.0,
                10.0,
                0.0,
                6.4,
                1.0,
                6.0,
            ),
            (
                Domain::Discrete {
                    values: vec![100.0, 470.0, 2200.0],
                },
                100.0,
                2200.0,
                100.0,
                500.0,
                1.0,
                470.0,
            ),
        ] {
            let mut config = OptimizationRunConfig {
                variables: vec![OptimizationVariable {
                    name: "X".into(),
                    min,
                    max,
                    initial,
                }],
                objective_unit: String::new(),
                objective_expression: Some(format!("{scale}*V(out)")),
                target: Some(target),
                cost_tolerance: 1e-10,
                max_iterations: 90,
                ..Default::default()
            };
            config
                .search
                .variable_domains
                .insert("x".into(), domain.clone());
            let data =
                run_optimization_analysis_with_context(deck, &config, None, ctx(&NoAbort)).unwrap();
            assert!(data.converged, "{domain:?}: {data:?}");
            assert!(
                (data.best_variables["X"] - expected).abs() <= expected.abs().max(1e-12) * 1e-5,
                "{domain:?}: {:?}",
                data.best_variables
            );
            for value in &data.variable_traces["X"] {
                assert!(*value >= min && *value <= max);
                match &domain {
                    Domain::Quantized { step } => {
                        assert_eq!((value - min) / step, ((value - min) / step).round())
                    }
                    Domain::Discrete { values } => assert!(values.contains(value)),
                    _ => {}
                }
            }
            let expected_cost = (scale * data.best_variables["X"] - target).powi(2);
            assert!((data.best_cost - expected_cost).abs() < 1e-8 * expected_cost.abs().max(1.0));
            config.algorithm = OptimizationAlgorithmMode::GradientDescent;
            if domain.is_discrete() {
                assert!(config.validate().unwrap_err().contains("pattern search"));
            }
        }
        let mut config = OptimizationRunConfig::default();
        config
            .search
            .variable_domains
            .insert("missing".into(), Domain::Logarithmic);
        assert!(
            config
                .validate()
                .unwrap_err()
                .contains("no configured variable")
        );
    }
}

#[cfg(test)]
mod exact_gradient_tests {
    //! Exact objective derivatives against central differences of re-solved
    //! operating points, and the searches they drive against the
    //! finite-difference searches they replace.

    use super::super::test_entry::ctx;
    use super::*;
    use rspice_core::NoAbort;
    use rspice_core::engine::DesignTangentMethod;

    /// Every element linear, so the exact path. RTOP and RBOT each reach two
    /// elements through expressions.
    const DIVIDER: &str = "\
Divider
.param RTOP=1k RBOT=2k
V1 in 0 3
R1 in out {RTOP}
R2 out 0 {RBOT}
R3 out 0 {2*RBOT+RTOP}
.op
.end
";

    /// A diode bias point: a series resistor, and the saturation current,
    /// which reaches the circuit only through the model card.
    const DIODE: &str = "\
Diode bias
.param RS=1k ISAT=1e-14
V1 in 0 5
R1 in d {RS}
D1 d 0 DMOD
.model DMOD D(IS={ISAT} N=1.05 RS=5)
.op
.end
";

    /// A four-resistor BJT bias point: the upper base resistor, and the
    /// forward beta, a model-card parameter.
    const BJT: &str = "\
BJT bias
.param RBIAS=47k BETA=120
VCC vcc 0 12
RB1 vcc b {RBIAS}
RB2 b 0 10k
RC vcc c 4.7k
RE e 0 1k
Q1 c b e QN
.model QN NPN(IS=1e-15 BF={BETA} VAF=80)
.op
.end
";

    fn voltage_config(node: &str, variables: &[(&str, f64)]) -> OptimizationRunConfig {
        OptimizationRunConfig {
            objective_unit: String::new(),
            objective_node: node.into(),
            variables: variables
                .iter()
                .map(|&(name, value)| OptimizationVariable {
                    name: name.into(),
                    min: value * 0.5,
                    max: value * 2.0,
                    initial: value,
                })
                .collect(),
            ..Default::default()
        }
    }

    struct Row {
        variable: String,
        method: Option<DesignTangentMethod>,
        exact: Result<f64, String>,
        central: f64,
    }

    /// The search's derivatives at `values`, each beside a reference: central
    /// differences of operating points re-solved at a relative step of
    /// ±`step` and ±`step`/2, Richardson-extrapolated.
    fn gradient_rows(deck: &str, node: &str, values: &[(&str, f64)], step: f64) -> Vec<Row> {
        let context = ctx(&NoAbort);
        let netlist = context.parse(deck).unwrap();
        let engine = Engine::new(context.engine_config(&netlist));
        let config = voltage_config(node, values);
        let point = |moved: Option<(&str, f64)>| -> HashMap<String, f64> {
            values
                .iter()
                .map(|&(name, value)| {
                    let value = match moved {
                        Some((target, moved)) if target == name => moved,
                        _ => value,
                    };
                    (name.to_owned(), value)
                })
                .collect()
        };
        let candidate = |vars: &HashMap<String, f64>| {
            materialize_optimization_candidate(
                &engine,
                &netlist,
                &config.variables,
                vars,
                None,
                &NoAbort,
            )
            .unwrap()
        };
        let nominal = candidate(&point(None));
        let (_, derivatives) =
            evaluate_voltage_objective_with_derivatives(&nominal, &config, context, &point(None))
                .unwrap();
        let derivatives = derivatives.unwrap();
        let parameters = values
            .iter()
            .map(|&(name, value)| (name.to_owned(), value))
            .collect::<Vec<_>>();
        let methods = Engine::new(context.engine_config(&nominal))
            .run_dc_op_with_design_tangents_with_abort(&nominal, &parameters, &NoAbort)
            .unwrap()
            .tangents
            .unwrap();
        values
            .iter()
            .zip(derivatives)
            .zip(methods)
            .map(|((&(name, value), exact), method)| {
                let h = value * step;
                let at = |moved: f64| {
                    evaluate_optimization_objective(
                        &candidate(&point(Some((name, moved)))),
                        &config,
                        context,
                    )
                    .unwrap()
                };
                let central = |h: f64| (at(value + h) - at(value - h)) / (2.0 * h);
                Row {
                    variable: name.into(),
                    method: method.ok().map(|tangent| tangent.method()),
                    exact,
                    central: (4.0 * central(0.5 * h) - central(h)) / 3.0,
                }
            })
            .collect()
    }

    fn assert_agreement(deck: &str, rows: &[Row], method: DesignTangentMethod, tolerance: f64) {
        for row in rows {
            let exact = row
                .exact
                .as_ref()
                .unwrap_or_else(|reason| panic!("{deck} {}: {reason}", row.variable));
            let relative = (exact - row.central).abs() / row.central.abs();
            eprintln!(
                "| {deck} | {} | {:?} | {exact:.9e} | {:.9e} | {relative:.1e} |",
                row.variable, row.method, row.central
            );
            assert_eq!(row.method, Some(method), "{deck} {}", row.variable);
            assert!(
                relative <= tolerance,
                "{deck} {}: exact {exact:e}, central {:e}, relative {relative:e}",
                row.variable,
                row.central
            );
        }
    }

    /// Relative agreement each derivative needs with its reference.
    const TOLERANCE: f64 = 1e-7;

    #[test]
    fn exact_objective_derivatives_match_central_differences_of_resolved_operating_points() {
        // The reference re-solves the operating point at 1e-3 and 5e-4 of
        // each parameter, so it carries the solves' convergence error over
        // that step; both paths agree with it to well inside the tolerances.
        assert_agreement(
            "divider",
            &gradient_rows(DIVIDER, "out", &[("RTOP", 1e3), ("RBOT", 2e3)], 1e-3),
            DesignTangentMethod::Exact,
            TOLERANCE,
        );
        assert_agreement(
            "diode",
            &gradient_rows(DIODE, "d", &[("RS", 1e3), ("ISAT", 1e-14)], 1e-3),
            DesignTangentMethod::ResidualDifference,
            TOLERANCE,
        );
        assert_agreement(
            "bjt",
            &gradient_rows(BJT, "c", &[("RBIAS", 47e3), ("BETA", 120.0)], 1e-3),
            DesignTangentMethod::ResidualDifference,
            TOLERANCE,
        );
    }

    /// Run a search and count the operating points it solved.
    fn counted(
        deck: &str,
        config: &OptimizationRunConfig,
        environment: Option<&rspice_core::engine::MonteCarloEnvironment>,
        exact_gradients: bool,
    ) -> (OptimizationData, usize) {
        OPERATING_POINTS.with(|count| count.set(0));
        let started = std::time::Instant::now();
        let data =
            run_optimization_analysis(deck, config, environment, ctx(&NoAbort), exact_gradients)
                .unwrap();
        eprintln!(
            "  exact_gradients={exact_gradients}: {:.1} ms (debug build)",
            started.elapsed().as_secs_f64() * 1e3
        );
        (data, OPERATING_POINTS.with(std::cell::Cell::get))
    }

    #[test]
    fn exact_gradients_reach_the_same_optimum_in_no_more_iterations_with_fewer_solves() {
        // K scales one resistor; each target lies inside K's range.
        let divider =
            "Scaled divider\n.param K=1\nV1 in 0 10\nR1 in out {K*1k}\nR2 out 0 1k\n.op\n.end\n";
        let diode = "Scaled diode\n.param K=1\nV1 in 0 5\nR1 in d {K*1k}\nD1 d 0 DMOD\n.model DMOD D(IS=1e-14 N=1.05 RS=5)\n.op\n.end\n";
        let bjt = "Scaled BJT\n.param K=1\nVCC vcc 0 12\nRB1 vcc b {K*47k}\nRB2 b 0 10k\nRC vcc c 4.7k\nRE e 0 1k\nQ1 c b e QN\n.model QN NPN(IS=1e-15 BF=120 VAF=80)\n.op\n.end\n";
        for (name, deck, node, unit, target, min, max, tolerance) in [
            ("divider", divider, "out", "V", 4.0, 0.2, 5.0, 1e-10),
            ("diode", diode, "d", "mV", 710.0, 0.2, 10.0, 1e-4),
            ("bjt", bjt, "c", "V", 6.0, 0.5, 2.0, 1e-10),
        ] {
            let config = OptimizationRunConfig {
                algorithm: OptimizationAlgorithmMode::GradientDescent,
                objective_unit: unit.into(),
                objective_node: node.into(),
                target: Some(target),
                variables: vec![OptimizationVariable {
                    name: "K".into(),
                    min,
                    max,
                    initial: 1.0,
                }],
                cost_tolerance: tolerance,
                max_iterations: 200,
                ..Default::default()
            };
            let (fd, fd_solves) = counted(deck, &config, None, false);
            let (exact, exact_solves) = counted(deck, &config, None, true);
            let per_iteration = |solves: usize, data: &OptimizationData| {
                solves as f64 / (data.iterations.len() - 1) as f64
            };
            eprintln!(
                "| {name} | fd: {} it, {fd_solves} OP ({:.2}/it), K {:.9}, cost {:.2e} | exact: {} it, {exact_solves} OP ({:.2}/it), K {:.9}, cost {:.2e} |",
                fd.iterations.len() - 1,
                per_iteration(fd_solves, &fd),
                fd.best_variables["K"],
                fd.best_cost,
                exact.iterations.len() - 1,
                per_iteration(exact_solves, &exact),
                exact.best_variables["K"],
                exact.best_cost,
            );
            assert!(fd.converged && exact.converged, "{name}: {fd:?}\n{exact:?}");
            assert!(
                fd.best_cost <= tolerance && exact.best_cost <= tolerance,
                "{name}"
            );
            assert!(
                (exact.best_variables["K"] - fd.best_variables["K"]).abs() <= 1e-4,
                "{name}: {:?} against {:?}",
                exact.best_variables,
                fd.best_variables
            );
            assert!(exact.iterations.len() <= fd.iterations.len(), "{name}");
            assert!(
                exact_solves < fd_solves,
                "{name}: {exact_solves} against {fd_solves}"
            );
        }
    }

    #[test]
    fn variables_without_an_exact_derivative_keep_the_finite_difference_search() {
        let deck =
            "Scaled divider\n.param K=1\nV1 in 0 10\nR1 in out {K*1k}\nR2 out 0 1k\n.op\n.end\n";
        let base = OptimizationRunConfig {
            algorithm: OptimizationAlgorithmMode::GradientDescent,
            objective_unit: String::new(),
            target: Some(4.0),
            variables: vec![OptimizationVariable {
                name: "K".into(),
                min: 0.2,
                max: 5.0,
                initial: 1.0,
            }],
            max_iterations: 12,
            ..Default::default()
        };
        // An operating-point expression has no derivative with respect to
        // the solution here, and a supply corner is an edit the deck's replay
        // does not reproduce: in both, the search is the finite-difference
        // search, evaluation for evaluation.
        let expression = OptimizationRunConfig {
            objective_expression: Some("V(out)".into()),
            ..base.clone()
        };
        let corner = rspice_core::engine::MonteCarloEnvironment {
            temperature_celsius: 27.0,
            supply_voltage: Some(11.0),
            nominal_supply_voltage: Some(10.0),
            supply_source_names: vec!["V1".into()],
        };
        for (name, config, environment) in [
            ("expression", &expression, None),
            ("supply corner", &base, Some(&corner)),
        ] {
            let (fd, fd_solves) = counted(deck, config, environment, false);
            let (exact, exact_solves) = counted(deck, config, environment, true);
            assert_eq!(exact.costs, fd.costs, "{name}");
            assert_eq!(exact.variable_traces, fd.variable_traces, "{name}");
            assert_eq!(exact_solves, fd_solves, "{name}");
        }
    }
}
