//! Expands authored PVT points into exact executable tasks.
//!
//! Applies per-point process, temperature, supply, and parameter overrides
//! while retaining the operating-point contract used by authorization.

use super::*;

/// Whether a point is the run's own reference point.
///
/// Exact comparison, not a tolerance: the reference process and temperature
/// are the same values the axes were resolved from, so a point that sits on
/// the reference carries its bits unchanged. A supply axis is nominal only at
/// the resolved nominal supply; a run with no supply axis left the deck's own
/// supply standing, which is nominal by construction.
pub(super) fn point_is_nominal(
    point: &PreparedPvtPoint,
    nominal_supply_voltage: Option<f64>,
    reference_process: ProcessCorner,
    reference_temperature_celsius: f64,
) -> bool {
    if point.process != reference_process
        || point.temperature_celsius != reference_temperature_celsius
    {
        return false;
    }
    match (point.voltage, nominal_supply_voltage) {
        (None, _) => true,
        (Some(voltage), Some(nominal)) => voltage == nominal,
        (Some(_), None) => false,
    }
}

/// Expand every task that is declared over the run set's PVT points into one
/// prepared task per point.
///
/// Three declarations reach this: an operating point bound to the run-set
/// axis, and the corner run and the temperature step, whose base analysis *is*
/// the thing declared over the points. All expand through the same per-point
/// ingredients — the process corner materialized into the point's own deck,
/// the point recorded on the task so its result can be attributed, and an
/// instance identity derived from the point so two points can never share one.
pub(super) fn expand_pvt_point_tasks(
    tasks: Vec<PreparedTask>,
    pvt_points: &[PreparedPvtPoint],
    executable_netlist: &str,
    reference_process: ProcessCorner,
    reference_temperature_celsius: f64,
) -> Result<Vec<PreparedTask>, PreparationError> {
    use crate::simulation::AnalysisConfig;
    use crate::simulation::dialog::{OpRunPointContext, OpTemperatureMode};

    let mut expanded = Vec::with_capacity(tasks.len().saturating_add(pvt_points.len()));
    let mut final_task = HashMap::<
        AnalysisInstanceId,
        (
            AnalysisInstanceId,
            ObjectRevision,
            ContentDigest,
            Option<String>,
        ),
    >::new();

    for mut prepared in tasks {
        let original_identity = prepared.instance_id;
        prepared.dependencies = prepared
            .dependencies
            .iter()
            .map(|dependency| {
                final_task
                    .get(dependency)
                    .map_or(*dependency, |(identity, _, _, _)| *identity)
            })
            .collect();
        let inherited_op_source_override = prepared
            .dependency_bindings
            .iter()
            .find(|binding| {
                matches!(
                    binding.kind(),
                    ExecutionArtifactKind::DcOperatingPointSeed
                        | ExecutionArtifactKind::QpssState
                        | ExecutionArtifactKind::PeriodicState
                        | ExecutionArtifactKind::HbState
                )
            })
            .and_then(|binding| final_task.get(&binding.producer_instance_id()))
            .map(|(_, _, _, source)| source.clone());
        for binding in &mut prepared.dependency_bindings {
            if let Some((identity, revision, digest, _)) =
                final_task.get(&binding.producer_instance_id())
            {
                binding.rebind_producer(*identity, *revision, *digest);
            }
        }
        if matches!(
            prepared.task.spec,
            AnalysisSpec::Pss {
                method: crate::simulation::multi_run::PssMethod::Shooting,
                ..
            } | AnalysisSpec::Qpss { .. }
                | AnalysisSpec::HarmonicBalance { .. }
                | AnalysisSpec::Hbsp { .. }
                | AnalysisSpec::Hbnoise { .. }
                | AnalysisSpec::Qpac { .. }
                | AnalysisSpec::Qpxf { .. }
                | AnalysisSpec::Qpnoise { .. }
                | AnalysisSpec::Pac
                | AnalysisSpec::Pxf
                | AnalysisSpec::Pnoise
                | AnalysisSpec::Pstb
                | AnalysisSpec::Psp { .. }
                | AnalysisSpec::PssSpectrum { .. }
        ) && let Some(source) = inherited_op_source_override
        {
            prepared.executable_netlist_override = source;
        }

        // A corner run and a temperature step are not one analysis swept along
        // an axis: each is a base analysis solved once per declared point, and
        // collapsing those solves into one scalar per node is what threw the
        // waveforms and the `.MEAS` results away. Each point earns its own task
        // so its result carries its own evidence and the point that produced
        // it.
        //
        // The declaration is no longer solved. Solving it would solve every
        // point a second time, and the family its plot reads is one scalar per
        // node per point — a view over the point results. It stays a task
        // because the run's authenticated receipt is built from this list and
        // the retained results must line up with it one for one; its turn in
        // the queue assembles the family instead of reaching the engine.
        //
        // Its position after the points is load-bearing. Results must remain an
        // exact ordered prefix of the receipt's tasks even when a run is
        // aborted or a task is blocked, and the assembly cannot produce a
        // result until its points have. Any earlier position would leave a hole
        // at the declaration's own index in every partial run.
        //
        // Its points are deliberately not its dependencies. The queue is
        // strictly vector order — a `VecDeque` popped from the front with one
        // task in flight — so the position alone orders it, whereas a
        // dependency edge would skip the family outright the moment a single
        // point failed to converge, which is the point a plot most needs.
        //
        // A dependent of the declaration binds to the last *point*: the
        // assembly produces no execution artifact, and the last point is the
        // task whose completion means the declared space has been solved. That
        // is the same rule the operating-point expansion below applies.
        let declared_point_tasks = match &prepared.task.spec {
            AnalysisSpec::Corner => Some(expand_corner_run_point_tasks(
                &prepared,
                executable_netlist,
                reference_process,
                reference_temperature_celsius,
            )?),
            // A parametric run declares a PVT space only when it carries a
            // temperature contract. Without one it steps a design parameter,
            // which the engine sweeps for itself inside a single solve and
            // which is not a condition a specification can be scoped to.
            AnalysisSpec::Parametric => match prepared.task.spec_options.temp.as_ref() {
                Some(contract) => Some(expand_temperature_run_point_tasks(
                    &prepared,
                    contract,
                    executable_netlist,
                    reference_process,
                    reference_temperature_celsius,
                )?),
                None => None,
            },
            _ => None,
        };
        if let Some(points) = declared_point_tasks {
            let last = points
                .last()
                .expect("an expansion refuses an empty declared space");
            final_task.insert(
                original_identity,
                (
                    last.instance_id,
                    last.source_revision,
                    last.config_digest,
                    last.executable_netlist_override.clone(),
                ),
            );
            expanded.extend(points);
            expanded.push(prepared);
            continue;
        }

        let Some(base_config) = operating_point_config(&prepared.task.spec) else {
            final_task.insert(
                original_identity,
                (
                    original_identity,
                    prepared.source_revision,
                    prepared.config_digest,
                    prepared.executable_netlist_override.clone(),
                ),
            );
            expanded.push(prepared);
            continue;
        };
        if !matches!(
            base_config.temperature_mode,
            OpTemperatureMode::PvtRunSet | OpTemperatureMode::ActiveRunSetAxis
        ) {
            final_task.insert(
                original_identity,
                (
                    original_identity,
                    prepared.source_revision,
                    prepared.config_digest,
                    prepared.executable_netlist_override.clone(),
                ),
            );
            expanded.push(prepared);
            continue;
        }

        let base_dependencies = prepared.dependencies.clone();
        let original_label = prepared.label.clone();
        let mut previous_point_identity = None;
        for (index, point) in pvt_points.iter().enumerate() {
            let mut point_task = prepared.clone();
            if pvt_points.len() > 1 {
                let derivation = prepared.derive(operating_point_run_point_role(
                    index,
                    pvt_points.len(),
                    point,
                ));
                point_task.adopt_derived_identity(derivation);
            }
            let instance_id = point_task.instance_id;
            point_task.dependencies = base_dependencies.clone();
            if let Some(previous) = previous_point_identity {
                point_task.dependencies.push(previous);
            }

            let mut config = base_config.clone();
            config.temperature_celsius = point.temperature_celsius;
            let (source_override, nominal_supply_voltage) =
                prepare_pvt_point_source(executable_netlist, point)?;
            config.run_point = OpRunPointContext {
                index,
                count: pvt_points.len(),
                process: point.process,
                supply_voltage: point.voltage,
                nominal_supply_voltage,
                supply_source_names: point.supply_source_names.clone(),
            };
            point_task.pvt_point = Some(
                crate::state::AnalysisResultPvtPoint::new(
                    point.process.short_name(),
                    point.voltage,
                    point.temperature_celsius,
                    point.corner_contract.as_ref().map(corner_contract_digest),
                    point_is_nominal(
                        point,
                        nominal_supply_voltage,
                        reference_process,
                        reference_temperature_celsius,
                    ),
                )
                .map_err(|error| {
                    PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        format!(
                            "Operating-point run point {}/{} cannot be attributed: {error}",
                            index + 1,
                            pvt_points.len()
                        ),
                    )
                })?,
            );
            point_task.executable_netlist_override = source_override;
            point_task.task.spec = operating_point_spec(&config);
            point_task.task.config = Some(AnalysisConfig::DcOp(config));
            if pvt_points.len() > 1 {
                point_task.label =
                    run_set_point_task_label(&original_label, point, index, pvt_points.len());
            }
            point_task.saved_output_contracts = prepared
                .saved_output_contracts
                .iter()
                .map(|contract| {
                    contract
                        .rebind_analysis(instance_id, &point_task.task.spec)
                        .map_err(|error| {
                            PreparationError::new(
                                PreparationStage::AnalysisPlan,
                                format!(
                                    "Failed to bind saved output to operating-point run point {}/{}: {error}",
                                    index + 1,
                                    pvt_points.len()
                                ),
                            )
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            point_task.config_digest = point_task.payload_digest();
            previous_point_identity = Some(instance_id);
            expanded.push(point_task);
        }
        let final_identity = previous_point_identity.expect("validated PVT point set is non-empty");
        let final_prepared = expanded
            .last()
            .expect("expanded operating-point task retains its final point");
        final_task.insert(
            original_identity,
            (
                final_identity,
                final_prepared.source_revision,
                final_prepared.config_digest,
                final_prepared.executable_netlist_override.clone(),
            ),
        );
    }

    Ok(expanded)
}

pub(super) fn prepare_pvt_point_source(
    executable_netlist: &str,
    point: &PreparedPvtPoint,
) -> Result<(Option<String>, Option<f64>), PreparationError> {
    let Some(contract) = point.corner_contract.as_ref() else {
        if point.voltage.is_some() {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                "PVT run point voltage is missing its authenticated corner contract",
            ));
        }
        let source = materialize_source_overrides(executable_netlist, &point.source_overrides)?;
        let source = materialize_parameter_overrides(&source, &point.parameter_overrides);
        return Ok(((source != executable_netlist).then_some(source), None));
    };

    let source = crate::services::simulation_runner::materialize_corner_process_source(
        executable_netlist,
        contract,
        process_to_corner_runner(point.process),
        &rspice_core::NoAbort,
    )
    .map_err(|error| {
        PreparationError::new(
            PreparationStage::ModelBindings,
            format!(
                "Failed to materialize the {} PVT process corner: {error}",
                point.process.short_name()
            ),
        )
    })?;

    let nominal_supply_voltage = if point.voltage.is_some() {
        let parsed = rspice_core::Netlist::parse(&source).map_err(|error| {
            PreparationError::new(
                PreparationStage::Netlist,
                format!("Prepared operating-point corner source is invalid: {error}"),
            )
            .at_line(super::super::parse_error_line(&error))
        })?;
        Some(match contract.nominal_voltage {
            Some(voltage) => voltage,
            None => crate::services::simulation_runner::infer_nominal_supply_voltage(
                &parsed,
                &point.supply_source_names,
                &rspice_core::NoAbort,
            )
            .map_err(|error| {
                PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!("Failed to resolve nominal PVT supply voltage: {error}"),
                )
            })?
            .ok_or_else(|| {
                PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    "A PVT supply axis requires a non-zero independent DC supply or an explicit nominal voltage",
                )
            })?,
        })
    } else {
        None
    };

    let source = materialize_source_overrides(&source, &point.source_overrides)?;
    let source = materialize_parameter_overrides(&source, &point.parameter_overrides);
    let source_override = (source != executable_netlist).then_some(source);
    Ok((source_override, nominal_supply_voltage))
}

