//! Periodic noise (pnoise) analysis: cyclostationary noise folding around a
//! harmonic-balance operating point.
//!
//! For every sweep offset one adjoint conversion-matrix solve yields the
//! transfer from a unit current at every (node, sideband) to the output at
//! the analysis frequency; each noise source then contributes through the
//! full sideband correlation of its periodically modulated intensity. This
//! captures what the stationary approximation cannot: noise transferred
//! through the LO-modulated small-signal parameters (switching mixers,
//! choppers, samplers), and shot noise that switches on and off with its
//! bias current.

use super::*;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::HbSolverState;
use crate::analysis::harmonic_balance::{HbConfig, PeriodicAcExcitation, PeriodicNoiseSource};

/// Result of periodic noise analysis.
#[derive(Debug, Clone)]
pub struct PnoiseAnalysisResult {
    /// Offset frequencies (Hz), the output analysis frequencies.
    pub frequencies: Vec<Value>,
    /// Total output noise voltage PSD at each offset (V^2/Hz).
    pub output_noise: Vec<Value>,
    /// Per-source contributions: `(label, psd per offset)`, summing to the
    /// total at every offset.
    pub contributors: Vec<(String, Vec<Value>)>,
    /// Input-referred noise (V^2/Hz): output noise divided by the squared
    /// magnitude of the conversion transfer from the input source (at its
    /// own frequency, sideband 0) to the output. Present when an input
    /// source was named.
    pub input_noise: Option<Vec<Value>>,
    /// Large-signal fundamental (Hz).
    pub fundamental_freq: Value,
    /// Whether the operating-point solve converged.
    pub converged: bool,
}

enum PnoiseOperatingPoint<'a> {
    Shooting(&'a super::super::PssOperatingPoint),
    HarmonicBalance(&'a HbOperatingPoint),
}

#[derive(Clone, Copy)]
struct ScaledPositive {
    mantissa: Value,
    exponent: i32,
}

fn checked_scaled_positive_product(
    factors: &[Value],
    quantity: &str,
) -> Result<ScaledPositive, SimulationError> {
    let mut mantissa = 1.0;
    let mut exponent = 0i32;
    for &factor in factors {
        if !factor.is_finite() || factor < 0.0 {
            return Err(SimulationError::Circuit(format!(
                "{quantity} contains an invalid factor {factor}"
            )));
        }
        if factor == 0.0 {
            return Ok(ScaledPositive {
                mantissa: 0.0,
                exponent: 0,
            });
        }
        let factor_exponent = libm::ilogb(factor);
        let factor_mantissa = libm::scalbn(factor, -factor_exponent);
        mantissa *= factor_mantissa;
        let mantissa_exponent = libm::ilogb(mantissa);
        mantissa = libm::scalbn(mantissa, -mantissa_exponent);
        exponent = exponent
            .checked_add(factor_exponent)
            .and_then(|value| value.checked_add(mantissa_exponent))
            .ok_or_else(|| {
                SimulationError::Circuit(format!("{quantity} exponent exceeds this platform"))
            })?;
    }
    if !mantissa.is_finite() || mantissa <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "{quantity} normalized mantissa is invalid ({mantissa})"
        )));
    }
    Ok(ScaledPositive { mantissa, exponent })
}

fn checked_scaled_positive_power_product(
    coefficient: Value,
    base: Value,
    power: Value,
    quantity: &str,
) -> Result<ScaledPositive, SimulationError> {
    if !coefficient.is_finite()
        || coefficient < 0.0
        || !base.is_finite()
        || base < 0.0
        || !power.is_finite()
    {
        return Err(SimulationError::Circuit(format!(
            "{quantity} has invalid coefficient/base/power ({coefficient}, {base}, {power})"
        )));
    }
    if coefficient == 0.0 {
        return Ok(ScaledPositive {
            mantissa: 0.0,
            exponent: 0,
        });
    }
    if base == 0.0 {
        return if power > 0.0 {
            Ok(ScaledPositive {
                mantissa: 0.0,
                exponent: 0,
            })
        } else if power == 0.0 {
            checked_scaled_positive_product(&[coefficient], quantity)
        } else {
            Err(SimulationError::Circuit(format!(
                "{quantity} is singular for a zero base and negative power"
            )))
        };
    }

    let log2_value = libm::log2(coefficient) + power * libm::log2(base);
    if !log2_value.is_finite() {
        return Err(SimulationError::Circuit(format!(
            "{quantity} exponent is non-finite"
        )));
    }
    let binary_exponent = libm::floor(log2_value);
    if binary_exponent < i32::MIN as Value || binary_exponent > i32::MAX as Value {
        return Err(SimulationError::Circuit(format!(
            "{quantity} exponent exceeds this platform"
        )));
    }
    let mantissa = libm::exp2(log2_value - binary_exponent);
    if !mantissa.is_finite() || mantissa <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "{quantity} normalized mantissa is invalid ({mantissa})"
        )));
    }
    Ok(ScaledPositive {
        mantissa,
        exponent: binary_exponent as i32,
    })
}

