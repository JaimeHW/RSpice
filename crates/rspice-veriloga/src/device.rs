//! Verilog-A Device Interface
//!
//! This module provides the runtime device interface for compiled Verilog-A models.
//! Devices can be instantiated in a circuit and stamped into the solver matrix.
//!
//! # Architecture
//!
//! ```text
//! CompiledModel (bytecode) + VmContext (runtime state)
//!         ↓
//! VerilogADevice (instance in circuit)
//!         ↓
//! stamp() → Matrix + RHS
//! ```
//!
//! # Native Compilation
//!
//! When the `native` feature is enabled, model construction requires a complete
//! native JIT image. Unsupported native compilation returns a typed error rather
//! than silently running the bytecode VM interpreter.
//!
//! Production circuit-builder flows that enable native Verilog-A carry the
//! canonical IR artifact and construct devices through
//! `VerilogADevice::try_new_with_canonical_ir` (itself `native`-gated). The
//! direct [`VerilogADevice::try_new`](crate::device::VerilogADevice::try_new)
//! constructor has no canonical artifact to consume, so under normal `native`
//! builds it fails closed instead of compiling from bytecode. Bytecode-native
//! construction is available only behind the internal
//! `native-bytecode-contract-tests` feature for backend contract tests.
//!
//! The native path is a performance backend, not a separate public ABI. Keep all
//! raw-pointer and Send/Sync safety reasoning in the native module and require a
//! targeted audit before expanding that boundary.

use crate::canonical_ir::CanonicalIrArtifact;
#[cfg(all(
    not(feature = "native"),
    not(all(feature = "wasm-jit", target_arch = "wasm32"))
))]
use crate::codegen::BytecodeProgram;
use crate::codegen::{AssignmentStep, CompiledModel, Instruction, StampIndex};
use crate::vm::{CURRENT_PAIR_GROUND, Vm, VmAcceptedCheckpoint, VmContext, VmError};
#[cfg(feature = "native")]
use crate::vm::{terminal_pair_current_endpoints, terminal_pair_current_len};
use smol_str::SmolStr;

/// Collision-resistant identity of every ordered runtime slot/table that a
/// compiled native or browser image addresses by numeric index.
///
/// Canonical MIR authenticates the equations, but derivative-shadow variables
/// and several bytecode-era runtime tables are materialized only in
/// [`CompiledModel`].  Their exact order is therefore part of the executable
/// ABI and must participate in cross-allocation cache reuse.
#[cfg(any(
    feature = "native",
    all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32")
))]
#[derive(Clone, Copy, PartialEq, Eq)]
struct CompiledModelLayoutIdentity(blake3::Hash);

#[cfg(any(
    feature = "native",
    all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32")
))]
fn compiled_model_layout_identity(model: &CompiledModel) -> CompiledModelLayoutIdentity {
    fn usize_field(hasher: &mut blake3::Hasher, value: usize) {
        hasher.update(&(value as u64).to_le_bytes());
    }

    fn string_field(hasher: &mut blake3::Hasher, value: &str) {
        usize_field(hasher, value.len());
        hasher.update(value.as_bytes());
    }

    fn stamp_index(hasher: &mut blake3::Hasher, index: &StampIndex) {
        let (tag, slot) = match index {
            StampIndex::Terminal(slot) => (0_u8, *slot),
            StampIndex::Internal(slot) => (1, *slot),
            StampIndex::Branch(slot) => (2, *slot),
            StampIndex::Ground => (3, 0),
        };
        hasher.update(&[tag]);
        usize_field(hasher, slot);
    }

    fn assignment_layout(hasher: &mut blake3::Hasher, steps: &[AssignmentStep]) {
        usize_field(hasher, steps.len());
        for step in steps {
            match step {
                AssignmentStep::Assign(assignment) => {
                    hasher.update(&[0]);
                    usize_field(hasher, assignment.var_index);
                }
                AssignmentStep::AssignIndexed {
                    base, len, lower, ..
                } => {
                    hasher.update(&[1]);
                    usize_field(hasher, *base);
                    usize_field(hasher, *len);
                    hasher.update(&lower.to_le_bytes());
                }
                AssignmentStep::Loop { body, .. } => {
                    hasher.update(&[2]);
                    assignment_layout(hasher, body);
                }
            }
        }
    }

    let mut hasher = blake3::Hasher::new();
    hasher.update(b"rspice-compiled-model-layout-v1\0");

    usize_field(&mut hasher, model.num_terminals);
    usize_field(&mut hasher, model.terminal_names.len());
    for name in &model.terminal_names {
        string_field(&mut hasher, name);
    }

    usize_field(&mut hasher, model.parameters.len());
    for parameter in &model.parameters {
        string_field(&mut hasher, &parameter.name);
    }

    usize_field(&mut hasher, model.num_variables);
    usize_field(&mut hasher, model.variable_names.len());
    for name in &model.variable_names {
        string_field(&mut hasher, name);
    }
    usize_field(&mut hasher, model.event_state_variables.len());
    for slot in &model.event_state_variables {
        usize_field(&mut hasher, *slot);
    }
    assignment_layout(&mut hasher, &model.assignment_steps);
    assignment_layout(&mut hasher, &model.noise_assignment_steps);

    usize_field(&mut hasher, model.internal_nodes);
    usize_field(&mut hasher, model.branch_sources.len());
    for source in &model.branch_sources {
        stamp_index(&mut hasher, &source.pos);
        stamp_index(&mut hasher, &source.neg);
        hasher.update(&[u8::from(source.indirect)]);
    }

    usize_field(&mut hasher, model.stamp_programs.len());
    for stamp in &model.stamp_programs {
        usize_field(&mut hasher, stamp.stamp_locations.len());
        for location in &stamp.stamp_locations {
            stamp_index(&mut hasher, &location.row);
            stamp_index(&mut hasher, &location.col);
            hasher.update(&location.sign.to_bits().to_le_bytes());
        }
        usize_field(&mut hasher, stamp.jacobian_programs.len());
        for jacobian in &stamp.jacobian_programs {
            stamp_index(&mut hasher, &jacobian.row);
            stamp_index(&mut hasher, &jacobian.col);
            match jacobian.col_axis {
                crate::codegen::ColumnAxis::Node(slot) => {
                    hasher.update(&[0]);
                    usize_field(&mut hasher, slot);
                }
                crate::codegen::ColumnAxis::Branch(slot) => {
                    hasher.update(&[1]);
                    usize_field(&mut hasher, slot);
                }
            }
            hasher.update(&jacobian.sign.to_bits().to_le_bytes());
        }
        usize_field(&mut hasher, stamp.reactive_jacobians.len());
        for jacobian in &stamp.reactive_jacobians {
            stamp_index(&mut hasher, &jacobian.row);
            stamp_index(&mut hasher, &jacobian.col);
            match jacobian.col_axis {
                crate::codegen::ColumnAxis::Node(slot) => {
                    hasher.update(&[0]);
                    usize_field(&mut hasher, slot);
                }
                crate::codegen::ColumnAxis::Branch(slot) => {
                    hasher.update(&[1]);
                    usize_field(&mut hasher, slot);
                }
            }
            hasher.update(&jacobian.sign.to_bits().to_le_bytes());
        }
        usize_field(&mut hasher, stamp.branch_ordinal.unwrap_or(usize::MAX));
        hasher.update(&[
            u8::from(stamp.indirect),
            u8::from(stamp.static_condition.is_some()),
        ]);
    }

    usize_field(&mut hasher, model.lookup_tables.len());
    usize_field(&mut hasher, model.laplace_filters.len());
    usize_field(&mut hasher, model.zi_filters.len());
    usize_field(&mut hasher, model.zi_filter_definitions.len());
    hasher.update(&model.noise_process_schema.to_le_bytes());
    usize_field(&mut hasher, model.noise_sources.len());
    for source in &model.noise_sources {
        usize_field(&mut hasher, source.process_id);
        stamp_index(&mut hasher, &source.pos);
        stamp_index(&mut hasher, &source.neg);
        hasher.update(&[u8::from(source.is_current)]);
        usize_field(&mut hasher, source.branch_ordinal.unwrap_or(usize::MAX));
        usize_field(&mut hasher, source.program_idx);
        hasher.update(&[
            u8::from(source.exponent_program.is_some()),
            u8::from(source.table.is_some()),
        ]);
        usize_field(&mut hasher, source.injections.len());
        for injection in &source.injections {
            stamp_index(&mut hasher, &injection.pos);
            stamp_index(&mut hasher, &injection.neg);
            hasher.update(&[u8::from(injection.is_current)]);
            usize_field(&mut hasher, injection.branch_ordinal.unwrap_or(usize::MAX));
            usize_field(&mut hasher, injection.program_idx);
            hasher.update(&injection.rhs_sign.to_bits().to_le_bytes());
        }
    }

    CompiledModelLayoutIdentity(hasher.finalize())
}

#[cfg(feature = "native")]
use crate::native::{NativeModel, NativeRequiredStorage, NativeStampKernelIo};
#[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
use crate::wasm_jit::{WasmJitExecutable, WasmJitExecutableEntry};

#[cfg(feature = "native")]
#[derive(Clone, Copy)]
enum NativeValueEntry {
    ParameterDefault(usize),
    StaticCondition(usize),
    StampValue(usize),
    Jacobian { stamp: usize, entry: usize },
    ReactiveJacobian { stamp: usize, entry: usize },
    NoisePsd(usize),
    NoiseExponent(usize),
}

#[cfg(test)]
mod runtime_checkpoint_codec_tests {
    use super::{
        CheckpointWordEncoder, RUNTIME_CHECKPOINT_STATE_VERSION, VerilogADeviceCheckpoint,
    };
    use crate::laplace::LaplaceCheckpoint;
    use crate::vm::{
        DelayCheckpoint, DelayConfiguration, PendingTransitionCheckpoint, SlewCheckpoint,
        TransitionCheckpoint, TransitionSegmentCheckpoint, VmAcceptedCheckpoint,
    };

    fn checkpoint_with_slew_entries() -> VerilogADeviceCheckpoint {
        VerilogADeviceCheckpoint {
            instance_name: "x1".into(),
            model_name: "model".into(),
            source_digest: "0123456789abcdef".repeat(4).into(),
            shape_identity: "fedcba9876543210".repeat(4).into(),
            state_version: RUNTIME_CHECKPOINT_STATE_VERSION,
            accepted: VmAcceptedCheckpoint {
                time: 2.5,
                variables: vec![-0.0, f64::INFINITY],
                state_values_prev: vec![f64::from_bits(1)],
                state_values_older: vec![-f64::MIN_POSITIVE],
                state_derivatives_prev: vec![1.0 / 3.0],
                state_initialized: vec![true],
                delay_buffers: vec![
                    DelayCheckpoint {
                        configuration: None,
                        samples: Vec::new(),
                    },
                    DelayCheckpoint {
                        configuration: Some(DelayConfiguration::Fixed { delay: 0.25 }),
                        samples: vec![(0.0, 1.0), (1.0, 2.0)],
                    },
                    DelayCheckpoint {
                        configuration: Some(DelayConfiguration::Bounded { max_delay: 2.0 }),
                        samples: vec![(0.5, -1.0), (2.0, 4.0)],
                    },
                ],
                transition_filters: vec![TransitionCheckpoint {
                    input: 3.0,
                    output: 1.5,
                    time: 2.5,
                    active: Some(TransitionSegmentCheckpoint {
                        origin_time: 1.0,
                        origin_value: 0.0,
                        destination: 3.0,
                        end_time: 4.0,
                    }),
                    pending: vec![
                        PendingTransitionCheckpoint {
                            start_time: 5.0,
                            destination: -1.0,
                            rise_time: 0.25,
                            fall_time: 0.5,
                        },
                        PendingTransitionCheckpoint {
                            start_time: 6.0,
                            destination: 2.0,
                            rise_time: 0.75,
                            fall_time: 1.0,
                        },
                    ],
                    initialized: true,
                }],
                slew_filters: vec![
                    SlewCheckpoint {
                        output: -0.0,
                        prev_time: 1.25,
                        next_corner_time: None,
                        initialized: false,
                    },
                    SlewCheckpoint {
                        output: 3.5,
                        prev_time: 2.5,
                        next_corner_time: Some(4.0),
                        initialized: true,
                    },
                ],
                cross_detectors: Vec::new(),
                laplace_filters: vec![LaplaceCheckpoint {
                    state: vec![-0.0, f64::MIN_POSITIVE],
                    older_state: vec![1.25, -2.5],
                    derivative: vec![3.0, -4.0],
                }],
                zi_filters: Vec::new(),
                timer_event_bound: Some(4.0),
            },
            prev_discontinuity: true,
        }
    }

    fn minimal_legacy_v1_words() -> Vec<u64> {
        let mut encoder = CheckpointWordEncoder::default();
        encoder.word(1);
        encoder.boolean(false);
        encoder.float(4.0);
        encoder.floats(&[]);
        encoder.floats(&[]);
        encoder.floats(&[]);
        encoder.floats(&[]);
        encoder.booleans(&[]);
        encoder.word(0); // delay buffers
        encoder.word(0); // transition filters
        encoder.word(2); // legacy slew filters: output + previous time only
        encoder.float(-0.0);
        encoder.float(1.25);
        encoder.float(3.5);
        encoder.float(2.5);
        encoder.word(0); // cross detectors
        encoder.word(0); // Laplace filters
        encoder.word(0); // Zi filters
        encoder.optional_float(None);
        encoder.words
    }

    #[test]
    fn accepted_runtime_word_payload_round_trips_ieee_bits_and_rejects_trailing_data() {
        let checkpoint = checkpoint_with_slew_entries();
        let words = checkpoint.to_words();
        let decoded = VerilogADeviceCheckpoint::from_words(
            checkpoint.instance_name.clone(),
            checkpoint.model_name.clone(),
            checkpoint.source_digest.clone(),
            checkpoint.shape_identity.clone(),
            &words,
        )
        .expect("canonical accepted payload decodes");
        assert_eq!(decoded, checkpoint);
        assert!(!decoded.accepted.slew_filters[0].initialized);
        assert!(decoded.accepted.slew_filters[1].initialized);
        assert_eq!(decoded.accepted.slew_filters[1].next_corner_time, Some(4.0));
        assert_eq!(decoded.accepted.transition_filters[0].pending.len(), 2);
        assert_eq!(
            decoded.accepted.delay_buffers[1].configuration,
            Some(DelayConfiguration::Fixed { delay: 0.25 })
        );
        assert_eq!(
            decoded.accepted.delay_buffers[2].configuration,
            Some(DelayConfiguration::Bounded { max_delay: 2.0 })
        );
        assert_eq!(
            decoded.accepted.transition_filters[0].active,
            checkpoint.accepted.transition_filters[0].active
        );
        assert_eq!(
            decoded.accepted.laplace_filters, checkpoint.accepted.laplace_filters,
            "all accepted Laplace integration lanes must round-trip exactly"
        );

        let mut trailing = words;
        trailing.push(0);
        assert!(
            VerilogADeviceCheckpoint::from_words(
                checkpoint.instance_name,
                checkpoint.model_name,
                checkpoint.source_digest,
                checkpoint.shape_identity,
                &trailing,
            )
            .expect_err("trailing words must fail closed")
            .contains("trailing words")
        );
    }

    #[test]
    fn current_runtime_word_payload_rejects_invalid_absdelay_configuration() {
        let checkpoint = checkpoint_with_slew_entries();
        let words = checkpoint.to_words();
        let delay_section = words
            .windows(4)
            .position(|window| window == [3, 0, 0, 1])
            .expect("delay section contains None followed by Fixed configuration");

        let mut invalid_tag = words.clone();
        invalid_tag[delay_section + 1] = 3;
        assert!(
            VerilogADeviceCheckpoint::from_words(
                checkpoint.instance_name.clone(),
                checkpoint.model_name.clone(),
                checkpoint.source_digest.clone(),
                checkpoint.shape_identity.clone(),
                &invalid_tag,
            )
            .expect_err("unknown delay configuration tags must fail closed")
            .contains("delay 0 configuration tag 3 is invalid")
        );

        let mut invalid_value = checkpoint.clone();
        invalid_value.accepted.delay_buffers[1].configuration =
            Some(DelayConfiguration::Fixed { delay: 0.0 });
        assert!(
            VerilogADeviceCheckpoint::from_words(
                invalid_value.instance_name.clone(),
                invalid_value.model_name.clone(),
                invalid_value.source_digest.clone(),
                invalid_value.shape_identity.clone(),
                &invalid_value.to_words(),
            )
            .expect_err("non-positive delay configurations must fail closed")
            .contains("configuration is not finite and positive")
        );

        let mut missing_configuration = checkpoint;
        missing_configuration.accepted.delay_buffers[1].configuration = None;
        assert!(
            VerilogADeviceCheckpoint::from_words(
                missing_configuration.instance_name.clone(),
                missing_configuration.model_name.clone(),
                missing_configuration.source_digest.clone(),
                missing_configuration.shape_identity.clone(),
                &missing_configuration.to_words(),
            )
            .expect_err("accepted samples without configuration must fail closed")
            .contains("unconfigured delay checkpoint contains accepted samples")
        );
    }

    #[test]
    fn current_runtime_word_payload_rejects_malformed_slew_boolean_and_truncation() {
        let checkpoint = checkpoint_with_slew_entries();
        let words = checkpoint.to_words();

        let first_slew_initialized = words
            .windows(5)
            .position(|window| {
                window
                    == [
                        2,
                        (-0.0_f64).to_bits(),
                        1.25_f64.to_bits(),
                        0,
                        u64::from(false),
                    ]
            })
            .expect("first slew entry has a unique serialized prefix")
            + 4;
        let mut malformed = words.clone();
        malformed[first_slew_initialized] = 2;
        assert!(
            VerilogADeviceCheckpoint::from_words(
                checkpoint.instance_name.clone(),
                checkpoint.model_name.clone(),
                checkpoint.source_digest.clone(),
                checkpoint.shape_identity.clone(),
                &malformed,
            )
            .expect_err("non-boolean slew initialization tag must fail closed")
            .contains("slew 0 initialized boolean tag 2 is invalid")
        );

        let slew_count = first_slew_initialized - 4;
        assert!(
            VerilogADeviceCheckpoint::from_words(
                checkpoint.instance_name,
                checkpoint.model_name,
                checkpoint.source_digest,
                checkpoint.shape_identity,
                &words[..slew_count + 4],
            )
            .expect_err("truncated slew entry must fail closed")
            .contains("slew filters declares 2 entries")
        );
    }

    #[test]
    fn legacy_v1_payload_is_fully_validated_without_migrating_slew_initialization() {
        let words = minimal_legacy_v1_words();
        VerilogADeviceCheckpoint::validate_legacy_v1_words(&words)
            .expect("complete two-word legacy slew entries validate");

        assert!(
            VerilogADeviceCheckpoint::from_words(
                "x1".into(),
                "model".into(),
                "source".into(),
                "shape".into(),
                &words,
            )
            .expect_err("the current decoder must not migrate legacy state")
            .contains("unsupported runtime Verilog-A state version 1")
        );

        let mut malformed_boolean = words.clone();
        malformed_boolean[1] = 2;
        assert!(
            VerilogADeviceCheckpoint::validate_legacy_v1_words(&malformed_boolean)
                .expect_err("legacy boolean tags remain strict")
                .contains("previous discontinuity boolean tag 2 is invalid")
        );

        assert!(
            VerilogADeviceCheckpoint::validate_legacy_v1_words(&words[..12])
                .expect_err("truncated two-word legacy slew entry must fail closed")
                .contains("slew filters declares 2 entries")
        );
    }

    #[test]
    fn legacy_v2_payload_is_fully_validated_without_migrating_idtmod_history() {
        let checkpoint = checkpoint_with_slew_entries();
        let words = checkpoint.to_legacy_v2_words_for_test();

        VerilogADeviceCheckpoint::validate_legacy_v2_words(&words)
            .expect("complete legacy common-format payload validates");
        assert!(
            VerilogADeviceCheckpoint::from_words(
                checkpoint.instance_name,
                checkpoint.model_name,
                checkpoint.source_digest,
                checkpoint.shape_identity,
                &words,
            )
            .expect_err("the current decoder must not reinterpret legacy idtmod history")
            .contains("unsupported runtime Verilog-A state version 2")
        );

        let mut malformed_boolean = words.clone();
        malformed_boolean[1] = 2;
        assert!(
            VerilogADeviceCheckpoint::validate_legacy_v2_words(&malformed_boolean)
                .expect_err("legacy v2 boolean tags remain strict")
                .contains("previous discontinuity boolean tag 2 is invalid")
        );

        let mut trailing = words;
        trailing.push(0);
        assert!(
            VerilogADeviceCheckpoint::validate_legacy_v2_words(&trailing)
                .expect_err("legacy v2 trailing words must fail closed")
                .contains("trailing words")
        );
    }

    #[test]
    fn legacy_v3_payload_is_validated_without_inventing_slew_corners() {
        let checkpoint = checkpoint_with_slew_entries();
        let words = checkpoint.to_legacy_v3_words_for_test();
        VerilogADeviceCheckpoint::validate_legacy_v3_words(&words)
            .expect("complete legacy v3 payload validates");
        assert!(
            VerilogADeviceCheckpoint::from_words(
                checkpoint.instance_name,
                checkpoint.model_name,
                checkpoint.source_digest,
                checkpoint.shape_identity,
                &words,
            )
            .expect_err("v3 cannot resume without its accepted slew corner")
            .contains("unsupported runtime Verilog-A state version 3")
        );
    }

    #[test]
    fn legacy_v4_payload_is_validated_without_inventing_transition_queue_history() {
        let checkpoint = checkpoint_with_slew_entries();
        let words = checkpoint.to_legacy_v4_words_for_test();
        VerilogADeviceCheckpoint::validate_legacy_v4_words(&words)
            .expect("complete legacy v4 payload validates");
        assert!(
            VerilogADeviceCheckpoint::from_words(
                checkpoint.instance_name,
                checkpoint.model_name,
                checkpoint.source_digest,
                checkpoint.shape_identity,
                &words,
            )
            .expect_err("v4 cannot resume without accepted transition queue history")
            .contains("unsupported runtime Verilog-A state version 4")
        );
    }

    #[test]
    fn legacy_v5_payload_is_validated_without_inventing_absdelay_configuration() {
        let checkpoint = checkpoint_with_slew_entries();
        let words = checkpoint.to_legacy_v5_words_for_test();
        VerilogADeviceCheckpoint::validate_legacy_v5_words(&words)
            .expect("complete legacy v5 delay history validates");
        assert!(
            VerilogADeviceCheckpoint::from_words(
                checkpoint.instance_name.clone(),
                checkpoint.model_name.clone(),
                checkpoint.source_digest.clone(),
                checkpoint.shape_identity.clone(),
                &words,
            )
            .expect_err("v5 cannot resume without its frozen absdelay definition")
            .contains("unsupported runtime Verilog-A state version 5")
        );

        let legacy_delay_section = words
            .windows(4)
            .position(|window| window == [3, 0, 2, 0])
            .expect("legacy delay section begins with empty and fixed histories");
        let mut malformed = words;
        malformed[legacy_delay_section + 3] = f64::NAN.to_bits();
        assert!(
            VerilogADeviceCheckpoint::validate_legacy_v5_words(&malformed)
                .expect_err("legacy v5 delay samples must remain fully validated")
                .contains("delay sample 0 is not finite")
        );
    }

    #[test]
    fn legacy_v6_payload_is_validated_without_inventing_laplace_history() {
        let checkpoint = checkpoint_with_slew_entries();
        let words = checkpoint.to_legacy_v6_words_for_test();
        VerilogADeviceCheckpoint::validate_legacy_v6_words(&words)
            .expect("complete legacy v6 Laplace state validates");
        assert!(
            VerilogADeviceCheckpoint::from_words(
                checkpoint.instance_name.clone(),
                checkpoint.model_name.clone(),
                checkpoint.source_digest.clone(),
                checkpoint.shape_identity.clone(),
                &words,
            )
            .expect_err("v6 cannot resume without older Laplace state and derivative history")
            .contains("unsupported runtime Verilog-A state version 6")
        );

        let state_word = words
            .windows(4)
            .position(|window| {
                window
                    == [
                        1, // one Laplace filter
                        2, // two state values
                        (-0.0_f64).to_bits(),
                        f64::MIN_POSITIVE.to_bits(),
                    ]
            })
            .expect("legacy Laplace state has a unique serialized prefix")
            + 2;
        let mut malformed = words;
        malformed[state_word] = f64::NAN.to_bits();
        assert!(
            VerilogADeviceCheckpoint::validate_legacy_v6_words(&malformed)
                .expect_err("legacy v6 Laplace state must remain fully validated")
                .contains("Laplace filter 0 state contains a non-finite value")
        );
    }

    #[test]
    fn current_runtime_word_payload_rejects_inconsistent_laplace_history_lanes() {
        let mut checkpoint = checkpoint_with_slew_entries();
        checkpoint.accepted.laplace_filters[0].older_state.pop();
        assert!(
            VerilogADeviceCheckpoint::from_words(
                checkpoint.instance_name.clone(),
                checkpoint.model_name.clone(),
                checkpoint.source_digest.clone(),
                checkpoint.shape_identity.clone(),
                &checkpoint.to_words(),
            )
            .expect_err("mismatched Laplace integration lanes must fail closed")
            .contains("history lengths are 2/1/2")
        );
    }
}

#[cfg(feature = "native")]
struct NativeEntryDependencies<'a> {
    current_pairs: &'a [usize],
    prior_currents: &'a [usize],
    branch_unknowns: &'a [usize],
}

/// Invalid instance parameter value reported before it can enter a model.
#[derive(Debug, Clone, PartialEq)]
pub enum ParameterValueError {
    NonFinite {
        parameter: SmolStr,
        value: f64,
    },
    NonInteger {
        parameter: SmolStr,
        value: f64,
    },
    OutOfRange {
        parameter: SmolStr,
        value: f64,
        constraint: String,
    },
    Excluded {
        parameter: SmolStr,
        value: f64,
    },
    InvalidConstraint {
        parameter: SmolStr,
        detail: String,
    },
}

impl std::fmt::Display for ParameterValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFinite { parameter, value } => {
                write!(
                    f,
                    "parameter '{parameter}' requires a finite value, got {value}"
                )
            }
            Self::NonInteger { parameter, value } => {
                write!(
                    f,
                    "integer parameter '{parameter}' requires an exact integer, got {value}"
                )
            }
            Self::OutOfRange {
                parameter,
                value,
                constraint,
            } => write!(
                f,
                "parameter '{parameter}' value {value} violates range {constraint}"
            ),
            Self::Excluded { parameter, value } => write!(
                f,
                "parameter '{parameter}' value {value} is explicitly excluded"
            ),
            Self::InvalidConstraint { parameter, detail } => {
                write!(
                    f,
                    "parameter '{parameter}' has invalid range metadata: {detail}"
                )
            }
        }
    }
}

impl std::error::Error for ParameterValueError {}

/// Content identity of one native compilation.
///
/// Keying on content rather than on the `Arc<CompiledModel>` address is what
/// lets a second engine build reuse the first build's image: the runtime model
/// cache hands back a freshly allocated `Arc` after a disk-cache hit, so
/// pointer identity reports a miss for a model that is byte-identical.
///
/// The digests are carried together because the emitted image is a function of
/// both artifacts the compiler consumes. `mir_digest` alone determines the
/// image for artifacts produced by one compiler build, but the pair costs
/// nothing and keeps the key honest if that ever stops being true.
#[cfg(feature = "native")]
#[derive(Clone)]
enum NativeCompileCacheKey {
    /// Internal bytecode-native contract-test cache lane. Production native
    /// construction must compile through `CanonicalMir`. Bytecode compilation
    /// is tied to the lifetime of the bytecode model allocation because that
    /// lane has no canonical artifact whose digest independently authenticates
    /// the complete compiler input.
    #[cfg(feature = "native-bytecode-contract-tests")]
    Bytecode {
        source_digest: SmolStr,
        module: SmolStr,
        owner: std::sync::Weak<CompiledModel>,
    },
    CanonicalMir {
        mir_digest: SmolStr,
        source_digest: SmolStr,
        module: SmolStr,
        layout: CompiledModelLayoutIdentity,
    },
}

#[cfg(feature = "native")]
impl PartialEq for NativeCompileCacheKey {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            #[cfg(feature = "native-bytecode-contract-tests")]
            (
                Self::Bytecode {
                    source_digest: left_source,
                    module: left_module,
                    ..
                },
                Self::Bytecode {
                    source_digest: right_source,
                    module: right_module,
                    ..
                },
            ) => left_source == right_source && left_module == right_module,
            (
                Self::CanonicalMir {
                    mir_digest: left_mir,
                    source_digest: left_source,
                    module: left_module,
                    layout: left_layout,
                },
                Self::CanonicalMir {
                    mir_digest: right_mir,
                    source_digest: right_source,
                    module: right_module,
                    layout: right_layout,
                },
            ) => {
                left_mir == right_mir
                    && left_source == right_source
                    && left_module == right_module
                    && left_layout == right_layout
            }
            #[cfg(feature = "native-bytecode-contract-tests")]
            _ => false,
        }
    }
}

#[cfg(feature = "native")]
impl Eq for NativeCompileCacheKey {}

/// Lifetime contract for a cached native image.
///
/// Canonical MIR authenticates every compiler input, so its images are
/// process-persistent and reusable across separately allocated runtime models.
/// The internal bytecode lane has no such artifact and is allocation-scoped:
/// its image and cached failures remain valid only while that exact
/// `CompiledModel` allocation is alive.
#[cfg(feature = "native")]
enum NativeCompileCacheRetention {
    Persistent,
    #[cfg(feature = "native-bytecode-contract-tests")]
    Model(std::sync::Weak<CompiledModel>),
}

#[cfg(feature = "native")]
impl NativeCompileCacheRetention {
    fn for_key(key: &NativeCompileCacheKey) -> Self {
        match key {
            #[cfg(feature = "native-bytecode-contract-tests")]
            NativeCompileCacheKey::Bytecode { owner, .. } => Self::Model(owner.clone()),
            NativeCompileCacheKey::CanonicalMir { .. } => Self::Persistent,
        }
    }

    fn is_dropped(&self) -> bool {
        match self {
            Self::Persistent => false,
            #[cfg(feature = "native-bytecode-contract-tests")]
            Self::Model(owner) => owner.strong_count() == 0,
        }
    }

    fn matches_key(&self, key: &NativeCompileCacheKey) -> bool {
        match (self, key) {
            (Self::Persistent, NativeCompileCacheKey::CanonicalMir { .. }) => true,
            #[cfg(feature = "native-bytecode-contract-tests")]
            (Self::Model(retained), NativeCompileCacheKey::Bytecode { owner, .. }) => {
                std::sync::Weak::ptr_eq(retained, owner)
            }
            #[cfg(feature = "native-bytecode-contract-tests")]
            _ => false,
        }
    }
}

/// Content identity of one browser-side compilation, matching
/// [`NativeCompileCacheKey`]'s canonical lane.
#[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
#[derive(Clone, PartialEq, Eq)]
struct WasmCompileCacheKey {
    mir_digest: SmolStr,
    source_digest: SmolStr,
    module: SmolStr,
    layout: CompiledModelLayoutIdentity,
}

/// Executable-image budget for the process-wide native compilation cache.
///
/// Entries hold committed executable pages, so the cache is bounded by bytes
/// rather than by entry count. Compilation failures are retained at zero cost
/// so a failing model is not recompiled once per instance.
#[cfg(feature = "native")]
const NATIVE_COMPILE_CACHE_DEFAULT_MAX_BYTES: usize = 512 * 1024 * 1024;

#[cfg(feature = "native")]
const NATIVE_COMPILE_CACHE_MAX_BYTES_ENV: &str = "RSPICE_VERILOGA_NATIVE_CACHE_MAX_BYTES";

/// Module names of the compilations that actually reached the backend, so
/// tests can assert a cache hit rather than infer one from wall-clock time.
///
/// Recorded per model rather than as one counter: the test binary compiles
/// models on several threads at once, so a single count sampled across a
/// window observes whatever other tests happened to compile meanwhile.
#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
static NATIVE_COMPILE_LOG: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

/// How many times `module` reached the backend.
#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
pub(crate) fn native_compile_count(module: &str) -> usize {
    NATIVE_COMPILE_LOG
        .lock()
        .expect("native compile log")
        .iter()
        .filter(|name| name.as_str() == module)
        .count()
}

#[cfg(feature = "native")]
struct NativeCompileCacheEntry {
    key: NativeCompileCacheKey,
    retention: NativeCompileCacheRetention,
    compiled: Result<std::sync::Arc<NativeModel>, String>,
    image_bytes: usize,
}

#[cfg(feature = "native")]
#[derive(Default)]
struct NativeCompileCache {
    /// Most-recently-used first.
    entries: Vec<NativeCompileCacheEntry>,
    image_bytes: usize,
}

#[cfg(feature = "native")]
impl NativeCompileCache {
    fn max_bytes() -> usize {
        std::env::var(NATIVE_COMPILE_CACHE_MAX_BYTES_ENV)
            .ok()
            .and_then(|raw| raw.trim().parse::<usize>().ok())
            .filter(|budget| *budget > 0)
            .unwrap_or(NATIVE_COMPILE_CACHE_DEFAULT_MAX_BYTES)
    }

    fn get(
        &mut self,
        key: &NativeCompileCacheKey,
    ) -> Option<Result<std::sync::Arc<NativeModel>, String>> {
        self.prune_dropped_bytecode_owners();
        let index = self
            .entries
            .iter()
            .position(|entry| entry.key == *key && entry.retention.matches_key(key))?;
        let entry = self.entries.remove(index);
        let compiled = entry.compiled.clone();
        self.entries.insert(0, entry);
        self.debug_assert_image_bytes();
        Some(compiled)
    }

    fn insert(
        &mut self,
        key: NativeCompileCacheKey,
        compiled: Result<std::sync::Arc<NativeModel>, String>,
    ) {
        self.prune_dropped_bytecode_owners();
        let retention = NativeCompileCacheRetention::for_key(&key);
        let image_bytes = compiled
            .as_ref()
            .map_or(0, |native| native.code_size_bytes());
        self.entries.insert(
            0,
            NativeCompileCacheEntry {
                key,
                retention,
                compiled,
                image_bytes,
            },
        );
        self.image_bytes = self
            .image_bytes
            .checked_add(image_bytes)
            .expect("native compile cache image accounting exceeds addressable memory");
        self.evict_to(Self::max_bytes());
        self.debug_assert_image_bytes();
    }

    /// Remove test-lane images after their bytecode model allocation has been
    /// dropped. Canonical MIR entries deliberately have no owner and remain
    /// reusable across model allocations and engine rebuilds.
    fn prune_dropped_bytecode_owners(&mut self) {
        let mut index = 0;
        while index < self.entries.len() {
            if self.entries[index].retention.is_dropped() {
                let evicted = self.entries.remove(index);
                self.image_bytes = self
                    .image_bytes
                    .checked_sub(evicted.image_bytes)
                    .expect("native compile cache image accounting underflowed while pruning");
            } else {
                index += 1;
            }
        }
        self.debug_assert_image_bytes();
    }

    fn debug_assert_image_bytes(&self) {
        #[cfg(debug_assertions)]
        {
            let accounted = self.entries.iter().fold(0usize, |total, entry| {
                total
                    .checked_add(entry.image_bytes)
                    .expect("native compile cache entry sizes exceed addressable memory")
            });
            debug_assert_eq!(
                self.image_bytes, accounted,
                "native compile cache executable-image accounting drifted"
            );
        }
    }

    /// Drop least-recently-used images until the budget is met, always keeping
    /// the entry that was just inserted so one oversized model cannot evict
    /// itself into an infinite recompile loop.
    fn evict_to(&mut self, max_bytes: usize) {
        while self.image_bytes > max_bytes && self.entries.len() > 1 {
            let Some(evicted) = self.entries.pop() else {
                break;
            };
            self.image_bytes = self
                .image_bytes
                .checked_sub(evicted.image_bytes)
                .expect("native compile cache image accounting underflowed while evicting");
        }
    }
}

/// A Verilog-A device instance in a circuit
///
/// Holds the compiled model, runtime context, and circuit connectivity.
#[derive(Debug, Clone)]
pub struct VerilogADevice {
    /// Device instance name
    pub name: SmolStr,
    /// Compiled model, shared between clones. Newton line-search snapshots
    /// clone devices per probe; sharing the (potentially megabyte-scale)
    /// program keeps that clone proportional to runtime state only.
    model: std::sync::Arc<CompiledModel>,
    /// Runtime execution context
    context: VmContext,
    /// Mapping from terminal index to circuit node ID (0 = ground)
    node_mapping: Vec<usize>,
    /// Mapping from internal node index to circuit node ID
    /// When the solver allocates circuit nodes for internal nodes, this maps them
    internal_node_indices: Vec<usize>,
    /// Number of internal nodes in this device
    num_internal_nodes: usize,
    /// Mapping from branch-current unknown ordinal to circuit node ID
    /// (the engine allocates one extra system unknown per potential
    /// contribution branch)
    branch_current_indices: Vec<usize>,
    /// Per stamp program: instance-static activation (parameter-only
    /// guards evaluated after parameter resolution)
    program_active: Vec<bool>,
    /// Per branch unknown: whether any potential contribution drives it
    /// (an undriven branch is forced to zero current)
    branch_active: Vec<bool>,
    /// Pre-computed matrix indices for O(1) stamping
    matrix_indices: MatrixIndices,
    /// Preallocated transaction buffer for one matrix-stamp pass. Solver
    /// callbacks are invoked only after the complete pass validates.
    stamp_matrix_buffer: Vec<(usize, usize, f64)>,
    /// Preallocated transaction buffer for one RHS-stamp pass.
    stamp_rhs_buffer: Vec<(usize, f64)>,
    /// Byte-addressable mirror of `program_active` for the fused drivers,
    /// which read activation out of a raw array rather than a packed
    /// `Vec<bool>`.
    #[cfg(any(feature = "native", all(feature = "wasm-jit", target_arch = "wasm32")))]
    fused_program_active: Vec<u8>,
    /// Flat, model-order Jacobian output storage for the fused stamp driver.
    #[cfg(any(feature = "native", all(feature = "wasm-jit", target_arch = "wasm32")))]
    fused_stamp_jacobians: Vec<f64>,
    /// Shared canonical CFG slice that evaluates raw grouped-noise metadata
    /// with exact source control flow and reaching definitions.
    canonical_noise_plan: Option<std::sync::Arc<CanonicalNoiseRuntimePlan>>,
    /// Native compiled model. In native mode this is required: construction
    /// fails if a complete native image cannot be produced.
    #[cfg(feature = "native")]
    native_model: std::sync::Arc<NativeModel>,
    /// Dense semantic export table for the worker-installed secondary module.
    #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
    wasm_jit_model: std::sync::Arc<WasmJitExecutable>,
    /// $discontinuity level at the last accepted timestep (edge detector)
    prev_discontinuity: bool,
}

