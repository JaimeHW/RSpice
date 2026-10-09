//! A QPSS dependency retains the independent lattice, never a common-period HB state.
use super::*;
use rspice_core::engine::QpssOperatingPoint;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpssStateArtifact {
    operating_point: Arc<QpssOperatingPoint>,
    spectral_real: Vec<Vec<f64>>,
    spectral_imaginary: Vec<Vec<f64>>,
    #[serde(default)]
    environment: Option<PeriodicOperatingEnvironment>,
}

impl QpssStateArtifact {
    pub fn operating_point(&self) -> &Arc<QpssOperatingPoint> {
        &self.operating_point
    }

    pub fn materialize_consumer(
        &self,
        source: &str,
        source_path: Option<&std::path::Path>,
        dependencies: &ResolvedExecutionDependencies,
        abort: &dyn rspice_core::abort_signal::AbortSignal,
    ) -> crate::error::ServiceRunResult<rspice_core::Netlist> {
        self.materialize_consumer_with_resource_limits(
            source,
            source_path,
            dependencies,
            rspice_core::ResourceLimits::default(),
            abort,
        )
    }

    pub fn materialize_consumer_with_resource_limits(
        &self,
        source: &str,
        source_path: Option<&std::path::Path>,
        dependencies: &ResolvedExecutionDependencies,
        limits: rspice_core::ResourceLimits,
        abort: &dyn rspice_core::abort_signal::AbortSignal,
    ) -> crate::error::ServiceRunResult<rspice_core::Netlist> {
        match &self.environment {
            Some(environment) => environment.materialize_with_resource_limits(
                source,
                source_path,
                dependencies,
                limits,
                abort,
            ),
            None => {
                crate::netlist_preparation::parse_runner_netlist_with_resource_limits_and_abort(
                    source,
                    source_path,
                    limits,
                    abort,
                )
            }
        }
    }

    pub(super) fn validate(&self) -> Result<(), ExecutionArtifactError> {
        if let Some(environment) = &self.environment {
            environment.validate()?;
        }
        self.operating_point
            .validate_retained_payload_with_abort(
                &rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort,
            )
            .map_err(|error| ExecutionArtifactError::InvalidPayload(error.to_string()))?;
        let rows = self.operating_point.complete_spectra();
        if self.spectral_real.len() != rows.len() || self.spectral_imaginary.len() != rows.len() {
            return Err(ExecutionArtifactError::InvalidPayload(
                "QPSS cached spectral row count differs".into(),
            ));
        }
        for (index, row) in rows.iter().enumerate() {
            let real = &self.spectral_real[index];
            let imaginary = &self.spectral_imaginary[index];
            if real.len() != row.len()
                || imaginary.len() != row.len()
                || row.iter().enumerate().any(|(column, value)| {
                    value.re.to_bits() != real[column].to_bits()
                        || value.im.to_bits() != imaginary[column].to_bits()
                })
            {
                return Err(ExecutionArtifactError::InvalidPayload(
                    "QPSS cached spectral coefficients differ".into(),
                ));
            }
        }
        Ok(())
    }

    pub(super) fn digest(&self) -> ContentDigest {
        let mut writer = CanonicalWriter::new("rspice.qpss-operating-point/v1");
        writer.string(self.operating_point.retained_identity());
        if let Some(environment) = &self.environment {
            writer.domain("periodic-operating-environment/v1");
            environment.encode(&mut writer);
        }
        writer.finish()
    }

    fn from_point(
        point: Arc<QpssOperatingPoint>,
        environment: Option<PeriodicOperatingEnvironment>,
    ) -> Result<Self, ExecutionArtifactError> {
        point
            .validate_retained_payload_with_abort(
                &rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort,
            )
            .map_err(|error| ExecutionArtifactError::InvalidPayload(error.to_string()))?;
        let (spectral_real, spectral_imaginary) = point
            .complete_spectra()
            .iter()
            .map(|row| split_complex_values(row))
            .unzip();
        let state = Self {
            environment,
            operating_point: point,
            spectral_real,
            spectral_imaginary,
        };
        state.validate()?;
        Ok(state)
    }
}

