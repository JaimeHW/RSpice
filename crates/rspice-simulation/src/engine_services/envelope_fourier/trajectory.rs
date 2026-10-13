//! Preserve the accepted analog signals and their physical current evidence.
use super::*;
use rspice_core::engine::{TransientDeviceOpTrace, TransientResult};
use rspice_core::{CurrentImpulseOwner, CurrentImpulseTrace};
use std::collections::{HashMap, HashSet};

pub(super) struct EnvelopeSignal {
    pub name: String,
    pub unit: &'static str,
    pub values: Vec<Value>,
    pub current: Option<CurrentImpulseTrace>,
    pub voltage: Option<rspice_core::VoltageImpulseTrace>,
}

pub(super) struct EnvelopeTrajectory {
    pub time: Vec<Value>,
    pub signals: Vec<EnvelopeSignal>,
    pub convergence:
        Option<std::sync::Arc<rspice_results::convergence_quality::TransientConvergenceEvidence>>,
}

impl EnvelopeTrajectory {
    pub(super) fn from_result(
        mut result: TransientResult,
        netlist: &rspice_core::Netlist,
        abort: &dyn AbortSignal,
    ) -> ServiceRunResult<Self> {
        ensure_not_aborted(abort)?;
        result
            .validate_impulses()
            .map_err(ServiceRunError::Failure)?;
        let branch_names = std::mem::take(&mut result.branch_names);
        let currents = std::mem::take(&mut result.branch_currents);
        if branch_names.len() != currents.len() {
            return Err(ServiceRunError::Failure(
                "Envelope branch names and current histories are inconsistent".into(),
            ));
        }
        let device_traces = std::mem::take(&mut result.device_op_traces);
        // Match Fourier's current-observation contract: ordinary transient
        // runs have sampled currents only. Once physical impulse observation
        // is active, each retained current must have explicit full coverage.
        let observes_impulses = result.current_impulses.is_some();
        let mut impulses: HashMap<_, _> = result
            .current_impulses
            .take()
            .unwrap_or_default()
            .into_iter()
            .map(|trace| (trace.owner.to_string().to_ascii_lowercase(), trace))
            .collect();
        let solved_branches: HashSet<String> = branch_names
            .iter()
            .map(|name| name.to_ascii_lowercase())
            .collect();
        // A metered terminal's impulse history is its meter's. Shared before
        // the branch currents below take the meter's own.
        for trace in &device_traces {
            if let Some((owner, Some(meter))) = device_trace_current(trace, &solved_branches)
                && let Some(history) = impulses.get(&meter).cloned()
            {
                impulses
                    .entry(owner.to_string().to_ascii_lowercase())
                    .or_insert(history);
            }
        }
        let observes_voltages = result.voltage_impulses.is_some();
        let mut voltage_impulses: HashMap<_, _> = result
            .voltage_impulses
            .take()
            .unwrap_or_default()
            .into_iter()
            .map(|trace| (trace.node_name.to_ascii_lowercase(), trace))
            .collect();
        let names = result.node_names.clone();
        let voltage =
            TransientData::from_retained_voltage_history_with_abort(result, &names, abort)?;
        let mut signals: Vec<_> = voltage
            .voltages
            .into_iter()
            .map(|(name, values)| {
                ensure_not_aborted(abort)?;
                let trace = if observes_voltages {
                    Some(
                        voltage_impulses
                            .remove(&name.to_ascii_lowercase())
                            .filter(|trace| trace.complete)
                            .ok_or_else(|| {
                                ServiceRunError::Failure(format!(
                                    "Envelope voltage '{name}' requires complete impulse history"
                                ))
                            })?,
                    )
                } else {
                    None
                };
                Ok(EnvelopeSignal {
                    name,
                    unit: "V",
                    values,
                    current: None,
                    voltage: trace,
                })
            })
            .collect::<ServiceRunResult<_>>()?;
        let mut seen: HashSet<_> = signals
            .iter()
            .map(|signal| signal.name.to_ascii_lowercase())
            .collect();
        let mut retain_current = |name: String, values: Vec<Value>| -> ServiceRunResult<()> {
            ensure_not_aborted(abort)?;
            if values.is_empty() {
                return Ok(());
            }
            if values.len() != voltage.time.len()
                || name.trim().is_empty()
                || !seen.insert(name.to_ascii_lowercase())
            {
                return Err(ServiceRunError::Failure(format!(
                    "Envelope current '{name}' has an invalid identity or sample count"
                )));
            }
            for (index, value) in values.iter().enumerate() {
                poll_periodically(abort, index)?;
                if !value.is_finite() {
                    return Err(ServiceRunError::Failure(format!(
                        "Envelope current '{name}' contains a non-finite sample"
                    )));
                }
            }
            let current = if observes_impulses {
                Some(
                    impulses
                        .remove(&name.to_ascii_lowercase())
                        .filter(|trace| trace.complete)
                        .ok_or_else(|| {
                            ServiceRunError::Failure(format!(
                                "Envelope current '{name}' requires complete impulse history"
                            ))
                        })?,
                )
            } else {
                None
            };
            signals.push(EnvelopeSignal {
                name,
                unit: "A",
                values,
                current,
                voltage: None,
            });
            Ok(())
        };
        for (name, values) in branch_names.into_iter().zip(currents) {
            retain_current(format!("I({name})"), values)?;
        }
        for trace in device_traces {
            if let Some((owner, _)) = device_trace_current(&trace, &solved_branches) {
                let name = owner.to_string();
                // Device OP recording collects an entire report. Project its
                // selected parameters, just as the core retains the selected
                // voltage and branch-current operands before integration.
                if netlist.saves.selects(&name)
                    || !netlist.measurements.is_empty()
                    || netlist.options.output_snapshots.unwrap_or(false)
                    || netlist.output_requests.iter().any(|request| {
                        request.selects_transient_device_parameter(
                            &trace.device_name,
                            &trace.parameter,
                        )
                    })
                {
                    retain_current(name, trace.values)?;
                }
            }
        }
        Ok(Self {
            time: voltage.time,
            signals,
            convergence: None,
        })
    }
}

