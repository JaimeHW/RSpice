//! Versioned canonical encoding for execution identities.
//!
//! This encoding is deliberately independent of `Debug`, serde formats, map
//! iteration order, and host word size. Every aggregate starts with a domain
//! separator; variable-length values carry a big-endian `u64` length.

mod analysis_spec;
#[cfg(test)]
mod qpac_controls_tests;
#[cfg(test)]
mod qpnoise_controls_tests;
#[cfg(test)]
mod qpss_controls_tests;
#[cfg(test)]
mod qpxf_controls_tests;
mod reliability;

pub(in crate::simulation) use analysis_spec::analysis_kind_tag;
use analysis_spec::{
    corner_process_tag, encode_analysis_spec, encode_f64_slice, encode_noise_contribution_detail,
    encode_noise_integration_mode, encode_periodic_carrier_tail, pac_sweep_tag, pnoise_sweep_tag,
    pxf_sweep_tag,
};

use sha2::{Digest as _, Sha256};

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

const CANONICAL_MAGIC: &[u8] = b"RSPICE-CANONICAL";
const CANONICAL_VERSION: u16 = 1;

pub(super) struct CanonicalWriter {
    hasher: Sha256,
}

impl crate::state::ConvergenceEncoder for CanonicalWriter {
    fn u64(&mut self, value: u64) {
        Self::u64(self, value);
    }
    fn f64(&mut self, value: f64) {
        Self::f64(self, value);
    }
    fn string(&mut self, value: &str) {
        Self::string(self, value);
    }
    fn tag(&mut self, value: u8) {
        Self::u8(self, value);
    }
}

impl CanonicalWriter {
    pub(super) fn new(domain: &str) -> Self {
        let mut writer = Self {
            hasher: Sha256::new(),
        };
        writer.raw(CANONICAL_MAGIC);
        writer.raw(&CANONICAL_VERSION.to_be_bytes());
        writer.domain(domain);
        writer
    }

    pub(super) fn domain(&mut self, value: &str) {
        self.raw(&[0xd0]);
        self.length(value.len());
        self.raw(value.as_bytes());
    }

    pub(super) fn bool(&mut self, value: bool) {
        self.raw(&[0x01, u8::from(value)]);
    }

    pub(super) fn u8(&mut self, value: u8) {
        self.raw(&[0x02, value]);
    }

    pub(super) fn u64(&mut self, value: u64) {
        self.raw(&[0x03]);
        self.raw(&value.to_be_bytes());
    }

    pub(super) fn usize(&mut self, value: usize) {
        self.u64(u64::try_from(value).expect("supported Rust targets use at most 64-bit usize"));
    }

    pub(super) fn i32(&mut self, value: i32) {
        self.raw(&[0x04]);
        self.raw(&value.to_be_bytes());
    }

    pub(super) fn f64(&mut self, value: f64) {
        self.raw(&[0x06]);
        self.raw(&value.to_bits().to_be_bytes());
    }

    pub(super) fn string(&mut self, value: &str) {
        self.raw(&[0x07]);
        self.length(value.len());
        self.raw(value.as_bytes());
    }

    pub(super) fn bytes(&mut self, value: &[u8]) {
        self.raw(&[0x08]);
        self.length(value.len());
        self.raw(value);
    }

    pub(super) fn digest(&mut self, value: ContentDigest) {
        self.raw(&[0x09]);
        self.raw(value.as_bytes());
    }

    pub(super) fn uuid(&mut self, value: uuid::Uuid) {
        self.raw(&[0x0c]);
        self.raw(value.as_bytes());
    }

    pub(super) fn sequence(&mut self, len: usize) {
        self.raw(&[0x0a]);
        self.length(len);
    }

    pub(super) fn option<T: ?Sized>(
        &mut self,
        value: Option<&T>,
        encode: impl FnOnce(&mut Self, &T),
    ) {
        match value {
            Some(value) => {
                self.raw(&[0x0b, 1]);
                encode(self, value);
            }
            None => self.raw(&[0x0b, 0]),
        }
    }

    pub(super) fn finish(self) -> ContentDigest {
        ContentDigest::from_bytes(self.hasher.finalize().into())
    }

    fn length(&mut self, len: usize) {
        self.raw(
            &u64::try_from(len)
                .expect("supported Rust targets use at most 64-bit usize")
                .to_be_bytes(),
        );
    }

    fn raw(&mut self, bytes: &[u8]) {
        self.hasher.update(bytes);
    }
}

