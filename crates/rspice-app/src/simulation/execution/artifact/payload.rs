//! Serializable numerical payloads and exact digests for execution artifacts.

use super::*;
mod dependencies;
pub(in crate::simulation) use dependencies::ResolvedExecutionDependencies;
mod qpss;
pub(in crate::simulation) use qpss::QpssStateArtifact;

#[cfg(any(target_arch = "wasm32", test))]
pub(in crate::simulation) type EncodedArtifactTransfer<'a> =
    (String, Vec<std::borrow::Cow<'a, [f64]>>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(in crate::simulation) struct TransientTrajectoryArtifact {
    #[serde(with = "f64_bits_vec")]
    time: Vec<f64>,
    #[serde(default)]
    current_impulses: Option<crate::state::CurrentImpulseHistoryEvidence>,
    #[serde(with = "f64_bits_map")]
    waveforms: BTreeMap<String, Vec<f64>>,
    #[serde(default)]
    convergence: Option<Arc<crate::state::TransientConvergenceEvidence>>,
    /// Spectra the producing solve recorded for the `.fft` cards it carried.
    ///
    /// They ride this envelope rather than a sibling artifact kind because the
    /// controller holds one artifact per producer: a transient with both a
    /// Fourier and an FFT dependent has to hand both from one payload.
    /// Defaulted, so an artifact without spectra is byte-identical to one
    /// produced before they existed — including its digest.
    #[serde(default)]
    spectra: Vec<Arc<crate::simulation::results::RecordedFftSpectrum>>,
}

mod f64_bits_vec {
    use serde::ser::SerializeSeq;
    use serde::{Deserialize, Deserializer, Serializer, de::Error as _};

    pub(super) fn serialize<S>(values: &[f64], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut sequence = serializer.serialize_seq(Some(values.len()))?;
        for value in values {
            sequence.serialize_element(&format!("{:016x}", value.to_bits()))?;
        }
        sequence.end()
    }

    pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::<String>::deserialize(deserializer)?
            .into_iter()
            .map(|encoded| {
                u64::from_str_radix(&encoded, 16)
                    .map(f64::from_bits)
                    .map_err(|_| D::Error::custom("invalid exact f64 bit pattern"))
            })
            .collect()
    }
}

mod f64_bits_map {
    use std::collections::BTreeMap;