pub const RUNTIME_CHECKPOINT_STATE_VERSION: u32 = 7;

/// Versioned accepted runtime state for one compiled Verilog-A instance.
/// Compiled programs, topology, and solver caches are intentionally absent.
#[derive(Debug, Clone, PartialEq)]
pub struct VerilogADeviceCheckpoint {
    pub instance_name: SmolStr,
    pub model_name: SmolStr,
    pub source_digest: SmolStr,
    pub shape_identity: SmolStr,
    pub state_version: u32,
    pub accepted: VmAcceptedCheckpoint,
    pub prev_discontinuity: bool,
}

impl VerilogADeviceCheckpoint {
    /// Exact, endian-independent word representation used by the outer
    /// portable checkpoint format. Floating-point values are stored by IEEE
    /// bits, preserving signed zero and every accepted finite value exactly.
    pub fn to_words(&self) -> Vec<u64> {
        self.to_words_with_format(self.state_version, true, true)
    }

    fn to_words_with_format(
        &self,
        state_version: u32,
        include_slew_corners: bool,
        include_transition_queue: bool,
    ) -> Vec<u64> {
        let mut encoder = CheckpointWordEncoder::default();
        encoder.word(u64::from(state_version));
        encoder.boolean(self.prev_discontinuity);
        encoder.float(self.accepted.time);
        encoder.floats(&self.accepted.variables);
        encoder.floats(&self.accepted.state_values_prev);
        encoder.floats(&self.accepted.state_values_older);
        encoder.floats(&self.accepted.state_derivatives_prev);
        encoder.booleans(&self.accepted.state_initialized);
        encoder.word(self.accepted.delay_buffers.len() as u64);
        for delay in &self.accepted.delay_buffers {
            if state_version >= 6 {
                match delay.configuration {
                    None => encoder.word(0),
                    Some(crate::vm::DelayConfiguration::Fixed { delay }) => {
                        encoder.word(1);
                        encoder.float(delay);
                    }
                    Some(crate::vm::DelayConfiguration::Bounded { max_delay }) => {
                        encoder.word(2);
                        encoder.float(max_delay);
                    }
                }
            }
            encoder.word(delay.samples.len() as u64);
            for &(time, value) in &delay.samples {
                encoder.float(time);
                encoder.float(value);
            }
        }
        encoder.word(self.accepted.transition_filters.len() as u64);
        for state in &self.accepted.transition_filters {
            if include_transition_queue {
                encoder.boolean(state.initialized);
                encoder.float(state.input);
                encoder.float(state.output);
                encoder.float(state.time);
                encoder.boolean(state.active.is_some());
                if let Some(active) = state.active {
                    encoder.float(active.origin_time);
                    encoder.float(active.origin_value);
                    encoder.float(active.destination);
                    encoder.float(active.end_time);
                }
                encoder.word(state.pending.len() as u64);
                for pending in &state.pending {
                    encoder.float(pending.start_time);
                    encoder.float(pending.destination);
                    encoder.float(pending.rise_time);
                    encoder.float(pending.fall_time);
                }
            } else {
                let active = state.active;
                encoder.float(state.output);
                encoder.float(active.map_or(state.input, |segment| segment.destination));
                encoder.float(active.map_or(state.time, |segment| segment.origin_time));
                encoder.float(active.map_or(state.time, |segment| segment.end_time));
                encoder.float(active.map_or(state.output, |segment| segment.origin_value));
            }
        }
        encoder.word(self.accepted.slew_filters.len() as u64);
        for state in &self.accepted.slew_filters {
            encoder.float(state.output);
            encoder.float(state.prev_time);
            if include_slew_corners {
                encoder.optional_float(state.next_corner_time);
            }
            encoder.boolean(state.initialized);
        }
        encoder.word(self.accepted.cross_detectors.len() as u64);
        for state in &self.accepted.cross_detectors {
            encoder.float(state.value);
            encoder.float(state.time);
            encoder.word(match state.side {
                -1 => 0,
                0 => 1,
                1 => 2,
                _ => u64::MAX,
            });
            encoder.float(state.last_event_time);
            encoder.float(state.last_crossing_time);
            encoder.boolean(state.initialized);
        }
        encoder.word(self.accepted.laplace_filters.len() as u64);
        for state in &self.accepted.laplace_filters {
            encoder.floats(&state.state);
            if state_version >= 7 {
                encoder.floats(&state.older_state);
                encoder.floats(&state.derivative);
            }
        }
        encoder.word(self.accepted.zi_filters.len() as u64);
        for state in &self.accepted.zi_filters {
            encoder.boolean(state.definition_frozen);
            encoder.floats(&state.num);
            encoder.floats(&state.den);
            encoder.float(state.period);
            encoder.float(state.first_transition);
            encoder.floats(&state.x_hist);
            encoder.floats(&state.y_hist);
            encoder.float(state.held);
            encoder.float(state.ramp_start_time);
            encoder.float(state.ramp_end_time);
            encoder.float(state.ramp_start_value);
            encoder.word(state.next_sample_index);
            encoder.optional_float(state.accepted_time);
            encoder.boolean(state.transient_active);
            encoder.boolean(state.transient_seen);
        }
        encoder.optional_float(self.accepted.timer_event_bound);
        encoder.words
    }

    pub fn from_words(
        instance_name: SmolStr,
        model_name: SmolStr,
        source_digest: SmolStr,
        shape_identity: SmolStr,
        words: &[u64],
    ) -> Result<Self, String> {
        Self::from_words_with_expected_version(
            instance_name,
            model_name,
            source_digest,
            shape_identity,
            words,
            RUNTIME_CHECKPOINT_STATE_VERSION,
        )
    }

    fn from_words_with_expected_version(
        instance_name: SmolStr,
        model_name: SmolStr,
        source_digest: SmolStr,
        shape_identity: SmolStr,
        words: &[u64],
        expected_version: u32,
    ) -> Result<Self, String> {
        use crate::laplace::LaplaceCheckpoint;
        use crate::vm::{
            CrossCheckpoint, DelayBuffer, DelayCheckpoint, DelayConfiguration,
            PendingTransitionCheckpoint, SlewCheckpoint, TransitionCheckpoint,
            TransitionSegmentCheckpoint,
        };
        use crate::zfilter::ZiCheckpoint;

        let mut decoder = CheckpointWordDecoder::new(words);
        let state_version = decoder.u32("state version")?;
        if state_version != expected_version {
            return Err(format!(
                "unsupported runtime Verilog-A state version {state_version}; expected version {expected_version}"
            ));
        }
        let prev_discontinuity = decoder.boolean("previous discontinuity")?;
        let time = decoder.float("accepted time")?;
        let variables = decoder.floats("variables")?;
        let state_values_prev = decoder.floats("previous state values")?;
        let state_values_older = decoder.floats("older state values")?;
        let state_derivatives_prev = decoder.floats("previous state derivatives")?;
        let state_initialized = decoder.booleans("state initialization flags")?;

        let delay_count = decoder.length("delay buffers", 1)?;
        let mut delay_buffers = Vec::with_capacity(delay_count);
        for index in 0..delay_count {
            let configuration = if state_version >= 6 {
                match decoder.word(&format!("delay {index} configuration"))? {
                    0 => None,
                    1 => Some(DelayConfiguration::Fixed {
                        delay: decoder.float(&format!("delay {index} fixed delay"))?,
                    }),
                    2 => Some(DelayConfiguration::Bounded {
                        max_delay: decoder.float(&format!("delay {index} maximum delay"))?,
                    }),
                    tag => {
                        return Err(format!("delay {index} configuration tag {tag} is invalid"));
                    }
                }
            } else {
                None
            };
            let count = decoder.length(&format!("delay {index} samples"), 2)?;
            let mut samples = Vec::with_capacity(count);
            for sample in 0..count {
                samples.push((
                    decoder.float(&format!("delay {index} sample {sample} time"))?,
                    decoder.float(&format!("delay {index} sample {sample} value"))?,
                ));
            }
            let checkpoint = DelayCheckpoint {
                configuration,
                samples,
            };
            let validation_checkpoint;
            let checkpoint_to_validate = if state_version >= 6 || checkpoint.samples.is_empty() {
                &checkpoint
            } else {
                // Runtime versions 1 through 5 did not serialize the frozen
                // absdelay definition. Validate every byte of their retained
                // history without manufacturing resumable current state.
                validation_checkpoint = DelayCheckpoint {
                    configuration: Some(DelayConfiguration::Fixed {
                        delay: f64::MIN_POSITIVE,
                    }),
                    samples: checkpoint.samples.clone(),
                };
                &validation_checkpoint
            };
            DelayBuffer::validate_checkpoint(checkpoint_to_validate)
                .map_err(|error| format!("delay {index} checkpoint is invalid: {error}"))?;
            delay_buffers.push(checkpoint);
        }

        let transition_has_queue = state_version >= 5;
        let transition_count = decoder.length(
            "transition filters",
            if transition_has_queue { 6 } else { 5 },
        )?;
        let mut transition_filters = Vec::with_capacity(transition_count);
        for index in 0..transition_count {
            if transition_has_queue {
                let initialized = decoder.boolean(&format!("transition {index} initialized"))?;
                let input = decoder.float(&format!("transition {index} input"))?;
                let output = decoder.float(&format!("transition {index} output"))?;
                let time = decoder.float(&format!("transition {index} accepted time"))?;
                let active = if decoder.boolean(&format!("transition {index} active"))? {
                    Some(TransitionSegmentCheckpoint {
                        origin_time: decoder
                            .float(&format!("transition {index} active origin time"))?,
                        origin_value: decoder
                            .float(&format!("transition {index} active origin value"))?,
                        destination: decoder
                            .float(&format!("transition {index} active destination"))?,
                        end_time: decoder.float(&format!("transition {index} active end time"))?,
                    })
                } else {
                    None
                };
                let pending_count = decoder.length(&format!("transition {index} pending"), 4)?;
                let mut pending = Vec::with_capacity(pending_count);
                for pending_index in 0..pending_count {
                    pending.push(PendingTransitionCheckpoint {
                        start_time: decoder.float(&format!(
                            "transition {index} pending {pending_index} start time"
                        ))?,
                        destination: decoder.float(&format!(
                            "transition {index} pending {pending_index} destination"
                        ))?,
                        rise_time: decoder.float(&format!(
                            "transition {index} pending {pending_index} rise time"
                        ))?,
                        fall_time: decoder.float(&format!(
                            "transition {index} pending {pending_index} fall time"
                        ))?,
                    });
                }
                transition_filters.push(TransitionCheckpoint {
                    input,
                    output,
                    time,
                    active,
                    pending,
                    initialized,
                });
            } else {
                let output = decoder.float(&format!("transition {index} output"))?;
                let target = decoder.float(&format!("transition {index} target"))?;
                let start_time = decoder.float(&format!("transition {index} start time"))?;
                let end_time = decoder.float(&format!("transition {index} end time"))?;
                let start_value = decoder.float(&format!("transition {index} start value"))?;
                transition_filters.push(TransitionCheckpoint {
                    input: target,
                    output,
                    time: start_time,
                    active: (end_time > start_time).then_some(TransitionSegmentCheckpoint {
                        origin_time: start_time,
                        origin_value: start_value,
                        destination: target,
                        end_time,
                    }),
                    pending: Vec::new(),
                    initialized: true,
                });
            }
        }

        let slew_has_corners = state_version >= 4;
        let slew_count = decoder.length("slew filters", if slew_has_corners { 4 } else { 3 })?;
        let mut slew_filters = Vec::with_capacity(slew_count);
        for index in 0..slew_count {
            slew_filters.push(SlewCheckpoint {
                output: decoder.float(&format!("slew {index} output"))?,
                prev_time: decoder.float(&format!("slew {index} previous time"))?,
                next_corner_time: if slew_has_corners {
                    decoder.optional_float(&format!("slew {index} catch-up corner"))?
                } else {
                    None
                },
                initialized: decoder.boolean(&format!("slew {index} initialized"))?,
            });
        }

        let cross_count = decoder.length("cross detectors", 6)?;
        let mut cross_detectors = Vec::with_capacity(cross_count);
        for index in 0..cross_count {
            cross_detectors.push(CrossCheckpoint {
                value: decoder.float(&format!("cross {index} value"))?,
                time: decoder.float(&format!("cross {index} time"))?,
                side: match decoder.word(&format!("cross {index} side"))? {
                    0 => -1,
                    1 => 0,
                    2 => 1,
                    value => return Err(format!("cross {index} side tag {value} is invalid")),
                },
                last_event_time: decoder.float(&format!("cross {index} last event"))?,
                last_crossing_time: decoder.float(&format!("cross {index} last crossing"))?,
                initialized: decoder.boolean(&format!("cross {index} initialized"))?,
            });
        }

        let laplace_has_exact_integration_history = state_version >= 7;
        let laplace_count = decoder.length(
            "Laplace filters",
            if laplace_has_exact_integration_history {
                3
            } else {
                1
            },
        )?;
        let mut laplace_filters = Vec::with_capacity(laplace_count);
        for index in 0..laplace_count {
            let state = decoder.floats(&format!("Laplace filter {index} state"))?;
            let older_state = if laplace_has_exact_integration_history {
                decoder.floats(&format!("Laplace filter {index} older state"))?
            } else {
                Vec::new()
            };
            let derivative = if laplace_has_exact_integration_history {
                decoder.floats(&format!("Laplace filter {index} derivative"))?
            } else {
                Vec::new()
            };
            if laplace_has_exact_integration_history
                && (older_state.len() != state.len() || derivative.len() != state.len())
            {
                return Err(format!(
                    "Laplace filter {index} history lengths are {}/{}/{}, expected three equal lanes",
                    state.len(),
                    older_state.len(),
                    derivative.len()
                ));
            }
            if state
                .iter()
                .chain(&older_state)
                .chain(&derivative)
                .any(|value| !value.is_finite())
            {
                return Err(format!(
                    "Laplace filter {index} state contains a non-finite value"
                ));
            }
            laplace_filters.push(LaplaceCheckpoint {
                state,
                older_state,
                derivative,
            });
        }

        let zi_count = decoder.length("Zi filters", 12)?;
        let mut zi_filters = Vec::with_capacity(zi_count);
        for index in 0..zi_count {
            zi_filters.push(ZiCheckpoint {
                definition_frozen: decoder.boolean(&format!("Zi filter {index} frozen"))?,
                num: decoder.floats(&format!("Zi filter {index} numerator"))?,
                den: decoder.floats(&format!("Zi filter {index} denominator"))?,
                period: decoder.float(&format!("Zi filter {index} period"))?,
                first_transition: decoder.float(&format!("Zi filter {index} first transition"))?,
                x_hist: decoder.floats(&format!("Zi filter {index} input history"))?,
                y_hist: decoder.floats(&format!("Zi filter {index} output history"))?,
                held: decoder.float(&format!("Zi filter {index} held output"))?,
                ramp_start_time: decoder.float(&format!("Zi filter {index} ramp start time"))?,
                ramp_end_time: decoder.float(&format!("Zi filter {index} ramp end time"))?,
                ramp_start_value: decoder.float(&format!("Zi filter {index} ramp start value"))?,
                next_sample_index: decoder.word(&format!("Zi filter {index} sample index"))?,
                accepted_time: decoder
                    .optional_float(&format!("Zi filter {index} accepted time"))?,
                transient_active: decoder.boolean(&format!("Zi filter {index} active"))?,
                transient_seen: decoder.boolean(&format!("Zi filter {index} seen"))?,
            });
        }
        let timer_event_bound = decoder.optional_float("timer event bound")?;
        decoder.finish()?;
        Ok(Self {
            instance_name,
            model_name,
            source_digest,
            shape_identity,
            state_version,
            accepted: VmAcceptedCheckpoint {
                time,
                variables,
                state_values_prev,
                state_values_older,
                state_derivatives_prev,
                state_initialized,
                delay_buffers,
                transition_filters,
                slew_filters,
                cross_detectors,
                laplace_filters,
                zi_filters,
                timer_event_bound,
            },
            prev_discontinuity,
        })
    }

    /// Validate and consume a complete legacy version-1 runtime payload.
    ///
    /// Version 1 serialized each slew checkpoint as only `(output,
    /// previous_time)`. It carried no initialization bit, so this method
    /// deliberately returns no checkpoint and performs no migration. Callers
    /// may use successful validation only to distinguish a well-formed legacy
    /// payload from corrupt data before reporting that it cannot be resumed by
    /// the current runtime.
    pub fn validate_legacy_v1_words(words: &[u64]) -> Result<(), String> {
        let mut decoder = CheckpointWordDecoder::new(words);
        let state_version = decoder.u32("state version")?;
        if state_version != 1 {
            return Err(format!(
                "legacy runtime Verilog-A payload requires state version 1, got {state_version}"
            ));
        }
        decoder.boolean("previous discontinuity")?;
        decoder.float("accepted time")?;
        decoder.floats("variables")?;
        decoder.floats("previous state values")?;
        decoder.floats("older state values")?;
        decoder.floats("previous state derivatives")?;
        decoder.booleans("state initialization flags")?;

        let delay_count = decoder.length("delay buffers", 1)?;
        for index in 0..delay_count {
            let count = decoder.length(&format!("delay {index} samples"), 2)?;
            for sample in 0..count {
                decoder.float(&format!("delay {index} sample {sample} time"))?;
                decoder.float(&format!("delay {index} sample {sample} value"))?;
            }
        }

        let transition_count = decoder.length("transition filters", 5)?;
        for index in 0..transition_count {
            decoder.float(&format!("transition {index} output"))?;
            decoder.float(&format!("transition {index} target"))?;
            decoder.float(&format!("transition {index} start time"))?;
            decoder.float(&format!("transition {index} end time"))?;
            decoder.float(&format!("transition {index} start value"))?;
        }

        let slew_count = decoder.length("slew filters", 2)?;
        for index in 0..slew_count {
            decoder.float(&format!("slew {index} output"))?;
            decoder.float(&format!("slew {index} previous time"))?;
        }

        let cross_count = decoder.length("cross detectors", 6)?;
        for index in 0..cross_count {
            decoder.float(&format!("cross {index} value"))?;
            decoder.float(&format!("cross {index} time"))?;
            match decoder.word(&format!("cross {index} side"))? {
                0..=2 => {}
                value => return Err(format!("cross {index} side tag {value} is invalid")),
            }
            decoder.float(&format!("cross {index} last event"))?;
            decoder.float(&format!("cross {index} last crossing"))?;
            decoder.boolean(&format!("cross {index} initialized"))?;
        }

        let laplace_count = decoder.length("Laplace filters", 1)?;
        for index in 0..laplace_count {
            decoder.floats(&format!("Laplace filter {index} state"))?;
        }

        let zi_count = decoder.length("Zi filters", 12)?;
        for index in 0..zi_count {
            decoder.boolean(&format!("Zi filter {index} frozen"))?;
            decoder.floats(&format!("Zi filter {index} numerator"))?;
            decoder.floats(&format!("Zi filter {index} denominator"))?;
            decoder.float(&format!("Zi filter {index} period"))?;
            decoder.float(&format!("Zi filter {index} first transition"))?;
            decoder.floats(&format!("Zi filter {index} input history"))?;
            decoder.floats(&format!("Zi filter {index} output history"))?;
            decoder.float(&format!("Zi filter {index} held output"))?;
            decoder.float(&format!("Zi filter {index} ramp start time"))?;
            decoder.float(&format!("Zi filter {index} ramp end time"))?;
            decoder.float(&format!("Zi filter {index} ramp start value"))?;
            decoder.word(&format!("Zi filter {index} sample index"))?;
            decoder.optional_float(&format!("Zi filter {index} accepted time"))?;
            decoder.boolean(&format!("Zi filter {index} active"))?;
            decoder.boolean(&format!("Zi filter {index} seen"))?;
        }
        decoder.optional_float("timer event bound")?;
        decoder.finish()
    }

    /// Validate and consume a complete legacy version-2 runtime payload.
    ///
    /// Version 2 serialized `idtmod` accepted lanes without recording whether
    /// the older lane shared the previous lane's modulo branch. Its word shape
    /// otherwise matches the current payload, so decode it strictly but never
    /// expose the result as restart-authoritative current state.
    pub fn validate_legacy_v2_words(words: &[u64]) -> Result<(), String> {
        Self::from_words_with_expected_version(
            SmolStr::new_inline("legacy"),
            SmolStr::new_inline("legacy"),
            SmolStr::new_inline("legacy"),
            SmolStr::new_inline("legacy"),
            words,
            2,
        )
        .map(drop)
    }

    /// Validate and consume a complete legacy version-3 runtime payload.
    /// Version 3 predates persisted exact `slew` catch-up corners, so it can
    /// be inspected for corruption but cannot be resumed exactly.
    pub fn validate_legacy_v3_words(words: &[u64]) -> Result<(), String> {
        Self::from_words_with_expected_version(
            SmolStr::new_inline("legacy"),
            SmolStr::new_inline("legacy"),
            SmolStr::new_inline("legacy"),
            SmolStr::new_inline("legacy"),
            words,
            3,
        )
        .map(drop)
    }

    /// Validate and consume a complete legacy version-4 runtime payload.
    /// Version 4 retained only one transition target and therefore cannot
    /// reconstruct the arbitrary pending queue or interruption origin needed
    /// for an exact resume.
    pub fn validate_legacy_v4_words(words: &[u64]) -> Result<(), String> {
        Self::from_words_with_expected_version(
            SmolStr::new_inline("legacy"),
            SmolStr::new_inline("legacy"),
            SmolStr::new_inline("legacy"),
            SmolStr::new_inline("legacy"),
            words,
            4,
        )
        .map(drop)
    }

    /// Validate and consume a complete legacy version-5 runtime payload.
    /// Version 5 persisted the accepted `absdelay` samples but not the frozen
    /// fixed/bounded delay definition. Its retained history remains fully
    /// parseable for corruption diagnostics, but it cannot be promoted into
    /// restart-authoritative current state without inventing configuration.
    pub fn validate_legacy_v5_words(words: &[u64]) -> Result<(), String> {
        Self::from_words_with_expected_version(
            SmolStr::new_inline("legacy"),
            SmolStr::new_inline("legacy"),
            SmolStr::new_inline("legacy"),
            SmolStr::new_inline("legacy"),
            words,
            5,
        )
        .map(drop)
    }

    /// Validate and consume a complete legacy version-6 runtime payload.
    /// Version 6 retained only the most recent accepted Laplace state. It did
    /// not preserve the older state or previous physical derivative required
    /// by Gear-2 and trapezoidal integration, so it cannot be resumed exactly.
    pub fn validate_legacy_v6_words(words: &[u64]) -> Result<(), String> {
        Self::from_words_with_expected_version(
            SmolStr::new_inline("legacy"),
            SmolStr::new_inline("legacy"),
            SmolStr::new_inline("legacy"),
            SmolStr::new_inline("legacy"),
            words,
            6,
        )
        .map(drop)
    }

    #[cfg(test)]
    fn to_legacy_v2_words_for_test(&self) -> Vec<u64> {
        self.to_words_with_format(2, false, false)
    }

    #[cfg(test)]
    fn to_legacy_v3_words_for_test(&self) -> Vec<u64> {
        self.to_words_with_format(3, false, false)
    }

    #[cfg(test)]
    fn to_legacy_v4_words_for_test(&self) -> Vec<u64> {
        self.to_words_with_format(4, true, false)
    }

    #[cfg(test)]
    fn to_legacy_v5_words_for_test(&self) -> Vec<u64> {
        self.to_words_with_format(5, true, true)
    }

    #[cfg(test)]
    fn to_legacy_v6_words_for_test(&self) -> Vec<u64> {
        self.to_words_with_format(6, true, true)
    }

    pub fn retained_value_count(&self) -> usize {
        self.to_words().len()
    }
}

#[derive(Default)]
struct CheckpointWordEncoder {
    words: Vec<u64>,
}

impl CheckpointWordEncoder {
    fn word(&mut self, value: u64) {
        self.words.push(value);
    }
    fn boolean(&mut self, value: bool) {
        self.word(u64::from(value));
    }
    fn float(&mut self, value: f64) {
        self.word(value.to_bits());
    }
    fn floats(&mut self, values: &[f64]) {
        self.word(values.len() as u64);
        self.words
            .extend(values.iter().map(|value| value.to_bits()));
    }
    fn booleans(&mut self, values: &[bool]) {
        self.word(values.len() as u64);
        self.words
            .extend(values.iter().map(|value| u64::from(*value)));
    }
    fn optional_float(&mut self, value: Option<f64>) {
        self.boolean(value.is_some());
        if let Some(value) = value {
            self.float(value);
        }
    }
}

struct CheckpointWordDecoder<'a> {
    words: &'a [u64],
    cursor: usize,
}

impl<'a> CheckpointWordDecoder<'a> {
    fn new(words: &'a [u64]) -> Self {
        Self { words, cursor: 0 }
    }
    fn remaining(&self) -> usize {
        self.words.len().saturating_sub(self.cursor)
    }
    fn word(&mut self, name: &str) -> Result<u64, String> {
        let value = self
            .words
            .get(self.cursor)
            .copied()
            .ok_or_else(|| format!("runtime Verilog-A payload ended before {name}"))?;
        self.cursor += 1;
        Ok(value)
    }
    fn u32(&mut self, name: &str) -> Result<u32, String> {
        u32::try_from(self.word(name)?).map_err(|_| format!("{name} exceeds u32"))
    }
    fn boolean(&mut self, name: &str) -> Result<bool, String> {
        match self.word(name)? {
            0 => Ok(false),
            1 => Ok(true),
            value => Err(format!("{name} boolean tag {value} is invalid")),
        }
    }
    fn float(&mut self, name: &str) -> Result<f64, String> {
        Ok(f64::from_bits(self.word(name)?))
    }
    fn length(&mut self, name: &str, words_per_entry: usize) -> Result<usize, String> {
        let length = usize::try_from(self.word(&format!("{name} count"))?)
            .map_err(|_| format!("{name} count exceeds this platform"))?;
        let required = length
            .checked_mul(words_per_entry)
            .ok_or_else(|| format!("{name} size overflows"))?;
        if required > self.remaining() {
            return Err(format!(
                "{name} declares {length} entries but only {} payload words remain",
                self.remaining()
            ));
        }
        Ok(length)
    }
    fn floats(&mut self, name: &str) -> Result<Vec<f64>, String> {
        let length = self.length(name, 1)?;
        let mut values = Vec::with_capacity(length);
        for index in 0..length {
            values.push(self.float(&format!("{name} value {index}"))?);
        }
        Ok(values)
    }
    fn booleans(&mut self, name: &str) -> Result<Vec<bool>, String> {
        let length = self.length(name, 1)?;
        let mut values = Vec::with_capacity(length);
        for index in 0..length {
            values.push(self.boolean(&format!("{name} value {index}"))?);
        }
        Ok(values)
    }
    fn optional_float(&mut self, name: &str) -> Result<Option<f64>, String> {
        if self.boolean(&format!("{name} present"))? {
            Ok(Some(self.float(name)?))
        } else {
            Ok(None)
        }
    }
    fn finish(self) -> Result<(), String> {
        if self.cursor == self.words.len() {
            Ok(())
        } else {
            Err(format!(
                "runtime Verilog-A payload has {} trailing words",
                self.words.len() - self.cursor
            ))
        }
    }
}

/// Pre-computed matrix indices for fast stamping
#[derive(Debug, Clone, Default)]
pub struct MatrixIndices {
    /// Jacobian mappings grouped per stamp program.
    pub jacobian: Vec<Vec<JacobianIndex>>,
    /// Reactive (charge) Jacobian mappings grouped per stamp program.
    pub reactive: Vec<Vec<JacobianIndex>>,
    /// RHS mappings grouped per stamp program.
    pub rhs: Vec<Vec<RhsIndex>>,
}

/// Single Jacobian matrix entry index
#[derive(Debug, Clone)]
pub struct JacobianIndex {
    /// Row in circuit matrix (None = ground)
    pub row: Option<usize>,
    /// Column in circuit matrix (None = ground)
    pub col: Option<usize>,
    /// Index into stamp programs
    pub program_idx: usize,
    /// Index into Jacobian programs
    pub jacobian_idx: usize,
    /// Sign multiplier
    pub sign: f64,
}

/// Single RHS vector entry index
#[derive(Debug, Clone)]
pub struct RhsIndex {
    /// Node in circuit (None = ground)
    pub node: Option<usize>,
    /// Sign multiplier
    pub sign: f64,
    /// Index into stamp programs
    pub program_idx: usize,
}

#[derive(Debug, Clone)]
struct CanonicalNoiseProcessValues {
    process_id: usize,
    active: usize,
    psd: usize,
    exponent: Option<usize>,
    table: Vec<usize>,
}

#[derive(Debug, Clone)]
struct CanonicalNoiseRuntimePlan {
    function: crate::canonical_ir::CfgFunction,
    outputs: Vec<crate::canonical_ir::ValueId>,
    processes: Vec<CanonicalNoiseProcessValues>,
    branch_endpoints: Vec<(Option<usize>, Option<usize>)>,
    used_branch_flows: std::collections::HashSet<usize>,
}

#[derive(Debug, Clone)]
struct EvaluatedCanonicalNoiseProcess {
    process_id: usize,
    active: bool,
    psd: f64,
    exponent: Option<f64>,
}

impl CanonicalNoiseRuntimePlan {
    fn build(artifact: &CanonicalIrArtifact, model: &CompiledModel) -> Result<Self, VmError> {
        crate::canonical_compat::validate_canonical_artifact_identity_for_model(model, artifact)
            .map_err(|detail| {
                VmError::InvalidModel(format!(
                    "canonical grouped-noise artifact/model mismatch: {detail}"
                ))
            })?;
        Self::validate_compiled_injections(artifact, model)?;
        if artifact.hir.module_name != model.name
            || artifact.hir.ports.len() != model.num_terminals
            || artifact
                .hir
                .ports
                .iter()
                .zip(&model.terminal_names)
                .any(|(canonical, compiled)| canonical.name != *compiled)
        {
            return Err(VmError::InvalidModel(
                "canonical grouped-noise HIR terminal layout does not match compiled model".into(),
            ));
        }
        let cfg =
            crate::canonical_ir::CfgModel::noise_metadata_from_hir(&artifact.hir, &artifact.mir)
                .map_err(|diagnostics| {
                    VmError::InvalidModel(format!(
                        "canonical grouped-noise CFG lowering failed: {diagnostics:?}"
                    ))
                })?;
        if cfg.noise_processes.len() != model.noise_sources.len() {
            return Err(VmError::InvalidModel(format!(
                "canonical grouped-noise process count {} does not match compiled count {}",
                cfg.noise_processes.len(),
                model.noise_sources.len()
            )));
        }

        let mut wanted = Vec::new();
        let mut processes = Vec::with_capacity(cfg.noise_processes.len());
        for (expected, (lowered, compiled)) in cfg
            .noise_processes
            .iter()
            .zip(&model.noise_sources)
            .enumerate()
        {
            if lowered.process_id as usize != expected || compiled.process_id != expected {
                return Err(VmError::InvalidModel(format!(
                    "grouped-noise process IDs must be dense and identical at index {expected}: canonical={}, compiled={}",
                    lowered.process_id, compiled.process_id
                )));
            }
            let compiled_kind = if compiled.table.is_some() {
                crate::canonical_ir::CanonicalNoiseSourceKind::Table
            } else if compiled.exponent_program.is_some() {
                crate::canonical_ir::CanonicalNoiseSourceKind::Flicker
            } else {
                crate::canonical_ir::CanonicalNoiseSourceKind::White
            };
            if lowered.kind != compiled_kind
                || lowered.log_interp
                    != compiled
                        .table
                        .as_ref()
                        .is_some_and(|(_, log_interp)| *log_interp)
                || lowered.label != compiled.name
                || lowered.exponent.is_some() != compiled.exponent_program.is_some()
                || lowered.table.len()
                    != compiled
                        .table
                        .as_ref()
                        .map_or(0, |(points, _)| points.len().saturating_mul(2))
            {
                return Err(VmError::InvalidModel(format!(
                    "canonical grouped-noise metadata shape disagrees for process {expected}"
                )));
            }
            let mut place = |value| {
                wanted.push(value);
                wanted.len() - 1
            };
            processes.push(CanonicalNoiseProcessValues {
                process_id: expected,
                active: place(lowered.active),
                psd: place(lowered.psd),
                exponent: lowered.exponent.map(&mut place),
                table: lowered.table.iter().copied().map(place).collect(),
            });
        }
        let (function, outputs, _) =
            crate::canonical_ir::cfg_opt::optimize_with_tracking(&cfg.function, &wanted, &[]);
        for value in &function.values {
            use crate::canonical_ir::CfgValueKind;
            if matches!(
                value.kind,
                CfgValueKind::Ddt { .. }
                    | CfgValueKind::DdtScale
                    | CfgValueKind::Idt { .. }
                    | CfgValueKind::IdtScale
                    | CfgValueKind::Transition { .. }
                    | CfgValueKind::TransitionDerivative { .. }
                    | CfgValueKind::Cross { .. }
                    | CfgValueKind::Above { .. }
                    | CfgValueKind::Timer { .. }
                    | CfgValueKind::Limit { .. }
                    | CfgValueKind::LimitPrevious { .. }
                    | CfgValueKind::Ddx { .. }
            ) {
                return Err(VmError::InvalidModel(
                    "grouped-noise PSD/exponent metadata reaches unsupported dynamic/history state"
                        .into(),
                ));
            }
        }
        let used_branch_flows = function
            .values
            .iter()
            .filter_map(|value| match value.kind {
                crate::canonical_ir::CfgValueKind::BranchFlow(branch) => Some(usize::from(branch)),
                _ => None,
            })
            .collect();
        let branch_endpoints = artifact
            .mir
            .branches
            .iter()
            .map(|branch| {
                (
                    branch.pos_node.map(usize::from),
                    branch.neg_node.map(usize::from),
                )
            })
            .collect();
        Ok(Self {
            function,
            outputs,
            processes,
            branch_endpoints,
            used_branch_flows,
        })
    }

    fn validate_compiled_injections(
        artifact: &CanonicalIrArtifact,
        model: &CompiledModel,
    ) -> Result<(), VmError> {
        fn invalid(detail: impl Into<String>) -> VmError {
            VmError::InvalidModel(format!(
                "canonical grouped-noise artifact/model mismatch: {}",
                detail.into()
            ))
        }

        fn same_index(left: &StampIndex, right: &StampIndex) -> bool {
            match (left, right) {
                (StampIndex::Ground, StampIndex::Ground) => true,
                (StampIndex::Terminal(left), StampIndex::Terminal(right))
                | (StampIndex::Internal(left), StampIndex::Internal(right))
                | (StampIndex::Branch(left), StampIndex::Branch(right)) => left == right,
                _ => false,
            }
        }

        fn current_pair(
            model: &CompiledModel,
            program: &crate::codegen::StampProgram,
            program_idx: usize,
        ) -> Result<(StampIndex, StampIndex), VmError> {
            let mut pos = None;
            let mut neg = None;
            for location in &program.stamp_locations {
                if !matches!(location.col, StampIndex::Ground) {
                    return Err(invalid(format!(
                        "grouped-noise injection program {program_idx} has a non-ground current source column"
                    )));
                }
                let valid_row = match location.row {
                    StampIndex::Terminal(index) => index < model.num_terminals,
                    StampIndex::Internal(index) => index < model.internal_nodes,
                    StampIndex::Ground => true,
                    StampIndex::Branch(_) => false,
                };
                if !valid_row {
                    return Err(invalid(format!(
                        "grouped-noise injection program {program_idx} has an invalid current source row"
                    )));
                }
                if location.sign == -1.0 {
                    if pos.replace(location.row.clone()).is_some() {
                        return Err(invalid(format!(
                            "grouped-noise injection program {program_idx} has duplicate positive current rows"
                        )));
                    }
                } else if location.sign == 1.0 {
                    if neg.replace(location.row.clone()).is_some() {
                        return Err(invalid(format!(
                            "grouped-noise injection program {program_idx} has duplicate negative current rows"
                        )));
                    }
                } else {
                    return Err(invalid(format!(
                        "grouped-noise injection program {program_idx} has invalid current-row sign {}",
                        location.sign
                    )));
                }
            }
            match (pos, neg) {
                (Some(pos), Some(neg)) if !same_index(&pos, &neg) => Ok((pos, neg)),
                (Some(pos), None) if !matches!(&pos, StampIndex::Ground) => {
                    Ok((pos, StampIndex::Ground))
                }
                (None, Some(neg)) if !matches!(&neg, StampIndex::Ground) => {
                    Ok((StampIndex::Ground, neg))
                }
                _ => Err(invalid(format!(
                    "grouped-noise injection program {program_idx} does not identify a valid current branch"
                ))),
            }
        }

        for (process_index, source) in model.noise_sources.iter().enumerate() {
            for (injection_index, injection) in source.injections.iter().enumerate() {
                let program = model.stamp_programs.get(injection.program_idx).ok_or_else(|| {
                    invalid(format!(
                        "noise process {process_index} injection {injection_index} references missing stamp program {}",
                        injection.program_idx
                    ))
                })?;
                let equation = artifact
                    .mir
                    .equations
                    .get(injection.program_idx)
                    .ok_or_else(|| {
                        invalid(format!(
                            "noise process {process_index} injection {injection_index} references missing canonical equation {}",
                            injection.program_idx
                        ))
                    })?;
                if usize::from(equation.id) != injection.program_idx {
                    return Err(invalid(format!(
                        "noise process {process_index} injection {injection_index} references canonical equation {} whose identity is {}",
                        injection.program_idx, equation.id
                    )));
                }

                let expected_kind = if program.indirect {
                    crate::canonical_ir::MirEquationKind::Indirect
                } else if program.branch_ordinal.is_some() {
                    crate::canonical_ir::MirEquationKind::Potential
                } else {
                    crate::canonical_ir::MirEquationKind::Current
                };
                if equation.kind != expected_kind {
                    return Err(invalid(format!(
                        "noise process {process_index} injection {injection_index} equation kind does not match stamp program {}",
                        injection.program_idx
                    )));
                }
                if injection.branch_ordinal != program.branch_ordinal {
                    return Err(invalid(format!(
                        "noise process {process_index} injection {injection_index} branch ordinal does not match stamp program {}",
                        injection.program_idx
                    )));
                }
                let expected_is_current = match expected_kind {
                    crate::canonical_ir::MirEquationKind::Current => Some(true),
                    crate::canonical_ir::MirEquationKind::Potential => Some(false),
                    // Canonical MIR deliberately makes an indirect constraint
                    // orientation-free and does not retain whether its source
                    // access was a flow or potential. Both forms use the same
                    // branch-row noise and multiplicity convention.
                    crate::canonical_ir::MirEquationKind::Indirect => None,
                };
                if expected_is_current.is_some_and(|expected| injection.is_current != expected) {
                    return Err(invalid(format!(
                        "noise process {process_index} injection {injection_index} contribution kind does not match stamp program {}",
                        injection.program_idx
                    )));
                }
                let expected_rhs_sign: f64 = match expected_kind {
                    crate::canonical_ir::MirEquationKind::Potential => 1.0,
                    crate::canonical_ir::MirEquationKind::Current
                    | crate::canonical_ir::MirEquationKind::Indirect => -1.0,
                };
                if injection.rhs_sign.to_bits() != expected_rhs_sign.to_bits() {
                    return Err(invalid(format!(
                        "noise process {process_index} injection {injection_index} RHS sign {} does not match expected {expected_rhs_sign}",
                        injection.rhs_sign
                    )));
                }

                let (expected_pos, expected_neg) = match program.branch_ordinal {
                    Some(ordinal) => {
                        let branch = model.branch_sources.get(ordinal).ok_or_else(|| {
                            invalid(format!(
                                "noise process {process_index} injection {injection_index} references missing branch source {ordinal}"
                            ))
                        })?;
                        if branch.indirect != program.indirect {
                            return Err(invalid(format!(
                                "noise process {process_index} injection {injection_index} branch source kind does not match stamp program {}",
                                injection.program_idx
                            )));
                        }
                        (branch.pos.clone(), branch.neg.clone())
                    }
                    None => current_pair(model, program, injection.program_idx)?,
                };
                if !same_index(&injection.pos, &expected_pos)
                    || !same_index(&injection.neg, &expected_neg)
                {
                    return Err(invalid(format!(
                        "noise process {process_index} injection {injection_index} endpoints do not match stamp program {}",
                        injection.program_idx
                    )));
                }
            }
        }
        Ok(())
    }

