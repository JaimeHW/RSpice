//! Saved-output semantics and bounded storage estimates.

use super::*;
use rspice_simulation_contract::config::{FrequencySweep, NoiseSweepType};

/// Static validation result for a candidate output contract. `RuntimeBound`
/// is not a placeholder: it records the precise evidence that cannot exist
/// until the solver has produced the retained source dataset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SavedOutputSemanticStatus {
    Valid { detail: String },
    RuntimeBound { reason: String },
    Invalid { reason: String },
}

/// Additional retained waveform/cache bytes attributable to one candidate
/// output across all enabled compatible prepared tasks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SavedOutputStorageEstimate {
    ExactBytes(u64),
    Indeterminate { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavedOutputPreflightReport {
    semantic_status: SavedOutputSemanticStatus,
    storage_estimate: SavedOutputStorageEstimate,
    /// The same bound, divided between the analyses that produce it.
    ///
    /// A caller pricing this output over a run set needs the parts: two
    /// analyses producing one output do not have to run at the same number of
    /// points, and multiplying the sum by the whole matrix prices a
    /// nominal-only analysis as if it crossed every corner.
    bytes_by_analysis: Vec<(AnalysisInstanceId, u64)>,
    compatible_analysis_count: usize,
    retained_engine_source_analysis_ids: Vec<AnalysisInstanceId>,
}

impl SavedOutputPreflightReport {
    pub fn invalid(reason: impl Into<String>) -> Self {
        let reason = reason.into();
        Self {
            semantic_status: SavedOutputSemanticStatus::Invalid {
                reason: reason.clone(),
            },
            storage_estimate: SavedOutputStorageEstimate::Indeterminate { reason },
            bytes_by_analysis: Vec::new(),
            compatible_analysis_count: 0,
            retained_engine_source_analysis_ids: Vec::new(),
        }
    }

    /// A report that bounds one output at exactly `bytes`, for tests that
    /// exercise arithmetic over reports rather than the preflight that
    /// produces them.
    #[cfg(test)]
    pub(crate) fn exact_for_test(bytes: u64) -> Self {
        Self {
            semantic_status: SavedOutputSemanticStatus::Valid {
                detail: "test".to_owned(),
            },
            storage_estimate: SavedOutputStorageEstimate::ExactBytes(bytes),
            // No analysis to attribute it to, so a ledger prices it at the
            // caller's default participation. That is what these fixtures
            // mean: one output, one bound, over the whole declared space.
            bytes_by_analysis: Vec::new(),
            compatible_analysis_count: 1,
            retained_engine_source_analysis_ids: Vec::new(),
        }
    }

    /// [`Self::exact_for_test`] with the bound attributed to one analysis, for
    /// a test that prices the same report over different participations.
    #[cfg(test)]
    pub(crate) fn exact_for_analysis_test(analysis: AnalysisInstanceId, bytes: u64) -> Self {
        Self {
            bytes_by_analysis: vec![(analysis, bytes)],
            ..Self::exact_for_test(bytes)
        }
    }

    pub const fn semantic_status(&self) -> &SavedOutputSemanticStatus {
        &self.semantic_status
    }

    pub const fn storage_estimate(&self) -> &SavedOutputStorageEstimate {
        &self.storage_estimate
    }

    /// [`Self::storage_estimate`], divided between the analyses producing it.
    ///
    /// Empty when the estimate is indeterminate, and empty for a fixture
    /// report that names no analysis. The entries always sum to the scalar.
    pub fn bytes_by_analysis(&self) -> &[(AnalysisInstanceId, u64)] {
        &self.bytes_by_analysis
    }

    pub const fn compatible_analysis_count(&self) -> usize {
        self.compatible_analysis_count
    }

    /// Prepared analyses whose complete engine waveform state must survive so
    /// this deferred output can be evaluated exactly after the run.
    pub fn retained_engine_source_analysis_ids(&self) -> &[AnalysisInstanceId] {
        &self.retained_engine_source_analysis_ids
    }
}

/// Conservative logical-byte ceiling for complete engine waveform source
/// state retained by deferred outputs.
///
/// The core resource contract bounds scalar result values for one analysis.
/// Counting every permitted value as f64 is the stable cross-platform upper
/// bound consumed by preflight. Several deferred outputs attached to the same
/// prepared analysis share this state and must count it exactly once.
pub fn retained_engine_source_upper_bound_bytes(analysis_count: usize) -> u64 {
    let values =
        u64::try_from(rspice_core::ResourceLimits::default().max_result_values).unwrap_or(u64::MAX);
    let per_analysis = values.saturating_mul(std::mem::size_of::<f64>() as u64);
    per_analysis.saturating_mul(u64::try_from(analysis_count).unwrap_or(u64::MAX))
}

pub fn preflight_saved_output<'a>(
    output: &SavedOutput,
    analyses: impl IntoIterator<Item = (AnalysisInstanceId, &'a AnalysisSpec)>,
    display_cache_sample_limit: usize,
) -> SavedOutputPreflightReport {
    let analyses = analyses.into_iter().collect::<Vec<_>>();
    let contracts = match compile_saved_output_contracts(output, analyses.iter().copied()) {
        Ok(contracts) => contracts,
        Err(reason) => return SavedOutputPreflightReport::invalid(reason),
    };

    let semantic_status = semantic_status(output, &contracts, &analyses);
    if let SavedOutputSemanticStatus::Invalid { reason } = &semantic_status {
        return SavedOutputPreflightReport::invalid(reason.clone());
    }
    let (storage_estimate, bytes_by_analysis) =
        storage_estimate(&contracts, &analyses, display_cache_sample_limit);
    let retained_engine_source_analysis_ids =
        if output.save_policy == SavedOutputPolicy::OnDemandFromRetainedState {
            contracts
                .iter()
                .map(PreparedSavedOutput::analysis_id)
                .collect()
        } else {
            Vec::new()
        };
    SavedOutputPreflightReport {
        semantic_status,
        storage_estimate,
        bytes_by_analysis,
        compatible_analysis_count: contracts.len(),
        retained_engine_source_analysis_ids,
    }
}

fn semantic_status(
    output: &SavedOutput,
    contracts: &[PreparedSavedOutput],
    analyses: &[(AnalysisInstanceId, &AnalysisSpec)],
) -> SavedOutputSemanticStatus {
    if output.kind == SavedOutputKind::RfPortQuantity {
        let mut requires_elaboration = false;
        for contract in contracts {
            let Some((_, spec)) = analyses
                .iter()
                .find(|(analysis_id, _)| *analysis_id == contract.analysis_id)
            else {
                return SavedOutputSemanticStatus::Invalid {
                    reason: format!(
                        "prepared analysis {} is absent from the preflight input",
                        contract.analysis_id
                    ),
                };
            };
            if let Err(reason) = validate_static_contract_semantics(output, spec) {
                return SavedOutputSemanticStatus::Invalid { reason };
            }
            requires_elaboration |= matches!(spec, AnalysisSpec::SParameter { .. });
        }
        if requires_elaboration {
            return SavedOutputSemanticStatus::RuntimeBound {
                reason: "RF port indices are bound to the elaborated circuit and its retained scattering traces".to_owned(),
            };
        }
        return SavedOutputSemanticStatus::Valid {
            detail: "RF port indices resolve to configured ports in every compatible analysis"
                .to_owned(),
        };
    }

    let reason = match output.kind {
        SavedOutputKind::RawVoltageOrCurrent => {
            "probe grammar and analysis ownership are valid; node/branch existence is bound to the sealed executable netlist"
        }
        SavedOutputKind::DerivedExpression => {
            "expression grammar and analysis ownership are valid; referenced traces are bound to each retained solver result"
        }
        SavedOutputKind::DeviceOperatingPointQuantity => {
            "device-quantity grammar and DC operating-point ownership are valid; device existence is bound to the sealed executable netlist"
        }
        SavedOutputKind::NoiseContributor => {
            "noise contributor grammar and analysis ownership are valid; contributor existence is bound to the retained noise report"
        }
        SavedOutputKind::RfPortQuantity => unreachable!("handled above"),
    };
    SavedOutputSemanticStatus::RuntimeBound {
        reason: reason.to_owned(),
    }
}

/// One output's bounded cost, and how it divides between the analyses that
/// produce it.
///
/// The scalar is the sum of the parts, so a reader can keep using it — but a
/// caller that prices the output over a run set needs the parts, because the
/// analyses producing it do not all run at the same number of points.
fn storage_estimate(
    contracts: &[PreparedSavedOutput],
    analyses: &[(AnalysisInstanceId, &AnalysisSpec)],
    display_cache_sample_limit: usize,
) -> (SavedOutputStorageEstimate, Vec<(AnalysisInstanceId, u64)>) {
    let mut by_analysis: Vec<(AnalysisInstanceId, u64)> = Vec::new();
    let mut total = 0_u64;
    for contract in contracts {
        if contract.policy == SavedOutputPolicy::OnDemandFromRetainedState {
            continue;
        }
        if contract.kind == SavedOutputKind::DerivedExpression {
            return (
                indeterminate(format!(
                    "'{}' storage depends on the evaluated sample domain and real or complex result",
                    contract.name
                )),
                Vec::new(),
            );
        }
        if contract.policy == SavedOutputPolicy::FailureDiagnosticsOnly {
            return (
                indeterminate(format!(
                    "'{}' is retained only on failure, so its storage depends on the partial dataset available at the failure boundary",
                    contract.name
                )),
                Vec::new(),
            );
        }
        let Some((_, spec)) = analyses
            .iter()
            .find(|(analysis_id, _)| *analysis_id == contract.analysis_id)
        else {
            return (
                indeterminate(format!(
                    "prepared analysis {} is absent from the preflight input",
                    contract.analysis_id
                )),
                Vec::new(),
            );
        };
        let sample_count = match deterministic_sample_count(contract, spec) {
            Ok(sample_count) => sample_count,
            Err(reason) => return (indeterminate(reason), Vec::new()),
        };
        let source_values = if stores_complex_components(contract.kind, spec.run_type()) {
            4_u64
        } else {
            2_u64
        };
        let source_bytes = sample_count
            .checked_mul(source_values)
            .and_then(|values| values.checked_mul(std::mem::size_of::<f64>() as u64));
        let cache_bytes = if contract.precision
            == SavedOutputPrecision::DisplayCacheWithFullSourcePrecision
            || contract.streaming == SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation
        {
            sample_count
                .min(display_cache_sample_limit as u64)
                .checked_mul(2)
                .and_then(|values| values.checked_mul(std::mem::size_of::<f32>() as u64))
        } else {
            Some(0)
        };
        let Some(output_bytes) =
            source_bytes.and_then(|source| cache_bytes.and_then(|cache| source.checked_add(cache)))
        else {
            return (
                indeterminate(
                    "saved-output storage estimate exceeds the supported 64-bit byte range",
                ),
                Vec::new(),
            );
        };
        let Some(next_total) = total.checked_add(output_bytes) else {
            return (
                indeterminate(
                    "aggregate saved-output storage estimate exceeds the supported 64-bit byte range",
                ),
                Vec::new(),
            );
        };
        total = next_total;
        match by_analysis
            .iter_mut()
            .find(|(analysis_id, _)| *analysis_id == contract.analysis_id)
        {
            // One analysis can produce several contracts for one output, and
            // it runs all of them at the same points, so they fold into one
            // line rather than becoming two entries priced separately.
            Some((_, bytes)) => *bytes = bytes.saturating_add(output_bytes),
            None => by_analysis.push((contract.analysis_id, output_bytes)),
        }
    }
    (SavedOutputStorageEstimate::ExactBytes(total), by_analysis)
}

fn indeterminate(reason: impl Into<String>) -> SavedOutputStorageEstimate {
    SavedOutputStorageEstimate::Indeterminate {
        reason: reason.into(),
    }
}

fn deterministic_sample_count(
    contract: &PreparedSavedOutput,
    spec: &AnalysisSpec,
) -> Result<u64, String> {
    if contract.kind == SavedOutputKind::DeviceOperatingPointQuantity {
        return Ok(1);
    }
    if let Some(grid) = contract.selection_grid {
        let intervals = ((grid.stop - grid.start) / grid.step).ceil();
        if !intervals.is_finite() || intervals < 0.0 || intervals >= u64::MAX as f64 {
            return Err(format!(
                "'{}' selected-point grid exceeds the supported estimate range",
                contract.name
            ));
        }
        return Ok(intervals as u64 + 1);
    }
    let count = match spec {
        AnalysisSpec::DcOp { .. } => Some(1_usize),
        AnalysisSpec::DcSweep {
            start,
            stop,
            step,
            source2,
            start2,
            stop2,
            step2,
            hysteresis,
            modes,
            ..
        } => {
            let primary = modes.primary.spec(*start, *stop, *step).points().len();
            if source2.is_some() {
                match (start2, stop2, step2) {
                    (Some(start), Some(stop), Some(step)) => {
                        Some(primary.saturating_mul(
                            modes.secondary.spec(*start, *stop, *step).points().len(),
                        ))
                    }
                    _ => None,
                }
            } else if *hysteresis {
                // Retained forward/reverse curves both include the turnaround.
                Some(primary.saturating_mul(2))
            } else {
                Some(primary)
            }
        }
        AnalysisSpec::Ac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
        }
        | AnalysisSpec::Disto {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            ..
        }
        | AnalysisSpec::SParameter {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            ..
        }
        | AnalysisSpec::Hbsp {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            ..
        }
        | AnalysisSpec::Hbnoise {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            ..
        }
        | AnalysisSpec::Psp {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            ..
        } => frequency_point_count(*start_freq, *stop_freq, *points_per_unit, *sweep),
        AnalysisSpec::Qpac { .. } | AnalysisSpec::Qpxf { .. } | AnalysisSpec::Qpnoise { .. } => {
            quasi_periodic_sample_count(spec)
        }
        AnalysisSpec::AcData {
            frequencies,
            table_options,
            ..
        } => (!table_options.from_netlist || !frequencies.is_empty()).then_some(frequencies.len()),
        AnalysisSpec::Noise {
            start_freq,
            stop_freq,
            points_per_decade,
            sweep,
            explicit_frequencies,
            ..
        } => explicit_frequencies.as_ref().map(Vec::len).or_else(|| {
            let sweep = match sweep {
                NoiseSweepType::Decade => FrequencySweep::Decade,
                NoiseSweepType::Octave => FrequencySweep::Octave,
                NoiseSweepType::Linear => FrequencySweep::Linear,
                NoiseSweepType::ExplicitFrequencyList | NoiseSweepType::Unsupported(_) => {
                    return None;
                }
            };
            frequency_point_count(*start_freq, *stop_freq, *points_per_decade, sweep)
        }),
        // One-sided, DC through Nyquist: the transform length decides it, so
        // a recorded FFT's point count is bounded by its own request.
        AnalysisSpec::Fft { request } => Some(request.points / 2 + 1),
        _ => None,
    };
    count
        .and_then(|count| u64::try_from(count).ok())
        .ok_or_else(|| {
            format!(
                "'{}' uses {}, whose prepared point count is data-dependent or not bounded by its analysis specification",
                contract.name,
                spec.run_type().display_name()
            )
        })
}