    use serde::ser::{SerializeMap, SerializeSeq};
    use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};

    struct BitSlice<'a>(&'a [f64]);

    impl Serialize for BitSlice<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;
            for value in self.0 {
                sequence.serialize_element(&format!("{:016x}", value.to_bits()))?;
            }
            sequence.end()
        }
    }

    pub(super) fn serialize<S>(
        waveforms: &BTreeMap<String, Vec<f64>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(waveforms.len()))?;
        for (name, values) in waveforms {
            map.serialize_entry(name, &BitSlice(values))?;
        }
        map.end()
    }

    pub(super) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<BTreeMap<String, Vec<f64>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        BTreeMap::<String, Vec<String>>::deserialize(deserializer)?
            .into_iter()
            .map(|(name, encoded_values)| {
                let values = encoded_values
                    .into_iter()
                    .map(|encoded| {
                        u64::from_str_radix(&encoded, 16)
                            .map(f64::from_bits)
                            .map_err(|_| D::Error::custom("invalid exact f64 bit pattern"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok((name, values))
            })
            .collect()
    }
}

impl TransientTrajectoryArtifact {
    /// Project a fresh local solve through the same payload checks used for
    /// authenticated task handoffs. This does not create a task identity.
    pub(in crate::simulation) fn from_result(
        result: &SimulationResult,
        required_waveforms: &[String],
        carry_spectra: bool,
    ) -> Result<Option<Self>, ExecutionArtifactError> {
        let SimulationResult::Transient {
            time,
            waveforms,
            convergence,
            spectra,
            events,
            ..
        } = result
        else {
            return Ok(None);
        };
        // An FFT consumer reads no waveform, so a request that asks only for
        // the recorded spectra is a complete request.
        if required_waveforms.is_empty() && !carry_spectra {
            return Err(ExecutionArtifactError::InvalidPayload(
                "transient artifact request contains no required waveforms".to_owned(),
            ));
        }
        let mut artifact_waveforms = BTreeMap::new();
        for (name, waveform) in waveforms {
            if !required_waveforms
                .iter()
                .any(|required| normalize_waveform_name(required) == normalize_waveform_name(name))
            {
                continue;
            }
            if waveform.is_complex || waveform.y_imag.is_some() {
                return Err(ExecutionArtifactError::InvalidPayload(format!(
                    "transient waveform '{name}' unexpectedly contains complex values"
                )));
            }
            if waveform.x_values.len() != time.len()
                || waveform
                    .x_values
                    .iter()
                    .zip(time)
                    .any(|(waveform_time, common_time)| {
                        waveform_time.to_bits() != common_time.to_bits()
                    })
            {
                return Err(ExecutionArtifactError::InvalidPayload(format!(
                    "transient waveform '{name}' does not use the result's canonical time axis"
                )));
            }
            if artifact_waveforms
                .insert(name.clone(), waveform.y_values.clone())
                .is_some()
            {
                return Err(ExecutionArtifactError::InvalidPayload(format!(
                    "transient trajectory repeats waveform '{name}'"
                )));
            }
        }
        for required in required_waveforms {
            if !artifact_waveforms
                .keys()
                .any(|name| normalize_waveform_name(name) == normalize_waveform_name(required))
            {
                return Err(ExecutionArtifactError::InvalidPayload(format!(
                    "required transient waveform '{required}' is absent from the producer result"
                )));
            }
        }
        // Two FFT instances with identical requests put the same card in the
        // deck twice, so the engine returns the same spectrum twice. They are
        // equal numbers under one key, and one copy is what the artifact holds.
        let mut carried: Vec<Arc<crate::simulation::results::RecordedFftSpectrum>> = Vec::new();
        if carry_spectra {
            for spectrum in spectra {
                if !carried
                    .iter()
                    .any(|held| held.request_key == spectrum.request_key)
                {
                    carried.push(Arc::clone(spectrum));
                }
            }
        }
        let trajectory = TransientTrajectoryArtifact {
            time: time.clone(),
            current_impulses: events.current_impulses.clone(),
            waveforms: artifact_waveforms,
            convergence: convergence.clone(),
            spectra: carried,
        };
        trajectory.validate()?;
        Ok(Some(trajectory))
    }

    pub(in crate::simulation) fn convergence(
        &self,
    ) -> Option<&Arc<crate::state::TransientConvergenceEvidence>> {
        self.convergence.as_ref()
    }

    pub(in crate::simulation) fn time(&self) -> &[f64] {
        &self.time
    }

    pub(in crate::simulation) fn waveform(&self, requested: &str) -> Option<&[f64]> {
        let requested = normalize_waveform_name(requested);
        self.waveforms
            .iter()
            .find(|(name, _)| normalize_waveform_name(name) == requested)
            .map(|(_, values)| values.as_slice())
    }

    pub(in crate::simulation) fn current_impulse_trace(
        &self,
        requested: &str,
    ) -> Result<Option<&rspice_core::CurrentImpulseTrace>, String> {
        let Some(history) = &self.current_impulses else {
            return Ok(None);
        };
        if !history.delivery_complete {
            return Err("Fourier current requires complete impulse delivery".into());
        }
        let canonical = |name: &str| name.trim().replace(':', ".").to_ascii_uppercase();
        let mut matches = history
            .traces
            .iter()
            .filter(|trace| canonical(&trace.owner.to_string()) == canonical(requested));
        let trace = matches.next().ok_or_else(|| {
            format!("Fourier current '{requested}' has no complete impulse history")
        })?;
        if matches.next().is_some() {
            return Err(format!(
                "Fourier current '{requested}' has ambiguous impulse history"
            ));
        }
        Ok(Some(trace))
    }

    /// The spectrum whose request key is `key`, if this solve recorded one.
    pub(in crate::simulation) fn spectrum(
        &self,
        key: &str,
    ) -> Option<&Arc<crate::simulation::results::RecordedFftSpectrum>> {
        self.spectra
            .iter()
            .find(|spectrum| spectrum.request_key == key)
    }

    fn validate(&self) -> Result<(), ExecutionArtifactError> {
        let numeric_values = self
            .waveforms
            .values()
            .fold(self.time.len(), |total, values| {
                total.saturating_add(values.len())
            })
            .saturating_add(self.convergence.as_deref().map_or(
                0,
                crate::state::TransientConvergenceEvidence::transfer_value_count,
            ))
            .saturating_add(self.current_impulses.as_ref().map_or(0, |history| {
                history.traces.iter().fold(2usize, |sum, trace| {
                    sum.saturating_add(trace.points.len().saturating_mul(2))
                })
            }))
            .saturating_add(
                self.spectra
                    .iter()
                    .map(|spectrum| spectrum.numeric_value_count())
                    .sum(),
            );
        if numeric_values > PeriodicStateArtifact::MAX_NUMERIC_VALUES {
            return Err(ExecutionArtifactError::InvalidPayload(
                "Transient trajectory and convergence evidence exceed the numeric payload limit"
                    .to_owned(),
            ));
        }
        if let Some(quality) = &self.convergence {
            quality
                .validate()
                .map_err(ExecutionArtifactError::InvalidPayload)?;
        }
        if self.time.len() < 3 {
            return Err(ExecutionArtifactError::InvalidPayload(
                "transient trajectory contains fewer than three samples".to_owned(),
            ));
        }
        if self
            .time
            .iter()
            .any(|sample| !sample.is_finite() || *sample < 0.0)
        {
            return Err(ExecutionArtifactError::InvalidPayload(
                "transient trajectory time axis contains a non-finite or negative sample"
                    .to_owned(),
            ));
        }
        if self.time.windows(2).any(|pair| pair[1] <= pair[0]) {
            return Err(ExecutionArtifactError::InvalidPayload(
                "transient trajectory time axis is not strictly increasing".to_owned(),
            ));
        }
        if let Some(history) = &self.current_impulses {
            history
                .validate()
                .map_err(ExecutionArtifactError::InvalidPayload)?;
            if history.start_time_s > self.time[0]
                || history.stop_time_s < self.time[self.time.len() - 1]
            {
                return Err(ExecutionArtifactError::InvalidPayload(
                    "current impulse history does not cover the trajectory".into(),
                ));
            }
        }
        // A trajectory with no waveform is meaningful only when it carries
        // something else this solve produced. A recorded spectrum is exactly
        // that: an FFT consumer reads no waveform at all.
        if self.waveforms.is_empty() && self.spectra.is_empty() {
            return Err(ExecutionArtifactError::InvalidPayload(
                "transient trajectory contains no waveforms".to_owned(),
            ));
        }
        let mut keys = std::collections::BTreeSet::new();
        for spectrum in &self.spectra {
            spectrum
                .validate()
                .map_err(ExecutionArtifactError::InvalidPayload)?;
            if !keys.insert(spectrum.request_key.clone()) {
                return Err(ExecutionArtifactError::InvalidPayload(format!(
                    "transient trajectory repeats recorded FFT request '{}'",
                    spectrum.request_key
                )));
            }
        }
        for (name, values) in &self.waveforms {
            if name.trim().is_empty() {
                return Err(ExecutionArtifactError::InvalidPayload(
                    "transient trajectory contains an unnamed waveform".to_owned(),
                ));
            }
            if values.len() != self.time.len() {
                return Err(ExecutionArtifactError::InvalidPayload(format!(
                    "transient waveform '{name}' has {} values for {} time samples",
                    values.len(),
                    self.time.len()
                )));
            }
            if values.iter().any(|value| !value.is_finite()) {
                return Err(ExecutionArtifactError::InvalidPayload(format!(
                    "transient waveform '{name}' contains a non-finite sample"
                )));
            }
        }
        Ok(())
    }

    fn digest(&self) -> ContentDigest {
        let mut writer = CanonicalWriter::new("rspice.transient-trajectory-artifact/v2");
        writer.option(self.convergence.as_ref(), |writer, quality| {
            quality.encode(writer)
        });
        writer.sequence(self.time.len());
        for value in &self.time {
            writer.f64(*value);
        }
        writer.sequence(self.waveforms.len());
        for (name, values) in &self.waveforms {
            writer.string(name);
            writer.sequence(values.len());
            for value in values {
                writer.f64(*value);
            }
        }
        // A conditional tail, so a trajectory that recorded no spectrum
        // digests exactly as it did before spectra existed.
        if !self.spectra.is_empty() {
            writer.domain("recorded-fft-spectra");
            writer.sequence(self.spectra.len());
            for spectrum in &self.spectra {
                writer.string(&spectrum.request_key);
                for column in [&spectrum.frequency, &spectrum.real, &spectrum.imaginary] {
                    writer.sequence(column.len());
                    for value in column {
                        writer.f64(*value);
                    }
                }
            }
        }
        if let Some(history) = &self.current_impulses {
            writer.domain("current-impulse-history-v1");
            writer.f64(history.start_time_s);
            writer.f64(history.stop_time_s);
            writer.bool(history.delivery_complete);
            writer.sequence(history.traces.len());
            for trace in &history.traces {
                match &trace.owner {
                    rspice_core::CurrentImpulseOwner::Branch { branch_name } => {
                        writer.u8(0);
                        writer.string(branch_name);
                    }
                    rspice_core::CurrentImpulseOwner::DeviceLead {
                        device_name,
                        parameter,
                    } => {
                        writer.u8(1);
                        writer.string(device_name);
                        writer.string(parameter);
                    }
                }
                writer.bool(trace.complete);
                writer.sequence(trace.points.len());
                for point in &trace.points {
                    writer.f64(point.time);
                    writer.f64(point.charge_coulombs);
                }
            }
        }
        writer.finish()
    }
}

/// Exact DC operating-point state consumed by a bound steady-state task.
///
/// The payload retains the core MNA basis rather than presentation maps. Its
/// source identity covers the circuit deck after process-corner materialization
/// and before analysis-local numerical options. Voltage scaling and temperature
/// let periodic solvers reproduce the OP physical environment exactly once.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(in crate::simulation) struct DcOperatingPointSeedArtifact {
    effective_source_content_digest: ContentDigest,
    temperature_celsius: f64,
    supply_voltage: Option<f64>,
    nominal_supply_voltage: Option<f64>,
    #[serde(default)]
    supply_source_names: Vec<String>,
    node_names: Vec<String>,
    branch_names: Vec<String>,
    #[serde(with = "f64_bits_vec")]
    solution: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::simulation) struct PeriodicOperatingEnvironment {
    source_basis_digest: ContentDigest,
    temperature_celsius: f64,
    supply_voltage: Option<f64>,
    nominal_supply_voltage: Option<f64>,
    supply_source_names: Vec<String>,
}

impl PeriodicOperatingEnvironment {
    pub(super) const fn temperature_celsius(&self) -> f64 {
        self.temperature_celsius
    }

    fn validate(&self) -> Result<(), ExecutionArtifactError> {
        if !self.temperature_celsius.is_finite() || self.temperature_celsius <= -273.15 {
            return Err(ExecutionArtifactError::InvalidPayload(
                "Periodic temperature must be finite and above absolute zero".into(),
            ));
        }
        match (self.supply_voltage, self.nominal_supply_voltage) {
            (None, None) => {}
            (Some(supply), Some(nominal))
                if supply.is_finite()
                    && supply > 0.0
                    && nominal.is_finite()
                    && nominal > 0.0
                    && !self.supply_source_names.is_empty() => {}
            _ => {
                return Err(ExecutionArtifactError::InvalidPayload(
                    "Periodic supply point must pair positive voltages with explicit source names"
                        .into(),
                ));
            }
        }
        Ok(())
    }

    fn encode(&self, writer: &mut CanonicalWriter) {
        writer.digest(self.source_basis_digest);
        writer.f64(self.temperature_celsius);
        writer.option(self.supply_voltage.as_ref(), |writer, value| {
            writer.f64(*value)
        });
        writer.option(self.nominal_supply_voltage.as_ref(), |writer, value| {
            writer.f64(*value)
        });
        writer.sequence(self.supply_source_names.len());
        for name in &self.supply_source_names {
            writer.string(name);
        }
    }

    pub(in crate::simulation) fn materialize(
        &self,
        source: &str,
        source_path: Option<&std::path::Path>,
        dependencies: &ResolvedExecutionDependencies,
        abort: &dyn rspice_core::abort_signal::AbortSignal,
    ) -> crate::services::simulation_runner::ServiceRunResult<rspice_core::Netlist> {
        use crate::services::simulation_runner as services;
        self.validate()
            .map_err(|error| services::ServiceRunError::Failure(error.to_string()))?;
        dependencies
            .validate_source_basis(source, self.source_basis_digest)
            .map_err(|error| services::ServiceRunError::Failure(error.to_string()))?;
        let temperature_source = services::source_with_run_temperature_with_abort(
            source,
            self.temperature_celsius,
            abort,
        )?;
        let mut circuit =
            services::parse_runner_netlist_with_abort(&temperature_source, source_path, abort)?;
        circuit.source_text = Some(source.to_owned());
        if let (Some(supply), Some(nominal)) = (self.supply_voltage, self.nominal_supply_voltage) {
            services::apply_voltage_corner(
                &mut circuit,
                supply,
                nominal,
                &self.supply_source_names,
                abort,
            )?;
        }
        circuit.options.temp = Some(self.temperature_celsius);
        Ok(circuit)
    }
}

impl DcOperatingPointSeedArtifact {
    pub(in crate::simulation) fn environment(&self) -> PeriodicOperatingEnvironment {
        PeriodicOperatingEnvironment {
            source_basis_digest: self.effective_source_content_digest,
            temperature_celsius: self.temperature_celsius,
            supply_voltage: self.supply_voltage,
            nominal_supply_voltage: self.nominal_supply_voltage,
            supply_source_names: self.supply_source_names.clone(),
        }
    }

    pub(in crate::simulation) const fn effective_source_content_digest(&self) -> ContentDigest {
        self.effective_source_content_digest
    }

    pub(in crate::simulation) const fn temperature_celsius(&self) -> f64 {
        self.temperature_celsius
    }

    pub(in crate::simulation) const fn supply_voltage(&self) -> Option<f64> {
        self.supply_voltage
    }

    pub(in crate::simulation) const fn nominal_supply_voltage(&self) -> Option<f64> {
        self.nominal_supply_voltage
    }

    pub(in crate::simulation) fn supply_source_names(&self) -> &[String] {
        &self.supply_source_names
    }

    pub(in crate::simulation) fn core_seed(
        &self,
    ) -> Result<rspice_core::engine::PssDcOperatingPointSeed, ExecutionArtifactError> {
        rspice_core::engine::PssDcOperatingPointSeed::try_new(
            self.node_names.clone(),
            self.branch_names.clone(),
            self.solution.clone(),
        )
        .map_err(|error| ExecutionArtifactError::InvalidPayload(error.to_string()))
    }

    fn validate(&self) -> Result<(), ExecutionArtifactError> {
        if !self.temperature_celsius.is_finite() || self.temperature_celsius <= -273.15 {
            return Err(ExecutionArtifactError::InvalidPayload(
                "DC operating-point seed temperature must be finite and above absolute zero"
                    .to_owned(),
            ));
        }
        match (self.supply_voltage, self.nominal_supply_voltage) {
            (None, None) => {}
            (Some(supply), Some(nominal))
                if supply.is_finite() && supply > 0.0 && nominal.is_finite() && nominal > 0.0 => {}
            _ => {
                return Err(ExecutionArtifactError::InvalidPayload(
                    "DC operating-point seed supply and nominal voltages must be paired positive finite values"
                        .to_owned(),
                ));
            }
        }
        if self.supply_voltage.is_some() && self.supply_source_names.is_empty() {
            return Err(ExecutionArtifactError::InvalidPayload(
                "DC operating-point seed supply requires explicitly bound source instances"
                    .to_owned(),
            ));
        }
        self.core_seed().map(|_| ())
    }

    fn digest(&self) -> ContentDigest {
        let mut writer = CanonicalWriter::new("rspice.dc-operating-point-seed-artifact/v1");
        writer.digest(self.effective_source_content_digest);
        writer.f64(self.temperature_celsius);
        writer.option(self.supply_voltage.as_ref(), |writer, value| {
            writer.f64(*value);
        });
        writer.option(self.nominal_supply_voltage.as_ref(), |writer, value| {
            writer.f64(*value);
        });
        writer.sequence(self.supply_source_names.len());
        for source in &self.supply_source_names {
            writer.string(source);
        }
        writer.sequence(self.node_names.len());
        for name in &self.node_names {
            writer.string(name);
        }
        writer.sequence(self.branch_names.len());
        for name in &self.branch_names {
            writer.string(name);
        }
        writer.sequence(self.solution.len());
        for value in &self.solution {
            writer.f64(*value);
        }
        writer.finish()
    }
}

/// Complete numerical shooting state consumed by PSS-dependent analyses.
/// The payload is immutable and its digest covers the orbit, monodromy,
/// Floquet data, and reactive phase-origin state bit-for-bit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(in crate::simulation) struct PeriodicStateArtifact {
    #[serde(default)]
    environment: Option<PeriodicOperatingEnvironment>,
    operating_point: Arc<rspice_core::engine::PssOperatingPoint>,
    result_floquet_real: Vec<f64>,
    result_floquet_imag: Vec<f64>,
    #[serde(default)]
    floquet_evidence: rspice_core::analysis::FloquetSpectrumEvidence,
    #[serde(default)]
    floquet_orbit_kind: rspice_core::analysis::FloquetOrbitKind,
    #[serde(default)]
    trivial_floquet_multiplier_index: Option<usize>,
    #[serde(default = "indeterminate_floquet_verdict")]
    floquet_verdict: rspice_core::analysis::FloquetStabilityVerdict,
    #[serde(default)]
    floquet_authenticated: bool,
    analysis_floquet_real: Vec<f64>,
    analysis_floquet_imag: Vec<f64>,
    #[serde(default)]
    analysis_is_stable: bool,
}

