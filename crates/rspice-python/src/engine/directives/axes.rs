//! Materializing a shared `.STEP`/`.TEMP` plan into run coordinates.
//!
//! A deck with a run axis executes its directives once per coordinate, against
//! the netlist the canonical materializer produced for that coordinate. Nothing
//! here decides what a directive means; it decides which netlist and which
//! coordinate identity a directive runs under.

use super::*;

pub(super) fn run_materialized_directives(
    py_engine: &PyEngine,
    py: Python<'_>,
    netlist: &PyNetlist,
    analyses: &[rspice_core::execution::MaterializedAnalysis],
    coordinate: &PyRunCoordinate,
    continue_on_error: bool,
    out: &mut DirectiveOutcomes,
) -> PyResult<()> {
    for analysis in analyses {
        let command = analysis.command().ok_or_else(|| {
            crate::errors::SimulationError::new_err(format!(
                "canonical materializer omitted the command for {}",
                analysis.id()
            ))
        })?;
        let context = ExecutionContext {
            analysis_id: Some(analysis.output_namespace().analysis_component()),
            coordinate: Some(coordinate.clone()),
            transient_startup: TransientStartup::Card,
            upstream_analysis_id: analysis.planned().request().upstream().map(|id| id.tag()),
        };
        execute_directive(
            py_engine,
            py,
            netlist,
            command,
            Some(&context),
            continue_on_error,
            out,
        )?;
    }

    // FOUR is intentionally absent from `MaterializedAnalysis`: it remains
    // attached to the coordinate-local netlist and is evaluated against that
    // coordinate's final transient after every physical analysis has run.
    let mut next_fourier_ordinal = 1usize;
    for command in &netlist.inner.analyses {
        if matches!(command, AnalysisCommand::Four { .. }) {
            let context = ExecutionContext {
                analysis_id: Some(format!("four-{next_fourier_ordinal:03}")),
                coordinate: Some(coordinate.clone()),
                transient_startup: TransientStartup::Card,
                upstream_analysis_id: None,
            };
            next_fourier_ordinal += 1;
            execute_directive(
                py_engine,
                py,
                netlist,
                command,
                Some(&context),
                continue_on_error,
                out,
            )?;
        }
    }
    Ok(())
}
pub(super) fn run_axis_plan(
    py_engine: &PyEngine,
    py: Python<'_>,
    netlist: &PyNetlist,
    plan: &rspice_core::execution::DeckPlan,
    continue_on_error: bool,
    out: &mut DirectiveOutcomes,
) -> PyResult<Vec<PyMeasurement>> {
    let core_engine = py_engine.engine_for_netlist(&netlist.inner);
    let materializer = run_interruptible(py, &py_engine.active_runs, |abort| {
        core_engine
            .prepare_deck_plan_materializer_with_abort(&netlist.inner, plan, abort)
            .map_err(materialized_run_simulation_error)
    })?;
    let implicit_op = plan.analyses().len() == 1
        && plan.analyses()[0].id().kind() == rspice_core::execution::AnalysisKind::ImplicitOp;
    let compatibility_axis = implicit_op.then(|| single_legacy_axis(plan)).flatten();
    let mut compatibility_operating_points = Vec::new();
    let mut compatibility_complete = compatibility_axis.is_some();
    let mut measurements = Vec::new();

    for run_index in 0..materializer.len() {
        let materialized_run = run_interruptible(py, &py_engine.active_runs, |abort| {
            materializer
                .materialize_run_with_abort(run_index, abort)
                .map_err(materialized_run_simulation_error)
        })?;
        let (core_coordinate, materialized_netlist, _topology, materialized_analyses) =
            materialized_run.into_parts();
        if core_coordinate.ordinal() != run_index {
            return Err(crate::errors::SimulationError::new_err(
                "axis materializer returned a mismatched run index",
            ));
        }
        let coordinate = PyRunCoordinate::from_core(&core_coordinate);
        let compatibility_value = compatibility_axis
            .as_ref()
            .and_then(|_| legacy_coordinate_value(&core_coordinate));
        let materialized = PyNetlist {
            inner: materialized_netlist,
            resource_limits: netlist.resource_limits,
        };
        let mut coordinate_out = DirectiveOutcomes::default();

        if implicit_op {
            let [implicit_analysis] = materialized_analyses.as_slice() else {
                return Err(crate::errors::SimulationError::new_err(format!(
                    "canonical materializer returned {} analyses for an implicit operating point; expected exactly one",
                    materialized_analyses.len()
                )));
            };
            if implicit_analysis.command().is_some() {
                return Err(crate::errors::SimulationError::new_err(
                    "canonical materializer attached an authored command to implicit OP",
                ));
            }
            let context = ExecutionContext {
                analysis_id: Some(implicit_analysis.output_namespace().analysis_component()),
                coordinate: Some(coordinate.clone()),
                transient_startup: TransientStartup::Card,
                upstream_analysis_id: None,
            };
            match py_engine.dc_op_impl(py, &materialized) {
                Ok(result) => {
                    if let Some(value) = compatibility_value {
                        compatibility_operating_points.push((value, result.inner.clone()));
                    } else {
                        compatibility_complete = false;
                    }
                    let handle = Py::new(py, result)?;
                    coordinate_out
                        .op
                        .push_with(handle, |handle| handle.clone_ref(py));
                    let mut record = PyAnalysisRecord::executed("op", ".op (implicit)".to_string());
                    record.set_execution_context(context.analysis_id, context.coordinate);
                    coordinate_out.records.push(record);
                }
                Err(error) if continue_on_error => {
                    compatibility_complete = false;
                    let mut record = PyAnalysisRecord::skipped(
                        "op",
                        ".op (implicit)".to_string(),
                        &crate::errors::describe_pyerr(py, &error),
                    );
                    record.set_execution_context(context.analysis_id, context.coordinate);
                    coordinate_out.records.push(record);
                }
                Err(error) => return Err(error),
            }
        } else {
            run_materialized_directives(
                py_engine,
                py,
                &materialized,
                &materialized_analyses,
                &coordinate,
                continue_on_error,
                &mut coordinate_out,
            )?;
        }

        evaluate_pending_fourier(py, &mut coordinate_out)?;
        let mut coordinate_measurements =
            evaluate_measurements(py, &materialized.inner, &coordinate_out);
        set_measurement_execution_context(&mut coordinate_measurements, plan, Some(&coordinate));
        measurements.append(&mut coordinate_measurements);
        out.append(coordinate_out);
    }

    if compatibility_complete
        && compatibility_operating_points.len() == materializer.len()
        && let Some((kind, name)) = compatibility_axis
    {
        let result = PyDcSweepResult::new_named(compatibility_operating_points, &name);
        match kind {
            rspice_core::execution::AxisKind::Temperature => out.temperature = Some(result),
            rspice_core::execution::AxisKind::Data | rspice_core::execution::AxisKind::Step => {
                out.step_result = Some(result)
            }
            rspice_core::execution::AxisKind::Alter => {}
            _ => {}
        }
    }
    Ok(measurements)
}
fn single_legacy_axis(
    plan: &rspice_core::execution::DeckPlan,
) -> Option<(rspice_core::execution::AxisKind, String)> {
    let axis = plan.axes().first().filter(|_| plan.axes().len() == 1)?;
    let name = match axis.step_target() {
        Some(rspice_core::execution::StepAxisTarget::Parameter { name }) => name.clone(),
        Some(rspice_core::execution::StepAxisTarget::Device {
            name,
            parameter: Some(parameter),
        }) => format!("{name}:{parameter}"),
        Some(rspice_core::execution::StepAxisTarget::Device {
            name,
            parameter: None,
        }) => name.clone(),
        Some(rspice_core::execution::StepAxisTarget::Model { name, parameter }) => {
            format!("{name}:{parameter}")
        }
        Some(rspice_core::execution::StepAxisTarget::Temperature) => "TEMP".to_string(),
        Some(_) => axis.name().to_string(),
        None if axis.kind() == rspice_core::execution::AxisKind::Temperature => "TEMP".to_string(),
        None => axis.name().to_string(),
    };
    Some((axis.kind(), name))
}

