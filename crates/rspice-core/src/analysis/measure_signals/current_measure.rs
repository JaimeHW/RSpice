//! Sparse charge accumulation at the same accepted row as live measurements.

use super::*;
use current_observation::{CurrentImpulseContribution, CurrentObservationError};

fn invalid(detail: impl Into<String>) -> CurrentObservationError {
    CurrentObservationError::Invalid {
        detail: detail.into(),
    }
}

struct Cursor<'a> {
    term: CurrentImpulseContribution<'a>,
    next: usize,
}

pub(super) struct Integral<'a> {
    cursors: Vec<Cursor<'a>>,
    charge: Value,
    correction: Value,
}

impl Integral<'_> {
    pub(super) fn advance(
        &mut self,
        state: &LiveMeasureState,
        previous_axis: Option<Value>,
        axis: Value,
        finite_result: Option<Value>,
        abort: &dyn AbortSignal,
    ) -> Result<Option<Value>, CurrentObservationError> {
        let LiveMeasureState::IntegralStatistic {
            integral,
            width,
            mode,
            previous,
            ..
        } = state
        else {
            return Err(invalid(
                "current integral has no integral measurement state",
            ));
        };
        if finite_result.is_none() || previous.is_none() {
            return Ok(finite_result);
        }
        if let Some(start) = previous_axis {
            for cursor in &mut self.cursors {
                if abort.is_aborted() {
                    return Err(CurrentObservationError::Aborted);
                }
                while let Some(point) = cursor.term.trace.points.get(cursor.next) {
                    if cursor.next.is_multiple_of(64) && abort.is_aborted() {
                        return Err(CurrentObservationError::Aborted);
                    }
                    if point.time > axis {
                        break;
                    }
                    cursor.next += 1;
                    // An accepted event belongs to (previous, current]. The
                    // first retained point, including a checkpoint seam, is
                    // the integration seed and contributes no interval.
                    if point.time <= start {
                        continue;
                    }
                    let charge = crate::numerics::scaled_exp_product(
                        &[point.charge_coulombs, cursor.term.weight],
                        &[],
                        0.0,
                    );
                    if !charge.is_finite() || (charge == 0.0 && cursor.term.weight != 0.0) {
                        return Err(invalid(
                            "weighted current impulse charge is not representable",
                        ));
                    }
                    crate::numerics::compensated_add(
                        &mut self.charge,
                        &mut self.correction,
                        charge,
                    );
                }
            }
        }
        let mut total = *integral;
        let mut correction = 0.0;
        crate::numerics::compensated_add(&mut total, &mut correction, self.charge);
        crate::numerics::compensated_add(&mut total, &mut correction, self.correction);
        let total = total + correction;
        let value = match mode {
            LiveIntegralMode::Average if *width > 0.0 => total / *width,
            LiveIntegralMode::Integral { direction } => total * *direction,
            _ => {
                return Err(invalid(
                    "charge impulses require an integral or average measurement",
                ));
            }
        };
        if !value.is_finite() {
            return Err(invalid("current integral or average is non-finite"));
        }
        Ok(Some(value))
    }
}

fn inclusive_window(axis: &[Value], lower: Value, upper: Value) -> Option<(Value, Value)> {
    let mut selected = axis
        .iter()
        .copied()
        .filter(|value| live_axis_in_window(*value, lower, upper, 1e-12));
    let first = selected.next()?;
    Some((first.next_down(), selected.next_back().unwrap_or(first)))
}

