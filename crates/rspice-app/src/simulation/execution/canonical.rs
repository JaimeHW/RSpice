//! Versioned canonical encoding for execution identities.
//!
//! This encoding is deliberately independent of `Debug`, serde formats, map
//! iteration order, and host word size. Every aggregate starts with a domain
//! separator; variable-length values carry a big-endian `u64` length.

mod analysis_spec;
#[cfg(test)]
mod monte_carlo_evaluator_tests;
#[cfg(test)]
mod qpac_controls_tests;
#[cfg(test)]
mod qpnoise_controls_tests;
#[cfg(test)]
mod qpss_controls_tests;
#[cfg(test)]
mod qpxf_controls_tests;

pub(in crate::simulation) use analysis_spec::analysis_kind_tag;
use analysis_spec::{
    corner_process_tag, encode_analysis_spec, encode_f64_slice, encode_noise_contribution_detail,
    encode_noise_integration_mode, encode_periodic_carrier_tail, pac_sweep_tag, pnoise_sweep_tag,
    pxf_sweep_tag,
};

use crate::product::{
    AnalysisInstanceId, ContentDigest, manual_deck_analysis_instance_id_from_tag,
};
use crate::services::drc::{DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType};
use crate::services::simulation_runner::{CornerBaseMode, CornerFrequencySweep, PnoiseReference};
use crate::simulation::AnalysisConfig;
use crate::simulation::config::{AcSweepType, PzAnalysisType};
use crate::simulation::dialog::{
    IntegrationMethod, OpAccuracy, OpAnnotation, OpConfig, OpDeviceDetail, OpHomotopy,
    OpInitialGuess, OpNodeInitialization, OpPreviousState, OpRunPointContext, OpSaveDevice,
    OpTemperatureMode,
};
use crate::simulation::multi_run::AnalysisSpec;
use crate::simulation::plan::{AnalysisNumericOverride, NumericOverrideOption, OverrideValue};
use crate::simulation::runner::SpecExecutionOptions;
use crate::state::CanonicalAnalysisKind;

use rspice_app_types::canonical::CanonicalWriter;

#[cfg(any(target_arch = "wasm32", test))]
pub(in crate::simulation) fn f64_sequence_digest(domain: &str, values: &[f64]) -> ContentDigest {
    let mut writer = CanonicalWriter::new(domain);
    writer.sequence(values.len());
    for value in values {
        writer.f64(*value);
    }
    writer.finish()
}

/// Authenticates every numerical and identity field required to reuse an HB
/// operating point. Authenticated producer identities use the v2 domain and
/// are part of the digest. Identityless legacy artifacts retain their exact v1
/// encoding for backward-compatible parsing, but the core engine refuses them
/// for dependent numerical reuse. Integral-bearing states use the v3 domain
/// and authenticate their own typed spectra independently of branch currents.
pub(in crate::simulation) fn hb_operating_point_digest(
    point: &rspice_core::engine::HbOperatingPoint,
) -> ContentDigest {
    let config = point.config();
    let has_integrals = !point.integral_spectra().is_empty();
    let mut writer = CanonicalWriter::new(if has_integrals {
        "rspice.hb-state-artifact/v3"
    } else if point.producer_identity().is_some() {
        "rspice.hb-state-artifact/v2"
    } else {
        "rspice.hb-state-artifact/v1"
    });
    if has_integrals {
        writer.bool(point.producer_identity().is_some());
    }
    if let Some(identity) = point.producer_identity() {
        writer.usize(identity.version() as usize);
        writer.string(identity.semantic_netlist_identity());
        writer.string(identity.resolved_simulation_identity());
        writer.string(identity.hb_source_transform_identity());
        writer.string(identity.retained_state_identity());
    }
    writer.f64(config.fundamental_freq);
    writer.usize(config.num_harmonics);
    writer.sequence(config.tones.len());
    for tone in &config.tones {
        writer.f64(tone.frequency);
        writer.usize(tone.num_harmonics);
        writer.string(&tone.name);
        writer.option(tone.source_name.as_deref(), |writer, source| {
            writer.string(source)
        });
    }
    writer.f64(config.tolerance);
    writer.f64(config.abstol);
    writer.usize(config.max_iterations);
    writer.f64(config.damping);
    writer.f64(config.min_damping);
    writer.usize(config.oversample_factor);
    writer.option(config.collocation_points.as_ref(), |writer, points| {
        writer.usize(*points)
    });
    writer.usize(config.max_mixing_order);
    writer.bool(config.use_krylov);
    writer.usize(config.gmres_restart);
    writer.bool(config.source_stepping);
    writer.bool(config.use_exact_jacobian);
    writer.bool(config.verbose);
    writer.usize(point.iterations());
    writer.f64(point.residual_norm());
    writer.sequence(point.node_names().len());
    for (node, spectrum) in point.node_names().iter().zip(point.spectral_state()) {
        writer.string(node);
        writer.sequence(spectrum.len());
        for value in spectrum {
            writer.f64(value.re);
            writer.f64(value.im);
        }
    }
    if has_integrals || !point.mna_branch_names().is_empty() {
        writer.sequence(point.mna_branch_names().len());
        for (branch, spectrum) in point
            .mna_branch_names()
            .iter()
            .zip(point.mna_branch_spectral_state())
        {
            writer.string(branch);
            writer.sequence(spectrum.len());
            for value in spectrum {
                writer.f64(value.re);
                writer.f64(value.im);
            }
        }
    }
    if has_integrals {
        writer.string("behavioral-sdt/v1");
        writer.sequence(point.integral_spectra().len());
        for spectrum in point.integral_spectra() {
            writer.string(&spectrum.name);
            writer.sequence(spectrum.coefficients.len());
            for value in &spectrum.coefficients {
                writer.f64(value.re);
                writer.f64(value.im);
            }
        }
    }
    writer.finish()
}

