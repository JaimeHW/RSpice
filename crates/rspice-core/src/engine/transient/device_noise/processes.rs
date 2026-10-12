//! The Verilog-A noise processes of a circuit, collected in one place.
//!
//! A *process* is one noise-function call of one instance — `white_noise`,
//! `flicker_noise`, `noise_table` or `noise_table_log` — identified by the
//! compiler's dense `process_id`. It carries one unit train and as many
//! injections as the call's value reaches circuit equations, each with its
//! own complex gain. Correlation follows the call, as LRM 2.4 §4.6.4.6
//! requires: the injections of one process share their train, and two
//! processes are independent even when the module gives them the same name,
//! because a name only groups the contribution report.
//!
//! Three device families carry such processes, and this collector is the
//! single walk over all of them:
//!
//! * runtime-compiled Verilog-A devices whose artifact routes noise through
//!   grouped processes;
//! * build-time generated devices with grouped processes, whose endpoints are
//!   mapped onto circuit rows the same way `.NOISE` maps them;
//! * the analog half of every mixed Verilog-AMS host, which is the same
//!   runtime device and so carries the same processes. Its discrete inputs
//!   are part of the device's own state, so a clone observes them frozen.
//!
//! Runtime devices whose artifact predates grouped processes, and generated
//! devices without them, report scalar sources through the ordinary noise
//! catalog instead, and are not walked here.
//!
//! Every injection is a current between `(node_pos, node_neg)` under the
//! convention `.NOISE` uses, with a potential contribution injected at its
//! branch row; the process's gains already carry the instance multiplicity
//! and the contribution's sign.

use crate::engine::SimulationError;
use crate::{CircuitData, Complex64, Value};

/// Which device family a process was collected from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(in crate::engine) enum NoiseProcessRoute {
    /// A runtime-compiled Verilog-A device.
    #[cfg_attr(not(feature = "veriloga"), allow(dead_code))]
    Runtime,
    /// A build-time generated Verilog-A device.
    #[cfg_attr(not(feature = "veriloga-builtins-base"), allow(dead_code))]
    Generated,
    /// The analog half of a mixed Verilog-AMS host.
    #[cfg_attr(not(feature = "veriloga"), allow(dead_code))]
    MixedHost,
}

/// The spectral shape of one process's noise function.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    not(any(feature = "veriloga", feature = "veriloga-builtins-base")),
    allow(dead_code)
)]
pub(in crate::engine) enum NoiseProcessSpectrum {
    /// `white_noise(pwr)`: the density is `psd` at every frequency.
    White,
    /// `flicker_noise(pwr, ef)`: the density is `psd / f^ef`.
    Flicker { exponent: Value },
    /// `noise_table` / `noise_table_log`: the density is `psd` times the
    /// table interpolated at the frequency and clamped at its ends.
    Table {
        points: Vec<(Value, Value)>,
        log_interp: bool,
    },
}

impl NoiseProcessSpectrum {
    /// The noise-function family, as the stream-seed key spells it.
    pub(in crate::engine) fn kind_label(&self) -> &'static str {
        match self {
            Self::White => "white",
            Self::Flicker { .. } => "flicker",
            Self::Table { .. } => "table",
        }
    }
}

/// One circuit injection of a process.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(
    not(any(feature = "veriloga", feature = "veriloga-builtins-base")),
    allow(dead_code)
)]
pub(in crate::engine) struct NoiseProcessInjection {
    /// One-based solution row the unit current enters; zero is ground.
    pub(in crate::engine) node_pos: usize,
    /// One-based solution row the unit current leaves; zero is ground.
    pub(in crate::engine) node_neg: usize,
    /// Complex gain from the process's unit current to this injection.
    pub(in crate::engine) gain: Complex64,
}

/// One noise-function call of one instance, evaluated at a bias and a
/// frequency.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    not(any(feature = "veriloga", feature = "veriloga-builtins-base")),
    allow(dead_code)
)]
pub(in crate::engine) struct CollectedNoiseProcess {
    /// The deck's name for the instance, as elaborated.
    pub(in crate::engine) instance: String,
    /// The module the instance is built from.
    pub(in crate::engine) module: String,
    pub(in crate::engine) route: NoiseProcessRoute,
    /// The compiler's dense identity of the call within its module.
    pub(in crate::engine) process_id: usize,
    /// The call's label, canonicalized as a persisted mechanism name, or
    /// `NOISE<process_id>` when the call has none.
    pub(in crate::engine) mechanism: String,
    pub(in crate::engine) spectrum: NoiseProcessSpectrum,
    /// The noise function's power argument at this bias: the density for
    /// white noise, the density at 1 Hz for flicker, and the table's scale.
    pub(in crate::engine) psd: Value,
    pub(in crate::engine) injections: Vec<NoiseProcessInjection>,
    /// The operators in the instance's module that no finite-state
    /// realization represents (`absdelay`, `zi_*`, `idtmod`). A shaped
    /// process from such a module may pass through one of them.
    pub(in crate::engine) non_rational_operators: Vec<&'static str>,
}