    fn evaluate(
        &self,
        context: &VmContext,
    ) -> Result<Vec<EvaluatedCanonicalNoiseProcess>, VmError> {
        let mut node_potentials = context.voltages.clone();
        node_potentials.extend_from_slice(&context.internal_voltages);
        let mut branch_flows = vec![0.0; self.branch_endpoints.len()];
        for &branch in &self.used_branch_flows {
            let Some(&(pos, neg)) = self.branch_endpoints.get(branch) else {
                return Err(VmError::InvalidModel(format!(
                    "canonical grouped-noise branch flow {branch} is outside the branch table"
                )));
            };
            let endpoint = |node: Option<usize>| -> Result<usize, VmError> {
                match node {
                    None => Ok(usize::MAX),
                    Some(node) if node < context.voltages.len() => Ok(node),
                    Some(node) => Err(VmError::InvalidModel(format!(
                        "canonical grouped-noise metadata reads unsupported internal-node branch flow {node}"
                    ))),
                }
            };
            branch_flows[branch] = context.try_current(endpoint(pos)?, endpoint(neg)?)?;
        }
        let mut analyses = std::collections::HashSet::new();
        for analysis in ["noise", "smallsig", "smallsignal", "small_signal"] {
            analyses.insert(SmolStr::new(analysis));
        }
        if context.analysis_initial_step {
            analyses.insert(SmolStr::new("__rspice_initial_step"));
        }
        if context.analysis_final_step {
            analyses.insert(SmolStr::new("__rspice_final_step"));
        }
        let inputs = crate::canonical_ir::CfgEvalInputs {
            parameters: context.parameters.clone(),
            parameter_given: context
                .param_given
                .iter()
                .map(|given| *given != 0)
                .collect(),
            port_connected: context
                .port_connected
                .iter()
                .map(|connected| *connected != 0)
                .collect(),
            event_state: context.accepted_event_variables().to_vec(),
            node_potentials,
            branch_flows,
            branch_unknown_flows: context.branch_current_values.clone(),
            temperature: context.temperature,
            thermal_voltage: context.vt(),
            multiplicity: context.multiplicity,
            time: context.time,
            analyses,
            simparams: Default::default(),
            ddt: 0.0,
            ddt_scale: 0.0,
            idt: 0.0,
            idt_scale: 0.0,
            event_controls: Default::default(),
            staged: Vec::new(),
        };
        let snapshot =
            crate::canonical_ir::evaluate_cfg(&self.function, &inputs).map_err(|error| {
                VmError::InvalidNumericResult(format!(
                    "canonical grouped-noise metadata evaluation failed: {error}"
                ))
            })?;
        let read = |position: usize| -> Result<f64, VmError> {
            let value = self
                .outputs
                .get(position)
                .and_then(|value| snapshot.value(*value))
                .ok_or_else(|| {
                    VmError::InvalidNumericResult(format!(
                        "canonical grouped-noise output {position} was not defined"
                    ))
                })?;
            Ok(value)
        };
        self.processes
            .iter()
            .map(|process| {
                // Table outputs are evaluated for their lazy-path diagnostics
                // even though the compiled artifact owns the validated table.
                for &position in &process.table {
                    let _ = read(position)?;
                }
                let active = read(process.active)?;
                if !active.is_finite() {
                    return Err(VmError::InvalidNumericResult(format!(
                        "canonical grouped-noise activation for process {} is non-finite",
                        process.process_id
                    )));
                }
                Ok(EvaluatedCanonicalNoiseProcess {
                    process_id: process.process_id,
                    active: active != 0.0,
                    psd: read(process.psd)?,
                    exponent: process.exponent.map(read).transpose()?,
                })
            })
            .collect()
    }
}

impl VerilogADevice {
    /// Whether noise must be evaluated through the correlated per-frequency
    /// process API. Schema-1 devices must never enter the eager legacy scalar
    /// PSD path because that would bypass CFG activation/reaching definitions.
    pub fn uses_grouped_noise_processes(&self) -> bool {
        self.model.noise_process_schema >= 1
    }

    /// Whether this instance has at least one structurally routed grouped
    /// process. This is allocation-free and does not evaluate activation or
    /// PSD metadata, so analysis sweeps can skip no-noise instances safely.
    pub fn has_grouped_noise_processes(&self) -> bool {
        self.uses_grouped_noise_processes()
            && self
                .model
                .noise_sources
                .iter()
                .any(|source| !source.injections.is_empty())
    }

    /// Activation-independent catalog of grouped noise processes that can
    /// reach at least one circuit equation. The list is structural: raw PSDs
    /// are not evaluated, so inactive and final-step-only processes remain
    /// discoverable by contribution UIs and saved-result validators.
    pub fn grouped_noise_process_catalog(&self) -> Vec<(usize, String)> {
        if !self.has_grouped_noise_processes() {
            return Vec::new();
        }
        self.model
            .noise_sources
            .iter()
            .filter(|source| !source.injections.is_empty())
            .map(|source| {
                (
                    source.process_id,
                    source
                        .name
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_else(|| format!("noise{}", source.process_id)),
                )
            })
            .collect()
    }

    fn finite_result(value: f64, context: impl Into<String>) -> Result<f64, VmError> {
        if value.is_finite() {
            Ok(value)
        } else {
            Err(VmError::InvalidNumericResult(format!(
                "{} evaluated to {value}",
                context.into()
            )))
        }
    }

    /// Audit one fused-driver result before any solver callback observes it.
    #[cfg(any(feature = "native", all(feature = "wasm-jit", target_arch = "wasm32")))]
    #[inline]
    fn finite_stamp_value(
        value: f64,
        stamp: usize,
        entry: Option<usize>,
        phase: &'static str,
    ) -> Result<f64, VmError> {
        if value.is_finite() {
            return Ok(value);
        }
        let context = match entry {
            Some(entry) => format!("{phase} {stamp}:{entry}"),
            None => format!("{phase} {stamp}"),
        };
        Err(VmError::InvalidNumericResult(format!(
            "{context} evaluated to {value}"
        )))
    }

    fn noise_power(value: f64, source_index: usize) -> Result<f64, VmError> {
        let value = Self::finite_result(value, format!("noise source {source_index} power"))?;
        if value < 0.0 {
            Err(VmError::InvalidNumericResult(format!(
                "noise source {source_index} power evaluated to negative value {value}"
            )))
        } else {
            Ok(value)
        }
    }

    /// Create a new device instance
    ///
    /// # Arguments
    /// * `name` - Instance name (e.g., "D1")
    /// * `model` - Compiled Verilog-A model. Pass an `Arc<CompiledModel>`
    ///   when instantiating a model many times: instances then share the
    ///   program and one JIT compilation.
    /// * `nodes` - Circuit node IDs for each terminal (0 = ground)
    pub fn new(
        name: impl Into<SmolStr>,
        model: impl Into<std::sync::Arc<CompiledModel>>,
        nodes: &[usize],
    ) -> Self {
        Self::try_new(name, model, nodes).unwrap_or_else(|err| {
            panic!("Verilog-A device construction failed: {}", err);
        })
    }

    /// Checked constructor for callers that can surface dependent-parameter
    /// default failures as diagnostics instead of panicking or accepting a
    /// zero default.
    pub fn try_new(
        name: impl Into<SmolStr>,
        model: impl Into<std::sync::Arc<CompiledModel>>,
        nodes: &[usize],
    ) -> Result<Self, VmError> {
        let model: std::sync::Arc<CompiledModel> = model.into();
        Self::try_new_inner(name, model, nodes, None)
    }

    /// Checked constructor that compiles stamp values from canonical MIR when
    /// the native backend is available. Unsupported MIR is a construction
    /// error; the bytecode stamp path is not used as a fallback.
    pub fn try_new_with_canonical_ir(
        name: impl Into<SmolStr>,
        model: impl Into<std::sync::Arc<CompiledModel>>,
        artifact: &CanonicalIrArtifact,
        nodes: &[usize],
    ) -> Result<Self, VmError> {
        let model: std::sync::Arc<CompiledModel> = model.into();
        Self::try_new_inner(name, model, nodes, Some(artifact))
    }