fn checked_pnoise_total(
    per_source: &[Value],
    sources: &[PeriodicNoiseSource],
    offset: Value,
) -> Result<Value, SimulationError> {
    if per_source.len() != sources.len() {
        return Err(SimulationError::Circuit(format!(
            "pnoise solver returned {} contributions for {} sources at offset {offset:.6e} Hz",
            per_source.len(),
            sources.len()
        )));
    }

    // Neumaier compensation preserves contributors far below the largest
    // source without reordering the public contributor list.
    let mut sum = 0.0;
    let mut compensation = 0.0;
    for (source, &value) in sources.iter().zip(per_source) {
        if !value.is_finite() || value < 0.0 {
            return Err(SimulationError::Circuit(format!(
                "pnoise source '{}' produced invalid output-noise density {value} at offset {offset:.6e} Hz",
                source.name
            )));
        }
        let next = sum + value;
        if !next.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "pnoise total output-noise density overflowed at offset {offset:.6e} Hz"
            )));
        }
        let correction = if sum.abs() >= value.abs() {
            (sum - next) + value
        } else {
            (value - next) + sum
        };
        compensation += correction;
        if !compensation.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "pnoise output-noise accumulation became non-finite at offset {offset:.6e} Hz"
            )));
        }
        sum = next;
    }

    let total = sum + compensation;
    if !total.is_finite() || total < 0.0 {
        return Err(SimulationError::Circuit(format!(
            "pnoise total output-noise density is invalid ({total}) at offset {offset:.6e} Hz"
        )));
    }
    Ok(total)
}

fn checked_input_referred_pnoise(
    output_noise: Value,
    transfer: Complex64,
    offset: Value,
) -> Result<Value, SimulationError> {
    if !output_noise.is_finite() || output_noise < 0.0 {
        return Err(SimulationError::Circuit(format!(
            "pnoise output-noise density is invalid ({output_noise}) before input referral at offset {offset:.6e} Hz"
        )));
    }
    if !transfer.re.is_finite() || !transfer.im.is_finite() {
        return Err(SimulationError::Circuit(format!(
            "pnoise input transfer is non-finite at offset {offset:.6e} Hz"
        )));
    }
    let transfer_scale = transfer.re.abs().max(transfer.im.abs());
    if transfer_scale == 0.0 {
        return Err(SimulationError::Circuit(format!(
            "pnoise input-referred density is undefined at the zero input-transfer magnitude at offset {offset:.6e} Hz"
        )));
    }
    if output_noise == 0.0 {
        return Ok(0.0);
    }

    // Form output_noise / |H|^2 in normalized binary parts. Direct norm_sqr
    // can overflow for a finite H or underflow before the division, and two
    // sequential divisions can round a representable subnormal to zero.
    let numerator_exponent = libm::ilogb(output_noise);
    let numerator_mantissa = libm::scalbn(output_noise, -numerator_exponent);
    let transfer_exponent = libm::ilogb(transfer_scale);
    let scaled_real = libm::scalbn(transfer.re, -transfer_exponent);
    let scaled_imag = libm::scalbn(transfer.im, -transfer_exponent);
    let squared_mantissa = scaled_real * scaled_real + scaled_imag * scaled_imag;
    if !squared_mantissa.is_finite() || squared_mantissa <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "pnoise normalized input-transfer magnitude is invalid at offset {offset:.6e} Hz"
        )));
    }
    let mantissa_exponent = libm::ilogb(squared_mantissa);
    let denominator_mantissa = libm::scalbn(squared_mantissa, -mantissa_exponent);
    let denominator_exponent = transfer_exponent
        .checked_mul(2)
        .and_then(|value| value.checked_add(mantissa_exponent))
        .ok_or_else(|| {
            SimulationError::Circuit(format!(
                "pnoise input-transfer exponent exceeds this platform at offset {offset:.6e} Hz"
            ))
        })?;
    let exponent = numerator_exponent
        .checked_sub(denominator_exponent)
        .ok_or_else(|| {
            SimulationError::Circuit(format!(
                "pnoise input-referred-noise exponent exceeds this platform at offset {offset:.6e} Hz"
            ))
        })?;
    let mantissa = numerator_mantissa / denominator_mantissa;
    let input_noise = libm::scalbn(mantissa, exponent);
    if !input_noise.is_finite() || input_noise <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "pnoise input-referred density is not representable ({input_noise}) at offset {offset:.6e} Hz"
        )));
    }
    Ok(input_noise)
}

