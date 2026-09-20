//! A QPSS dependency retains the independent lattice, never a common-period HB state.
use super::*;
use rspice_core::engine::QpssOperatingPoint;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(in crate::simulation) struct QpssStateArtifact {
    operating_point: Arc<QpssOperatingPoint>,
    spectral_real: Vec<Vec<f64>>,
    spectral_imaginary: Vec<Vec<f64>>,
}

impl QpssStateArtifact {
    pub(in crate::simulation) fn operating_point(&self) -> &Arc<QpssOperatingPoint> {
        &self.operating_point
    }

    pub(super) fn validate(&self) -> Result<(), ExecutionArtifactError> {
        self.operating_point
            .validate_retained_payload_with_abort(
                &rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort,
            )
            .map_err(|error| ExecutionArtifactError::InvalidPayload(error.to_string()))?;
        let rows = self.operating_point.spectra();
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
        writer.finish()
    }

    fn from_point(point: Arc<QpssOperatingPoint>) -> Result<Self, ExecutionArtifactError> {
        point
            .validate_retained_payload_with_abort(
                &rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort,
            )
            .map_err(|error| ExecutionArtifactError::InvalidPayload(error.to_string()))?;
        let (spectral_real, spectral_imaginary) = point
            .spectra()
            .iter()
            .map(|row| split_complex_values(row))
            .unzip();
        let state = Self {
            operating_point: point,
            spectral_real,
            spectral_imaginary,
        };
        state.validate()?;
        Ok(state)
    }
}