/// Freeze a global Run Set point into a spec-driven task's exact deck.
///
/// Service runners resolve `.OPTIONS TEMP` through their shared engine-config
/// builder. Supply scaling is materialized as exact DC card values because the
/// parsed-netlist environment hook is intentionally owned by configuration
/// requests and Monte Carlo. Selection and validation mirror the core supply
/// scaler: only explicitly bound independent voltage sources with a `DC` or
/// `DC ... AC ...` value are eligible.
pub(super) fn materialize_spec_run_environment_source(
    executable_netlist: &str,
    environment: &crate::simulation::runner::AnalysisExecutionEnvironment,
    point_index: usize,
    point_count: usize,
) -> Result<String, PreparationError> {
    if !environment.temperature_celsius.is_finite() || environment.temperature_celsius <= -273.15 {
        return Err(PreparationError::new(
            PreparationStage::AnalysisPlan,
            format!(
                "Run Set point {}/{} has an invalid temperature",
                point_index + 1,
                point_count
            ),
        ));
    }

    let mut source = executable_netlist.to_owned();
    match (
        environment.supply_voltage,
        environment.nominal_supply_voltage,
    ) {
        (None, None) => {}
        (Some(supply), Some(nominal)) => {
            if !supply.is_finite() || supply <= 0.0 || !nominal.is_finite() || nominal <= 0.0 {
                return Err(PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!(
                        "Run Set point {}/{} has an invalid supply or nominal voltage",
                        point_index + 1,
                        point_count
                    ),
                ));
            }
            if environment.supply_source_names.is_empty() {
                return Err(PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!(
                        "Run Set point {}/{} has a supply value without an explicitly bound source",
                        point_index + 1,
                        point_count
                    ),
                ));
            }
            let scale = supply / nominal;
            for source_name in &environment.supply_source_names {
                let parsed = rspice_core::Netlist::parse(&source).map_err(|error| {
                    let line = super::super::parse_error_line(&error);
                    PreparationError::new(
                        PreparationStage::Netlist,
                        format!(
                            "Cannot materialize Run Set supply {source_name:?} at point {}/{}: {error}",
                            point_index + 1,
                            point_count
                        ),
                    )
                    .at_line(line)
                })?;
                let element = parsed
                    .elements
                    .iter()
                    .find(|element| element.name.eq_ignore_ascii_case(source_name))
                    .ok_or_else(|| {
                        PreparationError::new(
                            PreparationStage::AnalysisPlan,
                            format!(
                                "Run Set supply source {source_name:?} is absent at point {}/{}",
                                point_index + 1,
                                point_count
                            ),
                        )
                    })?;
                let rspice_core::netlist::ElementKind::VoltageSource(spec) = &element.kind else {
                    return Err(PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        format!(
                            "Run Set supply binding {source_name:?} is not an independent voltage source"
                        ),
                    ));
                };
                let dc = scalable_supply_dc_value(spec).ok_or_else(|| {
                    PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        format!("Run Set supply source {source_name:?} has no scalable DC value"),
                    )
                })?;
                let scaled = dc * scale;
                if !scaled.is_finite() {
                    return Err(PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        format!(
                            "Run Set supply source {source_name:?} overflowed at point {}/{}",
                            point_index + 1,
                            point_count
                        ),
                    ));
                }
                source = replace_source_dc_card(&source, source_name, &scaled.to_string())
                    .ok_or_else(|| {
                        PreparationError::new(
                            PreparationStage::Netlist,
                            format!(
                                "Run Set could not locate the authored card for supply source {source_name:?}"
                            ),
                        )
                    })?;
            }
        }
        _ => {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!(
                    "Run Set point {}/{} must provide supply and nominal voltage together",
                    point_index + 1,
                    point_count
                ),
            ));
        }
    }

    Ok(splice_before_terminal_end_card(
        &source,
        &format!(".OPTIONS TEMP={}", environment.temperature_celsius),
    ))
}