/// Identity of the exact executable source and voltage mutation consumed by
/// one OP. Process-corner materialization is already represented in
/// `source`; supply scaling is authenticated separately because it is applied
/// to the parsed netlist immediately before circuit construction.
pub(in crate::simulation) fn operating_point_effective_source_digest(
    source: &str,
    run_point: OpRunPointContext,
) -> ContentDigest {
    let mut writer = CanonicalWriter::new("rspice.op-effective-source/v1");
    writer.digest(crate::state::content_digest(source));
    writer.option(run_point.supply_voltage.as_ref(), |writer, value| {
        writer.f64(*value);
    });
    writer.option(
        run_point.nominal_supply_voltage.as_ref(),
        |writer, value| writer.f64(*value),
    );
    writer.finish()
}

/// Stable manual-deck task identity derived from the exact expanded source.
///
/// Manual decks do not own durable [`SimulationPlan`](crate::simulation::plan::SimulationPlan)
/// instances. Their task identities therefore live in a dedicated UUID-v5
/// namespace and are reproducible only for an identical expanded source,
/// analysis kind, and same-kind occurrence. Simulation-plan runs must use the
/// instance IDs carried by their frozen plan and must never call this helper.
pub(in crate::simulation) fn manual_deck_analysis_instance_id(
    expanded_source_identity: ContentDigest,
    spec: &AnalysisSpec,
    kind_occurrence: usize,
) -> AnalysisInstanceId {
    manual_deck_analysis_instance_id_from_tag(
        expanded_source_identity,
        analysis_kind_tag(spec),
        kind_occurrence,
    )
}

/// Identity of one task's exact numerical contract.
///
/// The domain moved to `/v2` when per-analysis solver overrides joined the
/// payload: two tasks that differ only by their override run different solves
/// and must not share an identity. No persisted digest is ever re-derived from
/// its inputs — receipts store the bytes and later comparisons hash those — so
/// the change does not invalidate a retained run.
///
/// `/v3` is the same argument again, for the same reason: the override record
/// grew from nine options to the full advanced set, and the encoding is now
/// key-tagged and driven by the catalog. A record stating only the original
/// nine encodes differently under `/v3` than it did under `/v2`, so the domain
/// moves rather than letting two encodings collide inside one name.
/// `/v4` includes the SP noise request in the typed analysis encoding.
pub(in crate::simulation) fn analysis_config_digest(
    analysis_line: &str,
    spec: &AnalysisSpec,
    config: Option<&AnalysisConfig>,
    options: &SpecExecutionOptions,
    numeric_override: Option<&AnalysisNumericOverride>,
) -> ContentDigest {
    let mut writer = CanonicalWriter::new("rspice.analysis-config/v4");
    writer.domain("analysis-line");
    writer.string(analysis_line);
    encode_analysis_spec(&mut writer, spec);
    encode_analysis_config(&mut writer, config);
    encode_spec_options(&mut writer, options);
    encode_numeric_override(&mut writer, numeric_override);
    writer.finish()
}

/// Identity of a study evaluator, independent of reporting and plan revisions.
/// The source, sampler, physical point and engine law are bound separately by
/// the core population identity. Keep the complete configured prerequisite and
/// measurement contract, including saved OP state and its lineage.
pub(in crate::simulation) fn monte_carlo_evaluator_digest(
    base: &crate::simulation::runner::study::StudyRunConfig,
) -> ContentDigest {
    use crate::simulation::runner::study::StudyAnalysis;
    let mut base = base.clone();
    base.histogram_bins = 1;
    // Editing checkpoint selection or a reporting control advances the whole
    // plan revision, including unchanged prerequisites. Compatibility follows
    // their full frozen configurations instead. Keep instance identities and
    // every saved OP source/snapshot/result digest and numerical value intact.
    base.source_revision = crate::product::ObjectRevision::INITIAL;
    let operating_point = match &mut base.analysis {
        StudyAnalysis::Pss(config) => Some(&mut config.operating_point),
        StudyAnalysis::Qpss(config) => Some(&mut config.operating_point),
        StudyAnalysis::Hb(config) => Some(&mut config.operating_point),
        StudyAnalysis::Basic(_) | StudyAnalysis::Native(_) => None,
    };
    if let Some(operating_point) = operating_point {
        operating_point.source_revision = crate::product::ObjectRevision::INITIAL;
    }
    if let Some(postprocess) = &mut base.postprocess {
        postprocess.producer_source_revision = crate::product::ObjectRevision::INITIAL;
    }
    // Unit-aware observations change how limits interpret a population. Old
    // journals remain readable, but cannot supply trials to this evaluator.
    let mut writer = CanonicalWriter::new("rspice.monte-carlo-evaluator/v3");
    encode_spec_options(
        &mut writer,
        &SpecExecutionOptions {
            study_base: Some(base),
            ..Default::default()
        },
    );
    writer.finish()
}

/// Encode every option the catalog knows, in catalog order.
///
/// Driven by [`NumericOverrideOption::all`] rather than a list written out
/// here: an option added to the catalog and forgotten here would let two tasks
/// that run different solves share one identity, which is the one thing a
/// config digest exists to prevent. The key is written alongside the value so
/// that reordering the catalog cannot silently re-point an encoded value at a
/// different option.
fn encode_numeric_override(
    writer: &mut CanonicalWriter,
    numeric_override: Option<&AnalysisNumericOverride>,
) {
    writer.domain("analysis-numeric-override");
    writer.option(numeric_override, |writer, record| {
        for option in NumericOverrideOption::all() {
            let Some(value) = record.stated(option) else {
                continue;
            };
            writer.string(option.key());
            match value {
                OverrideValue::Real(value) => writer.f64(value),
                OverrideValue::Count(value) => writer.usize(value),
                OverrideValue::Flag(value) => writer.u8(u8::from(value)),
                OverrideValue::Method(method) => writer.u8(integration_method_tag(method)),
                OverrideValue::Damping(strategy) => writer.string(strategy.spice_name()),
                OverrideValue::Solver(solver) => {
                    writer.string(solver.spice_name().unwrap_or("AUTO"));
                }
                // The deck spelling, which is the digit the engine reads. A
                // separate tag table would be a second identity for one fact.
                OverrideValue::TimeDomainMode(mode) => writer.string(mode.spice_name()),
                // Length first, so that two schedules cannot encode alike by
                // one of them ending where the other's next stop begins.
                OverrideValue::TimeList(times) => {
                    writer.usize(times.len());
                    for time in times {
                        writer.f64(time);
                    }
                }
            }
        }
    });
}