    fn try_new_inner(
        name: impl Into<SmolStr>,
        model: std::sync::Arc<CompiledModel>,
        nodes: &[usize],
        canonical_artifact: Option<&CanonicalIrArtifact>,
    ) -> Result<Self, VmError> {
        Self::validate_compiled_assignment_layout(model.num_variables, &model.assignment_steps)?;
        Self::validate_compiled_assignment_layout(
            model.num_variables,
            &model.noise_assignment_steps,
        )?;
        #[cfg(all(
            not(feature = "native"),
            not(all(feature = "wasm-jit", target_arch = "wasm32"))
        ))]
        if let Some(artifact) = canonical_artifact {
            crate::canonical_compat::validate_canonical_artifact_identity_for_model(
                &model, artifact,
            )
            .map_err(|detail| {
                VmError::InvalidModel(format!("canonical artifact/model mismatch: {detail}"))
            })?;
        }
        #[cfg(all(
            not(feature = "native"),
            not(all(feature = "wasm-jit", target_arch = "wasm32"))
        ))]
        if model.noise_process_schema >= 1 {
            Self::validate_portable_noise_assignment_split(&model)?;
        }

        let num_terminals = model.num_terminals;
        let supplied_terminals = nodes.len().min(num_terminals);

        // Build node mapping
        let mut node_mapping = vec![0; num_terminals];
        for (i, &node) in nodes.iter().enumerate() {
            if i < num_terminals {
                node_mapping[i] = node;
            }
        }

        // Create context with terminal count and internal nodes
        let num_internal_nodes = model.internal_nodes;
        let mut context = VmContext::with_internal_nodes(num_terminals, num_internal_nodes);
        context.port_connected = (0..num_terminals)
            .map(|terminal| u8::from(terminal < supplied_terminals))
            .collect();

        // Initialize parameters to their constant defaults; dependent
        // defaults are resolved after instance parameters are applied
        for (i, param) in model.parameters.iter().enumerate() {
            context.set_param(i, param.default);
        }
        context.param_given = vec![0; model.parameters.len()];
        context.variables.resize(model.num_variables, 0.0);
        context.configure_event_state_variables(&model.event_state_variables)?;
        // Stateful runtime data referenced by the bytecode lives in the
        // per-instance context (the model stays immutable and shared)
        context.lookup_tables = model.lookup_tables.clone();
        context.laplace_filters = model.laplace_filters.clone();
        context.zi_filters = model.zi_filters.clone();
        Self::preallocate_vm_runtime_state(&mut context, &model)?;

        let canonical_noise_plan = if model.noise_process_schema >= 1
            && !model.noise_sources.is_empty()
        {
            let artifact = canonical_artifact.ok_or_else(|| {
                VmError::InvalidModel(
                    "grouped Verilog-A noise requires canonical IR metadata; no legacy fallback is permitted"
                        .into(),
                )
            })?;
            Some(std::sync::Arc::new(CanonicalNoiseRuntimePlan::build(
                artifact, &model,
            )?))
        } else {
            None
        };

        #[cfg(feature = "native")]
        let native_model = match canonical_artifact {
            Some(artifact) => Self::try_native_compile_with_canonical_ir(&model, artifact)?,
            #[cfg(feature = "native-bytecode-contract-tests")]
            None => Self::try_native_compile(&model)?,
            #[cfg(not(feature = "native-bytecode-contract-tests"))]
            None => {
                return Err(Self::missing_canonical_ir_native_error());
            }
        };

        #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
        let wasm_jit_model = match canonical_artifact {
            Some(artifact) => Self::try_wasm_compile_with_canonical_ir(&model, artifact)?,
            None => {
                return Err(VmError::WasmJit(
                    "VerilogADevice::try_new requires canonical IR when browser WASM JIT execution is enabled; use try_new_with_canonical_ir; no interpreter fallback"
                        .to_owned(),
                ));
            }
        };

        let num_branch_unknowns = model.branch_sources.len();
        let num_stamp_programs = model.stamp_programs.len();
        #[cfg(any(feature = "native", all(feature = "wasm-jit", target_arch = "wasm32")))]
        let fused_jacobian_count = model
            .stamp_programs
            .iter()
            .map(|stamp| stamp.jacobian_programs.len())
            .sum();
        let mut device = Self {
            name: name.into(),
            model,
            context,
            node_mapping,
            internal_node_indices: vec![0; num_internal_nodes],
            num_internal_nodes,
            branch_current_indices: vec![0; num_branch_unknowns],
            program_active: vec![true; num_stamp_programs],
            branch_active: vec![true; num_branch_unknowns],
            matrix_indices: MatrixIndices::default(),
            stamp_matrix_buffer: Vec::new(),
            stamp_rhs_buffer: Vec::new(),
            #[cfg(any(feature = "native", all(feature = "wasm-jit", target_arch = "wasm32")))]
            fused_program_active: vec![1; num_stamp_programs],
            #[cfg(any(feature = "native", all(feature = "wasm-jit", target_arch = "wasm32")))]
            fused_stamp_jacobians: vec![0.0; fused_jacobian_count],
            canonical_noise_plan,
            #[cfg(feature = "native")]
            native_model,
            #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
            wasm_jit_model,
            prev_discontinuity: false,
        };
        device.context.branch_current_values = vec![0.0; num_branch_unknowns];
        device.rebuild_matrix_indices();
        device.try_resolve_parameter_defaults()?;
        Ok(device)
    }

    fn validate_compiled_assignment_layout(
        num_variables: usize,
        assignment_steps: &[crate::codegen::AssignmentStep],
    ) -> Result<(), VmError> {
        const MAX_COMPILED_ASSIGNMENT_STEPS: usize = 4_194_304;

        let mut pending = vec![assignment_steps];
        let mut visited = 0_usize;
        while let Some(steps) = pending.pop() {
            for step in steps {
                visited = visited.checked_add(1).ok_or_else(|| {
                    VmError::InvalidModel("assignment-step count overflow".into())
                })?;
                if visited > MAX_COMPILED_ASSIGNMENT_STEPS {
                    return Err(VmError::InvalidModel(format!(
                        "assignment-step count exceeds safety limit {MAX_COMPILED_ASSIGNMENT_STEPS}"
                    )));
                }

                match step {
                    crate::codegen::AssignmentStep::Assign(assignment) => {
                        if assignment.var_index >= num_variables {
                            return Err(VmError::InvalidModel(format!(
                                "assignment target variable {} is outside declared variable storage length {num_variables}",
                                assignment.var_index
                            )));
                        }
                    }
                    crate::codegen::AssignmentStep::AssignIndexed { base, len, .. } => {
                        if *len == 0 {
                            return Err(VmError::InvalidModel(
                                "indexed assignment declares a zero-length variable range".into(),
                            ));
                        }
                        let end = base.checked_add(*len).ok_or_else(|| {
                            VmError::InvalidModel(
                                "indexed assignment variable range overflows address space".into(),
                            )
                        })?;
                        if end > num_variables {
                            return Err(VmError::InvalidModel(format!(
                                "indexed assignment variable range [{base}:{end}) exceeds declared variable storage length {num_variables}"
                            )));
                        }
                    }
                    crate::codegen::AssignmentStep::Loop { body, .. } => pending.push(body),
                }
            }
        }

        Ok(())
    }

    /// Preallocate interpreter runtime state vectors from bytecode instruction IDs.
    ///
    /// This avoids repeated dynamic growth during simulation hot paths and ensures
    /// stateful operators have stable dedicated slots.
    fn preallocate_vm_runtime_state(
        context: &mut VmContext,
        model: &CompiledModel,
    ) -> Result<(), VmError> {
        #[inline]
        fn update_max(max_slot: &mut Option<usize>, idx: usize) {
            *max_slot = Some(max_slot.map_or(idx, |prev| prev.max(idx)));
        }

        #[inline]
        fn required_slot_count(label: &str, max_idx: usize) -> Result<usize, VmError> {
            max_idx.checked_add(1).ok_or_else(|| {
                VmError::NativeJit(format!(
                    "native JIT {label} runtime state slot index {max_idx} cannot be represented; no interpreter fallback"
                ))
            })
        }

        let mut max_state = None;
        let mut max_delay_buffer = None;
        let mut max_transition_filter = None;
        let mut max_slew_filter = None;
        let mut max_cross_detector = None;

        let mut scan_program = |program: &crate::codegen::BytecodeProgram| {
            for instruction in &program.instructions {
                match instruction {
                    Instruction::DdtState(idx)
                    | Instruction::IdtState(idx)
                    | Instruction::IdtModState(idx)
                    | Instruction::LimitState(idx)
                    | Instruction::CanonicalLimitState(idx) => update_max(&mut max_state, *idx),
                    Instruction::AbsDelayState(idx)
                    | Instruction::AbsDelayStateMax(idx)
                    | Instruction::AbsDelayStateDerivative(idx)
                    | Instruction::AbsDelayStateDerivativeMax(idx) => {
                        update_max(&mut max_delay_buffer, *idx)
                    }
                    Instruction::TransitionState(idx)
                    | Instruction::TransitionStateDerivative(idx) => {
                        update_max(&mut max_transition_filter, *idx)
                    }
                    Instruction::SlewState(idx) | Instruction::SlewStateDerivative(idx) => {
                        update_max(&mut max_slew_filter, *idx)
                    }
                    Instruction::CrossState(idx)
                    | Instruction::AboveState(idx)
                    | Instruction::LastCrossingState(idx) => {
                        update_max(&mut max_cross_detector, *idx)
                    }
                    _ => {}
                }
            }
        };

        fn scan_steps(
            steps: &[crate::codegen::AssignmentStep],
            scan_program: &mut impl FnMut(&crate::codegen::BytecodeProgram),
        ) {
            for step in steps {
                match step {
                    crate::codegen::AssignmentStep::Assign(assignment) => {
                        scan_program(&assignment.program);
                    }
                    crate::codegen::AssignmentStep::AssignIndexed { index, value, .. } => {
                        scan_program(index);
                        scan_program(value);
                    }
                    crate::codegen::AssignmentStep::Loop { condition, body } => {
                        scan_program(condition);
                        scan_steps(body, scan_program);
                    }
                }
            }
        }

        for parameter in &model.parameters {
            if let Some(program) = &parameter.default_program {
                scan_program(program);
            }
        }

        scan_steps(&model.assignment_steps, &mut scan_program);
        scan_steps(&model.noise_assignment_steps, &mut scan_program);

        for stamp in &model.stamp_programs {
            if let Some(condition) = &stamp.static_condition {
                scan_program(condition);
            }
            scan_program(&stamp.value_program);
            for jac in &stamp.jacobian_programs {
                scan_program(&jac.program);
            }
            for jac in &stamp.reactive_jacobians {
                scan_program(&jac.program);
            }
        }

        for source in &model.noise_sources {
            scan_program(&source.psd_program);
            if let Some(program) = &source.exponent_program {
                scan_program(program);
            }
            for injection in &source.injections {
                scan_program(&injection.gain_program);
            }
        }

        if let Some(max_idx) = max_state {
            context.allocate_states(required_slot_count("state-value", max_idx)?);
        }
        if let Some(max_idx) = max_delay_buffer {
            context.allocate_delay_buffers(required_slot_count("delay-buffer", max_idx)?);
        }
        if let Some(max_idx) = max_transition_filter {
            context.allocate_transition_filters(required_slot_count("transition-filter", max_idx)?);
        }
        if let Some(max_idx) = max_slew_filter {
            context.allocate_slew_filters(required_slot_count("slew-filter", max_idx)?);
        }
        if let Some(max_idx) = max_cross_detector {
            context.allocate_cross_detectors(required_slot_count("cross-detector", max_idx)?);
        }

        Ok(())
    }

    /// Attempt to compile the model to native code.
    ///
    /// Compilations are shared process-wide per model `Arc`: a thousand
    /// instances of one model compile once. The result (including a
    /// failed attempt) is cached so construction stays O(1) after the
    /// first instance.
    #[cfg(all(feature = "native", not(feature = "native-bytecode-contract-tests")))]
    fn missing_canonical_ir_native_error() -> VmError {
        VmError::NativeJit(
            "VerilogADevice::try_new requires canonical IR when native JIT is enabled; use try_new_with_canonical_ir; no interpreter fallback"
                .to_string(),
        )
    }

    #[cfg(feature = "native")]
    fn native_runtime_error_to_vm(error: crate::native::NativeRuntimeError) -> VmError {
        match error.kind {
            crate::native::NativeRuntimeErrorKind::NativeJit => VmError::NativeJit(error.message),
            crate::native::NativeRuntimeErrorKind::InvalidNumericResult => {
                VmError::InvalidNumericResult(error.message)
            }
        }
    }

    #[cfg(feature = "native-bytecode-contract-tests")]
    fn try_native_compile(
        model: &std::sync::Arc<CompiledModel>,
    ) -> Result<std::sync::Arc<NativeModel>, VmError> {
        let cache_key = NativeCompileCacheKey::Bytecode {
            source_digest: model.source_digest.clone(),
            module: model.name.clone(),
            owner: std::sync::Arc::downgrade(model),
        };
        Self::try_native_compile_cached(model, cache_key, |model| {
            crate::native::compile_native(model)
        })
    }

    #[cfg(feature = "native")]
    fn try_native_compile_with_canonical_ir(
        model: &std::sync::Arc<CompiledModel>,
        artifact: &CanonicalIrArtifact,
    ) -> Result<std::sync::Arc<NativeModel>, VmError> {
        let cache_key = NativeCompileCacheKey::CanonicalMir {
            mir_digest: artifact.mir_digest.clone(),
            source_digest: model.source_digest.clone(),
            module: model.name.clone(),
            layout: compiled_model_layout_identity(model),
        };
        Self::try_native_compile_cached(model, cache_key, |model| {
            crate::native::compile_native_with_canonical_ir(model, artifact)
        })
    }

    #[cfg(feature = "native")]
    fn try_native_compile_cached(
        model: &std::sync::Arc<CompiledModel>,
        cache_key: NativeCompileCacheKey,
        compile: impl FnOnce(&CompiledModel) -> crate::native::JitResult<NativeModel>,
    ) -> Result<std::sync::Arc<NativeModel>, VmError> {
        use std::sync::Mutex;

        static NATIVE_CACHE: Mutex<Option<NativeCompileCache>> = Mutex::new(None);

        // The lock is held across compilation, as it always has been: two
        // threads reaching the same uncached model would otherwise both pay
        // the full compile and both commit an executable image.
        let mut guard = NATIVE_CACHE.lock().unwrap_or_else(|e| e.into_inner());
        let cache = guard.get_or_insert_with(NativeCompileCache::default);
        if let Some(cached) = cache.get(&cache_key) {
            return cached.map_err(VmError::NativeJit);
        }

        #[cfg(all(test, target_arch = "x86_64"))]
        NATIVE_COMPILE_LOG
            .lock()
            .expect("native compile log")
            .push(model.name.to_string());

        let compiled = match compile(model.as_ref()) {
            Ok(native) => {
                log::info!("[JIT] Model '{}' compiled to native code", model.name);
                #[cfg(debug_assertions)]
                eprintln!("[JIT] Model '{}' compiled to native code", model.name);
                Ok(std::sync::Arc::new(native))
            }
            Err(error) => {
                let msg = error.to_string();
                log::warn!(
                    "[JIT] Native compilation failed for '{}': {}",
                    model.name,
                    msg
                );
                #[cfg(debug_assertions)]
                eprintln!(
                    "[JIT] Native compilation failed for '{}': {}",
                    model.name, msg
                );
                Err(msg)
            }
        };
        cache.insert(cache_key, compiled.clone());
        compiled.map_err(VmError::NativeJit)
    }

    /// Browser-side entry-table cache, keyed on the same content identity as
    /// the native cache.
    ///
    /// The entries here are export-name tables, not code: the compiled modules
    /// themselves live in the worker's own bounded registry. The bound is an
    /// entry count matching that registry so the two tiers cannot disagree
    /// about how many models are resident.
    #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
    fn try_wasm_compile_with_canonical_ir(
        model: &std::sync::Arc<CompiledModel>,
        artifact: &CanonicalIrArtifact,
    ) -> Result<std::sync::Arc<WasmJitExecutable>, VmError> {
        use std::cell::RefCell;
        use std::sync::Arc;

        /// Mirrors `WASM_JIT_CACHE_MAX_MODELS` in the browser worker.
        const WASM_CACHE_MAX_MODELS: usize = 64;

        type CacheEntry = (WasmCompileCacheKey, Result<Arc<WasmJitExecutable>, String>);
        thread_local! {
            /// Most-recently-used first.
            static WASM_CACHE: RefCell<Vec<CacheEntry>> = const { RefCell::new(Vec::new()) };
        }

        let cache_key = WasmCompileCacheKey {
            mir_digest: artifact.mir_digest.clone(),
            source_digest: model.source_digest.clone(),
            module: model.name.clone(),
            layout: compiled_model_layout_identity(model),
        };
        WASM_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if let Some(index) = cache.iter().position(|(key, _)| *key == cache_key) {
                let entry = cache.remove(index);
                let compiled = entry.1.clone();
                cache.insert(0, entry);
                return compiled.map_err(VmError::WasmJit);
            }

            let compiled = crate::wasm_jit::compile_model_value_module(model, artifact)
                .and_then(|module| WasmJitExecutable::from_artifact(model, &module))
                .map(Arc::new)
                .map_err(|error| error.to_string());
            cache.insert(0, (cache_key, compiled.clone()));
            cache.truncate(WASM_CACHE_MAX_MODELS);
            compiled.map_err(VmError::WasmJit)
        })
    }

    /// Check if this device is using native compiled code.
    ///
    /// In native-feature builds, construction succeeds only with a complete
    /// native image, so every constructed device reports true.
    #[cfg(feature = "native")]
    pub fn is_using_native(&self) -> bool {
        true
    }

    /// Set the instance multiplicity (`m=` / $mfactor): the device stamps
    /// as m parallel copies. Non-positive values are rejected.
    pub fn set_multiplicity(&mut self, m: f64) {
        self.try_set_multiplicity(m).unwrap_or_else(|err| {
            panic!(
                "Verilog-A device '{}' model '{}' multiplicity update failed: {}",
                self.name, self.model.name, err
            )
        });
    }

    /// Checked multiplicity update. The previous context remains intact when
    /// validation or a dependent static-condition refresh fails.
    pub fn try_set_multiplicity(&mut self, m: f64) -> Result<(), VmError> {
        if !m.is_finite() || m <= 0.0 {
            return Err(VmError::InvalidRuntimeConfiguration(format!(
                "multiplicity must be finite and greater than zero, got {m}"
            )));
        }
        if self.context.multiplicity == m {
            return Ok(());
        }

        let previous = self.context.clone();
        self.context.multiplicity = m;
        if let Err(error) = self.try_refresh_static_conditions() {
            self.context = previous;
            return Err(error);
        }
        Ok(())
    }

    /// Instance multiplicity ($mfactor)
    pub fn multiplicity(&self) -> f64 {
        self.context.multiplicity
    }

    /// Maximum next transient step requested by `$bound_step` or a scheduled
    /// timer, sampled-filter, or slew catch-up event during the latest
    /// evaluation.
    pub fn transient_bound_step(&self) -> Option<f64> {
        self.try_transient_bound_step().unwrap_or_else(|error| {
            panic!(
                "Verilog-A device '{}' model '{}' transient breakpoint failed: {}",
                self.name, self.model.name, error
            )
        })
    }

    /// Checked transient bound for callers that surface event-scheduling
    /// errors instead of panicking.
    pub fn try_transient_bound_step(&self) -> Result<Option<f64>, VmError> {
        let model_bound = match self.variable("$bound_step") {
            None | Some(f64::INFINITY) => None,
            Some(bound) if bound.is_finite() && bound >= 0.0 => Some(bound),
            Some(bound) => {
                return Err(VmError::InvalidNumericResult(format!(
                    "$bound_step request must be finite and non-negative (or the internal +inf sentinel), got {bound}"
                )));
            }
        };
        let event_bound = self
            .context
            .transient_event_time()?
            .map(|target| target - self.context.time)
            .filter(|bound| bound.is_finite() && *bound > 0.0);
        Ok([model_bound, event_bound]
            .into_iter()
            .flatten()
            .reduce(f64::min))
    }

    /// Earliest exact absolute runtime event owned by accepted operator state.
    pub fn try_transient_event_time(&self) -> Result<Option<f64>, VmError> {
        self.context.transient_event_time()
    }

    /// Earliest interior `cross`/`above` root requested by the latest complete
    /// transient evaluation. Unlike a next-step bound, this asks the solver to
    /// reject the current endpoint and retry inside the attempted interval.
    pub fn try_transient_event_refinement_time(&self) -> Result<Option<f64>, VmError> {
        if self.context.analysis_type != 2 {
            return Ok(None);
        }
        self.context.cross_event_refinement_time()
    }

    /// Whether `$discontinuity` fired during the latest evaluation
    pub fn discontinuity_pending(&self) -> bool {
        self.variable("$discontinuity").is_some_and(|v| v != 0.0)
    }

    /// Number of native assignment chunks the JIT produced for this model
    #[cfg(feature = "native")]
    pub fn native_chunk_count(&self) -> usize {
        self.native_model.chunk_count()
    }

    /// Native entry-point composition diagnostics.
    #[cfg(feature = "native")]
    pub fn native_plan_stats(&self) -> crate::native::PlanStats {
        self.native_model.plan_stats()
    }

    /// Size of this model's shared immutable native executable image.
    #[cfg(feature = "native")]
    pub fn native_code_size_bytes(&self) -> usize {
        self.native_model.code_size_bytes()
    }

    /// Check if this device is using native compiled code
    #[cfg(not(feature = "native"))]
    pub fn is_using_native(&self) -> bool {
        false
    }

    /// Get the number of terminals
    pub fn num_terminals(&self) -> usize {
        self.model.num_terminals
    }

    /// Get the number of internal nodes
    pub fn num_internal_nodes(&self) -> usize {
        self.num_internal_nodes
    }

    /// Get terminal names
    pub fn terminal_names(&self) -> &[SmolStr] {
        &self.model.terminal_names
    }

    /// Get the circuit node for a terminal
    pub fn node_for_terminal(&self, terminal: usize) -> usize {
        self.node_mapping.get(terminal).copied().unwrap_or(0)
    }

    /// Set a parameter value by name
    ///
    /// aliasparam names resolve to their target parameter, so setting an
    /// alias is identical to setting the target directly.
    pub fn set_parameter(&mut self, name: &str, value: f64) -> bool {
        match self.try_set_parameter(name, value) {
            Ok(found) => found,
            Err(error) => {
                log::error!(
                    "Verilog-A instance '{}' rejected parameter assignment: {}",
                    self.name,
                    error
                );
                false
            }
        }
    }

    /// Checked parameter assignment. `Ok(false)` means the name is not a
    /// model parameter; malformed values and scalar constraint violations are
    /// errors. Call [`Self::try_resolve_parameter_defaults`] after applying all
    /// instance assignments to validate constraints that reference parameters.
    pub fn try_set_parameter(
        &mut self,
        name: &str,
        value: f64,
    ) -> Result<bool, ParameterValueError> {
        // Verilog-A is case-sensitive but SPICE decks are not: prefer an
        // exact match (parameter, then alias), then accept a
        // case-insensitive one (industry netlists write PSP's TOXO as
        // toxo). Aliases cannot collide with parameter names, so the
        // ordering only arbitrates between case-insensitive candidates.
        let params = &self.model.parameters;
        let index = params
            .iter()
            .enumerate()
            .filter(|(_, p)| p.is_public)
            .find_map(|(index, p)| (p.name == name).then_some(index))
            .or_else(|| {
                params
                    .iter()
                    .enumerate()
                    .filter(|(_, p)| p.is_public)
                    .find_map(|(index, p)| {
                        p.aliases
                            .iter()
                            .any(|a| a.as_str() == name)
                            .then_some(index)
                    })
            })
            .or_else(|| {
                params
                    .iter()
                    .enumerate()
                    .filter(|(_, p)| p.is_public)
                    .find_map(|(index, p)| p.name.eq_ignore_ascii_case(name).then_some(index))
            })
            .or_else(|| {
                params
                    .iter()
                    .enumerate()
                    .filter(|(_, p)| p.is_public)
                    .find_map(|(index, p)| {
                        p.aliases
                            .iter()
                            .any(|a| a.eq_ignore_ascii_case(name))
                            .then_some(index)
                    })
            });
        let Some(i) = index else { return Ok(false) };
        // Cross-parameter constraints are checked after all instance
        // assignments have been applied. Checking them here would make a
        // valid instance depend on the textual order of its assignments.
        self.validate_parameter_value(i, value, false)?;
        self.context.set_param(i, value);
        self.context.mark_param_given(i);
        for filter in &mut self.context.zi_filters {
            filter.invalidate_definition();
        }
        Ok(true)
    }

    fn validate_parameter_value(
        &mut self,
        parameter_index: usize,
        value: f64,
        resolve_dynamic_constraints: bool,
    ) -> Result<(), ParameterValueError> {
        let parameter = self
            .model
            .parameters
            .get(parameter_index)
            .cloned()
            .ok_or_else(|| ParameterValueError::InvalidConstraint {
                parameter: "<unknown>".into(),
                detail: format!("parameter index {parameter_index} is out of bounds"),
            })?;
        if !value.is_finite() {
            return Err(ParameterValueError::NonFinite {
                parameter: parameter.name.clone(),
                value,
            });
        }
        if parameter.is_integer
            && (value.fract() != 0.0 || value < f64::from(i32::MIN) || value > f64::from(i32::MAX))
        {
            return Err(ParameterValueError::NonInteger {
                parameter: parameter.name.clone(),
                value,
            });
        }

        let lower_source_count = usize::from(parameter.min.is_some())
            + usize::from(parameter.min_parameter.is_some())
            + usize::from(parameter.min_program.is_some());
        if lower_source_count > 1 {
            return Err(ParameterValueError::InvalidConstraint {
                parameter: parameter.name.clone(),
                detail: "lower bound has conflicting constant, parameter, or expression sources"
                    .to_string(),
            });
        }
        let upper_source_count = usize::from(parameter.max.is_some())
            + usize::from(parameter.max_parameter.is_some())
            + usize::from(parameter.max_program.is_some());
        if upper_source_count > 1 {
            return Err(ParameterValueError::InvalidConstraint {
                parameter: parameter.name.clone(),
                detail: "upper bound has conflicting constant, parameter, or expression sources"
                    .to_string(),
            });
        }

        let mut evaluate_program = |program: &crate::codegen::BytecodeProgram, label: String| {
            let mut vm = Vm::new(&mut self.context);
            let bound =
                vm.execute(program)
                    .map_err(|error| ParameterValueError::InvalidConstraint {
                        parameter: parameter.name.clone(),
                        detail: format!("{label} evaluation failed: {error}"),
                    })?;
            if !bound.is_finite() {
                return Err(ParameterValueError::InvalidConstraint {
                    parameter: parameter.name.clone(),
                    detail: format!("{label} evaluated to non-finite value {bound}"),
                });
            }
            Ok::<_, ParameterValueError>((bound, label))
        };
        let computed_min = if resolve_dynamic_constraints {
            parameter
                .min_program
                .as_ref()
                .map(|program| {
                    evaluate_program(program, "computed lower-bound expression".to_string())
                })
                .transpose()?
        } else {
            None
        };
        let computed_max = if resolve_dynamic_constraints {
            parameter
                .max_program
                .as_ref()
                .map(|program| {
                    evaluate_program(program, "computed upper-bound expression".to_string())
                })
                .transpose()?
        } else {
            None
        };
        let computed_exclusions = if resolve_dynamic_constraints {
            parameter
                .exclude_programs
                .iter()
                .enumerate()
                .map(|(index, program)| {
                    evaluate_program(program, format!("computed exclusion expression {index}"))
                })
                .collect::<Result<Vec<_>, _>>()?
        } else {
            Vec::new()
        };

        let dynamic_bound = |index: Option<usize>, label: &str| {
            index
                .map(|index| {
                    let bound_parameter = self.model.parameters.get(index).ok_or_else(|| {
                        ParameterValueError::InvalidConstraint {
                            parameter: parameter.name.clone(),
                            detail: format!("{label} parameter index {index} is out of bounds"),
                        }
                    })?;
                    let value = self.context.parameters.get(index).copied().ok_or_else(|| {
                        ParameterValueError::InvalidConstraint {
                            parameter: parameter.name.clone(),
                            detail: format!(
                                "{label} parameter '{}' has no runtime value",
                                bound_parameter.name
                            ),
                        }
                    })?;
                    if !value.is_finite() {
                        return Err(ParameterValueError::InvalidConstraint {
                            parameter: parameter.name.clone(),
                            detail: format!(
                                "{label} parameter '{}' has non-finite value {value}",
                                bound_parameter.name
                            ),
                        });
                    }
                    Ok((value, bound_parameter.name.to_string()))
                })
                .transpose()
        };
        let referenced_min = if resolve_dynamic_constraints {
            dynamic_bound(parameter.min_parameter, "lower-bound")?
        } else {
            None
        };
        let referenced_max = if resolve_dynamic_constraints {
            dynamic_bound(parameter.max_parameter, "upper-bound")?
        } else {
            None
        };
        let dynamic_min = computed_min.or(referenced_min);
        let dynamic_max = computed_max.or(referenced_max);
        let min = dynamic_min
            .as_ref()
            .map(|(value, _)| *value)
            .or(parameter.min);
        let max = dynamic_max
            .as_ref()
            .map(|(value, _)| *value)
            .or(parameter.max);

        if let (Some(min), Some(max)) = (min, max)
            && (min > max || (min == max && (parameter.min_exclusive || parameter.max_exclusive)))
        {
            return Err(ParameterValueError::InvalidConstraint {
                parameter: parameter.name.clone(),
                detail: format!("range is empty for lower bound {min} and upper bound {max}"),
            });
        }

        let below_min = min.is_some_and(|min| {
            if parameter.min_exclusive {
                value <= min
            } else {
                value < min
            }
        });
        let above_max = max.is_some_and(|max| {
            if parameter.max_exclusive {
                value >= max
            } else {
                value > max
            }
        });
        if below_min || above_max {
            let left = if parameter.min_exclusive { '(' } else { '[' };
            let right = if parameter.max_exclusive { ')' } else { ']' };
            let min = dynamic_min.map_or_else(
                || min.map_or_else(|| "-inf".to_string(), |bound| bound.to_string()),
                |(bound, name)| format!("{name}={bound}"),
            );
            let max = dynamic_max.map_or_else(
                || max.map_or_else(|| "inf".to_string(), |bound| bound.to_string()),
                |(bound, name)| format!("{name}={bound}"),
            );
            return Err(ParameterValueError::OutOfRange {
                parameter: parameter.name.clone(),
                value,
                constraint: format!("{left}{min}:{max}{right}"),
            });
        }
        let dynamically_excluded = if resolve_dynamic_constraints {
            parameter
                .exclude_parameters
                .iter()
                .try_fold(false, |excluded, index| {
                    match dynamic_bound(Some(*index), "excluded-value")? {
                        Some((bound, _)) => Ok(excluded || value == bound),
                        None => Err(ParameterValueError::InvalidConstraint {
                            parameter: parameter.name.clone(),
                            detail: "excluded-value reference is missing".to_string(),
                        }),
                    }
                })?
        } else {
            false
        };
        let computed_excluded = computed_exclusions
            .iter()
            .any(|(excluded, _)| value == *excluded);
        if parameter.exclude.contains(&value) || dynamically_excluded || computed_excluded {
            return Err(ParameterValueError::Excluded {
                parameter: parameter.name.clone(),
                value,
            });
        }
        Ok(())
    }

    /// Evaluate dependent parameter defaults for parameters the instance
    /// did not set, in declaration order.
    ///
    /// Must be called after all instance parameters have been applied;
    /// calling it again is harmless (it is idempotent for a fixed set of
    /// given parameters).
    pub fn resolve_parameter_defaults(&mut self) {
        self.try_resolve_parameter_defaults().unwrap_or_else(|err| {
            panic!(
                "Verilog-A device '{}' model '{}' parameter default resolution failed: {}",
                self.name, self.model.name, err
            )
        });
    }

    /// Checked dependent-parameter default evaluation. A malformed or stale
    /// compiled default program must not become a numeric zero.
    pub fn try_resolve_parameter_defaults(&mut self) -> Result<(), VmError> {
        for filter in &mut self.context.zi_filters {
            filter.invalidate_definition();
        }
        for i in 0..self.model.parameters.len() {
            if self.context.is_param_given(i) {
                continue;
            }
            if self.model.parameters[i].default_program.is_none() {
                continue;
            }

            #[cfg(feature = "native")]
            let value = self.run_native_parameter_default(i)?;

            #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
            let value = self.run_wasm_parameter_default(i)?;

            #[cfg(all(
                not(feature = "native"),
                not(all(feature = "wasm-jit", target_arch = "wasm32"))
            ))]
            let value = {
                let default_program = self.model.parameters[i]
                    .default_program
                    .clone()
                    .expect("default program checked above");
                let context = &mut self.context;
                let mut vm = Vm::new(context);
                vm.execute(&default_program)?
            };

            self.validate_parameter_value(i, value, false)
                .map_err(|error| VmError::ParameterValue(error.to_string()))?;
            self.context.set_param(i, value);
        }

        // A later override can tighten the range of an earlier parameter.
        // Revalidate the complete final vector after every default has been
        // resolved so declaration and instance assignment order cannot hide
        // a cross-parameter violation.
        for i in 0..self.model.parameters.len() {
            let value = self.context.parameters[i];
            self.validate_parameter_value(i, value, true)
                .map_err(|error| VmError::ParameterValue(error.to_string()))?;
        }

        // Topology guards depend on final parameter values.
        self.try_refresh_static_conditions()?;

        Ok(())
    }

    #[cfg(feature = "native")]
    fn run_native_parameter_default(&mut self, index: usize) -> Result<f64, VmError> {
        if self.context.variables.len() < self.model.num_variables {
            self.context.variables.resize(self.model.num_variables, 0.0);
        }

        let default_program = self.model.parameters[index]
            .default_program
            .clone()
            .expect("default program checked above");
        let native = std::sync::Arc::clone(&self.native_model);
        let context = &mut self.context;
        let mut vm = Vm::new(context);
        Self::run_value_program(
            &mut vm,
            &default_program,
            native.as_ref(),
            NativeValueEntry::ParameterDefault(index),
        )
    }

    #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
    fn run_wasm_parameter_default(&mut self, index: usize) -> Result<f64, VmError> {
        if self.context.variables.len() < self.model.num_variables {
            self.context.variables.resize(self.model.num_variables, 0.0);
        }
        self.wasm_jit_model
            .run_entry(
                WasmJitExecutableEntry::ParameterDefault(index),
                &mut self.context,
            )
            .map_err(VmError::WasmJit)
    }

    /// Set simulation temperature in Kelvin
    pub fn set_temperature(&mut self, temp_k: f64) {
        self.try_set_temperature(temp_k).unwrap_or_else(|err| {
            panic!(
                "Verilog-A device '{}' model '{}' temperature update failed: {}",
                self.name, self.model.name, err
            )
        });
    }

    /// Checked temperature update for callers that can surface native static
    /// guard refresh failures as diagnostics instead of panicking.
    pub fn try_set_temperature(&mut self, temp_k: f64) -> Result<(), VmError> {
        if !temp_k.is_finite() || temp_k <= 0.0 {
            return Err(VmError::InvalidRuntimeConfiguration(format!(
                "temperature must be finite and greater than zero kelvin, got {temp_k}"
            )));
        }
        if self.context.temperature == temp_k {
            return Ok(());
        }

        let previous = self.context.clone();
        self.context.temperature = temp_k;
        // Static guards may reference $temperature
        if let Err(error) = self.try_refresh_static_conditions() {
            self.context = previous;
            return Err(error);
        }

        Ok(())
    }

    /// Set simulation time
    pub fn set_time(&mut self, time: f64) {
        self.try_set_time(time).unwrap_or_else(|err| {
            panic!(
                "Verilog-A device '{}' model '{}' time update failed: {}",
                self.name, self.model.name, err
            )
        });
    }

    /// Checked simulation-time update.
    pub fn try_set_time(&mut self, time: f64) -> Result<(), VmError> {
        if !time.is_finite() || time < 0.0 {
            return Err(VmError::InvalidRuntimeConfiguration(format!(
                "simulation time must be finite and non-negative, got {time}"
            )));
        }
        self.context.time = time;
        Ok(())
    }

    /// Set the transient timestep (0 selects DC semantics for ddt/idt)
    pub fn set_timestep(&mut self, dt: f64) {
        self.try_set_timestep(dt).unwrap_or_else(|err| {
            panic!(
                "Verilog-A device '{}' model '{}' timestep update failed: {}",
                self.name, self.model.name, err
            )
        });
    }

    /// Checked transient-timestep update.
    pub fn try_set_timestep(&mut self, dt: f64) -> Result<(), VmError> {
        self.context.try_set_timestep(dt)
    }

    /// Select the transient solver's companion coefficients for analog
    /// integration operators at the current candidate timepoint.
    pub fn set_integration_coefficients(
        &mut self,
        coefficients: crate::vm::IntegrationCoefficients,
    ) {
        self.try_set_integration_coefficients(coefficients)
            .unwrap_or_else(|err| {
                panic!(
                    "Verilog-A device '{}' model '{}' integration update failed: {}",
                    self.name, self.model.name, err
                )
            });
    }

    /// Checked companion-coefficient update.
    pub fn try_set_integration_coefficients(
        &mut self,
        coefficients: crate::vm::IntegrationCoefficients,
    ) -> Result<(), VmError> {
        self.context.try_set_integration_coefficients(coefficients)
    }

    /// Set the analysis type (0=dc, 1=ac, 2=tran, 3=noise, 4=ic)
    pub fn set_analysis_type(&mut self, analysis: u8) {
        self.try_set_analysis_type(analysis).unwrap_or_else(|err| {
            panic!(
                "Verilog-A device '{}' model '{}' analysis update failed: {}",
                self.name, self.model.name, err
            )
        });
    }

    /// Checked analysis update for callers that can surface native static
    /// guard refresh failures as diagnostics instead of panicking.
    pub fn try_set_analysis_type(&mut self, analysis: u8) -> Result<(), VmError> {
        if analysis > 4 {
            return Err(VmError::InvalidRuntimeConfiguration(format!(
                "analysis type must be one of 0=dc, 1=ac, 2=tran, 3=noise, or 4=ic, got {analysis}"
            )));
        }
        let evaluation_mode = crate::vm::VerilogAEvaluationMode::default_for_analysis(analysis);
        if self.context.analysis_type == analysis {
            self.context.evaluation_mode = evaluation_mode;
            return Ok(());
        }

        let previous = self.context.clone();
        self.context.analysis_type = analysis;
        self.context.evaluation_mode = evaluation_mode;
        if let Err(error) = self.try_refresh_static_conditions() {
            self.context = previous;
            return Err(error);
        }

        Ok(())
    }

    /// Explicitly begin a fresh analysis on a reusable device instance.
    /// All device-owned dynamic state is reset even when the analysis code is
    /// unchanged from the preceding run. Instance configuration and compiled
    /// filter realizations survive, while each logical Zi site freezes its
    /// analysis-specific constant arguments lazily on first ordered execution.
    pub fn try_begin_analysis(&mut self, analysis: u8) -> Result<(), VmError> {
        self.try_set_analysis_type(analysis)?;
        self.context.reset_analysis_state();
        Ok(())
    }

    /// Panicking compatibility wrapper for [`Self::try_begin_analysis`].
    pub fn begin_analysis(&mut self, analysis: u8) {
        self.try_begin_analysis(analysis).unwrap_or_else(|error| {
            panic!(
                "Verilog-A device '{}' model '{}' analysis begin failed: {}",
                self.name, self.model.name, error
            )
        });
    }

    /// Mark whether the current evaluation is the first and/or final point
    /// of its analysis. A single-point analysis legitimately sets both.
    pub fn set_analysis_step(&mut self, initial: bool, final_step: bool) {
        self.try_set_analysis_step(initial, final_step)
            .unwrap_or_else(|err| {
                panic!(
                    "Verilog-A device '{}' model '{}' analysis-step update failed: {}",
                    self.name, self.model.name, err
                )
            });
    }

    /// Checked analysis-step update for callers that can surface native
    /// static-guard refresh failures as diagnostics.
    pub fn try_set_analysis_step(
        &mut self,
        initial: bool,
        final_step: bool,
    ) -> Result<(), VmError> {
        if self.context.analysis_initial_step == initial
            && self.context.analysis_final_step == final_step
        {
            return Ok(());
        }

        let previous = self.context.clone();
        self.context.analysis_initial_step = initial;
        self.context.analysis_final_step = final_step;
        if let Err(error) = self.try_refresh_static_conditions() {
            self.context = previous;
            return Err(error);
        }
        Ok(())
    }

    /// Commit integrator state after an accepted timestep
    pub fn advance_state(&mut self) {
        self.try_advance_state().unwrap_or_else(|error| {
            panic!(
                "Verilog-A device '{}' model '{}' state commit failed: {}",
                self.name, self.model.name, error
            )
        });
    }

    /// Checked accepted-state commit. Sampled filters refuse a commit that
    /// crossed or failed to evaluate a required sample edge.
    pub fn try_advance_state(&mut self) -> Result<(), VmError> {
        self.validate_advance_state()?;
        self.apply_validated_advance_state();
        Ok(())
    }

    /// Validate an accepted-state commit without mutating this instance.
    pub fn validate_advance_state(&self) -> Result<(), VmError> {
        self.context.validate_advance_state()
    }

    /// Apply a commit only after all runtime instances in the circuit have
    /// passed [`Self::validate_advance_state`].
    pub fn apply_validated_advance_state(&mut self) {
        let discontinuity = self.discontinuity_pending();
        self.context.apply_validated_advance_state();
        self.prev_discontinuity = discontinuity;
    }

    pub fn checkpoint_state(&self) -> Result<VerilogADeviceCheckpoint, VmError> {
        let checkpoint = VerilogADeviceCheckpoint {
            instance_name: self.name.clone(),
            model_name: self.model.name.clone(),
            source_digest: self.model.source_digest.clone(),
            shape_identity: self.checkpoint_shape_identity(),
            state_version: RUNTIME_CHECKPOINT_STATE_VERSION,
            accepted: self.context.accepted_checkpoint()?,
            prev_discontinuity: self.prev_discontinuity,
        };
        self.validate_checkpoint_state(&checkpoint)?;
        Ok(checkpoint)
    }

    pub fn validate_checkpoint_state(
        &self,
        checkpoint: &VerilogADeviceCheckpoint,
    ) -> Result<(), VmError> {
        let invalid = |message: String| VmError::InvalidNumericResult(message);
        if checkpoint.state_version != RUNTIME_CHECKPOINT_STATE_VERSION {
            return Err(invalid(format!(
                "unsupported runtime checkpoint state version {}",
                checkpoint.state_version
            )));
        }
        if checkpoint.instance_name != self.name
            || checkpoint.model_name != self.model.name
            || checkpoint.source_digest != self.model.source_digest
            || checkpoint.shape_identity != self.checkpoint_shape_identity()
        {
            return Err(invalid(
                "runtime checkpoint device identity or resolved shape does not match".into(),
            ));
        }
        let bound_step_index = self
            .model
            .variable_names
            .iter()
            .position(|name| name == "$bound_step");
        for (index, value) in checkpoint.accepted.variables.iter().copied().enumerate() {
            let allowed_bound_infinity = Some(index) == bound_step_index && value == f64::INFINITY;
            if !value.is_finite() && !allowed_bound_infinity {
                return Err(invalid(format!(
                    "checkpoint variable {index} is non-finite outside the $bound_step sentinel"
                )));
            }
        }
        self.context
            .validate_accepted_checkpoint(&checkpoint.accepted)
    }

    /// Infallible injection after collection-wide validation.
    pub fn apply_validated_checkpoint_state(&mut self, checkpoint: &VerilogADeviceCheckpoint) {
        self.context
            .restore_accepted_checkpoint(&checkpoint.accepted);
        self.prev_discontinuity = checkpoint.prev_discontinuity;
    }

    fn checkpoint_shape_identity(&self) -> SmolStr {
        let mut hasher = blake3::Hasher::new();
        let mut add_bytes = |bytes: &[u8]| {
            hasher.update(&(bytes.len() as u64).to_le_bytes());
            hasher.update(bytes);
        };
        add_bytes(self.model.name.as_bytes());
        add_bytes(self.model.source_digest.as_bytes());
        add_bytes(self.name.as_bytes());
        for value in [
            self.node_mapping.len(),
            self.internal_node_indices.len(),
            self.branch_current_indices.len(),
            self.context.variables.len(),
            self.context.state_values.len(),
            self.context.delay_buffers.len(),
            self.context.transition_filters.len(),
            self.context.slew_filters.len(),
            self.context.cross_detectors.len(),
            self.context.laplace_filters.len(),
            self.context.zi_filters.len(),
        ] {
            hasher.update(&(value as u64).to_le_bytes());
        }
        for value in &self.context.parameters {
            hasher.update(&value.to_bits().to_le_bytes());
        }
        hasher.update(&self.context.param_given);
        for value in &self.node_mapping {
            hasher.update(&(*value as u64).to_le_bytes());
        }
        SmolStr::new(hasher.finalize().to_hex().as_str())
    }

    /// Whether `$discontinuity` newly fired since the last accepted step
    pub fn discontinuity_rising(&self) -> bool {
        self.discontinuity_pending() && !self.prev_discontinuity
    }

    /// Set the circuit node indices for internal nodes
    ///
    /// Called during circuit setup when the solver allocates nodes for internal nodes.
    pub fn set_internal_node_indices(&mut self, indices: &[usize]) {
        self.try_set_internal_node_indices(indices)
            .unwrap_or_else(|err| {
                panic!(
                    "Verilog-A device '{}' model '{}' internal-node mapping failed: {}",
                    self.name, self.model.name, err
                )
            });
    }

    /// Checked internal-node mapping update.
    pub fn try_set_internal_node_indices(&mut self, indices: &[usize]) -> Result<(), VmError> {
        Self::validate_solver_indices("internal-node", indices, self.internal_node_indices.len())?;
        self.internal_node_indices.copy_from_slice(indices);
        self.rebuild_matrix_indices();
        Ok(())
    }

    /// Number of branch-current unknowns required by this device's
    /// potential contributions (the engine allocates one extra system
    /// unknown per entry)
    pub fn num_branch_unknowns(&self) -> usize {
        self.model.branch_sources.len()
    }

    /// Set the circuit node indices allocated for branch-current unknowns
    pub fn set_branch_current_indices(&mut self, indices: &[usize]) {
        self.try_set_branch_current_indices(indices)
            .unwrap_or_else(|err| {
                panic!(
                    "Verilog-A device '{}' model '{}' branch-current mapping failed: {}",
                    self.name, self.model.name, err
                )
            });
    }

    /// Checked branch-current unknown mapping update.
    pub fn try_set_branch_current_indices(&mut self, indices: &[usize]) -> Result<(), VmError> {
        Self::validate_solver_indices(
            "branch-current",
            indices,
            self.branch_current_indices.len(),
        )?;
        self.branch_current_indices.copy_from_slice(indices);
        self.rebuild_matrix_indices();
        Ok(())
    }

    fn validate_solver_indices(
        kind: &str,
        indices: &[usize],
        expected: usize,
    ) -> Result<(), VmError> {
        if indices.len() != expected {
            return Err(VmError::InvalidRuntimeConfiguration(format!(
                "{kind} mapping requires exactly {expected} index(es), got {}",
                indices.len()
            )));
        }
        if let Some(position) = indices.iter().position(|index| *index == 0) {
            return Err(VmError::InvalidRuntimeConfiguration(format!(
                "{kind} mapping index {position} resolves to ground"
            )));
        }
        let mut sorted = indices.to_vec();
        sorted.sort_unstable();
        if sorted.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(VmError::InvalidRuntimeConfiguration(format!(
                "{kind} mapping contains duplicate solver indices"
            )));
        }
        Ok(())
    }

    /// Re-evaluate instance-static activation conditions (mode guards
    /// peeled from contributions: parameter expressions or variables
    /// derived purely from parameters). A potential contribution whose
    /// guard is false leaves its branch open; a branch driven by no
    /// active potential contribution is forced to zero current.
    #[cfg(feature = "native")]
    fn try_refresh_static_conditions(&mut self) -> Result<(), VmError> {
        let model = &self.model;
        let native = self.native_model.as_ref();
        let mut program_active = vec![true; model.stamp_programs.len()];
        let mut branch_active = vec![false; model.branch_sources.len()];
        let has_static_conditions = model
            .stamp_programs
            .iter()
            .any(|program| program.static_condition.is_some());

        if has_static_conditions {
            // Static-activation discovery is speculative. Run it on a full
            // context clone so lazy Zi definition freezes and state candidates
            // in assignment expressions cannot leak into accepted state.
            let mut refresh_context = self.context.clone();
            let context = &mut refresh_context;
            let mut vm = Vm::new(context);
            Self::run_assignment_pass(&mut vm, model, native)?;

            for (idx, program) in model.stamp_programs.iter().enumerate() {
                let active = if program.static_condition.is_some() {
                    let condition = Self::run_value_program(
                        &mut vm,
                        program
                            .static_condition
                            .as_ref()
                            .expect("static condition checked above"),
                        native,
                        NativeValueEntry::StaticCondition(idx),
                    )?;
                    Self::finite_result(condition, format!("static condition {idx}"))? != 0.0
                } else {
                    true
                };
                program_active[idx] = active;
                if active
                    && let Some(ordinal) = program.branch_ordinal
                    && ordinal < branch_active.len()
                {
                    branch_active[ordinal] = true;
                }
            }
        } else {
            for program in &model.stamp_programs {
                if let Some(ordinal) = program.branch_ordinal
                    && ordinal < branch_active.len()
                {
                    branch_active[ordinal] = true;
                }
            }
        }

        self.program_active = program_active;
        self.sync_fused_program_active();
        self.branch_active = branch_active;
        Ok(())
    }

    /// Refresh the fused drivers' byte-addressable activation mirror.
    ///
    /// Static conditions are the only thing that deactivates a contribution,
    /// so the mirror is rebuilt where they are evaluated rather than on every
    /// dispatch.
    #[cfg(any(feature = "native", all(feature = "wasm-jit", target_arch = "wasm32")))]
    fn sync_fused_program_active(&mut self) {
        self.fused_program_active.clear();
        self.fused_program_active
            .extend(self.program_active.iter().map(|active| u8::from(*active)));
    }

    #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
    fn try_refresh_static_conditions(&mut self) -> Result<(), VmError> {
        let model = &self.model;
        let wasm = self.wasm_jit_model.as_ref();
        let mut program_active = vec![true; model.stamp_programs.len()];
        let mut branch_active = vec![false; model.branch_sources.len()];
        let has_static_conditions = model
            .stamp_programs
            .iter()
            .any(|program| program.static_condition.is_some());

        if has_static_conditions {
            let mut refresh_context = self.context.clone();
            let context = &mut refresh_context;
            let mut vm = Vm::new(context);
            Self::run_assignment_pass(&mut vm, model, wasm)?;
            for (idx, program) in model.stamp_programs.iter().enumerate() {
                let active = if program.static_condition.is_some() {
                    let condition = Self::run_value_program(
                        &mut vm,
                        program
                            .static_condition
                            .as_ref()
                            .expect("static condition checked above"),
                        wasm,
                        WasmJitExecutableEntry::StaticCondition(idx),
                    )?;
                    Self::finite_result(condition, format!("static condition {idx}"))? != 0.0
                } else {
                    true
                };
                program_active[idx] = active;
                if active
                    && let Some(ordinal) = program.branch_ordinal
                    && ordinal < branch_active.len()
                {
                    branch_active[ordinal] = true;
                }
            }
        } else {
            for program in &model.stamp_programs {
                if let Some(ordinal) = program.branch_ordinal
                    && ordinal < branch_active.len()
                {
                    branch_active[ordinal] = true;
                }
            }
        }
        self.program_active = program_active;
        self.sync_fused_program_active();
        self.branch_active = branch_active;
        Ok(())
    }

    #[cfg(feature = "native")]
    fn missing_native_static_condition_entry(index: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT missing static-condition entry for stamp {index}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_parameter_default_entry(index: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT missing parameter-default entry for parameter {index}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_noise_exponent_entry(index: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT missing noise exponent entry for source {index}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_stamp_value_entry(index: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT missing stamp-value entry {index}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_jacobian_entry(stamp: usize, entry: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT missing Jacobian entry {stamp}.{entry}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_reactive_jacobian_entry(stamp: usize, entry: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT missing reactive-Jacobian entry {stamp}.{entry}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_noise_psd_entry(index: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT missing noise PSD entry for source {index}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_terminal_pair_current_slot(pair_index: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT missing terminal-pair current slot {pair_index}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_branch_current_unknown(index: usize, available: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT missing branch-current unknown {index}; only {available} branch-current unknown(s) available; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_parameter_storage(required: usize, available: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT parameter storage has {available} slot(s), but compiled image requires {required}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_param_given_storage(required: usize, available: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT parameter-given storage has {available} slot(s), but compiled image requires {required}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_variable_storage(required: usize, available: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT variable storage has {available} slot(s), but compiled image requires {required}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_voltage_storage(required: usize, available: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT voltage storage has {available} terminal slot(s), but compiled image requires {required}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn mismatched_native_terminal_context(required: usize, available: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT terminal context has {available} terminal slot(s), but compiled image requires exactly {required}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_internal_voltage_storage(required: usize, available: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT internal-voltage storage has {available} slot(s), but compiled image requires {required}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_port_connected_storage(required: usize, available: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT port-connected storage has {available} slot(s), but compiled image requires {required}; no interpreter fallback"
        ))
    }

    #[cfg(feature = "native")]
    fn missing_native_runtime_storage(label: &str, required: usize, available: usize) -> VmError {
        VmError::NativeJit(format!(
            "native JIT {label} has {available} slot(s), but compiled image requires {required}; no interpreter fallback"
        ))
    }

    #[cfg(all(
        not(feature = "native"),
        not(all(feature = "wasm-jit", target_arch = "wasm32"))
    ))]
    fn try_refresh_static_conditions(&mut self) -> Result<(), VmError> {
        let model = &self.model;
        let mut program_active = vec![true; model.stamp_programs.len()];
        let mut branch_active = vec![false; model.branch_sources.len()];

        {
            let mut refresh_context = self.context.clone();
            let context = &mut refresh_context;
            let mut bytecode_vm = Vm::new(context);
            // Static guards may reference instance-static variables (e.g.
            // BSIM4rdsMod derived from the rdsmod parameter); run the
            // evaluation stream once so those variables hold their values.
            // Node voltages are irrelevant to instance-static expressions.
            Self::execute_assignment_programs(&mut bytecode_vm, model)?;
            for (idx, program) in model.stamp_programs.iter().enumerate() {
                let active = match &program.static_condition {
                    Some(condition) => {
                        let condition = bytecode_vm.execute(condition)?;
                        Self::finite_result(condition, format!("static condition {idx}"))? != 0.0
                    }
                    None => true,
                };
                program_active[idx] = active;
                if active
                    && let Some(ordinal) = program.branch_ordinal
                    && ordinal < branch_active.len()
                {
                    branch_active[ordinal] = true;
                }
            }
        }

        self.program_active = program_active;
        self.branch_active = branch_active;
        Ok(())
    }

    /// Get the circuit node index for an internal node
    pub fn internal_node_index(&self, internal_idx: usize) -> Option<usize> {
        self.internal_node_indices.get(internal_idx).copied()
    }

    /// Get the circuit node index allocated for a branch-current unknown
    pub fn branch_current_index(&self, ordinal: usize) -> Option<usize> {
        self.branch_current_indices.get(ordinal).copied()
    }

    /// Read an internal model variable by name (operating-point
    /// inspection). Returns the value from the most recent evaluation.
    pub fn variable(&self, name: &str) -> Option<f64> {
        let idx = self
            .model
            .variable_names
            .iter()
            .position(|n| n.as_str() == name)?;
        self.context.variables.get(idx).copied()
    }

    /// Iterate (name, value) over all internal model variables
    pub fn variables(&self) -> impl Iterator<Item = (&str, f64)> {
        self.model
            .variable_names
            .iter()
            .map(|n| n.as_str())
            .zip(self.context.variables.iter().copied())
    }

    /// Remap circuit node IDs after an external topology rewrite.
    pub fn remap_circuit_nodes(&mut self, mut remap: impl FnMut(usize) -> usize) {
        for node in &mut self.node_mapping {
            *node = remap(*node);
        }
        for node in &mut self.internal_node_indices {
            *node = remap(*node);
        }
        self.rebuild_matrix_indices();
    }

    /// Build mapped RHS stamp rows for each stamp program.
    ///
    /// Returns one entry per stamp program; each program entry contains
    /// `(node_index, sign)` pairs for non-ground RHS rows.
    pub fn mapped_rhs_rows(&self) -> Vec<Vec<(usize, f64)>> {
        self.matrix_indices
            .rhs
            .iter()
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(|entry| entry.node.map(|node| (node, entry.sign)))
                    .collect()
            })
            .collect()
    }

    /// Build mapped Jacobian matrix locations for each stamp program.
    ///
    /// Returns one entry per stamp program; each program entry contains
    /// `(row, col)` locations for each Jacobian program where `None` means ground.
    pub fn mapped_jacobian_locations(&self) -> Vec<Vec<(Option<usize>, Option<usize>)>> {
        self.matrix_indices
            .jacobian
            .iter()
            .map(|entries| entries.iter().map(|entry| (entry.row, entry.col)).collect())
            .collect()
    }

    /// Recompute cached matrix/RHS node mappings after topology changes.
    fn rebuild_matrix_indices(&mut self) {
        let mut rhs = vec![Vec::new(); self.model.stamp_programs.len()];
        let mut jacobian = vec![Vec::new(); self.model.stamp_programs.len()];
        let mut reactive = vec![Vec::new(); self.model.stamp_programs.len()];

        let map_entries = |entries: &[crate::codegen::JacobianEntry], program_idx: usize| {
            entries
                .iter()
                .enumerate()
                .map(|(jacobian_idx, jac_entry)| JacobianIndex {
                    row: Self::index_to_node(
                        &jac_entry.row,
                        &self.node_mapping,
                        &self.internal_node_indices,
                        &self.branch_current_indices,
                    ),
                    col: Self::index_to_node(
                        &jac_entry.col,
                        &self.node_mapping,
                        &self.internal_node_indices,
                        &self.branch_current_indices,
                    ),
                    program_idx,
                    jacobian_idx,
                    sign: jac_entry.sign,
                })
                .collect::<Vec<_>>()
        };

        for (program_idx, program) in self.model.stamp_programs.iter().enumerate() {
            rhs[program_idx] = program
                .stamp_locations
                .iter()
                .map(|loc| RhsIndex {
                    node: Self::index_to_node(
                        &loc.row,
                        &self.node_mapping,
                        &self.internal_node_indices,
                        &self.branch_current_indices,
                    ),
                    sign: loc.sign,
                    program_idx,
                })
                .collect();

            jacobian[program_idx] = map_entries(&program.jacobian_programs, program_idx);
            reactive[program_idx] = map_entries(&program.reactive_jacobians, program_idx);
        }

        self.matrix_indices = MatrixIndices {
            jacobian,
            reactive,
            rhs,
        };
        let matrix_capacity = self
            .model
            .branch_sources
            .len()
            .saturating_mul(4)
            .saturating_add(
                self.matrix_indices
                    .jacobian
                    .iter()
                    .flatten()
                    .filter(|entry| entry.row.is_some() && entry.col.is_some())
                    .count(),
            );
        let rhs_capacity = self
            .matrix_indices
            .rhs
            .iter()
            .flatten()
            .filter(|entry| entry.node.is_some())
            .count();
        self.stamp_matrix_buffer
            .reserve(matrix_capacity.saturating_sub(self.stamp_matrix_buffer.capacity()));
        self.stamp_rhs_buffer
            .reserve(rhs_capacity.saturating_sub(self.stamp_rhs_buffer.capacity()));
    }

    /// Stamp the reactive (charge/flux) Jacobian dQ/dx.
    ///
    /// AC analysis multiplies these entries by the angular frequency and
    /// adds them to the imaginary part of the system matrix: capacitances
    /// for current contributions, inductive terms on the branch rows of
    /// potential contributions.
    pub fn stamp_reactive<M>(&mut self, circuit_voltages: &[f64], mut matrix_add: M)
    where
        M: FnMut(usize, usize, f64),
    {
        if let Err(err) = self.try_stamp_reactive(circuit_voltages, &mut matrix_add) {
            panic!(
                "Verilog-A device '{}' model '{}' reactive stamping failed: {}",
                self.name, self.model.name, err
            );
        }
    }

    /// Checked reactive stamping path for callers that can report Verilog-A
    /// runtime diagnostics instead of unwinding.
    pub fn try_stamp_reactive<M>(
        &mut self,
        circuit_voltages: &[f64],
        mut matrix_add: M,
    ) -> Result<(), VmError>
    where
        M: FnMut(usize, usize, f64),
    {
        self.try_update_all_voltages(circuit_voltages)?;
        // Reactive stamping is a small-signal surface. It must never advance
        // Newton limiter history or replace the convergence result produced
        // by the nonlinear value pass.
        self.begin_evaluation(crate::vm::VerilogAEvaluationMode::SmallSignal);

        let context = &mut self.context;
        let model = &self.model;
        let matrix_indices = &self.matrix_indices;
        let program_active = &self.program_active;
        #[cfg(feature = "native")]
        let native = self.native_model.as_ref();
        #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
        let wasm = self.wasm_jit_model.as_ref();

        context.clear_currents();

        let mut vm = Vm::new(context);
        Self::run_assignment_pass(
            &mut vm,
            model,
            #[cfg(feature = "native")]
            native,
            #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
            wasm,
        )?;

        let m = vm.context.multiplicity;
        for (program_idx, program) in model.stamp_programs.iter().enumerate() {
            if !program_active.get(program_idx).copied().unwrap_or(true) {
                continue;
            }
            // Charge of m parallel copies scales the same way as current
            let scale = if program.branch_ordinal.is_none() {
                m
            } else {
                1.0
            };

            for entry in &matrix_indices.reactive[program_idx] {
                let model_entry = &program.reactive_jacobians[entry.jacobian_idx];
                let deriv = Self::run_value_program(
                    &mut vm,
                    &model_entry.program,
                    #[cfg(feature = "native")]
                    native,
                    #[cfg(feature = "native")]
                    NativeValueEntry::ReactiveJacobian {
                        stamp: program_idx,
                        entry: entry.jacobian_idx,
                    },
                    #[cfg(all(
                        not(feature = "native"),
                        feature = "wasm-jit",
                        target_arch = "wasm32"
                    ))]
                    wasm,
                    #[cfg(all(
                        not(feature = "native"),
                        feature = "wasm-jit",
                        target_arch = "wasm32"
                    ))]
                    WasmJitExecutableEntry::ReactiveJacobian {
                        stamp: program_idx,
                        entry: entry.jacobian_idx,
                    },
                )?;
                let deriv = Self::finite_result(
                    deriv * scale,
                    format!("reactive Jacobian {}:{}", program_idx, entry.jacobian_idx),
                )?;
                if let (Some(row), Some(col)) = (entry.row, entry.col) {
                    matrix_add(row, col, entry.sign * deriv);
                }
            }
        }
        Ok(())
    }

    /// Stamp the complete complex small-signal Jacobian for AC or noise.
    ///
    /// The ordinary real Jacobian plus `jω*dQ/dx` split cannot represent
    /// nested dynamic operators: their real and imaginary parts must compose
    /// before the final matrix entry is stamped.  This path first uses the
    /// selected native/VM/WASM backend to establish operating-point variables,
    /// current probes, and lazily frozen operator definitions.  A dedicated
    /// read-only complex evaluator then replays derivative shadows and the
    /// full Jacobian programs, including `ddt`, `idt`, Laplace, Zi, and
    /// transport-delay actions.  No transient history is advanced.
    pub fn try_stamp_small_signal_complex<M>(
        &mut self,
        circuit_voltages: &[f64],
        frequency_hz: f64,
        mut matrix_add: M,
    ) -> Result<(), VmError>
    where
        M: FnMut(usize, usize, f64, f64),
    {
        if !matches!(self.context.analysis_type, 1 | 3) {
            return Err(VmError::InvalidRuntimeConfiguration(format!(
                "complex small-signal stamping requires AC or noise analysis, got analysis type {}",
                self.context.analysis_type
            )));
        }
        if !frequency_hz.is_finite() || frequency_hz < 0.0 {
            return Err(VmError::InvalidRuntimeConfiguration(format!(
                "small-signal frequency must be finite and nonnegative, got {frequency_hz}"
            )));
        }

        self.try_update_all_voltages(circuit_voltages)?;
        // Replay procedural assignments from the same pre-pass image used by
        // the selected scalar backend. Seeding from its post-pass image would
        // execute self-referential/event-state assignments twice at every
        // frequency point.
        let variable_seed = self.context.variables.clone();
        self.try_evaluate_with_mode(crate::vm::VerilogAEvaluationMode::SmallSignal)?;

        let model = &self.model;
        let matrix_indices = &self.matrix_indices;
        let mut vm = crate::vm::SmallSignalVm::with_variable_seed(
            &self.context,
            frequency_hz,
            &variable_seed,
        )?;
        vm.execute_assignments(&model.assignment_steps)?;

        let matrix_capacity = model.branch_sources.len().saturating_mul(4).saturating_add(
            matrix_indices
                .jacobian
                .iter()
                .flatten()
                .filter(|entry| entry.row.is_some() && entry.col.is_some())
                .count(),
        );
        let mut real_structural = Vec::with_capacity(model.branch_sources.len().saturating_mul(4));
        Self::buffer_structural_branches(
            model,
            &self.branch_active,
            &self.node_mapping,
            &self.internal_node_indices,
            &self.branch_current_indices,
            &mut real_structural,
            self.context.multiplicity,
        );
        let mut entries = Vec::with_capacity(matrix_capacity);
        entries.extend(
            real_structural
                .into_iter()
                .map(|(row, col, value)| (row, col, value, 0.0)),
        );

        for (program_idx, program) in model.stamp_programs.iter().enumerate() {
            if !self
                .program_active
                .get(program_idx)
                .copied()
                .unwrap_or(true)
            {
                continue;
            }
            let scale = if program.branch_ordinal.is_none() {
                self.context.multiplicity
            } else {
                1.0
            };
            for mapped_entry in &matrix_indices.jacobian[program_idx] {
                let model_entry = &program.jacobian_programs[mapped_entry.jacobian_idx];
                let derivative = vm.execute(&model_entry.program)? * (mapped_entry.sign * scale);
                if !derivative.re.is_finite() || !derivative.im.is_finite() {
                    return Err(VmError::InvalidNumericResult(format!(
                        "complex small-signal Jacobian {}:{} is non-finite at {frequency_hz} Hz",
                        program_idx, mapped_entry.jacobian_idx
                    )));
                }
                if let (Some(row), Some(col)) = (mapped_entry.row, mapped_entry.col) {
                    entries.push((row, col, derivative.re, derivative.im));
                }
            }
        }

        // Publish atomically only after every program and coefficient has
        // validated, matching the nonlinear stamp contract.
        for (row, col, real, imag) in entries {
            matrix_add(row, col, real, imag);
        }
        Ok(())
    }

    /// Update terminal voltages from circuit solution
    ///
    /// Called before evaluating device equations.
    pub fn update_voltages(&mut self, circuit_voltages: &[f64]) {
        self.try_update_voltages(circuit_voltages)
            .unwrap_or_else(|err| {
                panic!(
                    "Verilog-A device '{}' model '{}' terminal voltage update failed: {}",
                    self.name, self.model.name, err
                )
            });
    }

    /// Checked terminal voltage update from circuit solution.
    pub fn try_update_voltages(&mut self, circuit_voltages: &[f64]) -> Result<(), VmError> {
        for (terminal, &node) in self.node_mapping.iter().enumerate() {
            if terminal < self.context.voltages.len() {
                let v =
                    Self::solution_value(circuit_voltages, node, "missing terminal solution slot")?;
                self.context.voltages[terminal] = v;
            }
        }
        Ok(())
    }

    /// Update both terminal and internal node voltages from circuit solution
    ///
    /// This is the full-featured method for solver integration.
    pub fn update_all_voltages(&mut self, circuit_voltages: &[f64]) {
        self.try_update_all_voltages(circuit_voltages)
            .unwrap_or_else(|err| {
                panic!(
                    "Verilog-A device '{}' model '{}' solution update failed: {}",
                    self.name, self.model.name, err
                )
            });
    }

    /// Checked update of terminals, internal nodes, and branch-current
    /// unknowns from a circuit solution.
    pub fn try_update_all_voltages(&mut self, circuit_voltages: &[f64]) -> Result<(), VmError> {
        // Update terminal voltages
        self.try_update_voltages(circuit_voltages)?;

        // Update internal node voltages
        for (internal_idx, &circuit_node) in self.internal_node_indices.iter().enumerate() {
            if internal_idx < self.context.internal_voltages.len() {
                let v = Self::solution_value(
                    circuit_voltages,
                    circuit_node,
                    "missing internal-node solution slot",
                )?;
                self.context.internal_voltages[internal_idx] = v;
            }
        }

        // Update branch-current unknown values
        for (ordinal, &circuit_node) in self.branch_current_indices.iter().enumerate() {
            if ordinal < self.context.branch_current_values.len() {
                let v = Self::solution_value(
                    circuit_voltages,
                    circuit_node,
                    "missing branch-current solution slot",
                )?;
                self.context.branch_current_values[ordinal] = v;
            }
        }
        Ok(())
    }

    fn solution_value(
        circuit_voltages: &[f64],
        circuit_node: usize,
        missing_message: &'static str,
    ) -> Result<f64, VmError> {
        if circuit_node == 0 {
            Ok(0.0)
        } else {
            circuit_voltages
                .get(circuit_node - 1)
                .copied()
                .ok_or(VmError::InvalidInstruction(missing_message))
        }
    }

    /// Evaluate the device: compute branch current
    ///
    /// Returns the current for each branch equation. Native builds require
    /// complete assignment and stamp-value entry points; non-native builds run
    /// the bytecode programs.
    pub fn evaluate(&mut self) -> Vec<f64> {
        self.try_evaluate().unwrap_or_else(|err| {
            panic!(
                "Verilog-A device '{}' model '{}' evaluation failed: {}",
                self.name, self.model.name, err
            )
        })
    }

    /// Checked evaluation path for callers that can surface runtime model
    /// errors as diagnostics instead of panicking.
    pub fn try_evaluate(&mut self) -> Result<Vec<f64>, VmError> {
        let mode =
            crate::vm::VerilogAEvaluationMode::default_for_analysis(self.context.analysis_type);
        self.try_evaluate_with_mode(mode)
    }

    /// Evaluate with an explicit limiter policy. Static probes and
    /// small-signal analyses bypass named limiter history.
    pub fn try_evaluate_with_mode(
        &mut self,
        mode: crate::vm::VerilogAEvaluationMode,
    ) -> Result<Vec<f64>, VmError> {
        #[cfg(feature = "native")]
        if self.native_model.evaluation_kernel_is_eligible() {
            return self.try_evaluate_native_kernel(mode);
        }

        #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
        if self.wasm_jit_model.evaluation_kernel_is_eligible() {
            return self.try_evaluate_wasm_kernel(mode);
        }

        self.begin_evaluation(mode);
        self.context.clear_currents();
        // Pre-reserve so the currents pointer stays stable while native
        // snapshots reference it across pushes
        self.context
            .currents
            .reserve(self.model.stamp_programs.len());

        let program_active = &self.program_active;
        #[cfg(feature = "native")]
        let native = self.native_model.as_ref();
        #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
        let wasm = self.wasm_jit_model.as_ref();
        let context = &mut self.context;
        let mut vm = Vm::new(context);
        Self::run_assignment_pass(
            &mut vm,
            &self.model,
            #[cfg(feature = "native")]
            native,
            #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
            wasm,
        )?;
        let mut currents = Vec::with_capacity(self.model.stamp_programs.len());

        for (program_idx, program) in self.model.stamp_programs.iter().enumerate() {
            if !program_active.get(program_idx).copied().unwrap_or(true) {
                currents.push(0.0);
                vm.context.currents.push(0.0);
                continue;
            }
            let value = Self::run_value_program(
                &mut vm,
                &program.value_program,
                #[cfg(feature = "native")]
                native,
                #[cfg(feature = "native")]
                NativeValueEntry::StampValue(program_idx),
                #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
                wasm,
                #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
                WasmJitExecutableEntry::StampValue(program_idx),
            )?;
            let value = Self::finite_result(
                value,
                format!("contribution {program_idx} during device evaluation"),
            )?;
            currents.push(value);
            vm.context.currents.push(value);
            if program.branch_ordinal.is_none()
                && let Some((pos, neg)) = Self::infer_current_terminal_pair(program)
            {
                vm.context.set_branch_current(pos, neg, value);
            }
        }
        #[cfg(feature = "native")]
        Self::run_post_assignment_pass(&mut vm, &self.model, native)?;
        #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
        Self::run_post_assignment_pass(&mut vm, &self.model, wasm)?;

        Ok(currents)
    }

    /// Evaluate through the browser's fused driver.
    ///
    /// One dispatch replaces the assignment call plus one JavaScript round
    /// trip per stamp. The driver publishes contributions into the context's
    /// own arrays, so the post-pass and the finiteness audit below read the
    /// same state the per-entry path would have produced.
    #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
    fn try_evaluate_wasm_kernel(
        &mut self,
        mode: crate::vm::VerilogAEvaluationMode,
    ) -> Result<Vec<f64>, VmError> {
        self.begin_evaluation(mode);
        let stamp_count = self.model.stamp_programs.len();
        if self.fused_program_active.len() != stamp_count {
            return Err(VmError::WasmJit(format!(
                "browser fused-evaluation buffer does not match compiled model shape ({}/{stamp_count} active flags); no interpreter fallback",
                self.fused_program_active.len()
            )));
        }

        self.context.prepare_indexed_currents(stamp_count);
        if self.context.variables.len() < self.model.num_variables {
            self.context.variables.resize(self.model.num_variables, 0.0);
        }

        let wasm = std::sync::Arc::clone(&self.wasm_jit_model);
        let fused = wasm
            .run_fused_kernel(&mut self.context, &self.fused_program_active, None)
            .map_err(VmError::WasmJit)?;
        if !fused {
            return Err(VmError::WasmJit(
                "browser JIT module is missing its fused evaluation entry; no interpreter fallback"
                    .into(),
            ));
        }

        for program_idx in 0..stamp_count {
            if self.fused_program_active[program_idx] != 0 {
                Self::finite_stamp_value(
                    self.context.currents[program_idx],
                    program_idx,
                    None,
                    "contribution during device evaluation",
                )?;
            }
        }
        {
            let mut vm = Vm::new(&mut self.context);
            Self::run_post_assignment_pass(&mut vm, &self.model, wasm.as_ref())?;
        }

        Ok(self.context.currents.clone())
    }

    #[cfg(feature = "native")]
    fn try_evaluate_native_kernel(
        &mut self,
        mode: crate::vm::VerilogAEvaluationMode,
    ) -> Result<Vec<f64>, VmError> {
        self.begin_evaluation(mode);
        let model = &self.model;
        let native = self.native_model.as_ref();
        let stamp_count = model.stamp_programs.len();
        if self.fused_program_active.len() != stamp_count {
            return Err(VmError::NativeJit(format!(
                "native fused-evaluation buffer does not match compiled model shape ({}/{stamp_count} active flags); no interpreter fallback",
                self.fused_program_active.len()
            )));
        }

        {
            let context = &mut self.context;
            context.prepare_indexed_currents(stamp_count);
            if context.variables.len() < model.num_variables {
                context.variables.resize(model.num_variables, 0.0);
            }
            Self::validate_native_storage(context, native)?;
            Self::validate_native_terminal_pair_table(context, native.num_terminals)?;
            Self::validate_native_branch_unknowns(
                context,
                native.evaluation_kernel_branch_unknowns(),
            )?;

            let ctx = Self::eval_context_from(context);
            let io = NativeStampKernelIo {
                program_active: self.fused_program_active.as_ptr(),
                jacobians: std::ptr::null_mut(),
            };
            let vars = context.variables.as_mut_ptr();
            ctx.clear_runtime_error();
            if !native.run_evaluation_kernel(&ctx, vars, &io) {
                return Err(VmError::NativeJit(
                    "native JIT image is missing its fused evaluation entry; no interpreter fallback"
                        .into(),
                ));
            }
            if let Some(error) = ctx.take_native_runtime_error() {
                return Err(Self::native_runtime_error_to_vm(error));
            }
        }

        for program_idx in 0..stamp_count {
            if self.fused_program_active[program_idx] != 0 {
                Self::finite_stamp_value(
                    self.context.currents[program_idx],
                    program_idx,
                    None,
                    "contribution during device evaluation",
                )?;
            }
        }
        {
            let mut vm = Vm::new(&mut self.context);
            Self::run_post_assignment_pass(&mut vm, model, native)?;
        }

        Ok(self.context.currents.clone())
    }

    /// Whether no named limiter changed its proposal during the latest
    /// limited Newton evaluation.
    #[inline]
    pub fn limiter_converged(&self) -> bool {
        self.context.limiter_active == 0
    }

    #[inline]
    fn begin_evaluation(&mut self, mode: crate::vm::VerilogAEvaluationMode) {
        self.context.evaluation_mode = mode;
        self.context.begin_stateful_evaluation();
        if mode.limiting_enabled() {
            self.context.limiter_active = 0;
        }
    }

    /// Build a native evaluation-context snapshot over the VM context.
    /// Raw pointers only — rebuild it after anything that may reallocate
    /// the underlying vectors.
    #[cfg(feature = "native")]
    fn eval_context_from(context: &mut VmContext) -> crate::native::EvalContext {
        let integration = context.integration_coefficients();
        crate::native::EvalContext {
            voltages: context.voltages.as_ptr(),
            internal_voltages: context.internal_voltages.as_ptr(),
            params: context.parameters.as_ptr(),
            branch_currents: context.terminal_pair_currents_ptr(),
            branch_currents_len: context.terminal_pair_currents_len(),
            currents: context.currents.as_mut_ptr() as *const f64,
            currents_len: context.currents.len(),
            num_terminals: context.terminal_count(),
            port_connected: context.port_connected.as_ptr(),
            port_connected_len: context.port_connected.len(),
            temperature: context.temperature,
            time: context.time,
            timestep: context.timestep(),
            // Pass null for empty vecs - as_ptr() on empty vec gives dangling non-null pointer
            state_prev: if context.state_values_prev.is_empty() {
                std::ptr::null()
            } else {
                context.state_values_prev.as_ptr()
            },
            state_values: if context.state_values.is_empty() {
                std::ptr::null_mut()
            } else {
                context.state_values.as_mut_ptr()
            },
            state_initialized: if context.state_initialized.is_empty() {
                std::ptr::null_mut()
            } else {
                context.state_initialized.as_mut_ptr() as *mut u8
            },
            state_initialized_len: context.state_initialized.len(),
            lookup_tables: if context.lookup_tables.is_empty() {
                std::ptr::null()
            } else {
                context.lookup_tables.as_ptr()
            },
            lookup_tables_len: context.lookup_tables.len(),
            laplace_filters: if context.laplace_filters.is_empty() {
                std::ptr::null_mut()
            } else {
                context.laplace_filters.as_mut_ptr()
            },
            laplace_filters_len: context.laplace_filters.len(),
            param_given: context.param_given.as_ptr(),
            param_given_len: context.param_given.len(),
            branch_unknowns: if context.branch_current_values.is_empty() {
                std::ptr::null()
            } else {
                context.branch_current_values.as_ptr()
            },
            analysis_type: context.analysis_type,
            multiplicity: context.multiplicity,
            zi_filters: if context.zi_filters.is_empty() {
                std::ptr::null_mut()
            } else {
                context.zi_filters.as_mut_ptr()
            },
            zi_filters_len: context.zi_filters.len(),
            transition_filters: if context.transition_filters.is_empty() {
                std::ptr::null_mut()
            } else {
                context.transition_filters.as_mut_ptr()
            },
            transition_filters_len: context.transition_filters.len(),
            slew_filters: if context.slew_filters.is_empty() {
                std::ptr::null_mut()
            } else {
                context.slew_filters.as_mut_ptr()
            },
            slew_filters_len: context.slew_filters.len(),
            delay_buffers: if context.delay_buffers.is_empty() {
                std::ptr::null_mut()
            } else {
                context.delay_buffers.as_mut_ptr()
            },
            delay_buffers_len: context.delay_buffers.len(),
            cross_detectors: if context.cross_detectors.is_empty() {
                std::ptr::null_mut()
            } else {
                context.cross_detectors.as_mut_ptr()
            },
            cross_detectors_len: context.cross_detectors.len(),
            state_prev_len: context.state_values_prev.len(),
            state_values_len: context.state_values.len(),
            timer_event_bound: &mut context.timer_event_bound,
            analysis_initial_step: u8::from(context.analysis_initial_step),
            analysis_final_step: u8::from(context.analysis_final_step),
            state_older: context.state_values_older.as_ptr(),
            state_older_len: context.state_values_older.len(),
            state_derivatives: context.state_derivatives.as_mut_ptr(),
            state_derivatives_len: context.state_derivatives.len(),
            state_derivatives_prev: context.state_derivatives_prev.as_ptr(),
            state_derivatives_prev_len: context.state_derivatives_prev.len(),
            integration_derivative_scale: integration.derivative_scale,
            integration_previous_value_scale: integration.previous_value_scale,
            integration_older_value_scale: integration.older_value_scale,
            integration_previous_derivative_scale: integration.previous_derivative_scale,
            integration_active: u8::from(integration.active),
            limiter_active: &mut context.limiter_active,
            limiting_enabled: u8::from(context.evaluation_mode.limiting_enabled()),
            runtime_status: Default::default(),
            state_candidate_valid: if context.state_candidate_valid.is_empty() {
                std::ptr::null_mut()
            } else {
                context.state_candidate_valid.as_mut_ptr()
            },
            state_candidate_valid_len: context.state_candidate_valid.len(),
            state_older_candidate: if context.state_older_candidate.is_empty() {
                std::ptr::null_mut()
            } else {
                context.state_older_candidate.as_mut_ptr()
            },
            state_older_candidate_len: context.state_older_candidate.len(),
        }
    }

    #[cfg(feature = "native")]
    fn native_entry_dependencies<'a>(
        native: &'a NativeModel,
        entry: NativeValueEntry,
    ) -> Result<NativeEntryDependencies<'a>, VmError> {
        Ok(match entry {
            NativeValueEntry::ParameterDefault(_) => NativeEntryDependencies {
                current_pairs: &[],
                prior_currents: &[],
                branch_unknowns: &[],
            },
            NativeValueEntry::StaticCondition(index) => NativeEntryDependencies {
                current_pairs: &[],
                prior_currents: &[],
                branch_unknowns: native
                    .static_condition_branch_unknowns(index)
                    .ok_or_else(|| Self::missing_native_static_condition_entry(index))?,
            },
            NativeValueEntry::StampValue(index) => NativeEntryDependencies {
                current_pairs: native
                    .stamp_value_current_pairs(index)
                    .ok_or_else(|| Self::missing_native_stamp_value_entry(index))?,
                prior_currents: native
                    .stamp_value_prior_currents(index)
                    .ok_or_else(|| Self::missing_native_stamp_value_entry(index))?,
                branch_unknowns: native
                    .stamp_value_branch_unknowns(index)
                    .ok_or_else(|| Self::missing_native_stamp_value_entry(index))?,
            },
            NativeValueEntry::Jacobian { stamp, entry } => NativeEntryDependencies {
                current_pairs: native
                    .jacobian_current_pairs(stamp, entry)
                    .ok_or_else(|| Self::missing_native_jacobian_entry(stamp, entry))?,
                prior_currents: native
                    .jacobian_prior_currents(stamp, entry)
                    .ok_or_else(|| Self::missing_native_jacobian_entry(stamp, entry))?,
                branch_unknowns: native
                    .jacobian_branch_unknowns(stamp, entry)
                    .ok_or_else(|| Self::missing_native_jacobian_entry(stamp, entry))?,
            },
            NativeValueEntry::ReactiveJacobian { stamp, entry } => NativeEntryDependencies {
                current_pairs: native
                    .reactive_jacobian_current_pairs(stamp, entry)
                    .ok_or_else(|| Self::missing_native_reactive_jacobian_entry(stamp, entry))?,
                prior_currents: native
                    .reactive_jacobian_prior_currents(stamp, entry)
                    .ok_or_else(|| Self::missing_native_reactive_jacobian_entry(stamp, entry))?,
                branch_unknowns: native
                    .reactive_jacobian_branch_unknowns(stamp, entry)
                    .ok_or_else(|| Self::missing_native_reactive_jacobian_entry(stamp, entry))?,
            },
            NativeValueEntry::NoisePsd(index) => NativeEntryDependencies {
                current_pairs: native
                    .noise_psd_current_pairs(index)
                    .ok_or_else(|| Self::missing_native_noise_psd_entry(index))?,
                prior_currents: native
                    .noise_psd_prior_currents(index)
                    .ok_or_else(|| Self::missing_native_noise_psd_entry(index))?,
                branch_unknowns: native
                    .noise_psd_branch_unknowns(index)
                    .ok_or_else(|| Self::missing_native_noise_psd_entry(index))?,
            },
            NativeValueEntry::NoiseExponent(index) => NativeEntryDependencies {
                current_pairs: native
                    .noise_exponent_current_pairs(index)
                    .ok_or_else(|| Self::missing_native_noise_exponent_entry(index))?,
                prior_currents: native
                    .noise_exponent_prior_currents(index)
                    .ok_or_else(|| Self::missing_native_noise_exponent_entry(index))?,
                branch_unknowns: native
                    .noise_exponent_branch_unknowns(index)
                    .ok_or_else(|| Self::missing_native_noise_exponent_entry(index))?,
            },
        })
    }

    /// Run one value-returning native entry point.
    #[cfg(feature = "native")]
    fn run_value_program(
        vm: &mut Vm<'_>,
        _program: &crate::codegen::BytecodeProgram,
        native: &NativeModel,
        entry: NativeValueEntry,
    ) -> Result<f64, VmError> {
        Self::validate_native_storage(vm.context, native)?;
        let dependencies = Self::native_entry_dependencies(native, entry)?;
        Self::validate_native_current_pairs(
            vm.context,
            native.num_terminals,
            dependencies.current_pairs,
        )?;
        Self::validate_native_prior_currents(vm.context, dependencies.prior_currents)?;
        Self::validate_native_branch_unknowns(vm.context, dependencies.branch_unknowns)?;

        let ctx = Self::eval_context_from(vm.context);
        let vars_ptr = vm.context.variables.as_ptr();
        ctx.clear_runtime_error();
        let value = match entry {
            NativeValueEntry::ParameterDefault(index) => native
                .run_parameter_default(index, &ctx, vars_ptr)
                .ok_or_else(|| Self::missing_native_parameter_default_entry(index))?,
            NativeValueEntry::StaticCondition(index) => native
                .run_static_condition(index, &ctx, vars_ptr)
                .ok_or_else(|| Self::missing_native_static_condition_entry(index))?,
            NativeValueEntry::StampValue(index) => native
                .run_stamp_value(index, &ctx, vars_ptr)
                .ok_or_else(|| Self::missing_native_stamp_value_entry(index))?,
            NativeValueEntry::Jacobian { stamp, entry } => native
                .run_jacobian(stamp, entry, &ctx, vars_ptr)
                .ok_or_else(|| Self::missing_native_jacobian_entry(stamp, entry))?,
            NativeValueEntry::ReactiveJacobian { stamp, entry } => native
                .run_reactive_jacobian(stamp, entry, &ctx, vars_ptr)
                .ok_or_else(|| Self::missing_native_reactive_jacobian_entry(stamp, entry))?,
            NativeValueEntry::NoisePsd(index) => native
                .run_noise_psd(index, &ctx, vars_ptr)
                .ok_or_else(|| Self::missing_native_noise_psd_entry(index))?,
            NativeValueEntry::NoiseExponent(index) => native
                .run_noise_exponent(index, &ctx, vars_ptr)
                .ok_or_else(|| Self::missing_native_noise_exponent_entry(index))?,
        };
        if let Some(error) = ctx.take_native_runtime_error() {
            return Err(Self::native_runtime_error_to_vm(error));
        }
        Ok(value)
    }

    #[cfg(feature = "native")]
    fn validate_native_current_pairs(
        context: &VmContext,
        compiled_terminal_count: usize,
        current_pairs: &[usize],
    ) -> Result<(), VmError> {
        if current_pairs.is_empty() {
            return Ok(());
        }

        let terminal_count = context.terminal_count();
        if terminal_count != compiled_terminal_count {
            return Err(Self::mismatched_native_terminal_context(
                compiled_terminal_count,
                terminal_count,
            ));
        }
        if compiled_terminal_count == 0 {
            return Err(Self::missing_native_terminal_pair_current_slot(0));
        }
        for pair_index in current_pairs {
            let Some((pos, neg)) =
                terminal_pair_current_endpoints(*pair_index, compiled_terminal_count)
            else {
                return Err(Self::missing_native_terminal_pair_current_slot(*pair_index));
            };
            context
                .try_current(pos, neg)
                .map_err(|_| Self::missing_native_terminal_pair_current_slot(*pair_index))?;
        }

        Ok(())
    }

    #[cfg(feature = "native")]
    fn validate_native_current_pair_storage(
        context: &VmContext,
        compiled_terminal_count: usize,
        current_pairs: &[usize],
    ) -> Result<(), VmError> {
        if current_pairs.is_empty() {
            return Ok(());
        }

        let terminal_count = context.terminal_count();
        if terminal_count != compiled_terminal_count {
            return Err(Self::mismatched_native_terminal_context(
                compiled_terminal_count,
                terminal_count,
            ));
        }
        let available = context.terminal_pair_currents_len();
        for pair_index in current_pairs {
            if terminal_pair_current_endpoints(*pair_index, compiled_terminal_count).is_none()
                || *pair_index >= available
            {
                return Err(Self::missing_native_terminal_pair_current_slot(*pair_index));
            }
        }

        Ok(())
    }

    #[cfg(feature = "native")]
    fn validate_native_terminal_pair_table(
        context: &VmContext,
        compiled_terminal_count: usize,
    ) -> Result<(), VmError> {
        let terminal_count = context.terminal_count();
        if terminal_count != compiled_terminal_count {
            return Err(Self::mismatched_native_terminal_context(
                compiled_terminal_count,
                terminal_count,
            ));
        }
        let expected = terminal_pair_current_len(compiled_terminal_count).ok_or_else(|| {
            VmError::NativeJit(
                "native terminal-pair current table dimensions overflow; no interpreter fallback"
                    .into(),
            )
        })?;
        let available = context.terminal_pair_currents_len();
        if available != expected {
            return Err(Self::missing_native_terminal_pair_current_slot(
                available.min(expected.saturating_sub(1)),
            ));
        }
        Ok(())
    }

    #[cfg(feature = "native")]
    fn validate_native_storage(context: &VmContext, native: &NativeModel) -> Result<(), VmError> {
        Self::validate_native_voltage_storage(
            context,
            native.num_terminals,
            native.num_internal_nodes,
        )?;
        Self::validate_native_parameter_storage(context, native.num_parameters)?;
        Self::validate_native_variable_storage(context, native.num_variables)?;
        Self::validate_native_runtime_storage(context, native.required_storage())
    }

    #[cfg(feature = "native")]
    fn validate_native_voltage_storage(
        context: &VmContext,
        required_terminals: usize,
        required_internal_nodes: usize,
    ) -> Result<(), VmError> {
        if context.voltages.len() < required_terminals {
            return Err(Self::missing_native_voltage_storage(
                required_terminals,
                context.voltages.len(),
            ));
        }
        if context.voltages.len() != required_terminals {
            return Err(Self::mismatched_native_terminal_context(
                required_terminals,
                context.voltages.len(),
            ));
        }
        if context.internal_voltages.len() < required_internal_nodes {
            return Err(Self::missing_native_internal_voltage_storage(
                required_internal_nodes,
                context.internal_voltages.len(),
            ));
        }
        if context.port_connected.len() < required_terminals {
            return Err(Self::missing_native_port_connected_storage(
                required_terminals,
                context.port_connected.len(),
            ));
        }

        Ok(())
    }

    #[cfg(feature = "native")]
    fn validate_native_parameter_storage(
        context: &VmContext,
        required: usize,
    ) -> Result<(), VmError> {
        if context.parameters.len() < required {
            return Err(Self::missing_native_parameter_storage(
                required,
                context.parameters.len(),
            ));
        }
        if context.param_given.len() < required {
            return Err(Self::missing_native_param_given_storage(
                required,
                context.param_given.len(),
            ));
        }

        Ok(())
    }

    #[cfg(feature = "native")]
    fn validate_native_variable_storage(
        context: &VmContext,
        required: usize,
    ) -> Result<(), VmError> {
        if context.variables.len() < required {
            return Err(Self::missing_native_variable_storage(
                required,
                context.variables.len(),
            ));
        }

        Ok(())
    }

    #[cfg(feature = "native")]
    fn validate_native_runtime_storage(
        context: &VmContext,
        required: NativeRequiredStorage,
    ) -> Result<(), VmError> {
        Self::validate_native_runtime_storage_len(
            "state-value storage",
            required.state_values,
            context.state_values.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "prior state-value storage",
            required.state_values_prev,
            context.state_values_prev.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "older state-value storage",
            required.state_values,
            context.state_values_older.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "candidate state-derivative storage",
            required.state_values,
            context.state_derivatives.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "prior state-derivative storage",
            required.state_values,
            context.state_derivatives_prev.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "state-initialization flag storage",
            required.state_initialized,
            context.state_initialized.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "integration candidate-valid storage",
            required.state_candidate_valid,
            context.state_candidate_valid.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "integration older-candidate storage",
            required.state_older_candidate,
            context.state_older_candidate.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "lookup-table storage",
            required.lookup_tables,
            context.lookup_tables.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "Laplace filter storage",
            required.laplace_filters,
            context.laplace_filters.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "zi filter storage",
            required.zi_filters,
            context.zi_filters.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "transition filter storage",
            required.transition_filters,
            context.transition_filters.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "slew filter storage",
            required.slew_filters,
            context.slew_filters.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "delay-buffer storage",
            required.delay_buffers,
            context.delay_buffers.len(),
        )?;
        Self::validate_native_runtime_storage_len(
            "cross-detector storage",
            required.cross_detectors,
            context.cross_detectors.len(),
        )
    }

    #[cfg(feature = "native")]
    fn validate_native_runtime_storage_len(
        label: &str,
        required: usize,
        available: usize,
    ) -> Result<(), VmError> {
        if available < required {
            return Err(Self::missing_native_runtime_storage(
                label, required, available,
            ));
        }

        Ok(())
    }

    #[cfg(feature = "native")]
    fn validate_native_prior_currents(
        context: &VmContext,
        prior_currents: &[usize],
    ) -> Result<(), VmError> {
        for current_index in prior_currents {
            if *current_index >= context.currents.len() {
                return Err(VmError::NativeJit(format!(
                    "native stamp requires prior contribution current {current_index}, but only {} current(s) have been evaluated; no interpreter fallback",
                    context.currents.len()
                )));
            }
        }

        Ok(())
    }

    #[cfg(feature = "native")]
    fn validate_native_branch_unknowns(
        context: &VmContext,
        branch_unknowns: &[usize],
    ) -> Result<(), VmError> {
        for index in branch_unknowns {
            if *index >= context.branch_current_values.len() {
                return Err(Self::missing_native_branch_current_unknown(
                    *index,
                    context.branch_current_values.len(),
                ));
            }
        }

        Ok(())
    }

    /// Dispatch one value entry through the required worker-installed module.
    #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
    fn run_value_program(
        vm: &mut Vm<'_>,
        _program: &crate::codegen::BytecodeProgram,
        wasm: &WasmJitExecutable,
        entry: WasmJitExecutableEntry,
    ) -> Result<f64, VmError> {
        wasm.run_entry(entry, vm.context).map_err(VmError::WasmJit)
    }

    /// Run one value-returning bytecode program.
    #[cfg(all(
        not(feature = "native"),
        not(all(feature = "wasm-jit", target_arch = "wasm32"))
    ))]
    fn run_value_program(
        vm: &mut Vm<'_>,
        program: &crate::codegen::BytecodeProgram,
    ) -> Result<f64, VmError> {
        vm.execute(program)
    }

    /// Execute the assignment pass through the required native entry point.
    #[cfg(feature = "native")]
    fn run_assignment_pass(
        vm: &mut Vm<'_>,
        model: &CompiledModel,
        native: &NativeModel,
    ) -> Result<(), VmError> {
        if vm.context.variables.len() < model.num_variables {
            vm.context.variables.resize(model.num_variables, 0.0);
        }
        Self::validate_native_storage(vm.context, native)?;
        // Assignment loads can be guarded by runtime control flow. Validate
        // their memory contract here without rejecting a currently-unpublished
        // value that the generated code may never read.
        Self::validate_native_current_pair_storage(
            vm.context,
            native.num_terminals,
            native.assignment_current_pairs(),
        )?;
        Self::validate_native_prior_currents(vm.context, native.assignment_prior_currents())?;
        Self::validate_native_branch_unknowns(vm.context, native.assignment_branch_unknowns())?;
        let ctx = Self::eval_context_from(vm.context);
        let vars_ptr = vm.context.variables.as_mut_ptr();
        ctx.clear_runtime_error();
        native.run_assignments(&ctx, vars_ptr);
        if let Some(error) = ctx.take_native_runtime_error() {
            return Err(Self::native_runtime_error_to_vm(error));
        }
        Ok(())
    }

    /// Execute assignments that consume the completed contribution-current
    /// vector. These entries are never run while contribution slots are still
    /// being populated.
    #[cfg(feature = "native")]
    fn run_post_assignment_pass(
        vm: &mut Vm<'_>,
        model: &CompiledModel,
        native: &NativeModel,
    ) -> Result<(), VmError> {
        if native.plan_stats().assignment_entry_points == 1 {
            return Ok(());
        }
        if vm.context.variables.len() < model.num_variables {
            vm.context.variables.resize(model.num_variables, 0.0);
        }
        Self::validate_native_storage(vm.context, native)?;
        Self::validate_native_current_pairs(
            vm.context,
            native.num_terminals,
            native.post_assignment_current_pairs(),
        )?;
        Self::validate_native_prior_currents(vm.context, native.post_assignment_prior_currents())?;
        Self::validate_native_branch_unknowns(
            vm.context,
            native.post_assignment_branch_unknowns(),
        )?;
        let ctx = Self::eval_context_from(vm.context);
        let vars_ptr = vm.context.variables.as_mut_ptr();
        ctx.clear_runtime_error();
        if !native.run_post_assignments(&ctx, vars_ptr) {
            return Err(VmError::NativeJit(
                "native JIT image is missing its post-current assignment entry; no interpreter fallback"
                    .into(),
            ));
        }
        if let Some(error) = ctx.take_native_runtime_error() {
            return Err(Self::native_runtime_error_to_vm(error));
        }
        Ok(())
    }

    #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
    fn run_assignment_pass(
        vm: &mut Vm<'_>,
        model: &CompiledModel,
        wasm: &WasmJitExecutable,
    ) -> Result<(), VmError> {
        if vm.context.variables.len() < model.num_variables {
            vm.context.variables.resize(model.num_variables, 0.0);
        }
        wasm.run_assignments(vm.context).map_err(VmError::WasmJit)
    }

    #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
    fn run_post_assignment_pass(
        vm: &mut Vm<'_>,
        model: &CompiledModel,
        wasm: &WasmJitExecutable,
    ) -> Result<(), VmError> {
        if vm.context.variables.len() < model.num_variables {
            vm.context.variables.resize(model.num_variables, 0.0);
        }
        wasm.run_post_assignments(vm.context)
            .map_err(VmError::WasmJit)
    }

    /// Execute the assignment pass through the bytecode interpreter.
    #[cfg(all(
        not(feature = "native"),
        not(all(feature = "wasm-jit", target_arch = "wasm32"))
    ))]
    fn run_assignment_pass(vm: &mut Vm<'_>, model: &CompiledModel) -> Result<(), VmError> {
        if vm.context.variables.len() < model.num_variables {
            vm.context.variables.resize(model.num_variables, 0.0);
        }
        Self::execute_assignment_steps(vm, &model.assignment_steps)
    }

    #[cfg(feature = "native")]
    fn populate_noise_current_probe_cache(
        vm: &mut Vm<'_>,
        model: &CompiledModel,
        program_active: &[bool],
        native: &NativeModel,
    ) -> Result<(), VmError> {
        for (program_idx, program) in model.stamp_programs.iter().enumerate() {
            let active = program_active.get(program_idx).copied().unwrap_or(true);
            let value = if active {
                let value = Self::run_value_program(
                    vm,
                    &program.value_program,
                    native,
                    NativeValueEntry::StampValue(program_idx),
                )?;
                Self::finite_result(
                    value,
                    format!("contribution {program_idx} during noise evaluation"),
                )?
            } else {
                0.0
            };
            Self::cache_current_probe_value(vm.context, program, value, active);
        }
        Ok(())
    }

    #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
    fn populate_noise_current_probe_cache(
        vm: &mut Vm<'_>,
        model: &CompiledModel,
        program_active: &[bool],
        wasm: &WasmJitExecutable,
    ) -> Result<(), VmError> {
        for (program_idx, program) in model.stamp_programs.iter().enumerate() {
            let active = program_active.get(program_idx).copied().unwrap_or(true);
            let value = if active {
                let value = Self::run_value_program(
                    vm,
                    &program.value_program,
                    wasm,
                    WasmJitExecutableEntry::StampValue(program_idx),
                )?;
                Self::finite_result(
                    value,
                    format!("contribution {program_idx} during noise evaluation"),
                )?
            } else {
                0.0
            };
            Self::cache_current_probe_value(vm.context, program, value, active);
        }
        Ok(())
    }

    #[cfg(all(
        not(feature = "native"),
        not(all(feature = "wasm-jit", target_arch = "wasm32"))
    ))]
    fn populate_noise_current_probe_cache(
        vm: &mut Vm<'_>,
        model: &CompiledModel,
        program_active: &[bool],
    ) -> Result<(), VmError> {
        for (program_idx, program) in model.stamp_programs.iter().enumerate() {
            let active = program_active.get(program_idx).copied().unwrap_or(true);
            let value = if active {
                let value = vm.execute(&program.value_program)?;
                Self::finite_result(
                    value,
                    format!("contribution {program_idx} during noise evaluation"),
                )?
            } else {
                0.0
            };
            Self::cache_current_probe_value(vm.context, program, value, active);
        }
        Ok(())
    }

    fn cache_current_probe_value(
        context: &mut VmContext,
        program: &crate::codegen::StampProgram,
        value: f64,
        active: bool,
    ) {
        context.currents.push(value);
        if active
            && program.branch_ordinal.is_none()
            && let Some((pos, neg)) = Self::infer_current_terminal_pair(program)
        {
            context.set_branch_current(pos, neg, value);
        }
    }

    /// Safety cap on runtime-loop iterations per evaluation (a model bug
    /// must not hang the Newton loop)
    #[cfg(all(
        not(feature = "native"),
        not(all(feature = "wasm-jit", target_arch = "wasm32"))
    ))]
    const MAX_RUNTIME_LOOP_ITERATIONS: usize = 100_000;

    /// Execute assignment programs and update VM variable storage.
    #[cfg(all(
        not(feature = "native"),
        not(all(feature = "wasm-jit", target_arch = "wasm32"))
    ))]
    fn execute_assignment_programs(vm: &mut Vm<'_>, model: &CompiledModel) -> Result<(), VmError> {
        if vm.context.variables.len() < model.num_variables {
            vm.context.variables.resize(model.num_variables, 0.0);
        }

        Self::execute_assignment_steps(vm, &model.assignment_steps)
    }

    /// Execute a sequence of evaluation steps (assignments and runtime
    /// loops), recursively
    #[cfg(all(
        not(feature = "native"),
        not(all(feature = "wasm-jit", target_arch = "wasm32"))
    ))]
    fn execute_assignment_steps(
        vm: &mut Vm<'_>,
        steps: &[crate::codegen::AssignmentStep],
    ) -> Result<(), VmError> {
        for step in steps {
            match step {
                crate::codegen::AssignmentStep::Assign(assignment) => {
                    if assignment.var_index >= vm.context.variables.len() {
                        return Err(VmError::InvalidInstruction(
                            "assignment target variable is outside runtime storage",
                        ));
                    }
                    let value = vm.execute(&assignment.program)?;
                    vm.context.variables[assignment.var_index] = value;
                }
                crate::codegen::AssignmentStep::AssignIndexed {
                    base,
                    len,
                    lower,
                    index,
                    value,
                } => {
                    let slot = vm
                        .execute(index)
                        .and_then(|raw| Vm::array_slot(raw, *base, *len, *lower));
                    let slot = slot?;
                    if slot >= vm.context.variables.len() {
                        return Err(VmError::InvalidInstruction(
                            "indexed assignment target is outside runtime storage",
                        ));
                    }
                    let value = vm.execute(value)?;
                    vm.context.variables[slot] = value;
                }
                crate::codegen::AssignmentStep::Loop { condition, body } => {
                    let mut iterations = 0usize;
                    loop {
                        let active = vm.execute(condition)?;
                        if active == 0.0 {
                            break;
                        }
                        Self::execute_assignment_steps(vm, body)?;
                        iterations += 1;
                        if iterations >= Self::MAX_RUNTIME_LOOP_ITERATIONS {
                            return Err(VmError::InvalidInstruction(
                                "runtime loop iteration limit exceeded",
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Compute Jacobian entries
    ///
    /// Returns (value, row_terminal, col_terminal, is_current) for each derivative.
    pub fn compute_jacobian(&mut self) -> Vec<JacobianEntry> {
        self.try_compute_jacobian().unwrap_or_else(|err| {
            panic!(
                "Verilog-A device '{}' model '{}' Jacobian evaluation failed: {}",
                self.name, self.model.name, err
            )
        })
    }

    /// Checked Jacobian evaluation path for callers that can surface
    /// runtime model errors as diagnostics instead of panicking.
    pub fn try_compute_jacobian(&mut self) -> Result<Vec<JacobianEntry>, VmError> {
        // A standalone Jacobian query belongs to the current nonlinear
        // evaluation and must not erase the convergence result established by
        // its value pass or advance limiter history a second time. Canonical
        // limiter Jacobians use the oriented proposal directly, so bypassing
        // candidate publication here preserves that contract.
        self.context.evaluation_mode = crate::vm::VerilogAEvaluationMode::StaticProbe;
        let context = &mut self.context;
        let model = &self.model;
        #[cfg(feature = "native")]
        let native = self.native_model.as_ref();
        #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
        let wasm = self.wasm_jit_model.as_ref();

        context.clear_currents();
        context.currents.reserve(model.stamp_programs.len());

        let program_active = &self.program_active;
        let mut vm = Vm::new(context);
        Self::run_assignment_pass(
            &mut vm,
            model,
            #[cfg(feature = "native")]
            native,
            #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
            wasm,
        )?;
        let mut entries = Vec::new();

        for (prog_idx, program) in model.stamp_programs.iter().enumerate() {
            if !program_active.get(prog_idx).copied().unwrap_or(true) {
                vm.context.currents.push(0.0);
                continue;
            }
            let value = Self::run_value_program(
                &mut vm,
                &program.value_program,
                #[cfg(feature = "native")]
                native,
                #[cfg(feature = "native")]
                NativeValueEntry::StampValue(prog_idx),
                #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
                wasm,
                #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
                WasmJitExecutableEntry::StampValue(prog_idx),
            )?;
            let value = Self::finite_result(
                value,
                format!("contribution {prog_idx} during Jacobian evaluation"),
            )?;
            vm.context.currents.push(value);
            if program.branch_ordinal.is_none()
                && let Some((pos, neg)) = Self::infer_current_terminal_pair(program)
            {
                vm.context.set_branch_current(pos, neg, value);
            }

            for (jac_idx, jac_entry) in program.jacobian_programs.iter().enumerate() {
                let value = Self::run_value_program(
                    &mut vm,
                    &jac_entry.program,
                    #[cfg(feature = "native")]
                    native,
                    #[cfg(feature = "native")]
                    NativeValueEntry::Jacobian {
                        stamp: prog_idx,
                        entry: jac_idx,
                    },
                    #[cfg(all(
                        not(feature = "native"),
                        feature = "wasm-jit",
                        target_arch = "wasm32"
                    ))]
                    wasm,
                    #[cfg(all(
                        not(feature = "native"),
                        feature = "wasm-jit",
                        target_arch = "wasm32"
                    ))]
                    WasmJitExecutableEntry::Jacobian {
                        stamp: prog_idx,
                        entry: jac_idx,
                    },
                )?;
                let value = Self::finite_result(value, format!("Jacobian {prog_idx}:{jac_idx}"))?;
                entries.push(JacobianEntry {
                    value: jac_entry.sign * value,
                    row: jac_entry.row.clone(),
                    col: jac_entry.col.clone(),
                    program_idx: prog_idx,
                    jacobian_idx: jac_idx,
                });
            }
        }
        #[cfg(feature = "native")]
        Self::run_post_assignment_pass(&mut vm, model, native)?;
        #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
        Self::run_post_assignment_pass(&mut vm, model, wasm)?;

        Ok(entries)
    }

    /// Stamp device into matrix and RHS
    ///
    /// This is the main interface for circuit simulation.
    ///
    /// # Arguments
    /// * `matrix_add` - Callback to add value at (row, col) in circuit matrix
    /// * `rhs_add` - Callback to add value at (node) in RHS vector
    /// * `circuit_voltages` - Current voltage solution
    pub fn stamp<M, R>(&mut self, circuit_voltages: &[f64], mut matrix_add: M, mut rhs_add: R)
    where
        M: FnMut(usize, usize, f64),
        R: FnMut(usize, f64),
    {
        if let Err(err) = self.try_stamp(circuit_voltages, &mut matrix_add, &mut rhs_add) {
            panic!(
                "Verilog-A device '{}' model '{}' stamping failed: {}",
                self.name, self.model.name, err
            );
        }
    }

    /// Checked stamping path for callers that can turn Verilog-A runtime
    /// faults into simulator diagnostics.
    pub fn try_stamp<M, R>(
        &mut self,
        circuit_voltages: &[f64],
        matrix_add: M,
        rhs_add: R,
    ) -> Result<(), VmError>
    where
        M: FnMut(usize, usize, f64),
        R: FnMut(usize, f64),
    {
        let mode =
            crate::vm::VerilogAEvaluationMode::default_for_analysis(self.context.analysis_type);
        self.try_stamp_with_mode(circuit_voltages, matrix_add, rhs_add, mode)
    }

    fn buffer_structural_branches(
        model: &CompiledModel,
        branch_active: &[bool],
        node_mapping: &[usize],
        internal_node_indices: &[usize],
        branch_current_indices: &[usize],
        matrix_buffer: &mut Vec<(usize, usize, f64)>,
        multiplicity: f64,
    ) {
        for (ordinal, source) in model.branch_sources.iter().enumerate() {
            let br = Self::index_to_node(
                &StampIndex::Branch(ordinal),
                node_mapping,
                internal_node_indices,
                branch_current_indices,
            );
            let Some(br) = br else { continue };

            if !branch_active.get(ordinal).copied().unwrap_or(false) {
                matrix_buffer.push((br, br, 1.0));
                continue;
            }

            let pos = Self::index_to_node(
                &source.pos,
                node_mapping,
                internal_node_indices,
                branch_current_indices,
            );
            let neg = Self::index_to_node(
                &source.neg,
                node_mapping,
                internal_node_indices,
                branch_current_indices,
            );

            if let Some(p) = pos {
                matrix_buffer.push((p, br, multiplicity));
                if !source.indirect {
                    matrix_buffer.push((br, p, 1.0));
                }
            }
            if let Some(n) = neg {
                matrix_buffer.push((n, br, -multiplicity));
                if !source.indirect {
                    matrix_buffer.push((br, n, -1.0));
                }
            }
        }
    }

    /// Checked stamping with an explicit named-limiter evaluation policy.
    pub fn try_stamp_with_mode<M, R>(
        &mut self,
        circuit_voltages: &[f64],
        matrix_add: M,
        rhs_add: R,
        mode: crate::vm::VerilogAEvaluationMode,
    ) -> Result<(), VmError>
    where
        M: FnMut(usize, usize, f64),
        R: FnMut(usize, f64),
    {
        #[cfg(any(feature = "native", all(feature = "wasm-jit", target_arch = "wasm32")))]
        if self.fused_stamp_driver_is_active() {
            return self.try_stamp_fused_kernel(circuit_voltages, matrix_add, rhs_add, mode);
        }

        self.try_stamp_scalar_with_mode(circuit_voltages, matrix_add, rhs_add, mode)
    }

    /// Whether stamping dispatches the fused whole-model driver rather than
    /// one call per contribution and per derivative.
    ///
    /// Public because the browser qualification gate asserts it: a probe that
    /// quietly fell back to the per-entry path would still produce the right
    /// numbers, and would report a fused dispatch it never made.
    pub fn fused_stamp_driver_is_active(&self) -> bool {
        #[cfg(feature = "native")]
        {
            self.native_model.stamp_kernel_is_eligible()
        }
        #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
        {
            self.wasm_jit_model.stamp_kernel_is_eligible()
        }
        #[cfg(all(
            not(feature = "native"),
            not(all(feature = "wasm-jit", target_arch = "wasm32"))
        ))]
        {
            false
        }
    }

    /// Stamp through a fused whole-model driver.
    ///
    /// One dispatch evaluates the assignment pass, every contribution and
    /// every Jacobian entry. Only the dispatch is backend-specific: both
    /// backends publish contributions into the context and write the same
    /// flat, model-order Jacobian array, so the algebra that decides what the
    /// solver actually sees exists once. The backends already share their
    /// fusion predicate; sharing this too is what keeps that agreement
    /// meaningful.
    #[cfg(any(feature = "native", all(feature = "wasm-jit", target_arch = "wasm32")))]
    fn try_stamp_fused_kernel<M, R>(
        &mut self,
        circuit_voltages: &[f64],
        mut matrix_add: M,
        mut rhs_add: R,
        mode: crate::vm::VerilogAEvaluationMode,
    ) -> Result<(), VmError>
    where
        M: FnMut(usize, usize, f64),
        R: FnMut(usize, f64),
    {
        self.try_update_all_voltages(circuit_voltages)?;
        self.begin_evaluation(mode);
        self.stamp_matrix_buffer.clear();
        self.stamp_rhs_buffer.clear();

        let model = &self.model;
        let stamp_count = model.stamp_programs.len();

        #[cfg(feature = "native")]
        {
            let native = self.native_model.as_ref();
            let expected_jacobians = native.plan_stats().jacobian_entry_points;
            if self.fused_program_active.len() != stamp_count
                || self.fused_stamp_jacobians.len() != expected_jacobians
            {
                return Err(VmError::NativeJit(format!(
                    "native fused-stamp buffers do not match compiled model shape ({}/{stamp_count} active flags, {}/{expected_jacobians} Jacobians); no interpreter fallback",
                    self.fused_program_active.len(),
                    self.fused_stamp_jacobians.len()
                )));
            }

            let context = &mut self.context;
            context.prepare_indexed_currents(stamp_count);
            if context.variables.len() < model.num_variables {
                context.variables.resize(model.num_variables, 0.0);
            }
            Self::validate_native_storage(context, native)?;
            Self::validate_native_terminal_pair_table(context, native.num_terminals)?;
            Self::validate_native_prior_currents(context, native.assignment_prior_currents())?;
            Self::validate_native_branch_unknowns(context, native.assignment_branch_unknowns())?;
            Self::validate_native_branch_unknowns(context, native.stamp_kernel_branch_unknowns())?;

            self.fused_stamp_jacobians.fill(0.0);
            let ctx = Self::eval_context_from(context);
            let io = NativeStampKernelIo {
                program_active: self.fused_program_active.as_ptr(),
                jacobians: self.fused_stamp_jacobians.as_mut_ptr(),
            };
            let vars = context.variables.as_mut_ptr();
            ctx.clear_runtime_error();
            if !native.run_stamp_kernel(&ctx, vars, &io) {
                return Err(VmError::NativeJit(
                    "native JIT image is missing its fused stamp entry; no interpreter fallback"
                        .into(),
                ));
            }
            if let Some(error) = ctx.take_native_runtime_error() {
                return Err(Self::native_runtime_error_to_vm(error));
            }
        }

        #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
        {
            let expected_jacobians = model
                .stamp_programs
                .iter()
                .map(|stamp| stamp.jacobian_programs.len())
                .sum::<usize>();
            if self.fused_program_active.len() != stamp_count
                || self.fused_stamp_jacobians.len() != expected_jacobians
            {
                return Err(VmError::WasmJit(format!(
                    "browser fused-stamp buffers do not match compiled model shape ({}/{stamp_count} active flags, {}/{expected_jacobians} Jacobians); no interpreter fallback",
                    self.fused_program_active.len(),
                    self.fused_stamp_jacobians.len()
                )));
            }

            self.context.prepare_indexed_currents(stamp_count);
            if self.context.variables.len() < model.num_variables {
                self.context.variables.resize(model.num_variables, 0.0);
            }

            self.fused_stamp_jacobians.fill(0.0);
            let wasm = std::sync::Arc::clone(&self.wasm_jit_model);
            let fused = wasm
                .run_fused_kernel(
                    &mut self.context,
                    &self.fused_program_active,
                    Some(&mut self.fused_stamp_jacobians),
                )
                .map_err(VmError::WasmJit)?;
            if !fused {
                return Err(VmError::WasmJit(
                    "browser JIT module is missing its fused stamp entry; no interpreter fallback"
                        .into(),
                ));
            }
        }

        // Validate every driver result before any solver callback observes it,
        // then publish contribution currents for later simulator APIs.
        let mut jacobian_base = 0usize;
        for (program_idx, program) in model.stamp_programs.iter().enumerate() {
            if self.fused_program_active[program_idx] != 0 {
                let value = Self::finite_stamp_value(
                    self.context.currents[program_idx],
                    program_idx,
                    None,
                    "contribution",
                )?;
                self.context.currents[program_idx] = value;
                if program.branch_ordinal.is_none()
                    && let Some((pos, neg)) = Self::infer_current_terminal_pair(program)
                {
                    self.context.set_branch_current(pos, neg, value);
                }
                for entry_idx in 0..program.jacobian_programs.len() {
                    Self::finite_stamp_value(
                        self.fused_stamp_jacobians[jacobian_base + entry_idx],
                        program_idx,
                        Some(entry_idx),
                        "Jacobian",
                    )?;
                }
            }
            jacobian_base += program.jacobian_programs.len();
        }
        {
            let mut vm = Vm::new(&mut self.context);
            #[cfg(feature = "native")]
            Self::run_post_assignment_pass(&mut vm, model, self.native_model.as_ref())?;
            #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
            Self::run_post_assignment_pass(&mut vm, model, self.wasm_jit_model.as_ref())?;
        }

        let m = self.context.multiplicity;
        Self::buffer_structural_branches(
            model,
            &self.branch_active,
            &self.node_mapping,
            &self.internal_node_indices,
            &self.branch_current_indices,
            &mut self.stamp_matrix_buffer,
            m,
        );

        jacobian_base = 0;
        for (program_idx, program) in model.stamp_programs.iter().enumerate() {
            if self.fused_program_active[program_idx] == 0 {
                jacobian_base += program.jacobian_programs.len();
                continue;
            }

            let scale = if program.branch_ordinal.is_none() {
                m
            } else {
                1.0
            };
            let value = self.context.currents[program_idx];
            let mut eq_value =
                Self::finite_stamp_value(value * scale, program_idx, None, "scaled contribution")?;

            for jacobian_entry in &self.matrix_indices.jacobian[program_idx] {
                let model_entry = &program.jacobian_programs[jacobian_entry.jacobian_idx];
                let deriv = Self::finite_stamp_value(
                    self.fused_stamp_jacobians[jacobian_base + jacobian_entry.jacobian_idx] * scale,
                    program_idx,
                    Some(jacobian_entry.jacobian_idx),
                    "Jacobian",
                )?;

                match (program.branch_ordinal, program.indirect) {
                    (None, _) | (Some(_), true) => {
                        if model_entry.sign > 0.0 {
                            let x_col = Self::axis_value(&self.context, &model_entry.col_axis);
                            eq_value -= deriv * x_col;
                        }
                    }
                    (Some(_), false) => {
                        let x_col = Self::axis_value(&self.context, &model_entry.col_axis);
                        eq_value += model_entry.sign * deriv * x_col;
                    }
                }

                if let (Some(row), Some(col)) = (jacobian_entry.row, jacobian_entry.col) {
                    self.stamp_matrix_buffer
                        .push((row, col, jacobian_entry.sign * deriv));
                }
            }

            eq_value = Self::finite_stamp_value(
                eq_value,
                program_idx,
                None,
                "equivalent source for contribution",
            )?;
            for entry in &self.matrix_indices.rhs[program_idx] {
                if let Some(row) = entry.node {
                    self.stamp_rhs_buffer.push((row, entry.sign * eq_value));
                }
            }
            jacobian_base += program.jacobian_programs.len();
        }
        for &(row, col, value) in &self.stamp_matrix_buffer {
            matrix_add(row, col, value);
        }
        for &(row, value) in &self.stamp_rhs_buffer {
            rhs_add(row, value);
        }
        self.stamp_matrix_buffer.clear();
        self.stamp_rhs_buffer.clear();
        Ok(())
    }

    fn try_stamp_scalar_with_mode<M, R>(
        &mut self,
        circuit_voltages: &[f64],
        mut matrix_add: M,
        mut rhs_add: R,
        mode: crate::vm::VerilogAEvaluationMode,
    ) -> Result<(), VmError>
    where
        M: FnMut(usize, usize, f64),
        R: FnMut(usize, f64),
    {
        // Update context with the full solution (terminals, internal
        // nodes, and branch-current unknowns)
        self.try_update_all_voltages(circuit_voltages)?;
        self.begin_evaluation(mode);
        self.stamp_matrix_buffer.clear();
        self.stamp_rhs_buffer.clear();

        // Extract disjoint fields to satisfy borrow checker
        let context = &mut self.context;
        let model = &self.model;
        let matrix_indices = &self.matrix_indices;
        #[cfg(feature = "native")]
        let native = self.native_model.as_ref();
        #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
        let wasm = self.wasm_jit_model.as_ref();
        // Instance multiplicity: m parallel copies scale every flow
        // (current) stamp by m; potential and constraint rows stay
        // per-copy, as do probed currents and internal node voltages
        let m = context.multiplicity;

        context.clear_currents();
        // Native snapshots hold a raw pointer into `currents` while values
        // push; pre-reserve so it never reallocates mid-pass
        context.currents.reserve(model.stamp_programs.len());

        let mut vm = Vm::new(context);
        Self::run_assignment_pass(
            &mut vm,
            model,
            #[cfg(feature = "native")]
            native,
            #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
            wasm,
        )?;

        // Buffer structural branch stamps with the computed entries. Nothing
        // is visible to the solver until every native/interpreter result and
        // post-current assignment has validated.
        Self::buffer_structural_branches(
            model,
            &self.branch_active,
            &self.node_mapping,
            &self.internal_node_indices,
            &self.branch_current_indices,
            &mut self.stamp_matrix_buffer,
            m,
        );

        for (program_idx, program) in model.stamp_programs.iter().enumerate() {
            if !self
                .program_active
                .get(program_idx)
                .copied()
                .unwrap_or(true)
            {
                vm.context.currents.push(0.0);
                continue;
            }

            // Compute the contribution value (branch current for current
            // contributions, source voltage for potential contributions).
            let value = Self::run_value_program(
                &mut vm,
                &program.value_program,
                #[cfg(feature = "native")]
                native,
                #[cfg(feature = "native")]
                NativeValueEntry::StampValue(program_idx),
                #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
                wasm,
                #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
                WasmJitExecutableEntry::StampValue(program_idx),
            )?;
            let value = Self::finite_result(value, format!("contribution {program_idx}"))?;

            // Probed currents stay per-copy; only the stamps scale
            vm.context.currents.push(value);
            if program.branch_ordinal.is_none()
                && let Some((pos, neg)) = Self::infer_current_terminal_pair(program)
            {
                vm.context.set_branch_current(pos, neg, value);
            }

            // Flow contributions of m parallel copies inject m times the
            // per-copy current; potential and constraint rows are per-copy
            let scale = if program.branch_ordinal.is_none() {
                m
            } else {
                1.0
            };

            // Companion model: solve A*x_new = z with the device linearized
            // at x_old.
            //
            // Current contributions: each KCL row receives the equivalent
            // current Ieq = I(x_old) - sum_col dI/dx_col * x_col_old and
            // the Jacobian stamps both KCL rows (entry sign tracks the row).
            //
            // Potential contributions: the branch row carries
            // V(p) - V(n) - E(x) = 0; the entries hold -dE/dx (sign -1)
            // and the RHS receives Eeq = E - sum dE/dx * x_old, which is
            // exactly value + sum(sign * deriv * x_old).
            let mut eq_value =
                Self::finite_result(value * scale, format!("scaled contribution {program_idx}"))?;

            for jacobian_entry in &matrix_indices.jacobian[program_idx] {
                let model_entry = &program.jacobian_programs[jacobian_entry.jacobian_idx];
                let deriv = Self::run_value_program(
                    &mut vm,
                    &model_entry.program,
                    #[cfg(feature = "native")]
                    native,
                    #[cfg(feature = "native")]
                    NativeValueEntry::Jacobian {
                        stamp: program_idx,
                        entry: jacobian_entry.jacobian_idx,
                    },
                    #[cfg(all(
                        not(feature = "native"),
                        feature = "wasm-jit",
                        target_arch = "wasm32"
                    ))]
                    wasm,
                    #[cfg(all(
                        not(feature = "native"),
                        feature = "wasm-jit",
                        target_arch = "wasm32"
                    ))]
                    WasmJitExecutableEntry::Jacobian {
                        stamp: program_idx,
                        entry: jacobian_entry.jacobian_idx,
                    },
                )?;
                let deriv = Self::finite_result(
                    deriv * scale,
                    format!("Jacobian {}:{}", program_idx, jacobian_entry.jacobian_idx),
                )?;

                // Accumulate the companion RHS term once per derivative
                // column. Current contributions (and indirect constraint
                // rows, which stamp the same way onto the branch row)
                // duplicate entries per row with +1/-1 signs: count only
                // the positive copy. Potential contributions carry single
                // -1-signed entries whose sign already encodes the
                // subtraction.
                match (program.branch_ordinal, program.indirect) {
                    (None, _) | (Some(_), true) => {
                        if model_entry.sign > 0.0 {
                            let x_col = Self::axis_value(vm.context, &model_entry.col_axis);
                            eq_value -= deriv * x_col;
                        }
                    }
                    (Some(_), false) => {
                        let x_col = Self::axis_value(vm.context, &model_entry.col_axis);
                        eq_value += model_entry.sign * deriv * x_col;
                    }
                }

                if let (Some(row), Some(col)) = (jacobian_entry.row, jacobian_entry.col) {
                    self.stamp_matrix_buffer
                        .push((row, col, jacobian_entry.sign * deriv));
                }
            }

            eq_value = Self::finite_result(
                eq_value,
                format!("equivalent source for contribution {program_idx}"),
            )?;

            // RHS: current contributions stamp -/+ Ieq at the KCL rows;
            // potential contributions stamp +Eeq at the branch row
            for entry in &matrix_indices.rhs[program_idx] {
                if let Some(row) = entry.node {
                    self.stamp_rhs_buffer.push((row, entry.sign * eq_value));
                }
            }
        }
        #[cfg(feature = "native")]
        Self::run_post_assignment_pass(&mut vm, model, native)?;
        #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
        Self::run_post_assignment_pass(&mut vm, model, wasm)?;
        for &(row, col, value) in &self.stamp_matrix_buffer {
            matrix_add(row, col, value);
        }
        for &(row, value) in &self.stamp_rhs_buffer {
            rhs_add(row, value);
        }
        self.stamp_matrix_buffer.clear();
        self.stamp_rhs_buffer.clear();
        Ok(())
    }

    /// Evaluate the model's noise sources at an operating point.
    ///
    /// PSDs come from the contribution expressions' `white_noise` /
    /// `flicker_noise` terms (amplitude-squared scaling folded in at
    /// compile time). Current-contribution sources inject across their
    /// node pair; potential-contribution sources inject at the branch
    /// row as a series EMF, so their PSD is in V²/Hz. Node ids follow the
    /// engine convention (0 = ground). Mode-disabled contributions
    /// contribute nothing.
    pub fn noise_sources(&mut self, circuit_voltages: &[f64]) -> Vec<EvaluatedNoiseSource> {
        self.try_noise_sources(circuit_voltages)
            .unwrap_or_else(|err| {
                panic!(
                    "Verilog-A device '{}' model '{}' noise evaluation failed: {}",
                    self.name, self.model.name, err
                )
            })
    }

    /// Whether a noise term is an ordinary KCL current injection. An
    /// indirect current constraint owns a branch unknown and its residual is
    /// stamped on that branch row, so it has the mapping and multiplicity
    /// convention of a potential contribution.
    fn noise_scales_as_current(
        model: &CompiledModel,
        is_current: bool,
        branch_ordinal: Option<usize>,
        program_idx: usize,
    ) -> bool {
        is_current
            && !(branch_ordinal.is_some()
                && model
                    .stamp_programs
                    .get(program_idx)
                    .is_some_and(|program| program.indirect))
    }

    #[cfg(all(
        not(feature = "native"),
        not(all(feature = "wasm-jit", target_arch = "wasm32"))
    ))]
    fn assignment_step_reads_contribution_current(step: &AssignmentStep) -> bool {
        let program_reads_current = |program: &BytecodeProgram| {
            program
                .instructions
                .iter()
                .any(|instruction| matches!(instruction, Instruction::PushCurrent(_, _)))
        };
        match step {
            AssignmentStep::Assign(assignment) => program_reads_current(&assignment.program),
            AssignmentStep::AssignIndexed { index, value, .. } => {
                program_reads_current(index) || program_reads_current(value)
            }
            AssignmentStep::Loop { condition, body } => {
                program_reads_current(condition)
                    || body
                        .iter()
                        .any(Self::assignment_step_reads_contribution_current)
            }
        }
    }

    #[cfg(all(
        not(feature = "native"),
        not(all(feature = "wasm-jit", target_arch = "wasm32"))
    ))]
    fn validate_portable_noise_assignment_split(model: &CompiledModel) -> Result<(), VmError> {
        fn program_reads(program: &BytecodeProgram, out: &mut std::collections::HashSet<usize>) {
            for instruction in &program.instructions {
                match instruction {
                    Instruction::PushVariable(slot) => {
                        out.insert(*slot);
                    }
                    Instruction::PushVariableDyn { base, len, .. } => {
                        out.extend(*base..base.saturating_add(*len));
                    }
                    _ => {}
                }
            }
        }
        fn step_targets(step: &AssignmentStep, out: &mut std::collections::HashSet<usize>) {
            match step {
                AssignmentStep::Assign(assignment) => {
                    out.insert(assignment.var_index);
                }
                AssignmentStep::AssignIndexed { base, len, .. } => {
                    out.extend(*base..base.saturating_add(*len));
                }
                AssignmentStep::Loop { body, .. } => {
                    for step in body {
                        step_targets(step, out);
                    }
                }
            }
        }
        fn step_reads(step: &AssignmentStep, out: &mut std::collections::HashSet<usize>) {
            match step {
                AssignmentStep::Assign(assignment) => program_reads(&assignment.program, out),
                AssignmentStep::AssignIndexed { index, value, .. } => {
                    program_reads(index, out);
                    program_reads(value, out);
                }
                AssignmentStep::Loop { condition, body } => {
                    program_reads(condition, out);
                    for step in body {
                        step_reads(step, out);
                    }
                }
            }
        }

        let Some(split) = model
            .assignment_steps
            .iter()
            .position(Self::assignment_step_reads_contribution_current)
        else {
            return Ok(());
        };
        let mut post_targets = std::collections::HashSet::new();
        for step in &model.assignment_steps[split..] {
            step_targets(step, &mut post_targets);
        }
        let mut pre_current_roots = std::collections::HashSet::new();
        for stamp in &model.stamp_programs {
            program_reads(&stamp.value_program, &mut pre_current_roots);
        }
        loop {
            let before = pre_current_roots.len();
            for step in model.assignment_steps[..split].iter().rev() {
                let mut targets = std::collections::HashSet::new();
                step_targets(step, &mut targets);
                if targets
                    .iter()
                    .any(|target| pre_current_roots.contains(target))
                {
                    step_reads(step, &mut pre_current_roots);
                }
            }
            if pre_current_roots.len() == before {
                break;
            }
        }
        if let Some(slot) = post_targets.intersection(&pre_current_roots).copied().min() {
            let name = model
                .variable_names
                .get(slot)
                .map(SmolStr::as_str)
                .unwrap_or("<unnamed>");
            return Err(VmError::InvalidModel(format!(
                "assignment variable '{name}' (slot {slot}) depends on a contribution current but is required before contribution-current evaluation"
            )));
        }
        Ok(())
    }

    /// Checked noise-source evaluation path for callers that can surface
    /// runtime model diagnostics instead of panicking or dropping sources.
    #[cfg(any(feature = "native", all(feature = "wasm-jit", target_arch = "wasm32")))]
    pub fn try_noise_sources(
        &mut self,
        circuit_voltages: &[f64],
    ) -> Result<Vec<EvaluatedNoiseSource>, VmError> {
        self.try_update_all_voltages(circuit_voltages)?;
        self.begin_evaluation(crate::vm::VerilogAEvaluationMode::SmallSignal);

        let context = &mut self.context;
        let model = &self.model;
        let program_active = &self.program_active;
        let legacy_noise_artifact = model.noise_process_schema == 0;
        #[cfg(feature = "native")]
        let native = self.native_model.as_ref();
        #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
        let wasm = self.wasm_jit_model.as_ref();

        context.clear_currents();
        context.currents.reserve(model.stamp_programs.len());
        let mut vm = Vm::new(context);
        Self::run_assignment_pass(
            &mut vm,
            model,
            #[cfg(feature = "native")]
            native,
            #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
            wasm,
        )?;
        Self::populate_noise_current_probe_cache(
            &mut vm,
            model,
            program_active,
            #[cfg(feature = "native")]
            native,
            #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
            wasm,
        )?;
        Self::run_post_assignment_pass(
            &mut vm,
            model,
            #[cfg(feature = "native")]
            native,
            #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
            wasm,
        )?;

        let circuit_node = |index: &StampIndex| -> usize {
            match index {
                StampIndex::Terminal(t) => self.node_mapping.get(*t).copied().unwrap_or(0),
                StampIndex::Internal(i) => self.internal_node_indices.get(*i).copied().unwrap_or(0),
                StampIndex::Branch(k) => self.branch_current_indices.get(*k).copied().unwrap_or(0),
                StampIndex::Ground => 0,
            }
        };

        let mut sources = Vec::with_capacity(model.noise_sources.len());
        for (idx, source) in model.noise_sources.iter().enumerate() {
            let active = if source.injections.is_empty() {
                program_active
                    .get(source.program_idx)
                    .copied()
                    .unwrap_or(true)
            } else {
                source.injections.iter().any(|injection| {
                    program_active
                        .get(injection.program_idx)
                        .copied()
                        .unwrap_or(true)
                })
            };
            if !active {
                continue;
            }
            let psd = Self::run_value_program(
                &mut vm,
                &source.psd_program,
                #[cfg(feature = "native")]
                native,
                #[cfg(feature = "native")]
                NativeValueEntry::NoisePsd(idx),
                #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
                wasm,
                #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
                WasmJitExecutableEntry::NoisePsd(idx),
            )?;
            let psd = Self::noise_power(psd, idx)?;
            if psd == 0.0 {
                continue;
            }
            let m = vm.context.multiplicity;
            let scales_as_current = Self::noise_scales_as_current(
                model,
                source.is_current,
                source.branch_ordinal,
                source.program_idx,
            );
            let psd = if scales_as_current { psd * m } else { psd / m };
            let psd = Self::finite_result(psd, format!("scaled noise source {idx} power"))?;
            let exponent = source
                .exponent_program
                .as_ref()
                .map(|program| {
                    Self::run_value_program(
                        &mut vm,
                        program,
                        #[cfg(feature = "native")]
                        native,
                        #[cfg(feature = "native")]
                        NativeValueEntry::NoiseExponent(idx),
                        #[cfg(all(
                            not(feature = "native"),
                            feature = "wasm-jit",
                            target_arch = "wasm32"
                        ))]
                        wasm,
                        #[cfg(all(
                            not(feature = "native"),
                            feature = "wasm-jit",
                            target_arch = "wasm32"
                        ))]
                        WasmJitExecutableEntry::NoiseExponent(idx),
                    )
                })
                .transpose()?
                .map(|value| Self::finite_result(value, format!("noise source {idx} exponent")))
                .transpose()?;

            let (node_pos, node_neg) = match (scales_as_current, source.branch_ordinal) {
                (false, Some(ordinal)) => (
                    self.branch_current_indices
                        .get(ordinal)
                        .copied()
                        .unwrap_or(0),
                    0,
                ),
                _ => (circuit_node(&source.pos), circuit_node(&source.neg)),
            };

            sources.push(EvaluatedNoiseSource {
                // Pre-grouped cache artifacts did not persist a process id,
                // so serde initializes every source to zero. Give those
                // artifacts deterministic dense identities by source index.
                process_id: if legacy_noise_artifact {
                    idx
                } else {
                    source.process_id
                },
                node_pos,
                node_neg,
                psd,
                exponent,
                table: source.table.clone(),
                name: source
                    .name
                    .as_ref()
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| format!("noise{idx}")),
            });
        }
        Ok(sources)
    }

    #[cfg(any(feature = "native", all(feature = "wasm-jit", target_arch = "wasm32")))]
    fn prepare_canonical_noise_operating_point(
        &mut self,
        circuit_voltages: &[f64],
    ) -> Result<(), VmError> {
        self.try_update_all_voltages(circuit_voltages)?;
        self.begin_evaluation(crate::vm::VerilogAEvaluationMode::SmallSignal);
        let context = &mut self.context;
        let model = &self.model;
        let program_active = &self.program_active;
        #[cfg(feature = "native")]
        let native = self.native_model.as_ref();
        #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
        let wasm = self.wasm_jit_model.as_ref();
        context.clear_currents();
        context.currents.reserve(model.stamp_programs.len());
        let mut vm = Vm::new(context);
        Self::run_assignment_pass(
            &mut vm,
            model,
            #[cfg(feature = "native")]
            native,
            #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
            wasm,
        )?;
        Self::populate_noise_current_probe_cache(
            &mut vm,
            model,
            program_active,
            #[cfg(feature = "native")]
            native,
            #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
            wasm,
        )?;
        Self::run_post_assignment_pass(
            &mut vm,
            model,
            #[cfg(feature = "native")]
            native,
            #[cfg(all(not(feature = "native"), feature = "wasm-jit", target_arch = "wasm32"))]
            wasm,
        )
    }

    #[cfg(all(
        not(feature = "native"),
        not(all(feature = "wasm-jit", target_arch = "wasm32"))
    ))]
    fn prepare_canonical_noise_operating_point(
        &mut self,
        circuit_voltages: &[f64],
    ) -> Result<(), VmError> {
        self.try_update_all_voltages(circuit_voltages)?;
        self.begin_evaluation(crate::vm::VerilogAEvaluationMode::SmallSignal);
        let model = &self.model;
        let split = model
            .assignment_steps
            .iter()
            .position(Self::assignment_step_reads_contribution_current)
            .unwrap_or(model.assignment_steps.len());
        let context = &mut self.context;
        context.clear_currents();
        context.currents.reserve(model.stamp_programs.len());
        let mut vm = Vm::new(context);
        Self::execute_assignment_steps(&mut vm, &model.assignment_steps[..split])?;
        Self::populate_noise_current_probe_cache(&mut vm, model, &self.program_active)?;
        Self::execute_assignment_steps(&mut vm, &model.assignment_steps[split..])
    }

    fn try_grouped_noise_processes_from_canonical_cfg(
        &mut self,
        circuit_voltages: &[f64],
        frequency_hz: f64,
    ) -> Result<Vec<EvaluatedNoiseProcess>, VmError> {
        self.prepare_canonical_noise_operating_point(circuit_voltages)?;
        let multiplicity = self.context.multiplicity;
        if !multiplicity.is_finite() || multiplicity <= 0.0 {
            return Err(VmError::InvalidRuntimeConfiguration(format!(
                "noise multiplicity must be finite and positive, got {multiplicity}"
            )));
        }
        let plan = self.canonical_noise_plan.as_ref().ok_or_else(|| {
            VmError::InvalidModel("grouped-noise canonical runtime plan is missing".into())
        })?;
        let metadata = plan.evaluate(&self.context)?;
        if metadata.len() != self.model.noise_sources.len() {
            return Err(VmError::InvalidModel(
                "grouped-noise canonical runtime result shape changed after construction".into(),
            ));
        }
        let variable_seed = self.context.variables.clone();
        let mut vm = crate::vm::SmallSignalVm::with_variable_seed(
            &self.context,
            frequency_hz,
            &variable_seed,
        )?;
        vm.execute_assignments(if self.model.noise_assignment_steps.is_empty() {
            &self.model.assignment_steps
        } else {
            &self.model.noise_assignment_steps
        })?;
        let circuit_node = |index: &StampIndex| -> Result<usize, VmError> {
            match index {
                StampIndex::Terminal(terminal) => self
                    .node_mapping
                    .get(*terminal)
                    .copied()
                    .ok_or_else(|| {
                        VmError::InvalidModel(format!(
                            "grouped-noise terminal endpoint {terminal} is outside the device mapping"
                        ))
                    }),
                StampIndex::Internal(internal) => self
                    .internal_node_indices
                    .get(*internal)
                    .copied()
                    .ok_or_else(|| {
                        VmError::InvalidModel(format!(
                            "grouped-noise internal endpoint {internal} is outside the device mapping"
                        ))
                    }),
                StampIndex::Branch(branch) => self
                    .branch_current_indices
                    .get(*branch)
                    .copied()
                    .ok_or_else(|| {
                        VmError::InvalidModel(format!(
                            "grouped-noise branch endpoint {branch} is outside the device mapping"
                        ))
                    }),
                StampIndex::Ground => Ok(0),
            }
        };
        let mut processes = Vec::with_capacity(metadata.len());
        for (expected, metadata) in metadata.into_iter().enumerate() {
            if metadata.process_id != expected {
                return Err(VmError::InvalidModel(format!(
                    "grouped-noise runtime process ID {} is not dense at {expected}",
                    metadata.process_id
                )));
            }
            let source = self.model.noise_sources.get(expected).ok_or_else(|| {
                VmError::InvalidModel("grouped-noise source index is outside model table".into())
            })?;
            if !metadata.active || source.injections.is_empty() {
                continue;
            }
            let mut injections = Vec::with_capacity(source.injections.len());
            let mut any_active_injection = false;
            for (injection_index, injection) in source.injections.iter().enumerate() {
                let injection_active = self
                    .program_active
                    .get(injection.program_idx)
                    .copied()
                    .ok_or_else(|| {
                        VmError::InvalidModel(format!(
                            "noise process {expected} injection {injection_index} activation index {} is outside the model table",
                            injection.program_idx
                        ))
                    })?;
                if !injection_active {
                    continue;
                }
                // Activation is structural/control-flow activation of an
                // individual injection, not its numeric gain. A live zero
                // gain must still validate the source's raw PSD.
                any_active_injection = true;
                let scales_as_current = Self::noise_scales_as_current(
                    &self.model,
                    injection.is_current,
                    injection.branch_ordinal,
                    injection.program_idx,
                );
                let gain_scale = if scales_as_current {
                    multiplicity.sqrt()
                } else {
                    multiplicity.sqrt().recip()
                };
                if !matches!(injection.rhs_sign, -1.0 | 1.0) {
                    return Err(VmError::InvalidModel(format!(
                        "noise process {expected} injection RHS sign must be +1 or -1, got {}",
                        injection.rhs_sign
                    )));
                }
                let gain = vm.execute(&injection.gain_program)? * gain_scale * injection.rhs_sign;
                if !gain.re.is_finite() || !gain.im.is_finite() {
                    return Err(VmError::InvalidNumericResult(format!(
                        "noise process {expected} injection gain is non-finite at {frequency_hz} Hz"
                    )));
                }
                let (node_pos, node_neg) = match (scales_as_current, injection.branch_ordinal) {
                    (false, Some(ordinal)) => {
                        let branch_node = self
                            .branch_current_indices
                            .get(ordinal)
                            .copied()
                            .ok_or_else(|| {
                                VmError::InvalidModel(format!(
                                    "noise process {expected} injection {injection_index} branch ordinal {ordinal} is outside the device mapping"
                                ))
                            })?;
                        (branch_node, 0)
                    }
                    _ => (circuit_node(&injection.pos)?, circuit_node(&injection.neg)?),
                };
                injections.push(EvaluatedNoiseInjection {
                    node_pos,
                    node_neg,
                    gain,
                });
            }
            if !any_active_injection {
                continue;
            }
            let psd = Self::noise_power(metadata.psd, expected)?;
            if psd == 0.0 {
                continue;
            }
            let exponent = metadata
                .exponent
                .map(|value| {
                    Self::finite_result(value, format!("noise process {expected} exponent"))
                })
                .transpose()?;
            processes.push(EvaluatedNoiseProcess {
                process_id: expected,
                psd,
                exponent,
                table: source.table.clone(),
                name: source
                    .name
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| format!("noise{expected}")),
                injections,
            });
        }
        Ok(processes)
    }

    /// Evaluate coherent, frequency-dependent injections for every
    /// syntactic Verilog-A noise process. Scalar PSD metadata is evaluated by
    /// the selected native/VM/WASM backend through `try_noise_sources`; only
    /// complex gain composition uses the shared read-only small-signal VM.
    pub fn try_noise_processes_at_frequency(
        &mut self,
        circuit_voltages: &[f64],
        frequency_hz: f64,
    ) -> Result<Vec<EvaluatedNoiseProcess>, VmError> {
        if !frequency_hz.is_finite() || frequency_hz < 0.0 {
            return Err(VmError::InvalidRuntimeConfiguration(format!(
                "noise frequency must be finite and nonnegative, got {frequency_hz}"
            )));
        }
        if self.canonical_noise_plan.is_some() {
            return self
                .try_grouped_noise_processes_from_canonical_cfg(circuit_voltages, frequency_hz);
        }
        self.try_update_all_voltages(circuit_voltages)?;
        let variable_seed = self.context.variables.clone();
        let scalar_sources = self.try_noise_sources(circuit_voltages)?;
        let multiplicity = self.context.multiplicity;
        if !multiplicity.is_finite() || multiplicity <= 0.0 {
            return Err(VmError::InvalidRuntimeConfiguration(format!(
                "noise multiplicity must be finite and positive, got {multiplicity}"
            )));
        }

        let mut vm = crate::vm::SmallSignalVm::with_variable_seed(
            &self.context,
            frequency_hz,
            &variable_seed,
        )?;
        vm.execute_assignments(if self.model.noise_assignment_steps.is_empty() {
            &self.model.assignment_steps
        } else {
            &self.model.noise_assignment_steps
        })?;

        let circuit_node = |index: &StampIndex| -> usize {
            match index {
                StampIndex::Terminal(terminal) => {
                    self.node_mapping.get(*terminal).copied().unwrap_or(0)
                }
                StampIndex::Internal(internal) => self
                    .internal_node_indices
                    .get(*internal)
                    .copied()
                    .unwrap_or(0),
                StampIndex::Branch(branch) => self
                    .branch_current_indices
                    .get(*branch)
                    .copied()
                    .unwrap_or(0),
                StampIndex::Ground => 0,
            }
        };

        let mut processes = Vec::with_capacity(scalar_sources.len());
        let legacy_noise_artifact = self.model.noise_process_schema == 0;
        for scalar in scalar_sources {
            let Some(source) = self.model.noise_sources.get(scalar.process_id) else {
                return Err(VmError::InvalidInstruction(
                    "evaluated noise process has no compiled definition",
                ));
            };
            if !legacy_noise_artifact && source.process_id != scalar.process_id {
                return Err(VmError::InvalidModel(format!(
                    "grouped-noise process table is not dense at index {}",
                    scalar.process_id
                )));
            }
            let source_scales_as_current = Self::noise_scales_as_current(
                &self.model,
                source.is_current,
                source.branch_ordinal,
                source.program_idx,
            );
            let legacy_scale = if source_scales_as_current {
                multiplicity
            } else {
                multiplicity.recip()
            };
            let psd = Self::finite_result(
                scalar.psd / legacy_scale,
                format!("noise process {} raw power", source.process_id),
            )?;
            let mut injections = Vec::with_capacity(source.injections.len());
            if source.injections.is_empty() {
                // Backward-compatible cache migration: artifacts serialized
                // before grouped injections carried exactly one unit-gain
                // injection in the legacy source fields.
                let gain_scale = if source_scales_as_current {
                    multiplicity.sqrt()
                } else {
                    multiplicity.sqrt().recip()
                };
                let (node_pos, node_neg) = match (source_scales_as_current, source.branch_ordinal) {
                    (false, Some(ordinal)) => (
                        self.branch_current_indices
                            .get(ordinal)
                            .copied()
                            .unwrap_or(0),
                        0,
                    ),
                    _ => (circuit_node(&source.pos), circuit_node(&source.neg)),
                };
                injections.push(EvaluatedNoiseInjection {
                    node_pos,
                    node_neg,
                    gain: num_complex::Complex64::new(gain_scale, 0.0),
                });
            }
            for injection in &source.injections {
                if !self
                    .program_active
                    .get(injection.program_idx)
                    .copied()
                    .unwrap_or(true)
                {
                    continue;
                }
                let injection_scales_as_current = Self::noise_scales_as_current(
                    &self.model,
                    injection.is_current,
                    injection.branch_ordinal,
                    injection.program_idx,
                );
                let gain_scale = if injection_scales_as_current {
                    multiplicity.sqrt()
                } else {
                    multiplicity.sqrt().recip()
                };
                if !matches!(injection.rhs_sign, -1.0 | 1.0) {
                    return Err(VmError::InvalidModel(format!(
                        "noise process {} injection RHS sign must be +1 or -1, got {}",
                        source.process_id, injection.rhs_sign
                    )));
                }
                let gain = vm.execute(&injection.gain_program)? * gain_scale * injection.rhs_sign;
                if !gain.re.is_finite() || !gain.im.is_finite() {
                    return Err(VmError::InvalidNumericResult(format!(
                        "noise process {} injection gain is non-finite at {frequency_hz} Hz",
                        source.process_id
                    )));
                }
                let (node_pos, node_neg) =
                    match (injection_scales_as_current, injection.branch_ordinal) {
                        (false, Some(ordinal)) => (
                            self.branch_current_indices
                                .get(ordinal)
                                .copied()
                                .unwrap_or(0),
                            0,
                        ),
                        _ => (circuit_node(&injection.pos), circuit_node(&injection.neg)),
                    };
                injections.push(EvaluatedNoiseInjection {
                    node_pos,
                    node_neg,
                    gain,
                });
            }
            if injections.is_empty() {
                continue;
            }
            processes.push(EvaluatedNoiseProcess {
                process_id: scalar.process_id,
                psd,
                exponent: scalar.exponent,
                table: scalar.table,
                name: scalar.name,
                injections,
            });
        }
        Ok(processes)
    }

    /// Checked noise-source evaluation path for callers that can surface
    /// runtime model diagnostics instead of panicking or dropping sources.
    #[cfg(all(
        not(feature = "native"),
        not(all(feature = "wasm-jit", target_arch = "wasm32"))
    ))]
    pub fn try_noise_sources(
        &mut self,
        circuit_voltages: &[f64],
    ) -> Result<Vec<EvaluatedNoiseSource>, VmError> {
        self.try_update_all_voltages(circuit_voltages)?;
        self.begin_evaluation(crate::vm::VerilogAEvaluationMode::SmallSignal);

        let context = &mut self.context;
        let model = &self.model;
        let program_active = &self.program_active;
        let legacy_noise_artifact = model.noise_process_schema == 0;

        context.clear_currents();
        context.currents.reserve(model.stamp_programs.len());
        let mut vm = Vm::new(context);
        let assignment_split = model
            .assignment_steps
            .iter()
            .position(Self::assignment_step_reads_contribution_current)
            .unwrap_or(model.assignment_steps.len());
        Self::execute_assignment_steps(&mut vm, &model.assignment_steps[..assignment_split])?;
        Self::populate_noise_current_probe_cache(&mut vm, model, program_active)?;
        // Preserve source order while running every assignment exactly once:
        // the suffix beginning at the first contribution-current read waits
        // until the contribution vector is complete. This matches the
        // native/WASM split without double-running self-referential or
        // stateful assignments.
        Self::execute_assignment_steps(&mut vm, &model.assignment_steps[assignment_split..])?;

        let circuit_node = |index: &StampIndex| -> usize {
            match index {
                StampIndex::Terminal(t) => self.node_mapping.get(*t).copied().unwrap_or(0),
                StampIndex::Internal(i) => self.internal_node_indices.get(*i).copied().unwrap_or(0),
                StampIndex::Branch(k) => self.branch_current_indices.get(*k).copied().unwrap_or(0),
                StampIndex::Ground => 0,
            }
        };

        let mut sources = Vec::with_capacity(model.noise_sources.len());
        for (idx, source) in model.noise_sources.iter().enumerate() {
            let active = if source.injections.is_empty() {
                program_active
                    .get(source.program_idx)
                    .copied()
                    .unwrap_or(true)
            } else {
                source.injections.iter().any(|injection| {
                    program_active
                        .get(injection.program_idx)
                        .copied()
                        .unwrap_or(true)
                })
            };
            if !active {
                continue;
            }
            let psd_program = &source.psd_program;
            let psd = vm.execute(psd_program)?;
            let psd = Self::noise_power(psd, idx)?;
            if psd == 0.0 {
                continue;
            }
            // m uncorrelated parallel copies: current-noise powers add
            // (x m); series voltage-noise EMFs average (/ m)
            let m = vm.context.multiplicity;
            let scales_as_current = Self::noise_scales_as_current(
                model,
                source.is_current,
                source.branch_ordinal,
                source.program_idx,
            );
            let psd = if scales_as_current { psd * m } else { psd / m };
            let psd = Self::finite_result(psd, format!("scaled noise source {idx} power"))?;
            let exponent = source
                .exponent_program
                .as_ref()
                .map(|p| vm.execute(p))
                .transpose()?
                .map(|value| Self::finite_result(value, format!("noise source {idx} exponent")))
                .transpose()?;

            // Potential-contribution noise is a series EMF on the branch
            // equation row; current noise injects across the node pair
            let (node_pos, node_neg) = match (scales_as_current, source.branch_ordinal) {
                (false, Some(ordinal)) => (
                    self.branch_current_indices
                        .get(ordinal)
                        .copied()
                        .unwrap_or(0),
                    0,
                ),
                _ => (circuit_node(&source.pos), circuit_node(&source.neg)),
            };

            sources.push(EvaluatedNoiseSource {
                // Pre-grouped cache artifacts did not persist a process id,
                // so serde initializes every source to zero. Give those
                // artifacts deterministic dense identities by source index.
                process_id: if legacy_noise_artifact {
                    idx
                } else {
                    source.process_id
                },
                node_pos,
                node_neg,
                psd,
                exponent,
                table: source.table.clone(),
                name: source
                    .name
                    .as_ref()
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| format!("noise{idx}")),
            });
        }
        Ok(sources)
    }

    /// Value of a differentiation axis: a unified node voltage or a
    /// branch-current unknown
    fn axis_value(context: &VmContext, axis: &crate::codegen::ColumnAxis) -> f64 {
        match axis {
            crate::codegen::ColumnAxis::Node(node) => Self::unified_node_voltage(context, *node),
            crate::codegen::ColumnAxis::Branch(k) => context
                .branch_current_values
                .get(*k)
                .copied()
                .unwrap_or(0.0),
        }
    }

    /// Voltage of a unified node index (terminals first, then internal
    /// nodes; usize::MAX is the global reference)
    fn unified_node_voltage(context: &VmContext, node: usize) -> f64 {
        let num_terminals = context.terminal_count();
        if node == usize::MAX {
            0.0
        } else if node < num_terminals {
            context.voltages.get(node).copied().unwrap_or(0.0)
        } else {
            context
                .internal_voltages
                .get(node - num_terminals)
                .copied()
                .unwrap_or(0.0)
        }
    }

    /// Convert a StampIndex to circuit node
    fn index_to_node(
        index: &StampIndex,
        node_mapping: &[usize],
        internal_node_indices: &[usize],
        branch_current_indices: &[usize],
    ) -> Option<usize> {
        match index {
            StampIndex::Terminal(t) => {
                let node = node_mapping.get(*t).copied().unwrap_or(0);
                if node > 0 { Some(node - 1) } else { None }
            }
            StampIndex::Internal(i) => {
                let node = internal_node_indices.get(*i).copied().unwrap_or(0);
                if node > 0 { Some(node - 1) } else { None }
            }
            StampIndex::Branch(k) => {
                let node = branch_current_indices.get(*k).copied().unwrap_or(0);
                if node > 0 { Some(node - 1) } else { None }
            }
            StampIndex::Ground => None,
        }
    }

    /// Convert a stamp index to matrix node index for this device instance.
    pub fn stamp_index_to_node(&self, index: &StampIndex) -> Option<usize> {
        match index {
            StampIndex::Internal(i) => {
                let mapped = self.internal_node_indices.get(*i).copied().unwrap_or(0);
                if mapped > 0 {
                    Some(mapped - 1)
                } else {
                    Some(self.model.num_terminals + *i)
                }
            }
            _ => Self::index_to_node(
                index,
                &self.node_mapping,
                &self.internal_node_indices,
                &self.branch_current_indices,
            ),
        }
    }

    fn infer_current_terminal_pair(
        program: &crate::codegen::StampProgram,
    ) -> Option<(usize, usize)> {
        let mut pos_endpoint = None;
        let mut neg_endpoint = None;

        for loc in &program.stamp_locations {
            let endpoint = match loc.row {
                StampIndex::Terminal(term) => term,
                StampIndex::Ground => CURRENT_PAIR_GROUND,
                _ => continue,
            };

            if loc.sign < 0.0 {
                if pos_endpoint.replace(endpoint).is_some() {
                    return None;
                }
            } else if loc.sign > 0.0 && neg_endpoint.replace(endpoint).is_some() {
                return None;
            }
        }

        match (pos_endpoint, neg_endpoint) {
            (Some(pos), Some(neg)) if pos != neg => Some((pos, neg)),
            _ => None,
        }
    }
}