fn indeterminate_floquet_verdict() -> rspice_core::analysis::FloquetStabilityVerdict {
    rspice_core::analysis::FloquetStabilityVerdict::Indeterminate
}

fn pss_floquet_contract_is_authenticated(
    result: &rspice_core::analysis::pss::PssResult,
    monodromy_order: usize,
) -> bool {
    if !result.has_consistent_floquet_contract() {
        return false;
    }
    match &result.floquet_evidence {
        rspice_core::analysis::FloquetSpectrumEvidence::Qualified { certificate } => {
            certificate.is_valid()
                && monodromy_order > 0
                && certificate.problem_order == monodromy_order
                && certificate.problem_order == result.floquet_multipliers.len()
        }
        rspice_core::analysis::FloquetSpectrumEvidence::NoDynamicModes => {
            monodromy_order == 0
                && result.floquet_multipliers.is_empty()
                && result.floquet_orbit_kind == rspice_core::analysis::FloquetOrbitKind::Driven
        }
        _ => false,
    }
}

impl PeriodicStateArtifact {
    const MAX_NUMERIC_VALUES: usize = 16_777_216;

    pub(in crate::simulation) fn operating_point(&self) -> &rspice_core::engine::PssOperatingPoint {
        &self.operating_point
    }

    pub(in crate::simulation) fn materialize_consumer(
        &self,
        source: &str,
        source_path: Option<&std::path::Path>,
        dependencies: &ResolvedExecutionDependencies,
        abort: &dyn rspice_core::abort_signal::AbortSignal,
    ) -> crate::services::simulation_runner::ServiceRunResult<rspice_core::Netlist> {
        match &self.environment {
            Some(environment) => environment.materialize(source, source_path, dependencies, abort),
            None => crate::services::simulation_runner::parse_runner_netlist_with_abort(
                source,
                source_path,
                abort,
            ),
        }
    }

    pub(in crate::simulation) fn validate_consumer_basis(
        &self,
        consumer: &str,
        fundamental_freq: f64,
        num_harmonics: usize,
        tolerance: f64,
        require_autonomous: bool,
    ) -> Result<(), ExecutionArtifactError> {
        Self::validate_operating_point_consumer_basis(
            &self.operating_point,
            consumer,
            fundamental_freq,
            num_harmonics,
            tolerance,
            require_autonomous,
        )
    }

