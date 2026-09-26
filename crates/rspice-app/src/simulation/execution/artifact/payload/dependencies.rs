//! Authenticated prerequisite artifacts and bounded worker transfer.

use super::*;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(in crate::simulation) struct ResolvedExecutionDependencies {
    pub(super) snapshot_digest: Option<ContentDigest>,
    pub(super) bindings: Vec<PreparedDependencyBinding>,
    pub(super) artifacts: Vec<ExecutionArtifactEnvelope>,
    #[serde(default)]
    pub(super) source: Option<DependencySourceContext>,
}

impl ResolvedExecutionDependencies {
    pub(in crate::simulation) fn bind_source(&mut self, executable: &str, basis: ContentDigest) {
        if self.bindings.is_empty() {
            return;
        }
        self.source = Some(DependencySourceContext {
            executable: crate::state::content_digest(executable),
            basis,
        });
    }

    pub(in crate::simulation) fn validate_source_basis(
        &self,
        source: &str,
        expected_basis: ContentDigest,
    ) -> Result<(), ExecutionArtifactError> {
        let actual = crate::state::content_digest(source);
        let basis = match self.source {
            Some(context) if context.executable == actual => context.basis,
            Some(_) => {
                return Err(ExecutionArtifactError::ContractMismatch(
                    "periodic consumer executable source differs from its authorized dispatch"
                        .into(),
                ));
            }
            None => actual,
        };
        if basis != expected_basis {
            return Err(ExecutionArtifactError::ContractMismatch(
                "periodic consumer circuit source differs from its bound operating point".into(),
            ));
        }
        Ok(())
    }

    pub(in crate::simulation) fn resolve(
        snapshot_digest: ContentDigest,
        bindings: Vec<PreparedDependencyBinding>,
        artifacts: &HashMap<AnalysisInstanceId, ExecutionArtifactEnvelope>,
    ) -> Result<Self, ExecutionArtifactError> {
        if bindings.is_empty() {
            return Ok(Self::default());
        }
        let mut resolved = Vec::with_capacity(bindings.len());
        for binding in &bindings {
            let artifact = artifacts.get(&binding.producer_instance_id).ok_or(
                ExecutionArtifactError::Missing {
                    producer: binding.producer_instance_id,
                    kind: binding.kind,
                },
            )?;
            artifact.validate_against(snapshot_digest, binding)?;
            resolved.push(artifact.clone());
        }
        Ok(Self {
            snapshot_digest: Some(snapshot_digest),
            bindings,
            artifacts: resolved,
            source: None,
        })
    }

    pub(in crate::simulation) fn validate_for_spec(
        &self,
        spec: &AnalysisSpec,
        options: &SpecExecutionOptions,
    ) -> Result<(), ExecutionArtifactError> {
        let expected_kinds = required_artifact_kinds(spec, options);
        let expected_count = usize::from(!expected_kinds.is_empty());
        if self.bindings.len() != expected_count || self.artifacts.len() != expected_count {
            return Err(ExecutionArtifactError::ContractMismatch(format!(
                "{} requires {expected_count} typed execution artifact(s), received {} binding(s) and {} artifact(s)",
                spec.run_type().display_name(),
                self.bindings.len(),
                self.artifacts.len()
            )));
        }
        if expected_count == 0 {
            if self.snapshot_digest.is_some() {
                return Err(ExecutionArtifactError::ContractMismatch(
                    "an artifact-free task carried an unexpected dependency snapshot identity"
                        .to_owned(),
                ));
            }
            return Ok(());
        }
        let snapshot_digest = self.snapshot_digest.ok_or_else(|| {
            ExecutionArtifactError::ContractMismatch(
                "typed execution dependencies have no prepared snapshot identity".to_owned(),
            )
        })?;
        let binding = &self.bindings[0];
        if !expected_kinds.contains(&binding.kind) {
            return Err(ExecutionArtifactError::ContractMismatch(format!(
                "{} requires a {} artifact",
                spec.run_type().display_name(),
                expected_kinds
                    .iter()
                    .map(|kind| kind.producer_label())
                    .collect::<Vec<_>>()
                    .join(" or ")
            )));
        }
        self.artifacts[0].validate_against(snapshot_digest, binding)
    }

    /// Which typed artifact this task was resolved against, or `None` for a
    /// task that binds none.
    ///
    /// The dispatch reads the carrier family off the artifact rather than off
    /// the request, because the artifact is the thing that was actually
    /// produced; the request's own `FROM=` is checked against it inside the
    /// service, where a mismatch is one sentence naming both.
    pub(in crate::simulation) fn artifact_kind(&self) -> Option<ExecutionArtifactKind> {
        self.bindings.first().map(|binding| binding.kind)
    }