/// One noise source evaluated at an operating point
#[derive(Debug, Clone)]
pub struct EvaluatedNoiseSource {
    /// Dense syntactic process identity.
    pub process_id: usize,
    /// Positive injection circuit node (0 = ground); for potential
    /// contributions this is the branch-equation row's unknown
    pub node_pos: usize,
    /// Negative injection circuit node (0 = ground)
    pub node_neg: usize,
    /// Power spectral density at the operating point (A²/Hz for current
    /// contributions, V²/Hz for potential contributions). For table
    /// sources this is the scale applied to the interpolated value.
    pub psd: f64,
    /// Flicker frequency exponent: S(f) = psd / f^exp (None = white)
    pub exponent: Option<f64>,
    /// Frequency-interpolated PSD table: sorted (f, p) points and whether
    /// interpolation runs in log-log coordinates
    pub table: Option<(Vec<(f64, f64)>, bool)>,
    /// Source label
    pub name: String,
}

/// One complex circuit injection of a coherent noise process.
#[derive(Debug, Clone)]
pub struct EvaluatedNoiseInjection {
    pub node_pos: usize,
    pub node_neg: usize,
    pub gain: num_complex::Complex64,
}

/// One syntactic Verilog-A noise process evaluated at a frequency.
#[derive(Debug, Clone)]
pub struct EvaluatedNoiseProcess {
    pub process_id: usize,
    pub psd: f64,
    pub exponent: Option<f64>,
    pub table: Option<(Vec<(f64, f64)>, bool)>,
    pub name: String,
    pub injections: Vec<EvaluatedNoiseInjection>,
}

