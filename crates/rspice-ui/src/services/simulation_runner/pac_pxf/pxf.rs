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
use super::super::periodic_carrier::{PeriodicCarrier, PeriodicCarrierState};
use super::super::{
    ServiceRunError, ServiceRunResult, build_resolved_periodic_engine, build_voltage_output_expr,
    is_ground_like, parse_runner_netlist_with_abort,
};
use super::shared::normalize_pac_node_name;
// =============================================================================
// PXF (Periodic Transfer Function) Analysis
// =============================================================================

/// Frequency sweep type for periodic transfer-function analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
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
    /// Which periodic solve this run linearizes around; the card's `FROM=`.
    pub carrier: PeriodicCarrier,
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
            carrier: PeriodicCarrier::Preceding,
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
        // The carrier itself is not a range check: it names a family, and
        // whether the state this run was handed belongs to that family is
        // `PeriodicCarrierState::accepted_by`, asked where both are in hand.
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
            // The selector the request states, in the engine's own vocabulary.
            // It records which family this run linearizes around; the entry
            // that runs below is chosen from the state actually handed over,
            // and the two are checked against each other once.
            source: match self.carrier {
                PeriodicCarrier::Preceding => {
                    rspice_core::netlist::PeriodicSourceSelector::Preceding
                }
                PeriodicCarrier::Pss => rspice_core::netlist::PeriodicSourceSelector::Pss,
                PeriodicCarrier::Hb => rspice_core::netlist::PeriodicSourceSelector::Hb,
            },
        }
    }
}

/// PXF analysis data.
#[derive(Debug, Clone)]
pub struct PxfData {
    /// Swept baseband offsets, in hertz.
    ///
    /// This is the abscissa the card authored and the one every curve below is
    /// stated against. It is **not** the frequency the drive is applied at:
    /// that is `offset + input_sideband * f0`, and the two differ for every
    /// sideband but zero. Core's `TransferPoint::freq_in` -- whose name says
    /// otherwise and whose documentation says this -- is where the value comes
    /// from; the field is named for what it holds on the way to the sheet.
    pub offset_frequencies: Vec<Value>,
    /// Absolute frequency the converted response appears at, in hertz:
    /// `output_sideband * f0 + offset`, where the offset is the swept
    /// abscissa beside it.
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
    run_pxf_analysis_on_materialized_with_abort(
        &netlist,
        config,
        Some(PeriodicCarrierState::Shooting(operating_point)),
        abort,
    )
}