impl Engine {
    /// Run periodic noise analysis at `output_node` (optionally referenced
    /// to `output_ref` for a differential output) over `offsets`.
    ///
    /// `fundamental_freq` is the large-signal periodicity; `max_sideband`
    /// bounds the folding range (sidebands -K..=K participate). Resistor
    /// thermal noise is stationary; junction shot noise and FET channel
    /// thermal noise are modulated by the periodic operating point.
    #[allow(clippy::too_many_arguments)]
    pub fn run_pnoise(
        &self,
        netlist: &Netlist,
        fundamental_freq: Value,
        offsets: &[Value],
        output_node: &str,
        output_ref: Option<&str>,
        input_source: Option<&str>,
        max_sideband: i32,
    ) -> Result<PnoiseAnalysisResult, SimulationError> {
        self.run_pnoise_with_abort(
            netlist,
            fundamental_freq,
            offsets,
            output_node,
            output_ref,
            input_source,
            max_sideband,
            &NoAbort,
        )
    }

    /// Run driven periodic noise with cooperative cancellation.
    #[allow(clippy::too_many_arguments)]
    pub fn run_pnoise_with_abort(
        &self,
        netlist: &Netlist,
        fundamental_freq: Value,
        offsets: &[Value],
        output_node: &str,
        output_ref: Option<&str>,
        input_source: Option<&str>,
        max_sideband: i32,
        abort: &dyn AbortSignal,
    ) -> Result<PnoiseAnalysisResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let engine = self.resolved_for_netlist(netlist);
        engine.run_pnoise_impl(
            netlist,
            fundamental_freq,
            offsets,
            output_node,
            output_ref,
            input_source,
            max_sideband,
            None,
            abort,
        )
    }

    /// Run driven periodic noise from an exact retained shooting-PSS orbit.
    /// No periodic operating-point solve is performed in this path.
    #[allow(clippy::too_many_arguments)]
    pub fn run_pnoise_from_pss_with_abort(
        &self,
        netlist: &Netlist,
        offsets: &[Value],
        output_node: &str,
        output_ref: Option<&str>,
        input_source: Option<&str>,
        max_sideband: i32,
        operating_point: &super::super::PssOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<PnoiseAnalysisResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if operating_point.config().is_autonomous() {
            return Err(SimulationError::Circuit(
                "driven pnoise cannot consume an autonomous PSS operating point; use oscillator pnoise"
                    .to_string(),
            ));
        }
        let engine = self.resolved_for_netlist(netlist);
        engine.run_pnoise_impl(
            netlist,
            operating_point.analysis().result.frequency,
            offsets,
            output_node,
            output_ref,
            input_source,
            max_sideband,
            Some(PnoiseOperatingPoint::Shooting(operating_point)),
            abort,
        )
    }

    /// Run driven periodic noise from an exact retained harmonic-balance
    /// operating point. The frozen spectral state is consumed directly and
    /// the large-signal operating point is never re-solved.
    #[allow(clippy::too_many_arguments)]
    pub fn run_pnoise_from_hb_with_abort(
        &self,
        netlist: &Netlist,
        offsets: &[Value],
        output_node: &str,
        output_ref: Option<&str>,
        input_source: Option<&str>,
        max_sideband: i32,
        operating_point: &HbOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<PnoiseAnalysisResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let engine = self.resolved_for_netlist(netlist);
        engine.run_pnoise_impl(
            netlist,
            operating_point.config().fundamental_freq,
            offsets,
            output_node,
            output_ref,
            input_source,
            max_sideband,
            Some(PnoiseOperatingPoint::HarmonicBalance(operating_point)),
            abort,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn run_pnoise_impl(
        &self,
        netlist: &Netlist,
        fundamental_freq: Value,
        offsets: &[Value],
        output_node: &str,
        output_ref: Option<&str>,
        input_source: Option<&str>,
        max_sideband: i32,
        operating_point: Option<PnoiseOperatingPoint<'_>>,
        abort: &dyn AbortSignal,
    ) -> Result<PnoiseAnalysisResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if !fundamental_freq.is_finite() || fundamental_freq <= 0.0 {
            return Err(SimulationError::Circuit(
                "pnoise requires a positive fundamental frequency".to_string(),
            ));
        }
        if offsets.is_empty() {
            return Err(SimulationError::Circuit(
                "pnoise frequency sweep is empty".to_string(),
            ));
        }
        if let Some((index, offset)) = offsets
            .iter()
            .copied()
            .enumerate()
            .find(|(_, offset)| !offset.is_finite() || *offset < 0.0)
        {
            return Err(SimulationError::Circuit(format!(
                "pnoise offset frequencies must be finite and non-negative, got offsets[{index}]={offset}"
            )));
        }
        if max_sideband < 0 {
            return Err(SimulationError::Circuit(
                "pnoise max_sideband must be non-negative".to_string(),
            ));
        }
        self.ensure_analysis_points(offsets.len())?;
        let sideband_count = (max_sideband as usize).saturating_mul(2).saturating_add(1);
        self.ensure_analysis_points(sideband_count)?;

        let span = (max_sideband as usize).saturating_mul(2);
        let op_harmonics = span.max(8);
        if let Some(operating_point) = &operating_point
            && op_harmonics
                > match operating_point {
                    PnoiseOperatingPoint::Shooting(point) => point.spectral_harmonic_capacity(),
                    PnoiseOperatingPoint::HarmonicBalance(point) => {
                        point.spectral_harmonic_capacity()
                    }
                }
        {
            let capacity = match operating_point {
                PnoiseOperatingPoint::Shooting(point) => point.spectral_harmonic_capacity(),
                PnoiseOperatingPoint::HarmonicBalance(point) => point.spectral_harmonic_capacity(),
            };
            return Err(SimulationError::Circuit(format!(
                "pnoise requires {op_harmonics} periodic harmonics for its sideband span, but the retained periodic state has capacity {capacity}"
            )));
        }
        let hb_config = match &operating_point {
            Some(PnoiseOperatingPoint::HarmonicBalance(point)) => point.config().clone(),
            _ => HbConfig::new(fundamental_freq)
                .with_harmonics(op_harmonics)
                .with_oversample(4),
        };
        let hb_config = self.hb_config_for_netlist(netlist, hb_config)?;
        self.hb_validate_config(&hb_config)?;
        if let Some(PnoiseOperatingPoint::HarmonicBalance(point)) = &operating_point {
            point.authenticate_for_reuse(netlist, &self.config, &hb_config)?;
        }

        let circuit = self.build_circuit_with_abort(netlist, abort)?;
        let num_nodes = circuit.num_nodes();
        if num_nodes == 0 {
            return Err(SimulationError::Circuit("Circuit has no nodes".to_string()));
        }
        let periodic_unknowns = num_nodes
            .checked_add(circuit.voltage_sources.len())
            .and_then(|count| count.checked_add(circuit.inductors.len()))
            .ok_or_else(|| {
                SimulationError::Circuit(
                    "pnoise periodic node and branch count overflows this platform".to_string(),
                )
            })?;
        let lifted_unknowns = periodic_unknowns.checked_mul(sideband_count).ok_or_else(|| {
            SimulationError::Circuit(format!(
                "pnoise lifted dimension {periodic_unknowns} MNA unknowns x {sideband_count} sidebands overflows this platform"
            ))
        })?;
        self.ensure_matrix_unknowns(lifted_unknowns)?;
        if let Some(summary) = Self::hb_unsupported_nonlinear_device_summary(&circuit, num_nodes) {
            return Err(HbError::UnsupportedNonlinearDevices(summary).into());
        }
        if let Some(summary) = Self::hb_periodic_mna_unsupported_summary(&circuit) {
            return Err(SimulationError::Circuit(format!(
                "pnoise exact periodic MNA is unavailable because the circuit contains {summary}"
            )));
        }

        self.ensure_result_shape(op_harmonics.saturating_add(1), num_nodes.saturating_mul(2))?;
        let drive_tones = Self::hb_collect_drive_tones(&hb_config)?;

        let mut solver = HbSolver::try_new(hb_config.clone(), num_nodes).map_err(|error| {
            SimulationError::Circuit(format!("pnoise solver construction failed: {error}"))
        })?;
        let node_names = self.hb_build_node_names(&circuit, num_nodes);
        solver.set_node_names(node_names.clone());

        // One exact canonical V/L MNA registry owns both the periodic
        // operating point and every subsequent adjoint/forward linearization.
        // Register authored voltage-source spectra first so the canonical
        // descriptors carry the same large-signal constraints Newton solves.
        self.hb_stamp_resistors(&circuit, &mut solver);
        self.hb_stamp_capacitors(&circuit, &mut solver);
        self.hb_stamp_voltage_sources(&circuit, &mut solver, &hb_config, &drive_tones)?;
        self.hb_stamp_periodic_mna_branches(&circuit, &mut solver)?;
        self.hb_stamp_current_sources(&circuit, &mut solver, &hb_config, &drive_tones)?;

        let has_nonlinear = Self::hb_has_supported_nonlinear_devices(&circuit, num_nodes);
        if has_nonlinear {
            self.hb_stamp_supported_nonlinear_devices(&circuit, &mut solver, num_nodes);
        }
        let branch_names = solver.try_periodic_mna_branch_names().map_err(|error| {
            SimulationError::Circuit(format!(
                "pnoise branch metadata construction failed: {error}"
            ))
        })?;

        if let Some(PnoiseOperatingPoint::HarmonicBalance(point)) = &operating_point {
            point.authenticate_for_reuse(netlist, &self.config, &hb_config)?;
        }

        let solve_operating_point = operating_point.is_none();
        let mut state = if let Some(operating_point) = operating_point {
            match operating_point {
                PnoiseOperatingPoint::Shooting(point) => {
                    self.hb_state_from_pss_operating_point(point, &hb_config, &node_names)?
                }
                PnoiseOperatingPoint::HarmonicBalance(point) => {
                    point.to_solver_state(&node_names, &branch_names)?
                }
            }
        } else {
            HbSolverState::new(num_nodes, op_harmonics)
        };
        state
            .try_prepare_mna_branches(branch_names.len(), hb_config.num_harmonics)
            .map_err(|error| {
                SimulationError::Circuit(format!(
                    "pnoise operating-point MNA state construction failed: {error}"
                ))
            })?;
        if solve_operating_point {
            if has_nonlinear {
                solver
                    .solve_newton_with_abort(&mut state, abort)
                    .map_err(|e| match e {
                        crate::analysis::HbError::Aborted => SimulationError::Aborted,
                        _ => SimulationError::Circuit(format!(
                            "pnoise operating-point solve failed: {e}"
                        )),
                    })?;
            } else {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                solver.solve_linear(&mut state).map_err(|e| {
                    SimulationError::Circuit(format!("pnoise operating-point solve failed: {e}"))
                })?;
            }
        }

        let out_idx = node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(output_node.trim()))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "pnoise output node '{output_node}' not found in circuit nodes"
                ))
            })?;
        let ref_idx = output_ref
            .map(|name| {
                node_names
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case(name.trim()))
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "pnoise output reference node '{name}' not found in circuit nodes"
                        ))
                    })
            })
            .transpose()?;

        let temperature = self.config.temperature;
        use crate::constants::K_BOLTZMANN;

        // Stationary resistor thermal sources: 4kT*G between the resistor
        // terminals (DC-only intensity spectrum).
        let hb_dc_voltage = |row: usize| -> Value {
            if row == 0 {
                0.0
            } else {
                state
                    .x
                    .get(row - 1)
                    .and_then(|s| s.first())
                    .map(|c| c.re)
                    .unwrap_or(0.0)
            }
        };

        let mut sources: Vec<PeriodicNoiseSource> = Vec::new();
        for i in 0..circuit.resistors.len() {
            if !circuit.resistors.noisy.get(i).copied().unwrap_or(true) {
                continue;
            }
            let g = circuit.resistors.conductances[i];
            if !(g.is_finite() && g > 0.0) {
                continue;
            }
            let np = circuit.resistors.stamps[i].pp.row;
            let nn = circuit.resistors.stamps[i].nn.row;
            let name = circuit
                .resistors
                .names
                .get(i)
                .cloned()
                .unwrap_or_else(|| format!("R#{i}"));
            let source_temperature = circuit.resistor_noise_temperature(i, temperature);
            if !source_temperature.is_finite() || source_temperature <= 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "pnoise resistor '{name}' absolute noise temperature must be finite and positive, got {source_temperature} K"
                )));
            }
            let thermal_density = checked_scaled_positive_product(
                &[4.0, K_BOLTZMANN, source_temperature, g],
                &format!("pnoise resistor '{name}' thermal-noise density"),
            )?;
            sources.push(PeriodicNoiseSource {
                name: format!("{name} thermal"),
                node_pos: Self::hb_node_to_solver_index(np, num_nodes),
                node_neg: Self::hb_node_to_solver_index(nn, num_nodes),
                psd: vec![Complex64::new(thermal_density.mantissa, 0.0)],
                binary_scale_exponent: thermal_density.exponent,
                flicker: None,
            });

            // Model-card resistor flicker rides on the DC bias current,
            // matching the stationary .noise treatment.
            if let Some(&Some((coefficient, af, ef))) = circuit.resistors.flicker.get(i) {
                let i_dc = g * (hb_dc_voltage(np) - hb_dc_voltage(nn));
                if !i_dc.is_finite() {
                    return Err(SimulationError::Circuit(format!(
                        "pnoise resistor '{name}' has a non-finite DC current for flicker noise"
                    )));
                }
                let flicker_density = checked_scaled_positive_power_product(
                    coefficient,
                    i_dc.abs(),
                    af,
                    &format!("pnoise resistor '{name}' flicker coefficient"),
                )?;
                if flicker_density.mantissa > 0.0 {
                    sources.push(PeriodicNoiseSource {
                        name: format!("{name} flicker"),
                        node_pos: Self::hb_node_to_solver_index(np, num_nodes),
                        node_neg: Self::hb_node_to_solver_index(nn, num_nodes),
                        psd: vec![Complex64::new(0.0, 0.0)],
                        binary_scale_exponent: flicker_density.exponent,
                        flicker: Some((flicker_density.mantissa, ef)),
                    });
                }
            }
        }

        // Diode flicker (KF * |Id|^AF / f) at the periodic-average bias.
        for diode in &circuit.diodes.devices {
            if diode.kf > 0.0 {
                let va = hb_dc_voltage(diode.node_anode);
                let vc = hb_dc_voltage(diode.node_cathode);
                let arg = ((va - vc) / (diode.n * diode.vt)).min(40.0);
                let i_dc = diode.is * (arg.exp() - 1.0);
                if !i_dc.is_finite() {
                    return Err(SimulationError::Circuit(format!(
                        "pnoise diode '{}' has a non-finite DC current for flicker noise",
                        diode.name
                    )));
                }
                let flicker_density = checked_scaled_positive_power_product(
                    diode.kf,
                    i_dc.abs(),
                    diode.af,
                    &format!("pnoise diode '{}' flicker coefficient", diode.name),
                )?;
                if flicker_density.mantissa > 0.0 {
                    sources.push(PeriodicNoiseSource {
                        name: format!("{} flicker", diode.name),
                        node_pos: Self::hb_node_to_solver_index(diode.node_anode, num_nodes),
                        node_neg: Self::hb_node_to_solver_index(diode.node_cathode, num_nodes),
                        psd: vec![Complex64::new(0.0, 0.0)],
                        binary_scale_exponent: flicker_density.exponent,
                        flicker: Some((flicker_density.mantissa, 1.0)),
                    });
                }
            }
        }

        // Cyclostationary device sources from the converged waveforms.
        sources.extend(
            solver
                .device_noise_sources(&state, temperature)
                .map_err(|error| {
                    SimulationError::Circuit(format!(
                        "pnoise nonlinear-device source construction failed: {error}"
                    ))
                })?,
        );
        let values_per_point = sources
            .len()
            .checked_add(2)
            .and_then(|count| count.checked_add(usize::from(input_source.is_some())))
            .ok_or_else(|| {
                SimulationError::Circuit(
                    "pnoise retained result width overflows this platform".to_string(),
                )
            })?;
        self.ensure_result_shape(offsets.len(), values_per_point)?;

        // Input transfer for input-referred noise: the conversion transfer
        // from the named source (unit excitation at sideband 0) to the
        // output at the analysis frequency.
        let input_port = input_source
            .map(|name| Self::pac_input_port(&circuit, name, num_nodes))
            .transpose()?;
        let input_excitation = input_port.as_ref().map(|port| PeriodicAcExcitation {
            sideband: 0,
            injections: port.node_injections.clone(),
        });
        let input_branch_voltage = input_port
            .as_ref()
            .and_then(|port| port.voltage_source_index)
            .map(|source_index| {
                solver
                    .periodic_voltage_source_branch(source_index)
                    .map(|branch| (branch, Complex64::new(1.0, 0.0)))
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "pnoise input voltage source '{}' has no periodic MNA branch",
                            input_source.unwrap_or("")
                        ))
                    })
            })
            .transpose()?;
        let input_branch_voltage_column = input_branch_voltage.as_ref().map(std::slice::from_ref);
        let input_branch_voltages: &[&[(usize, Complex64)]] =
            input_branch_voltage_column.as_slice();

        let mut result_frequencies = Vec::new();
        result_frequencies
            .try_reserve_exact(offsets.len())
            .map_err(|error| {
                SimulationError::Circuit(format!(
                    "pnoise frequency-result allocation failed: {error}"
                ))
            })?;
        result_frequencies.extend_from_slice(offsets);

        let mut output_noise = Vec::new();
        output_noise
            .try_reserve_exact(offsets.len())
            .map_err(|error| {
                SimulationError::Circuit(format!("pnoise output-result allocation failed: {error}"))
            })?;
        let mut input_noise = if input_excitation.is_some() {
            let mut values = Vec::new();
            values.try_reserve_exact(offsets.len()).map_err(|error| {
                SimulationError::Circuit(format!("pnoise input-result allocation failed: {error}"))
            })?;
            Some(values)
        } else {
            None
        };
        let mut contributors: Vec<(String, Vec<Value>)> = Vec::new();
        contributors
            .try_reserve_exact(sources.len())
            .map_err(|error| {
                SimulationError::Circuit(format!(
                    "pnoise contributor-result allocation failed: {error}"
                ))
            })?;
        for source in &sources {
            let mut name = String::new();
            name.try_reserve_exact(source.name.len()).map_err(|error| {
                SimulationError::Circuit(format!(
                    "pnoise contributor-name allocation failed: {error}"
                ))
            })?;
            name.push_str(&source.name);
            let mut values = Vec::new();
            values.try_reserve_exact(offsets.len()).map_err(|error| {
                SimulationError::Circuit(format!(
                    "pnoise contributor-value allocation failed for '{}': {error}",
                    source.name
                ))
            })?;
            contributors.push((name, values));
        }
        for &offset in offsets {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let per_source = solver
                .solve_periodic_noise(
                    &state,
                    offset,
                    -max_sideband,
                    max_sideband,
                    out_idx,
                    ref_idx,
                    &sources,
                )
                .map_err(|e| {
                    SimulationError::Circuit(format!(
                        "pnoise solve failed at offset {offset:.6e} Hz: {e}"
                    ))
                })?;
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let total = checked_pnoise_total(&per_source, &sources, offset)?;
            output_noise.push(total);
            for (slot, &value) in contributors.iter_mut().zip(&per_source) {
                slot.1.push(value);
            }

            if let (Some(excitation), Some(acc)) = (input_excitation.as_ref(), input_noise.as_mut())
            {
                let response = solver
                    .solve_periodic_ac_with_branch_voltages(
                        &state,
                        offset,
                        -max_sideband,
                        max_sideband,
                        std::slice::from_ref(excitation),
                        input_branch_voltages,
                    )
                    .map_err(|e| {
                        SimulationError::Circuit(format!(
                            "pnoise input transfer failed at offset {offset:.6e} Hz: {e}"
                        ))
                    })?;
                let zero_idx = max_sideband as usize; // k = 0 with range -K..K
                let response_for_excitation = response.first().ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "pnoise input transfer returned no excitation response at offset {offset:.6e} Hz"
                    ))
                })?;
                let mut h = response_for_excitation
                    .get(out_idx)
                    .and_then(|sidebands| sidebands.get(zero_idx))
                    .copied()
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "pnoise input transfer returned an incomplete output response at offset {offset:.6e} Hz"
                        ))
                    })?;
                if let Some(r) = ref_idx {
                    h -= response_for_excitation
                        .get(r)
                        .and_then(|sidebands| sidebands.get(zero_idx))
                        .copied()
                        .ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                "pnoise input transfer returned an incomplete reference response at offset {offset:.6e} Hz"
                            ))
                        })?;
                }
                acc.push(checked_input_referred_pnoise(total, h, offset)?);
            }
        }

        if output_noise.len() != offsets.len()
            || contributors
                .iter()
                .any(|(_, values)| values.len() != offsets.len())
            || input_noise
                .as_ref()
                .is_some_and(|values| values.len() != offsets.len())
            || input_source.is_some() != input_noise.is_some()
        {
            return Err(SimulationError::Circuit(
                "pnoise result publication is incomplete".to_string(),
            ));
        }

        Ok(PnoiseAnalysisResult {
            frequencies: result_frequencies,
            output_noise,
            contributors,
            input_noise,
            fundamental_freq,
            converged: state.converged,
        })
    }
}