/// Result of Jacobian computation
#[derive(Debug, Clone)]
pub struct JacobianEntry {
    /// Computed derivative value
    pub value: f64,
    /// Row stamp index
    pub row: StampIndex,
    /// Column stamp index
    pub col: StampIndex,
    /// Index of the stamp program
    pub program_idx: usize,
    /// Index within Jacobian programs
    pub jacobian_idx: usize,
}

/// Builder for creating device instances with parameter overrides
pub struct DeviceBuilder {
    model: CompiledModel,
    name: SmolStr,
    nodes: Vec<usize>,
    params: Vec<(String, f64)>,
    temperature: f64,
    #[cfg(feature = "native")]
    canonical_ir: Option<CanonicalIrArtifact>,
}

impl DeviceBuilder {
    /// Create a new builder
    pub fn new(model: CompiledModel, name: impl Into<SmolStr>) -> Self {
        Self {
            model,
            name: name.into(),
            nodes: Vec::new(),
            params: Vec::new(),
            temperature: 300.15, // 27°C
            #[cfg(feature = "native")]
            canonical_ir: None,
        }
    }

    /// Set terminal connections
    pub fn nodes(mut self, nodes: &[usize]) -> Self {
        self.nodes = nodes.to_vec();
        self
    }

    /// Set a parameter
    pub fn param(mut self, name: &str, value: f64) -> Self {
        self.params.push((name.to_string(), value));
        self
    }