    pub(in crate::simulation) fn validate_for_config(&self) -> Result<(), ExecutionArtifactError> {
        if self.snapshot_digest.is_none() && self.bindings.is_empty() && self.artifacts.is_empty() {
            Ok(())
        } else {
            Err(ExecutionArtifactError::ContractMismatch(
                "configuration-backed task carried unexpected typed execution dependencies"
                    .to_owned(),
            ))
        }
    }

    pub(in crate::simulation) fn transient_trajectory(
        &self,
    ) -> Result<&TransientTrajectoryArtifact, ExecutionArtifactError> {
        if self.artifacts.len() != 1
            || self.bindings.len() != 1
            || self.bindings[0].kind != ExecutionArtifactKind::TransientTrajectory
        {
            return Err(ExecutionArtifactError::ContractMismatch(
                "exactly one transient-trajectory artifact is required".to_owned(),
            ));
        }
        self.artifacts[0].trajectory().ok_or_else(|| {
            ExecutionArtifactError::ContractMismatch(
                "resolved transient dependency has no trajectory payload".to_owned(),
            )
        })
    }

    pub(in crate::simulation) fn periodic_state(
        &self,
    ) -> Result<&PeriodicStateArtifact, ExecutionArtifactError> {
        if self.artifacts.len() != 1
            || self.bindings.len() != 1
            || self.bindings[0].kind != ExecutionArtifactKind::PeriodicState
        {
            return Err(ExecutionArtifactError::ContractMismatch(
                "exactly one periodic-state artifact is required".to_owned(),
            ));
        }
        self.artifacts[0].periodic_state().ok_or_else(|| {
            ExecutionArtifactError::ContractMismatch(
                "resolved periodic dependency has no numerical state payload".to_owned(),
            )
        })
    }

    pub(in crate::simulation) fn hb_state(
        &self,
    ) -> Result<&HbStateArtifact, ExecutionArtifactError> {
        if self.artifacts.len() != 1
            || self.bindings.len() != 1
            || self.bindings[0].kind != ExecutionArtifactKind::HbState
        {
            return Err(ExecutionArtifactError::ContractMismatch(
                "exactly one HB-state artifact is required".to_owned(),
            ));
        }
        self.artifacts[0].hb_state().ok_or_else(|| {
            ExecutionArtifactError::ContractMismatch(
                "resolved HB dependency has no numerical state payload".to_owned(),
            )
        })
    }

    pub(in crate::simulation) fn dc_operating_point_seed(
        &self,
    ) -> Result<&DcOperatingPointSeedArtifact, ExecutionArtifactError> {
        if self.artifacts.len() != 1
            || self.bindings.len() != 1
            || self.bindings[0].kind != ExecutionArtifactKind::DcOperatingPointSeed
        {
            return Err(ExecutionArtifactError::ContractMismatch(
                "exactly one DC operating-point seed artifact is required".to_owned(),
            ));
        }
        self.artifacts[0].dc_operating_point_seed().ok_or_else(|| {
            ExecutionArtifactError::ContractMismatch(
                "resolved operating-point dependency has no DC seed payload".to_owned(),
            )
        })
    }

    /// Split numerical payloads from request metadata for the browser worker.
    ///
    /// The metadata is deliberately a small JSON object containing only
    /// identities, digests, names, and buffer references. Floating-point
    /// samples travel separately as `Float64Array` transfer buffers, avoiding
    /// per-sample string/JavaScript-object expansion while retaining their
    /// exact IEEE-754 bit patterns.
    #[cfg(test)]
    pub(in crate::simulation) fn encode_transfer(
        &self,
    ) -> Result<(String, Vec<Vec<f64>>), ExecutionArtifactError> {
        let (encoded, buffers) = self.encode_transfer_borrowed()?;
        Ok((
            encoded,
            buffers
                .into_iter()
                .map(std::borrow::Cow::into_owned)
                .collect(),
        ))
    }

    /// Encode transfer metadata while borrowing the numerical payloads.
    ///
    /// The browser sender uses this form to copy each retained artifact slice
    /// directly into its transferable `Float64Array`. Keeping the owned form
    /// above is useful for deterministic native transport round-trip tests,
    /// but must not introduce a second full payload allocation on the browser
    /// main thread.
    #[cfg(any(target_arch = "wasm32", test))]
    pub(in crate::simulation) fn encode_transfer_borrowed(
        &self,
    ) -> Result<EncodedArtifactTransfer<'_>, ExecutionArtifactError> {
        self.validate_transport_integrity()?;