fn legacy_coordinate_value(coordinate: &rspice_core::execution::RunCoordinate) -> Option<f64> {
    let assignment = coordinate.assignments().first()?;
    match assignment.value() {
        rspice_core::execution::RunAxisValue::Numeric(value) => Some(*value),
        rspice_core::execution::RunAxisValue::DataRow(_) => Some(assignment.value_index() as f64),
        rspice_core::execution::RunAxisValue::AlterVariant { .. } => None,
        _ => None,
    }
}

pub(super) fn set_measurement_execution_context(
    measurements: &mut [PyMeasurement],
    plan: &rspice_core::execution::DeckPlan,
    coordinate: Option<&PyRunCoordinate>,
) {
    for measurement in measurements {
        let kind = match measurement.analysis.as_str() {
            analysis if analysis.eq_ignore_ascii_case("TRAN") => {
                Some(rspice_core::execution::AnalysisKind::Tran)
            }
            analysis if analysis.eq_ignore_ascii_case("DC") => {
                Some(rspice_core::execution::AnalysisKind::Dc)
            }
            analysis if analysis.eq_ignore_ascii_case("AC") => {
                Some(rspice_core::execution::AnalysisKind::Ac)
            }
            analysis if analysis.eq_ignore_ascii_case("NOISE") => {
                Some(rspice_core::execution::AnalysisKind::Noise)
            }
            _ => None,
        };
        let analysis_id = kind.and_then(|kind| {
            plan.analyses()
                .iter()
                .rev()
                .find(|analysis| analysis.id().kind() == kind)
                .map(|analysis| analysis.id().tag())
        });
        measurement.set_execution_context(analysis_id, coordinate.cloned());
    }
}