/// The identity a process keeps across biases and frequencies.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(in crate::engine) struct NoiseProcessKey {
    pub(in crate::engine) route: NoiseProcessRoute,
    pub(in crate::engine) instance: String,
    pub(in crate::engine) process_id: usize,
}

impl CollectedNoiseProcess {
    pub(in crate::engine) fn key(&self) -> NoiseProcessKey {
        NoiseProcessKey {
            route: self.route,
            instance: self.instance.clone(),
            process_id: self.process_id,
        }
    }

    /// Instance and mechanism, as a diagnostic names the process.
    pub(in crate::engine) fn describe(&self) -> String {
        format!("{}:{}", self.instance, self.mechanism)
    }
}

/// Whether the circuit holds any device whose noise this collector walks,
/// active at the present bias or not.
///
/// Structural: no PSD is evaluated, so a process whose activation is off at
/// the operating point still counts. A transient-noise run uses it to keep
/// its sample grid when every process is dormant at `t = 0`.
pub(in crate::engine) fn has_structural_noise_processes(circuit: &CircuitData) -> bool {
    #[cfg(feature = "veriloga")]
    {
        if circuit
            .veriloga_devices()
            .iter()
            .any(|device| device.has_grouped_noise_processes())
        {
            return true;
        }
        if circuit
            .mixed_signal_hosts
            .iter()
            .any(|host| host_carries_processes(host.analog_device()))
        {
            return true;
        }
    }
    #[cfg(feature = "veriloga-builtins-base")]
    if circuit
        .generated_veriloga_devices()
        .iter()
        .any(|device| !device.grouped_noise_process_catalog().is_empty())
    {
        return true;
    }
    #[cfg(not(any(feature = "veriloga", feature = "veriloga-builtins-base")))]
    let _ = circuit;
    false
}

/// Every active noise process of the circuit at `solution`, with its gains
/// evaluated at `frequency`.
///
/// The order is the circuit's own device order — runtime devices, generated
/// devices, then mixed hosts — and each device lists its processes by
/// `process_id`. Inactive processes, and processes whose power is zero at
/// this bias, are absent; a consumer that tracks a process over time treats
/// an absent process as one that injects nothing.
pub(in crate::engine) fn collect_noise_processes(
    circuit: &CircuitData,
    solution: &[Value],
    frequency: Value,
) -> Result<Vec<CollectedNoiseProcess>, SimulationError> {
    let mut processes = Vec::new();
    #[cfg(feature = "veriloga")]
    for device in circuit.veriloga_devices().iter() {
        if device.has_grouped_noise_processes() {
            collect_runtime_device(
                device,
                device.name.as_str(),
                NoiseProcessRoute::Runtime,
                solution,
                frequency,
                &mut processes,
            )?;
        }
    }
    #[cfg(feature = "veriloga-builtins-base")]
    for device in circuit.generated_veriloga_devices().iter() {
        if device.has_grouped_noise_processes() {
            collect_generated_device(circuit, device, solution, frequency, &mut processes)?;
        }
    }
    #[cfg(feature = "veriloga")]
    for host in &circuit.mixed_signal_hosts {
        let device = host.analog_device();
        if host_carries_processes(device) {
            collect_runtime_device(
                device,
                host.instance_name(),
                NoiseProcessRoute::MixedHost,
                solution,
                frequency,
                &mut processes,
            )?;
        }
    }
    #[cfg(not(any(feature = "veriloga", feature = "veriloga-builtins-base")))]
    let _ = (circuit, solution, frequency, &mut processes);
    Ok(processes)
}

/// Whether a host's analog device has noise this collector must evaluate.
///
/// The ordinary noise catalog never walks hosts, so a host whose artifact
/// predates grouped processes is evaluated here too: the runtime presents
/// its scalar sources as single-injection processes.
#[cfg(feature = "veriloga")]
fn host_carries_processes(device: &rspice_veriloga::device::VerilogADevice) -> bool {
    device.has_grouped_noise_processes() || !device.uses_grouped_noise_processes()
}

/// The operators that leave a shaped process without a finite-state
/// realization.
#[cfg(feature = "veriloga")]
fn non_rational_shaping_operators(
    device: &rspice_veriloga::device::VerilogADevice,
) -> Vec<&'static str> {
    device
        .non_rational_analog_operators()
        .into_iter()
        .filter(|operator| matches!(*operator, "absdelay" | "zi" | "idtmod"))
        .collect()
}