    pub(in crate::simulation) fn validate_operating_point_consumer_basis(
        point: &rspice_core::engine::PssOperatingPoint,
        consumer: &str,
        fundamental_freq: f64,
        num_harmonics: usize,
        tolerance: f64,
        require_autonomous: bool,
    ) -> Result<(), ExecutionArtifactError> {
        let config = point.config();
        let producer_frequency = if config.is_autonomous() {
            1.0 / config.period_guess
        } else {
            config.fundamental_freq
        };
        if producer_frequency.to_bits() != fundamental_freq.to_bits()
            || config.num_harmonics != num_harmonics
            || config.tolerance.to_bits() != tolerance.to_bits()
        {
            return Err(ExecutionArtifactError::ContractMismatch(format!(
                "{consumer} periodic basis ({fundamental_freq:.16e} Hz, {num_harmonics} harmonics, tolerance {tolerance:.16e}) does not exactly match producer PSS basis ({producer_frequency:.16e} Hz, {} harmonics, tolerance {:.16e})",
                config.num_harmonics, config.tolerance
            )));
        }
        validate_periodic_state_contract(
            consumer,
            PeriodicStateCapability {
                shooting: true,
                autonomous: config.is_autonomous(),
            },
            require_autonomous,
        )
        .map_err(ExecutionArtifactError::ContractMismatch)?;
        Ok(())
    }

    fn validate(&self) -> Result<(), ExecutionArtifactError> {
        if let Some(environment) = &self.environment {
            environment.validate()?;
        }
        let analysis = self.operating_point.analysis();
        let validation = if let Some(identity) = self.operating_point.producer_identity() {
            rspice_core::engine::PssOperatingPoint::try_from_authenticated_parts(
                identity.clone(),
                self.operating_point.config().clone(),
                analysis.clone(),
                self.operating_point.shooting_state_basis().to_vec(),
                self.operating_point.shooting_state().to_vec(),
            )
        } else {
            rspice_core::engine::PssOperatingPoint::try_from_parts(
                self.operating_point.config().clone(),
                analysis.clone(),
                self.operating_point.shooting_state().to_vec(),
            )
        };
        validation.map_err(|error| ExecutionArtifactError::InvalidPayload(error.to_string()))?;

        if self.floquet_evidence != analysis.result.floquet_evidence
            || self.floquet_orbit_kind != analysis.result.floquet_orbit_kind
            || self.trivial_floquet_multiplier_index
                != analysis.result.trivial_floquet_multiplier_index
        {
            return Err(ExecutionArtifactError::InvalidPayload(
                "periodic-state Floquet evidence compatibility metadata does not match the canonical result"
                    .to_owned(),
            ));
        }
        let computed_verdict = analysis.result.stability_verdict();
        let computed_authenticated =
            pss_floquet_contract_is_authenticated(&analysis.result, analysis.monodromy.len());
        if self.floquet_verdict != computed_verdict {
            return Err(ExecutionArtifactError::InvalidPayload(
                "periodic-state Floquet verdict compatibility metadata does not match the canonical result"
                    .to_owned(),
            ));
        }
        if self.floquet_authenticated != computed_authenticated || !computed_authenticated {
            return Err(ExecutionArtifactError::InvalidPayload(
                "periodic-state payload lacks authenticated Floquet evidence".to_owned(),
            ));
        }
        if self.analysis_is_stable != analysis.is_stable
            || analysis.is_stable
                != (computed_verdict == rspice_core::analysis::FloquetStabilityVerdict::Stable)
        {
            return Err(ExecutionArtifactError::InvalidPayload(
                "periodic-state stability compatibility flag does not match the canonical Floquet verdict"
                    .to_owned(),
            ));
        }

        validate_complex_cache(
            "PSS result Floquet",
            &analysis.result.floquet_multipliers,
            &self.result_floquet_real,
            &self.result_floquet_imag,
        )?;
        validate_complex_cache(
            "PSS analysis Floquet",
            &analysis.floquet_multipliers,
            &self.analysis_floquet_real,
            &self.analysis_floquet_imag,
        )?;

        let mut value_count = analysis.result.time.len();
        for waveform in analysis
            .result
            .waveforms
            .iter()
            .chain(&analysis.result.branch_waveforms)
        {
            value_count = value_count
                .checked_add(waveform.values.len())
                .ok_or_else(|| {
                    ExecutionArtifactError::InvalidPayload(
                        "periodic-state numeric payload size overflows this platform".to_owned(),
                    )
                })?;
        }
        for row in &analysis.monodromy {
            value_count = value_count.checked_add(row.len()).ok_or_else(|| {
                ExecutionArtifactError::InvalidPayload(
                    "periodic-state numeric payload size overflows this platform".to_owned(),
                )
            })?;
        }
        value_count = value_count
            .checked_add(analysis.result.floquet_multipliers.len().saturating_mul(2))
            .and_then(|count| {
                count.checked_add(analysis.floquet_multipliers.len().saturating_mul(2))
            })
            .and_then(|count| count.checked_add(self.operating_point.shooting_state().len()))
            .ok_or_else(|| {
                ExecutionArtifactError::InvalidPayload(
                    "periodic-state numeric payload size overflows this platform".to_owned(),
                )
            })?;
        if value_count > Self::MAX_NUMERIC_VALUES {
            return Err(ExecutionArtifactError::InvalidPayload(format!(
                "periodic-state payload contains {value_count} numerical values, exceeding the authenticated transport limit {}",
                Self::MAX_NUMERIC_VALUES
            )));
        }
        Ok(())
    }

    fn digest(&self) -> ContentDigest {
        let analysis = self.operating_point.analysis();
        let config = self.operating_point.config();
        let result = &analysis.result;
        let mut writer =
            CanonicalWriter::new(if self.operating_point.producer_identity().is_some() {
                "rspice.periodic-state-artifact/v4"
            } else {
                "rspice.periodic-state-artifact/v2"
            });
        if let Some(identity) = self.operating_point.producer_identity() {
            let (version, semantic_netlist, resolved_simulation, pss_config, retained_state) =
                identity.canonical_parts();
            writer.usize(version as usize);
            writer.string(semantic_netlist);
            writer.string(resolved_simulation);
            writer.string(pss_config);
            writer.string(retained_state);
        }
        writer.f64(config.fundamental_freq);
        writer.usize(config.num_harmonics);
        writer.f64(config.tstab);
        writer.usize(config.max_iterations);
        writer.f64(config.tolerance);
        writer.f64(config.abstol);
        writer.bool(config.auto_period);
        writer.option(config.oscillator_node.as_deref(), |writer, value| {
            writer.string(value)
        });
        writer.f64(config.period_guess);
        writer.usize(config.tstab_periods);
        writer.f64(config.damping_factor);
        writer.f64(config.max_period_change);
        writer.option(config.integration_method.as_ref(), |writer, method| {
            writer.u8(match method {
                rspice_core::numerics::integration::IntegrationMethod::BackwardEuler => 0,
                rspice_core::numerics::integration::IntegrationMethod::Trapezoidal => 1,
                rspice_core::numerics::integration::IntegrationMethod::Gear2 => 2,
                rspice_core::numerics::integration::IntegrationMethod::TrapGear => 3,
            });
        });
        writer.usize(config.points_per_period);
        writer.bool(config.verbose);
        writer.f64(result.period);
        writer.f64(result.frequency);
        writer.usize(result.iterations);
        writer.f64(result.residual_norm);
        writer.sequence(result.time.len());
        for value in &result.time {
            writer.f64(*value);
        }
        writer.sequence(result.node_names.len());
        for (name, waveform) in result.node_names.iter().zip(&result.waveforms) {
            writer.string(name);
            writer.sequence(waveform.values.len());
            for value in &waveform.values {
                writer.f64(*value);
            }
        }
        if self.operating_point.producer_identity().is_some() || !result.branch_waveforms.is_empty()
        {
            writer.sequence(result.branch_names.len());
            for (name, waveform) in result.branch_names.iter().zip(&result.branch_waveforms) {
                writer.string(name);
                writer.sequence(waveform.values.len());
                for value in &waveform.values {
                    writer.f64(*value);
                }
            }
        }
        writer.bool(result.period_detected);
        encode_complex_values(&mut writer, &result.floquet_multipliers);
        encode_floquet_evidence(&mut writer, &self.floquet_evidence);
        encode_floquet_orbit_kind(&mut writer, self.floquet_orbit_kind);
        writer.option(
            self.trivial_floquet_multiplier_index.as_ref(),
            |writer, index| writer.usize(*index),
        );
        encode_floquet_verdict(&mut writer, self.floquet_verdict);
        writer.bool(self.floquet_authenticated);
        writer.usize(analysis.iterations);
        writer.f64(analysis.final_residual);
        writer.f64(analysis.period);
        writer.sequence(analysis.monodromy.len());
        for row in &analysis.monodromy {
            writer.sequence(row.len());
            for value in row {
                writer.f64(*value);
            }
        }
        encode_complex_values(&mut writer, &analysis.floquet_multipliers);
        writer.bool(analysis.is_stable);
        writer.bool(self.analysis_is_stable);
        if self.operating_point.producer_identity().is_some() {
            writer.sequence(self.operating_point.shooting_state_basis().len());
            for name in self.operating_point.shooting_state_basis() {
                writer.string(name);
            }
        }
        writer.sequence(self.operating_point.shooting_state().len());
        for value in self.operating_point.shooting_state() {
            writer.f64(*value);
        }
        if let Some(environment) = &self.environment {
            writer.domain("periodic-operating-environment/v1");
            environment.encode(&mut writer);
        }
        writer.finish()
    }
}