/// Count the native effective axis independently of output compatibility. The
/// request owns signed/explicit grids and logarithmic endpoint conventions.
fn quasi_periodic_sample_count(spec: &AnalysisSpec) -> Option<usize> {
    let limits = rspice_core::ResourceLimits::default();
    match spec {
        AnalysisSpec::Qpac { .. } => spec.qpac_card().ok().and_then(|card| {
            rspice_core::engine::QpacRequest::validate_qpac_card(&card, &limits).ok()
        }),
        AnalysisSpec::Qpnoise { .. } => spec.qpnoise_card().ok().and_then(|card| {
            rspice_core::engine::QpnoiseRequest::validate_qpnoise_card(&card, &limits).ok()
        }),
        AnalysisSpec::Qpxf { .. } => spec.qpxf_card().ok().and_then(|card| {
            rspice_core::engine::QpxfRequest::validate_qpxf_card(&card, &limits).ok()
        }),
        _ => None,
    }
}

fn frequency_point_count(
    start: f64,
    stop: f64,
    points_per_unit: usize,
    sweep: FrequencySweep,
) -> Option<usize> {
    let variation = match sweep {
        FrequencySweep::Linear => rspice_core::netlist::FreqVariation::Lin,
        FrequencySweep::Decade => rspice_core::netlist::FreqVariation::Dec,
        FrequencySweep::Octave => rspice_core::netlist::FreqVariation::Oct,
    };
    rspice_core::analysis::ac::try_ac_sweep_point_count_bounded_with_abort(
        variation,
        points_per_unit,
        start,
        stop,
        rspice_core::ResourceLimits::default().max_analysis_points,
        &rspice_core::NoAbort,
    )
    .ok()
}