pub(super) fn compile<'a>(
    netlist: &Netlist,
    result: &'a TransientResult,
    statement: &MeasureStatement,
    state: &LiveMeasureState,
    abort: &dyn AbortSignal,
) -> Result<Option<Integral<'a>>, CurrentObservationError> {
    if result.current_impulses.is_none() {
        return Ok(None);
    }
    let full = result
        .time
        .first()
        .copied()
        .zip(result.time.last().copied());
    let Some((first, last)) = full else {
        return Ok(None);
    };
    let regular = |spec: &str,
                   window: Option<(Value, Value)>|
     -> Result<(), CurrentObservationError> {
        let Some(window) = window else {
            return Ok(());
        };
        let terms = current_observation::resolve(Some(netlist), result, spec, window, abort)?;
        if terms.is_empty() {
            Ok(())
        } else {
            Err(invalid(format!(
                "measurement '{}' requires a finite-valued signal: '{spec}' has charge impulses in its observation window; use INTEG or AVG for total current",
                statement.name
            )))
        }
    };
    let condition = |condition: &LiveCondition, window| -> Result<(), CurrentObservationError> {
        regular(&condition.left.authored, window)?;
        if let LiveConditionOperand::Waveform(signal) = &condition.right {
            regular(&signal.authored, window)?;
        }
        Ok(())
    };
    match state {
        LiveMeasureState::IntegralStatistic {
            signal,
            lower,
            upper,
            mode,
            ..
        } if !matches!(mode, LiveIntegralMode::Rms) => {
            // Preserve the established accepted-row finite quadrature window.
            // Impulses use exact endpoints, never the row-selection tolerance.
            if matches!(&statement.measure_type, MeasureType::Integ { from: Some(from), to: Some(to), .. } if from > to)
            {
                return Err(invalid("Empty range"));
            }
            let Some((start_below, stop)) = inclusive_window(&result.time, *lower, *upper) else {
                return Ok(None);
            };
            let start = start_below.next_up();
            let terms = current_observation::resolve(
                Some(netlist),
                result,
                &signal.authored,
                (start, stop),
                abort,
            )?;
            if terms.is_empty() {
                return Ok(None);
            }
            let mut cursors = Vec::new();
            cursors
                .try_reserve_exact(terms.len())
                .map_err(|_| invalid("cannot allocate current integral cursors"))?;
            cursors.extend(terms.into_iter().map(|term| Cursor { term, next: 0 }));
            return Ok(Some(Integral {
                cursors,
                charge: 0.0,
                correction: 0.0,
            }));
        }
        LiveMeasureState::IntegralStatistic {
            signal,
            lower,
            upper,
            ..
        }
        | LiveMeasureState::Extremum {
            signal,
            lower,
            upper,
            ..
        }
        | LiveMeasureState::PeakToPeak {
            signal,
            lower,
            upper,
            ..
        } => {
            regular(
                &signal.authored,
                inclusive_window(&result.time, *lower, *upper),
            )?;
        }
        LiveMeasureState::Point {
            signal,
            at,
            condition: when,
            lower,
            upper,
            ..
        } => {
            let window = if let Some(at) = at {
                Some((at.next_down(), *at))
            } else {
                inclusive_window(&result.time, *lower, *upper)
            };
            if let Some(signal) = signal {
                regular(&signal.authored, window)?;
            }
            if let Some(when) = when {
                condition(when, window)?;
            }
        }
        LiveMeasureState::ErrorFunction {
            measured,
            comparison,
            lower,
            upper,
            ..
        } => {
            let window = inclusive_window(&result.time, *lower, *upper);
            regular(&measured.authored, window)?;
            regular(&comparison.authored, window)?;
        }
        LiveMeasureState::RiseFall { signal, .. } | LiveMeasureState::FileError { signal, .. } => {
            regular(&signal.authored, Some((first.next_down(), last)))?
        }
        LiveMeasureState::Equation { .. } | LiveMeasureState::Param { .. } => {
            let (from, to, td) = match state {
                LiveMeasureState::Equation { from, to, td, .. } => (*from, *to, *td),
                _ => (None, None, None),
            };
            let window = inclusive_window(
                &result.time,
                from.unwrap_or(first).max(td.unwrap_or(first)),
                to.unwrap_or(last),
            );
            if let MeasureType::Equation { expression, .. } | MeasureType::Param { expression } =
                &statement.measure_type
            {
                let spec = if matches!(
                    expression.kind,
                    crate::netlist::measure::MeasureExpressionKind::Expression
                ) {
                    format!("{{{}}}", expression.text)
                } else {
                    expression.text.clone()
                };
                regular(&spec, window)?;
            }
        }
        LiveMeasureState::Delay(delay) => {
            for clause in [&delay.trigger, &delay.target] {
                if let Some(condition) = &clause.condition {
                    let window = inclusive_window(
                        &result.time,
                        clause.from.unwrap_or(first).max(clause.td.unwrap_or(first)),
                        clause.to.unwrap_or(last),
                    );
                    regular(&condition.left.authored, window)?;
                    if let LiveConditionOperand::Waveform(signal) = &condition.right {
                        regular(&signal.authored, window)?;
                    }
                }
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CurrentImpulseOwner, CurrentImpulsePoint, CurrentImpulseTrace};

    fn fixture() -> TransientResult {
        TransientResult {
            time: (0..=8).map(|n| n as Value / 8.0).collect(),
            step_sizes: vec![0.125; 9],
            voltages: vec![vec![3.0; 9]],
            branch_currents: vec![vec![1.0; 9]],
            num_nodes: 1,
            node_names: vec!["out".into()],
            branch_names: vec!["V1".into()],
            digital_traces: vec![],
            digital_buses: vec![],
            real_traces: vec![],
            device_op_traces: vec![TransientDeviceOpTrace {
                device_name: "Q1".into(),
                parameter: "ic".into(),
                values: vec![0.0; 9],
            }],
            store_traces: vec![],
            fft_results: vec![],
            current_impulses: Some(vec![
                CurrentImpulseTrace {
                    owner: CurrentImpulseOwner::Branch {
                        branch_name: "V1".into(),
                    },
                    complete: true,
                    points: [
                        (0.0, 99.0),
                        (0.25, 0.003),
                        (0.5, 0.002),
                        (0.75, -0.001),
                        (1.0, 0.004),
                    ]
                    .into_iter()
                    .map(|(time, charge_coulombs)| CurrentImpulsePoint {
                        time,
                        charge_coulombs,
                    })
                    .collect(),
                },
                CurrentImpulseTrace {
                    owner: CurrentImpulseOwner::DeviceLead {
                        device_name: "Q1".into(),
                        parameter: "ic".into(),
                    },
                    complete: true,
                    points: vec![CurrentImpulsePoint {
                        time: 0.5,
                        charge_coulombs: -0.002,
                    }],
                },
            ]),
        }
    }

    fn deck(cards: &str) -> Netlist {
        Netlist::parse(&format!(
            "* current measurements\nV1 out 0 0\nR1 out 0 1k\n{cards}\n.end\n"
        ))
        .unwrap()
    }

    fn value(results: &[MeasureResult], name: &str, expected: Value) {
        let result = results
            .iter()
            .find(|r| r.name.eq_ignore_ascii_case(name))
            .unwrap();
        assert!(result.passed, "{result:?}");
        assert!(
            (result.value.unwrap() - expected).abs() < 2e-14,
            "{result:?} != {expected}"
        );
    }

    #[test]
    fn current_measure_integral_average_and_affine_lead_charge_are_physical() {
        let netlist = deck(
            ".meas tran charge INTEG I(V1) FROM=0 TO=1\n.meas tran mean AVG I(V1) FROM=0 TO=1\n.meas tran middle INTEG I(V1) FROM=.25 TO=.75\n.meas tran middle_avg AVG I(V1) FROM=.25 TO=.75\n.meas tran lead INTEG IC(Q1)\n.meas tran affine INTEG {2*I(V1)+IC(Q1)}",
        );
        let results = evaluate_tran_measurements(&netlist, &fixture());
        value(&results, "charge", 1.008);
        value(&results, "mean", 1.008);
        value(&results, "middle", 0.501);
        value(&results, "middle_avg", 1.002);
        value(&results, "lead", -0.002);
        value(&results, "affine", 2.014);
    }

    #[test]
    fn current_measure_charge_reaches_same_row_and_forward_dependencies() {
        let netlist = deck(
            ".meas tran before EQN {charge}\n.meas tran charge INTEG I(V1)\n.meas tran after EQN {charge}\n.meas tran picked FIND charge AT=.5\n.meas tran final PARAM {charge}",
        );
        let result = fixture();
        let results = evaluate_tran_measurements(&netlist, &result);
        value(&results, "before", 0.879);
        value(&results, "after", 1.008);
        value(&results, "picked", 0.505);
        value(&results, "final", 1.008);
        let traces = evaluate_tran_equation_measurements(&netlist, &result).unwrap();
        let after = traces
            .iter()
            .find(|trace| trace.name.eq_ignore_ascii_case("after"))
            .unwrap();
        assert!((after.values[4] - 0.505).abs() < 2e-14);
    }

    #[test]
    fn current_measure_missing_coverage_propagates_only_through_actual_reads() {
        let netlist = deck(
            ".meas tran charge INTEG I(V1)\n.meas tran used EQN {charge+1}\n.meas tran unused EQN {IF(0,charge,7)}\n.meas tran unused_param PARAM {IF(0,charge,7)}\n.meas tran good AVG V(out)",
        );
        let mut result = fixture();
        result.current_impulses = Some(vec![]);
        let results = evaluate_tran_measurements(&netlist, &result);
        for name in ["charge", "used"] {
            let failed = results
                .iter()
                .find(|r| r.name.eq_ignore_ascii_case(name))
                .unwrap();
            assert!(!failed.passed && failed.value.is_none(), "{failed:?}");
            assert!(
                failed.error.as_ref().unwrap().contains("impulse history"),
                "{failed:?}"
            );
        }
        value(&results, "unused", 7.0);
        value(&results, "unused_param", 7.0);
        value(&results, "good", 3.0);
    }

    #[test]
    fn current_measure_nonlinear_and_at_impulse_refuse_but_regular_points_work() {
        let netlist = deck(
            ".meas tran rms RMS I(V1)\n.meas tran peak MAX I(V1)\n.meas tran at_jump FIND I(V1) AT=.5\n.meas tran away FIND I(V1) AT=.625\n.meas tran bad_product INTEG {I(V1)*I(V1)}",
        );
        let results = evaluate_tran_measurements(&netlist, &fixture());
        for name in ["rms", "peak", "at_jump", "bad_product"] {
            let failed = results
                .iter()
                .find(|r| r.name.eq_ignore_ascii_case(name))
                .unwrap();
            assert!(!failed.passed && failed.value.is_none(), "{failed:?}");
        }
        value(&results, "away", 1.0);
    }

    #[test]
    fn current_measure_window_resume_and_legacy_sampled_contracts_agree() {
        let netlist = deck(
            ".meas tran charge INTEG I(V1) FROM=.5 TO=1\n.meas tran mean AVG I(V1) FROM=.5 TO=1",
        );
        let full = fixture();
        let expected = evaluate_tran_measurements(&netlist, &full);
        let mut resumed = full.clone();
        resumed.time.drain(..4);
        resumed.branch_currents[0].drain(..4);
        resumed.voltages[0].drain(..4);
        resumed.device_op_traces[0].values.drain(..4);
        for trace in resumed.current_impulses.as_mut().unwrap() {
            trace.points.retain(|p| p.time > 0.5);
        }
        assert_eq!(evaluate_tran_measurements(&netlist, &resumed), expected);
        value(&expected, "charge", 0.503);
        value(&expected, "mean", 1.006);
        resumed.current_impulses = None;
        let legacy = evaluate_tran_measurements(&netlist, &resumed);
        value(&legacy, "charge", 0.5);
        value(&legacy, "mean", 1.0);
    }

    #[test]
    fn current_measure_aliases_keep_finite_and_impulse_owners_together() {
        for (probe, expected) in [
            ("IR(V1)", 1.008),
            ("IC(Q1)", -0.002),
            ("@Q1[ic]", -0.002),
            ("N(Q1:ic)", -0.002),
        ] {
            let netlist = deck(&format!(".meas tran charge INTEG {probe}"));
            value(
                &evaluate_tran_measurements(&netlist, &fixture()),
                "charge",
                expected,
            );
        }
    }

    #[test]
    fn current_measure_vector_point_refusals_preserve_unrelated_records() {
        let netlist = deck(
            ".meas tran_cont bad FIND I(V1) WHEN TIME=.5\n.meas tran_cont good FIND V(out) WHEN TIME=.5",
        );
        let results = evaluate_tran_continuous_measurements(&netlist, &fixture());
        assert_eq!(results.len(), 2);
        assert!(
            results[0]
                .failure
                .as_ref()
                .unwrap()
                .contains("charge impulses")
        );
        assert!(results[0].records.is_empty());
        assert!(results[1].passed(), "{:?}", results[1]);
        assert_eq!(results[1].records.len(), 1);
    }

    #[test]
    fn current_measure_contracts_apply_to_total_charge_and_cancellation_is_typed() {
        let netlist = Netlist::parse_with_options(
            "* measurement contract\nV1 out 0 0\nR1 out 0 1k\n.meas tran checked INTEG I(V1) GOAL=1.008 FAILVALUE=1.005\n.end\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        ).unwrap();
        let result = fixture();
        let measured = evaluate_tran_measurements(&netlist, &result);
        assert!((measured[0].raw_value.unwrap() - 1.008).abs() < 2e-14);
        assert!(!measured[0].passed && measured[0].failure_limit_exceeded);
        assert_eq!(measured[0].expected, Some(1.008));
        let abort = crate::abort_signal::CountingAbort::new(2);
        assert!(matches!(
            evaluate_tran_measurements_with_abort(&netlist, &result, &abort),
            Err(SimulationError::Aborted)
        ));
    }

    #[test]
    fn current_measure_complete_zero_allows_nonlinear_but_requires_finite_samples() {
        let netlist = deck(
            ".meas tran nonlinear INTEG {I(V1)*I(V1)}\n.meas tran rms RMS I(V1)\n.meas tran charge INTEG I(V1)",
        );
        let mut result = fixture();
        for trace in result.current_impulses.as_mut().unwrap() {
            trace.points.clear();
        }
        let values = evaluate_tran_measurements(&netlist, &result);
        for name in ["nonlinear", "rms", "charge"] {
            value(&values, name, 1.0);
        }
        result.branch_currents.clear();
        for failed in evaluate_tran_measurements(&netlist, &result) {
            assert!(!failed.passed && failed.raw_value.is_none(), "{failed:?}");
        }
    }

    #[test]
    fn current_measure_rejects_partial_malformed_and_unrepresentable_charge() {
        let netlist = deck(".meas tran charge INTEG {2*I(V1)}\n.meas tran good AVG V(out)");
        for mutation in 0..4 {
            let mut result = fixture();
            let trace = &mut result.current_impulses.as_mut().unwrap()[0];
            match mutation {
                0 => trace.complete = false,
                1 => trace.points[1].time = Value::NAN,
                2 => trace.points[1].charge_coulombs = Value::MAX,
                _ => trace.points[1].time = trace.points[0].time,
            }
            let results = evaluate_tran_measurements(&netlist, &result);
            assert!(
                !results[0].passed && results[0].raw_value.is_none(),
                "{results:?}"
            );
            value(&results, "good", 3.0);
        }
    }

    #[test]
    fn current_measure_impulse_endpoints_are_exact_even_between_samples() {
        let netlist = deck(".meas tran charge INTEG I(V1) FROM=.25 TO=.75");
        let mut result = fixture();
        result.current_impulses.as_mut().unwrap()[0].points = [
            (0.25, 99.0),
            (0.25_f64.next_up(), 0.01),
            (0.3, 0.02),
            (0.75, 0.03),
            (0.75_f64.next_up(), 77.0),
        ]
        .into_iter()
        .map(|(time, charge_coulombs)| CurrentImpulsePoint {
            time,
            charge_coulombs,
        })
        .collect();
        value(
            &evaluate_tran_measurements(&netlist, &result),
            "charge",
            0.56,
        );
    }

    #[test]
    fn current_measure_device_parameter_operand_requires_balanced_brackets() {
        for operand in ["@Q1", "@Q1[", "@Q1[]", "@Q1[ic", "@[ic]"] {
            assert!(
                Netlist::parse(&format!(
                    "* malformed probe\n.meas tran bad INTEG {operand}\n.end"
                ))
                .is_err(),
                "{operand}"
            );
        }
    }
}