fn encode_floquet_evidence(
    writer: &mut CanonicalWriter,
    evidence: &rspice_core::analysis::FloquetSpectrumEvidence,
) {
    match evidence {
        rspice_core::analysis::FloquetSpectrumEvidence::NotComputed => writer.u8(0),
        rspice_core::analysis::FloquetSpectrumEvidence::NoDynamicModes => writer.u8(1),
        rspice_core::analysis::FloquetSpectrumEvidence::Qualified { certificate } => {
            writer.u8(2);
            writer.usize(certificate.problem_order);
            writer.f64(certificate.max_backward_error);
            writer.f64(certificate.qualification_tolerance);
        }
        rspice_core::analysis::FloquetSpectrumEvidence::LegacyUnknown => writer.u8(3),
        _ => writer.u8(u8::MAX),
    }
}

fn encode_floquet_orbit_kind(
    writer: &mut CanonicalWriter,
    orbit_kind: rspice_core::analysis::FloquetOrbitKind,
) {
    writer.u8(match orbit_kind {
        rspice_core::analysis::FloquetOrbitKind::Driven => 0,
        rspice_core::analysis::FloquetOrbitKind::Autonomous => 1,
        _ => u8::MAX,
    });
}

fn encode_floquet_verdict(
    writer: &mut CanonicalWriter,
    verdict: rspice_core::analysis::FloquetStabilityVerdict,
) {
    writer.u8(match verdict {
        rspice_core::analysis::FloquetStabilityVerdict::Stable => 0,
        rspice_core::analysis::FloquetStabilityVerdict::Unstable => 1,
        rspice_core::analysis::FloquetStabilityVerdict::Marginal => 2,
        rspice_core::analysis::FloquetStabilityVerdict::Indeterminate => 3,
        _ => u8::MAX,
    });
}

fn validate_complex_cache(
    label: &str,
    values: &[num_complex::Complex64],
    real: &[f64],
    imaginary: &[f64],
) -> Result<(), ExecutionArtifactError> {
    if values.len() != real.len()
        || values.len() != imaginary.len()
        || values
            .iter()
            .zip(real)
            .zip(imaginary)
            .any(|((value, re), im)| {
                value.re.to_bits() != re.to_bits() || value.im.to_bits() != im.to_bits()
            })
    {
        return Err(ExecutionArtifactError::InvalidPayload(format!(
            "{label} transfer cache does not match the retained complex values"
        )));
    }
    Ok(())
}

fn split_complex_values(values: &[num_complex::Complex64]) -> (Vec<f64>, Vec<f64>) {
    values
        .iter()
        .map(|value| (value.re, value.im))
        .unzip::<_, _, Vec<_>, Vec<_>>()
}

#[cfg(any(target_arch = "wasm32", test))]
fn join_complex_values(
    label: &str,
    real: &[f64],
    imaginary: &[f64],
) -> Result<Vec<num_complex::Complex64>, ExecutionArtifactError> {
    if real.len() != imaginary.len() {
        return Err(ExecutionArtifactError::InvalidPayload(format!(
            "{label} real/imaginary vector lengths differ ({} versus {})",
            real.len(),
            imaginary.len()
        )));
    }
    Ok(real
        .iter()
        .copied()
        .zip(imaginary.iter().copied())
        .map(|(re, im)| num_complex::Complex64::new(re, im))
        .collect())
}

