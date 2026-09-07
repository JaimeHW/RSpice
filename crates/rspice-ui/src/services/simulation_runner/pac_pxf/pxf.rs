//! Periodic transfer function analysis.
//!
//! Transfer from every input sideband to one output, which is how conversion
//! gain and image rejection are measured.
//!
//! The run itself belongs to the engine: `.PXF` is an authored card, and
//! [`rspice_core::Engine::run_pxf_card_from_pss_with_abort`] is a PAC solve
//! plus one read of the conversion element the card's sideband pair names.
//! What is left here is the Studio's own share — turning a dialog or a deck
//! line into that card, and turning the engine's result into the curves the
//! sheet plots.

use std::path::Path;

use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;

use super::super::error::{ensure_not_aborted, poll_periodically};
use super::super::{
    ServiceRunError, ServiceRunResult, build_resolved_periodic_engine, build_voltage_output_expr,
    is_ground_like, parse_runner_netlist_with_abort,
};
use super::shared::normalize_pac_node_name;
// =============================================================================
// PXF (Periodic Transfer Function) Analysis
// =============================================================================

/// Frequency sweep type for periodic transfer-function analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PxfFrequencySweep {
    Decade,
    Octave,
    Linear,
}

impl PxfFrequencySweep {
    fn to_variation(self) -> rspice_core::netlist::FreqVariation {
        match self {
            Self::Decade => rspice_core::netlist::FreqVariation::Dec,
            Self::Octave => rspice_core::netlist::FreqVariation::Oct,
            Self::Linear => rspice_core::netlist::FreqVariation::Lin,
        }
    }
}

/// Explicit configuration for PXF execution.
#[derive(Debug, Clone)]
pub struct PxfRunConfig {
    pub pss_fundamental_freq: Value,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: Value,
    pub start_freq: Value,
    pub stop_freq: Value,
    pub points_per_unit: usize,
    pub sweep: PxfFrequencySweep,
    pub input_source: String,
    pub input_sideband: i32,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub output_sideband: i32,
    pub max_sideband: i32,
    pub reltol: Value,
    pub abstol: Value,
}

impl Default for PxfRunConfig {
    fn default() -> Self {
        Self {
            pss_fundamental_freq: 1e6,
            pss_num_harmonics: 10,
            pss_tolerance: 1e-3,
            start_freq: 1e3,
            stop_freq: 1e9,
            points_per_unit: 10,
            sweep: PxfFrequencySweep::Decade,
            input_source: "VIN".to_string(),
            input_sideband: 1,
            output_node: "VOUT".to_string(),
            output_ref: None,
            output_sideband: 1,
            max_sideband: 5,
            reltol: 1e-3,
            abstol: 1e-12,
        }
    }
}

impl PxfRunConfig {
    fn validate(&self) -> Result<(), String> {
        if !self.pss_fundamental_freq.is_finite() || self.pss_fundamental_freq <= 0.0 {
            return Err("PXF requires a positive PSS fundamental frequency".to_string());
        }
        if self.pss_num_harmonics == 0 {
            return Err("PXF requires at least one PSS harmonic".to_string());
        }
        if !self.pss_tolerance.is_finite() || self.pss_tolerance <= 0.0 {
            return Err("PXF requires a positive PSS tolerance".to_string());
        }
        if !self.start_freq.is_finite() || self.start_freq <= 0.0 {
            return Err("PXF start frequency must be positive".to_string());
        }
        if !self.stop_freq.is_finite() || self.stop_freq < self.start_freq {
            return Err("PXF stop frequency must be >= start frequency".to_string());
        }
        if self.points_per_unit == 0 {
            return Err("PXF points per unit must be greater than zero".to_string());
        }
        if self.max_sideband < 0 {
            return Err("PXF max sideband must be non-negative".to_string());
        }
        if self.input_source.trim().is_empty() {
            return Err("PXF input source must be specified".to_string());
        }
        if self.output_node.trim().is_empty() {
            return Err("PXF output node must be specified".to_string());
        }
        if self.input_sideband.abs() > self.max_sideband {
            return Err(format!(
                "PXF input sideband {} exceeds configured max sideband {}",
                self.input_sideband, self.max_sideband
            ));
        }
        if self.output_sideband.abs() > self.max_sideband {
            return Err(format!(
                "PXF output sideband {} exceeds configured max sideband {}",
                self.output_sideband, self.max_sideband
            ));
        }
        if let Some(reference) = self
            .output_ref
            .as_deref()
            .map(str::trim)
            .filter(|node| !node.is_empty() && !is_ground_like(node))
            && reference.eq_ignore_ascii_case(self.output_node.trim())
        {
            return Err("PXF output node and output reference cannot be the same node".to_string());
        }
        if !self.reltol.is_finite() || self.reltol <= 0.0 {
            return Err("PXF relative tolerance must be positive".to_string());
        }
        if !self.abstol.is_finite() || self.abstol <= 0.0 {
            return Err("PXF absolute tolerance must be positive".to_string());
        }
        Ok(())
    }