fn scalable_supply_dc_value(spec: &rspice_core::netlist::SourceSpec) -> Option<f64> {
    match spec {
        rspice_core::netlist::SourceSpec::Dc(value) => Some(*value),
        rspice_core::netlist::SourceSpec::DcAc { dc_value, .. } => Some(*dc_value),
        _ => None,
    }
}

fn materialize_parameter_overrides(
    executable_netlist: &str,
    overrides: &[(String, String)],
) -> String {
    if overrides.is_empty() {
        return executable_netlist.to_owned();
    }
    let mut block = String::new();
    for (name, value) in overrides {
        block.push_str(".param ");
        block.push_str(name);
        block.push('=');
        block.push_str(value);
        block.push('\n');
    }
    splice_before_terminal_end_card(executable_netlist, block.trim_end())
}

fn materialize_source_overrides(
    executable_netlist: &str,
    overrides: &[(String, String)],
) -> Result<String, PreparationError> {
    let mut source = executable_netlist.to_owned();
    for (name, value) in overrides {
        let parsed = rspice_core::Netlist::parse(&source).map_err(|error| {
            PreparationError::new(
                PreparationStage::Netlist,
                format!("Cannot validate Run Set source binding {name:?}: {error}"),
            )
            .at_line(super::super::parse_error_line(&error))
        })?;
        let element = parsed
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(name))
            .ok_or_else(|| {
                PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!("Run Set source binding {name:?} is absent from the executable deck"),
                )
            })?;
        let spec = match &element.kind {
            rspice_core::netlist::ElementKind::VoltageSource(spec)
            | rspice_core::netlist::ElementKind::CurrentSource(spec) => spec,
            _ => {
                return Err(PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!("Run Set source binding {name:?} is not an independent source"),
                ));
            }
        };
        if !source_spec_has_explicit_dc(spec) {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!(
                    "Run Set source binding {name:?} has no explicit DC value that can be replaced"
                ),
            ));
        }
        source = replace_source_dc_card(&source, name, value).ok_or_else(|| {
            PreparationError::new(
                PreparationStage::Netlist,
                format!("Run Set could not locate the authored card for source {name:?}"),
            )
        })?;
    }
    Ok(source)
}