fn encode_complex_values(writer: &mut CanonicalWriter, values: &[num_complex::Complex64]) {
    writer.sequence(values.len());
    for value in values {
        writer.f64(value.re);
        writer.f64(value.im);
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(in crate::simulation) struct HbStateArtifact {
    #[serde(default)]
    environment: Option<PeriodicOperatingEnvironment>,
    operating_point: Arc<rspice_core::engine::HbOperatingPoint>,
    spectral_real: Vec<Vec<f64>>,
    spectral_imaginary: Vec<Vec<f64>>,
    #[serde(default)]
    mna_branch_spectral_real: Vec<Vec<f64>>,
    #[serde(default)]
    mna_branch_spectral_imaginary: Vec<Vec<f64>>,
    #[serde(default)]
    integral_spectral_real: Vec<Vec<f64>>,
    #[serde(default)]
    integral_spectral_imaginary: Vec<Vec<f64>>,
}

impl HbStateArtifact {
    const MAX_NUMERIC_VALUES: usize = 16_777_216;

    pub(in crate::simulation) fn materialize_consumer(
        &self,
        source: &str,
        source_path: Option<&std::path::Path>,
        dependencies: &ResolvedExecutionDependencies,
        abort: &dyn rspice_core::abort_signal::AbortSignal,
    ) -> crate::services::simulation_runner::ServiceRunResult<rspice_core::Netlist> {
        match &self.environment {
            Some(environment) => environment.materialize(source, source_path, dependencies, abort),
            None => crate::services::simulation_runner::parse_runner_netlist_with_abort(
                source,
                source_path,
                abort,
            ),
        }
    }

    pub(in crate::simulation) fn operating_point(&self) -> &rspice_core::engine::HbOperatingPoint {
        &self.operating_point
    }

    fn validate(&self) -> Result<(), ExecutionArtifactError> {
        if let Some(environment) = &self.environment {
            environment.validate()?;
        }
        self.operating_point
            .validate()
            .map_err(|error| ExecutionArtifactError::InvalidPayload(error.to_string()))?;
        if self.spectral_real.len() != self.operating_point.spectral_state().len()
            || self.spectral_imaginary.len() != self.operating_point.spectral_state().len()
        {
            return Err(ExecutionArtifactError::InvalidPayload(
                "HB-state transfer cache row count does not match the retained state".to_owned(),
            ));
        }
        let mut numeric_values = 0usize;
        for (index, coefficients) in self.operating_point.spectral_state().iter().enumerate() {
            let real = &self.spectral_real[index];
            let imaginary = &self.spectral_imaginary[index];
            validate_complex_cache("HB spectral row", coefficients, real, imaginary)?;
            numeric_values = numeric_values
                .checked_add(real.len().saturating_mul(2))
                .ok_or_else(|| {
                    ExecutionArtifactError::InvalidPayload(
                        "HB-state numeric payload size overflows this platform".to_owned(),
                    )
                })?;
        }
        if self.mna_branch_spectral_real.len()
            != self.operating_point.mna_branch_spectral_state().len()
            || self.mna_branch_spectral_imaginary.len()
                != self.operating_point.mna_branch_spectral_state().len()
        {
            return Err(ExecutionArtifactError::InvalidPayload(
                "HB-state MNA branch transfer cache row count does not match the retained state"
                    .to_owned(),
            ));
        }
        for (index, coefficients) in self
            .operating_point
            .mna_branch_spectral_state()
            .iter()
            .enumerate()
        {
            let real = &self.mna_branch_spectral_real[index];
            let imaginary = &self.mna_branch_spectral_imaginary[index];
            validate_complex_cache("HB MNA branch spectral row", coefficients, real, imaginary)?;
            numeric_values = numeric_values
                .checked_add(real.len().saturating_mul(2))
                .ok_or_else(|| {
                    ExecutionArtifactError::InvalidPayload(
                        "HB-state numeric payload size overflows this platform".to_owned(),
                    )
                })?;
        }
        if self.integral_spectral_real.len() != self.operating_point.integral_spectra().len()
            || self.integral_spectral_imaginary.len()
                != self.operating_point.integral_spectra().len()
        {
            return Err(ExecutionArtifactError::InvalidPayload(
                "HB-state integral transfer cache row count does not match the retained state"
                    .to_owned(),
            ));
        }
        for (index, spectrum) in self.operating_point.integral_spectra().iter().enumerate() {
            let real = &self.integral_spectral_real[index];
            let imaginary = &self.integral_spectral_imaginary[index];
            validate_complex_cache(
                "HB integral spectral row",
                &spectrum.coefficients,
                real,
                imaginary,
            )?;
            numeric_values = numeric_values
                .checked_add(real.len().saturating_mul(2))
                .ok_or_else(|| {
                    ExecutionArtifactError::InvalidPayload(
                        "HB-state numeric payload size overflows this platform".to_owned(),
                    )
                })?;
        }
        if numeric_values > Self::MAX_NUMERIC_VALUES {
            return Err(ExecutionArtifactError::InvalidPayload(format!(
                "HB-state payload contains {numeric_values} numerical values, exceeding the authenticated transport limit {}",
                Self::MAX_NUMERIC_VALUES
            )));
        }
        Ok(())
    }

    fn digest(&self) -> ContentDigest {
        let state = super::super::hb_operating_point_digest(&self.operating_point);
        let Some(environment) = &self.environment else {
            return state;
        };
        let mut writer = CanonicalWriter::new("rspice.hb-operating-environment/v1");
        writer.digest(state);
        environment.encode(&mut writer);
        writer.finish()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum ExecutionArtifactPayload {
    TransientTrajectory(Arc<TransientTrajectoryArtifact>),
    PeriodicState(Arc<PeriodicStateArtifact>),
    HbState(Arc<HbStateArtifact>),
    QpssState(Arc<QpssStateArtifact>),
    DcOperatingPointSeed(Arc<DcOperatingPointSeedArtifact>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(in crate::simulation) struct ExecutionArtifactEnvelope {
    snapshot_digest: ContentDigest,
    producer_instance_id: AnalysisInstanceId,
    producer_source_revision: ObjectRevision,
    producer_config_digest: ContentDigest,
    kind: ExecutionArtifactKind,
    payload_digest: ContentDigest,
    payload: ExecutionArtifactPayload,
}

impl ExecutionArtifactEnvelope {
    pub(in crate::simulation) fn from_transient_result(
        snapshot_digest: ContentDigest,
        producer_instance_id: AnalysisInstanceId,
        producer_source_revision: ObjectRevision,
        producer_config_digest: ContentDigest,
        result: &SimulationResult,
        required_waveforms: &[String],
        carry_spectra: bool,
    ) -> Result<Option<Self>, ExecutionArtifactError> {
        let Some(trajectory) =
            TransientTrajectoryArtifact::from_result(result, required_waveforms, carry_spectra)?
        else {
            return Ok(None);
        };
        let payload_digest = trajectory.digest();
        Ok(Some(Self {
            snapshot_digest,
            producer_instance_id,
            producer_source_revision,
            producer_config_digest,
            kind: ExecutionArtifactKind::TransientTrajectory,
            payload_digest,
            payload: ExecutionArtifactPayload::TransientTrajectory(Arc::new(trajectory)),
        }))
    }
    #[cfg(test)]
    pub(in crate::simulation) fn from_periodic_result(
        snapshot_digest: ContentDigest,
        producer_instance_id: AnalysisInstanceId,
        producer_source_revision: ObjectRevision,
        producer_config_digest: ContentDigest,
        producer_spec: &AnalysisSpec,
        result: &SimulationResult,
    ) -> Result<Option<Self>, ExecutionArtifactError> {
        Self::from_periodic_result_with_environment(
            snapshot_digest,
            producer_instance_id,
            producer_source_revision,
            producer_config_digest,
            producer_spec,
            result,
            None,
        )
    }

    pub(in crate::simulation) fn from_periodic_result_with_environment(
        snapshot_digest: ContentDigest,
        producer_instance_id: AnalysisInstanceId,
        producer_source_revision: ObjectRevision,
        producer_config_digest: ContentDigest,
        producer_spec: &AnalysisSpec,
        result: &SimulationResult,
        environment: Option<PeriodicOperatingEnvironment>,
    ) -> Result<Option<Self>, ExecutionArtifactError> {
        let SimulationResult::Transient { periodic_state, .. } = result else {
            return Err(ExecutionArtifactError::InvalidPayload(
                "PSS producer returned a non-periodic result variant".to_owned(),
            ));
        };
        let operating_point = periodic_state.as_ref().ok_or_else(|| {
            ExecutionArtifactError::InvalidPayload(
                "PSS producer did not retain the required numerical periodic state".to_owned(),
            )
        })?;
        validate_periodic_producer_config(producer_spec, operating_point.config())?;
        let (result_floquet_real, result_floquet_imag) =
            split_complex_values(&operating_point.analysis().result.floquet_multipliers);
        let (analysis_floquet_real, analysis_floquet_imag) =
            split_complex_values(&operating_point.analysis().floquet_multipliers);
        let analysis = operating_point.analysis();
        let floquet_verdict = analysis.result.stability_verdict();
        let floquet_authenticated =
            pss_floquet_contract_is_authenticated(&analysis.result, analysis.monodromy.len());
        let periodic_state = PeriodicStateArtifact {
            environment,
            operating_point: Arc::clone(operating_point),
            result_floquet_real,
            result_floquet_imag,
            floquet_evidence: analysis.result.floquet_evidence.clone(),
            floquet_orbit_kind: analysis.result.floquet_orbit_kind,
            trivial_floquet_multiplier_index: analysis.result.trivial_floquet_multiplier_index,
            floquet_verdict,
            floquet_authenticated,
            analysis_floquet_real,
            analysis_floquet_imag,
            analysis_is_stable: analysis.is_stable,
        };
        periodic_state.validate()?;
        let payload_digest = periodic_state.digest();
        Ok(Some(Self {
            snapshot_digest,
            producer_instance_id,
            producer_source_revision,
            producer_config_digest,
            kind: ExecutionArtifactKind::PeriodicState,
            payload_digest,
            payload: ExecutionArtifactPayload::PeriodicState(Arc::new(periodic_state)),
        }))
    }

    #[cfg(test)]
    pub(in crate::simulation) fn from_hb_result(
        snapshot_digest: ContentDigest,
        producer_instance_id: AnalysisInstanceId,
        producer_source_revision: ObjectRevision,
        producer_config_digest: ContentDigest,
        producer_spec: &AnalysisSpec,
        result: &SimulationResult,
    ) -> Result<Option<Self>, ExecutionArtifactError> {
        Self::from_hb_result_with_environment(
            snapshot_digest,
            producer_instance_id,
            producer_source_revision,
            producer_config_digest,
            producer_spec,
            Some("Synthetic HB artifact without option overrides\n.end\n"),
            result,
            None,
        )
    }

    pub(in crate::simulation) fn from_hb_result_with_environment(
        snapshot_digest: ContentDigest,
        producer_instance_id: AnalysisInstanceId,
        producer_source_revision: ObjectRevision,
        producer_config_digest: ContentDigest,
        producer_spec: &AnalysisSpec,
        producer_source: Option<&str>,
        result: &SimulationResult,
        environment: Option<PeriodicOperatingEnvironment>,
    ) -> Result<Option<Self>, ExecutionArtifactError> {
        let SimulationResult::HarmonicBalance {
            operating_point, ..
        } = result
        else {
            return Err(ExecutionArtifactError::InvalidPayload(
                "HB producer returned a non-HB result variant".to_owned(),
            ));
        };
        validate_hb_producer_config(
            producer_spec,
            producer_source,
            environment.as_ref(),
            operating_point.config(),
        )?;
        let (spectral_real, spectral_imaginary): (Vec<_>, Vec<_>) = operating_point
            .spectral_state()
            .iter()
            .map(|row| split_complex_values(row))
            .unzip();
        let (mna_branch_spectral_real, mna_branch_spectral_imaginary): (Vec<_>, Vec<_>) =
            operating_point
                .mna_branch_spectral_state()
                .iter()
                .map(|row| split_complex_values(row))
                .unzip();
        let (integral_spectral_real, integral_spectral_imaginary): (Vec<_>, Vec<_>) =
            operating_point
                .integral_spectra()
                .iter()
                .map(|spectrum| split_complex_values(&spectrum.coefficients))
                .unzip();
        let state = HbStateArtifact {
            environment,
            operating_point: Arc::clone(operating_point),
            spectral_real,
            spectral_imaginary,
            mna_branch_spectral_real,
            mna_branch_spectral_imaginary,
            integral_spectral_real,
            integral_spectral_imaginary,
        };
        state.validate()?;
        let payload_digest = state.digest();
        Ok(Some(Self {
            snapshot_digest,
            producer_instance_id,
            producer_source_revision,
            producer_config_digest,
            kind: ExecutionArtifactKind::HbState,
            payload_digest,
            payload: ExecutionArtifactPayload::HbState(Arc::new(state)),
        }))
    }

    pub(in crate::simulation) fn from_dc_operating_point_result(
        snapshot_digest: ContentDigest,
        producer_instance_id: AnalysisInstanceId,
        producer_source_revision: ObjectRevision,
        producer_config_digest: ContentDigest,
        effective_source_content_digest: ContentDigest,
        prepared_config: &crate::simulation::dialog::OpConfig,
        result: &SimulationResult,
    ) -> Result<Option<Self>, ExecutionArtifactError> {
        let SimulationResult::DcOp(result) = result else {
            return Err(ExecutionArtifactError::InvalidPayload(
                "operating-point producer returned a non-OP result variant".to_owned(),
            ));
        };
        if &result.configuration != prepared_config {
            return Err(ExecutionArtifactError::InvalidPayload(
                "worker-returned operating-point configuration does not match the authenticated prepared producer configuration"
                    .to_owned(),
            ));
        }
        let run_point = prepared_config.run_point.clone();
        let seed = DcOperatingPointSeedArtifact {
            effective_source_content_digest,
            temperature_celsius: prepared_config.temperature_celsius,
            supply_voltage: run_point.supply_voltage,
            nominal_supply_voltage: run_point.nominal_supply_voltage,
            supply_source_names: run_point.supply_source_names,
            node_names: result.mna_node_names.clone(),
            branch_names: result.mna_branch_names.clone(),
            solution: result.mna_solution.clone(),
        };
        seed.validate()?;
        let payload_digest = seed.digest();
        Ok(Some(Self {
            snapshot_digest,
            producer_instance_id,
            producer_source_revision,
            producer_config_digest,
            kind: ExecutionArtifactKind::DcOperatingPointSeed,
            payload_digest,
            payload: ExecutionArtifactPayload::DcOperatingPointSeed(Arc::new(seed)),
        }))
    }

    pub(in crate::simulation) fn trajectory(&self) -> Option<&TransientTrajectoryArtifact> {
        match &self.payload {
            ExecutionArtifactPayload::TransientTrajectory(trajectory) => Some(trajectory),
            ExecutionArtifactPayload::QpssState(_)
            | ExecutionArtifactPayload::PeriodicState(_)
            | ExecutionArtifactPayload::HbState(_)
            | ExecutionArtifactPayload::DcOperatingPointSeed(_) => None,
        }
    }

    pub(in crate::simulation) fn periodic_state(&self) -> Option<&PeriodicStateArtifact> {
        match &self.payload {
            ExecutionArtifactPayload::PeriodicState(state) => Some(state),
            ExecutionArtifactPayload::QpssState(_)
            | ExecutionArtifactPayload::TransientTrajectory(_)
            | ExecutionArtifactPayload::HbState(_)
            | ExecutionArtifactPayload::DcOperatingPointSeed(_) => None,
        }
    }

    pub(in crate::simulation) fn hb_state(&self) -> Option<&HbStateArtifact> {
        match &self.payload {
            ExecutionArtifactPayload::HbState(state) => Some(state),
            ExecutionArtifactPayload::QpssState(_)
            | ExecutionArtifactPayload::TransientTrajectory(_)
            | ExecutionArtifactPayload::PeriodicState(_)
            | ExecutionArtifactPayload::DcOperatingPointSeed(_) => None,
        }
    }

    pub(in crate::simulation) fn dc_operating_point_seed(
        &self,
    ) -> Option<&DcOperatingPointSeedArtifact> {
        match &self.payload {
            ExecutionArtifactPayload::DcOperatingPointSeed(seed) => Some(seed),
            ExecutionArtifactPayload::QpssState(_)
            | ExecutionArtifactPayload::TransientTrajectory(_)
            | ExecutionArtifactPayload::PeriodicState(_)
            | ExecutionArtifactPayload::HbState(_) => None,
        }
    }

    fn validate_against(
        &self,
        snapshot_digest: ContentDigest,
        binding: &PreparedDependencyBinding,
    ) -> Result<(), ExecutionArtifactError> {
        if self.snapshot_digest != snapshot_digest {
            return Err(ExecutionArtifactError::StaleSnapshot {
                expected: snapshot_digest,
                actual: self.snapshot_digest,
            });
        }
        if self.kind != binding.kind
            || self.producer_instance_id != binding.producer_instance_id
            || self.producer_source_revision != binding.producer_source_revision
            || self.producer_config_digest != binding.producer_config_digest
        {
            return Err(ExecutionArtifactError::ProducerMismatch {
                expected: binding.producer_instance_id,
                actual: self.producer_instance_id,
            });
        }
        let actual_digest = match self.kind {
            ExecutionArtifactKind::TransientTrajectory => {
                let ExecutionArtifactPayload::TransientTrajectory(trajectory) = &self.payload
                else {
                    return Err(ExecutionArtifactError::InvalidPayload(
                        "transient artifact carries the wrong payload variant".to_owned(),
                    ));
                };
                trajectory.validate()?;
                trajectory.digest()
            }
            ExecutionArtifactKind::PeriodicState => {
                let ExecutionArtifactPayload::PeriodicState(periodic_state) = &self.payload else {
                    return Err(ExecutionArtifactError::InvalidPayload(
                        "periodic-state artifact carries the wrong payload variant".to_owned(),
                    ));
                };
                periodic_state.validate()?;
                periodic_state.digest()
            }
            ExecutionArtifactKind::QpssState => {
                let ExecutionArtifactPayload::QpssState(state) = &self.payload else {
                    return Err(ExecutionArtifactError::InvalidPayload(
                        "QPSS artifact carries the wrong payload variant".into(),
                    ));
                };
                state.validate()?;
                state.digest()
            }
            ExecutionArtifactKind::HbState => {
                let ExecutionArtifactPayload::HbState(state) = &self.payload else {
                    return Err(ExecutionArtifactError::InvalidPayload(
                        "HB-state artifact carries the wrong payload variant".to_owned(),
                    ));
                };
                state.validate()?;
                state.digest()
            }
            ExecutionArtifactKind::DcOperatingPointSeed => {
                let ExecutionArtifactPayload::DcOperatingPointSeed(seed) = &self.payload else {
                    return Err(ExecutionArtifactError::InvalidPayload(
                        "DC operating-point seed artifact carries the wrong payload variant"
                            .to_owned(),
                    ));
                };
                seed.validate()?;
                seed.digest()
            }
        };
        if actual_digest != self.payload_digest {
            return Err(ExecutionArtifactError::PayloadDigestMismatch {
                expected: self.payload_digest,
                actual: actual_digest,
            });
        }
        Ok(())
    }
}

fn normalize_waveform_name(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.len() >= 3
        && (trimmed.starts_with("V(") || trimmed.starts_with("v("))
        && trimmed.ends_with(')')
    {
        return trimmed[2..trimmed.len() - 1].trim().to_ascii_uppercase();
    }
    trimmed.to_ascii_uppercase()
}

/// Both digests are fixed by snapshot preparation before the one-use dispatch.
/// One authenticates the actual worker input; the other identifies the common
/// circuit source before each analysis adds its own numerical controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DependencySourceContext {
    executable: ContentDigest,
    basis: ContentDigest,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
struct TransferBufferRef {
    buffer: usize,
    len: usize,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TransientTrajectoryTransferMetadata {
    #[serde(default)]
    current_impulses: Option<crate::state::CurrentImpulseHistoryEvidence>,
    time: TransferBufferRef,
    waveforms: BTreeMap<String, TransferBufferRef>,
    #[serde(default)]
    convergence: Option<crate::simulation::results::ConvergenceTransport<TransferBufferRef>>,
    #[serde(default)]
    spectra: Vec<RecordedFftSpectrumTransferMetadata>,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RecordedFftSpectrumTransferMetadata {
    request_key: String,
    evidence: crate::state::FftSpectrumEvidence,
    frequency: TransferBufferRef,
    real: TransferBufferRef,
    imaginary: TransferBufferRef,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct PeriodicWaveformTransferMetadata {
    node_name: String,
    values: TransferBufferRef,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct PeriodicStateTransferMetadata {
    #[serde(default)]
    environment: Option<PeriodicOperatingEnvironment>,
    #[serde(default)]
    producer_identity: Option<rspice_core::engine::PssOperatingPointIdentity>,
    config_fundamental_freq: f64,
    config_num_harmonics: usize,
    config_tstab: f64,
    config_max_iterations: usize,
    config_tolerance: f64,
    config_abstol: f64,
    config_auto_period: bool,
    config_oscillator_node: Option<String>,
    config_period_guess: f64,
    config_tstab_periods: usize,
    config_damping_factor: f64,
    config_max_period_change: f64,
    config_integration_method: Option<u8>,
    config_points_per_period: usize,
    config_verbose: bool,
    result_period: f64,
    result_frequency: f64,
    result_iterations: usize,
    result_residual_norm: f64,
    time: TransferBufferRef,
    waveforms: Vec<PeriodicWaveformTransferMetadata>,
    #[serde(default)]
    branch_waveforms: Vec<PeriodicWaveformTransferMetadata>,
    period_detected: bool,
    result_floquet_real: TransferBufferRef,
    result_floquet_imag: TransferBufferRef,
    #[serde(default)]
    floquet_evidence: rspice_core::analysis::FloquetSpectrumEvidence,
    #[serde(default)]
    floquet_orbit_kind: rspice_core::analysis::FloquetOrbitKind,
    #[serde(default)]
    trivial_floquet_multiplier_index: Option<usize>,
    #[serde(default = "indeterminate_floquet_verdict")]
    floquet_verdict: rspice_core::analysis::FloquetStabilityVerdict,
    #[serde(default)]
    floquet_authenticated: bool,
    analysis_iterations: usize,
    analysis_final_residual: f64,
    analysis_period: f64,
    monodromy: Vec<TransferBufferRef>,
    analysis_floquet_real: TransferBufferRef,
    analysis_floquet_imag: TransferBufferRef,
    is_stable: bool,
    #[serde(default)]
    analysis_is_stable: bool,
    #[serde(default)]
    shooting_state_basis: Vec<String>,
    shooting_state: TransferBufferRef,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct HbSpectrumTransferMetadata {
    node_name: String,
    real: TransferBufferRef,
    imaginary: TransferBufferRef,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct HbBranchSpectrumTransferMetadata {
    branch_name: String,
    real: TransferBufferRef,
    imaginary: TransferBufferRef,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct HbIntegralSpectrumTransferMetadata {
    name: String,
    real: TransferBufferRef,
    imaginary: TransferBufferRef,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct HbStateTransferMetadata {
    #[serde(default)]
    environment: Option<PeriodicOperatingEnvironment>,
    config: rspice_core::analysis::HbConfig,
    #[serde(default)]
    producer_identity: Option<rspice_core::engine::HbOperatingPointIdentity>,
    spectra: Vec<HbSpectrumTransferMetadata>,
    #[serde(default)]
    mna_branch_spectra: Vec<HbBranchSpectrumTransferMetadata>,
    #[serde(default)]
    integral_spectra: Vec<HbIntegralSpectrumTransferMetadata>,
    iterations: usize,
    residual_norm: f64,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct DcOperatingPointSeedTransferMetadata {
    effective_source_content_digest: ContentDigest,
    temperature_celsius: f64,
    supply_voltage: Option<f64>,
    nominal_supply_voltage: Option<f64>,
    #[serde(default)]
    supply_source_names: Vec<String>,
    node_names: Vec<String>,
    branch_names: Vec<String>,
    solution: TransferBufferRef,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum ExecutionArtifactPayloadTransferMetadata {
    TransientTrajectory(Box<TransientTrajectoryTransferMetadata>),
    PeriodicState(Box<PeriodicStateTransferMetadata>),
    HbState(Box<HbStateTransferMetadata>),
    QpssState(Box<qpss::QpssStateTransferMetadata>),
    DcOperatingPointSeed(DcOperatingPointSeedTransferMetadata),
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ExecutionArtifactTransferMetadata {
    snapshot_digest: ContentDigest,
    producer_instance_id: AnalysisInstanceId,
    producer_source_revision: ObjectRevision,
    producer_config_digest: ContentDigest,
    kind: ExecutionArtifactKind,
    payload_digest: ContentDigest,
    payload: ExecutionArtifactPayloadTransferMetadata,
}

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ResolvedExecutionDependenciesTransferMetadata {
    #[serde(default)]
    source: Option<DependencySourceContext>,
    snapshot_digest: Option<ContentDigest>,
    bindings: Vec<PreparedDependencyBinding>,
    artifacts: Vec<ExecutionArtifactTransferMetadata>,
}

#[cfg(any(target_arch = "wasm32", test))]
fn push_transfer_slice<'a>(
    buffers: &mut Vec<std::borrow::Cow<'a, [f64]>>,
    values: &'a [f64],
) -> TransferBufferRef {
    let reference = TransferBufferRef {
        buffer: buffers.len(),
        len: values.len(),
    };
    buffers.push(std::borrow::Cow::Borrowed(values));
    reference
}

#[cfg(any(target_arch = "wasm32", test))]
fn take_transfer_buffer(
    buffers: &mut [Option<Vec<f64>>],
    reference: TransferBufferRef,
) -> Result<Vec<f64>, ExecutionArtifactError> {
    if reference.len > PeriodicStateArtifact::MAX_NUMERIC_VALUES {
        return Err(ExecutionArtifactError::Transport(format!(
            "dependency transfer buffer {} declares {} values, exceeding the {}-value limit",
            reference.buffer,
            reference.len,
            PeriodicStateArtifact::MAX_NUMERIC_VALUES
        )));
    }
    let slot = buffers.get_mut(reference.buffer).ok_or_else(|| {
        ExecutionArtifactError::Transport(format!(
            "dependency transfer references missing buffer {}",
            reference.buffer
        ))
    })?;
    let actual_len = slot.as_ref().map(Vec::len).ok_or_else(|| {
        ExecutionArtifactError::Transport(format!(
            "dependency transfer buffer {} is referenced more than once",
            reference.buffer
        ))
    })?;
    if actual_len != reference.len {
        return Err(ExecutionArtifactError::Transport(format!(
            "dependency transfer buffer {} has length {}, expected {}",
            reference.buffer, actual_len, reference.len
        )));
    }
    Ok(slot.take().expect("validated occupied transfer slot"))
}
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub(in crate::simulation) enum ExecutionArtifactError {
    #[error("missing {kind:?} artifact from bound prerequisite {producer}")]
    Missing {
        producer: AnalysisInstanceId,
        kind: ExecutionArtifactKind,
    },
    #[error("dependency artifact belongs to producer {actual}, expected {expected}")]
    ProducerMismatch {
        expected: AnalysisInstanceId,
        actual: AnalysisInstanceId,
    },
    #[error("dependency artifact belongs to stale snapshot {actual}, expected {expected}")]
    StaleSnapshot {
        expected: ContentDigest,
        actual: ContentDigest,
    },
    #[error("dependency artifact payload digest is {actual}, expected {expected}")]
    PayloadDigestMismatch {
        expected: ContentDigest,
        actual: ContentDigest,
    },
    #[error("invalid dependency artifact payload: {0}")]
    InvalidPayload(String),
    #[error("invalid typed dependency contract: {0}")]
    ContractMismatch(String),
    #[cfg(any(target_arch = "wasm32", test))]
    #[error("invalid dependency artifact transfer: {0}")]
    Transport(String),
}

#[cfg(test)]
mod tests;
