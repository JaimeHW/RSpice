//! Bind continuation images only after every task's circuit and run point exist.
use super::*;
use crate::engine_bridge::EngineBridge;
use crate::transient_checkpoint::{TransientCheckpointInput, decode_bytes};
use rspice_core::{AbortSignal, ResourceLimits};
use rspice_results::transient_checkpoint::TransientCheckpointEvidence;

impl PreparedTask {
    pub(crate) fn with_transient_resumes(
        mut self,
        mut images: Vec<TransientCheckpointEvidence>,
    ) -> Self {
        images.sort_by_key(TransientCheckpointEvidence::digest);
        self.transient_resumes = images.into();
        self.config_digest = self.payload_digest();
        self
    }
}

pub(super) fn digest_with_resumes(
    base: ContentDigest,
    images: &[TransientCheckpointEvidence],
) -> ContentDigest {
    if images.is_empty() {
        return base;
    }
    let mut writer = CanonicalWriter::new("rspice.prepared-transient-resumes/v1");
    writer.digest(base);
    writer.usize(images.len());
    for image in images {
        writer.digest(image.digest());
    }
    writer.finish()
}

fn invalid(message: impl Into<String>) -> PreparationError {
    PreparationError::new(PreparationStage::AnalysisPlan, message)
}

fn preparation_error(error: crate::error::SimulationError) -> PreparationError {
    PreparationError::from_simulation(
        PreparationStage::AnalysisPlan,
        "Transient checkpoint preparation",
        error,
    )
}