/// Run PXF from an exact retained harmonic-balance state.
///
/// `Engine::run_pxf_card_from_hb_with_abort` reads the same path through the
/// same conversion matrix its shooting sibling reads; the readout below is
/// therefore the one readout, not a second one for this carrier.
pub fn run_pxf_analysis_from_hb_with_source_path_and_abort(
    netlist_text: &str,
    config: &PxfRunConfig,
    operating_point: &rspice_core::engine::HbOperatingPoint,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PxfData> {
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    run_pxf_analysis_on_materialized_with_abort(
        &netlist,
        config,
        Some(PeriodicCarrierState::HarmonicBalance(operating_point)),
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
    run_pxf_analysis_on_materialized_with_abort(&netlist, config, None, abort)
}

pub(crate) fn run_pxf_analysis_on_materialized_with_abort(
    netlist: &rspice_core::Netlist,
    config: &PxfRunConfig,
    carrier: Option<PeriodicCarrierState<'_>>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PxfData> {
    ensure_not_aborted(abort)?;
    config.validate().map_err(ServiceRunError::Failure)?;
    if let Some(carrier) = carrier {
        carrier
            .accepted_by(config.carrier, ".PXF")
            .map_err(ServiceRunError::Failure)?;
    }

    let engine = build_resolved_periodic_engine(
        netlist,
        carrier.map_or(config.pss_tolerance, |carrier| {
            carrier.engine_tolerance(config.pss_tolerance)
        }),
        "PXF resolved producer configuration is invalid",
    )?;
    let card = config.to_card();

    let owned_carrier;
    let carrier = match carrier {
        Some(carrier) => carrier,
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
            PeriodicCarrierState::Shooting(&owned_carrier)
        }
    };

    let result = match carrier {
        PeriodicCarrierState::Shooting(operating_point) => {
            engine.run_pxf_card_from_pss_with_abort(netlist, &card, operating_point, abort)
        }
        PeriodicCarrierState::HarmonicBalance(operating_point) => {
            engine.run_pxf_card_from_hb_with_abort(netlist, &card, operating_point, abort)
        }
    }
    .map_err(|error| ServiceRunError::from_core("PXF error", error))?;

    // Nothing below re-reads what the entry already established. It refuses an
    // empty transfer, a non-finite transfer value, and an offset grid that is
    // not finite, positive and strictly increasing; and it derives `freq_out`
    // through `SidebandTransfer::output_frequency`, which refuses a
    // non-representable absolute frequency by coordinate.
    let mut offset_frequencies = Vec::with_capacity(result.points.len());
    let mut output_frequencies = Vec::with_capacity(result.points.len());
    let mut transfer = Vec::with_capacity(result.points.len());
    for (index, point) in result.points.iter().enumerate() {
        poll_periodically(abort, index)?;
        offset_frequencies.push(point.freq_in);
        output_frequencies.push(point.freq_out);
        transfer.push(point.transfer);
    }

    let group_delay_curve = result.group_delay_curve();
    let group_delay = (!group_delay_curve.is_empty()).then_some(group_delay_curve);

    let output_label =
        build_voltage_output_expr(config.output_node.trim(), config.output_ref.as_deref());

    ensure_not_aborted(abort)?;
    Ok(PxfData {
        offset_frequencies,
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
    use rspice_core::abort_signal::{ImmediateAbort, NoAbort};

    /// An RC low-pass whose corner sits exactly on the carrier fundamental, so
    /// the transfer is a closed form at every sideband and the carrier is cheap
    /// to solve. The deck carries no analysis cards: the configurations below
    /// are what a Studio form or a `.PXF` line would have produced.
    const FIXTURE_DECK: &str = "PXF converted output frequency fixture\n\
         vin in 0 dc 0 ac 1\n\
         r1 in out 1k\n\
         c1 out 0 159.154943091895p\n\
         .end\n";
    const FIXTURE_FUNDAMENTAL: Value = 1.0e6;
    const FIXTURE_OUTPUT_SIDEBAND: i32 = 1;
    /// `LIN 3` over 100 kHz..500 kHz. These are baseband offsets, not the
    /// absolute frequencies the drive is applied at.
    const FIXTURE_OFFSETS: [Value; 3] = [1.0e5, 3.0e5, 5.0e5];

    fn fixture_config(input_sideband: i32) -> PxfRunConfig {
        PxfRunConfig {
            pss_fundamental_freq: FIXTURE_FUNDAMENTAL,
            pss_num_harmonics: 8,
            pss_tolerance: 1.0e-3,
            start_freq: FIXTURE_OFFSETS[0],
            stop_freq: FIXTURE_OFFSETS[2],
            points_per_unit: FIXTURE_OFFSETS.len(),
            sweep: PxfFrequencySweep::Linear,
            input_source: "VIN".to_owned(),
            input_sideband,
            output_node: "OUT".to_owned(),
            output_ref: None,
            output_sideband: FIXTURE_OUTPUT_SIDEBAND,
            max_sideband: 1,
            reltol: 1.0e-3,
            abstol: 1.0e-12,
            carrier: PeriodicCarrier::Preceding,
        }
    }

    fn fixture_run(input_sideband: i32) -> PxfData {
        // The carrier has to come out of the same elaboration the runner will
        // perform: a retained operating point carries the semantic identity of
        // the circuit it was solved on, and the entry refuses one that does not
        // match the netlist it is handed.
        let netlist = parse_runner_netlist_with_abort(FIXTURE_DECK, None, &NoAbort)
            .expect("the fixture deck parses");
        let engine = build_resolved_periodic_engine(&netlist, 1.0e-3, "fixture engine")
            .expect("the fixture engine configuration resolves");
        let carrier = engine
            .run_pss_operating_point_with_abort(
                &netlist,
                rspice_core::analysis::PssConfig::new(FIXTURE_FUNDAMENTAL)
                    .with_harmonics(8)
                    .with_points_per_period(128)
                    .with_tstab_periods(0),
                &NoAbort,
            )
            .expect("the linear carrier converges");

        run_pxf_analysis_from_pss_with_source_path_and_abort(
            FIXTURE_DECK,
            &fixture_config(input_sideband),
            &carrier,
            None,
            &NoAbort,
        )
        .expect("the PXF run publishes a transfer")
    }

    /// Both carriers read the same path through the same conversion matrix.
    ///
    /// `.PXF` is one element of the matrix `.PAC` fills, so the argument is
    /// the one made in `pac_around_hb_and_pac_around_pss_agree_on_a_linear_circuit`:
    /// an RC network's small-signal linearization does not depend on the
    /// operating point, the periodically time-varying system is therefore time
    /// invariant, and its conversion matrix is diagonal. The diagonal element
    /// this test reads is the network's own AC transfer at the **drive**
    /// frequency `offset + input_sideband * f0`, which is the closed form both
    /// runs are held to. The off-diagonal element at `INPUTSIDEBAND=0` with
    /// `OUTSIDEBAND=1` must be zero for the same reason, and that is asserted
    /// against the same budget.
    ///
    /// Bound: `1e-9` relative, a round-off budget rather than either solver's
    /// convergence tolerance — see the `.PAC` test for why neither solver's
    /// iteration enters a linear network's answer.
    #[test]
    fn pxf_around_hb_and_pxf_around_pss_agree_on_a_linear_circuit() {
        use crate::services::simulation_runner::hb::{
            HbRunConfig, HbToneRunConfig, run_hb_analysis_with_abort,
        };

        const RESISTANCE: Value = 1.0e3;
        const CAPACITANCE: Value = 159.154_943_091_895e-12;
        const BOUND: Value = 1.0e-9;

        let harmonic_balance = run_hb_analysis_with_abort(
            FIXTURE_DECK,
            &HbRunConfig {
                tones: vec![HbToneRunConfig::new(FIXTURE_FUNDAMENTAL, 8)],
                reltol: 1.0e-10,
                ..HbRunConfig::default()
            },
            &NoAbort,
        )
        .expect("the harmonic-balance carrier converges")
        .operating_point;

        for input_sideband in [0, 1] {
            let from_pss = fixture_run(input_sideband);
            let from_hb = run_pxf_analysis_from_hb_with_source_path_and_abort(
                FIXTURE_DECK,
                &fixture_config(input_sideband),
                harmonic_balance.as_ref(),
                None,
                &NoAbort,
            )
            .expect("the harmonic-balance-carried run publishes a transfer");

            assert_eq!(from_pss.offset_frequencies, from_hb.offset_frequencies);
            assert_eq!(from_pss.output_frequencies, from_hb.output_frequencies);
            assert_eq!(from_pss.input_sideband, from_hb.input_sideband);
            assert_eq!(from_pss.output_sideband, from_hb.output_sideband);

            for (index, offset) in from_pss.offset_frequencies.iter().copied().enumerate() {
                // The diagonal element is the transfer at the frequency the
                // drive is applied at; every other element of a time-invariant
                // conversion matrix is zero.
                let closed_form = if input_sideband == FIXTURE_OUTPUT_SIDEBAND {
                    let drive = (input_sideband as Value).mul_add(FIXTURE_FUNDAMENTAL, offset);
                    Complex64::new(1.0, 0.0)
                        / Complex64::new(
                            1.0,
                            std::f64::consts::TAU * drive * RESISTANCE * CAPACITANCE,
                        )
                } else {
                    Complex64::new(0.0, 0.0)
                };
                for (label, value) in [
                    ("shooting", from_pss.transfer[index]),
                    ("harmonic balance", from_hb.transfer[index]),
                ] {
                    assert!(
                        (value - closed_form).norm() <= BOUND,
                        "the {label} carrier reports {value} for sideband {input_sideband} -> \
                         {FIXTURE_OUTPUT_SIDEBAND} at offset {offset} Hz, and the network's own \
                         conversion matrix holds {closed_form}"
                    );
                }
                assert!(
                    (from_pss.transfer[index] - from_hb.transfer[index]).norm() <= BOUND,
                    "the two carriers disagree at offset {offset} Hz: {} versus {}",
                    from_pss.transfer[index],
                    from_hb.transfer[index]
                );
            }
        }
    }

    /// The old value, the new value, and why the new one is right.
    ///
    /// Until the engine ran this analysis the Studio derived the published
    /// "Converted Output Frequency" as
    /// `offset + (output_sideband - input_sideband) * f0`, which reads the
    /// swept variable as the absolute frequency the drive is applied at. It is
    /// not one: `ConversionMatrix::get_transfer` hands each sweep point
    /// straight through as `SidebandTransfer::frequency_offset`, so the swept
    /// variable *is* the baseband offset, and the frequency the converted
    /// response appears at is `output_sideband * f0 + offset`.
    ///
    /// At `INPUTSIDEBAND=1` -- the default, and until the step before this one
    /// the only value the Studio could write -- the two disagree by exactly one
    /// whole fundamental. With `f0 = 1 MHz` and `OUTSIDEBAND=1` this curve read
    /// 100 kHz / 300 kHz / 500 kHz and now reads 1.1 MHz / 1.3 MHz / 1.5 MHz.
    #[test]
    fn the_converted_output_frequency_moves_by_one_fundamental_at_input_sideband_one() {
        let published = fixture_run(1);

        assert_eq!(
            published.offset_frequencies,
            FIXTURE_OFFSETS.to_vec(),
            "the swept abscissa is the authored baseband offset"
        );
        assert_eq!(published.input_sideband, 1);
        assert_eq!(published.output_sideband, FIXTURE_OUTPUT_SIDEBAND);

        for (offset, converted) in FIXTURE_OFFSETS
            .iter()
            .copied()
            .zip(&published.output_frequencies)
        {
            let before = offset + Value::from(FIXTURE_OUTPUT_SIDEBAND - 1) * FIXTURE_FUNDAMENTAL;
            let after = Value::from(FIXTURE_OUTPUT_SIDEBAND).mul_add(FIXTURE_FUNDAMENTAL, offset);
            assert_eq!(
                before, offset,
                "the old formula published the offset itself"
            );
            assert_eq!(*converted, after);
            assert_eq!(
                *converted - before,
                FIXTURE_FUNDAMENTAL,
                "the correction is exactly one fundamental"
            );
        }
        assert_eq!(
            published.output_frequencies,
            vec![1.1e6, 1.3e6, 1.5e6],
            "the response appears one fundamental above each swept offset"
        );
    }

    /// The other half of the same fixture: at `INPUTSIDEBAND=0` the old
    /// formula's `(out - in)` happens to equal `out`, so it was accidentally
    /// correct and this number does not move. `INPUTSIDEBAND` only became
    /// authorable in the step before this one, which is why the error was
    /// reachable for every PXF run the Studio had ever made and invisible in
    /// this one case.
    #[test]
    fn the_converted_output_frequency_is_unchanged_at_input_sideband_zero() {
        let published = fixture_run(0);

        assert_eq!(published.offset_frequencies, FIXTURE_OFFSETS.to_vec());
        assert_eq!(published.input_sideband, 0);

        for (offset, converted) in FIXTURE_OFFSETS
            .iter()
            .copied()
            .zip(&published.output_frequencies)
        {
            let before = offset + Value::from(FIXTURE_OUTPUT_SIDEBAND) * FIXTURE_FUNDAMENTAL;
            let after = Value::from(FIXTURE_OUTPUT_SIDEBAND).mul_add(FIXTURE_FUNDAMENTAL, offset);
            assert_eq!(*converted, after);
            assert_eq!(
                before, *converted,
                "at INPUTSIDEBAND=0 the old formula was accidentally correct"
            );
        }
        assert_eq!(published.output_frequencies, vec![1.1e6, 1.3e6, 1.5e6]);
    }

    /// The group delay the sheet plots is core's own curve: one sample shorter
    /// than the sweep, on the midpoints of the swept offsets.
    #[test]
    fn the_group_delay_curve_sits_on_the_swept_offsets_midpoints() {
        let published = fixture_run(1);
        let group_delay = published
            .group_delay
            .expect("three sweep points give two group-delay samples");

        assert_eq!(group_delay.len(), FIXTURE_OFFSETS.len() - 1);
        assert_eq!(group_delay[0].0, 2.0e5);
        assert_eq!(group_delay[1].0, 4.0e5);
    }

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
