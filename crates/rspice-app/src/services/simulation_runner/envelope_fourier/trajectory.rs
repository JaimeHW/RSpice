//! Preserve the accepted analog signals and their physical current evidence.
use super::*;
use rspice_core::{CurrentImpulseTrace, engine::TransientResult};
use std::collections::{HashMap, HashSet};

pub(super) struct EnvelopeSignal {
    pub name: String,
    pub unit: &'static str,
    pub values: Vec<Value>,
    pub current: Option<CurrentImpulseTrace>,
}

pub(super) struct EnvelopeTrajectory {
    pub time: Vec<Value>,
    pub signals: Vec<EnvelopeSignal>,
    pub convergence: Option<std::sync::Arc<crate::state::TransientConvergenceEvidence>>,
}

impl EnvelopeTrajectory {
    pub(super) fn from_result(
        mut result: TransientResult,
        netlist: &rspice_core::Netlist,
        abort: &dyn AbortSignal,
    ) -> ServiceRunResult<Self> {
        ensure_not_aborted(abort)?;
        result
            .validate_current_impulses()
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
        let names = result.node_names.clone();
        let voltage =
            TransientData::from_retained_voltage_history_with_abort(result, &names, abort)?;
        let mut signals: Vec<_> = voltage
            .voltages
            .into_iter()
            .map(|(name, values)| EnvelopeSignal {
                name,
                unit: "V",
                values,
                current: None,
            })
            .collect();
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
            });
            Ok(())
        };
        for (name, values) in branch_names.into_iter().zip(currents) {
            retain_current(format!("I({name})"), values)?;
        }
        for trace in device_traces {
            if let Some(owner) = trace.current_owner() {
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

pub(super) fn run_transient(
    netlist: &rspice_core::Netlist,
    config: &EnvelopeRunConfig,
    step_time: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<EnvelopeTrajectory> {
    let engine = Engine::new(build_engine_config(netlist, None));
    let result = engine
        .run_tran_with_abort(netlist, config.stop_time, step_time, abort)
        .map_err(|error| ServiceRunError::from_core("Envelope transient analysis", error))?;
    let mut data = EnvelopeTrajectory::from_result(result, netlist, abort)?;
    data.convergence = Some(std::sync::Arc::new(
        crate::state::TransientConvergenceEvidence::capture(
            engine.convergence_quality(),
            &data.time,
            abort,
        )
        .map_err(ServiceRunError::from)?,
    ));
    Ok(data)
}