    /// Set temperature
    pub fn temperature(mut self, temp_k: f64) -> Self {
        self.temperature = temp_k;
        self
    }

    /// Provide the canonical IR artifact required by native JIT builds.
    #[cfg(feature = "native")]
    pub fn canonical_ir(mut self, artifact: CanonicalIrArtifact) -> Self {
        self.canonical_ir = Some(artifact);
        self
    }

    /// Build the device
    pub fn build(self) -> VerilogADevice {
        self.try_build().unwrap_or_else(|err| {
            panic!("Verilog-A device builder failed: {err}");
        })
    }

    /// Checked build path that reports native-JIT and parameter-default
    /// failures instead of panicking.
    pub fn try_build(self) -> Result<VerilogADevice, VmError> {
        let Self {
            model,
            name,
            nodes,
            params,
            temperature,
            #[cfg(feature = "native")]
            canonical_ir,
        } = self;

        #[cfg(feature = "native")]
        let mut device = {
            let canonical_ir = canonical_ir.ok_or_else(|| {
                VmError::NativeJit(
                    "DeviceBuilder requires canonical IR when native JIT is enabled; no interpreter fallback"
                        .to_string(),
                )
            })?;
            VerilogADevice::try_new_with_canonical_ir(name, model, &canonical_ir, &nodes)?
        };

        #[cfg(not(feature = "native"))]
        let mut device = VerilogADevice::try_new(name, model, &nodes)?;

        device.try_set_temperature(temperature)?;

        for (name, value) in params {
            match device
                .try_set_parameter(&name, value)
                .map_err(|error| VmError::ParameterValue(error.to_string()))?
            {
                true => {}
                false => {
                    return Err(VmError::ParameterValue(format!(
                        "unknown parameter '{name}' for model '{}'",
                        device.model.name
                    )));
                }
            }
        }
        device.try_resolve_parameter_defaults()?;

        Ok(device)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(all(
    test,
    not(feature = "native"),
    not(all(feature = "wasm-jit", target_arch = "wasm32"))
))]
mod bytecode_assignment_integrity_tests {
    use super::VerilogADevice;
    use crate::codegen::{AssignmentProgram, AssignmentStep, BytecodeProgram, Instruction};
    use crate::vm::{Vm, VmContext, VmError};

    #[test]
    fn scalar_assignment_outside_runtime_storage_fails_closed() {
        let mut context = VmContext::default();
        context.variables = vec![7.0];
        let mut vm = Vm::new(&mut context);
        let steps = [AssignmentStep::Assign(AssignmentProgram {
            var_index: 1,
            program: BytecodeProgram {
                instructions: vec![Instruction::PushParam(999)],
            },
        })];

        let error = VerilogADevice::execute_assignment_steps(&mut vm, &steps)
            .expect_err("malformed scalar assignment targets must not disappear");
        assert!(matches!(&error, VmError::InvalidInstruction(_)));
        assert!(error.to_string().contains("assignment target variable"));
        assert_eq!(vm.context.variables, vec![7.0]);
    }

    #[test]
    fn indexed_assignment_outside_runtime_storage_fails_closed() {
        let mut context = VmContext::default();
        context.variables = vec![7.0];
        let mut vm = Vm::new(&mut context);
        let steps = [AssignmentStep::AssignIndexed {
            base: 1,
            len: 1,
            lower: 0,
            index: BytecodeProgram {
                instructions: vec![Instruction::PushConst(0.0)],
            },
            value: BytecodeProgram {
                instructions: vec![Instruction::PushParam(999)],
            },
        }];

        let error = VerilogADevice::execute_assignment_steps(&mut vm, &steps)
            .expect_err("malformed indexed assignment targets must not disappear");
        assert!(matches!(&error, VmError::InvalidInstruction(_)));
        assert!(error.to_string().contains("indexed assignment target"));
        assert_eq!(vm.context.variables, vec![7.0]);
    }

    #[test]
    fn compiled_assignment_layout_rejects_corrupt_scalar_and_indexed_targets() {
        let scalar = [AssignmentStep::Assign(AssignmentProgram {
            var_index: 1,
            program: BytecodeProgram {
                instructions: vec![Instruction::PushConst(9.0)],
            },
        })];
        let error = VerilogADevice::validate_compiled_assignment_layout(1, &scalar)
            .expect_err("out-of-range scalar target must fail model validation");
        assert!(matches!(error, VmError::InvalidModel(_)));

        let indexed = [AssignmentStep::AssignIndexed {
            base: usize::MAX,
            len: 2,
            lower: 0,
            index: BytecodeProgram {
                instructions: vec![Instruction::PushConst(0.0)],
            },
            value: BytecodeProgram {
                instructions: vec![Instruction::PushConst(9.0)],
            },
        }];
        let error = VerilogADevice::validate_compiled_assignment_layout(1, &indexed)
            .expect_err("overflowing indexed target must fail model validation");
        assert!(matches!(error, VmError::InvalidModel(_)));
    }
}

#[cfg(all(
    test,
    not(feature = "native"),
    not(all(feature = "wasm-jit", target_arch = "wasm32"))
))]
mod bytecode_laplace_derivative_tests {
    use super::VerilogADevice;
    use crate::{CompilerOptions, VerilogACompiler};

    const SOURCE: &str = r#"
`include "disciplines.vams"
module laplace_derivative_device(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ laplace_nd(V(p, n), '{1.0}, '{1.0, 1.0});
endmodule
"#;

    fn evaluate_at(voltage: f64) -> f64 {
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(SOURCE).expect("compile Laplace fixture");
        let mut device =
            VerilogADevice::try_new("LAPLACE_FD", model, &[1, 0]).expect("build device");
        device.try_begin_analysis(2).expect("begin transient");
        device.set_timestep(0.5);
        device.update_voltages(&[voltage]);
        device.try_evaluate().expect("evaluate transient")[0]
    }

    #[test]
    fn transient_device_jacobian_matches_primal_finite_difference() {
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(SOURCE).expect("compile Laplace fixture");
        let mut device =
            VerilogADevice::try_new("LAPLACE_JAC", model, &[1, 0]).expect("build device");
        device.try_begin_analysis(2).expect("begin transient");
        device.set_timestep(0.5);
        device.update_voltages(&[0.75]);

        let epsilon = 1.0e-6;
        let finite_difference =
            (evaluate_at(0.75 + epsilon) - evaluate_at(0.75 - epsilon)) / (2.0 * epsilon);
        let jacobian = device
            .try_compute_jacobian()
            .expect("compute transient Jacobian");

        assert!((finite_difference - 1.0 / 3.0).abs() < 1.0e-9);
        assert!(
            jacobian
                .iter()
                .any(|entry| (entry.value - finite_difference).abs() < 1.0e-9),
            "Jacobian {jacobian:?} does not contain finite-difference action {finite_difference}"
        );
    }
}

#[cfg(test)]
mod complex_small_signal_device_tests {
    use super::VerilogADevice;
    use crate::{CompilerOptions, VerilogACompiler};
    use std::hint::black_box;
    use std::time::{Duration, Instant};

    fn device(source: &str) -> VerilogADevice {
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let runtime = compiler
            .compile_runtime(source, None)
            .expect("compile complex small-signal fixture");
        #[cfg(any(feature = "native", all(feature = "wasm-jit", target_arch = "wasm32")))]
        let device = VerilogADevice::try_new_with_canonical_ir(
            "AC_COMPLEX",
            runtime.model,
            &runtime.canonical_ir,
            &[1, 0],
        );
        #[cfg(not(any(feature = "native", all(feature = "wasm-jit", target_arch = "wasm32"))))]
        let device = VerilogADevice::try_new("AC_COMPLEX", runtime.model, &[1, 0]);
        device.expect("build complex small-signal fixture")
    }

    fn one_port_admittance(source: &str, voltage: f64, frequency_hz: f64) -> (f64, f64) {
        let mut device = device(source);
        device.try_begin_analysis(1).expect("begin AC analysis");
        let mut entries = Vec::new();
        device
            .try_stamp_small_signal_complex(&[voltage], frequency_hz, |row, col, re, im| {
                if row == 0 && col == 0 {
                    entries.push((re, im));
                }
            })
            .expect("stamp complex small-signal fixture");
        assert!(!entries.is_empty(), "fixture produced no one-port Jacobian");
        entries
            .into_iter()
            .fold((0.0, 0.0), |(ar, ai), (br, bi)| (ar + br, ai + bi))
    }

    #[test]
    fn device_laplace_assignment_shadow_retains_complex_phase() {
        let source = r#"
`include "disciplines.vams"
module assigned_laplace(p, n);
    inout p, n;
    electrical p, n;
    real y;
    analog begin
        y = laplace_nd(V(p, n), '{1.0}, '{1.0, 1.0});
        I(p, n) <+ y * y;
    end
endmodule
"#;
        let (real, imag) = one_port_admittance(source, 2.0, 1.0 / std::f64::consts::TAU);
        assert!((real - 2.0).abs() <= 1.0e-12, "{real}+j{imag}");
        assert!((imag + 2.0).abs() <= 1.0e-12, "{real}+j{imag}");
    }

    #[test]
    fn device_ddt_and_idt_use_physical_frequency_actions_once() {
        let ddt = r#"
`include "disciplines.vams"
module ddt_ac(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ ddt(2.0e-6 * V(p, n));
endmodule
"#;
        let idt = r#"
`include "disciplines.vams"
module idt_ac(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ idt(V(p, n), 0.0);
endmodule
"#;
        let frequency = 10.0;
        let omega = std::f64::consts::TAU * frequency;
        let (ddt_real, ddt_imag) = one_port_admittance(ddt, 0.0, frequency);
        assert_eq!(ddt_real, 0.0);
        assert!((ddt_imag - omega * 2.0e-6).abs() <= 1.0e-15);
        let (idt_real, idt_imag) = one_port_admittance(idt, 0.0, frequency);
        assert_eq!(idt_real, 0.0);
        assert!((idt_imag + 1.0 / omega).abs() <= 1.0e-15);
    }

    #[test]
    fn device_zi_and_absdelay_apply_quarter_cycle_phase() {
        let zi = r#"
`include "disciplines.vams"
module zi_ac(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ zi_nd(V(p, n), '{0.0, 1.0}, '{1.0}, 1.0);
endmodule
"#;
        let delay = r#"
`include "disciplines.vams"
module delay_ac(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ absdelay(V(p, n), 0.25);
endmodule
"#;
        for (label, source, frequency_hz) in [("Zi", zi, 0.25), ("absdelay", delay, 1.0)] {
            let (real, imag) = one_port_admittance(source, 0.0, frequency_hz);
            assert!(real.abs() <= 1.0e-14, "{label}: {real}+j{imag}");
            assert!((imag + 1.0).abs() <= 1.0e-14, "{label}: {real}+j{imag}");
        }
    }

    /// Focused release benchmark for the complex frequency-domain path.
    ///
    /// The split real/reactive path is an exact reference for this deliberately
    /// simple RC admittance, so the same process reports both the compatibility
    /// baseline and the path required by nested filters. Run with:
    /// `cargo test -p rspice-veriloga --release --all-features
    /// complex_small_signal_stamp_microbenchmark -- --ignored --nocapture`.
    #[test]
    #[ignore = "manual release microbenchmark"]
    fn complex_small_signal_stamp_microbenchmark() {
        assert!(
            !cfg!(debug_assertions),
            "rerun the microbenchmark with --release"
        );
        let source = r#"
`include "disciplines.vams"
module ac_stamp_bench(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ 1.0e-3 * V(p, n) + ddt(1.0e-6 * V(p, n));
endmodule
"#;
        let frequency_hz = 1.0e3;
        let omega = std::f64::consts::TAU * frequency_hz;
        let iterations = std::env::var("RSPICE_SMALL_SIGNAL_BENCH_ITERATIONS")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(50_000)
            .max(1);
        let samples = 5;
        let mut complex = device(source);
        let mut split = device(source);
        complex.try_begin_analysis(1).unwrap();
        split.try_begin_analysis(1).unwrap();

        let run_complex = |device: &mut VerilogADevice| {
            let start = Instant::now();
            let mut checksum = 0.0;
            for _ in 0..iterations {
                device
                    .try_stamp_small_signal_complex(&[0.0], frequency_hz, |row, col, re, im| {
                        checksum += black_box((row + col + 1) as f64) * (re + im);
                    })
                    .unwrap();
            }
            (start.elapsed(), black_box(checksum))
        };
        let run_split = |device: &mut VerilogADevice| {
            let start = Instant::now();
            let mut checksum = 0.0;
            for _ in 0..iterations {
                device
                    .try_stamp(
                        &[0.0],
                        |row, col, value| {
                            checksum += black_box((row + col + 1) as f64) * value;
                        },
                        |_, _| {},
                    )
                    .unwrap();
                device
                    .try_stamp_reactive(&[0.0], |row, col, value| {
                        checksum += black_box((row + col + 1) as f64) * omega * value;
                    })
                    .unwrap();
            }
            (start.elapsed(), black_box(checksum))
        };

        let _ = run_complex(&mut complex);
        let _ = run_split(&mut split);
        let mut complex_times = Vec::with_capacity(samples);
        let mut split_times = Vec::with_capacity(samples);
        let mut complex_checksum = 0.0;
        let mut split_checksum = 0.0;
        for sample in 0..samples {
            let (first, second) = if sample % 2 == 0 {
                let complex = run_complex(&mut complex);
                let split = run_split(&mut split);
                (complex, split)
            } else {
                let split = run_split(&mut split);
                let complex = run_complex(&mut complex);
                (complex, split)
            };
            complex_times.push(first.0);
            split_times.push(second.0);
            complex_checksum += first.1;
            split_checksum += second.1;
        }
        let median = |times: &mut Vec<Duration>| {
            times.sort_unstable();
            times[times.len() / 2].as_secs_f64() * 1.0e9 / iterations as f64
        };
        let complex_ns = median(&mut complex_times);
        let split_ns = median(&mut split_times);
        let expected_checksum = samples as f64 * iterations as f64 * (1.0e-3 + omega * 1.0e-6);
        assert!(
            (complex_checksum - expected_checksum).abs() <= expected_checksum * 1.0e-9,
            "complex checksum {complex_checksum:e}, expected {expected_checksum:e}"
        );
        assert!(
            (split_checksum - expected_checksum).abs() <= expected_checksum * 1.0e-9,
            "split checksum {split_checksum:e}, expected {expected_checksum:e}"
        );
        eprintln!(
            "complex-small-signal-microbench iterations={iterations} samples={samples} complex_median_ns_per_stamp={complex_ns:.3} split_reference_median_ns_per_stamp={split_ns:.3} ratio={:.3} checksum={complex_checksum:.17e}",
            complex_ns / split_ns
        );
    }
}

#[cfg(test)]
mod analysis_lifecycle_tests {
    use super::VerilogADevice;
    use crate::{CompilerOptions, VerilogACompiler};

    #[test]
    fn public_begin_analysis_is_failure_atomic_and_starts_with_fresh_history() {
        let source = r#"
`include "disciplines.vams"
module begin_analysis_reset(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ absdelay(V(p, n), 0.5);
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let runtime = compiler
            .compile_runtime(source, None)
            .expect("compile begin-analysis reset fixture");
        let fresh_model = runtime.model.clone();
        let mut device = {
            #[cfg(any(feature = "native", all(feature = "wasm-jit", target_arch = "wasm32")))]
            {
                VerilogADevice::try_new_with_canonical_ir(
                    "BEGINRESET1",
                    runtime.model,
                    &runtime.canonical_ir,
                    &[1, 0],
                )
            }
            #[cfg(not(any(
                feature = "native",
                all(feature = "wasm-jit", target_arch = "wasm32")
            )))]
            {
                VerilogADevice::try_new("BEGINRESET1", runtime.model, &[1, 0])
            }
        }
        .expect("build begin-analysis reset fixture");
        device.try_begin_analysis(2).unwrap();
        device.set_time(0.0);
        device.set_timestep(0.5);
        device.update_voltages(&[1.0]);
        device.try_evaluate().expect("evaluate first delay sample");
        device
            .try_advance_state()
            .expect("commit first delay sample");
        device.set_time(1.0);
        device.set_timestep(1.0);
        device.update_voltages(&[3.0]);
        let history_dependent = device
            .try_evaluate()
            .expect("evaluate interpolated delay history");
        device
            .try_advance_state()
            .expect("commit interpolated delay history");

        let before_invalid = device
            .checkpoint_state()
            .expect("capture committed state before rejected begin");
        let error = device
            .try_begin_analysis(5)
            .expect_err("an invalid analysis code must fail closed");
        assert!(error.to_string().contains("analysis type"), "got: {error}");
        assert_eq!(
            device
                .checkpoint_state()
                .expect("capture state after rejected begin"),
            before_invalid,
            "a rejected analysis begin must not partially reset the instance"
        );

        device
            .try_begin_analysis(2)
            .expect("a valid analysis begin resets the reusable instance");
        device.set_time(0.0);
        device.set_timestep(1.0);
        device.update_voltages(&[9.0]);
        device
            .try_evaluate()
            .expect("seed the reused device's fresh delay history");
        device
            .try_advance_state()
            .expect("accept the reused device's fresh delay anchor");
        device.set_time(1.0);
        device.set_timestep(1.0);
        device.update_voltages(&[9.0]);
        let restarted = device
            .try_evaluate()
            .expect("evaluate the reused device after a fresh analysis");

        let mut fresh = {
            #[cfg(any(feature = "native", all(feature = "wasm-jit", target_arch = "wasm32")))]
            {
                VerilogADevice::try_new_with_canonical_ir(
                    "BEGINRESET2",
                    fresh_model,
                    &runtime.canonical_ir,
                    &[1, 0],
                )
            }
            #[cfg(not(any(
                feature = "native",
                all(feature = "wasm-jit", target_arch = "wasm32")
            )))]
            {
                VerilogADevice::try_new("BEGINRESET2", fresh_model, &[1, 0])
            }
        }
        .expect("build independent fresh-analysis reference");
        fresh.try_begin_analysis(2).unwrap();
        fresh.set_time(0.0);
        fresh.set_timestep(1.0);
        fresh.update_voltages(&[9.0]);
        fresh
            .try_evaluate()
            .expect("seed independent fresh delay history");
        fresh
            .try_advance_state()
            .expect("accept independent fresh delay anchor");
        fresh.set_time(1.0);
        fresh.set_timestep(1.0);
        fresh.update_voltages(&[9.0]);
        let expected = fresh
            .try_evaluate()
            .expect("evaluate independent fresh-analysis reference");

        let bits = |values: &[f64]| {
            values
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        };
        assert_eq!(
            bits(&restarted),
            bits(&expected),
            "a same-code fresh analysis must be independent of committed delay history"
        );
        assert_ne!(
            bits(&restarted),
            bits(&history_dependent),
            "the fixture must distinguish a reset analysis from the prior trajectory"
        );
    }
}

#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
mod tests {
    use super::*;
    use crate::codegen::{
        AssignmentProgram, AssignmentStep, BytecodeProgram, ColumnAxis, CompiledNoiseSource,
        JacobianEntry,
    };
    use crate::{CompilerOptions, VerilogACompiler};
    use std::sync::Arc;