#[cfg(feature = "veriloga")]
fn collect_runtime_device(
    device: &rspice_veriloga::device::VerilogADevice,
    instance: &str,
    route: NoiseProcessRoute,
    solution: &[Value],
    frequency: Value,
    processes: &mut Vec<CollectedNoiseProcess>,
) -> Result<(), SimulationError> {
    let mut probe = device.clone();
    probe.try_set_analysis_type(3).map_err(|err| {
        SimulationError::Circuit(format!(
            "Verilog-A device '{instance}' noise analysis setup failed: {err}"
        ))
    })?;
    let evaluated = probe
        .try_noise_processes_at_frequency(solution, frequency)
        .map_err(|err| {
            SimulationError::Circuit(format!(
                "Verilog-A device '{instance}' coherent noise evaluation failed at \
                 {frequency:e} Hz: {err}"
            ))
        })?;
    if evaluated.is_empty() {
        return Ok(());
    }
    let module = device.model_name().to_string();
    let non_rational_operators = non_rational_shaping_operators(device);
    for process in evaluated {
        let spectrum = match (process.table, process.exponent) {
            (Some((points, log_interp)), _) => NoiseProcessSpectrum::Table { points, log_interp },
            (None, Some(exponent)) => NoiseProcessSpectrum::Flicker { exponent },
            (None, None) => NoiseProcessSpectrum::White,
        };
        processes.push(CollectedNoiseProcess {
            instance: instance.to_string(),
            module: module.clone(),
            route,
            process_id: process.process_id,
            mechanism: canonical_mechanism(&process.name),
            spectrum,
            psd: process.psd,
            injections: process
                .injections
                .into_iter()
                .map(|injection| NoiseProcessInjection {
                    node_pos: injection.node_pos,
                    node_neg: injection.node_neg,
                    gain: Complex64::new(injection.gain.re, injection.gain.im),
                })
                .collect(),
            non_rational_operators: non_rational_operators.clone(),
        });
    }
    Ok(())
}

#[cfg(feature = "veriloga-builtins-base")]
fn collect_generated_device(
    circuit: &CircuitData,
    device: &crate::device::veriloga_builtins::BuiltinVerilogAInstance,
    solution: &[Value],
    frequency: Value,
    processes: &mut Vec<CollectedNoiseProcess>,
) -> Result<(), SimulationError> {
    let instance = device.instance_name.as_str();
    let evaluated = device
        .evaluate_noise_processes_at_frequency(
            solution,
            circuit.num_nodes(),
            frequency,
            circuit.generated_simulation_parameters,
        )
        .map_err(|err| {
            SimulationError::Circuit(format!(
                "Generated Verilog-A device '{instance}' coherent noise evaluation failed at \
                 {frequency:e} Hz: {err}"
            ))
        })?;
    for process in evaluated
        .into_iter()
        .filter(|process| process.active && !process.injections.is_empty())
    {
        processes.push(generated_process(circuit, device, process)?);
    }
    Ok(())
}