fn source_spec_has_explicit_dc(spec: &rspice_core::netlist::SourceSpec) -> bool {
    use rspice_core::netlist::SourceSpec;
    match spec {
        SourceSpec::Dc(_)
        | SourceSpec::DcAc { .. }
        | SourceSpec::DcTransient { .. }
        | SourceSpec::DcAcTransient { .. } => true,
        SourceSpec::Distortion { inner, .. } | SourceSpec::RfPort { inner, .. } => {
            source_spec_has_explicit_dc(inner)
        }
        _ => false,
    }
}

fn replace_source_dc_card(source: &str, source_name: &str, value: &str) -> Option<String> {
    let mut replaced = false;
    let mut inside_subcircuit = false;
    let mut output = String::with_capacity(source.len() + value.len());
    for line in source.split_inclusive('\n') {
        let newline = line.ends_with('\n');
        let content = line.trim_end_matches(['\r', '\n']);
        let mut tokens = content.split_whitespace().collect::<Vec<_>>();
        let directive = tokens.first().copied().unwrap_or_default();
        if directive.eq_ignore_ascii_case(".subckt") {
            inside_subcircuit = true;
        }
        if !replaced
            && !inside_subcircuit
            && tokens
                .first()
                .is_some_and(|token| token.eq_ignore_ascii_case(source_name))
            && tokens.len() >= 4
        {
            let value_index = tokens
                .iter()
                .position(|token| token.eq_ignore_ascii_case("DC"))
                .and_then(|index| (index + 1 < tokens.len()).then_some(index + 1))
                .unwrap_or(3);
            tokens[value_index] = value;
            output.push_str(&tokens.join(" "));
            replaced = true;
        } else {
            output.push_str(content);
        }
        if directive.eq_ignore_ascii_case(".ends") {
            inside_subcircuit = false;
        }
        if newline {
            output.push('\n');
        }
    }
    replaced.then_some(output)
}