impl ExecutionArtifactEnvelope {
    pub(in crate::simulation) fn from_qpss_result(
        snapshot_digest: ContentDigest,
        producer_instance_id: AnalysisInstanceId,
        producer_source_revision: ObjectRevision,
        producer_config_digest: ContentDigest,
        producer_spec: &AnalysisSpec,
        result: &SimulationResult,
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
            .driven_qpss_config()
            .map_err(ExecutionArtifactError::ContractMismatch)?;
        if &expected != operating_point.config() {
            return Err(ExecutionArtifactError::ContractMismatch(
                "QPSS result configuration differs from its prepared producer".into(),
            ));
        }
        let state = QpssStateArtifact::from_point(Arc::clone(operating_point))?;
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

    pub(in crate::simulation) fn qpss_state(&self) -> Option<&QpssStateArtifact> {
        match &self.payload {
            ExecutionArtifactPayload::QpssState(state) => Some(state),
            _ => None,
        }
    }
}

impl ResolvedExecutionDependencies {
    pub(in crate::simulation) fn qpss_state(
        &self,
    ) -> Result<&QpssStateArtifact, ExecutionArtifactError> {
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

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct QpssStateTransferMetadata {
    point: rspice_core::engine::QpssOperatingPointMetadata,
    real: Vec<TransferBufferRef>,
    imaginary: Vec<TransferBufferRef>,
}

#[cfg(any(target_arch = "wasm32", test))]
impl QpssStateArtifact {
    pub(super) fn encode_transfer<'a>(
        &'a self,
        buffers: &mut Vec<std::borrow::Cow<'a, [f64]>>,
    ) -> QpssStateTransferMetadata {
        let (point, _) = self.operating_point.as_ref().clone().into_transfer_parts();
        QpssStateTransferMetadata {
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

#[cfg(any(target_arch = "wasm32", test))]
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
            operating_point: Arc::new(point),
            spectral_real,
            spectral_imaginary,
        };
        state.validate()?;
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::multi_run::FrequencySweep;

    fn setup() -> (
        AnalysisSpec,
        SimulationResult,
        ContentDigest,
        PreparedDependencyBinding,
        ExecutionArtifactEnvelope,
    ) {
        let draft = crate::simulation::plan::QpssDraft {
            tones: "1k,1414.213562373095".into(),
            harmonics: "1,1".into(),
            ..Default::default()
        };
        let spec = draft.to_spec().unwrap();
        let data = crate::services::simulation_runner::run_qpss_analysis_with_source_path_and_abort(
            "QPSS artifact\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\nI1 0 out SIN(0 .001 1414.213562373095)\n.end\n",
            spec.driven_qpss_config().unwrap(), None, &rspice_core::NoAbort,
        ).unwrap();
        let result = SimulationResult::from_qpss_operating_point(data.operating_point).unwrap();
        let snapshot = ContentDigest::from_bytes([19; 32]);
        let binding = PreparedDependencyBinding::qpss_state(
            AnalysisInstanceId::new(),
            ObjectRevision::new(7).unwrap(),
            ContentDigest::from_bytes([20; 32]),
        );
        let artifact = ExecutionArtifactEnvelope::from_qpss_result(
            snapshot,
            binding.producer_instance_id,
            binding.producer_source_revision,
            binding.producer_config_digest,
            &spec,
            &result,
        )
        .unwrap()
        .unwrap();
        (spec, result, snapshot, binding, artifact)
    }

    fn consumers() -> Vec<AnalysisSpec> {
        vec![
            AnalysisSpec::Qpac {
                start_freq: 10.0,
                stop_freq: 100.0,
                points_per_unit: 3,
                sweep: FrequencySweep::Linear,
                input_source: "V1".into(),
                output_node: "out".into(),
                output_ref: "0".into(),
                input_lattice: vec![0, 0],
                output_lattice: vec![1, -1],
                controls: Default::default(),
            },
            AnalysisSpec::Qpxf {
                start_freq: 10.0,
                stop_freq: 100.0,
                points_per_unit: 3,
                sweep: FrequencySweep::Linear,
                input_source: "V1".into(),
                output_node: "out".into(),
                output_ref: "0".into(),
                input_lattice: vec![0, 0],
                output_lattice: vec![1, -1],
                group_delay: true,
                controls: Default::default(),
            },
            AnalysisSpec::Qpnoise {
                start_freq: 10.0,
                stop_freq: 100.0,
                points_per_unit: 3,
                sweep: FrequencySweep::Linear,
                input_source: "V1".into(),
                output_node: "out".into(),
                output_ref: "0".into(),
                lattice_min: [-1, -1],
                lattice_max: [1, 1],
                integrated_noise: true,
                contributor_ranking: true,
            },
        ]
    }

    #[test]
    fn qpss_artifact_binds_and_transfers_exact_independent_tone_state() {
        let (spec, _, snapshot, binding, artifact) = setup();
        let resolved = ResolvedExecutionDependencies::resolve(
            snapshot,
            vec![binding.clone()],
            &HashMap::from([(binding.producer_instance_id, artifact)]),
        )
        .unwrap();
        for consumer in consumers() {
            validate_prepared_dependency_contract(&consumer, &spec).unwrap();
            resolved
                .validate_for_spec(&consumer, &SpecExecutionOptions::default())
                .unwrap();
            assert!(
                validate_prepared_dependency_contract(&consumer, &AnalysisSpec::LegacyDcOp)
                    .is_err()
            );
            let mut autonomous = spec.clone();
            if let AnalysisSpec::Qpss { autonomous, .. } = &mut autonomous {
                *autonomous = true;
            }
            assert!(validate_prepared_dependency_contract(&consumer, &autonomous).is_err());
        }
        assert!(resolved.hb_state().is_err());
        let (metadata, buffers) = resolved.encode_transfer().unwrap();
        let restored = ResolvedExecutionDependencies::decode_transfer(&metadata, buffers).unwrap();
        assert_eq!(
            restored.qpss_state().unwrap().operating_point(),
            resolved.qpss_state().unwrap().operating_point()
        );
        let json = serde_json::to_string(&resolved).unwrap();
        let native: ResolvedExecutionDependencies = serde_json::from_str(&json).unwrap();
        native
            .validate_for_spec(&consumers()[0], &SpecExecutionOptions::default())
            .unwrap();
    }

    #[test]
    fn qpss_artifact_rejects_changed_producer_and_transferred_evidence() {
        let (mut spec, result, snapshot, binding, artifact) = setup();
        if let AnalysisSpec::Qpss { controls, .. } = &mut spec {
            controls.max_backtracks += 1;
        }
        assert!(
            ExecutionArtifactEnvelope::from_qpss_result(
                snapshot,
                binding.producer_instance_id,
                binding.producer_source_revision,
                binding.producer_config_digest,
                &spec,
                &result
            )
            .is_err()
        );
        let artifacts = HashMap::from([(binding.producer_instance_id, artifact)]);
        assert!(
            ResolvedExecutionDependencies::resolve(
                ContentDigest::from_bytes([99; 32]),
                vec![binding.clone()],
                &artifacts
            )
            .is_err()
        );
        let resolved =
            ResolvedExecutionDependencies::resolve(snapshot, vec![binding], &artifacts).unwrap();
        let (metadata, mut buffers) = resolved.encode_transfer().unwrap();
        buffers[0][0] += 1.0;
        assert!(ResolvedExecutionDependencies::decode_transfer(&metadata, buffers).is_err());
        let (metadata, buffers) = resolved.encode_transfer().unwrap();
        let mut metadata: serde_json::Value = serde_json::from_str(&metadata).unwrap();
        metadata["artifacts"][0]["payload"]["QpssState"]["point"]["config"]["solver"]["relative_tolerance"] =
            serde_json::json!(0.01);
        assert!(
            ResolvedExecutionDependencies::decode_transfer(
                &serde_json::to_string(&metadata).unwrap(),
                buffers
            )
            .is_err()
        );
        let (metadata, buffers) = resolved.encode_transfer().unwrap();
        let mut metadata: serde_json::Value = serde_json::from_str(&metadata).unwrap();
        metadata["artifacts"][0]["payload"]["QpssState"]["real"][1]["buffer"] =
            serde_json::json!(0);
        assert!(
            ResolvedExecutionDependencies::decode_transfer(
                &serde_json::to_string(&metadata).unwrap(),
                buffers
            )
            .is_err()
        );
    }
}