/// A tag is a permanent identity, not a position: renumbering one renames
/// every digest already computed under it. A retired method's tag therefore
/// stays vacant rather than being reused by whatever is added next — the gaps
/// here are the retired Gear spellings, which all named the one second-order
/// Gear integrator the survivor names.
const fn integration_method_tag(method: IntegrationMethod) -> u8 {
    match method {
        IntegrationMethod::Trap => 0,
        IntegrationMethod::Euler => 1,
        IntegrationMethod::Gear2 => 3,
        IntegrationMethod::TrapGear => 4,
    }
}

fn encode_analysis_config(writer: &mut CanonicalWriter, config: Option<&AnalysisConfig>) {
    writer.domain("engine-analysis-config");
    writer.option(config, |writer, config| match config {
        AnalysisConfig::DcOp(config) => {
            writer.u8(0);
            encode_op_config(writer, config);
        }
        AnalysisConfig::DcSweep(config) => {
            writer.u8(1);
            writer.string(&config.source);
            writer.f64(config.start);
            writer.f64(config.stop);
            writer.f64(config.step);
            writer.option(config.source2.as_ref(), |w, v| w.string(v));
            writer.option(config.start2.as_ref(), |w, v| w.f64(*v));
            writer.option(config.stop2.as_ref(), |w, v| w.f64(*v));
            writer.option(config.step2.as_ref(), |w, v| w.f64(*v));
        }
        AnalysisConfig::Transient(config) => {
            writer.u8(2);
            writer.f64(config.stop_time);
            writer.f64(config.step_time);
            writer.f64(config.start_time);
            writer.option(config.max_timestep.as_ref(), |w, v| w.f64(*v));
            writer.bool(config.uic);
        }
        AnalysisConfig::Ac(config) => {
            writer.u8(3);
            writer.u8(match config.sweep_type {
                AcSweepType::Decade => 0,
                AcSweepType::Octave => 1,
                AcSweepType::Linear => 2,
            });
            writer.usize(config.num_points);
            writer.f64(config.start_freq);
            writer.f64(config.stop_freq);
        }
        AnalysisConfig::Noise(config) => {
            writer.u8(4);
            writer.string(&config.output_node);
            writer.string(&config.reference_node);
            writer.string(&config.input_source);
            writer.u8(match config.sweep_type {
                AcSweepType::Decade => 0,
                AcSweepType::Octave => 1,
                AcSweepType::Linear => 2,
            });
            writer.usize(config.num_points);
            writer.f64(config.start_freq);
            writer.f64(config.stop_freq);
            writer.option(config.explicit_frequencies.as_ref(), |w, frequencies| {
                encode_f64_slice(w, frequencies);
            });
            writer.option(config.data_table_name.as_ref(), |w, name| w.string(name));
            encode_noise_contribution_detail(writer, config.contribution_detail);
            encode_noise_integration_mode(writer, config.integration_mode);
            writer.f64(config.temperature_kelvin);
        }
        AnalysisConfig::PoleZero(config) => {
            writer.u8(5);
            writer.string(&config.input_node);
            writer.string(&config.input_ref);
            writer.string(&config.output_node);
            writer.string(&config.output_ref);
            writer.string(&config.transfer_type);
            writer.u8(match config.analysis_type {
                PzAnalysisType::PoleZero => 0,
                PzAnalysisType::PolesOnly => 1,
                PzAnalysisType::ZerosOnly => 2,
            });
        }
        AnalysisConfig::Sensitivity(config) => {
            writer.u8(6);
            writer.string(&config.output_var);
            writer.bool(config.ac_mode);
            writer.option(config.frequency.as_ref(), |w, v| w.f64(*v));
        }
    });
}

fn encode_op_config(writer: &mut CanonicalWriter, config: &OpConfig) {
    encode_op_fields(
        writer,
        config.temperature_mode,
        config.temperature_celsius,
        config.initial_guess,
        config.node_initialization,
        config.homotopy,
        config.annotation,
        config.device_detail,
        config.save_device_op,
        config.accuracy,
        &config.selected_devices,
        config.previous_state.as_ref(),
        &config.violation_devices,
        config.violation_source_content_digest.as_ref(),
        config.run_point.clone(),
    );
}