pub(super) fn operating_point_config(
    spec: &AnalysisSpec,
) -> Option<crate::simulation::dialog::OpConfig> {
    use crate::simulation::dialog::OpConfig;

    match spec {
        AnalysisSpec::LegacyDcOp => Some(OpConfig::default()),
        AnalysisSpec::DcOp {
            temperature_mode,
            temperature_celsius,
            initial_guess,
            node_initialization,
            homotopy,
            annotation,
            device_detail,
            save_device_op,
            accuracy,
            selected_devices,
            previous_state,
            violation_devices,
            violation_source_content_digest,
            run_point,
        } => Some(OpConfig {
            temperature_mode: *temperature_mode,
            temperature_celsius: *temperature_celsius,
            initial_guess: *initial_guess,
            node_initialization: *node_initialization,
            homotopy: *homotopy,
            annotation: *annotation,
            device_detail: *device_detail,
            save_device_op: *save_device_op,
            accuracy: *accuracy,
            selected_devices: selected_devices.clone(),
            previous_state: previous_state.clone(),
            violation_devices: violation_devices.clone(),
            violation_source_content_digest: *violation_source_content_digest,
            run_point: run_point.clone(),
        }),
        _ => None,
    }
}

pub(super) fn operating_point_spec(config: &crate::simulation::dialog::OpConfig) -> AnalysisSpec {
    AnalysisSpec::DcOp {
        temperature_mode: config.temperature_mode,
        temperature_celsius: config.temperature_celsius,
        initial_guess: config.initial_guess,
        node_initialization: config.node_initialization,
        homotopy: config.homotopy,
        annotation: config.annotation,
        device_detail: config.device_detail,
        save_device_op: config.save_device_op,
        accuracy: config.accuracy,
        selected_devices: config.selected_devices.clone(),
        previous_state: config.previous_state.clone(),
        violation_devices: config.violation_devices.clone(),
        violation_source_content_digest: config.violation_source_content_digest,
        run_point: config.run_point.clone(),
    }
}