/// Translate one generated process onto circuit rows.
///
/// The same validation and endpoint mapping `.NOISE` applies, so a process
/// reaches the transient and the small-signal analyses on identical rows.
#[cfg(feature = "veriloga-builtins-base")]
fn generated_process(
    circuit: &CircuitData,
    device: &crate::device::veriloga_builtins::BuiltinVerilogAInstance,
    process: crate::device::veriloga_builtins::BuiltinEvaluatedNoiseProcess,
) -> Result<CollectedNoiseProcess, SimulationError> {
    use crate::device::veriloga_builtins::{GeneratedNoiseInjection, GeneratedNoiseKind};

    let instance = device.instance_name.as_str();
    let descriptor = process.descriptor;
    let mechanism = match descriptor.label {
        Some(label) => canonical_mechanism(label),
        None => format!("NOISE{}", descriptor.process_id),
    };
    let invalid = |detail: String| {
        SimulationError::Circuit(format!(
            "Generated Verilog-A device '{instance}' coherent noise process '{mechanism}' is \
             invalid: {detail}"
        ))
    };
    if !process.psd.is_finite() || process.psd < 0.0 {
        return Err(invalid(format!(
            "PSD must be finite and nonnegative, got {}",
            process.psd
        )));
    }
    let spectrum = match descriptor.kind {
        GeneratedNoiseKind::White => {
            if descriptor.table_len != 0
                || process.exponent.is_some()
                || !process.table_operands.is_empty()
            {
                return Err(invalid(
                    "white noise carried flicker or table metadata".to_string(),
                ));
            }
            NoiseProcessSpectrum::White
        }
        GeneratedNoiseKind::Flicker => {
            if descriptor.table_len != 0 || !process.table_operands.is_empty() {
                return Err(invalid("flicker noise carried table metadata".to_string()));
            }
            let exponent = process.exponent.ok_or_else(|| {
                invalid("active flicker noise has no frequency exponent".to_string())
            })?;
            NoiseProcessSpectrum::Flicker { exponent }
        }
        GeneratedNoiseKind::Table => {
            if process.exponent.is_some() {
                return Err(invalid(
                    "table noise carried a flicker exponent".to_string(),
                ));
            }
            if descriptor.table_len == 0
                || !descriptor.table_len.is_multiple_of(2)
                || process.table_operands.len() != descriptor.table_len
            {
                return Err(invalid(format!(
                    "table metadata declares {} operands and evaluated {}",
                    descriptor.table_len,
                    process.table_operands.len()
                )));
            }
            let mut points = Vec::with_capacity(descriptor.table_len / 2);
            for pair in process.table_operands.chunks_exact(2) {
                let (frequency, power) = (pair[0], pair[1]);
                let valid = if !frequency.is_finite() || !power.is_finite() {
                    false
                } else if descriptor.table_log_interp {
                    frequency > 0.0 && power > 0.0
                } else {
                    frequency >= 0.0 && power >= 0.0
                };
                if !valid {
                    return Err(invalid(format!(
                        "table point ({frequency}, {power}) violates {} interpolation \
                         requirements",
                        if descriptor.table_log_interp {
                            "positive log-log"
                        } else {
                            "nonnegative linear"
                        }
                    )));
                }
                points.push((frequency, power));
            }
            points.sort_by(|left, right| left.0.total_cmp(&right.0));
            if let Some(duplicate) = points
                .windows(2)
                .find(|adjacent| adjacent[0].0 == adjacent[1].0)
            {
                return Err(invalid(format!(
                    "table frequency points must be unique; duplicate frequency {}",
                    duplicate[0].0
                )));
            }
            NoiseProcessSpectrum::Table {
                points,
                log_interp: descriptor.table_log_interp,
            }
        }
    };
    let mut injections = Vec::with_capacity(process.injections.len());
    for injection in process.injections {
        let (node_pos, node_neg) = match injection.mapped.injection {
            GeneratedNoiseInjection::Current { node_pos, node_neg } => {
                if node_pos > circuit.num_nodes() || node_neg > circuit.num_nodes() {
                    return Err(invalid(format!(
                        "mapped current endpoints ({node_pos}, {node_neg}) exceed the {} \
                         circuit nodes",
                        circuit.num_nodes()
                    )));
                }
                (node_pos, node_neg)
            }
            GeneratedNoiseInjection::Potential { branch } => {
                if branch == 0 || branch > circuit.num_branches() {
                    return Err(invalid(format!(
                        "mapped potential branch {branch} is outside the {} circuit branches",
                        circuit.num_branches()
                    )));
                }
                (circuit.get_branch_matrix_index(branch), 0)
            }
        };
        let gain = Complex64::new(injection.gain.re, injection.gain.im);
        if !gain.re.is_finite() || !gain.im.is_finite() {
            return Err(invalid(format!(
                "injection gain ({}, {}) is non-finite",
                gain.re, gain.im
            )));
        }
        injections.push(NoiseProcessInjection {
            node_pos,
            node_neg,
            gain,
        });
    }
    Ok(CollectedNoiseProcess {
        instance: instance.to_string(),
        module: device.model_name.to_string(),
        route: NoiseProcessRoute::Generated,
        process_id: descriptor.process_id,
        mechanism,
        spectrum,
        psd: process.psd,
        injections,
        // Generated code is emitted from the same front end, and its shaped
        // processes are classified by their gains; it has no separately
        // reportable operator inventory.
        non_rational_operators: Vec::new(),
    })
}

/// Canonical mechanism name for a Verilog-A noise label.
///
/// The label is whatever string literal the module passed to the noise
/// function, so it may carry spaces and punctuation no saved result can be
/// written with. Every byte outside the persistable shape becomes `_` and the
/// rest is upper-cased, which is how the code generator composes the compiled
/// catalog's own mechanism names and how `.NOISE` names the contribution.
#[cfg(any(feature = "veriloga", feature = "veriloga-builtins-base"))]
fn canonical_mechanism(label: &str) -> String {
    let mut mechanism = label
        .bytes()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'#') {
                byte.to_ascii_uppercase() as char
            } else {
                '_'
            }
        })
        .collect::<String>();
    mechanism.truncate(crate::analysis::NOISE_MECHANISM_MAX_BYTES);
    if mechanism.is_empty() {
        mechanism.push_str("NOISE");
    }
    mechanism
}