    #[test]
    fn native_device_send_sync_are_derived_from_owned_fields() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<VerilogADevice>();
    }

    #[test]
    fn native_device_construction_accepts_legacy_laplace_derivative_metadata() {
        let source = r#"
`include "disciplines.vams"
module native_laplace_metadata(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ laplace_nd(V(p, n), '{1.0}, '{1.0, 1.0});
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile Laplace model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile Laplace canonical IR");

        VerilogADevice::try_new_with_canonical_ir(
            "NATIVE_LAPLACE_METADATA",
            model,
            &artifact,
            &[1, 0],
        )
        .expect("construct native device with legacy Laplace derivative metadata");
    }

    /// A second engine build reuses the first build's image.
    ///
    /// The runtime model cache allocates a fresh `Arc<CompiledModel>` whenever
    /// it restores a record from disk, so an address-keyed cache reports a miss
    /// for a byte-identical model and recompiles it. Large compact models cost
    /// seconds there, so this is pinned on content identity.
    #[test]
    fn native_compilation_is_reused_across_distinct_model_allocations() {
        let source = r#"
`include "disciplines.vams"
module native_cache_identity(p, n);
  inout p, n;
  electrical p, n;
  parameter real resistance = 4.0;
  analog I(p, n) <+ V(p, n) / resistance;
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler
            .compile(source)
            .expect("compile cache-identity model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile cache-identity canonical IR");

        let first = Arc::new(model.clone());
        VerilogADevice::try_new_with_canonical_ir(
            "XCACHE1",
            Arc::clone(&first),
            &artifact,
            &[1, 0],
        )
        .expect("build first cache-identity device");
        assert_eq!(
            native_compile_count("native_cache_identity"),
            1,
            "the first device must reach the backend exactly once"
        );

        // A separate allocation holding identical content: what a disk-cache
        // hit hands back on the next engine build.
        let second = Arc::new(model.clone());
        assert!(
            !Arc::ptr_eq(&first, &second),
            "the test must exercise two distinct allocations"
        );
        VerilogADevice::try_new_with_canonical_ir("XCACHE2", second, &artifact, &[1, 0])
            .expect("build second cache-identity device");
        assert_eq!(
            native_compile_count("native_cache_identity"),
            1,
            "an identical model must not be recompiled for a new allocation"
        );

        // Dropping every device must not invalidate the cache either: the
        // engine rebuild that motivates this cache drops the old circuit first.
        drop(first);
        VerilogADevice::try_new_with_canonical_ir("XCACHE3", Arc::new(model), &artifact, &[1, 0])
            .expect("build third cache-identity device");
        assert_eq!(
            native_compile_count("native_cache_identity"),
            1,
            "the cache must survive the last live model reference being dropped"
        );
    }

    #[test]
    fn native_compile_cache_rejects_same_source_with_different_variable_layout() {
        let source = r#"
`include "disciplines.vams"
module native_cache_layout_identity(p, n);
  inout p, n;
  electrical p, n;
  real voltage;
  analog begin
    voltage = V(p, n);
    I(p, n) <+ voltage * voltage;
  end
endmodule
"#;
        let model = VerilogACompiler::new(CompilerOptions::default())
            .compile(source)
            .expect("compile cache-layout model");
        let mut reordered = model.clone();
        let shadow_slots = reordered
            .variable_names
            .iter()
            .enumerate()
            .filter_map(|(slot, name)| name.contains('@').then_some(slot))
            .collect::<Vec<_>>();
        assert!(
            shadow_slots.len() >= 2,
            "fixture must contain at least two derivative-shadow slots"
        );
        reordered
            .variable_names
            .swap(shadow_slots[0], shadow_slots[1]);

        let first_layout = compiled_model_layout_identity(&model);
        let reordered_layout = compiled_model_layout_identity(&reordered);
        assert_ne!(
            first_layout.0, reordered_layout.0,
            "ordered variable slots must participate in executable cache identity"
        );

        let mut cache = NativeCompileCache::default();
        let first_key = NativeCompileCacheKey::CanonicalMir {
            mir_digest: "same-mir".into(),
            source_digest: model.source_digest.clone(),
            module: model.name.clone(),
            layout: first_layout,
        };
        let reordered_key = NativeCompileCacheKey::CanonicalMir {
            mir_digest: "same-mir".into(),
            source_digest: model.source_digest.clone(),
            module: model.name.clone(),
            layout: reordered_layout,
        };
        cache.insert(first_key, Err("first layout".into()));
        assert!(
            cache.get(&reordered_key).is_none(),
            "a same-source cache entry must not be reused across variable-slot layouts"
        );
    }

    #[test]
    fn native_compile_cache_evicts_by_image_bytes_and_keeps_the_newest_entry() {
        let mut cache = NativeCompileCache::default();
        for index in 0..4_u32 {
            cache.insert(
                NativeCompileCacheKey::CanonicalMir {
                    mir_digest: SmolStr::new(format!("mir{index}")),
                    source_digest: SmolStr::new("src"),
                    module: SmolStr::new("m"),
                    layout: CompiledModelLayoutIdentity(blake3::hash(&index.to_le_bytes())),
                },
                Err(format!("failure {index}")),
            );
        }
        assert_eq!(cache.entries.len(), 4, "failures cost no image bytes");

        cache.image_bytes = 8;
        cache.evict_to(0);
        assert_eq!(
            cache.entries.len(),
            1,
            "eviction must retain the most recent entry so an oversized image cannot loop"
        );
    }

    fn compile(source: &str) -> CompiledModel {
        VerilogACompiler::new(CompilerOptions::default())
            .compile(source)
            .expect("Verilog-A source must compile")
    }

    fn native_test_device(model: CompiledModel) -> VerilogADevice {
        let model = Arc::new(model);
        let num_terminals = model.num_terminals;
        let num_internal_nodes = model.internal_nodes;
        let num_branch_unknowns = model.branch_sources.len();
        let num_stamp_programs = model.stamp_programs.len();
        let jacobian_entry_points = model
            .stamp_programs
            .iter()
            .map(|stamp| stamp.jacobian_programs.len())
            .collect::<Vec<_>>();
        let reactive_jacobian_entry_points = model
            .stamp_programs
            .iter()
            .map(|stamp| stamp.reactive_jacobians.len())
            .collect::<Vec<_>>();
        let native_jacobian_count = jacobian_entry_points.iter().sum();

        let mut context = VmContext::with_internal_nodes(num_terminals, num_internal_nodes);
        context.port_connected = vec![1; num_terminals];
        for (i, param) in model.parameters.iter().enumerate() {
            context.set_param(i, param.default);
        }
        context.param_given = vec![0; model.parameters.len()];
        context.variables.resize(model.num_variables, 0.0);
        context
            .configure_event_state_variables(&model.event_state_variables)
            .expect("native test model event-state layout configures");
        context.lookup_tables = model.lookup_tables.clone();
        context.laplace_filters = model.laplace_filters.clone();
        context.zi_filters = model.zi_filters.clone();
        VerilogADevice::preallocate_vm_runtime_state(&mut context, &model)
            .expect("native test model runtime state preallocates");

        let mut device = VerilogADevice {
            name: SmolStr::new("NTEST"),
            model,
            context,
            node_mapping: vec![0; num_terminals],
            internal_node_indices: vec![0; num_internal_nodes],
            num_internal_nodes,
            branch_current_indices: vec![0; num_branch_unknowns],
            program_active: vec![true; num_stamp_programs],
            branch_active: vec![true; num_branch_unknowns],
            matrix_indices: MatrixIndices::default(),
            stamp_matrix_buffer: Vec::new(),
            stamp_rhs_buffer: Vec::new(),
            native_model: Arc::new(NativeModel::new_for_test_with_shape(
                num_terminals,
                num_internal_nodes,
                0,
                num_stamp_programs,
                jacobian_entry_points,
                reactive_jacobian_entry_points,
            )),
            fused_program_active: vec![1; num_stamp_programs],
            fused_stamp_jacobians: vec![0.0; native_jacobian_count],
            // None, exactly as `try_new` would decide here: this helper builds
            // a device straight from a `CompiledModel` with no canonical IR
            // artifact, and a grouped-noise plan is built only from one. The
            // native tests below drive stamping and dispatch, never noise.
            canonical_noise_plan: None,
            prev_discontinuity: false,
        };
        device.context.branch_current_values = vec![0.0; num_branch_unknowns];
        device.rebuild_matrix_indices();
        device
    }

    fn assert_native_hard_fail(err: VmError, feature: &str) {
        let msg = err.to_string();
        assert!(
            msg.contains("native JIT"),
            "error must identify native JIT failure, got: {msg}"
        );
        assert!(
            msg.contains(feature),
            "error must identify {feature}, got: {msg}"
        );
        assert!(
            msg.contains("no interpreter fallback"),
            "error must state the hard-fail contract, got: {msg}"
        );
    }

    fn poisoned_bytecode_program() -> BytecodeProgram {
        BytecodeProgram {
            instructions: vec![Instruction::PushParam(999)],
        }
    }

    #[cfg(not(feature = "native-bytecode-contract-tests"))]
    #[test]
    fn native_try_new_requires_canonical_ir() {
        let model = compile(
            r#"
`include "disciplines.vams"
module native_try_new_requires_canonical(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n);
endmodule
"#,
        );

        let err = VerilogADevice::try_new("N1", model, &[1, 0])
            .expect_err("normal native try_new must require canonical IR");
        assert_native_hard_fail(err, "requires canonical IR");
    }

    #[test]
    fn native_device_builder_requires_canonical_ir() {
        let model = compile(
            r#"
`include "disciplines.vams"
module builder_requires_canonical(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n);
endmodule
"#,
        );

        let err = DeviceBuilder::new(model, "B1")
            .nodes(&[1, 0])
            .try_build()
            .expect_err("native DeviceBuilder must require canonical IR");

        assert_native_hard_fail(err, "DeviceBuilder requires canonical IR");
    }

    #[test]
    fn native_device_builder_uses_canonical_ir() {
        let source = r#"
`include "disciplines.vams"
module builder_uses_canonical(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 2.0;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile builder model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile builder canonical IR");

        let mut device = DeviceBuilder::new(model, "B2")
            .nodes(&[1, 0])
            .canonical_ir(artifact)
            .try_build()
            .expect("native DeviceBuilder uses canonical IR");

        assert!(device.is_using_native());
        device.update_voltages(&[4.0]);
        assert_eq!(
            device
                .try_evaluate()
                .expect("canonical builder device evaluates"),
            vec![2.0]
        );
    }

    #[test]
    fn native_device_runs_completed_current_assignments_on_all_value_paths() {
        let source = r#"
`include "disciplines.vams"
module completed_current_assignment_paths(p, n);
    inout p, n;
    electrical p, n, x;
    real sensed, reverse, port_n;
    analog begin
        I(x, n) <+ 2.0 * V(p, n);
        I(x, n) <+ 1.0;
        sensed = I(x, n);
        reverse = I(n, x);
        port_n = I(<n>);
    end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler
            .compile(source)
            .expect("compile completed-current device model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile completed-current canonical IR");
        let mut device =
            VerilogADevice::try_new_with_canonical_ir("POSTCURRENT1", model, &artifact, &[1, 0])
                .expect("build completed-current native device");
        let solution = [2.0_f64, 0.0_f64];
        device
            .try_update_all_voltages(&solution)
            .expect("update terminal and internal voltages");

        let assert_outputs = |device: &VerilogADevice| {
            assert_eq!(device.variable("sensed"), Some(5.0));
            assert_eq!(device.variable("reverse"), Some(-5.0));
            assert_eq!(device.variable("port_n"), Some(-5.0));
        };

        assert_eq!(
            device.try_evaluate().expect("native device evaluation"),
            vec![4.0, 1.0]
        );
        assert_outputs(&device);

        device
            .try_compute_jacobian()
            .expect("native standalone Jacobian evaluation");
        assert_outputs(&device);

        device
            .try_stamp(&solution, |_, _, _| {}, |_, _| {})
            .expect("native fused stamp evaluation");
        assert_outputs(&device);

        assert!(
            device
                .try_noise_sources(&solution)
                .expect("native noise operating-point evaluation")
                .is_empty()
        );
        assert_outputs(&device);
    }

    #[test]
    fn native_scalar_stamp_preserves_inactive_contribution_current_slots() {
        let source = r#"
`include "disciplines.vams"
module inactive_prior_current_slot(p, n);
    inout p, n;
    electrical p, n, x;
    parameter integer enabled = 0;
    real sensed;
    analog begin
        if (enabled)
            I(x, n) <+ 10.0;
        I(x, n) <+ I(x, n) + 1.0;
        sensed = I(x, n);
    end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler
            .compile(source)
            .expect("compile inactive prior-current model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile inactive prior-current canonical IR");
        let mut device =
            VerilogADevice::try_new_with_canonical_ir("INACTIVEPRIOR1", model, &artifact, &[1, 0])
                .expect("build inactive prior-current native device");

        device
            .try_stamp(&[0.0, 0.0], |_, _, _| {}, |_, _| {})
            .expect("scalar stamp retains the inactive contribution slot");

        assert_eq!(device.context.currents, vec![0.0, 1.0]);
        assert_eq!(device.variable("sensed"), Some(1.0));
    }

    #[test]
    fn native_stamp_paths_publish_no_solver_callbacks_after_late_runtime_failure() {
        let source = r#"
`include "disciplines.vams"
module transactional_stamp_failure(p, n);
    inout p, n;
    electrical p, n;
    integer idx;
    real values[0:0];
    analog begin
        idx = V(p, n);
        I(p, n) <+ V(p, n);
        I(p, n) <+ values[idx];
    end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler
            .compile(source)
            .expect("compile transactional stamp fixture");
        assert_eq!(
            model.stamp_programs.len(),
            2,
            "fixture requires a valid contribution before the failing one"
        );
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile transactional stamp canonical IR");
        let mut device = VerilogADevice::try_new_with_canonical_ir(
            "STAMPTRANSACTION1",
            model,
            &artifact,
            &[1, 0],
        )
        .expect("build transactional native device");
        assert!(
            device.native_model.stamp_kernel_is_eligible(),
            "fixture must exercise both scalar and fused stamp paths"
        );

        let mut scalar_matrix_calls = 0usize;
        let mut scalar_rhs_calls = 0usize;
        let scalar_error = device
            .try_stamp_scalar_with_mode(
                &[2.0, 0.0],
                |_, _, _| scalar_matrix_calls += 1,
                |_, _| scalar_rhs_calls += 1,
                crate::vm::VerilogAEvaluationMode::StaticProbe,
            )
            .expect_err("late scalar contribution failure must surface");
        assert!(
            scalar_error
                .to_string()
                .contains("array index 2 outside declared bounds [0:0]"),
            "unexpected scalar stamp error: {scalar_error}"
        );
        assert_eq!(
            (scalar_matrix_calls, scalar_rhs_calls),
            (0, 0),
            "scalar stamping must publish atomically"
        );

        let mut fused_matrix_calls = 0usize;
        let mut fused_rhs_calls = 0usize;
        let fused_error = device
            .try_stamp_with_mode(
                &[2.0, 0.0],
                |_, _, _| fused_matrix_calls += 1,
                |_, _| fused_rhs_calls += 1,
                crate::vm::VerilogAEvaluationMode::StaticProbe,
            )
            .expect_err("late fused contribution failure must surface");
        assert!(
            fused_error
                .to_string()
                .contains("array index 2 outside declared bounds [0:0]"),
            "unexpected fused stamp error: {fused_error}"
        );
        assert_eq!(
            (fused_matrix_calls, fused_rhs_calls),
            (0, 0),
            "fused stamping must publish atomically"
        );
    }

    #[test]
    fn named_limiter_device_modes_preserve_history_and_report_convergence() {
        let source = r#"
`include "disciplines.vams"
module limiter_device_modes(p, n);
    inout p, n;
    electrical p, n;
    analog function real force_value;
        input proposed, previous, forced;
        real proposed, previous, forced;
        begin
            force_value = forced;
        end
    endfunction
    analog I(p, n) <+ $limit(V(p, n), "force_value", 0.1);
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler
            .compile(source)
            .expect("compile limiter device model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile limiter device canonical IR");
        let mut device =
            VerilogADevice::try_new_with_canonical_ir("LIMITMODES1", model, &artifact, &[1, 0])
                .expect("build native limiter device");

        device
            .try_stamp_with_mode(
                &[0.5],
                |_, _, _| {},
                |_, _| {},
                crate::vm::VerilogAEvaluationMode::NewtonLimited,
            )
            .expect("limited Newton stamp");
        assert_eq!(device.context.currents, vec![0.1]);
        assert!(!device.limiter_converged());
        let limited_history = device.context.state_values.clone();
        let limited_initialized = device.context.state_initialized.clone();
        assert!(limited_initialized.iter().any(|initialized| *initialized));

        device
            .try_compute_jacobian()
            .expect("standalone Jacobian query after limited value pass");
        assert_eq!(device.context.state_values, limited_history);
        assert_eq!(device.context.state_initialized, limited_initialized);
        assert!(
            !device.limiter_converged(),
            "standalone Jacobian evaluation must preserve the value-pass convergence result"
        );

        device
            .try_stamp_with_mode(
                &[0.8],
                |_, _, _| {},
                |_, _| {},
                crate::vm::VerilogAEvaluationMode::StaticProbe,
            )
            .expect("static probe stamp");
        assert_eq!(device.context.currents, vec![0.8]);
        assert_eq!(device.context.state_values, limited_history);
        assert_eq!(device.context.state_initialized, limited_initialized);
        assert!(
            !device.limiter_converged(),
            "static probe must not rewrite the preceding Newton convergence result"
        );

        device
            .try_stamp_with_mode(
                &[0.9],
                |_, _, _| {},
                |_, _| {},
                crate::vm::VerilogAEvaluationMode::SmallSignal,
            )
            .expect("small-signal stamp");
        assert_eq!(device.context.currents, vec![0.9]);
        assert_eq!(device.context.state_values, limited_history);
        assert_eq!(device.context.state_initialized, limited_initialized);
        assert!(!device.limiter_converged());

        device
            .try_stamp_with_mode(
                &[0.1],
                |_, _, _| {},
                |_, _| {},
                crate::vm::VerilogAEvaluationMode::NewtonLimited,
            )
            .expect("converged limited Newton stamp");
        assert_eq!(device.context.currents, vec![0.1]);
        assert!(
            device.limiter_converged(),
            "a new limited stamp must clear active once before its value and Jacobian passes"
        );
    }

    #[cfg(feature = "native-bytecode-contract-tests")]
    #[test]
    fn named_limiter_bytecode_contract_construction_fails_closed() {
        let source = r#"
`include "disciplines.vams"
module named_limiter_requires_canonical(p, n);
    inout p, n;
    electrical p, n;
    analog function real force_value;
        input proposed, previous, forced;
        real proposed, previous, forced;
        begin
            force_value = forced;
        end
    endfunction
    analog I(p, n) <+ $limit(V(p, n), "force_value", 0.1);
endmodule
"#;
        let model = compile(source);
        assert!(
            model.stamp_programs.iter().any(|program| {
                program
                    .value_program
                    .instructions
                    .iter()
                    .any(|instruction| matches!(instruction, Instruction::CanonicalLimitState(_)))
            }),
            "named limiter must carry an explicit non-executable canonical state marker"
        );

        let err = VerilogADevice::try_new("LIMITBYTECODE1", model, &[1, 0])
            .expect_err("named limiter bytecode construction must fail closed");
        assert_native_hard_fail(err, "canonical-only named limiter metadata");
    }

    #[cfg(feature = "native-bytecode-contract-tests")]
    #[test]
    fn native_compile_cache_prunes_dropped_models_before_fresh_compile() {
        let source = r#"
`include "disciplines.vams"
module native_cache_churn_guard(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 2.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());

        let stale_image = {
            let model = Arc::new(compiler.compile(source).expect("compile first model"));
            let mut first = VerilogADevice::try_new("CACHEOLD1", Arc::clone(&model), &[1, 0])
                .expect("first model compiles to native");
            let second = VerilogADevice::try_new("CACHEOLD2", Arc::clone(&model), &[1, 0])
                .expect("same model reuses native cache");

            assert!(
                Arc::ptr_eq(&first.native_model, &second.native_model),
                "same CompiledModel Arc should share one native image"
            );
            first.update_voltages(&[4.0]);
            let currents = first.try_evaluate().expect("cached native image evaluates");
            assert_eq!(currents, vec![2.0]);

            Arc::downgrade(&first.native_model)
        };

        let model = Arc::new(compiler.compile(source).expect("compile fresh model"));
        let mut fresh = VerilogADevice::try_new("CACHEFRESH1", Arc::clone(&model), &[1, 0])
            .expect("fresh model compiles after stale cache entry");
        assert!(fresh.is_using_native());
        assert!(
            stale_image.upgrade().is_none(),
            "fresh compile must prune native image cached only for a dropped CompiledModel"
        );

        fresh.update_voltages(&[6.0]);
        let currents = fresh
            .try_evaluate()
            .expect("fresh native image evaluates after cache churn");
        assert_eq!(currents, vec![3.0]);
    }

    #[test]
    fn preallocates_runtime_state_from_all_native_entry_surfaces() {
        let mut model = compile(
            r#"
`include "disciplines.vams"
module prealloc_reactive_noise(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 1.0;
    analog I(p, n) <+ V(p, n) * 0.0;
endmodule
"#,
        );
        assert_eq!(model.stamp_programs.len(), 1);

        model.parameters[0].default_program = Some(BytecodeProgram {
            instructions: vec![Instruction::LimitState(8)],
        });
        model.stamp_programs[0].static_condition = Some(BytecodeProgram {
            instructions: vec![
                Instruction::AbsDelayState(6),
                Instruction::AbsDelayStateMax(9),
            ],
        });
        model.stamp_programs[0]
            .reactive_jacobians
            .push(JacobianEntry {
                row: StampIndex::Terminal(0),
                col: StampIndex::Terminal(0),
                col_axis: ColumnAxis::Node(0),
                sign: 1.0,
                program: BytecodeProgram {
                    instructions: vec![
                        Instruction::DdtState(3),
                        Instruction::AbsDelayState(2),
                        Instruction::AbsDelayStateDerivative(10),
                        Instruction::TransitionState(1),
                    ],
                },
            });
        model.noise_sources.push(CompiledNoiseSource {
            process_id: 0,
            pos: StampIndex::Terminal(0),
            neg: StampIndex::Ground,
            is_current: true,
            branch_ordinal: None,
            program_idx: 0,
            psd_program: BytecodeProgram {
                instructions: vec![
                    Instruction::IdtState(5),
                    Instruction::SlewState(4),
                    Instruction::AbsDelayStateDerivativeMax(11),
                ],
            },
            exponent_program: Some(BytecodeProgram {
                instructions: vec![
                    Instruction::LimitState(6),
                    Instruction::CrossState(5),
                    Instruction::AboveState(7),
                    Instruction::LastCrossingState(8),
                ],
            }),
            table: None,
            name: None,
            injections: Vec::new(),
        });

        let mut context = VmContext::with_internal_nodes(model.num_terminals, model.internal_nodes);
        VerilogADevice::preallocate_vm_runtime_state(&mut context, &model)
            .expect("stateful test model runtime state preallocates");

        assert_eq!(context.state_values.len(), 9);
        assert_eq!(context.state_values_prev.len(), 9);
        assert_eq!(context.state_initialized.len(), 9);
        assert_eq!(context.delay_buffers.len(), 12);
        assert_eq!(context.transition_filters.len(), 2);
        assert_eq!(context.slew_filters.len(), 5);
        assert_eq!(context.cross_detectors.len(), 9);
    }

    #[test]
    fn native_runtime_preallocation_rejects_state_slot_count_overflow() {
        let mut model = compile(
            r#"
`include "disciplines.vams"
module native_state_slot_overflow(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n);
endmodule
"#,
        );
        model.stamp_programs[0]
            .value_program
            .instructions
            .push(Instruction::TransitionState(usize::MAX));

        let mut context = VmContext::with_internal_nodes(model.num_terminals, model.internal_nodes);
        let err = VerilogADevice::preallocate_vm_runtime_state(&mut context, &model)
            .expect_err("overflowing runtime state slot must hard-fail");

        assert_native_hard_fail(err, "transition-filter runtime state slot");
    }

    #[cfg(feature = "native-bytecode-contract-tests")]
    #[test]
    fn native_runtime_helper_error_reaches_device_as_native_jit_error() {
        let model = compile(
            r#"
`include "disciplines.vams"
module native_laplace_error_path(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ laplace_nd(V(p, n), '{1.0}, '{1.0, 1.0});
endmodule
"#,
        );

        let mut device = VerilogADevice::try_new("LERR1", model, &[1, 0])
            .expect("laplace model uses native JIT");
        device.update_voltages(&[1.0]);
        assert_eq!(
            device.context.laplace_filters.len(),
            1,
            "fixture must allocate one Laplace filter"
        );
        device.context.laplace_filters.clear();

        let err = device
            .try_evaluate()
            .expect_err("native helper metadata error must hard-fail");

        assert_native_hard_fail(err, "Laplace");
    }

    #[test]
    fn native_state_storage_preflights_before_dispatch() {
        let source = r#"
`include "disciplines.vams"
module state_storage_preflight(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ ddt(V(p, n));
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile state model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile state canonical IR");
        let mut device =
            VerilogADevice::try_new_with_canonical_ir("STATEPRE1", model, &artifact, &[1, 0])
                .expect("state model uses native JIT");
        assert_eq!(
            device.context.state_values.len(),
            1,
            "fixture must allocate one state slot"
        );

        device.context.state_values.clear();
        let err = device
            .try_evaluate()
            .expect_err("missing state storage must preflight before native dispatch");

        assert_native_hard_fail(err, "state-value storage");
    }

    #[test]
    fn native_integration_history_storage_preflights_before_dispatch() {
        fn populated_context() -> VmContext {
            let mut context = VmContext::default();
            context.state_values.resize(1, 0.0);
            context.state_values_prev.resize(1, 0.0);
            context.state_values_older.resize(1, 0.0);
            context.state_derivatives.resize(1, 0.0);
            context.state_derivatives_prev.resize(1, 0.0);
            context.state_initialized.resize(1, false);
            context.state_candidate_valid.resize(1, 0);
            context
        }

        let required = NativeRequiredStorage {
            state_values: 1,
            state_values_prev: 1,
            state_initialized: 1,
            state_candidate_valid: 1,
            ..NativeRequiredStorage::default()
        };

        let mut context = populated_context();
        context.state_values_older.clear();
        let error = VerilogADevice::validate_native_runtime_storage(&context, required)
            .expect_err("missing older state storage must hard-fail before dispatch");
        assert_native_hard_fail(error, "older state-value storage");

        let mut context = populated_context();
        context.state_derivatives.clear();
        let error = VerilogADevice::validate_native_runtime_storage(&context, required)
            .expect_err("missing candidate derivative storage must hard-fail before dispatch");
        assert_native_hard_fail(error, "candidate state-derivative storage");

        let mut context = populated_context();
        context.state_derivatives_prev.clear();
        let error = VerilogADevice::validate_native_runtime_storage(&context, required)
            .expect_err("missing prior derivative storage must hard-fail before dispatch");
        assert_native_hard_fail(error, "prior state-derivative storage");

        let mut context = populated_context();
        context.state_candidate_valid.clear();
        let error = VerilogADevice::validate_native_runtime_storage(&context, required)
            .expect_err("missing integration candidate storage must hard-fail before dispatch");
        assert_native_hard_fail(error, "integration candidate-valid storage");
    }

    #[test]
    fn native_transition_storage_preflights_before_dispatch() {
        let source = r#"
`include "disciplines.vams"
module transition_storage_preflight(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ transition(V(p, n), 0.0, 1.0e-9, 1.0e-9);
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile transition model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile transition canonical IR");
        let mut device =
            VerilogADevice::try_new_with_canonical_ir("TRANPRE1", model, &artifact, &[1, 0])
                .expect("transition model uses native JIT");
        assert_eq!(
            device.context.transition_filters.len(),
            1,
            "fixture must allocate one transition filter"
        );

        device.context.transition_filters.clear();
        let err = device
            .try_evaluate()
            .expect_err("missing transition filter storage must preflight before native dispatch");

        assert_native_hard_fail(err, "transition filter storage");
    }

    #[test]
    fn native_current_pair_preflight_errors_use_hard_fail_contract() {
        let context = VmContext::with_internal_nodes(0, 0);
        let err = VerilogADevice::validate_native_current_pairs(&context, 0, &[0])
            .expect_err("missing terminal-pair storage must hard-fail in native mode");

        assert_native_hard_fail(err, "terminal-pair current slot 0");
    }

    #[test]
    fn native_current_pair_preflight_rejects_terminal_count_mismatch() {
        let context = VmContext::with_internal_nodes(2, 0);
        let err = VerilogADevice::validate_native_current_pairs(&context, 1, &[0])
            .expect_err("terminal-pair preflight must use the compiled terminal shape");

        assert_native_hard_fail(err, "terminal context");
    }

    #[test]
    fn native_voltage_storage_rejects_terminal_count_mismatch() {
        let context = VmContext::with_internal_nodes(2, 0);
        let err = VerilogADevice::validate_native_voltage_storage(&context, 1, 0)
            .expect_err("native dispatch must reject stale terminal context shapes");

        assert_native_hard_fail(err, "terminal context");
    }

    #[test]
    fn native_parameter_storage_preflights_before_dispatch() {
        let context = VmContext::with_internal_nodes(2, 0);
        let err = VerilogADevice::validate_native_parameter_storage(&context, 1)
            .expect_err("missing parameter storage must hard-fail in native mode");

        assert_native_hard_fail(err, "parameter storage");
    }

    #[test]
    fn native_param_given_storage_preflights_before_dispatch() {
        let mut context = VmContext::with_internal_nodes(2, 0);
        context.parameters = vec![1.0];
        let err = VerilogADevice::validate_native_parameter_storage(&context, 1)
            .expect_err("missing parameter-given storage must hard-fail in native mode");

        assert_native_hard_fail(err, "parameter-given storage");
    }

    #[test]
    fn native_variable_storage_preflights_before_dispatch() {
        let context = VmContext::with_internal_nodes(2, 0);
        let err = VerilogADevice::validate_native_variable_storage(&context, 1)
            .expect_err("missing variable storage must hard-fail in native mode");

        assert_native_hard_fail(err, "variable storage");
    }

    #[test]
    fn native_terminal_voltage_storage_preflights_before_dispatch() {
        let context = VmContext::with_internal_nodes(1, 0);
        let err = VerilogADevice::validate_native_voltage_storage(&context, 2, 0)
            .expect_err("missing terminal-voltage storage must hard-fail in native mode");

        assert_native_hard_fail(err, "voltage storage");
    }

    #[test]
    fn native_internal_voltage_storage_preflights_before_dispatch() {
        let context = VmContext::with_internal_nodes(2, 0);
        let err = VerilogADevice::validate_native_voltage_storage(&context, 2, 1)
            .expect_err("missing internal-voltage storage must hard-fail in native mode");

        assert_native_hard_fail(err, "internal-voltage storage");
    }

    #[test]
    fn native_port_connected_storage_preflights_before_dispatch() {
        let mut context = VmContext::with_internal_nodes(2, 0);
        context.port_connected.clear();
        let err = VerilogADevice::validate_native_voltage_storage(&context, 2, 0)
            .expect_err("missing port-connected storage must hard-fail in native mode");

        assert_native_hard_fail(err, "port-connected storage");
    }

    #[test]
    fn native_laplace_storage_preflights_before_dispatch() {
        let context = VmContext::with_internal_nodes(2, 0);
        let required = NativeRequiredStorage {
            laplace_filters: 1,
            ..NativeRequiredStorage::default()
        };
        let err = VerilogADevice::validate_native_runtime_storage(&context, required)
            .expect_err("missing Laplace storage must hard-fail in native mode");

        assert_native_hard_fail(err, "Laplace filter storage");
    }

    #[test]
    fn native_branch_unknown_preflight_errors_use_hard_fail_contract() {
        let context = VmContext::with_internal_nodes(0, 0);
        let err = VerilogADevice::validate_native_branch_unknowns(&context, &[0])
            .expect_err("missing branch-current storage must hard-fail in native mode");

        assert_native_hard_fail(err, "branch-current unknown 0");
    }

    #[test]
    fn native_assignment_branch_unknown_preflights_before_dispatch() {
        let source = r#"
`include "disciplines.vams"
module assignment_branch_unknown_preflight(p, n);
    inout p, n;
    electrical p, n;
    real sensed;
    analog begin
        sensed = I(p, n);
        V(p, n) <+ sensed;
    end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile assignment model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile assignment canonical IR");
        let mut device =
            VerilogADevice::try_new_with_canonical_ir("ABRANCH1", model, &artifact, &[1, 0])
                .expect("assignment branch-current model uses native JIT");
        assert_eq!(
            device.context.branch_current_values.len(),
            1,
            "fixture must allocate one branch-current unknown"
        );

        device.context.branch_current_values.clear();
        let err = device
            .try_evaluate()
            .expect_err("assignment branch-current load must preflight before native dispatch");

        assert_native_hard_fail(err, "branch-current unknown 0");
    }

    #[test]
    fn native_missing_noise_exponent_entry_uses_hard_fail_contract() {
        let err = VerilogADevice::missing_native_noise_exponent_entry(2);

        assert_native_hard_fail(err, "noise exponent entry");
    }

    #[test]
    fn native_noise_exponent_missing_entry_preflights_before_dependency_tables() {
        let native = NativeModel::new_for_test(0, 0, vec![], vec![]);
        let mut context = VmContext::with_internal_nodes(0, 0);
        let mut vm = Vm::new(&mut context);
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushConst(1.0)],
        };

        let err = VerilogADevice::run_value_program(
            &mut vm,
            &program,
            &native,
            NativeValueEntry::NoiseExponent(0),
        )
        .expect_err("missing optional native noise exponent entry must not index dependencies");

        assert_native_hard_fail(err, "noise exponent entry");
    }

    #[test]
    fn native_missing_static_condition_entry_hard_fails_before_dependency_tables() {
        let native = NativeModel::new_for_test(0, 0, vec![], vec![]);
        let mut context = VmContext::with_internal_nodes(0, 0);
        let mut vm = Vm::new(&mut context);
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushConst(1.0)],
        };

        let err = VerilogADevice::run_value_program(
            &mut vm,
            &program,
            &native,
            NativeValueEntry::StaticCondition(0),
        )
        .expect_err("missing static condition entry must not index dependencies");

        assert_native_hard_fail(err, "static-condition entry");
    }

    #[test]
    fn native_missing_stamp_value_entry_hard_fails_before_dependency_tables() {
        let native = NativeModel::new_for_test(0, 0, vec![], vec![]);
        let mut context = VmContext::with_internal_nodes(0, 0);
        let mut vm = Vm::new(&mut context);
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushConst(1.0)],
        };

        let err = VerilogADevice::run_value_program(
            &mut vm,
            &program,
            &native,
            NativeValueEntry::StampValue(0),
        )
        .expect_err("missing stamp entry must not index dependencies");

        assert_native_hard_fail(err, "stamp-value entry 0");
    }

    #[test]
    fn native_missing_jacobian_entry_hard_fails_before_dependency_tables() {
        let native = NativeModel::new_for_test(0, 1, vec![0], vec![0]);
        let mut context = VmContext::with_internal_nodes(0, 0);
        let mut vm = Vm::new(&mut context);
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushConst(1.0)],
        };

        let err = VerilogADevice::run_value_program(
            &mut vm,
            &program,
            &native,
            NativeValueEntry::Jacobian { stamp: 0, entry: 0 },
        )
        .expect_err("missing Jacobian entry must not index dependencies");

        assert_native_hard_fail(err, "Jacobian entry 0.0");
    }

    #[test]
    fn native_missing_reactive_jacobian_entry_hard_fails_before_dependency_tables() {
        let native = NativeModel::new_for_test(0, 1, vec![0], vec![0]);
        let mut context = VmContext::with_internal_nodes(0, 0);
        let mut vm = Vm::new(&mut context);
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushConst(1.0)],
        };

        let err = VerilogADevice::run_value_program(
            &mut vm,
            &program,
            &native,
            NativeValueEntry::ReactiveJacobian { stamp: 0, entry: 0 },
        )
        .expect_err("missing reactive-Jacobian entry must not index dependencies");

        assert_native_hard_fail(err, "reactive-Jacobian entry 0.0");
    }

    #[test]
    fn native_missing_noise_psd_entry_hard_fails_before_dependency_tables() {
        let native = NativeModel::new_for_test(0, 0, vec![], vec![]);
        let mut context = VmContext::with_internal_nodes(0, 0);
        let mut vm = Vm::new(&mut context);
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushConst(1.0)],
        };

        let err = VerilogADevice::run_value_program(
            &mut vm,
            &program,
            &native,
            NativeValueEntry::NoisePsd(0),
        )
        .expect_err("missing noise PSD entry must not index dependencies");

        assert_native_hard_fail(err, "noise PSD entry");
    }

    #[test]
    fn native_missing_parameter_default_entry_hard_fails_without_bytecode_execution() {
        let model = compile(
            r#"
`include "disciplines.vams"
module dependent_default(p, n);
    inout p, n;
    electrical p, n;
    parameter real base = 2.0;
    parameter real derived = base * 3.0;
    analog I(p, n) <+ V(p, n) * derived;
endmodule
"#,
        );
        assert!(
            model
                .parameters
                .iter()
                .any(|param| param.default_program.is_some()),
            "fixture must contain a dependent parameter default program"
        );

        let mut device = native_test_device(model);
        let err = device
            .try_resolve_parameter_defaults()
            .expect_err("native mode must not execute dependent default bytecode fallback");

        assert_native_hard_fail(err, "missing parameter-default entry");
    }

    #[test]
    fn native_evaluate_and_jacobian_use_native_entries_without_bytecode_execution() {
        let mut model = compile(
            r#"
`include "disciplines.vams"
module poison_value_paths(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n) * 2.0;
endmodule
"#,
        );
        assert_eq!(model.stamp_programs.len(), 1);
        assert!(
            !model.stamp_programs[0].jacobian_programs.is_empty(),
            "fixture must contain Jacobian bytecode"
        );

        model.stamp_programs[0].value_program = poisoned_bytecode_program();
        for jacobian in &mut model.stamp_programs[0].jacobian_programs {
            jacobian.program = poisoned_bytecode_program();
        }

        let mut device = native_test_device(model);

        let currents = device
            .try_evaluate()
            .expect("native evaluation must ignore poisoned stamp bytecode");
        assert_eq!(currents, vec![1.0]);

        let jacobians = device
            .try_compute_jacobian()
            .expect("native Jacobian evaluation must ignore poisoned bytecode");
        assert!(
            !jacobians.is_empty(),
            "fixture should still expose native Jacobian entries"
        );
        assert!(
            jacobians
                .iter()
                .all(|entry| entry.value.abs().to_bits() == 2.0_f64.to_bits()),
            "native test Jacobian stubs should supply magnitude 2.0, got {jacobians:?}"
        );
    }

    #[test]
    fn native_static_condition_refresh_uses_native_entries_without_bytecode_execution() {
        let mut model = compile(
            r#"
`include "disciplines.vams"
module static_condition(p, n);
    inout p, n;
    electrical p, n;
    parameter real enabled = 1.0;
    real guard;
    analog begin
        guard = enabled;
        if (guard)
            I(p, n) <+ V(p, n) * 1.0e-3;
    end
endmodule
"#,
        );
        assert!(
            model
                .stamp_programs
                .iter()
                .any(|program| program.static_condition.is_some()),
            "fixture must contain a static condition program"
        );
        assert!(
            !model.assignment_steps.is_empty(),
            "fixture must contain assignment bytecode"
        );
        model.assignment_steps = vec![AssignmentStep::Assign(AssignmentProgram {
            var_index: 0,
            program: BytecodeProgram {
                instructions: vec![Instruction::PushParam(999)],
            },
        })];
        for program in &mut model.stamp_programs {
            if program.static_condition.is_some() {
                program.static_condition = Some(BytecodeProgram {
                    instructions: vec![Instruction::PushParam(999)],
                });
            }
        }

        let mut device = native_test_device(model);
        device
            .try_resolve_parameter_defaults()
            .expect("native mode must refresh static conditions without bytecode fallback");

        assert!(device.program_active.iter().all(|active| *active));
    }

    #[test]
    fn native_analysis_static_condition_refreshes_when_analysis_type_changes() {
        let source = r#"
`include "disciplines.vams"
module analysis_static_condition(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        if (analysis("static"))
            I(p, n) <+ V(p, n);
    end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler
            .compile(source)
            .expect("compile analysis static-condition model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile analysis static-condition canonical IR");
        assert!(
            model
                .stamp_programs
                .iter()
                .any(|program| program.static_condition.is_some()),
            "analysis(\"static\") guard should be peeled into a topology guard"
        );

        let mut device =
            VerilogADevice::try_new_with_canonical_ir("ANSTATIC1", model, &artifact, &[1, 0])
                .expect("analysis static condition compiles natively");

        assert!(
            device.program_active.iter().all(|active| *active),
            "default DC analysis should activate analysis(\"static\") guarded stamps"
        );

        device
            .try_set_analysis_type(2)
            .expect("transient analysis change refreshes native static conditions");
        assert!(
            device.program_active.iter().all(|active| !*active),
            "transient analysis should deactivate analysis(\"static\") guarded stamps"
        );

        device
            .try_set_analysis_type(4)
            .expect("IC analysis change refreshes native static conditions");
        assert!(
            device.program_active.iter().all(|active| *active),
            "IC analysis should reactivate analysis(\"static\") guarded stamps"
        );
    }

    #[test]
    fn native_static_refresh_without_conditions_does_not_execute_assignment_bytecode() {
        let mut model = compile(
            r#"
`include "disciplines.vams"
module unconditional_vsource(p, n);
    inout p, n;
    electrical p, n;
    real value;
    analog begin
        value = 1.0;
        V(p, n) <+ value;
    end
endmodule
"#,
        );
        assert!(
            !model.assignment_steps.is_empty(),
            "fixture must contain assignment bytecode"
        );
        assert!(
            !model.branch_sources.is_empty(),
            "fixture must contain a branch-current unknown"
        );
        assert!(
            model
                .stamp_programs
                .iter()
                .all(|program| program.static_condition.is_none()),
            "fixture must not contain static condition programs"
        );
        model.assignment_steps = vec![AssignmentStep::Assign(AssignmentProgram {
            var_index: 0,
            program: BytecodeProgram {
                instructions: vec![Instruction::PushParam(999)],
            },
        })];

        let mut device = native_test_device(model);
        device
            .try_refresh_static_conditions()
            .expect("models without static conditions require no assignment evaluation");

        assert!(device.program_active.iter().all(|active| *active));
        assert!(device.branch_active.iter().all(|active| *active));
    }
}