fn encode_op_fields(
    writer: &mut CanonicalWriter,
    temperature_mode: OpTemperatureMode,
    temperature_celsius: f64,
    initial_guess: OpInitialGuess,
    node_initialization: OpNodeInitialization,
    homotopy: OpHomotopy,
    annotation: OpAnnotation,
    device_detail: OpDeviceDetail,
    save_device_op: OpSaveDevice,
    accuracy: OpAccuracy,
    selected_devices: &[String],
    previous_state: Option<&OpPreviousState>,
    violation_devices: &[String],
    violation_source_content_digest: Option<&ContentDigest>,
    run_point: OpRunPointContext,
) {
    writer.u8(match temperature_mode {
        OpTemperatureMode::PvtRunSet => 0,
        OpTemperatureMode::Nominal27C => 1,
        OpTemperatureMode::Explicit => 2,
        OpTemperatureMode::ActiveRunSetAxis => 3,
    });
    writer.f64(temperature_celsius);
    writer.u8(match initial_guess {
        OpInitialGuess::Automatic => 0,
        OpInitialGuess::PreviousConverged => 1,
        OpInitialGuess::UserNodeVoltages => 2,
        OpInitialGuess::ZeroState => 3,
        OpInitialGuess::PreviousCompatible => 4,
    });
    writer.u8(match node_initialization {
        OpNodeInitialization::UseIcAndNodeset => 0,
        OpNodeInitialization::IgnoreIcAndNodeset => 1,
        OpNodeInitialization::ForceIcValues => 2,
        OpNodeInitialization::ValidateOnly => 3,
    });
    writer.u8(match homotopy {
        OpHomotopy::Adaptive => 0,
        OpHomotopy::SourceStepping => 1,
        OpHomotopy::GminStepping => 2,
        OpHomotopy::PseudoTransient => 3,
        OpHomotopy::None => 4,
    });
    writer.u8(match annotation {
        OpAnnotation::VoltagesAndCurrents => 0,
        OpAnnotation::VoltagesOnly => 1,
        OpAnnotation::VoltagesAndDeviceOp => 2,
        OpAnnotation::None => 3,
    });
    writer.u8(match device_detail {
        OpDeviceDetail::SelectedAndViolations => 0,
        OpDeviceDetail::AllDevices => 1,
        OpDeviceDetail::ViolationsOnly => 2,
        OpDeviceDetail::None => 3,
    });
    writer.u8(match save_device_op {
        OpSaveDevice::Enabled => 0,
        OpSaveDevice::Disabled => 1,
        OpSaveDevice::FinalPointOnly => 2,
    });
    writer.u8(match accuracy {
        OpAccuracy::Fast => 0,
        OpAccuracy::Balanced => 1,
        OpAccuracy::Accurate => 2,
        OpAccuracy::Robust => 3,
    });
    writer.sequence(selected_devices.len());
    for device in selected_devices {
        writer.string(device);
    }
    writer.option(previous_state, |writer, previous| {
        writer.digest(previous.source_content_digest);
        writer.digest(previous.producer_snapshot_digest);
        writer.digest(previous.producer_result_digest);
        writer.sequence(previous.node_names.len());
        for name in &previous.node_names {
            writer.string(name);
        }
        writer.sequence(previous.branch_names.len());
        for name in &previous.branch_names {
            writer.string(name);
        }
        writer.sequence(previous.solution.len());
        for value in &previous.solution {
            writer.f64(*value);
        }
    });
    writer.sequence(violation_devices.len());
    for device in violation_devices {
        writer.string(device);
    }
    writer.option(violation_source_content_digest, |writer, digest| {
        writer.digest(*digest);
    });
    writer.usize(run_point.index);
    writer.usize(run_point.count);
    writer.u8(match run_point.process {
        crate::product::ProcessCorner::TT => 0,
        crate::product::ProcessCorner::SS => 1,
        crate::product::ProcessCorner::FF => 2,
        crate::product::ProcessCorner::SF => 3,
        crate::product::ProcessCorner::FS => 4,
    });
    writer.option(run_point.supply_voltage.as_ref(), |writer, voltage| {
        writer.f64(*voltage);
    });
    writer.option(
        run_point.nominal_supply_voltage.as_ref(),
        |writer, voltage| writer.f64(*voltage),
    );
    writer.sequence(run_point.supply_source_names.len());
    for source in &run_point.supply_source_names {
        writer.string(source);
    }
}

pub(in crate::simulation) fn manual_source_receipt_digest(
    source: &str,
    executable_netlist: &str,
    origin: Option<&str>,
    dependency_closure_digest: ContentDigest,
    task_digests: &[ContentDigest],
) -> ContentDigest {
    let mut writer = CanonicalWriter::new("rspice.manual-source-check-receipt/v2");
    writer.domain("source");
    writer.string(source);
    writer.domain("origin");
    writer.option(origin, |writer, origin| writer.string(origin));
    writer.domain("expanded-executable-netlist");
    writer.string(executable_netlist);
    writer.domain("sealed-dependency-closure");
    writer.digest(dependency_closure_digest);
    writer.domain("accepted-task-configs");
    writer.sequence(task_digests.len());
    for digest in task_digests {
        writer.digest(*digest);
    }
    writer.finish()
}

/// Canonical identity of the exact complete external source files and edges
/// observed by the core include processor. Full file bytes are included so a
/// change confined to an unselected library section still invalidates the
/// retained export bundle evidence rather than silently changing later output.
pub(in crate::simulation) fn sealed_dependency_closure_digest(
    dependencies: &[rspice_core::netlist::ResolvedIncludeDependency],
) -> ContentDigest {
    let mut writer = CanonicalWriter::new("rspice.sealed-manual-dependency-closure/v1");
    writer.sequence(dependencies.len());
    for dependency in dependencies {
        writer.string(&dependency.owner_path().to_string_lossy().replace('\\', "/"));
        writer.usize(dependency.directive_line());
        writer.string(dependency.requested_path());
        writer.string(
            &dependency
                .resolved_path()
                .to_string_lossy()
                .replace('\\', "/"),
        );
        writer.option(dependency.selected_section(), |writer, section| {
            writer.string(section)
        });
        writer.string(dependency.source());
    }
    writer.finish()
}

pub(in crate::simulation) fn drc_receipt_digest(
    topology_revision: u64,
    result: &DrcResult,
) -> ContentDigest {
    let mut violation_digests = result
        .violations()
        .iter()
        .map(drc_violation_digest)
        .collect::<Vec<_>>();
    violation_digests.sort_unstable();

    let mut writer = CanonicalWriter::new("rspice.drc-receipt/v1");
    writer.domain("topology-revision");
    writer.u64(topology_revision);
    writer.domain("completed");
    writer.bool(result.completed);
    // `duration_ms` is intentionally excluded: it is wall-clock telemetry,
    // not verification evidence.
    writer.domain("violations-order-independent");
    writer.sequence(violation_digests.len());
    for digest in violation_digests {
        writer.digest(digest);
    }
    writer.finish()
}

fn drc_violation_digest(violation: &DrcViolation) -> ContentDigest {
    let mut writer = CanonicalWriter::new("rspice.drc-violation/v1");
    writer.usize(violation.id);
    writer.u8(drc_violation_type_tag(violation.violation_type));
    writer.u8(drc_severity_tag(violation.severity));
    writer.string(&violation.message);
    encode_drc_location(&mut writer, &violation.location);

    let mut related = violation.related_items.clone();
    related.sort();
    writer.sequence(related.len());
    for item in related {
        writer.string(&item);
    }
    writer.finish()
}