    /// The authored `.PXF` card this configuration states.
    ///
    /// The names are canonicalized here rather than in the card: a Studio form
    /// may hold `V(out)` where a deck line holds `out`, and
    /// [`normalize_pac_node_name`] is what `.PAC` already uses to make the two
    /// the same node. Everything past this point is the engine's reading of a
    /// card, identical to the one the CLI and the wasm surface run.
    fn to_card(&self) -> rspice_core::netlist::PxfCard {
        rspice_core::netlist::PxfCard {
            sweep: rspice_core::netlist::PeriodicSweep {
                variation: self.sweep.to_variation(),
                points: self.points_per_unit,
                start_freq: self.start_freq,
                stop_freq: self.stop_freq,
            },
            input_source: self.input_source.trim().to_owned(),
            input_sideband: self.input_sideband,
            output_node: normalize_pac_node_name(&self.output_node),
            output_ref: self
                .output_ref
                .as_deref()
                .map(str::trim)
                .filter(|node| !node.is_empty())
                .map(str::to_owned),
            output_sideband: self.output_sideband,
            max_sideband: self.max_sideband,
            reltol: self.reltol,
            abstol: self.abstol,
            // The Studio only ever runs `.PXF` against a shooting carrier: its
            // manual-deck reader refuses `FROM=HB` outright and its dialog
            // offers no such control, so naming the selector is the honest
            // record of which engine entry runs below.
            source: rspice_core::netlist::PeriodicSourceSelector::Pss,
        }
    }
}

/// PXF analysis data.
#[derive(Debug, Clone)]
pub struct PxfData {
    /// Input frequency sweep points (Hz).
    pub frequencies: Vec<Value>,
    /// Converted output frequency for each input sweep point (Hz).
    pub output_frequencies: Vec<Value>,
    /// Complex transfer H(input sideband -> output sideband).
    pub transfer: Vec<Complex64>,
    /// Optional group delay curve [(Hz, s)], on its own midpoint abscissa.
    pub group_delay: Option<Vec<(Value, Value)>>,
    /// Input sideband index.
    pub input_sideband: i32,
    /// Output sideband index.
    pub output_sideband: i32,
    /// Output label.
    pub output_label: String,
}