#[cfg(test)]
mod publication_tests {
    use super::*;

    #[test]
    fn input_referral_is_scale_safe_and_rejects_a_transfer_null() {
        let large = checked_input_referred_pnoise(1.0e-200, Complex64::new(1.0e-200, 0.0), 1.0e3)
            .expect("small nonzero gain has a representable input-referred result");
        assert!((large - 1.0e200).abs() <= 4.0 * Value::EPSILON * 1.0e200);

        let small = checked_input_referred_pnoise(1.0e200, Complex64::new(1.0e200, 0.0), 1.0e3)
            .expect("large finite gain has a representable input-referred result");
        assert!((small - 1.0e-200).abs() <= 4.0 * Value::EPSILON * 1.0e-200);

        let maximum_components = checked_input_referred_pnoise(
            Value::MAX,
            Complex64::new(Value::MAX, Value::MAX),
            1.0e3,
        )
        .expect("finite gain components need not have a materialized finite magnitude");
        let expected = 0.5 / Value::MAX;
        assert_eq!(maximum_components.to_bits(), expected.to_bits());

        let null = checked_input_referred_pnoise(1.0, Complex64::new(0.0, 0.0), 1.0e3)
            .expect_err("input referral is undefined at an exact transfer null");
        assert!(null.to_string().contains("zero input-transfer"));
    }