fn encode_drc_location(writer: &mut CanonicalWriter, location: &DrcLocation) {
    writer.domain("drc-location");
    match location {
        DrcLocation::Point { x, y } => {
            writer.u8(0);
            writer.f64(*x);
            writer.f64(*y);
        }
        DrcLocation::Component { id, name } => {
            writer.u8(1);
            writer.u64(*id);
            writer.string(name);
        }
        DrcLocation::Wire { id } => {
            writer.u8(2);
            writer.u64(*id);
        }
        DrcLocation::NetLabel { name } => {
            writer.u8(3);
            writer.string(name);
        }
        DrcLocation::Node { net_name } => {
            writer.u8(4);
            writer.string(net_name);
        }
        DrcLocation::Global => writer.u8(5),
        DrcLocation::SymbolPin {
            reference,
            pin_name,
            point,
        } => {
            writer.u8(6);
            writer.string(&reference.library);
            writer.string(&reference.cell);
            writer.string(&reference.view);
            writer.string(pin_name);
            writer.option(point.as_ref(), |writer, point| {
                writer.i32(point.x);
                writer.i32(point.y);
            });
        }
        DrcLocation::Bus { id } => {
            writer.u8(7);
            writer.u64(*id);
        }
        DrcLocation::BusTap { id } => {
            writer.u8(8);
            writer.u64(*id);
        }
    }
}