pub(super) fn route_resumes(
    parts: &mut SnapshotParts,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<(), PreparationError> {
    if !parts.tasks.iter().any(|task| {
        task.task.spec_options.tran_checkpoint.is_some() || !task.transient_resumes.is_empty()
    }) {
        return Ok(());
    }
    PreparationError::check_abort(abort)?;
    parts
        .project_veriloga_runtimes
        .install()
        .map_err(|error| PreparationError::new(PreparationStage::ModelBindings, error))?;
    let mut bridge_limits = limits;
    bridge_limits.max_netlist_bytes = limits.max_expanded_source_bytes;
    let bridge = EngineBridge::new()
        .with_resource_limits(bridge_limits)
        .with_measurement_references(parts.measurement_references.clone());
    let mut required = HashSet::new();
    let mut used = HashSet::new();
    let mut selections: HashMap<AnalysisInstanceId, Arc<[TransientCheckpointEvidence]>> =
        HashMap::new();
    let mut selected_bytes = 0usize;
    for task in &parts.tasks {
        if task.transient_resumes.is_empty() {
            continue;
        }
        if let Some(previous) = selections.get(&task.authored_instance_id) {
            if !Arc::ptr_eq(previous, &task.transient_resumes) {
                return Err(invalid(
                    "Expanded transient tasks disagree about selected checkpoints",
                ));
            }
            continue;
        }
        selections.insert(task.authored_instance_id, task.transient_resumes.clone());
        for image in task.transient_resumes.iter() {
            if !required.insert((task.authored_instance_id, image.digest())) {
                return Err(invalid("Repeated transient checkpoint selection"));
            }
            selected_bytes = selected_bytes.saturating_add(image.bytes().len());
        }
    }
    PreparationError::check_limit(
        rspice_core::ResourceKind::ExternalDataBytes,
        selected_bytes,
        limits.max_external_data_bytes,
    )?;
    let mut changes = HashMap::new();
    let mut resumed_times = HashMap::new();
    let mut routed_bytes = 0usize;
    for task in &mut parts.tasks {
        PreparationError::check_abort(abort)?;
        let Some(mut request) = task.task.spec_options.tran_checkpoint.clone() else {
            if !task.transient_resumes.is_empty() {
                return Err(invalid(
                    "Selected transient images have no continuation policy",
                ));
            }
            continue;
        };
        // A small-signal card around a checkpoint's state carries the image
        // it reads and no schedule; the engine validates the image against
        // the task's deck when it restores it.
        if matches!(
            task.task.spec,
            AnalysisSpec::Ac { .. } | AnalysisSpec::Noise { .. }
        ) && request.times.is_empty()
            && task.transient_resumes.is_empty()
            && let Some(input) = &request.resume
        {
            routed_bytes = routed_bytes.saturating_add(input.byte_len());
            PreparationError::check_limit(
                rspice_core::ResourceKind::ExternalDataBytes,
                routed_bytes,
                limits.max_external_data_bytes,
            )?;
            continue;
        }
        let AnalysisSpec::Transient { stop_time, .. } = task.task.spec else {
            return Err(invalid(
                "Transient checkpoint requires an ordinary transient task",
            ));
        };
        if task.config_digest != task.payload_digest() {
            return Err(invalid("Transient task payload changed after preparation"));
        }
        request
            .validate_times(stop_time, limits)
            .map_err(preparation_error)?;
        let config = crate::analysis_preparation::analysis_spec_to_config(&task.task.spec)
            .map_err(invalid)?;
        let source = task
            .executable_netlist_override
            .as_deref()
            .unwrap_or(&parts.executable_netlist);
        if !task.transient_resumes.is_empty() {
            if request.resume.is_some() {
                return Err(invalid(
                    "Transient continuation has two competing selection policies",
                ));
            }
            let mut chosen = None;
            let mut reason = None;
            for image in task.transient_resumes.iter() {
                PreparationError::check_abort(abort)?;
                let checkpoint =
                    decode_bytes(image.bytes(), limits, abort).map_err(preparation_error)?;
                let times: Vec<_> = request
                    .times
                    .iter()
                    .copied()
                    .filter(|time| *time > checkpoint.time)
                    .collect();
                match bridge.validate_transient_checkpoint(
                    &config,
                    source,
                    task.execution_environment.clone(),
                    Some(&checkpoint),
                    &times,
                    abort,
                ) {
                    Ok(()) => {
                        if chosen.is_some() {
                            return Err(invalid(format!(
                                "{}: more than one selected checkpoint matches this run point; select one accepted state for each point",
                                task.label
                            )));
                        }
                        chosen = Some((image.clone(), times));
                    }
                    Err(error)
                        if matches!(
                            error,
                            crate::error::SimulationError::Aborted
                                | crate::error::SimulationError::ResourceLimit { .. }
                                | crate::error::SimulationError::ResourceFailure(_)
                        ) =>
                    {
                        return Err(preparation_error(error));
                    }
                    Err(error) => {
                        reason = Some(error.to_string());
                    }
                }
            }
            let (image, times) = chosen.ok_or_else(|| invalid(format!(
                "{}: no selected checkpoint matches this run point. Restore compatible circuit/settings or clear the resume selection to run fresh. {}",
                task.label, reason.unwrap_or_default(),
            )))?;
            routed_bytes = routed_bytes.saturating_add(image.bytes().len());
            PreparationError::check_limit(
                rspice_core::ResourceKind::ExternalDataBytes,
                routed_bytes,
                limits.max_external_data_bytes,
            )?;
            request.times = times;
            request.resume = Some(
                TransientCheckpointInput::from_bytes_with_limits(
                    image.bytes().to_vec(),
                    limits,
                    abort,
                )
                .map_err(preparation_error)?,
            );
            resumed_times.insert(task.instance_id, image.accepted_time());
            used.insert((task.authored_instance_id, image.digest()));
        } else {
            let checkpoint = request
                .resume
                .as_ref()
                .map(|image| image.decode(limits, abort))
                .transpose()
                .map_err(preparation_error)?;
            if let Some(checkpoint) = &checkpoint {
                resumed_times.insert(task.instance_id, checkpoint.time);
            }
            bridge
                .validate_transient_checkpoint(
                    &config,
                    source,
                    task.execution_environment.clone(),
                    checkpoint.as_ref(),
                    &request.times,
                    abort,
                )
                .map_err(preparation_error)?;
            if let Some(input) = &request.resume {
                routed_bytes = routed_bytes.saturating_add(input.byte_len());
                PreparationError::check_limit(
                    rspice_core::ResourceKind::ExternalDataBytes,
                    routed_bytes,
                    limits.max_external_data_bytes,
                )?;
            }
        }
        request
            .validate_schedule(stop_time, limits)
            .map_err(|error| invalid(error.to_string()))?;
        let previous = task.config_digest;
        task.task.spec_options.tran_checkpoint = Some(request);
        task.transient_resumes = Arc::from([]);
        task.config_digest = task.payload_digest();
        if task.config_digest != previous {
            changes.insert(
                task.instance_id,
                (task.source_revision, previous, task.config_digest),
            );
        }
    }
    if !required.is_subset(&used) {
        return Err(invalid(
            "A selected transient checkpoint matches no requested Run Set point; include its compatible point or clear the unused selection",
        ));
    }
    // Point routing changes the producer payload before the snapshot is sealed.
    // Rebind only edges that authenticated the previous producer; never repair
    // an already forged or stale edge into a valid dependency.
    for task in &mut parts.tasks {
        for binding in &mut task.dependency_bindings {
            let producer = binding.producer_instance_id();
            if let Some(&accepted_time) = resumed_times.get(&producer)
                && let AnalysisSpec::Fourier { start_time, .. } = task.task.spec
                && start_time < accepted_time
            {
                return Err(invalid(format!(
                    "{}: Fourier window starts before the checkpoint at {accepted_time:.12e} s; select an earlier checkpoint or a later Fourier window",
                    task.label
                )));
            }
            if let Some(&(revision, before, after)) = changes.get(&producer)
                && binding.producer_source_revision() == revision
                && binding.producer_config_digest() == before
            {
                binding.rebind_producer(producer, revision, after);
            }
        }
    }
    Ok(())
}
