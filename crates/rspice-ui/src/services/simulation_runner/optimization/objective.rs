//! Optimization expressions use the same parser and projection as circuit outputs.

use super::*;
use rspice_core::execution::AnalysisResultKind;
use rspice_core::execution::{
    ProjectionSource, SignalProjection, observable_lookup, operating_point_observable_series,
    operating_point_projection_signals,
};
use rspice_core::netlist::Netlist;

pub fn validate_optimization_expression(expression: &str) -> Result<(), String> {
    if expression.trim().is_empty() {
        return Err("Optimization expression must not be empty".into());
    }
    if expression
        .chars()
        .any(|ch| ch.is_control() || matches!(ch, '{' | '}'))
    {
        return Err(
            "Enter one optimization expression without enclosing braces or line breaks".into(),
        );
    }
    Ok(())
}

pub(super) fn evaluate(
    expression: &str,
    netlist: &Netlist,
    dc: &rspice_core::SimulationResult,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Value> {
    validate_optimization_expression(expression).map_err(ServiceRunError::Failure)?;
    // The expression is one braced operand. It cannot introduce a second
    // output column or card, nor inherit an unrelated .SAVE selection.
    let selection = parse_runner_netlist_with_abort(
        &format!("Optimization objective\n.print op {{{expression}}}\n.end\n"),
        None,
        abort,
    )?;
    let projection = SignalProjection::from_netlist(&selection)
        .map_err(|error| ServiceRunError::from_core("Invalid optimization expression", error))?;
    let observables = operating_point_observable_series(dc);
    let signals = operating_point_projection_signals(dc).map_err(|error| {
        ServiceRunError::Failure(format!("Optimization observation error: {error}"))
    })?;
    let source =
        ProjectionSource::new(AnalysisResultKind::OperatingPoint, "optimization objective")
            .with_axis(vec![0.0])
            .with_signals(signals)
            .with_lookup(observable_lookup(&observables));
    let projected = projection
        .project(&netlist.params, &source, abort)
        .map_err(|error| ServiceRunError::from_core("Optimization expression failed", error))?;
    let columns = projected.signals();
    let value = if columns.len() == 1 {
        columns[0]
            .real()
            .filter(|values| values.len() == 1)
            .filter(|_| columns[0].validity().iter().all(|valid| *valid))
            .and_then(|values| values.first())
            .copied()
    } else {
        None
    };
    value.filter(|value| value.is_finite()).ok_or_else(|| {
        ServiceRunError::Failure(
            "Optimization expression must produce one finite real operating-point value".into(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::NoAbort;

    #[test]
    fn optimization_expressions_use_signed_current_power_parameters_and_functions() {
        let deck = "Objective expressions\n.param RLOAD=2k gain=4\nV1 in 0 1\nR1 in 0 {RLOAD}\n.save V(in)\n.end\n";
        let netlist = parse_runner_netlist_with_abort(deck, None, &NoAbort).unwrap();
        let dc = Engine::new(build_engine_config(&netlist, None))
            .run_dc_op_with_abort(&netlist, &NoAbort)
            .unwrap();
        for (expression, expected) in [
            ("I(V1)", -0.0005),
            ("I(R1)", 0.0005),
            ("-V(in)*I(V1)", 0.0005),
            ("gain*abs(I(V1))", 0.002),
            ("(V(in)-0.5)^2 + 1e3*I(V1)^2", 0.25025),
        ] {
            let value = evaluate(expression, &netlist, &dc, &NoAbort).unwrap();
            assert!((value - expected).abs() < 1e-12, "{expression}: {value}");
        }
        for expression in [
            "V(missing)",
            "bogus(V(in))",
            "V(in)/0",
            "I(V1)}\n.op\n.print op {V(in)",
        ] {
            assert!(
                evaluate(expression, &netlist, &dc, &NoAbort).is_err(),
                "{expression}"
            );
        }
    }

    #[test]
    fn optimization_can_find_a_current_target_without_a_voltage_objective() {
        let deck = "Current target\n.param RLOAD=2k\nV1 in 0 1\nR1 in 0 {RLOAD}\n.end\n";
        let config = OptimizationRunConfig {
            objective_expression: Some("I(V1)".into()),
            objective_node: String::new(),
            objective_ref: String::new(),
            variables: vec![OptimizationVariable {
                name: "RLOAD".into(),
                min: 500.0,
                max: 5000.0,
                initial: 4000.0,
            }],
            target: Some(-0.001),
            cost_tolerance: 1e-16,
            max_iterations: 180,
            ..Default::default()
        };
        let result = run_optimization_analysis_with_config_and_source_path_and_abort(
            deck, &config, None, &NoAbort,
        )
        .unwrap();
        assert!(result.converged, "{result:?}");
        assert!(result.best_cost <= 1e-16, "{}", result.best_cost);
        assert!(
            (result.best_variables["RLOAD"] - 1000.0).abs() < 0.1,
            "{:?}",
            result.best_variables
        );
    }
}
