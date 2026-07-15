//! Versioned canonical encoding for execution identities.
//!
//! This encoding is deliberately independent of `Debug`, serde formats, map
//! iteration order, and host word size. Every aggregate starts with a domain
//! separator; variable-length values carry a big-endian `u64` length.

use sha2::{Digest as _, Sha256};

use crate::product::ContentDigest;
use crate::services::drc::{DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType};
use crate::services::simulation_runner::{
    CornerBaseMode, CornerFrequencySweep, CornerProcess, PacFrequencySweep, PnoiseFrequencySweep,
    PnoiseReference, PxfFrequencySweep, TfFrequencySweep,
};
use crate::simulation::AnalysisConfig;
use crate::simulation::config::{AcSweepType, PzAnalysisType};
use crate::simulation::multi_run::{
    AnalysisSpec, FrequencySweep, OptimizationAlgorithm, OptimizationGoal,
};
use crate::simulation::runner::SpecExecutionOptions;
use crate::state::SimulationRunIntent;

const CANONICAL_MAGIC: &[u8] = b"RSPICE-CANONICAL";
const CANONICAL_VERSION: u16 = 1;

pub(super) struct CanonicalWriter {
    hasher: Sha256,
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

pub(in crate::simulation) fn analysis_instance_id(
    intent: SimulationRunIntent,
    spec: &AnalysisSpec,
    kind_occurrence: usize,
) -> ContentDigest {
    let mut writer = CanonicalWriter::new("rspice.analysis-instance/v1");
    writer.domain("intent");
    writer.u8(run_intent_tag(intent));
    writer.domain("analysis-kind");
    writer.u8(analysis_kind_tag(spec));
    writer.domain("kind-occurrence");
    writer.usize(kind_occurrence);
    writer.finish()
}

pub(in crate::simulation) fn analysis_config_digest(
    analysis_line: &str,
    spec: &AnalysisSpec,
    config: Option<&AnalysisConfig>,
    options: &SpecExecutionOptions,
) -> ContentDigest {
    let mut writer = CanonicalWriter::new("rspice.analysis-config/v1");
    writer.domain("analysis-line");
    writer.string(analysis_line);
    encode_analysis_spec(&mut writer, spec);
    encode_analysis_config(&mut writer, config);
    encode_spec_options(&mut writer, options);
    writer.finish()
}

fn encode_analysis_config(writer: &mut CanonicalWriter, config: Option<&AnalysisConfig>) {
    writer.domain("engine-analysis-config");
    writer.option(config, |writer, config| match config {
        AnalysisConfig::DcOp => writer.u8(0),
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

pub(in crate::simulation) fn manual_source_receipt_digest(
    source: &str,
    executable_netlist: &str,
    origin: Option<&str>,
    task_digests: &[ContentDigest],
) -> ContentDigest {
    let mut writer = CanonicalWriter::new("rspice.manual-source-check-receipt/v1");
    writer.domain("source");
    writer.string(source);
    writer.domain("origin");
    writer.option(origin, |writer, origin| writer.string(origin));
    writer.domain("expanded-executable-netlist");
    writer.string(executable_netlist);
    writer.domain("accepted-task-configs");
    writer.sequence(task_digests.len());
    for digest in task_digests {
        writer.digest(*digest);
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
            writer.usize(*id);
            writer.string(name);
        }
        DrcLocation::Wire { id } => {
            writer.u8(2);
            writer.usize(*id);
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
    }
}

fn encode_analysis_spec(writer: &mut CanonicalWriter, spec: &AnalysisSpec) {
    writer.domain("analysis-spec");
    writer.u8(analysis_kind_tag(spec));
    match spec {
        AnalysisSpec::DcOp
        | AnalysisSpec::Tf
        | AnalysisSpec::Pac
        | AnalysisSpec::Pnoise
        | AnalysisSpec::Pxf
        | AnalysisSpec::Pstb
        | AnalysisSpec::MonteCarlo
        | AnalysisSpec::Parametric
        | AnalysisSpec::Corner => {}
        AnalysisSpec::DcSweep {
            source_name,
            start,
            stop,
            step,
            source2,
            start2,
            stop2,
            step2,
        } => {
            writer.string(source_name);
            writer.f64(*start);
            writer.f64(*stop);
            writer.f64(*step);
            writer.option(source2.as_ref(), |w, v| w.string(v));
            writer.option(start2.as_ref(), |w, v| w.f64(*v));
            writer.option(stop2.as_ref(), |w, v| w.f64(*v));
            writer.option(step2.as_ref(), |w, v| w.f64(*v));
        }
        AnalysisSpec::Ac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
        } => {
            writer.f64(*start_freq);
            writer.f64(*stop_freq);
            writer.usize(*points_per_unit);
            encode_frequency_sweep(writer, *sweep);
        }
        AnalysisSpec::AcData {
            table_name,
            frequencies,
        } => {
            writer.string(table_name);
            encode_f64_slice(writer, frequencies);
        }
        AnalysisSpec::Disto {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            f2_over_f1,
        } => {
            writer.f64(*start_freq);
            writer.f64(*stop_freq);
            writer.usize(*points_per_unit);
            encode_frequency_sweep(writer, *sweep);
            writer.option(f2_over_f1.as_ref(), |w, v| w.f64(*v));
        }
        AnalysisSpec::Transient {
            stop_time,
            step_time,
            start_time,
            max_timestep,
            uic,
        } => {
            writer.f64(*stop_time);
            writer.f64(*step_time);
            writer.f64(*start_time);
            writer.option(max_timestep.as_ref(), |w, v| w.f64(*v));
            writer.bool(*uic);
        }
        AnalysisSpec::Noise {
            output_node,
            start_freq,
            stop_freq,
            points_per_decade,
            temperature,
        } => {
            writer.string(output_node);
            writer.f64(*start_freq);
            writer.f64(*stop_freq);
            writer.usize(*points_per_decade);
            writer.f64(*temperature);
        }
        AnalysisSpec::Pss {
            fundamental_freq,
            num_harmonics,
            tolerance,
        } => {
            writer.f64(*fundamental_freq);
            writer.usize(*num_harmonics);
            writer.f64(*tolerance);
        }
        AnalysisSpec::HarmonicBalance {
            tones,
            reltol,
            abstol,
            max_iterations,
            damping,
            oversample,
            collocation_points,
            max_mixing_order,
            use_krylov,
            gmres_restart,
            source_stepping,
            verbose,
        } => {
            writer.sequence(tones.len());
            for tone in tones {
                writer.f64(tone.frequency);
                writer.usize(tone.harmonics);
                writer.option(tone.source.as_ref(), |w, v| w.string(v));
                writer.option(tone.name.as_ref(), |w, v| w.string(v));
            }
            writer.f64(*reltol);
            writer.f64(*abstol);
            writer.usize(*max_iterations);
            writer.f64(*damping);
            writer.usize(*oversample);
            writer.option(collocation_points.as_ref(), |w, v| w.usize(*v));
            writer.usize(*max_mixing_order);
            writer.bool(*use_krylov);
            writer.usize(*gmres_restart);
            writer.bool(*source_stepping);
            writer.bool(*verbose);
        }
        AnalysisSpec::Sensitivity {
            output_var,
            ac_mode,
            frequency,
        } => {
            writer.string(output_var);
            writer.bool(*ac_mode);
            writer.option(frequency.as_ref(), |w, v| w.f64(*v));
        }
        AnalysisSpec::PoleZero {
            input_node,
            input_ref,
            output_node,
            output_ref,
            transfer_type,
            analysis_type,
        } => {
            writer.string(input_node);
            writer.string(input_ref);
            writer.string(output_node);
            writer.string(output_ref);
            writer.string(transfer_type);
            writer.string(analysis_type);
        }
        AnalysisSpec::Stb {
            probe_node,
            start_freq,
            stop_freq,
            sweep,
            points_per_decade,
        } => {
            writer.string(probe_node);
            writer.f64(*start_freq);
            writer.f64(*stop_freq);
            encode_frequency_sweep(writer, *sweep);
            writer.usize(*points_per_decade);
        }
        AnalysisSpec::Reliability {
            target_years,
            enable_hci,
            enable_nbti,
            enable_em,
            min_stress_voltage,
        } => {
            encode_f64_slice(writer, target_years);
            writer.bool(*enable_hci);
            writer.bool(*enable_nbti);
            writer.bool(*enable_em);
            writer.f64(*min_stress_voltage);
        }
        AnalysisSpec::Optimization {
            variables,
            objective_node,
            objective_ref,
            goal,
            target,
            algorithm,
            max_iterations,
            cost_tolerance,
            fd_step,
            initial_step,
            min_step,
        } => {
            writer.sequence(variables.len());
            for variable in variables {
                writer.string(&variable.name);
                writer.f64(variable.min);
                writer.f64(variable.max);
                writer.f64(variable.initial);
            }
            writer.string(objective_node);
            writer.string(objective_ref);
            writer.u8(match goal {
                OptimizationGoal::Minimize => 0,
                OptimizationGoal::Maximize => 1,
                OptimizationGoal::Target => 2,
            });
            writer.option(target.as_ref(), |w, v| w.f64(*v));
            writer.u8(match algorithm {
                OptimizationAlgorithm::GradientDescent => 0,
                OptimizationAlgorithm::PatternSearch => 1,
                OptimizationAlgorithm::SimulatedAnnealing => 2,
            });
            writer.usize(*max_iterations);
            writer.f64(*cost_tolerance);
            writer.f64(*fd_step);
            writer.f64(*initial_step);
            writer.f64(*min_step);
        }
        AnalysisSpec::Soa {
            stop_time,
            step_time,
            check_vgs_max,
            max_vgs,
            check_vds_max,
            max_vds,
            check_vbe_max,
            max_vbe,
            check_vce_max,
            max_vce,
        } => {
            writer.f64(*stop_time);
            writer.f64(*step_time);
            writer.bool(*check_vgs_max);
            writer.f64(*max_vgs);
            writer.bool(*check_vds_max);
            writer.f64(*max_vds);
            writer.bool(*check_vbe_max);
            writer.f64(*max_vbe);
            writer.bool(*check_vce_max);
            writer.f64(*max_vce);
        }
        AnalysisSpec::SParameter {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            z0,
            ports,
        } => {
            writer.f64(*start_freq);
            writer.f64(*stop_freq);
            writer.usize(*points_per_unit);
            encode_frequency_sweep(writer, *sweep);
            writer.f64(*z0);
            writer.sequence(ports.len());
            for port in ports {
                writer.string(&port.node_pos);
                writer.string(&port.node_neg);
                writer.option(port.z0.as_ref(), |w, v| w.f64(*v));
            }
        }
        AnalysisSpec::Envelope {
            fundamental_freq,
            stop_time,
            num_harmonics,
            max_step,
        } => {
            writer.f64(*fundamental_freq);
            writer.f64(*stop_time);
            writer.usize(*num_harmonics);
            writer.option(max_step.as_ref(), |w, v| w.f64(*v));
        }
        AnalysisSpec::Fourier {
            fundamental_freq,
            num_harmonics,
            output_node,
            output_ref,
            start_time,
            stop_time,
        } => {
            writer.f64(*fundamental_freq);
            writer.usize(*num_harmonics);
            writer.string(output_node);
            writer.string(output_ref);
            writer.f64(*start_time);
            writer.f64(*stop_time);
        }
    }
}

fn encode_spec_options(writer: &mut CanonicalWriter, options: &SpecExecutionOptions) {
    writer.domain("spec-execution-options");
    writer.option(options.temp.as_ref(), |writer, config| {
        encode_f64_slice(writer, &config.temperatures_c);
        encode_corner_base_mode(writer, &config.base_mode);
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
        encode_corner_base_mode(writer, &config.base_mode);
        writer.sequence(config.model_bindings.len());
        for binding in &config.model_bindings {
            writer.u8(corner_process_tag(binding.process));
            writer.string(&binding.source_label);
            writer.option(binding.section.as_ref(), |w, v| w.string(v));
            writer.string(&binding.materialized_model_cards);
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
        writer.i32(config.max_sideband);
        writer.string(&config.input_source);
        writer.string(&config.output_node);
        writer.option(config.output_ref.as_ref(), |w, v| w.string(v));
        writer.f64(config.pac_magnitude);
        writer.bool(config.include_dc);
        writer.f64(config.reltol);
        writer.f64(config.abstol);
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
    });
    writer.option(options.tf.as_ref(), |writer, config| {
        writer.f64(config.start_freq);
        writer.f64(config.stop_freq);
        writer.usize(config.points_per_unit);
        writer.u8(tf_sweep_tag(config.sweep));
        writer.string(&config.input_source);
        writer.string(&config.output_node);
        writer.option(config.output_ref.as_ref(), |w, v| w.string(v));
        writer.bool(config.group_delay);
        writer.bool(config.input_impedance);
        writer.bool(config.output_impedance);
    });
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
}

fn encode_corner_base_mode(writer: &mut CanonicalWriter, mode: &CornerBaseMode) {
    writer.domain("corner-base-mode");
    match mode {
        CornerBaseMode::Op => writer.u8(0),
        CornerBaseMode::DcSweep {
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
        }
        CornerBaseMode::Transient {
            stop_time,
            step_time,
        } => {
            writer.u8(2);
            writer.f64(*stop_time);
            writer.f64(*step_time);
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

fn encode_f64_slice(writer: &mut CanonicalWriter, values: &[f64]) {
    writer.sequence(values.len());
    for value in values {
        writer.f64(*value);
    }
}

fn encode_frequency_sweep(writer: &mut CanonicalWriter, sweep: FrequencySweep) {
    writer.u8(match sweep {
        FrequencySweep::Decade => 0,
        FrequencySweep::Octave => 1,
        FrequencySweep::Linear => 2,
    });
}

pub(in crate::simulation) fn analysis_kind_tag(spec: &AnalysisSpec) -> u8 {
    match spec {
        AnalysisSpec::DcOp => 0,
        AnalysisSpec::DcSweep { .. } => 1,
        AnalysisSpec::Ac { .. } => 2,
        AnalysisSpec::AcData { .. } => 3,
        AnalysisSpec::Disto { .. } => 4,
        AnalysisSpec::Transient { .. } => 5,
        AnalysisSpec::Noise { .. } => 6,
        AnalysisSpec::Pss { .. } => 7,
        AnalysisSpec::HarmonicBalance { .. } => 8,
        AnalysisSpec::Tf => 9,
        AnalysisSpec::Sensitivity { .. } => 10,
        AnalysisSpec::PoleZero { .. } => 11,
        AnalysisSpec::Pac => 12,
        AnalysisSpec::Pnoise => 13,
        AnalysisSpec::Pxf => 14,
        AnalysisSpec::Pstb => 15,
        AnalysisSpec::Stb { .. } => 16,
        AnalysisSpec::MonteCarlo => 17,
        AnalysisSpec::Parametric => 18,
        AnalysisSpec::Corner => 19,
        AnalysisSpec::Reliability { .. } => 20,
        AnalysisSpec::Optimization { .. } => 21,
        AnalysisSpec::Soa { .. } => 22,
        AnalysisSpec::SParameter { .. } => 23,
        AnalysisSpec::Envelope { .. } => 24,
        AnalysisSpec::Fourier { .. } => 25,
    }
}

fn run_intent_tag(intent: SimulationRunIntent) -> u8 {
    match intent {
        SimulationRunIntent::SimulateRunSet => 0,
        SimulationRunIntent::ManualDeck => 1,
    }
}

fn corner_process_tag(process: CornerProcess) -> u8 {
    match process {
        CornerProcess::TT => 0,
        CornerProcess::SS => 1,
        CornerProcess::FF => 2,
        CornerProcess::SF => 3,
        CornerProcess::FS => 4,
    }
}

fn pac_sweep_tag(sweep: PacFrequencySweep) -> u8 {
    match sweep {
        PacFrequencySweep::Decade => 0,
        PacFrequencySweep::Octave => 1,
        PacFrequencySweep::Linear => 2,
    }
}

fn pxf_sweep_tag(sweep: PxfFrequencySweep) -> u8 {
    match sweep {
        PxfFrequencySweep::Decade => 0,
        PxfFrequencySweep::Octave => 1,
        PxfFrequencySweep::Linear => 2,
    }
}

fn tf_sweep_tag(sweep: TfFrequencySweep) -> u8 {
    match sweep {
        TfFrequencySweep::Decade => 0,
        TfFrequencySweep::Octave => 1,
        TfFrequencySweep::Linear => 2,
    }
}

fn pnoise_sweep_tag(sweep: PnoiseFrequencySweep) -> u8 {
    match sweep {
        PnoiseFrequencySweep::Decade => 0,
        PnoiseFrequencySweep::Octave => 1,
        PnoiseFrequencySweep::Linear => 2,
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
        DrcViolationType::ShortCircuit => 6,
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::drc::{DrcLocation, DrcViolation};

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
}
