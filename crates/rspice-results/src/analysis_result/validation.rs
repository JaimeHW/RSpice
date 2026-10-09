//! Validate complete retained analyses and reconcile their independently versioned evidence.

use super::*;
use crate::soa_evidence::{
    SoaEvaluationEvidence, SoaRuleVerdictEvidence, SoaViolationEvidence,
    SoaViolationSeverityEvidence,
};
use crate::soa_source::SoaSourceHistory;
use crate::validation::{normalized_f64, same_retained_float};
use std::collections::HashSet;
mod pxf;
mod soa_derating;
mod soa_duration;
mod soa_envelope;
mod soa_reporting;
mod soa_source;

fn contains_retained_coordinate(sorted: &[f64], target: f64) -> bool {
    let target = normalized_f64(target);
    sorted
        .binary_search_by(|probe| normalized_f64(*probe).total_cmp(&target))
        .is_ok()
}

impl<W: AsRef<RetainedWaveform>> AnalysisResult<W> {
    /// Validate relationships between independently versioned retained fields.
    /// Historical analyses may legitimately lack a newer payload; when both
    /// fields exist they must describe one coherent execution.
    pub fn validate_retained_evidence(&self) -> Result<(), String> {
        self.validate_native_scalar_units()?;
        if let Some(checkpoint) = &self.transient_checkpoint {
            checkpoint.validate_for(self)?;
        }
        if let Some(checkpoint) = &self.monte_carlo_checkpoint {
            checkpoint.validate_for(self.into())?;
        }
        let retained_basis = self
            .result_payload
            .as_ref()
            .map(AnalysisResultPayload::retained_waveform_basis)
            .transpose()?
            .flatten();
        self.validate_saved_output_receipts(retained_basis.as_deref())?;
        if let Some(quality) = &self.convergence {
            quality.validate()?;
        }
        if let Some(source) = &self.import_source {
            source.validate()?;
            if let Some(coordinate) = &source.coordinate {
                coordinate.validate_domain(self.analysis_type)?;
            }
            if self.provenance.is_some() || !self.saved_output_receipts.is_empty() {
                return Err(
                    "imported results cannot carry native prepared-task or saved-output receipts"
                        .to_owned(),
                );
            }
        }
        let mut waveform_names = HashSet::with_capacity(self.waveforms.len());
        for waveform in self.waveforms.iter().map(AsRef::as_ref) {
            let name = waveform.name.trim();
            if name.is_empty() || waveform.name.chars().any(char::is_control) {
                return Err("retained waveform requires a non-empty control-free name".to_owned());
            }
            if !waveform_names.insert(waveform.name.as_str()) {
                return Err(format!(
                    "retained waveform name '{}' is duplicated",
                    waveform.name
                ));
            }
            if waveform.x.len() != waveform.y.len() {
                return Err(format!(
                    "retained waveform '{}' has {} coordinates but {} values",
                    waveform.name,
                    waveform.x.len(),
                    waveform.y.len()
                ));
            }
            if waveform.x.iter().any(|value| !value.is_finite())
                || waveform.y.iter().any(|value| value.is_infinite())
            {
                return Err(format!(
                    "retained waveform '{}' contains a non-finite coordinate or infinite value",
                    waveform.name
                ));
            }
            // Imported adapters explicitly admit unavailable source samples.
            // Native derived PXF delay gaps must be reproducible from finite
            // retained transfer samples. A solver NaN is still invalid.
            if self.import_source.is_none()
                && waveform.has_missing_samples()
                && !pxf::verified_group_delay(self, waveform)
            {
                return Err(format!(
                    "native retained waveform '{}' contains an unavailable sample without import attribution or verified derived evidence",
                    waveform.name
                ));
            }
            if waveform
                .unit
                .as_ref()
                .is_some_and(|unit| unit.trim().is_empty() || unit.chars().any(char::is_control))
            {
                return Err(format!(
                    "retained waveform '{}' has an invalid engineering unit",
                    waveform.name
                ));
            }
            if let Some(complex) = &waveform.complex {
                if complex.source_name.trim().is_empty()
                    || complex.source_name.chars().any(char::is_control)
                {
                    return Err(format!(
                        "retained waveform '{}' has an invalid complex-source name",
                        waveform.name
                    ));
                }
                if complex.real.len() != waveform.x.len() || complex.imag.len() != waveform.x.len()
                {
                    return Err(format!(
                        "retained waveform '{}' complex components do not match its {} coordinates",
                        waveform.name,
                        waveform.x.len()
                    ));
                }
                if complex
                    .real
                    .iter()
                    .chain(complex.imag.iter())
                    .any(|value| value.is_infinite())
                {
                    return Err(format!(
                        "retained waveform '{}' contains a non-finite complex component",
                        waveform.name
                    ));
                }
                if complex
                    .real
                    .iter()
                    .zip(complex.imag.iter())
                    .zip(waveform.y.iter())
                    .any(|((real, imag), value)| {
                        real.is_nan() != imag.is_nan() || real.is_nan() != value.is_nan()
                    })
                {
                    return Err(format!(
                        "retained waveform '{}' has inconsistent complex sample availability",
                        waveform.name
                    ));
                }
            }
        }

        let valid_text =
            |text: &str| !text.trim().is_empty() && !text.chars().any(char::is_control);
        if let Some(dc_op) = &self.dc_op {
            for (group, values) in [
                ("node voltage", dc_op.node_voltages.as_slice()),
                ("branch current", dc_op.branch_currents.as_slice()),
                ("device power", dc_op.power_dissipation.as_slice()),
            ] {
                let mut names = HashSet::with_capacity(values.len());
                for value in values {
                    if !valid_text(&value.name) || !valid_text(&value.unit) {
                        return Err(format!(
                            "retained {group} requires a valid canonical name and engineering unit"
                        ));
                    }
                    if !names.insert(value.name.as_str()) {
                        return Err(format!("retained {group} '{}' is duplicated", value.name));
                    }
                    if !value.value.is_finite() {
                        return Err(format!(
                            "retained {group} '{}' contains a non-finite value",
                            value.name
                        ));
                    }
                }
            }
        }
        if let Some(report) = &self.device_op {
            if !report.labels_resolve() {
                return Err("retained device operating-point labels do not resolve".to_owned());
            }
            let mut devices = HashSet::with_capacity(report.entries.len());
            for entry in &report.entries {
                if !valid_text(&entry.name) || !devices.insert(entry.name.as_str()) {
                    return Err(format!(
                        "retained device operating-point identity '{}' is invalid or duplicated",
                        entry.name
                    ));
                }
                if entry.params.iter().any(|(_, value)| !value.is_finite()) {
                    return Err(format!(
                        "retained device operating-point entry '{}' contains a non-finite value",
                        entry.name
                    ));
                }
            }
        }
        if let Some(noise) = &self.noise_summary {
            if let Some(quantity) = noise.input_quantity
                && (self.waveforms.iter().map(AsRef::as_ref).any(|wave| {
                    wave.name == "inoise" && wave.unit.as_deref() != Some(quantity.density_unit())
                }) || (noise.noise_figure.is_some()
                    && quantity != rspice_core::analysis::noise::NoiseInputQuantity::Voltage))
            {
                return Err("Noise input quantity disagrees with its retained evidence".into());
            }
            if let Some(conversion) = &noise.conversion {
                conversion.validate(noise.band)?;
                if let Some(sampling) = &conversion.sampling {
                    let timing = sampling.request.is_timing();
                    let unit = if timing { "s²/Hz" } else { "V²/Hz" };
                    if self.analysis_type != AnalysisType::Pnoise
                        || noise.noise_figure.is_some()
                        || (timing && noise.total_rms.is_some())
                        || self
                            .waveforms
                            .iter()
                            .map(AsRef::as_ref)
                            .filter(|wave| wave.name == "onoise" || wave.name.starts_with("noise("))
                            .any(|wave| wave.unit.as_deref() != Some(unit))
                    {
                        return Err("Sampled noise units or result family disagree with the retained sampling evidence".into());
                    }
                }
                if conversion.input_source.trim().is_empty()
                    && (self.analysis_type == AnalysisType::Hbnoise
                        || self
                            .waveforms
                            .iter()
                            .map(AsRef::as_ref)
                            .any(|wave| wave.name == "inoise"))
                {
                    return Err(
                        "Input-referred periodic noise requires a retained input source".into(),
                    );
                }
                if !matches!(
                    self.analysis_type,
                    AnalysisType::Hbnoise | AnalysisType::Pnoise
                ) {
                    return Err("Conversion channels require a periodic-noise result".into());
                }
                if noise
                    .noise_figure
                    .as_ref()
                    .is_some_and(|figure| figure.input_source != conversion.input_source)
                {
                    return Err(
                        "Noise figure and conversion channels have different input sources".into(),
                    );
                }
                if self.waveforms.is_empty()
                    || self.waveforms.iter().map(AsRef::as_ref).any(|wave| {
                        wave.x.first().copied() != Some(noise.band.0)
                            || wave.x.last().copied() != Some(noise.band.1)
                    })
                {
                    return Err(
                        "Periodic-noise curves must cover their retained offset band".into(),
                    );
                }
            }
            if let Some(figure) = &noise.noise_figure {
                figure.validate()?;
                let mut curves = self
                    .waveforms
                    .iter()
                    .map(AsRef::as_ref)
                    .filter(|wave| wave.name == "Noise figure (SSB)");
                let matches = curves.next().is_some_and(|wave| {
                    wave.unit.as_deref() == Some("dB")
                        && wave.x.as_slice() == figure.frequencies.as_slice()
                        && wave.y.as_slice() == figure.decibels.as_slice()
                });
                if self.analysis_type != AnalysisType::Hbnoise
                    || !matches
                    || curves.next().is_some()
                {
                    return Err("Noise figure must match its retained HBNOISE decibel curve".into());
                }
                if noise.band
                    != (
                        *figure.frequencies.first().unwrap(),
                        *figure.frequencies.last().unwrap(),
                    )
                {
                    return Err("Noise figure does not cover the retained noise band".into());
                }
            }
            if !noise.band.0.is_finite()
                || !noise.band.1.is_finite()
                || noise.band.0 < 0.0
                || noise.band.1 < noise.band.0
                || noise
                    .total_rms
                    .is_some_and(|value| !value.is_finite() || value < 0.0)
                || noise
                    .input_rms
                    .is_some_and(|value| !value.is_finite() || value < 0.0)
            {
                return Err("retained noise summary has an invalid band or RMS total".to_owned());
            }
            let mut contributors = HashSet::with_capacity(noise.rows.len());
            for row in &noise.rows {
                if !valid_text(&row.device)
                    || !valid_text(&row.mechanism)
                    || !row.power.is_finite()
                    || row.power < 0.0
                    || !row.share_pct.is_finite()
                    || !(0.0..=100.0).contains(&row.share_pct)
                    || !contributors.insert((row.device.as_str(), row.mechanism.as_str()))
                {
                    return Err(format!(
                        "retained noise contribution '{} / {}' is invalid or duplicated",
                        row.device, row.mechanism
                    ));
                }
            }
        }
        let mut measurement_names = HashSet::with_capacity(self.measurements.len());
        for measurement in &self.measurements {
            if let Some(units) = &measurement.units {
                units.validate()?;
            }
            if !valid_text(&measurement.name)
                || !measurement_names.insert(measurement.name.as_str())
            {
                return Err(format!(
                    "retained measurement identity '{}' is invalid or duplicated",
                    measurement.name
                ));
            }
            if [
                measurement.value,
                measurement.raw_value,
                measurement.expected,
                measurement.tolerance,
                measurement.failure_limit,
                measurement.event_axis,
            ]
            .into_iter()
            .flatten()
            .any(|value| !value.is_finite())
                || measurement.tolerance.is_some_and(|value| value < 0.0)
                || (measurement.passed
                    && (measurement.value.is_none() || measurement.error.is_some()))
            {
                return Err(format!(
                    "retained measurement '{}' has contradictory or non-finite evidence",
                    measurement.name
                ));
            }
            if measurement.value.is_some() != measurement.raw_value.is_some() {
                return Err(format!(
                    "retained measurement '{}' must carry its raw value exactly when it carries a published value",
                    measurement.name
                ));
            }
            let expected_exceeded = match (measurement.raw_value, measurement.failure_limit) {
                (Some(raw_value), Some(limit)) => raw_value.abs() >= limit,
                _ => false,
            };
            if measurement.failure_limit_exceeded != expected_exceeded {
                return Err(format!(
                    "retained measurement '{}' FAILVALUE verdict does not match abs(raw_value) >= failure_limit",
                    measurement.name
                ));
            }
            if measurement.failure_limit_exceeded && measurement.passed {
                return Err(format!(
                    "retained measurement '{}' cannot pass after its FAILVALUE limit was reached",
                    measurement.name
                ));
            }
        }

        if let Some(metadata) = &self.family_metadata {
            metadata.validate_for(self.analysis_type)?;
        }
        if let Some(payload) = &self.result_payload {
            payload.validate_for(self.analysis_type)?;
        }
        self.validate_retained_display_basis(retained_basis.as_deref())?;
        if let Some(AnalysisResultPayload::DcSweep { evidence }) = &self.result_payload {
            evidence.validate_retained_traces(self.waveforms.iter().map(AsRef::as_ref).map(
                |trace| crate::dc_sweep::DcTraceView {
                    name: &trace.name,
                    unit: trace.unit.as_deref(),
                    x: &trace.x,
                    sample_count: trace.y.len(),
                    complex: trace.complex.is_some(),
                },
            ))?;
        }

        match (&self.family_metadata, &self.result_payload) {
            (None, Some(AnalysisResultPayload::Soa { .. })) => {
                return Err("SOA payload is missing its retained time axis".to_owned());
            }
            (
                Some(AnalysisResultFamilyMetadata::Soa { time }),
                Some(AnalysisResultPayload::Soa {
                    source_history,
                    evaluations,
                    violations,
                }),
            ) => {
                if let Some(source) = source_history {
                    soa_source::validate_report(source, time, &self.waveforms)?;
                    soa_reporting::validate(self, time, evaluations, violations)?;
                }
                let rules = evaluations
                    .iter()
                    .map(|evaluation| {
                        (
                            (evaluation.device_id.as_str(), evaluation.parameter),
                            evaluation,
                        )
                    })
                    .collect::<std::collections::BTreeMap<_, _>>();
                let mut exact_worst_events = std::collections::BTreeSet::new();
                let mut derating_event_counts = std::collections::BTreeMap::<_, usize>::new();
                let mut derating_traces = std::collections::BTreeMap::new();
                for evaluation in evaluations.iter().filter(|e| {
                    e.derating.is_some() || e.envelope.is_some() || e.duration.is_some()
                }) {
                    let duration = evaluation
                        .duration
                        .map(|_| soa_duration::validate(self, time, evaluation))
                        .transpose()?;
                    let mut events = duration.as_ref().map_or(0, |duration| duration.events);
                    let limits = if evaluation.envelope.is_some() {
                        let raw_events = soa_envelope::validate(self, time, evaluation)?;
                        if duration.is_none() {
                            events = raw_events;
                        }
                        Some(soa_derating::trace(
                            self,
                            &crate::safety::soa_envelope_limit_waveform_name(
                                &evaluation.device_id,
                                evaluation.parameter.runtime_parameter(),
                            ),
                            time,
                            "A",
                        )?)
                    } else if evaluation.derating.is_some() {
                        let raw_events = soa_derating::validate(self, time, evaluation)?;
                        if duration.is_none() {
                            events = raw_events;
                        }
                        Some(soa_derating::trace(
                            self,
                            &crate::safety::soa_power_limit_waveform_name(&evaluation.device_id),
                            time,
                            "W",
                        )?)
                    } else {
                        None
                    };
                    let stress = soa_derating::trace(
                        self,
                        &crate::safety::soa_stress_waveform_name(
                            &evaluation.device_id,
                            evaluation.parameter.runtime_parameter(),
                        ),
                        time,
                        &evaluation.unit,
                    )?;
                    derating_traces.insert(
                        (evaluation.device_id.as_str(), evaluation.parameter),
                        (limits, stress, events, duration),
                    );
                }
                for violation in violations {
                    let key = (violation.device_id.as_str(), violation.parameter);
                    let evaluation = rules.get(&key).ok_or_else(|| {
                        format!(
                            "SOA event for '{}' has no matching evaluated rule",
                            violation.device_id
                        )
                    })?;
                    let expected_limit = if let Some((limits, stress, _, duration)) =
                        derating_traces.get(&key)
                    {
                        let index = time
                            .binary_search_by(|t| t.total_cmp(&violation.time_s))
                            .map_err(|_| "SOA derating event has no exact retained sample")?;
                        if !same_retained_float(stress[index], violation.actual_value) {
                            return Err(
                                "SOA derating event contradicts its retained stress sample".into(),
                            );
                        }
                        if duration.as_ref().is_some_and(|duration| {
                            soa_duration::severity(duration.verdicts[index])
                                != Some(violation.severity)
                        }) {
                            return Err(
                                "SOA event contradicts its duration-qualified sample severity"
                                    .into(),
                            );
                        }
                        *derating_event_counts.entry(key).or_default() += 1;
                        limits.map_or(evaluation.limit_value, |values| values[index])
                    } else {
                        evaluation.limit_value
                    };
                    if !same_retained_float(violation.limit_value, expected_limit) {
                        return Err(format!(
                            "SOA event for '{}' contradicts its evaluated rule limit",
                            violation.device_id
                        ));
                    }
                    if evaluation.duration.is_none()
                        && crate::safety::compare_soa_stress(
                            violation.actual_value,
                            violation.limit_value,
                            evaluation.worst_actual_value,
                            evaluation.limit_value,
                        )
                        .is_gt()
                    {
                        return Err(format!(
                            "SOA event for '{}' exceeds its retained worst point",
                            violation.device_id
                        ));
                    }
                    if !contains_retained_coordinate(time, violation.time_s) {
                        return Err(format!(
                            "SOA event for '{}' does not reference an exact retained sample",
                            violation.device_id
                        ));
                    }
                    let expected_severity = match evaluation.verdict {
                        SoaRuleVerdictEvidence::Pass => None,
                        SoaRuleVerdictEvidence::Warning => {
                            Some(SoaViolationSeverityEvidence::Warning)
                        }
                        SoaRuleVerdictEvidence::Violation => {
                            Some(SoaViolationSeverityEvidence::Violation)
                        }
                        SoaRuleVerdictEvidence::Critical => {
                            Some(SoaViolationSeverityEvidence::Critical)
                        }
                    };
                    if expected_severity == Some(violation.severity)
                        && same_retained_float(
                            violation.actual_value,
                            evaluation.worst_actual_value,
                        )
                        && same_retained_float(violation.time_s, evaluation.worst_time_s)
                    {
                        exact_worst_events.insert(key);
                    }
                }
                for evaluation in evaluations {
                    if evaluation.derating.is_some()
                        || evaluation.envelope.is_some()
                        || evaluation.duration.is_some()
                    {
                        let key = (evaluation.device_id.as_str(), evaluation.parameter);
                        let expected_events = derating_traces[&key].2;
                        if derating_event_counts.get(&key).copied().unwrap_or(0) != expected_events
                        {
                            return Err("SOA derating events do not cover every retained warning/violation sample".into());
                        }
                    }
                    let retained_sample_count = u64::try_from(time.len())
                        .map_err(|_| "SOA time axis exceeds the retained count range".to_owned())?;
                    if evaluation.sample_count != retained_sample_count {
                        return Err(format!(
                            "SOA evaluation for '{}' covers {} samples but the retained run has {}",
                            evaluation.device_id,
                            evaluation.sample_count,
                            time.len()
                        ));
                    }
                    if !contains_retained_coordinate(time, evaluation.worst_time_s) {
                        return Err(format!(
                            "SOA evaluation for '{}' does not reference an exact retained sample",
                            evaluation.device_id
                        ));
                    }
                    let key = (evaluation.device_id.as_str(), evaluation.parameter);
                    if evaluation.verdict != SoaRuleVerdictEvidence::Pass
                        && !exact_worst_events.contains(&key)
                    {
                        return Err(format!(
                            "SOA evaluation for '{}' has no exact event at its worst point",
                            evaluation.device_id
                        ));
                    }
                }
            }
            (Some(AnalysisResultFamilyMetadata::Soa { .. }), Some(payload))
                if !matches!(payload, AnalysisResultPayload::Soa { .. }) =>
            {
                return Err("SOA metadata has a mismatched retained payload".to_owned());
            }
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