        let mut buffers = Vec::new();
        let artifacts = self
            .artifacts
            .iter()
            .map(|artifact| {
                let payload = match &artifact.payload {
                    ExecutionArtifactPayload::DcOperatingPointSeed(seed) => {
                        let solution = push_transfer_slice(&mut buffers, &seed.solution);
                        ExecutionArtifactPayloadTransferMetadata::DcOperatingPointSeed(
                            DcOperatingPointSeedTransferMetadata {
                                effective_source_content_digest: seed
                                    .effective_source_content_digest,
                                temperature_celsius: seed.temperature_celsius,
                                supply_voltage: seed.supply_voltage,
                                nominal_supply_voltage: seed.nominal_supply_voltage,
                                supply_source_names: seed.supply_source_names.clone(),
                                node_names: seed.node_names.clone(),
                                branch_names: seed.branch_names.clone(),
                                solution,
                            },
                        )
                    }
                    ExecutionArtifactPayload::TransientTrajectory(trajectory) => {
                        let time = push_transfer_slice(&mut buffers, &trajectory.time);
                        let waveforms = trajectory
                            .waveforms
                            .iter()
                            .map(|(name, values)| {
                                (name.clone(), push_transfer_slice(&mut buffers, values))
                            })
                            .collect();
                        let convergence = trajectory.convergence.as_ref().map(|quality|
                            rspice_simulation_contract::convergence_transport::ConvergenceTransport::from_evidence(quality, |values| {
                                let reference = TransferBufferRef { buffer: buffers.len(), len: values.len() };
                                buffers.push(values);
                                reference
                            }));
                        let spectra = trajectory
                            .spectra
                            .iter()
                            .map(|spectrum| RecordedFftSpectrumTransferMetadata {
                                request_key: spectrum.request_key.clone(),
                                evidence: spectrum.evidence.clone(),
                                frequency: push_transfer_slice(&mut buffers, &spectrum.frequency),
                                real: push_transfer_slice(&mut buffers, &spectrum.real),
                                imaginary: push_transfer_slice(&mut buffers, &spectrum.imaginary),
                            })
                            .collect();
                        ExecutionArtifactPayloadTransferMetadata::TransientTrajectory(Box::new(
                            TransientTrajectoryTransferMetadata { time, waveforms, convergence, spectra, current_impulses: trajectory.current_impulses.clone() },
                        ))
                    }
                    ExecutionArtifactPayload::PeriodicState(periodic) => {
                        let analysis = periodic.operating_point.analysis();
                        let config = periodic.operating_point.config();
                        let result = &analysis.result;
                        let time = push_transfer_slice(&mut buffers, &result.time);
                        let waveforms = result
                            .node_names
                            .iter()
                            .zip(&result.waveforms)
                            .map(|(node_name, waveform)| PeriodicWaveformTransferMetadata {
                                node_name: node_name.clone(),
                                values: push_transfer_slice(&mut buffers, &waveform.values),
                            })
                            .collect();
                        let branch_waveforms = result
                            .branch_names
                            .iter()
                            .zip(&result.branch_waveforms)
                            .map(|(node_name, waveform)| PeriodicWaveformTransferMetadata {
                                node_name: node_name.clone(),
                                values: push_transfer_slice(&mut buffers, &waveform.values),
                            })
                            .collect();
                        let result_floquet_real =
                            push_transfer_slice(&mut buffers, &periodic.result_floquet_real);
                        let result_floquet_imag =
                            push_transfer_slice(&mut buffers, &periodic.result_floquet_imag);
                        let monodromy = analysis
                            .monodromy
                            .iter()
                            .map(|row| push_transfer_slice(&mut buffers, row))
                            .collect();
                        let analysis_floquet_real =
                            push_transfer_slice(&mut buffers, &periodic.analysis_floquet_real);
                        let analysis_floquet_imag =
                            push_transfer_slice(&mut buffers, &periodic.analysis_floquet_imag);
                        let shooting_state = push_transfer_slice(
                            &mut buffers,
                            periodic.operating_point.shooting_state(),
                        );
                        ExecutionArtifactPayloadTransferMetadata::PeriodicState(Box::new(
                            PeriodicStateTransferMetadata {
                                environment: periodic.environment.clone(),
                                producer_identity: periodic
                                    .operating_point
                                    .producer_identity()
                                    .cloned(),
                                config_fundamental_freq: config.fundamental_freq,
                                config_num_harmonics: config.num_harmonics,
                                config_tstab: config.tstab,
                                config_max_iterations: config.max_iterations,
                                config_tolerance: config.tolerance,
                                config_abstol: config.abstol,
                                config_auto_period: config.auto_period,
                                config_oscillator_node: config.oscillator_node.clone(),
                                config_period_guess: config.period_guess,
                                config_tstab_periods: config.tstab_periods,
                                config_damping_factor: config.damping_factor,
                                config_max_period_change: config.max_period_change,
                                config_integration_method: config.integration_method.map(
                                    |method| match method {
                                        rspice_core::numerics::integration::IntegrationMethod::BackwardEuler => {
                                            0
                                        }
                                        rspice_core::numerics::integration::IntegrationMethod::Trapezoidal => 1,
                                        rspice_core::numerics::integration::IntegrationMethod::Gear2 => 2,
                                        rspice_core::numerics::integration::IntegrationMethod::TrapGear => 3,
                                    },
                                ),
                                config_points_per_period: config.points_per_period,
                                config_verbose: config.verbose,
                                result_period: result.period,
                                result_frequency: result.frequency,
                                result_iterations: result.iterations,
                                result_residual_norm: result.residual_norm,
                                time,
                                waveforms,
                                branch_waveforms,
                                period_detected: result.period_detected,
                                result_floquet_real,
                                result_floquet_imag,
                                floquet_evidence: periodic.floquet_evidence.clone(),
                                floquet_orbit_kind: periodic.floquet_orbit_kind,
                                trivial_floquet_multiplier_index: periodic
                                    .trivial_floquet_multiplier_index,
                                floquet_verdict: periodic.floquet_verdict,
                                floquet_authenticated: periodic.floquet_authenticated,
                                analysis_iterations: analysis.iterations,
                                analysis_final_residual: analysis.final_residual,
                                analysis_period: analysis.period,
                                monodromy,
                                analysis_floquet_real,
                                analysis_floquet_imag,
                                is_stable: analysis.is_stable,
                                analysis_is_stable: periodic.analysis_is_stable,
                                shooting_state_basis: periodic
                                    .operating_point
                                    .shooting_state_basis()
                                    .to_vec(),
                                shooting_state,
                            },
                        ))
                    }
                    ExecutionArtifactPayload::QpssState(state) => {
                        ExecutionArtifactPayloadTransferMetadata::QpssState(Box::new(
                            state.encode_transfer(&mut buffers),
                        ))
                    }
                    ExecutionArtifactPayload::HbState(state) => {
                        let spectra = state
                            .operating_point
                            .node_names()
                            .iter()
                            .enumerate()
                            .map(|(index, node_name)| HbSpectrumTransferMetadata {
                                node_name: node_name.clone(),
                                real: push_transfer_slice(
                                    &mut buffers,
                                    &state.spectral_real[index],
                                ),
                                imaginary: push_transfer_slice(
                                    &mut buffers,
                                    &state.spectral_imaginary[index],
                                ),
                            })
                            .collect();
                        let mna_branch_spectra = state
                            .operating_point
                            .mna_branch_names()
                            .iter()
                            .enumerate()
                            .map(|(index, branch_name)| HbBranchSpectrumTransferMetadata {
                                branch_name: branch_name.clone(),
                                real: push_transfer_slice(
                                    &mut buffers,
                                    &state.mna_branch_spectral_real[index],
                                ),
                                imaginary: push_transfer_slice(
                                    &mut buffers,
                                    &state.mna_branch_spectral_imaginary[index],
                                ),
                            })
                            .collect();
                        let integral_spectra = state
                            .operating_point
                            .integral_spectra()
                            .iter()
                            .enumerate()
                            .map(|(index, spectrum)| HbIntegralSpectrumTransferMetadata {
                                name: spectrum.name.clone(),
                                real: push_transfer_slice(
                                    &mut buffers,
                                    &state.integral_spectral_real[index],
                                ),
                                imaginary: push_transfer_slice(
                                    &mut buffers,
                                    &state.integral_spectral_imaginary[index],
                                ),
                            })
                            .collect();
                        ExecutionArtifactPayloadTransferMetadata::HbState(
                            Box::new(HbStateTransferMetadata {
                                environment: state.environment.clone(),
                                config: state.operating_point.config().clone(),
                                producer_identity: state
                                    .operating_point
                                    .producer_identity()
                                    .cloned(),
                                spectra,
                                mna_branch_spectra,
                                integral_spectra,
                                iterations: state.operating_point.iterations(),
                                residual_norm: state.operating_point.residual_norm(),
                            }),
                        )
                    }
                };
                ExecutionArtifactTransferMetadata {
                    snapshot_digest: artifact.snapshot_digest,
                    producer_instance_id: artifact.producer_instance_id,
                    producer_source_revision: artifact.producer_source_revision,
                    producer_config_digest: artifact.producer_config_digest,
                    kind: artifact.kind,
                    payload_digest: artifact.payload_digest,
                    payload,
                }
            })
            .collect();
        let metadata = ResolvedExecutionDependenciesTransferMetadata {
            source: self.source,
            snapshot_digest: self.snapshot_digest,
            bindings: self.bindings.clone(),
            artifacts,
        };
        let encoded = serde_json::to_string(&metadata).map_err(|error| {
            ExecutionArtifactError::Transport(format!(
                "could not encode dependency transfer metadata: {error}"
            ))
        })?;
        Ok((encoded, buffers))
    }

    /// Reconstruct and authenticate dependencies received from transfer
    /// buffers. Every buffer must be referenced exactly once and every
    /// reconstructed artifact must still match its prepared binding and
    /// content digest.
    #[cfg(any(target_arch = "wasm32", test))]
    pub(in crate::simulation) fn decode_transfer(
        encoded: &str,
        buffers: Vec<Vec<f64>>,
    ) -> Result<Self, ExecutionArtifactError> {
        const MAX_METADATA_BYTES: usize = 4 * 1024 * 1024;
        const MAX_TRANSFER_BUFFERS: usize = 65_536;
        if encoded.len() > MAX_METADATA_BYTES {
            return Err(ExecutionArtifactError::Transport(format!(
                "dependency metadata contains {} bytes, exceeding the {MAX_METADATA_BYTES}-byte limit",
                encoded.len()
            )));
        }
        if buffers.len() > MAX_TRANSFER_BUFFERS {
            return Err(ExecutionArtifactError::Transport(format!(
                "dependency transfer contains {} buffers, exceeding the {MAX_TRANSFER_BUFFERS}-buffer limit",
                buffers.len()
            )));
        }
        let numeric_values = buffers.iter().try_fold(0usize, |total, values| {
            total.checked_add(values.len()).ok_or_else(|| {
                ExecutionArtifactError::Transport(
                    "dependency transfer numeric size overflows this platform".to_owned(),
                )
            })
        })?;
        if numeric_values > PeriodicStateArtifact::MAX_NUMERIC_VALUES {
            return Err(ExecutionArtifactError::Transport(format!(
                "dependency transfer contains {numeric_values} numerical values, exceeding the {}-value limit",
                PeriodicStateArtifact::MAX_NUMERIC_VALUES
            )));
        }
        let metadata: ResolvedExecutionDependenciesTransferMetadata = serde_json::from_str(encoded)
            .map_err(|error| {
                ExecutionArtifactError::Transport(format!(
                    "could not decode dependency transfer metadata: {error}"
                ))
            })?;
        let mut buffers = buffers.into_iter().map(Some).collect::<Vec<_>>();
        let artifacts = metadata
            .artifacts
            .into_iter()
            .map(|artifact| {
                let payload = match artifact.payload {
                    ExecutionArtifactPayloadTransferMetadata::DcOperatingPointSeed(metadata) => {
                        let seed = DcOperatingPointSeedArtifact {
                            effective_source_content_digest: metadata
                                .effective_source_content_digest,
                            temperature_celsius: metadata.temperature_celsius,
                            supply_voltage: metadata.supply_voltage,
                            nominal_supply_voltage: metadata.nominal_supply_voltage,
                            supply_source_names: metadata.supply_source_names,
                            node_names: metadata.node_names,
                            branch_names: metadata.branch_names,
                            solution: take_transfer_buffer(&mut buffers, metadata.solution)?,
                        };
                        seed.validate()?;
                        ExecutionArtifactPayload::DcOperatingPointSeed(Arc::new(seed))
                    }
                    ExecutionArtifactPayloadTransferMetadata::TransientTrajectory(metadata) => {
                        let time = take_transfer_buffer(&mut buffers, metadata.time)?;
                        let waveforms = metadata
                            .waveforms
                            .into_iter()
                            .map(|(name, reference)| {
                                take_transfer_buffer(&mut buffers, reference)
                                    .map(|values| (name, values))
                            })
                            .collect::<Result<_, _>>()?;
                        let convergence = metadata.convergence.map(|quality|
                            quality.into_evidence(|reference| reference.len, |reference| take_transfer_buffer(&mut buffers, reference)
                                .map_err(|error| error.to_string())))
                            .transpose().map_err(ExecutionArtifactError::Transport)?.map(Arc::new);
                        let mut spectra = Vec::with_capacity(metadata.spectra.len());
                        for spectrum in metadata.spectra {
                            spectra.push(Arc::new(
                                crate::simulation::results::RecordedFftSpectrum {
                                    request_key: spectrum.request_key,
                                    evidence: spectrum.evidence,
                                    frequency: take_transfer_buffer(
                                        &mut buffers,
                                        spectrum.frequency,
                                    )?,
                                    real: take_transfer_buffer(&mut buffers, spectrum.real)?,
                                    imaginary: take_transfer_buffer(
                                        &mut buffers,
                                        spectrum.imaginary,
                                    )?,
                                },
                            ));
                        }
                        let trajectory = TransientTrajectoryArtifact { time, waveforms, convergence, spectra, current_impulses: metadata.current_impulses };
                        trajectory.validate()?;
                        ExecutionArtifactPayload::TransientTrajectory(Arc::new(trajectory))
                    }
                    ExecutionArtifactPayloadTransferMetadata::PeriodicState(metadata) => {
                        let time = take_transfer_buffer(&mut buffers, metadata.time)?;
                        let mut node_names = Vec::with_capacity(metadata.waveforms.len());
                        let mut waveforms = Vec::with_capacity(metadata.waveforms.len());
                        for waveform in metadata.waveforms {
                            node_names.push(waveform.node_name);
                            waveforms.push(
                                rspice_core::analysis::pss::PeriodicWaveform::from_values(
                                    take_transfer_buffer(&mut buffers, waveform.values)?,
                                ),
                            );
                        }
                        let mut branch_names = Vec::with_capacity(metadata.branch_waveforms.len());
                        let mut branch_waveforms = Vec::with_capacity(metadata.branch_waveforms.len());
                        for waveform in metadata.branch_waveforms {
                            branch_names.push(waveform.node_name);
                            branch_waveforms.push(
                                rspice_core::analysis::pss::PeriodicWaveform::from_values(
                                    take_transfer_buffer(&mut buffers, waveform.values)?,
                                ),
                            );
                        }
                        let result_floquet_real = take_transfer_buffer(
                            &mut buffers,
                            metadata.result_floquet_real,
                        )?;
                        let result_floquet_imag = take_transfer_buffer(
                            &mut buffers,
                            metadata.result_floquet_imag,
                        )?;
                        let result_floquet_multipliers = join_complex_values(
                            "PSS result Floquet",
                            &result_floquet_real,
                            &result_floquet_imag,
                        )?;
                        let monodromy = metadata
                            .monodromy
                            .into_iter()
                            .map(|reference| take_transfer_buffer(&mut buffers, reference))
                            .collect::<Result<Vec<_>, _>>()?;
                        let analysis_floquet_real = take_transfer_buffer(
                            &mut buffers,
                            metadata.analysis_floquet_real,
                        )?;
                        let analysis_floquet_imag = take_transfer_buffer(
                            &mut buffers,
                            metadata.analysis_floquet_imag,
                        )?;
                        let analysis_floquet_multipliers = join_complex_values(
                            "PSS analysis Floquet",
                            &analysis_floquet_real,
                            &analysis_floquet_imag,
                        )?;
                        let shooting_state =
                            take_transfer_buffer(&mut buffers, metadata.shooting_state)?;
                        let result = rspice_core::analysis::pss::PssResult {
                            period: metadata.result_period,
                            frequency: metadata.result_frequency,
                            iterations: metadata.result_iterations,
                            residual_norm: metadata.result_residual_norm,
                            time,
                            waveforms,
                            node_names,
                            branch_names,
                            branch_waveforms,
                            period_detected: metadata.period_detected,
                            floquet_multipliers: result_floquet_multipliers,
                            floquet_evidence: metadata.floquet_evidence.clone(),
                            floquet_orbit_kind: metadata.floquet_orbit_kind,
                            trivial_floquet_multiplier_index: metadata
                                .trivial_floquet_multiplier_index,
                        };
                        let computed_verdict = result.stability_verdict();
                        let computed_authenticated =
                            pss_floquet_contract_is_authenticated(&result, monodromy.len());
                        if metadata.floquet_verdict != computed_verdict {
                            return Err(ExecutionArtifactError::InvalidPayload(
                                "periodic-state transfer Floquet verdict does not match its evidence"
                                    .to_owned(),
                            ));
                        }
                        if metadata.floquet_authenticated != computed_authenticated
                            || !computed_authenticated
                        {
                            return Err(ExecutionArtifactError::InvalidPayload(
                                "periodic-state transfer lacks authenticated Floquet evidence"
                                    .to_owned(),
                            ));
                        }
                        let analysis = rspice_core::engine::PssAnalysisResult {
                            result,
                            iterations: metadata.analysis_iterations,
                            final_residual: metadata.analysis_final_residual,
                            period: metadata.analysis_period,
                            monodromy,
                            floquet_multipliers: analysis_floquet_multipliers,
                            is_stable: metadata.is_stable,
                        };
                        let integration_method = metadata
                            .config_integration_method
                            .map(|tag| match tag {
                                0 => Ok(rspice_core::numerics::integration::IntegrationMethod::BackwardEuler),
                                1 => Ok(rspice_core::numerics::integration::IntegrationMethod::Trapezoidal),
                                2 => Ok(rspice_core::numerics::integration::IntegrationMethod::Gear2),
                                3 => Ok(rspice_core::numerics::integration::IntegrationMethod::TrapGear),
                                _ => Err(ExecutionArtifactError::Transport(format!(
                                    "periodic-state transfer has unknown integration-method tag {tag}"
                                ))),
                            })
                            .transpose()?;
                        let config = rspice_core::analysis::PssConfig {
                            fundamental_freq: metadata.config_fundamental_freq,
                            num_harmonics: metadata.config_num_harmonics,
                            tstab: metadata.config_tstab,
                            max_iterations: metadata.config_max_iterations,
                            tolerance: metadata.config_tolerance,
                            abstol: metadata.config_abstol,
                            auto_period: metadata.config_auto_period,
                            oscillator_node: metadata.config_oscillator_node,
                            period_guess: metadata.config_period_guess,
                            tstab_periods: metadata.config_tstab_periods,
                            damping_factor: metadata.config_damping_factor,
                            max_period_change: metadata.config_max_period_change,
                            integration_method,
                            points_per_period: metadata.config_points_per_period,
                            verbose: metadata.config_verbose,
                        };
                        let operating_point = if let Some(producer_identity) =
                            metadata.producer_identity
                        {
                            rspice_core::engine::PssOperatingPoint::try_from_authenticated_parts(
                                producer_identity,
                                config,
                                analysis,
                                metadata.shooting_state_basis,
                                shooting_state,
                            )
                        } else {
                            rspice_core::engine::PssOperatingPoint::try_from_parts(
                                config,
                                analysis,
                                shooting_state,
                            )
                        }
                        .map_err(|error| {
                            ExecutionArtifactError::InvalidPayload(error.to_string())
                        })?;
                        let periodic = PeriodicStateArtifact {
                            environment: metadata.environment,
                            operating_point: Arc::new(operating_point),
                            result_floquet_real,
                            result_floquet_imag,
                            floquet_evidence: metadata.floquet_evidence,
                            floquet_orbit_kind: metadata.floquet_orbit_kind,
                            trivial_floquet_multiplier_index: metadata
                                .trivial_floquet_multiplier_index,
                            floquet_verdict: metadata.floquet_verdict,
                            floquet_authenticated: metadata.floquet_authenticated,
                            analysis_floquet_real,
                            analysis_floquet_imag,
                            analysis_is_stable: metadata.analysis_is_stable,
                        };
                        periodic.validate()?;
                        ExecutionArtifactPayload::PeriodicState(Arc::new(periodic))
                    }
                    ExecutionArtifactPayloadTransferMetadata::QpssState(metadata) => {
                        ExecutionArtifactPayload::QpssState(Arc::new(metadata.decode(&mut buffers)?))
                    }
                    ExecutionArtifactPayloadTransferMetadata::HbState(metadata) => {
                        let mut node_names = Vec::with_capacity(metadata.spectra.len());
                        let mut spectral_state = Vec::with_capacity(metadata.spectra.len());
                        let mut spectral_real = Vec::with_capacity(metadata.spectra.len());
                        let mut spectral_imaginary = Vec::with_capacity(metadata.spectra.len());
                        for spectrum in metadata.spectra {
                            node_names.push(spectrum.node_name);
                            let real = take_transfer_buffer(&mut buffers, spectrum.real)?;
                            let imaginary =
                                take_transfer_buffer(&mut buffers, spectrum.imaginary)?;
                            spectral_state.push(join_complex_values(
                                "HB spectral row",
                                &real,
                                &imaginary,
                            )?);
                            spectral_real.push(real);
                            spectral_imaginary.push(imaginary);
                        }
                        let mut mna_branch_names =
                            Vec::with_capacity(metadata.mna_branch_spectra.len());
                        let mut mna_branch_spectral_state =
                            Vec::with_capacity(metadata.mna_branch_spectra.len());
                        let mut mna_branch_spectral_real =
                            Vec::with_capacity(metadata.mna_branch_spectra.len());
                        let mut mna_branch_spectral_imaginary =
                            Vec::with_capacity(metadata.mna_branch_spectra.len());
                        for spectrum in metadata.mna_branch_spectra {
                            mna_branch_names.push(spectrum.branch_name);
                            let real = take_transfer_buffer(&mut buffers, spectrum.real)?;
                            let imaginary =
                                take_transfer_buffer(&mut buffers, spectrum.imaginary)?;
                            mna_branch_spectral_state.push(join_complex_values(
                                "HB MNA branch spectral row",
                                &real,
                                &imaginary,
                            )?);
                            mna_branch_spectral_real.push(real);
                            mna_branch_spectral_imaginary.push(imaginary);
                        }
                        let mut integral_spectra =
                            Vec::with_capacity(metadata.integral_spectra.len());
                        let mut integral_spectral_real =
                            Vec::with_capacity(metadata.integral_spectra.len());
                        let mut integral_spectral_imaginary =
                            Vec::with_capacity(metadata.integral_spectra.len());
                        for spectrum in metadata.integral_spectra {
                            let real = take_transfer_buffer(&mut buffers, spectrum.real)?;
                            let imaginary =
                                take_transfer_buffer(&mut buffers, spectrum.imaginary)?;
                            integral_spectra.push(rspice_core::engine::HbIntegralSpectrum {
                                name: spectrum.name,
                                coefficients: join_complex_values(
                                    "HB integral spectral row",
                                    &real,
                                    &imaginary,
                                )?,
                            });
                            integral_spectral_real.push(real);
                            integral_spectral_imaginary.push(imaginary);
                        }
                        let operating_point = rspice_core::engine::HbOperatingPoint::try_from_complete_parts(
                            metadata.config,
                            node_names,
                            spectral_state,
                            mna_branch_names,
                            mna_branch_spectral_state,
                            integral_spectra,
                            metadata.iterations,
                            metadata.residual_norm,
                            metadata.producer_identity,
                        )
                        .map_err(|error| {
                            ExecutionArtifactError::InvalidPayload(error.to_string())
                        })?;
                        let state = HbStateArtifact {
                            environment: metadata.environment,
                            operating_point: Arc::new(operating_point),
                            spectral_real,
                            spectral_imaginary,
                            mna_branch_spectral_real,
                            mna_branch_spectral_imaginary,
                            integral_spectral_real,
                            integral_spectral_imaginary,
                        };
                        state.validate()?;
                        ExecutionArtifactPayload::HbState(Arc::new(state))
                    }
                };
                Ok(ExecutionArtifactEnvelope {
                    snapshot_digest: artifact.snapshot_digest,
                    producer_instance_id: artifact.producer_instance_id,
                    producer_source_revision: artifact.producer_source_revision,
                    producer_config_digest: artifact.producer_config_digest,
                    kind: artifact.kind,
                    payload_digest: artifact.payload_digest,
                    payload,
                })
            })
            .collect::<Result<Vec<_>, ExecutionArtifactError>>()?;
        if let Some(unused) = buffers.iter().position(Option::is_some) {
            return Err(ExecutionArtifactError::Transport(format!(
                "dependency transfer buffer {unused} is unreferenced"
            )));
        }

        let resolved = Self {
            source: metadata.source,
            snapshot_digest: metadata.snapshot_digest,
            bindings: metadata.bindings,
            artifacts,
        };
        resolved.validate_transport_integrity()?;
        Ok(resolved)
    }

    #[cfg(any(target_arch = "wasm32", test))]
    fn validate_transport_integrity(&self) -> Result<(), ExecutionArtifactError> {
        if self.bindings.is_empty() && self.artifacts.is_empty() {
            return if self.snapshot_digest.is_none() {
                Ok(())
            } else {
                Err(ExecutionArtifactError::Transport(
                    "artifact-free dependencies carry a snapshot identity".to_owned(),
                ))
            };
        }
        let snapshot_digest = self.snapshot_digest.ok_or_else(|| {
            ExecutionArtifactError::Transport(
                "dependency transfer has no prepared snapshot identity".to_owned(),
            )
        })?;
        if self.bindings.len() != self.artifacts.len() {
            return Err(ExecutionArtifactError::Transport(format!(
                "dependency transfer contains {} bindings and {} artifacts",
                self.bindings.len(),
                self.artifacts.len()
            )));
        }
        for (binding, artifact) in self.bindings.iter().zip(&self.artifacts) {
            artifact.validate_against(snapshot_digest, binding)?;
        }
        Ok(())
    }
}