pub(super) fn validate_retained_operating_point_contract(
    task: &QueuedAnalysis,
    source_digest: ContentDigest,
    executable_source: &str,
) -> Result<(), String> {
    let spec_config = operating_point_config(&task.spec);
    let Some(spec_config) = spec_config else {
        return Ok(());
    };
    spec_config.validate_for_execution()?;
    let effective_source_digest = super::super::canonical::operating_point_effective_source_digest(
        executable_source,
        spec_config.run_point.clone(),
    );
    if let Some(previous) = spec_config.previous_state.as_ref()
        && spec_config.initial_guess
            != crate::simulation::dialog::OpInitialGuess::PreviousCompatible
        && previous.source_content_digest != effective_source_digest
    {
        return Err(
            "the retained previous solution belongs to different executable source content"
                .to_owned(),
        );
    }
    if let Some(soa_source_digest) = spec_config.violation_source_content_digest
        && soa_source_digest != source_digest
    {
        return Err(
            "the retained SOA violation evidence belongs to different executable source content"
                .to_owned(),
        );
    }
    if let Some(crate::simulation::AnalysisConfig::DcOp(config)) = task.config.as_ref()
        && config != &spec_config
    {
        return Err(
            "the operating-point spec and engine configuration carry different contracts"
                .to_owned(),
        );
    }
    Ok(())
}
