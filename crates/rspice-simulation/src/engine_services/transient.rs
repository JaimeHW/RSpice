//! Transient analysis.
//!
//! Time-domain integration from an initial condition, returning node
//! voltage waveforms.

use super::{ServiceRunError, ServiceRunResult};
use crate::error::{ensure_not_aborted, poll_periodically};
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::TransientResult;
use std::collections::HashSet;

/// Transient analysis waveform data
#[derive(Debug, Clone)]
pub struct TransientData {
    /// Time points
    pub time: Vec<Value>,

    /// Node voltages: (node_name, values)
    pub voltages: Vec<(String, Vec<Value>)>,

    /// What the solver had to do to produce these waveforms.
    ///
    /// Force-accepted points converged in Newton but failed the local
    /// truncation error test. Missing evidence means the quality is unknown.
    pub convergence:
        Option<std::sync::Arc<rspice_results::convergence_quality::TransientConvergenceEvidence>>,
}

impl TransientData {
    /// Validate the time axis and every retained voltage without requiring a
    /// non-ground voltage column. Terminal checks may observe grounded devices
    /// or accepted current traces retained separately from these voltages.
    pub(super) fn from_retained_voltage_history_with_abort(
        result: TransientResult,
        node_names: &[String],
        abort: &dyn AbortSignal,
    ) -> ServiceRunResult<Self> {
        ensure_not_aborted(abort)?;
        let invalid_time_axis = || {
            ServiceRunError::Failure(
                "Transient engine returned an empty, non-finite, or non-increasing time axis"
                    .to_owned(),
            )
        };
        if result.time.is_empty() {
            return Err(invalid_time_axis());
        }
        for (index, &time) in result.time.iter().enumerate() {
            poll_periodically(abort, index)?;
            if !time.is_finite() || (index > 0 && time <= result.time[index - 1]) {
                return Err(invalid_time_axis());
            }
        }
        if node_names != result.node_names || node_names.len() != result.voltages.len() {
            return Err(ServiceRunError::Failure(format!(
                "Transient engine returned {} node names for {} voltage waveforms",
                node_names.len(),
                result.voltages.len()
            )));
        }

        let mut voltages = Vec::with_capacity(node_names.len());
        let mut seen_names = HashSet::with_capacity(node_names.len());

        for (name, samples) in node_names.iter().zip(result.voltages) {
            ensure_not_aborted(abort)?;
            let normalized = name.trim().to_ascii_lowercase();
            if normalized.is_empty() || !seen_names.insert(normalized) {
                return Err(ServiceRunError::Failure(
                    "Transient engine returned an empty or duplicate node identity".to_owned(),
                ));
            }
            if samples.is_empty() {
                continue;
            }
            if samples.len() != result.time.len() {
                return Err(ServiceRunError::Failure(format!(
                    "Transient node '{name}' returned an invalid sample vector"
                )));
            }
            for (sample_index, sample) in samples.iter().enumerate() {
                poll_periodically(abort, sample_index)?;
                if !sample.is_finite() {
                    return Err(ServiceRunError::Failure(format!(
                        "Transient node '{name}' returned an invalid sample vector"
                    )));
                }
            }
            if name == "0" || name.eq_ignore_ascii_case("gnd") {
                continue;
            }

            voltages.push((format!("V({name})"), samples));
        }
        ensure_not_aborted(abort)?;
        Ok(Self {
            time: result.time,
            voltages,
            // The engine is not in scope here; callers that have it overwrite
            // this from `Engine::convergence_quality`.
            convergence: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;
    use rspice_core::abort_signal::NoAbort;

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

    fn conversion_result() -> TransientResult {
        TransientResult {
            voltage_impulses: None,
            current_impulses: None,
            time: vec![0.0, 1.0],
            step_sizes: vec![0.0, 1.0],
            voltages: vec![vec![1.0, 1.0], vec![0.5, 0.5]],
            branch_currents: Vec::new(),
            num_nodes: 2,
            node_names: vec!["out".to_string(), "sense".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            digital_buses: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        }
    }

    #[test]
    fn transient_conversion_observes_counter_abort() {
        let result = conversion_result();
        let names = result.node_names.clone();
        let abort = AbortOnPoll::new(2);

        let converted =
            TransientData::from_retained_voltage_history_with_abort(result, &names, &abort);

        assert!(matches!(converted, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn transient_conversion_skips_traces_omitted_by_output_selection() {
        for omitted in [0, 1] {
            let mut result = conversion_result();
            let kept = result.voltages[1 - omitted].clone();
            result.voltages[omitted].clear();
            let names = result.node_names.clone();
            let data =
                TransientData::from_retained_voltage_history_with_abort(result, &names, &NoAbort)
                    .expect("an omitted trace does not invalidate the retained ones");
            assert_eq!(
                data.voltages,
                vec![(format!("V({})", names[1 - omitted]), kept)]
            );
        }
    }

    #[test]
    fn transient_conversion_rejects_malformed_retained_data() {
        let mut cases = Vec::new();
        for time in [
            vec![],
            vec![0.0, Value::NAN],
            vec![0.0, Value::INFINITY],
            vec![0.0, 0.0],
            vec![1.0, 0.0],
        ] {
            let mut result = conversion_result();
            result.time = time;
            cases.push(result);
        }
        for samples in [vec![0.5], vec![0.5, Value::NAN], vec![0.5, Value::INFINITY]] {
            let mut result = conversion_result();
            result.voltages[1] = samples;
            cases.push(result);
        }
        for name in ["", "OUT"] {
            let mut result = conversion_result();
            result.node_names[1] = name.to_owned();
            // Omitted traces still participate in node-identity validation.
            result.voltages[1].clear();
            cases.push(result);
        }
        let mut missing_vector = conversion_result();
        missing_vector.voltages.pop();
        cases.push(missing_vector);
        for (index, result) in cases.into_iter().enumerate() {
            let names = result.node_names.clone();
            assert!(
                matches!(
                    TransientData::from_retained_voltage_history_with_abort(
                        result, &names, &NoAbort
                    ),
                    Err(ServiceRunError::Failure(_))
                ),
                "malformed case {index} must fail"
            );
        }
        let result = conversion_result();
        let mut names = result.node_names.clone();
        names.swap(0, 1);
        assert!(matches!(
            TransientData::from_retained_voltage_history_with_abort(result, &names, &NoAbort),
            Err(ServiceRunError::Failure(_))
        ));
    }

    #[test]
    fn transient_conversion_polls_during_time_and_sample_validation() {
        let mut result = conversion_result();
        result.time = (0..256).map(Value::from).collect();
        result.time[255] = Value::NAN;
        let names = result.node_names.clone();
        assert!(matches!(
            TransientData::from_retained_voltage_history_with_abort(
                result,
                &names,
                &AbortOnPoll::new(3)
            ),
            Err(ServiceRunError::Aborted)
        ));

        let mut result = conversion_result();
        result.time = (0..256).map(Value::from).collect();
        result.voltages = vec![vec![1.0; 256], vec![0.5; 256]];
        result.voltages[0][255] = Value::NAN;
        let names = result.node_names.clone();
        // Entry, four time-axis polls, then the first node and its samples.
        assert!(matches!(
            TransientData::from_retained_voltage_history_with_abort(
                result,
                &names,
                &AbortOnPoll::new(8)
            ),
            Err(ServiceRunError::Aborted)
        ));
    }
}