fn encode_spec_options(writer: &mut CanonicalWriter, options: &SpecExecutionOptions) {
    if let Some(bins) = options.mc_histogram_bins {
        writer.domain("monte-carlo-histogram-bins/v1");
        writer.usize(bins);
    }
    if let Some(checkpoint) = &options.mc_checkpoint {
        writer.domain("monte-carlo-checkpoint-request/v1");
        writer.usize(checkpoint.publish_every.get());
        writer.option(checkpoint.trial_range.as_ref(), |writer, range| {
            writer.usize(range.start);
            writer.usize(range.end);
        });
        writer.option(checkpoint.resume.as_ref(), |writer, input| {
            writer.digest(input.digest())
        });
    }
    writer.domain("spec-execution-options");
    writer.option(options.temp.as_ref(), |writer, config| {
        encode_f64_slice(writer, &config.temperatures_c);
        encode_corner_base_mode(writer, &config.base_mode);
    });
    writer.option(options.parametric_base.as_ref(), |writer, mode| {
        encode_corner_base_mode(writer, mode);
    });
    writer.option(options.corner.as_ref(), |writer, config| {
        writer.sequence(config.process_corners.len());
        for process in &config.process_corners {
            writer.u8(corner_process_tag(*process));
        }
        encode_f64_slice(writer, &config.voltages);
        encode_f64_slice(writer, &config.temperatures_c);
        writer.bool(config.full_matrix);
        writer.option(config.nominal_voltage.as_ref(), |w, v| w.f64(*v));
        writer.sequence(config.supply_source_names.len());
        for source in &config.supply_source_names {
            writer.string(source);
        }
        encode_corner_base_mode(writer, &config.base_mode);
        writer.sequence(config.model_bindings.len());
        for binding in &config.model_bindings {
            writer.u8(corner_process_tag(binding.process));
            writer.string(&binding.source_label);
            writer.option(binding.section.as_ref(), |w, v| w.string(v));
            writer.string(&binding.materialized_model_cards);
        }
        // An explicit point list is the run's identity, not a presentation of
        // the axes: a filtered space and the full expansion it was derived from
        // narrow to the same axis values, so without these the two would share
        // a config digest and one run's results could be attributed to the
        // other.
        writer.sequence(config.points.len());
        for point in &config.points {
            writer.u8(corner_process_tag(point.process));
            writer.f64(point.voltage);
            writer.f64(point.temperature_c);
        }
    });
    writer.option(options.pac.as_ref(), |writer, config| {
        writer.f64(config.pss_fundamental_freq);
        writer.usize(config.pss_num_harmonics);
        writer.f64(config.pss_tolerance);
        writer.f64(config.start_freq);
        writer.f64(config.stop_freq);
        writer.usize(config.points_per_unit);
        writer.u8(pac_sweep_tag(config.sweep));
        // The top of the range keeps the slot the single symmetric bound held,
        // because for every symmetric run the two are the same number. The
        // bottom is a conditional tail below, so a symmetric run — which is
        // every run recorded before the range could be stated asymmetrically —
        // digests to the same bytes it always did.
        writer.i32(config.sideband_max);
        writer.string(&config.input_source);
        writer.string(&config.output_node);
        writer.option(config.output_ref.as_ref(), |w, v| w.string(v));
        writer.f64(config.pac_magnitude);
        writer.bool(config.include_dc);
        writer.f64(config.reltol);
        writer.f64(config.abstol);
        encode_periodic_carrier_tail(writer, config.carrier);
        if config.sideband_min != -config.sideband_max {
            writer.i32(config.sideband_min);
        }
    });
    writer.option(options.pxf.as_ref(), |writer, config| {
        writer.f64(config.pss_fundamental_freq);
        writer.usize(config.pss_num_harmonics);
        writer.f64(config.pss_tolerance);
        writer.f64(config.start_freq);
        writer.f64(config.stop_freq);
        writer.usize(config.points_per_unit);
        writer.u8(pxf_sweep_tag(config.sweep));
        writer.string(&config.input_source);
        writer.i32(config.input_sideband);
        writer.string(&config.output_node);
        writer.option(config.output_ref.as_ref(), |w, v| w.string(v));
        writer.i32(config.output_sideband);
        writer.i32(config.max_sideband);
        writer.f64(config.reltol);
        writer.f64(config.abstol);
        encode_periodic_carrier_tail(writer, config.carrier);
    });
    // Preserve the retired TF execution-option slot so unrelated analysis
    // identities remain byte-for-byte stable. TF authority now lives in the
    // structured analysis spec encoded above.
    writer.bool(false);
    writer.option(options.pnoise.as_ref(), |writer, config| {
        writer.f64(config.pss_fundamental_freq);
        writer.usize(config.pss_num_harmonics);
        writer.f64(config.pss_tolerance);
        writer.f64(config.start_freq);
        writer.f64(config.stop_freq);
        writer.usize(config.points_per_unit);
        writer.u8(pnoise_sweep_tag(config.sweep));
        writer.i32(config.max_sideband);
        writer.string(&config.output_node);
        writer.option(config.output_ref.as_ref(), |w, v| w.string(v));
        writer.string(&config.input_source);
        writer.u8(match config.noise_ref {
            PnoiseReference::Output => 0,
            PnoiseReference::Input => 1,
            PnoiseReference::Phase => 2,
        });
        writer.bool(config.integrated_noise);
        writer.bool(config.noise_summary);
        writer.f64(config.reltol);
        writer.f64(config.abstol);
        encode_periodic_carrier_tail(writer, config.carrier);
        if let Some(sampling) = &config.sampling {
            writer.string("pnoise-sampling-v1");
            encode_pnoise_sampling(writer, sampling);
        }
        if config.input_sideband != 0 || config.output_sideband != 0 {
            writer.string("pnoise-conversion-sidebands-v1");
            writer.i32(config.input_sideband);
            writer.i32(config.output_sideband);
        }
    });
    writer.option(options.pstb.as_ref(), |writer, config| {
        writer.f64(config.pss_fundamental_freq);
        writer.usize(config.pss_num_harmonics);
        writer.f64(config.pss_tolerance);
        writer.string(&config.probe_instance);
        writer.usize(config.max_harmonics);
        writer.usize(config.num_multipliers);
        writer.f64(config.stability_threshold);
        writer.bool(config.detect_subharmonics);
        writer.f64(config.eigenvalue_tolerance);
    });
    if let Some(statistics) = &options.mc_statistics {
        writer.domain("monte-carlo-custom-statistics/v1");
        writer.sequence(statistics.variations.len());
        for row in &statistics.variations {
            use crate::simulation::dialog::mc::statistics::{McScope, McShape};
            writer.string(&row.parameter);
            writer.u8(match row.scope {
                McScope::Process => 0,
                McScope::Mismatch => 1,
            });
            writer.u8(match row.distribution {
                McShape::Gaussian => 0,
                McShape::Uniform => 1,
                McShape::Lognormal => 2,
            });
            writer.f64(row.spread);
            writer.bool(row.percent);
        }
        if statistics.variations.iter().any(|row| row.bounds.is_some()) {
            writer.domain("monte-carlo-parameter-bounds/v1");
            writer.sequence(statistics.variations.len());
            for row in &statistics.variations {
                writer.option(row.bounds.as_ref(), |writer, bounds| {
                    writer.option(bounds.lower.as_ref(), |writer, value| writer.f64(*value));
                    writer.option(bounds.upper.as_ref(), |writer, value| writer.f64(*value));
                    writer.option(bounds.sigma_cutoff.as_ref(), |writer, value| {
                        writer.f64(*value)
                    });
                    writer.u64(u64::from(bounds.max_attempts));
                });
            }
        }
        writer.sequence(statistics.correlations.len());
        for row in &statistics.correlations {
            writer.u8(match row.scope {
                crate::simulation::dialog::mc::statistics::McScope::Process => 0,
                _ => 1,
            });
            writer.sequence(row.parameters.len());
            for name in &row.parameters {
                writer.string(name);
            }
            writer.f64(row.coefficient);
        }
    }
    if let Some(base) = &options.study_base {
        writer.domain("configured-study-base/v1");
        writer.string(&base.instance_id.to_string());
        writer.u64(base.source_revision.get());
        match &base.analysis {
            crate::simulation::runner::study::StudyAnalysis::Basic(config) => {
                encode_analysis_config(writer, Some(config));
            }
            crate::simulation::runner::study::StudyAnalysis::Pss(pss) => {
                writer.domain("study-seeded-pss/v1");
                encode_analysis_spec(writer, &pss.request);
                writer.uuid(pss.operating_point.instance_id.as_uuid());
                writer.u64(pss.operating_point.source_revision.get());
                encode_op_config(writer, &pss.operating_point.config);
                writer.string(&pss.operating_point.numeric_options);
            }
            crate::simulation::runner::study::StudyAnalysis::Qpss(qpss) => {
                writer.domain("study-seeded-qpss/v1");
                encode_analysis_spec(writer, &qpss.request);
                writer.uuid(qpss.operating_point.instance_id.as_uuid());
                writer.u64(qpss.operating_point.source_revision.get());
                encode_op_config(writer, &qpss.operating_point.config);
                writer.string(&qpss.operating_point.numeric_options);
            }
            crate::simulation::runner::study::StudyAnalysis::Hb(hb) => {
                writer.domain("study-seeded-hb/v1");
                encode_analysis_spec(writer, &hb.request);
                writer.uuid(hb.operating_point.instance_id.as_uuid());
                writer.u64(hb.operating_point.source_revision.get());
                encode_op_config(writer, &hb.operating_point.config);
                writer.string(&hb.operating_point.numeric_options);
            }
            crate::simulation::runner::study::StudyAnalysis::Native(spec) => {
                writer.domain("study-native-spec/v1");
                encode_analysis_spec(writer, spec);
            }
        }
        if let Some(postprocess) = &base.postprocess {
            writer.domain(
                if matches!(
                    postprocess.request,
                    AnalysisSpec::Hbsp { .. } | AnalysisSpec::Hbnoise { .. }
                ) {
                    "study-hb-consumer/v1"
                } else if matches!(
                    postprocess.request,
                    AnalysisSpec::Pac
                        | AnalysisSpec::Pxf
                        | AnalysisSpec::Pnoise
                        | AnalysisSpec::Pstb
                        | AnalysisSpec::Psp { .. }
                        | AnalysisSpec::Qpac { .. }
                        | AnalysisSpec::Qpxf { .. }
                        | AnalysisSpec::Qpnoise { .. }
                ) {
                    "study-periodic-consumer/v1"
                } else {
                    "study-transient-postprocessor/v1"
                },
            );
            writer.uuid(postprocess.producer_instance_id.as_uuid());
            writer.u64(postprocess.producer_source_revision.get());
            writer.string(&postprocess.producer_analysis_line);
            writer.string(&postprocess.producer_numeric_options);
            encode_analysis_spec(writer, &postprocess.request);
            if let Some(options) = &postprocess.periodic_options {
                writer.domain("study-periodic-consumer-options/v1");
                encode_spec_options(writer, &options.execution_options());
            }
        }
        writer.string(&base.analysis_line);
        writer.string(&base.numeric_options);
        writer.sequence(base.measurements.len());
        for measurement in &base.measurements {
            writer.string(measurement);
        }
        writer.usize(base.histogram_bins);
        if !base.constraints.is_empty() {
            writer.domain("optimization-constraints/v1");
            writer.sequence(base.constraints.len());
            for term in &base.constraints {
                writer.string(&term.measurement);
                if !term.unit.is_empty() {
                    writer.string("optimization-measurement-unit/v1");
                    writer.string(&term.unit);
                }
                writer.option(term.lower.as_ref(), |writer, value| writer.f64(*value));
                writer.option(term.upper.as_ref(), |writer, value| writer.f64(*value));
                writer.f64(term.tolerance);
                writer.f64(term.scale);
            }
        }
        if !base.objective_terms.is_empty() {
            writer.domain("weighted-optimization-objectives/v1");
            writer.sequence(base.objective_terms.len());
            for term in &base.objective_terms {
                writer.string(&term.measurement);
                if !term.unit.is_empty() {
                    writer.string("optimization-measurement-unit/v1");
                    writer.string(&term.unit);
                }
                writer.u8(match term.goal {
                    crate::simulation::optimizer::OptimizationObjectiveGoal::Minimize => 0,
                    crate::simulation::optimizer::OptimizationObjectiveGoal::Maximize => 1,
                    crate::simulation::optimizer::OptimizationObjectiveGoal::Target => 2,
                });
                writer.option(term.target.as_ref(), |writer, value| writer.f64(*value));
                writer.f64(term.scale);
                writer.f64(term.weight);
            }
        }
    }
}