/// The current a device trace is, if it is one, and the impulse key of the
/// solved branch it is measured in.
///
/// The output layer's rule: a lead current (`@q1[ic]`, Xyce's `I` and one
/// letter) is a current, and so is a parameter whose `device:param` names a
/// branch the run solved — a metered Verilog-A terminal, the branch of its
/// zero-volt meter, whatever the length of its label (`@x1[iout]`).
fn device_trace_current(
    trace: &TransientDeviceOpTrace,
    solved_branches: &HashSet<String>,
) -> Option<(CurrentImpulseOwner, Option<String>)> {
    let branch =
        format!("{}:{}", trace.device_name.trim(), trace.parameter.trim()).to_ascii_lowercase();
    let meter = solved_branches
        .contains(&branch)
        .then(|| format!("i({branch})"));
    let owner = trace.current_owner().or_else(|| {
        meter.is_some().then(|| CurrentImpulseOwner::DeviceLead {
            device_name: trace.device_name.clone(),
            parameter: trace.parameter.clone(),
        })
    })?;
    Some((owner, meter))
}

pub(super) fn run_transient(
    netlist: &rspice_core::Netlist,
    config: &EnvelopeRunConfig,
    step_time: Value,
    context: ServiceContext<'_>,
) -> ServiceRunResult<EnvelopeTrajectory> {
    let abort = context.abort;
    let engine = Engine::new(context.engine_config(netlist));
    let result = engine
        .run_tran_with_abort(netlist, config.stop_time, step_time, abort)
        .map_err(|error| ServiceRunError::from_core("Envelope transient analysis", error))?;
    let mut data = EnvelopeTrajectory::from_result(result, netlist, abort)?;
    data.convergence = Some(std::sync::Arc::new(
        rspice_results::convergence_quality::TransientConvergenceEvidence::capture(
            engine.convergence_quality(),
            &data.time,
            abort,
        )
        .map_err(ServiceRunError::from)?,
    ));
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::CurrentImpulsePoint;

    fn meter_impulse(branch: &str, charge: Value) -> CurrentImpulseTrace {
        CurrentImpulseTrace {
            derivatives: Vec::new(),
            owner: CurrentImpulseOwner::Branch {
                branch_name: branch.into(),
            },
            complete: true,
            points: vec![CurrentImpulsePoint {
                time: 0.25,
                charge_coulombs: charge,
            }],
        }
    }

    /// A metered Verilog-A terminal is a current whatever the length of its
    /// label: `@x1[iout]` is retained in amperes, as `@x1[ia]` is, and both
    /// carry the impulse history of the meter they are measured in.
    #[test]
    fn a_metered_terminal_current_of_any_label_length_is_a_current() {
        let netlist = rspice_core::Netlist::parse(
            "metered terminals\nV1 in 0 1\nX1 in 0 dev\n.save @x1[iout] @x1[ia]\n.end\n",
        )
        .expect("the deck parses");
        let lead = |parameter: &str, current: Value| TransientDeviceOpTrace {
            device_name: "X1".into(),
            parameter: parameter.into(),
            values: vec![current; 3],
        };
        let result = TransientResult {
            time: vec![0.0, 0.5, 1.0],
            step_sizes: vec![0.0, 0.5, 0.5],
            voltages: vec![vec![1.0; 3]],
            branch_currents: vec![vec![1.0e-3; 3], vec![2.0e-3; 3]],
            num_nodes: 1,
            node_names: vec!["in".into()],
            branch_names: vec!["X1:IOUT".into(), "X1:IA".into()],
            digital_traces: vec![],
            digital_buses: vec![],
            real_traces: vec![],
            device_op_traces: vec![lead("iout", 1.0e-3), lead("ia", 2.0e-3)],
            store_traces: vec![],
            fft_results: vec![],
            voltage_impulses: None,
            current_impulses: Some(vec![
                meter_impulse("X1:IOUT", 3.0e-9),
                meter_impulse("X1:IA", 4.0e-9),
            ]),
        };
        let trajectory = EnvelopeTrajectory::from_result(result, &netlist, &NoAbort)
            .expect("every retained current has its impulse history");
        for (name, current, charge) in [("@X1[iout]", 1.0e-3, 3.0e-9), ("@X1[ia]", 2.0e-3, 4.0e-9)]
        {
            let signal = trajectory
                .signals
                .iter()
                .find(|signal| signal.name == name)
                .unwrap_or_else(|| panic!("{name} is retained"));
            assert_eq!(signal.unit, "A", "{name}");
            assert_eq!(signal.values, [current; 3], "{name}");
            let impulses = signal.current.as_ref().expect("an impulse history");
            assert_eq!(impulses.points[0].charge_coulombs, charge, "{name}");
        }
    }
}