fn stores_complex_components(kind: SavedOutputKind, run_type: AnalysisRunType) -> bool {
    kind == SavedOutputKind::RfPortQuantity
        || kind == SavedOutputKind::RawVoltageOrCurrent
            && matches!(
                run_type,
                AnalysisRunType::Ac
                    | AnalysisRunType::Pac
                    | AnalysisRunType::Pxf
                    | AnalysisRunType::Pstb
                    | AnalysisRunType::Stb
                    | AnalysisRunType::SParameter
                    | AnalysisRunType::Hbsp
                    | AnalysisRunType::Psp
                    | AnalysisRunType::Qpac
                    | AnalysisRunType::Qpxf
            )
}

#[cfg(test)]
mod tests {
    use super::*;
    const DISPLAY_CACHE_SAMPLE_LIMIT: usize = 4096;
    fn output(policy: SavedOutputPolicy, precision: SavedOutputPrecision) -> SavedOutput {
        SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            "output_voltage",
            "V(out)",
            SavedOutputCompatibility::AllCompatibleAnalyses,
            policy,
            precision,
            SavedOutputStreaming::StoreOnly,
        )
        .expect("valid output")
    }

    fn transient_spec() -> AnalysisSpec {
        AnalysisSpec::Transient {
            stop_time: 1.0,
            step_time: 0.25,
            start_time: 0.0,
            max_timestep: Some(0.1),
            uic: false,
        }
    }

    #[test]
    fn preflight_estimates_frequency_storage_from_runtime_grids() {
        let output = output(
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::FullSourcePrecision,
        );
        let analysis_id = AnalysisInstanceId::new();
        for (start_freq, stop_freq, points_per_unit, sweep, expected_count) in [
            (1.0, 1000.0, 10, FrequencySweep::Decade, 31),
            (10.0, 80.0, 2, FrequencySweep::Octave, 7),
            (0.0, 100.0, 3, FrequencySweep::Linear, 3),
            (0.0, 100.0, 2, FrequencySweep::Linear, 1),
            (100.0, 100.0, 10, FrequencySweep::Decade, 1),
        ] {
            let spec = AnalysisSpec::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
            };
            let report =
                preflight_saved_output(&output, [(analysis_id, &spec)], DISPLAY_CACHE_SAMPLE_LIMIT);
            assert_eq!(report.compatible_analysis_count(), 1);
            assert_eq!(
                report.storage_estimate(),
                &SavedOutputStorageEstimate::ExactBytes(expected_count * 4 * 8)
            );
            assert!(matches!(
                report.semantic_status(),
                SavedOutputSemanticStatus::RuntimeBound { .. }
            ));
        }
    }

    #[test]
    fn preflight_marks_adaptive_transient_capture_indeterminate() {
        let output = output(
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::FullSourcePrecision,
        );
        let spec = transient_spec();
        let report = preflight_saved_output(
            &output,
            [(AnalysisInstanceId::new(), &spec)],
            DISPLAY_CACHE_SAMPLE_LIMIT,
        );
        assert!(matches!(
            report.storage_estimate(),
            SavedOutputStorageEstimate::Indeterminate { reason }
                if reason.contains("data-dependent")
        ));
    }

    #[test]
    fn preflight_exactly_bounds_default_schematic_probe_grid() {
        let output = output(
            SavedOutputPolicy::SelectedAndFinalPoints,
            SavedOutputPrecision::DisplayCacheWithFullSourcePrecision,
        );
        let spec = transient_spec();
        let report = preflight_saved_output(
            &output,
            [(AnalysisInstanceId::new(), &spec)],
            DISPLAY_CACHE_SAMPLE_LIMIT,
        );

        // 0 through 1 at 0.25 is five retained samples.  A real voltage
        // waveform stores an f64 x/y pair and the requested display cache
        // stores an f32 x/y pair for each sample.
        assert_eq!(
            report.storage_estimate(),
            &SavedOutputStorageEstimate::ExactBytes(5 * ((2 * 8) + (2 * 4)))
        );
    }

    #[test]
    fn qp_transfer_counts_use_explicit_signed_and_logarithmic_native_grids() {
        use rspice_simulation_contract::quasi_periodic_draft::QuasiPeriodicAcDraft;
        use rspice_simulation_contract::quasi_periodic_draft::QuasiPeriodicTransferDraft;
        let ac = QuasiPeriodicAcDraft {
            explicit_offsets: "-1, 0, 1, 2".into(),
            ..Default::default()
        };
        let mut xf = QuasiPeriodicTransferDraft {
            explicit_frequencies: "-1, 0, 1".into(),
            ..Default::default()
        };
        assert_eq!(quasi_periodic_sample_count(&ac.to_spec().unwrap()), Some(4));
        assert_eq!(quasi_periodic_sample_count(&xf.to_spec().unwrap()), Some(3));
        xf.explicit_frequencies.clear();
        xf.sweep.start = "-10".into();
        xf.sweep.stop = "10".into();
        xf.sweep.sweep = 2;
        xf.sweep.points = "5".into();
        assert_eq!(quasi_periodic_sample_count(&xf.to_spec().unwrap()), Some(5));
        xf.sweep.start = "1".into();
        xf.sweep.stop = "12".into();
        xf.sweep.sweep = 0;
        xf.sweep.points = "3".into();
        assert_eq!(
            quasi_periodic_sample_count(&xf.to_spec().unwrap()),
            Some(4),
            "native logarithmic grids use ceil(density*span)"
        );
    }
}