    #[test]
    fn contributor_publication_rejects_shape_and_range_failures() {
        let source = |name: &str| PeriodicNoiseSource {
            name: name.to_string(),
            node_pos: 0,
            node_neg: usize::MAX,
            psd: vec![Complex64::new(0.0, 0.0)],
            binary_scale_exponent: 0,
            flicker: None,
        };

        let mismatch = checked_pnoise_total(&[], &[source("one")], 1.0e3)
            .expect_err("a truncated solver result must fail publication");
        assert!(mismatch.to_string().contains("1 sources"));

        let overflow = checked_pnoise_total(
            &[Value::MAX, Value::MAX],
            &[source("one"), source("two")],
            1.0e3,
        )
        .expect_err("an unrepresentable total must fail publication");
        assert!(overflow.to_string().contains("overflowed"));

        let minimum = Value::from_bits(1);
        let subnormal =
            checked_pnoise_total(&[minimum, minimum], &[source("one"), source("two")], 1.0e3)
                .expect("a representable subnormal total must remain published");
        assert_eq!(subnormal, Value::from_bits(2));
    }

    #[test]
    fn source_power_products_preserve_out_of_range_flicker_coefficients() {
        let scaled = checked_scaled_positive_power_product(
            1.0e-200,
            1.0e-200,
            2.0,
            "test flicker coefficient",
        )
        .expect("an out-of-range elementary coefficient remains scaled");
        assert!(scaled.mantissa.is_finite() && scaled.mantissa > 0.0);
        assert!(scaled.exponent < -1074);

        let representable = checked_scaled_positive_power_product(
            1.0e200,
            1.0e-200,
            2.0,
            "test flicker coefficient",
        )
        .expect("a representable power product remains accurate");
        let value = libm::scalbn(representable.mantissa, representable.exponent);
        assert!((value - 1.0e-200).abs() <= 2.0e-12 * 1.0e-200);
    }
}