fn encode_dc_modes(writer: &mut CanonicalWriter, modes: &crate::simulation::config::DcSweepModes) {
    use crate::simulation::config::{DcAxisMode, DcSweepModes};
    if modes == &DcSweepModes::default() {
        return;
    }
    writer.string("dc-axis-modes-v1");
    for mode in [&modes.primary, &modes.secondary] {
        match mode {
            DcAxisMode::Linear => writer.u8(0),
            DcAxisMode::List { values } => {
                writer.u8(1);
                encode_f64_slice(writer, values);
            }
            DcAxisMode::Decade { points_per_decade } => {
                writer.u8(2);
                writer.usize(*points_per_decade);
            }
            DcAxisMode::Octave { points_per_octave } => {
                writer.u8(3);
                writer.usize(*points_per_octave);
            }
        }
    }
}

fn encode_corner_base_mode(writer: &mut CanonicalWriter, mode: &CornerBaseMode) {
    writer.domain("corner-base-mode");
    match mode {
        CornerBaseMode::Op => writer.u8(0),
        CornerBaseMode::ConfiguredOp(config) => {
            writer.u8(6);
            encode_op_config(writer, config);
        }
        CornerBaseMode::DcSweep {
            modes,
            source_name,
            start,
            stop,
            step,
        } => {
            writer.u8(1);
            writer.string(source_name);
            writer.f64(*start);
            writer.f64(*stop);
            writer.f64(*step);
            encode_dc_modes(writer, modes);
        }
        CornerBaseMode::DcSweepNested {
            modes,
            source_name,
            start,
            stop,
            step,
            source2,
            start2,
            stop2,
            step2,
        } => {
            writer.u8(4);
            writer.string(source_name);
            writer.f64(*start);
            writer.f64(*stop);
            writer.f64(*step);
            writer.string(source2);
            writer.f64(*start2);
            writer.f64(*stop2);
            writer.f64(*step2);
            encode_dc_modes(writer, modes);
        }
        CornerBaseMode::Transient {
            stop_time,
            step_time,
        } => {
            writer.u8(2);
            writer.f64(*stop_time);
            writer.f64(*step_time);
        }
        CornerBaseMode::TransientWindow {
            stop_time,
            step_time,
            start_time,
            max_timestep,
            uic,
        } => {
            writer.u8(5);
            writer.f64(*stop_time);
            writer.f64(*step_time);
            writer.f64(*start_time);
            writer.option(max_timestep.as_ref(), |writer, value| writer.f64(*value));
            writer.bool(*uic);
        }
        CornerBaseMode::Ac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
        } => {
            writer.u8(3);
            writer.f64(*start_freq);
            writer.f64(*stop_freq);
            writer.usize(*points_per_unit);
            writer.u8(match sweep {
                CornerFrequencySweep::Decade => 0,
                CornerFrequencySweep::Octave => 1,
                CornerFrequencySweep::Linear => 2,
            });
        }
    }
}