/// Run PXF standalone -- solving its own periodic carrier rather than
/// receiving one -- with explicit configuration and cancellation.
///
/// Test-only. PXF ships as a dependent task through
/// [`run_pxf_analysis_from_pss_with_source_path_and_abort`].
#[cfg(test)]
pub fn run_pxf_analysis_with_config_and_abort(
    netlist_text: &str,
    config: &PxfRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PxfData> {
    run_pxf_analysis_with_config_and_source_path_and_abort(netlist_text, config, None, abort)
}

/// Run PXF from an exact retained PSS state with direct-call source-relative
/// include and model resolution.
pub fn run_pxf_analysis_from_pss_with_source_path_and_abort(
    netlist_text: &str,
    config: &PxfRunConfig,
    operating_point: &rspice_core::engine::PssOperatingPoint,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PxfData> {
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    run_pxf_analysis_for_netlist_with_operating_point_abort(
        &netlist,
        config,
        Some(operating_point),
        abort,
    )
}

/// Run PXF analysis with source-path resolution and cancellation, solving its
/// own periodic carrier.
///
/// Test-only; see [`run_pxf_analysis_with_config_and_abort`].
#[cfg(test)]
pub fn run_pxf_analysis_with_config_and_source_path_and_abort(
    netlist_text: &str,
    config: &PxfRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PxfData> {
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    run_pxf_analysis_for_netlist_with_operating_point_abort(&netlist, config, None, abort)
}

fn run_pxf_analysis_for_netlist_with_operating_point_abort(
    netlist: &rspice_core::Netlist,
    config: &PxfRunConfig,
    operating_point: Option<&rspice_core::engine::PssOperatingPoint>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PxfData> {
    ensure_not_aborted(abort)?;
    config.validate().map_err(ServiceRunError::Failure)?;

    let engine = build_resolved_periodic_engine(
        netlist,
        config.pss_tolerance,
        "PXF resolved producer configuration is invalid",
    )?;
    let card = config.to_card();

    let owned_carrier;
    let carrier = match operating_point {
        Some(operating_point) => operating_point,
        None => {
            owned_carrier = engine
                .run_pss_operating_point_with_abort(
                    netlist,
                    rspice_core::analysis::PssConfig::new(config.pss_fundamental_freq)
                        .with_harmonics(config.pss_num_harmonics)
                        .with_tolerance(config.pss_tolerance),
                    abort,
                )
                .map_err(|error| ServiceRunError::from_core("PXF prerequisite PSS", error))?;
            &owned_carrier
        }
    };

    let result = engine
        .run_pxf_card_from_pss_with_abort(netlist, &card, carrier, abort)
        .map_err(|error| ServiceRunError::from_core("PXF error", error))?;

    // Nothing below re-reads what the entry already established. It refuses an
    // empty transfer, a non-finite transfer value, and an offset grid that is
    // not finite, positive and strictly increasing; and it derives `freq_out`
    // through `SidebandTransfer::output_frequency`, which refuses a
    // non-representable absolute frequency by coordinate.
    let mut frequencies = Vec::with_capacity(result.points.len());
    let mut output_frequencies = Vec::with_capacity(result.points.len());
    let mut transfer = Vec::with_capacity(result.points.len());
    for (index, point) in result.points.iter().enumerate() {
        poll_periodically(abort, index)?;
        frequencies.push(point.freq_in);
        // Derived here, for one more commit, exactly as this runner has always
        // derived it. `point.freq_out` is core's own answer and is not the same
        // number; publishing it is a deliberate correction, and it belongs in a
        // commit a reviewer can read on its own rather than inside a deletion.
        output_frequencies.push(
            point.freq_in
                + Value::from(config.output_sideband - config.input_sideband)
                    * result.fundamental_freq,
        );
        transfer.push(point.transfer);
    }

    let group_delay_curve = result.group_delay_curve();
    let group_delay = (!group_delay_curve.is_empty()).then_some(group_delay_curve);

    let output_label =
        build_voltage_output_expr(config.output_node.trim(), config.output_ref.as_deref());

    ensure_not_aborted(abort)?;
    Ok(PxfData {
        frequencies,
        output_frequencies,
        transfer,
        group_delay,
        input_sideband: result.input_sideband,
        output_sideband: result.output_sideband,
        output_label,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::ImmediateAbort;

    #[test]
    fn pxf_service_preserves_typed_entry_abort() {
        let result = run_pxf_analysis_with_config_and_abort(
            "not a netlist",
            &PxfRunConfig::default(),
            &ImmediateAbort,
        );

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }
}