pub(in crate::simulation) fn content_digest(domain: &str, bytes: &[u8]) -> ContentDigest {
    let mut writer = CanonicalWriter::new(domain);
    writer.bytes(bytes);
    writer.finish()
}

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
/// for dependent numerical reuse.
pub(in crate::simulation) fn hb_operating_point_digest(
    point: &rspice_core::engine::HbOperatingPoint,
) -> ContentDigest {
    let config = point.config();
    let mut writer = CanonicalWriter::new(if point.producer_identity().is_some() {
        "rspice.hb-state-artifact/v2"
    } else {
        "rspice.hb-state-artifact/v1"
    });
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
    if !point.mna_branch_names().is_empty() {
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
    writer.digest(crate::workbench::documents::netlist_document::source_content_digest(source));
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
        encode_analysis_config(writer, Some(&base.analysis));
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
        AnalysisSpec::Reliability { .. } => CanonicalAnalysisKind::Reliability,
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
mod tests {
    use super::*;
    use crate::services::drc::{DrcLocation, DrcViolation};
    use crate::simulation::config::{
        NoiseContributionDetail, NoiseIntegrationMode, NoiseSweepType,
    };
    use crate::simulation::multi_run::{
        EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve, FrequencySweep,
        TfAccuracy, TfNormalization,
    };

    #[test]
    fn analysis_kind_tags_are_append_only() {
        // These tags are hashed into the config digest, so renumbering one
        // silently redefines every prepared snapshot already on disk. Pin the
        // boundary: PSS keeps 7, and the newest variant took the next free
        // number rather than a gap.
        assert_eq!(
            analysis_kind_tag(&AnalysisSpec::PssSpectrum { num_harmonics: 20 }),
            35
        );
        assert_eq!(analysis_kind_tag(&exact_pss_spec()), 7);
    }

    /// A transient-noise plan with no authored floor digests to exactly the
    /// bytes it digested to before the field existed.
    ///
    /// The identity of every prepared snapshot already on disk depends on it.
    /// The reference is built from the writer primitives in the order the arm
    /// used before the floor was added, rather than pinned as a hex literal:
    /// a literal states what the answer is, and this states *why* it is that —
    /// the eight fields, nothing appended. A `writer.option` around the new
    /// field would tag the `None` and fail here, which is the point.
    #[test]
    fn an_absent_noise_floor_leaves_the_plan_digest_unchanged() {
        let transient_noise = |noise_fmin| AnalysisSpec::TransientNoise {
            stop_time: 1.0e-6,
            step_time: 1.0e-9,
            start_time: 2.0e-7,
            max_timestep: 2.5e-10,
            seed: 97,
            noise_fmax: 5.0e8,
            noise_fmin,
            scale: 0.5,
            uic: true,
        };
        let spec = transient_noise(None);
        let mut encoded = CanonicalWriter::new("test");
        encode_analysis_spec(&mut encoded, &spec);

        let mut before_the_field = CanonicalWriter::new("test");
        before_the_field.domain("analysis-spec");
        before_the_field.u8(analysis_kind_tag(&spec));
        before_the_field.f64(1.0e-6);
        before_the_field.f64(1.0e-9);
        before_the_field.f64(2.0e-7);
        before_the_field.f64(2.5e-10);
        before_the_field.u64(97);
        before_the_field.f64(5.0e8);
        before_the_field.f64(0.5);
        before_the_field.bool(true);

        assert_eq!(
            encoded.finish(),
            before_the_field.finish(),
            "an unauthored noise floor must not move a saved plan's identity"
        );

        // And an authored one must move it, or two plans running different
        // bands would share one identity.
        let mut authored = CanonicalWriter::new("test");
        encode_analysis_spec(&mut authored, &transient_noise(Some(1.0e3)));
        let mut absent = CanonicalWriter::new("test");
        encode_analysis_spec(&mut absent, &spec);
        assert_ne!(authored.finish(), absent.finish());
    }

    /// A sensitivity plan restored from a project saved before filters
    /// existed digests to exactly the bytes it digested to then.
    ///
    /// Same shape of reference as the two above — the three fields the arm
    /// wrote, nothing appended — with one difference that is the whole point:
    /// the value that leaves the digest alone is `PARAM:*`, not the empty
    /// string. A saved plan computed the deck's design parameters, so that is
    /// what it must go on computing, and a plan whose filter was emptied runs
    /// a different analysis and has to say so.
    #[test]
    fn a_design_parameter_filter_at_one_frequency_leaves_the_plan_digest_unchanged() {
        let sensitivity = |filter: &str| AnalysisSpec::Sensitivity {
            output_var: "V(out)".to_owned(),
            ac_mode: true,
            frequency: Some(1.0e6),
            filter: filter.to_owned(),
            sweep: None,
        };
        let spec = sensitivity(crate::simulation::config::DESIGN_PARAMETERS_FILTER);
        let mut encoded = CanonicalWriter::new("test");
        encode_analysis_spec(&mut encoded, &spec);

        let mut before_the_field = CanonicalWriter::new("test");
        before_the_field.domain("analysis-spec");
        before_the_field.u8(analysis_kind_tag(&spec));
        before_the_field.string("V(out)");
        before_the_field.bool(true);
        before_the_field.option(Some(&1.0e6), |writer, value| writer.f64(*value));

        assert_eq!(
            encoded.finish(),
            before_the_field.finish(),
            "a plan restored from before filters existed must keep its identity"
        );
    }

    /// Emptying the filter is a different analysis, and a different plan.
    ///
    /// The engine reads an empty filter as every device and model parameter
    /// and no design parameter — the opposite selection from `PARAM:*`. If
    /// the two digested alike, a stored result computed under one would be
    /// presented as current for the other.
    #[test]
    fn an_emptied_filter_is_a_different_plan_from_one_saved_before_filters() {
        let sensitivity = |filter: &str| AnalysisSpec::Sensitivity {
            output_var: "V(out)".to_owned(),
            ac_mode: false,
            frequency: None,
            filter: filter.to_owned(),
            sweep: None,
        };
        let digest = |filter: &str| {
            let mut writer = CanonicalWriter::new("test");
            encode_analysis_spec(&mut writer, &sensitivity(filter));
            writer.finish()
        };
        let legacy = digest(crate::simulation::config::DESIGN_PARAMETERS_FILTER);
        assert_ne!(legacy, digest(""));
        assert_ne!(legacy, digest("R* PARAM:*"));
        assert_ne!(digest(""), digest("R*"));
        assert_eq!(digest("R*"), digest("R*"));
    }

    /// A DC mismatch plan with no authored share threshold digests to exactly
    /// the bytes it digested to before the field existed.
    ///
    /// Same reasoning as the noise floor above, and the same shape of
    /// reference: the six fields the arm wrote, nothing appended. `.DCMATCH`'s
    /// own default threshold is zero, so a `writer.option` tag — or an
    /// unconditional `writer.f64(0.0)` — would redefine the identity of every
    /// DC mismatch plan already saved.
    #[test]
    fn an_unauthored_share_threshold_leaves_the_plan_digest_unchanged() {
        let dc_mismatch = |contribution_threshold| AnalysisSpec::DcMismatch {
            output_expression: "V(out)".to_owned(),
            sigma_multiplier: 1.0,
            contributor_limit: 10,
            include_process: false,
            include_mismatch: true,
            normalized_contributions: true,
            contribution_threshold,
        };
        let spec = dc_mismatch(None);
        let mut encoded = CanonicalWriter::new("test");
        encode_analysis_spec(&mut encoded, &spec);

        let mut before_the_field = CanonicalWriter::new("test");
        before_the_field.domain("analysis-spec");
        before_the_field.u8(analysis_kind_tag(&spec));
        before_the_field.string("V(out)");
        before_the_field.f64(1.0);
        before_the_field.usize(10);
        before_the_field.bool(false);
        before_the_field.bool(true);
        before_the_field.bool(true);

        assert_eq!(
            encoded.finish(),
            before_the_field.finish(),
            "an unauthored share threshold must not move a saved plan's identity"
        );

        // And an authored one must move it, or two plans trimming their
        // contributor lists differently would share one identity.
        let mut authored = CanonicalWriter::new("test");
        encode_analysis_spec(&mut authored, &dc_mismatch(Some(0.05)));
        let mut absent = CanonicalWriter::new("test");
        encode_analysis_spec(&mut absent, &spec);
        assert_ne!(authored.finish(), absent.finish());
    }

    /// A Monte Carlo plan that named no subset digests to exactly the bytes it
    /// digested to when the arm wrote nothing at all.
    ///
    /// The arm was empty, so the reference is the kind tag and nothing after
    /// it. That makes the conditional tail load-bearing in both directions: an
    /// unnamed subset must not move a saved plan's identity, and two runs that
    /// vary different parameters must not share one.
    #[test]
    fn an_unauthored_vary_only_leaves_the_plan_digest_unchanged() {
        let monte_carlo = |params: Vec<String>| AnalysisSpec::MonteCarlo {
            variation_source: crate::simulation::dialog::McVariationSource::ParameterTolerance,
            params,
        };
        let spec = monte_carlo(Vec::new());
        let mut encoded = CanonicalWriter::new("test");
        encode_analysis_spec(&mut encoded, &spec);

        let mut before_the_field = CanonicalWriter::new("test");
        before_the_field.domain("analysis-spec");
        before_the_field.u8(analysis_kind_tag(&spec));

        assert_eq!(
            encoded.finish(),
            before_the_field.finish(),
            "an unnamed Monte Carlo subset must not move a saved plan's identity"
        );

        let mut named = CanonicalWriter::new("test");
        encode_analysis_spec(&mut named, &monte_carlo(vec!["RLOAD".to_owned()]));
        let mut unnamed = CanonicalWriter::new("test");
        encode_analysis_spec(&mut unnamed, &spec);
        assert_ne!(named.finish(), unnamed.finish());

        // Order is part of the request the card writes, so it is part of the
        // identity too.
        let mut forward = CanonicalWriter::new("test");
        encode_analysis_spec(
            &mut forward,
            &monte_carlo(vec!["RA".to_owned(), "RB".to_owned()]),
        );
        let mut reversed = CanonicalWriter::new("test");
        encode_analysis_spec(
            &mut reversed,
            &monte_carlo(vec!["RB".to_owned(), "RA".to_owned()]),
        );
        assert_ne!(forward.finish(), reversed.finish());
    }

    #[test]
    fn a_pss_spectrum_digest_follows_its_harmonic_count() {
        let twenty = AnalysisSpec::PssSpectrum { num_harmonics: 20 };
        let nine = AnalysisSpec::PssSpectrum { num_harmonics: 9 };
        let mut left = CanonicalWriter::new("test");
        let mut right = CanonicalWriter::new("test");
        encode_analysis_spec(&mut left, &twenty);
        encode_analysis_spec(&mut right, &nine);

        assert_ne!(left.finish(), right.finish());
    }

    fn exact_pss_spec() -> AnalysisSpec {
        AnalysisSpec::Pss {
            method: crate::simulation::multi_run::PssMethod::Shooting,
            fundamental_freq: 1.0e6,
            tone_sources: vec!["VIN".to_owned()],
            tstab_periods: 20,
            points_per_period: 512,
            tolerance: 1.0e-6,
            oscillator_mode: false,
            oscillator_node: None,
            num_harmonics: 20,
            integration_method: None,
            tstab: 0.0,
            max_iterations: 100,
            abstol: 1.0e-12,
            damping: 1.0,
            max_period_change: 0.1,
            verbose: false,
        }
    }

    fn exact_noise_spec() -> AnalysisSpec {
        AnalysisSpec::Noise {
            output_node: "out".to_owned(),
            reference_node: "ref".to_owned(),
            input_source: "VIN".to_owned(),
            start_freq: 10.0,
            stop_freq: 1.0e6,
            points_per_decade: 30,
            sweep: NoiseSweepType::Decade,
            explicit_frequencies: None,
            data_table_name: None,
            contribution_detail: NoiseContributionDetail::Top50,
            integration_mode: NoiseIntegrationMode::Enabled,
            temperature: 300.15,
        }
    }

    /// The solver trace is part of the request's identity.
    ///
    /// It changes no number the solve produces, and it is still encoded, for
    /// the reason the whole encoder works on: a digest identifies the request
    /// the engine was handed, not the answer it gave back. The byte was
    /// already on the end of the harmonic-balance arm before any form could
    /// author it, so a request that leaves the switch off digests exactly as
    /// it did — which is what lets every sealed manifest still open.
    #[test]
    fn a_harmonic_balance_request_that_asks_for_a_solver_trace_is_a_different_request() {
        let base = AnalysisSpec::HarmonicBalance {
            tones: vec![crate::simulation::multi_run::HbToneSpec::new(1.0e9, 9)],
            reltol: 1.0e-6,
            abstol: 1.0e-12,
            max_iterations: 100,
            damping: 1.0,
            min_damping: 0.01,
            oversample: 2,
            collocation_points: None,
            max_mixing_order: 5,
            use_krylov: false,
            gmres_restart: 30,
            source_stepping: false,
            use_exact_jacobian: true,
            verbose: false,
        };
        let digest = |spec: &AnalysisSpec| {
            analysis_config_digest(".hb", spec, None, &SpecExecutionOptions::default(), None)
        };
        let quiet = digest(&base);
        let mut traced = base.clone();
        let AnalysisSpec::HarmonicBalance { verbose, .. } = &mut traced else {
            unreachable!()
        };
        *verbose = true;
        assert_ne!(quiet, digest(&traced));
    }

    #[test]
    fn sp_noise_changes_identity_even_when_the_directive_is_unchanged() {
        let mut spec = AnalysisSpec::SParameter {
            start_freq: 1e6,
            stop_freq: 3e6,
            points_per_unit: 3,
            sweep: FrequencySweep::Linear,
            z0: 50.0,
            ports: Vec::new(),
            do_noise: false,
        };
        let digest = |spec: &AnalysisSpec| {
            analysis_config_digest(".sp", spec, None, &SpecExecutionOptions::default(), None)
        };
        let scattering = digest(&spec);
        let AnalysisSpec::SParameter { do_noise, .. } = &mut spec else {
            unreachable!()
        };
        *do_noise = true;
        assert_ne!(scattering, digest(&spec));
    }

    #[test]
    fn noise_digest_changes_for_every_exact_execution_field() {
        let base = exact_noise_spec();
        let digest = |spec: &AnalysisSpec, config: Option<&AnalysisConfig>| {
            analysis_config_digest(
                ".noise",
                spec,
                config,
                &SpecExecutionOptions::default(),
                None,
            )
        };
        let baseline = digest(&base, None);

        let mut variants = Vec::new();
        macro_rules! changed {
            ($field:ident, $value:expr) => {{
                let mut spec = base.clone();
                let AnalysisSpec::Noise { $field, .. } = &mut spec else {
                    unreachable!()
                };
                *$field = $value;
                variants.push(spec);
            }};
        }
        changed!(output_node, "other".to_owned());
        changed!(reference_node, "0".to_owned());
        changed!(input_source, "VOTHER".to_owned());
        changed!(start_freq, 11.0);
        changed!(stop_freq, 2.0e6);
        changed!(points_per_decade, 31);
        changed!(sweep, NoiseSweepType::Octave);
        changed!(explicit_frequencies, Some(vec![10.0, 100.0]));
        changed!(data_table_name, Some("points".to_owned()));
        changed!(contribution_detail, NoiseContributionDetail::Top20);
        changed!(integration_mode, NoiseIntegrationMode::OutputNoiseOnly);
        changed!(temperature, 398.15);

        for variant in variants {
            assert_ne!(baseline, digest(&variant, None), "variant: {variant:?}");
        }

        let base_config = crate::simulation::config::NoiseAnalysisConfig::default();
        let mut changed_config = base_config.clone();
        changed_config.contribution_detail = NoiseContributionDetail::SummaryOnly;
        assert_ne!(
            digest(&base, Some(&AnalysisConfig::Noise(base_config)),),
            digest(&base, Some(&AnalysisConfig::Noise(changed_config)),)
        );
    }

    /// A filtered run space narrows to the same axis values as the full
    /// expansion it came from, so the axes alone cannot tell the two apart. If
    /// the digest could not either, a filtered run and the full run would share
    /// one identity and either's results could be attributed to the other.
    #[test]
    fn corner_digest_changes_when_points_are_excluded_from_the_same_axes() {
        use crate::services::simulation_runner::{CornerPoint, CornerProcess, CornerRunConfig};

        let axes = CornerRunConfig {
            process_corners: vec![CornerProcess::TT],
            voltages: vec![0.9, 1.1],
            temperatures_c: vec![-40.0, 125.0],
            full_matrix: true,
            nominal_voltage: Some(1.0),
            ..CornerRunConfig::default()
        };
        let point = |voltage: f64, temperature_c: f64| CornerPoint {
            process: CornerProcess::TT,
            voltage,
            temperature_c,
        };
        // Every axis value is still drawn on, so the two declarations are
        // indistinguishable from their axes alone.
        let filtered = CornerRunConfig {
            points: vec![point(0.9, -40.0), point(1.1, -40.0), point(1.1, 125.0)],
            ..axes.clone()
        };
        let reordered = CornerRunConfig {
            points: vec![point(1.1, 125.0), point(0.9, -40.0), point(1.1, -40.0)],
            ..axes.clone()
        };

        let digest = |corner: &CornerRunConfig| {
            analysis_config_digest(
                ".corner",
                &AnalysisSpec::Corner,
                None,
                &SpecExecutionOptions {
                    corner: Some(corner.clone()),
                    ..SpecExecutionOptions::default()
                },
                None,
            )
        };

        assert_ne!(digest(&axes), digest(&filtered));
        // Execution order is part of the contract: the manifest labels points
        // by position, so a reordered list is a different run.
        assert_ne!(digest(&filtered), digest(&reordered));
        assert_eq!(digest(&filtered), digest(&filtered.clone()));
    }

    /// An unauthored carrier leaves the plan digest exactly as it was.
    ///
    /// The digest identifies the encoding as well as the request, so a field
    /// appended in the middle of one of these arms would give every saved plan
    /// in the family a new identity and detach it from its own results. The
    /// carrier is therefore a conditional tail: the card's absent `FROM=` — the
    /// only thing any request written before this lane could have meant — adds
    /// no bytes, and a named carrier, which binds a different producer and is a
    /// different run, adds one.
    ///
    /// The reference is built from the same writer primitives in the order
    /// that stood before the tail existed, so this is a check against the
    /// encoding rather than against a recorded hash that would be regenerated
    /// along with the defect.
    #[test]
    fn an_unauthored_carrier_leaves_the_plan_digest_unchanged() {
        use crate::services::simulation_runner::{PacRunConfig, PeriodicCarrier};

        let digest = |carrier: PeriodicCarrier| {
            analysis_config_digest(
                ".pac",
                &AnalysisSpec::Pac,
                None,
                &SpecExecutionOptions {
                    pac: Some(PacRunConfig {
                        carrier,
                        ..PacRunConfig::default()
                    }),
                    ..SpecExecutionOptions::default()
                },
                None,
            )
        };

        // The same primitives in the same order `analysis_config_digest` and
        // `encode_spec_options` used before the tail existed.
        let config = PacRunConfig::default();
        let mut writer = CanonicalWriter::new("rspice.analysis-config/v4");
        writer.domain("analysis-line");
        writer.string(".pac");
        encode_analysis_spec(&mut writer, &AnalysisSpec::Pac);
        encode_analysis_config(&mut writer, None);
        writer.domain("spec-execution-options");
        for _ in 0..3 {
            writer.option(None::<&()>, |_, _: &()| unreachable!());
        }
        writer.option(Some(&config), |writer, config| {
            writer.f64(config.pss_fundamental_freq);
            writer.usize(config.pss_num_harmonics);
            writer.f64(config.pss_tolerance);
            writer.f64(config.start_freq);
            writer.f64(config.stop_freq);
            writer.usize(config.points_per_unit);
            writer.u8(pac_sweep_tag(config.sweep));
            writer.i32(config.sideband_max);
            writer.string(&config.input_source);
            writer.string(&config.output_node);
            writer.option(config.output_ref.as_ref(), |w, v| w.string(v));
            writer.f64(config.pac_magnitude);
            writer.bool(config.include_dc);
            writer.f64(config.reltol);
            writer.f64(config.abstol);
        });
        // The `.PXF` slot, the retired TF flag, then `.PNOISE` and `.PSTB`.
        writer.option(None::<&()>, |_, _: &()| unreachable!());
        writer.bool(false);
        for _ in 0..2 {
            writer.option(None::<&()>, |_, _: &()| unreachable!());
        }
        encode_numeric_override(&mut writer, None);

        assert_eq!(
            digest(PeriodicCarrier::Preceding),
            writer.finish(),
            "the absent FROM= keyword must add no bytes to the digest"
        );
        assert_ne!(
            digest(PeriodicCarrier::Preceding),
            digest(PeriodicCarrier::Pss),
            "a named carrier binds a different producer and is a different run"
        );
        assert_ne!(
            digest(PeriodicCarrier::Pss),
            digest(PeriodicCarrier::Hb),
            "the two named carriers are two different runs"
        );
    }

    /// Giving the harmonic-balance carrier a route changes what a request can
    /// *bind*, not what it digests.
    ///
    /// The carrier byte already distinguished the three positions, and a run
    /// linearized about a shooting `.PSS` is the same run it was before the
    /// other family became routable. If this moved, every saved plan carrying
    /// a `FROM=PSS` request would be detached from its own results — and the
    /// plumbing that chooses a producer, the prerequisite role, and the
    /// service entry are all outside the digest for exactly that reason.
    ///
    /// Reconstructed from the writer primitives, in the order and at the
    /// position the tail has always held, so a byte moved anywhere in the arm
    /// fails here instead of being regenerated along with the defect. One arm
    /// proves it for all three, because all three call the one
    /// `encode_periodic_carrier_tail`, and the test above pins that function's
    /// three positions.
    #[test]
    fn a_pss_carried_request_keeps_its_digest() {
        use crate::services::simulation_runner::{PacRunConfig, PeriodicCarrier};

        let config = PacRunConfig {
            carrier: PeriodicCarrier::Pss,
            ..PacRunConfig::default()
        };
        let digest = analysis_config_digest(
            ".pac",
            &AnalysisSpec::Pac,
            None,
            &SpecExecutionOptions {
                pac: Some(config.clone()),
                ..SpecExecutionOptions::default()
            },
            None,
        );

        let mut writer = CanonicalWriter::new("rspice.analysis-config/v4");
        writer.domain("analysis-line");
        writer.string(".pac");
        encode_analysis_spec(&mut writer, &AnalysisSpec::Pac);
        encode_analysis_config(&mut writer, None);
        writer.domain("spec-execution-options");
        for _ in 0..3 {
            writer.option(None::<&()>, |_, _: &()| unreachable!());
        }
        writer.option(Some(&config), |writer, config| {
            writer.f64(config.pss_fundamental_freq);
            writer.usize(config.pss_num_harmonics);
            writer.f64(config.pss_tolerance);
            writer.f64(config.start_freq);
            writer.f64(config.stop_freq);
            writer.usize(config.points_per_unit);
            writer.u8(pac_sweep_tag(config.sweep));
            writer.i32(config.sideband_max);
            writer.string(&config.input_source);
            writer.string(&config.output_node);
            writer.option(config.output_ref.as_ref(), |w, v| w.string(v));
            writer.f64(config.pac_magnitude);
            writer.bool(config.include_dc);
            writer.f64(config.reltol);
            writer.f64(config.abstol);
            // The shooting position's one byte, last in the arm, where the
            // symmetric sideband range adds nothing after it.
            writer.u8(0);
        });
        writer.option(None::<&()>, |_, _: &()| unreachable!());
        writer.bool(false);
        for _ in 0..2 {
            writer.option(None::<&()>, |_, _: &()| unreachable!());
        }
        encode_numeric_override(&mut writer, None);

        assert_eq!(
            digest,
            writer.finish(),
            "a FROM=PSS request digests the bytes it digested before the other family had a route"
        );
    }

    /// A symmetric sideband range digests exactly as the single bound it
    /// replaced, and an asymmetric one earns its own identity.
    ///
    /// The bottom of the range is the second conditional tail on this arm, for
    /// the same reason the carrier is the first: every run recorded before the
    /// range could be stated asymmetrically was symmetric, and giving those
    /// runs a new digest would detach each from its own results.
    #[test]
    fn a_symmetric_sideband_range_leaves_the_plan_digest_unchanged() {
        use crate::services::simulation_runner::PacRunConfig;

        let digest = |sideband_min: i32, sideband_max: i32| {
            analysis_config_digest(
                ".pac",
                &AnalysisSpec::Pac,
                None,
                &SpecExecutionOptions {
                    pac: Some(PacRunConfig {
                        sideband_min,
                        sideband_max,
                        ..PacRunConfig::default()
                    }),
                    ..SpecExecutionOptions::default()
                },
                None,
            )
        };

        // The default range is symmetric, so it must digest as the arm did
        // before the bottom end existed — which is what the reference in
        // `an_unauthored_carrier_leaves_the_plan_digest_unchanged` builds.
        assert_eq!(digest(-5, 5), digest(-5, 5));
        assert_ne!(
            digest(-5, 5),
            digest(-2, 5),
            "an asymmetric range is a different solve over different sidebands"
        );
        assert_ne!(digest(-2, 5), digest(-2, 7));
        // Zero is its own symmetric case: `-0 == 0`, so it adds no tail.
        assert_eq!(digest(0, 0), digest(0, 0));
    }

    #[test]
    fn pss_digest_changes_for_every_exact_contract_value() {
        let base = AnalysisSpec::Pss {
            method: crate::simulation::multi_run::PssMethod::Shooting,
            fundamental_freq: 1.0e6,
            tone_sources: vec!["VCLK".to_owned()],
            tstab_periods: 20,
            points_per_period: 512,
            tolerance: 1.0e-7,
            oscillator_mode: false,
            oscillator_node: None,
            num_harmonics: 20,
            integration_method: None,
            tstab: 0.0,
            max_iterations: 100,
            abstol: 1.0e-12,
            damping: 1.0,
            max_period_change: 0.1,
            verbose: false,
        };
        let digest = |spec: &AnalysisSpec| {
            analysis_config_digest(".pss", spec, None, &SpecExecutionOptions::default(), None)
        };
        let baseline = digest(&base);
        let mut variants = Vec::new();
        macro_rules! changed {
            ($field:ident, $value:expr) => {{
                let mut spec = base.clone();
                let AnalysisSpec::Pss { $field, .. } = &mut spec else {
                    unreachable!()
                };
                *$field = $value;
                variants.push(spec);
            }};
        }
        changed!(
            method,
            crate::simulation::multi_run::PssMethod::HarmonicBalance
        );
        changed!(fundamental_freq, 2.0e6);
        changed!(tone_sources, vec!["VLO".to_owned(), "VRF".to_owned()]);
        changed!(tstab_periods, 21);
        changed!(points_per_period, 1024);
        changed!(tolerance, 2.0e-8);
        changed!(oscillator_mode, true);
        changed!(oscillator_node, Some("osc".to_owned()));
        changed!(num_harmonics, 21);
        changed!(
            integration_method,
            Some(crate::simulation::dialog::IntegrationMethod::Euler)
        );
        changed!(tstab, 3.0e-9);
        changed!(max_iterations, 250);
        changed!(abstol, 1.0e-15);
        changed!(damping, 0.75);
        changed!(max_period_change, 0.25);
        changed!(verbose, true);

        for variant in variants {
            assert_ne!(baseline, digest(&variant), "variant: {variant:?}");
        }
    }

    #[test]
    fn operating_point_digest_is_sensitive_to_every_contract_field() {
        let base = AnalysisSpec::dc_op();
        let digest = |spec: &AnalysisSpec| {
            analysis_config_digest(".op", spec, None, &SpecExecutionOptions::default(), None)
        };
        let baseline = digest(&base);

        let mut variants = Vec::new();
        macro_rules! changed {
            ($field:ident, $value:expr) => {{
                let mut spec = base.clone();
                let AnalysisSpec::DcOp { $field, .. } = &mut spec else {
                    unreachable!()
                };
                *$field = $value;
                variants.push(spec);
            }};
        }
        changed!(temperature_mode, OpTemperatureMode::Explicit);
        changed!(temperature_celsius, 91.25);
        changed!(initial_guess, OpInitialGuess::ZeroState);
        changed!(node_initialization, OpNodeInitialization::ForceIcValues);
        changed!(homotopy, OpHomotopy::SourceStepping);
        changed!(annotation, OpAnnotation::VoltagesOnly);
        changed!(device_detail, OpDeviceDetail::AllDevices);
        changed!(save_device_op, OpSaveDevice::Disabled);
        changed!(accuracy, OpAccuracy::Accurate);
        changed!(selected_devices, vec!["M1".to_owned()]);
        changed!(
            previous_state,
            Some(OpPreviousState {
                source_content_digest: ContentDigest::from_bytes([1; 32]),
                producer_snapshot_digest: ContentDigest::from_bytes([2; 32]),
                producer_result_digest: ContentDigest::from_bytes([3; 32]),
                node_names: vec!["out".to_owned()],
                branch_names: vec!["V1".to_owned()],
                solution: vec![1.0, -1.0e-3],
            })
        );
        changed!(violation_devices, vec!["M2".to_owned()]);
        changed!(
            violation_source_content_digest,
            Some(ContentDigest::from_bytes([4; 32]))
        );
        changed!(
            run_point,
            OpRunPointContext {
                index: 1,
                count: 2,
                ..OpRunPointContext::default()
            }
        );

        for variant in variants {
            assert_ne!(
                baseline,
                digest(&variant),
                "unchanged digest for {variant:?}"
            );
        }
    }

    #[test]
    fn length_prefixes_prevent_concatenation_collisions() {
        let mut left = CanonicalWriter::new("collision-test");
        left.string("ab");
        left.string("c");
        let mut right = CanonicalWriter::new("collision-test");
        right.string("a");
        right.string("bc");
        assert_ne!(left.finish(), right.finish());
    }

    #[test]
    fn domain_separators_prevent_cross_type_collisions() {
        assert_ne!(
            content_digest("source/v1", b"same"),
            content_digest("model/v1", b"same")
        );
    }

    #[test]
    fn fourier_result_controls_are_bound_into_the_config_digest() {
        let spec = |compute_thd, normalize| AnalysisSpec::Fourier {
            fundamental_freq: 1.0e6,
            num_harmonics: 10,
            num_periods: 1,
            output_node: "out".to_owned(),
            output_ref: "0".to_owned(),
            additional_outputs: Vec::new(),
            start_time: 0.0,
            stop_time: 10.0e-6,
            compute_thd,
            normalize,
        };
        let digest = |spec: &AnalysisSpec| {
            analysis_config_digest(".four", spec, None, &SpecExecutionOptions::default(), None)
        };

        let baseline = digest(&spec(true, false));
        assert_ne!(baseline, digest(&spec(false, false)));
        assert_ne!(baseline, digest(&spec(true, true)));
    }

    /// A Fourier plan that names one output digests to exactly the bytes it
    /// digested to when the arm ended at `normalize`.
    ///
    /// The output list is appended as a conditional tail for that reason: a
    /// one-output plan saved before the list existed must keep its identity,
    /// while two runs that decompose different outputs must not share one.
    #[test]
    fn a_single_output_fourier_leaves_the_plan_digest_unchanged() {
        let fourier = |additional_outputs: Vec<String>| AnalysisSpec::Fourier {
            fundamental_freq: 1.0e6,
            num_harmonics: 10,
            num_periods: 1,
            output_node: "out".to_owned(),
            output_ref: "0".to_owned(),
            additional_outputs,
            start_time: 0.0,
            stop_time: 10.0e-6,
            compute_thd: true,
            normalize: false,
        };
        let spec = fourier(Vec::new());
        let mut encoded = CanonicalWriter::new("test");
        encode_analysis_spec(&mut encoded, &spec);

        let mut before_the_field = CanonicalWriter::new("test");
        before_the_field.domain("analysis-spec");
        before_the_field.u8(analysis_kind_tag(&spec));
        before_the_field.f64(1.0e6);
        before_the_field.usize(10);
        before_the_field.string("out");
        before_the_field.string("0");
        before_the_field.f64(0.0);
        before_the_field.f64(10.0e-6);
        before_the_field.bool(true);
        before_the_field.bool(false);

        assert_eq!(
            encoded.finish(),
            before_the_field.finish(),
            "a one-output Fourier plan must not move a saved plan's identity"
        );

        let mut one = CanonicalWriter::new("test");
        encode_analysis_spec(&mut one, &spec);
        let mut two = CanonicalWriter::new("test");
        encode_analysis_spec(&mut two, &fourier(vec!["V(mid)".to_owned()]));
        assert_ne!(one.finish(), two.finish());

        // The card writes the list in authored order, so the order is part of
        // the identity too.
        let mut forward = CanonicalWriter::new("test");
        encode_analysis_spec(
            &mut forward,
            &fourier(vec!["V(mid)".to_owned(), "I(V1)".to_owned()]),
        );
        let mut reversed = CanonicalWriter::new("test");
        encode_analysis_spec(
            &mut reversed,
            &fourier(vec!["I(V1)".to_owned(), "V(mid)".to_owned()]),
        );
        assert_ne!(forward.finish(), reversed.finish());
    }

    #[test]
    fn every_transfer_function_field_is_bound_into_the_config_digest() {
        let base = AnalysisSpec::Tf {
            input_source: "VIN_DIFF".to_owned(),
            output_expression: "V(afe_out)".to_owned(),
            transfer_gain: true,
            input_resistance: true,
            output_resistance: true,
            normalization: TfNormalization::None,
            accuracy: TfAccuracy::Balanced,
        };
        let digest = |spec: &AnalysisSpec| {
            analysis_config_digest(".tf", spec, None, &SpecExecutionOptions::default(), None)
        };
        let baseline = digest(&base);
        let mutations: [fn(&mut AnalysisSpec); 7] = [
            |spec| {
                let AnalysisSpec::Tf { input_source, .. } = spec else {
                    unreachable!()
                };
                *input_source = "IIN_CAL".to_owned();
            },
            |spec| {
                let AnalysisSpec::Tf {
                    output_expression, ..
                } = spec
                else {
                    unreachable!()
                };
                *output_expression = "I(VDD)".to_owned();
            },
            |spec| {
                let AnalysisSpec::Tf { transfer_gain, .. } = spec else {
                    unreachable!()
                };
                *transfer_gain = false;
            },
            |spec| {
                let AnalysisSpec::Tf {
                    input_resistance, ..
                } = spec
                else {
                    unreachable!()
                };
                *input_resistance = false;
            },
            |spec| {
                let AnalysisSpec::Tf {
                    output_resistance, ..
                } = spec
                else {
                    unreachable!()
                };
                *output_resistance = false;
            },
            |spec| {
                let AnalysisSpec::Tf { normalization, .. } = spec else {
                    unreachable!()
                };
                *normalization = TfNormalization::RelativeToNominal;
            },
            |spec| {
                let AnalysisSpec::Tf { accuracy, .. } = spec else {
                    unreachable!()
                };
                *accuracy = TfAccuracy::Robust;
            },
        ];

        for mutation in mutations {
            let mut changed = base.clone();
            mutation(&mut changed);
            assert_ne!(baseline, digest(&changed));
        }
    }

    #[test]
    fn envelope_owned_controls_are_bound_into_the_config_digest() {
        let base = AnalysisSpec::Envelope {
            initialization: Default::default(),
            fundamental_freq: 1.0e6,
            additional_carrier_tones: vec![2.0e6],
            stop_time: 10.0e-3,
            num_harmonics: 9,
            envelope_step: Some(1.0e-6),
            modulation_sources: vec!["VIN_AM".to_owned()],
            initial_periodic_solve: EnvelopeInitialPeriodicSolve::HarmonicBalance,
            adaptive_mode: EnvelopeAdaptiveMode::Enabled,
            extraction_path: EnvelopeExtractionPath::Projection,
        };
        let digest = |spec: &AnalysisSpec| {
            analysis_config_digest(".envlp", spec, None, &SpecExecutionOptions::default(), None)
        };
        let baseline = digest(&base);

        let mut changed_tones = base.clone();
        if let AnalysisSpec::Envelope {
            additional_carrier_tones,
            ..
        } = &mut changed_tones
        {
            additional_carrier_tones.push(3.0e6);
        }
        assert_ne!(baseline, digest(&changed_tones));

        let mut changed_sources = base.clone();
        if let AnalysisSpec::Envelope {
            modulation_sources, ..
        } = &mut changed_sources
        {
            modulation_sources.push("VCTRL".to_owned());
        }
        assert_ne!(baseline, digest(&changed_sources));

        let mut changed_initial = base.clone();
        if let AnalysisSpec::Envelope {
            initial_periodic_solve,
            ..
        } = &mut changed_initial
        {
            *initial_periodic_solve = EnvelopeInitialPeriodicSolve::PeriodicSteadyState;
        }
        assert_ne!(baseline, digest(&changed_initial));

        let mut changed_adaptive = base.clone();
        if let AnalysisSpec::Envelope { adaptive_mode, .. } = &mut changed_adaptive {
            *adaptive_mode = EnvelopeAdaptiveMode::EventAlignedOnly;
        }
        assert_ne!(baseline, digest(&changed_adaptive));
    }

    #[test]
    fn manual_task_ids_are_reproducible_and_bound_to_source_kind_and_occurrence() {
        let source = content_digest("manual-expanded-source/v1", b"deck\n.op\n.end\n");
        let changed_source = content_digest(
            "manual-expanded-source/v1",
            b"deck\nR1 out 0 1k\n.op\n.end\n",
        );
        let op = AnalysisSpec::dc_op();
        let first = manual_deck_analysis_instance_id(source, &op, 0);

        assert_eq!(first, manual_deck_analysis_instance_id(source, &op, 0));
        assert_eq!(
            first,
            manual_deck_analysis_instance_id_from_tag(source, analysis_kind_tag(&op), 0,)
        );
        assert_ne!(
            first,
            manual_deck_analysis_instance_id(changed_source, &op, 0)
        );
        assert_ne!(first, manual_deck_analysis_instance_id(source, &op, 1));
        assert_ne!(
            first,
            manual_deck_analysis_instance_id(
                source,
                &AnalysisSpec::Transient {
                    stop_time: 1.0,
                    step_time: 0.1,
                    start_time: 0.0,
                    max_timestep: None,
                    uic: false,
                },
                0,
            )
        );
    }

    #[test]
    fn drc_receipt_is_independent_of_incidental_violation_order() {
        let mut first = DrcResult::new();
        first.add_violation(DrcViolation::new(
            1,
            DrcViolationType::MissingGround,
            "missing ground",
            DrcLocation::Global,
        ));
        first.add_violation(DrcViolation::new(
            2,
            DrcViolationType::UnconnectedPin,
            "pin",
            DrcLocation::Wire { id: 9 },
        ));
        first.completed = true;
        let mut second = DrcResult::new();
        for violation in first.violations().iter().rev() {
            second.add_violation(violation.clone());
        }
        second.completed = true;
        assert_eq!(
            drc_receipt_digest(7, &first),
            drc_receipt_digest(7, &second)
        );
    }

    #[test]
    fn drc_receipt_retains_full_durable_bus_identity() {
        let digest_for = |location| {
            let mut result = DrcResult::new();
            result.add_violation(DrcViolation::new(
                1,
                DrcViolationType::MalformedBus,
                "malformed bus",
                location,
            ));
            result.completed = true;
            drc_receipt_digest(9, &result)
        };

        assert_ne!(
            digest_for(DrcLocation::Bus { id: 1 }),
            digest_for(DrcLocation::Bus {
                id: 1 + (1_u64 << 32),
            })
        );
        assert_ne!(
            digest_for(DrcLocation::BusTap { id: 1 }),
            digest_for(DrcLocation::BusTap {
                id: 1 + (1_u64 << 32),
            })
        );
        assert_ne!(
            digest_for(DrcLocation::Component {
                id: 1,
                name: "R1".to_owned(),
            }),
            digest_for(DrcLocation::Component {
                id: 1 + (1_u64 << 32),
                name: "R1".to_owned(),
            })
        );
        assert_ne!(
            digest_for(DrcLocation::Wire { id: 1 }),
            digest_for(DrcLocation::Wire {
                id: 1 + (1_u64 << 32),
            })
        );
    }
    #[test]
    fn periodic_port_noise_options_have_distinct_execution_identities() {
        use rspice_core::analysis::s_param::PeriodicPortNoiseReference;
        let request = |hb: bool, reference| {
            if hb {
                AnalysisSpec::Hbsp {
                    start_freq: 1e4,
                    stop_freq: 1e4,
                    points_per_unit: 1,
                    sweep: crate::simulation::multi_run::FrequencySweep::Linear,
                    ports: Vec::new(),
                    max_sideband: 1,
                    mixed_mode: false,
                    noise_parameters: true,
                    noise_reference: reference,
                }
            } else {
                AnalysisSpec::Psp {
                    start_freq: 1e4,
                    stop_freq: 1e4,
                    points_per_unit: 1,
                    sweep: crate::simulation::multi_run::FrequencySweep::Linear,
                    ports: Vec::new(),
                    max_sideband: 1,
                    mixed_mode: false,
                    noise_parameters: true,
                    noise_reference: reference,
                }
            }
        };
        let baseline = PeriodicPortNoiseReference::default();
        let digest = |spec: &AnalysisSpec| {
            analysis_config_digest(
                "* periodic network",
                spec,
                None,
                &SpecExecutionOptions::default(),
                None,
            )
        };
        for hb in [false, true] {
            let spec = request(hb, Some(baseline.clone()));
            assert!(spec.validate().is_ok());
            let expected = digest(&spec);
            let restored: AnalysisSpec =
                serde_json::from_str(&serde_json::to_string(&spec).unwrap()).unwrap();
            assert_eq!(expected, digest(&restored));
            assert_ne!(expected, digest(&request(hb, None)));
            let changes: [fn(&mut PeriodicPortNoiseReference); 7] = [
                |r| r.input_port = 2,
                |r| r.output_port = 1,
                |r| r.input_sideband = -1,
                |r| r.output_sideband = 1,
                |r| r.reference_temperature_kelvin = 325.0,
                |r| r.termination_temperature_kelvin = 0.0,
                |r| r.image_sideband = Some(-1),
            ];
            for change in changes {
                let mut reference = baseline.clone();
                change(&mut reference);
                assert_ne!(expected, digest(&request(hb, Some(reference))));
            }
            let mut invalid = baseline.clone();
            invalid.input_sideband = 2;
            assert!(request(hb, Some(invalid)).validate().is_err());
            let mut disabled = spec;
            match &mut disabled {
                AnalysisSpec::Psp {
                    noise_parameters, ..
                }
                | AnalysisSpec::Hbsp {
                    noise_parameters, ..
                } => *noise_parameters = false,
                _ => unreachable!(),
            }
            assert!(disabled.validate().is_err());
        }
    }
}