/// The canonical protocol kind of one execution specification.
///
/// This is the only place a specification is classified. The tag it carries,
/// the receipt layer's accepted set, and the result family the task produces
/// all live together on [`CanonicalAnalysisKind`], so widening the protocol
/// cannot teach dispatch a tag the receipt layer will refuse.
pub(in crate::simulation) const fn canonical_analysis_kind(
    spec: &AnalysisSpec,
) -> CanonicalAnalysisKind {
    match spec {
        AnalysisSpec::LegacyDcOp | AnalysisSpec::DcOp { .. } => CanonicalAnalysisKind::DcOp,
        AnalysisSpec::DcSweep { .. } => CanonicalAnalysisKind::DcSweep,
        AnalysisSpec::Ac { .. } => CanonicalAnalysisKind::Ac,
        AnalysisSpec::AcData { .. } => CanonicalAnalysisKind::AcData,
        AnalysisSpec::Disto { .. } => CanonicalAnalysisKind::Disto,
        AnalysisSpec::Transient { .. } => CanonicalAnalysisKind::Transient,
        AnalysisSpec::Noise { .. } => CanonicalAnalysisKind::Noise,
        AnalysisSpec::Pss { .. } => CanonicalAnalysisKind::Pss,
        AnalysisSpec::HarmonicBalance { .. } => CanonicalAnalysisKind::HarmonicBalance,
        AnalysisSpec::Tf { .. } => CanonicalAnalysisKind::Tf,
        AnalysisSpec::Sensitivity { .. } => CanonicalAnalysisKind::Sensitivity,
        AnalysisSpec::PoleZero { .. } => CanonicalAnalysisKind::PoleZero,
        AnalysisSpec::Pac => CanonicalAnalysisKind::Pac,
        AnalysisSpec::Pnoise => CanonicalAnalysisKind::Pnoise,
        AnalysisSpec::Pxf => CanonicalAnalysisKind::Pxf,
        AnalysisSpec::Pstb => CanonicalAnalysisKind::Pstb,
        AnalysisSpec::Stb { .. } => CanonicalAnalysisKind::Stb,
        AnalysisSpec::MonteCarlo { .. } => CanonicalAnalysisKind::MonteCarlo,
        AnalysisSpec::Parametric => CanonicalAnalysisKind::Parametric,
        AnalysisSpec::Corner => CanonicalAnalysisKind::Corner,
        AnalysisSpec::Optimization { .. } => CanonicalAnalysisKind::Optimization,
        AnalysisSpec::Soa { .. } => CanonicalAnalysisKind::Soa,
        AnalysisSpec::SParameter { .. } => CanonicalAnalysisKind::SParameter,
        AnalysisSpec::Envelope { .. } => CanonicalAnalysisKind::Envelope,
        AnalysisSpec::Fourier { .. } => CanonicalAnalysisKind::Fourier,
        AnalysisSpec::Qpss { .. } => CanonicalAnalysisKind::Qpss,
        AnalysisSpec::Hbsp { .. } => CanonicalAnalysisKind::Hbsp,
        AnalysisSpec::Hbnoise { .. } => CanonicalAnalysisKind::Hbnoise,
        AnalysisSpec::Psp { .. } => CanonicalAnalysisKind::Psp,
        AnalysisSpec::Qpac { .. } => CanonicalAnalysisKind::Qpac,
        AnalysisSpec::Qpnoise { .. } => CanonicalAnalysisKind::Qpnoise,
        AnalysisSpec::Qpxf { .. } => CanonicalAnalysisKind::Qpxf,
        AnalysisSpec::TransientNoise { .. } => CanonicalAnalysisKind::TransientNoise,
        AnalysisSpec::DcMismatch { .. } => CanonicalAnalysisKind::DcMismatch,
        AnalysisSpec::PssSpectrum { .. } => CanonicalAnalysisKind::PssSpectrum,
        AnalysisSpec::Fft { .. } => CanonicalAnalysisKind::Fft,
    }
}

fn drc_severity_tag(severity: DrcSeverity) -> u8 {
    match severity {
        DrcSeverity::Info => 0,
        DrcSeverity::Warning => 1,
        DrcSeverity::Error => 2,
        DrcSeverity::Critical => 3,
    }
}

fn drc_violation_type_tag(violation_type: DrcViolationType) -> u8 {
    match violation_type {
        DrcViolationType::FloatingNode => 0,
        DrcViolationType::UnconnectedPin => 1,
        DrcViolationType::OrphanNetLabel => 2,
        DrcViolationType::DanglingWire => 3,
        DrcViolationType::ShortedOutputs => 4,
        DrcViolationType::MissingGround => 5,
        DrcViolationType::SourceToSource => 7,
        DrcViolationType::DuplicateName => 8,
        DrcViolationType::EmptyName => 9,
        DrcViolationType::InvalidName => 10,
        DrcViolationType::MissingParameter => 11,
        DrcViolationType::ValueOutOfRange => 12,
        DrcViolationType::UnknownComponent => 13,
        DrcViolationType::SymbolUnplacedPin => 14,
        DrcViolationType::SymbolOrphanedPin => 15,
        DrcViolationType::SymbolPinOffGrid => 16,
        DrcViolationType::MalformedBus => 17,
        DrcViolationType::UnnamedBus => 18,
        DrcViolationType::BusRangeConflict => 19,
        DrcViolationType::DanglingBusTap => 20,
        DrcViolationType::MixedBusTap => 21,
        DrcViolationType::DuplicateBusMemberDriver => 22,
        DrcViolationType::OffSheetConnectorWithoutPartner => 23,
        DrcViolationType::CaseCollidingNetNames => 24,
        DrcViolationType::VectorWidthMismatch => 25,
    }
}

#[cfg(test)]
mod tests;

fn encode_pnoise_sampling(
    writer: &mut CanonicalWriter,
    sampling: &rspice_core::analysis::pnoise::PeriodicNoiseSampling,
) {
    use rspice_core::analysis::pnoise::{
        PeriodicNoiseEdge, PeriodicNoiseEdgeDirection, PeriodicNoiseSampling,
    };
    fn edge(writer: &mut CanonicalWriter, edge: &PeriodicNoiseEdge) {
        writer.f64(edge.threshold_volts);
        writer.u8(match edge.direction {
            PeriodicNoiseEdgeDirection::Rising => 0,
            PeriodicNoiseEdgeDirection::Falling => 1,
            PeriodicNoiseEdgeDirection::Either => 2,
        });
        writer.usize(edge.occurrence);
        writer.f64(edge.phase_tolerance_degrees);
        writer.f64(edge.minimum_slew_volts_per_second);
    }
    match sampling {
        PeriodicNoiseSampling::Phase { phase_degrees } => {
            writer.u8(0);
            writer.f64(*phase_degrees);
        }
        PeriodicNoiseSampling::Edge { edge: output } => {
            writer.u8(1);
            edge(writer, output);
        }
        PeriodicNoiseSampling::Delay {
            edge: output,
            reference_node,
            reference_ref,
            reference_edge,
            periods,
        } => {
            writer.u8(2);
            edge(writer, output);
            writer.string(reference_node);
            writer.option(reference_ref.as_ref(), |writer, name| writer.string(name));
            edge(writer, reference_edge);
            writer.u64(u64::from(*periods));
        }
    }
}
