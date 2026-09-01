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

fn pnoise_physical_constants(
    dialect: crate::engine::SpiceDialect,
) -> crate::analysis::noise::NoisePhysicalConstants {
    match dialect {
        crate::engine::SpiceDialect::Xyce => {
            crate::analysis::noise::NoisePhysicalConstants::XYCE_7_10
        }
        crate::engine::SpiceDialect::BestAvailable | crate::engine::SpiceDialect::Ngspice => {
            crate::analysis::noise::NoisePhysicalConstants::MODERN
        }
    }
}

/// Identify authored colored-noise controls whose intensity depends on the
/// periodically varying device current. The low-level periodic solver has an
/// exact stationary colored-source contract, but these device controls need
/// cyclostationary colored correlation rather than a DC-bias substitution.
fn unsupported_device_colored_noise(circuit: &CircuitData) -> Vec<String> {
    let mut unsupported = Vec::new();
    for (index, flicker) in circuit.resistors.flicker.iter().enumerate() {
        if !circuit.resistors.noisy[index] {
            continue;
        }
        if let Some((coefficient, af, ef)) = flicker
            && *coefficient != 0.0
        {
            let name = circuit
                .resistors
                .names
                .get(index)
                .map(String::as_str)
                .unwrap_or("<unnamed resistor>");
            unsupported.push(format!(
                "resistor '{name}' flicker noise (coefficient={coefficient}, AF={af}, EF={ef})"
            ));
        }
    }
    for (index, flicker) in circuit.resistor_branches.flicker.iter().enumerate() {
        if !circuit.resistor_branches.noisy[index] {
            continue;
        }
        if let Some((coefficient, af, ef)) = flicker
            && *coefficient != 0.0
        {
            let name = circuit
                .resistor_branches
                .names
                .get(index)
                .map(String::as_str)
                .unwrap_or("<unnamed branch-form resistor>");
            unsupported.push(format!(
                "branch-form resistor '{name}' cyclostationary flicker noise (coefficient={coefficient}, AF={af}, EF={ef})"
            ));
        }
    }
    for diode in &circuit.diodes.devices {
        if diode.kf != 0.0 {
            unsupported.push(format!(
                "diode '{}' flicker noise (KF={}, AF={})",
                diode.name, diode.kf, diode.af
            ));
        }
    }
    for mos in &circuit.mosfets.devices {
        if mos.kf != 0.0 {
            unsupported.push(format!(
                "MOSFET '{}' flicker noise (KF={}, AF={}, EF={})",
                mos.name, mos.kf, mos.af, mos.ef
            ));
        }
    }
    for jfet in &circuit.jfets {
        if jfet.params.kf != 0.0 {
            unsupported.push(format!(
                "JFET '{}' flicker noise (KF={}, AF={}, EF={})",
                jfet.name, jfet.params.kf, jfet.params.af, jfet.params.ef
            ));
        }
    }
    unsupported
}