impl ExecutionArtifactEnvelope {
    pub fn from_qpss_result_with_environment(
        snapshot_digest: ContentDigest,
        producer_instance_id: AnalysisInstanceId,
        producer_source_revision: ObjectRevision,
        producer_config_digest: ContentDigest,
        producer_spec: &AnalysisSpec,
        result: &SimulationResult,
        environment: Option<PeriodicOperatingEnvironment>,
    ) -> Result<Option<Self>, ExecutionArtifactError> {
        let SimulationResult::Qpss {
            operating_point, ..
        } = result
        else {
            return Err(ExecutionArtifactError::InvalidPayload(
                "QPSS producer returned a different result family".into(),
            ));
        };
        let expected = producer_spec
            .qpss_config()
            .map_err(ExecutionArtifactError::ContractMismatch)?;
        if &expected != operating_point.config() {
            return Err(ExecutionArtifactError::ContractMismatch(
                "QPSS result configuration differs from its prepared producer".into(),
            ));
        }
        let state = QpssStateArtifact::from_point(Arc::clone(operating_point), environment)?;
        Ok(Some(Self {
            snapshot_digest,
            producer_instance_id,
            producer_source_revision,
            producer_config_digest,
            kind: ExecutionArtifactKind::QpssState,
            payload_digest: state.digest(),
            payload: ExecutionArtifactPayload::QpssState(Arc::new(state)),
        }))
    }

    pub fn qpss_state(&self) -> Option<&QpssStateArtifact> {
        match &self.payload {
            ExecutionArtifactPayload::QpssState(state) => Some(state),
            _ => None,
        }
    }
}

impl ResolvedExecutionDependencies {
    pub fn qpss_state(&self) -> Result<&QpssStateArtifact, ExecutionArtifactError> {
        if self.artifacts.len() != 1
            || self.bindings.len() != 1
            || self.bindings[0].kind != ExecutionArtifactKind::QpssState
        {
            return Err(ExecutionArtifactError::ContractMismatch(
                "exactly one QPSS-state artifact is required".into(),
            ));
        }
        self.artifacts[0].qpss_state().ok_or_else(|| {
            ExecutionArtifactError::ContractMismatch(
                "resolved QPSS dependency has no independent-tone state".into(),
            )
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct QpssStateTransferMetadata {
    point: rspice_core::engine::QpssOperatingPointMetadata,
    real: Vec<TransferBufferRef>,
    imaginary: Vec<TransferBufferRef>,
    #[serde(default)]
    environment: Option<PeriodicOperatingEnvironment>,
}

impl QpssStateArtifact {
    pub(super) fn encode_transfer<'a>(
        &'a self,
        buffers: &mut Vec<std::borrow::Cow<'a, [f64]>>,
    ) -> QpssStateTransferMetadata {
        let (point, _) = self.operating_point.as_ref().clone().into_transfer_parts();
        QpssStateTransferMetadata {
            environment: self.environment.clone(),
            point,
            real: self
                .spectral_real
                .iter()
                .map(|row| push_transfer_slice(buffers, row))
                .collect(),
            imaginary: self
                .spectral_imaginary
                .iter()
                .map(|row| push_transfer_slice(buffers, row))
                .collect(),
        }
    }
}

impl QpssStateTransferMetadata {
    pub(super) fn decode(
        self,
        buffers: &mut [Option<Vec<f64>>],
    ) -> Result<QpssStateArtifact, ExecutionArtifactError> {
        if self.real.len() != self.imaginary.len()
            || self
                .real
                .iter()
                .zip(&self.imaginary)
                .any(|(a, b)| a.len != b.len)
        {
            return Err(ExecutionArtifactError::InvalidPayload(
                "QPSS transferred real/imaginary shapes differ".into(),
            ));
        }
        let lengths = self.real.iter().map(|row| row.len).collect::<Vec<_>>();
        self.point
            .validate_transfer_layout_with_abort(
                &lengths,
                &rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort,
            )
            .map_err(|error| ExecutionArtifactError::InvalidPayload(error.to_string()))?;
        let mut spectra = Vec::with_capacity(lengths.len());
        let mut spectral_real = Vec::with_capacity(lengths.len());
        let mut spectral_imaginary = Vec::with_capacity(lengths.len());
        for (real, imaginary) in self.real.into_iter().zip(self.imaginary) {
            let real = take_transfer_buffer(buffers, real)?;
            let imaginary = take_transfer_buffer(buffers, imaginary)?;
            spectra.push(join_complex_values(
                "QPSS dependency row",
                &real,
                &imaginary,
            )?);
            spectral_real.push(real);
            spectral_imaginary.push(imaginary);
        }
        let point = QpssOperatingPoint::from_transfer_parts_with_abort(
            self.point,
            spectra,
            &rspice_core::ResourceLimits::default(),
            &rspice_core::NoAbort,
        )
        .map_err(|error| ExecutionArtifactError::InvalidPayload(error.to_string()))?;
        let state = QpssStateArtifact {
            environment: self.environment,
            operating_point: Arc::new(point),
            spectral_real,
            spectral_imaginary,
        };
        state.validate()?;
        Ok(state)
    }
}