fn validate_resistor_noise_metadata(circuit: &CircuitData) -> Result<(), SimulationError> {
    let count = circuit.resistors.len();
    for (label, actual) in [
        ("stamps", circuit.resistors.stamps.len()),
        ("conductances", circuit.resistors.conductances.len()),
        (
            "small-signal conductances",
            circuit.resistors.small_signal_conductances.len(),
        ),
        (
            "noise-temperature offsets",
            circuit.resistors.noise_temperature_offsets.len(),
        ),
        ("NOISY controls", circuit.resistors.noisy.len()),
        ("flicker controls", circuit.resistors.flicker.len()),
    ] {
        if actual != count {
            return Err(SimulationError::Circuit(format!(
                "pnoise resistor metadata is misaligned: {count} names but {actual} {label}"
            )));
        }
    }
    let absolute_count = circuit.resistor_absolute_noise_temperature_count();
    if absolute_count > count {
        return Err(SimulationError::Circuit(format!(
            "pnoise resistor metadata is misaligned: {count} names but {absolute_count} absolute noise temperatures"
        )));
    }
    let branch_count = circuit.resistor_branches.len();
    for (label, actual) in [
        ("positive nodes", circuit.resistor_branches.node_pos.len()),
        ("negative nodes", circuit.resistor_branches.node_neg.len()),
        (
            "branch indices",
            circuit.resistor_branches.branch_indices.len(),
        ),
        (
            "DC resistances",
            circuit.resistor_branches.resistances.len(),
        ),
        (
            "small-signal resistances",
            circuit.resistor_branches.small_signal_resistances.len(),
        ),
        (
            "reported resistances",
            circuit.resistor_branches.reported_resistances.len(),
        ),
        (
            "noise-temperature offsets",
            circuit.resistor_branches.noise_temperature_offsets.len(),
        ),
        (
            "absolute noise temperatures",
            circuit.resistor_branches.absolute_noise_temperatures.len(),
        ),
        ("NOISY controls", circuit.resistor_branches.noisy.len()),
        ("flicker controls", circuit.resistor_branches.flicker.len()),
    ] {
        if actual != branch_count {
            return Err(SimulationError::Circuit(format!(
                "pnoise branch-form resistor metadata is misaligned: {branch_count} names but {actual} {label}"
            )));
        }
    }
    Ok(())
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

fn checked_scaled_positive_ratio(
    factors: &[Value],
    divisor: Value,
    quantity: &str,
) -> Result<ScaledPositive, SimulationError> {
    if !divisor.is_finite() || divisor <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "{quantity} has invalid positive divisor {divisor}"
        )));
    }
    let divisor_exponent = libm::ilogb(divisor);
    let divisor_mantissa = libm::scalbn(divisor, -divisor_exponent);
    if !divisor_mantissa.is_finite() || divisor_mantissa <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "{quantity} normalized divisor is invalid ({divisor_mantissa})"
        )));
    }
    let mut normalized_factors = Vec::new();
    normalized_factors
        .try_reserve_exact(factors.len().saturating_add(1))
        .map_err(|error| {
            SimulationError::Circuit(format!("{quantity} factor allocation failed: {error}"))
        })?;
    normalized_factors.extend_from_slice(factors);
    normalized_factors.push(1.0 / divisor_mantissa);
    let mut scaled = checked_scaled_positive_product(&normalized_factors, quantity)?;
    scaled.exponent = scaled
        .exponent
        .checked_sub(divisor_exponent)
        .ok_or_else(|| {
            SimulationError::Circuit(format!("{quantity} exponent exceeds this platform"))
        })?;
    Ok(scaled)
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
        if let Some(PnoiseOperatingPoint::Shooting(point)) = &operating_point {
            point.authenticate_for_reuse(netlist, &self.config, point.config())?;
        }

        let circuit = self.build_circuit_with_abort(netlist, abort)?;
        validate_resistor_noise_metadata(&circuit)?;
        let num_nodes = circuit.num_nodes();
        if num_nodes == 0 {
            return Err(SimulationError::Circuit("Circuit has no nodes".to_string()));
        }
        let periodic_branches = circuit
            .num_branches()
            .checked_add(Self::hb_periodic_extra_branch_count(&circuit)?)
            .ok_or_else(|| {
                SimulationError::Circuit(
                    "pnoise canonical and distributed-network branch count overflows this platform"
                        .to_string(),
                )
            })?;
        let periodic_unknowns = num_nodes.checked_add(periodic_branches).ok_or_else(|| {
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
        let unsupported_colored = unsupported_device_colored_noise(&circuit);
        if !unsupported_colored.is_empty() {
            return Err(SimulationError::Circuit(format!(
                "driven pnoise requires exact cyclostationary colored-noise folding, which is not implemented for {}; set the listed noise coefficient exactly to zero to disable that mechanism",
                unsupported_colored.join(", ")
            )));
        }
        let temperature = self.config.temperature;
        let physical_constants = pnoise_physical_constants(self.config.spice_dialect);
        if !temperature.is_finite() || temperature <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "pnoise absolute analysis temperature must be finite and positive, got {temperature} K"
            )));
        }
        if !physical_constants.boltzmann.is_finite()
            || physical_constants.boltzmann <= 0.0
            || !physical_constants.electron_charge.is_finite()
            || physical_constants.electron_charge <= 0.0
        {
            return Err(SimulationError::Circuit(format!(
                "pnoise physical constants must be finite and positive, got k={}, q={}",
                physical_constants.boltzmann, physical_constants.electron_charge
            )));
        }

        self.ensure_result_shape(op_harmonics.saturating_add(1), num_nodes.saturating_mul(2))?;
        let drive_tones = Self::hb_collect_drive_tones(&hb_config)?;

        let mut solver = HbSolver::try_new(hb_config.clone(), num_nodes).map_err(|error| {
            SimulationError::Circuit(format!("pnoise solver construction failed: {error}"))
        })?;
        let node_names = self.hb_build_node_names(&circuit, num_nodes);
        solver.set_node_names(node_names.clone());

        // One exact canonical V/L/R MNA registry owns both the periodic
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
        if let Some(PnoiseOperatingPoint::Shooting(point)) = &operating_point {
            point.authenticate_for_reuse(netlist, &self.config, point.config())?;
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

        let k_b = physical_constants.boltzmann;

        // Stationary resistor thermal sources: 4kT*G between the resistor
        // terminals (DC-only intensity spectrum).
        let mut sources: Vec<PeriodicNoiseSource> = Vec::new();

        // `.OPTIONS RSHUNT` is a physical resistor from every electrical
        // node to ground. It is distinct from numerical GMIN and therefore
        // contributes one independent stationary 4kT*G source per stamped
        // electrical row. Private DAE state rows are deliberately excluded.
        let shunt_conductance = circuit.global_shunt_conductance();
        if !shunt_conductance.is_finite() || shunt_conductance < 0.0 {
            return Err(SimulationError::Circuit(format!(
                "pnoise .OPTIONS RSHUNT conductance is invalid ({shunt_conductance})"
            )));
        }
        if shunt_conductance > 0.0 {
            let shunt_density = checked_scaled_positive_product(
                &[4.0, k_b, temperature, shunt_conductance],
                "pnoise .OPTIONS RSHUNT thermal-noise density",
            )?;
            for node_index in 0..num_nodes {
                if circuit.is_non_electrical_state_matrix_index(node_index) {
                    continue;
                }
                let node_name = node_names.get(node_index).ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "pnoise .OPTIONS RSHUNT node metadata is missing for electrical row {node_index}"
                    ))
                })?;
                sources.push(PeriodicNoiseSource {
                    name: format!("RSHUNT:{node_name} thermal"),
                    node_pos: node_index,
                    node_neg: usize::MAX,
                    psd: vec![Complex64::new(shunt_density.mantissa, 0.0)],
                    binary_scale_exponent: shunt_density.exponent,
                    flicker: None,
                });
            }
        }

        for i in 0..circuit.resistors.len() {
            if !circuit.resistors.noisy[i] {
                continue;
            }
            let g = circuit.resistors.small_signal_conductance(i);
            if !g.is_finite() || g < 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "pnoise resistor '{}' has invalid conductance {g}",
                    circuit.resistors.names[i]
                )));
            }
            if g == 0.0 {
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
                &[4.0, k_b, source_temperature, g],
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
        }

        for i in 0..circuit.resistor_branches.len() {
            if !circuit.resistor_branches.noisy[i] {
                continue;
            }
            let name = &circuit.resistor_branches.names[i];
            let resistance = circuit.resistor_branches.small_signal_resistances[i];
            if !resistance.is_finite() || resistance < 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "pnoise branch-form resistor '{name}' has invalid noise resistance {resistance}"
                )));
            }
            // An exact ideal short has zero Thevenin noise voltage, and a
            // parallel Norton source cannot perturb its constrained terminals.
            if resistance == 0.0 {
                continue;
            }
            let source_temperature = circuit.resistor_branches.noise_temperature(i, temperature);
            if !source_temperature.is_finite() || source_temperature <= 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "pnoise branch-form resistor '{name}' absolute noise temperature must be finite and positive, got {source_temperature} K"
                )));
            }
            let thermal_density = checked_scaled_positive_ratio(
                &[4.0, k_b, source_temperature],
                resistance,
                &format!("pnoise branch-form resistor '{name}' thermal-noise density"),
            )?;
            sources.push(PeriodicNoiseSource {
                name: format!("{name} thermal"),
                node_pos: Self::hb_node_to_solver_index(
                    circuit.resistor_branches.node_pos[i],
                    num_nodes,
                ),
                node_neg: Self::hb_node_to_solver_index(
                    circuit.resistor_branches.node_neg[i],
                    num_nodes,
                ),
                psd: vec![Complex64::new(thermal_density.mantissa, 0.0)],
                binary_scale_exponent: thermal_density.exponent,
                flicker: None,
            });
        }

        // Cyclostationary device sources from the converged waveforms.
        sources.extend(
            solver
                .device_noise_sources(&state, temperature, physical_constants)
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
    fn driven_pnoise_rejects_every_misaligned_branch_resistor_noise_vector() {
        let mut base = crate::CircuitData::new();
        let out = base.get_or_create_node("out");
        let branch = base.allocate_branch_named("RAuth");
        base.resistor_branches.add_with_reported(
            "RAuth".to_string(),
            out,
            0,
            branch,
            0.6,
            1.2,
            0.6,
        );
        for label in [
            "noise-temperature offsets",
            "absolute noise temperatures",
            "NOISY controls",
            "flicker controls",
        ] {
            let mut circuit = base.clone();
            match label {
                "noise-temperature offsets" => {
                    circuit.resistor_branches.noise_temperature_offsets.clear();
                }
                "absolute noise temperatures" => {
                    circuit
                        .resistor_branches
                        .absolute_noise_temperatures
                        .clear();
                }
                "NOISY controls" => circuit.resistor_branches.noisy.clear(),
                "flicker controls" => circuit.resistor_branches.flicker.clear(),
                _ => unreachable!(),
            }
            let error = validate_resistor_noise_metadata(&circuit).unwrap_err();
            let message = error.to_string();
            assert!(
                message.contains("branch-form resistor metadata is misaligned")
                    && message.contains(label),
                "unexpected {label} alignment diagnostic: {message}"
            );
        }
    }

    #[test]
    fn driven_pnoise_selects_the_complete_dialect_constant_pair() {
        let modern = pnoise_physical_constants(crate::engine::SpiceDialect::Ngspice);
        assert_eq!(modern.boltzmann, crate::constants::K_BOLTZMANN);
        assert_eq!(modern.electron_charge, crate::constants::Q_ELECTRON);

        let xyce = pnoise_physical_constants(crate::engine::SpiceDialect::Xyce);
        assert_eq!(xyce.boltzmann, crate::constants::XYCE_K_BOLTZMANN);
        assert_eq!(xyce.electron_charge, crate::constants::XYCE_Q_ELECTRON);
    }

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
}
