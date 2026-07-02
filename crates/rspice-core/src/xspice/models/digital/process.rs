use crate::xspice::external::{
    DigitalProcessRuntime, DigitalProcessSpec, start_digital_process_runtime,
};
use crate::xspice::{
    CmContext, CmError, CmResult, CodeModel, DigitalState, DigitalStrength, DigitalValue,
    EvaluationPhase, ParamSpec, PortSpec, PortType,
};
use std::sync::{Arc, Mutex};

/// External digital process code model.
#[derive(Debug, Default)]
pub struct DigitalProcess;

type DigitalProcessRuntimeResource = Mutex<Box<dyn DigitalProcessRuntime>>;
type DigitalProcessIoScratchResource = DigitalProcessIoScratch;

const RESOURCE_RUNTIME: &str = "d_process.runtime";
const RESOURCE_IO_SCRATCH: &str = "d_process.io_scratch";
const STATE_PREV_CLK: usize = 0;
const STATE_PREV_RESET: usize = 1;
const STATE_RNG: usize = 2;
const STATE_DOUT_START: usize = 3;
const STATE_ZERO: i64 = 0;
const STATE_ONE: i64 = 1;
const STATE_UNKNOWN: i64 = 2;
const D_PROCESS_RNG_SEED: i64 = 0x4d59_5df4;
const D_PROCESS_MAX_PROCESS_PARAMS: usize = 1022;

fn d_process_error(message: impl Into<String>) -> CmError {
    CmError::EvaluationError(format!("d_process: {}", message.into()))
}

fn d_process_delay(ctx: &CmContext, name: &str, default: f64) -> CmResult<f64> {
    let value = ctx.param_or(name, default);
    if !value.is_finite() {
        return Err(CmError::InvalidParameter {
            name: name.to_string(),
            message: format!("value must be finite, got {value}"),
        });
    }
    Ok(value)
}

#[derive(Default)]
struct DigitalProcessIoScratch {
    input_bytes: Vec<u8>,
    output_bytes: Vec<u8>,
}

fn with_io_scratch<R>(
    ctx: &mut CmContext,
    f: impl FnOnce(&mut CmContext, &mut DigitalProcessIoScratch) -> CmResult<R>,
) -> CmResult<R> {
    let mut scratch = {
        let scratch = ctx
            .resource_mut::<DigitalProcessIoScratchResource>(RESOURCE_IO_SCRATCH)
            .ok_or_else(|| {
                d_process_error("I/O scratch is not initialized or is not uniquely owned")
            })?;
        std::mem::take(scratch)
    };
    let result = f(ctx, &mut scratch);
    let restore = ctx
        .resource_mut::<DigitalProcessIoScratchResource>(RESOURCE_IO_SCRATCH)
        .ok_or_else(|| d_process_error("I/O scratch is not initialized or is not uniquely owned"))
        .map(|slot| {
            *slot = scratch;
        });

    match (result, restore) {
        (Ok(value), Ok(())) => Ok(value),
        (Err(err), Ok(())) => Err(err),
        (_, Err(err)) => Err(err),
    }
}

fn packed_byte_len(bit_count: usize) -> usize {
    if bit_count == 0 {
        0
    } else {
        (bit_count - 1) / 8 + 1
    }
}

fn digital_state_code(value: DigitalValue) -> i64 {
    match value.state.logic_level() {
        Some(false) => STATE_ZERO,
        Some(true) => STATE_ONE,
        None => STATE_UNKNOWN,
    }
}

fn output_value_from_code(code: i64) -> DigitalValue {
    match code {
        STATE_ZERO => DigitalValue::zero(),
        STATE_ONE => DigitalValue::one(),
        _ => DigitalValue::new(DigitalState::Unknown, DigitalStrength::HighZ),
    }
}

fn unknown_high_z() -> DigitalValue {
    DigitalValue::new(DigitalState::Unknown, DigitalStrength::HighZ)
}

fn set_unknown_outputs(ctx: &mut CmContext, output_width: usize) -> CmResult<()> {
    ctx.set_output_digital_vector_from_context_fn("out", output_width, 0.0, |_, _| unknown_high_z())
}

fn validate_shape(input_width: usize, output_width: usize) -> CmResult<()> {
    if input_width > u8::MAX as usize {
        return Err(d_process_error(format!(
            "input width {input_width} exceeds the official maximum of 255"
        )));
    }
    if output_width == 0 || output_width > u8::MAX as usize {
        return Err(d_process_error(format!(
            "output width {output_width} must be between 1 and 255"
        )));
    }
    Ok(())
}

fn next_unknown_input_bit(seed: &mut u32) -> bool {
    if *seed == 0 {
        *seed = D_PROCESS_RNG_SEED as u32;
    }
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;
    *seed & 1 != 0
}

fn resize_zeroed(bytes: &mut Vec<u8>, len: usize) -> CmResult<()> {
    bytes.clear();
    if bytes.capacity() < len {
        let additional = len - bytes.capacity();
        bytes.try_reserve_exact(additional).map_err(|err| {
            d_process_error(format!("unable to reserve {len} I/O byte(s): {err}"))
        })?;
    }
    bytes.resize(len, 0);
    Ok(())
}

fn pack_input_bits(ctx: &mut CmContext, input_width: usize, bytes: &mut Vec<u8>) -> CmResult<()> {
    resize_zeroed(bytes, packed_byte_len(input_width))?;
    let inputs = ctx.input_digital_vector_values("in").unwrap_or(&[]);
    let mut rng_seed = ctx.int_state(STATE_RNG) as u32;
    let mut rng_changed = false;

    for bit_index in 0..input_width {
        let value = inputs.get(bit_index).copied().unwrap_or_default();
        let bit = match digital_state_code(value) {
            STATE_ZERO => false,
            STATE_ONE => true,
            _ => {
                rng_changed = true;
                next_unknown_input_bit(&mut rng_seed)
            }
        };
        if bit {
            bytes[bit_index >> 3] |= 1u8 << (bit_index & 7);
        }
    }

    if rng_changed {
        ctx.set_int_state(STATE_RNG, rng_seed as i64);
    }
    Ok(())
}

fn output_code(output_bytes: &[u8], bit_index: usize) -> i64 {
    if ((output_bytes[bit_index >> 3] >> (bit_index & 7)) & 1) != 0 {
        STATE_ONE
    } else {
        STATE_ZERO
    }
}

impl CodeModel for DigitalProcess {
    fn name(&self) -> &str {
        "d_process"
    }

    fn description(&self) -> &str {
        "External digital process"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::vector_input("in", PortType::Digital)
                    .nullable()
                    .with_description("Input vector"),
                PortSpec::input("clk", PortType::Digital).with_description("Clock input"),
                PortSpec::input("reset", PortType::Digital)
                    .nullable()
                    .with_description("Synchronous reset input"),
                PortSpec::vector_output("out", PortType::Digital)
                    .with_vector_min_len(1)
                    .with_description("Output vector"),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("clk_delay", 1.0e-9),
                ParamSpec::real("reset_delay", 1.0e-9),
                ParamSpec::string("process_file", "")
                    .required()
                    .with_description("Executable process path or provider identifier"),
                ParamSpec::string_vector("process_params", Vec::new())
                    .with_description("Executable process arguments"),
                ParamSpec::integer("reset_state", 0),
                ParamSpec::real("input_load", 1.0e-12),
                ParamSpec::real("clk_load", 1.0e-12),
                ParamSpec::real("reset_load", 1.0e-12),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        let input_width = ctx.port_width("in");
        let output_width = ctx.port_width("out");
        validate_shape(input_width, output_width)?;

        ctx.allocate_int_states(STATE_DOUT_START + output_width);
        ctx.set_int_state(STATE_PREV_CLK, STATE_ZERO);
        ctx.set_int_state(STATE_PREV_RESET, STATE_ZERO);
        ctx.set_int_state(STATE_RNG, D_PROCESS_RNG_SEED);
        for index in 0..output_width {
            ctx.set_int_state(STATE_DOUT_START + index, STATE_UNKNOWN);
        }

        let process_file = ctx
            .string_param("process_file")
            .ok_or_else(|| CmError::InvalidParameter {
                name: "process_file".to_string(),
                message: "must be set".to_string(),
            })?
            .to_string();
        let process_params = ctx
            .string_vector_param("process_params")
            .map(|values| {
                values
                    .iter()
                    .take(D_PROCESS_MAX_PROCESS_PARAMS)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();
        let spec = DigitalProcessSpec {
            process_file,
            process_params,
            input_count: input_width,
            output_count: output_width,
        };
        let runtime = start_digital_process_runtime(&spec)?;
        let runtime: Arc<DigitalProcessRuntimeResource> = Arc::new(Mutex::new(runtime));
        ctx.set_resource(RESOURCE_RUNTIME, runtime);
        ctx.set_resource(
            RESOURCE_IO_SCRATCH,
            Arc::new(DigitalProcessIoScratch::default()),
        );
        set_unknown_outputs(ctx, output_width)?;
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let input_width = ctx.port_width("in");
        let output_width = ctx.port_width("out");
        validate_shape(input_width, output_width)?;

        if ctx.evaluation_phase() == EvaluationPhase::RollbackableProbe {
            return Ok(());
        }

        if ctx.time == 0.0 {
            for index in 0..output_width {
                ctx.set_int_state(STATE_DOUT_START + index, STATE_UNKNOWN);
            }
            set_unknown_outputs(ctx, output_width)?;
            return Ok(());
        }

        let clk_code = ctx
            .input_digital("clk")
            .map(digital_state_code)
            .unwrap_or(STATE_ZERO);
        let reset_code = ctx
            .input_digital("reset")
            .map(digital_state_code)
            .unwrap_or(STATE_ZERO);
        let prev_clk_code = ctx.int_state(STATE_PREV_CLK);

        if prev_clk_code != STATE_ONE && clk_code == STATE_ONE {
            with_io_scratch(ctx, |ctx, scratch| {
                pack_input_bits(ctx, input_width, &mut scratch.input_bytes)?;
                resize_zeroed(&mut scratch.output_bytes, packed_byte_len(output_width))?;
                let signed_time = if reset_code == STATE_ONE {
                    -ctx.time
                } else {
                    ctx.time
                };

                {
                    let runtime = ctx
                        .resource::<DigitalProcessRuntimeResource>(RESOURCE_RUNTIME)
                        .ok_or_else(|| d_process_error("runtime is not initialized"))?;
                    let mut runtime = runtime
                        .lock()
                        .map_err(|_| d_process_error("runtime lock is poisoned"))?;
                    let DigitalProcessIoScratch {
                        input_bytes,
                        output_bytes,
                    } = scratch;
                    if let Err(err) = runtime.exchange(signed_time, input_bytes, output_bytes) {
                        log::warn!(
                            "d_process external exchange failed at {signed_time:.16e}; applying the returned output buffer like ngspice: {err}"
                        );
                    }
                }

                let clk_delay = d_process_delay(ctx, "clk_delay", 1.0e-9)?;
                for index in 0..output_width {
                    let new_code = output_code(&scratch.output_bytes, index);
                    if new_code != ctx.int_state(STATE_DOUT_START + index) {
                        ctx.set_output_digital_vector_element(
                            "out",
                            index,
                            output_value_from_code(new_code),
                            clk_delay,
                        );
                        ctx.set_int_state(STATE_DOUT_START + index, new_code);
                    }
                }

                Ok(())
            })?;
        }

        ctx.set_int_state(STATE_PREV_CLK, clk_code);
        ctx.set_int_state(STATE_PREV_RESET, reset_code);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::AnalysisType;
    use crate::xspice::context::InputValue;
    use crate::xspice::{ParamType, PortDirection};

    struct StaticProcessRuntime;

    impl DigitalProcessRuntime for StaticProcessRuntime {
        fn exchange(
            &mut self,
            _signed_time: f64,
            _input_bytes: &[u8],
            output_bytes: &mut [u8],
        ) -> CmResult<()> {
            output_bytes.fill(1);
            Ok(())
        }
    }

    struct FailingProcessRuntime;

    impl DigitalProcessRuntime for FailingProcessRuntime {
        fn exchange(
            &mut self,
            _signed_time: f64,
            _input_bytes: &[u8],
            _output_bytes: &mut [u8],
        ) -> CmResult<()> {
            Err(CmError::EvaluationError(
                "synthetic process error".to_string(),
            ))
        }
    }

    #[test]
    fn d_process_metadata_matches_ngspice46_interface() {
        let ports = DigitalProcess.ports();
        assert_eq!(
            ports
                .iter()
                .map(|port| port.name.as_str())
                .collect::<Vec<_>>(),
            vec!["in", "clk", "reset", "out"]
        );
        assert_eq!(ports[0].direction, PortDirection::In);
        assert_eq!(ports[0].default_type, PortType::Digital);
        assert!(ports[0].is_vector);
        assert!(ports[0].null_allowed);
        assert_eq!(ports[0].vector_min_len, None);
        assert_eq!(ports[1].direction, PortDirection::In);
        assert!(!ports[1].is_vector);
        assert!(!ports[1].null_allowed);
        assert_eq!(ports[2].direction, PortDirection::In);
        assert!(!ports[2].is_vector);
        assert!(ports[2].null_allowed);
        assert_eq!(ports[3].direction, PortDirection::Out);
        assert!(ports[3].is_vector);
        assert!(!ports[3].null_allowed);
        assert_eq!(ports[3].vector_min_len, Some(1));

        let params = DigitalProcess.parameters();
        assert_eq!(
            params
                .iter()
                .map(|param| (param.name.as_str(), &param.param_type))
                .collect::<Vec<_>>(),
            vec![
                ("clk_delay", &ParamType::Real),
                ("reset_delay", &ParamType::Real),
                ("process_file", &ParamType::String),
                ("process_params", &ParamType::StringVector),
                ("reset_state", &ParamType::Integer),
                ("input_load", &ParamType::Real),
                ("clk_load", &ParamType::Real),
                ("reset_load", &ParamType::Real),
            ]
        );
        assert_eq!(params[0].default, 1.0e-9);
        assert_eq!(params[1].default, 1.0e-9);
        assert!(params[2].required);
        assert_eq!(params[4].default, 0.0);
        assert_eq!(params[5].default, 1.0e-12);
        assert_eq!(params[6].default, 1.0e-12);
        assert_eq!(params[7].default, 1.0e-12);
    }

    #[test]
    fn d_process_rejects_nonfinite_clk_delay_before_queueing_outputs() {
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.time = 1.0e-9;
        ctx.set_port_width("in", 0);
        ctx.set_port_width("out", 1);
        ctx.allocate_int_states(STATE_DOUT_START + 1);
        ctx.set_int_state(STATE_PREV_CLK, STATE_ZERO);
        ctx.set_int_state(STATE_PREV_RESET, STATE_ZERO);
        ctx.set_int_state(STATE_RNG, D_PROCESS_RNG_SEED);
        ctx.set_int_state(STATE_DOUT_START, STATE_UNKNOWN);
        ctx.set_param("clk_delay", f64::NAN);
        ctx.set_input_digital("clk", DigitalValue::one());
        ctx.set_input_digital("reset", DigitalValue::zero());
        let runtime: Arc<DigitalProcessRuntimeResource> =
            Arc::new(Mutex::new(Box::new(StaticProcessRuntime)));
        ctx.set_resource(RESOURCE_RUNTIME, runtime);
        ctx.set_resource(
            RESOURCE_IO_SCRATCH,
            Arc::new(DigitalProcessIoScratch::default()),
        );

        let err = DigitalProcess
            .evaluate(&mut ctx)
            .expect_err("nonfinite d_process clk_delay must fail evaluation");

        assert!(
            err.to_string().contains("clk_delay"),
            "error should identify clk_delay, got {err:?}"
        );
        assert!(
            ctx.take_pending_events().is_empty(),
            "d_process must not queue outputs after rejecting clk_delay"
        );
        assert_eq!(ctx.int_state(STATE_DOUT_START), STATE_UNKNOWN);
    }

    #[test]
    fn d_process_pack_input_bits_reuses_buffer_and_commits_rng_once() {
        let mut ctx = CmContext::new();
        ctx.allocate_int_states(STATE_DOUT_START);
        ctx.set_int_state(STATE_RNG, D_PROCESS_RNG_SEED);
        ctx.set_input(
            "in",
            InputValue::DigitalVector(vec![
                DigitalValue::one(),
                DigitalValue::zero(),
                DigitalValue::unknown(),
                DigitalValue::one(),
                DigitalValue::unknown(),
                DigitalValue::zero(),
                DigitalValue::unknown(),
                DigitalValue::unknown(),
                DigitalValue::one(),
            ]),
        );

        let mut expected_seed = D_PROCESS_RNG_SEED as u32;
        let unknown_2 = next_unknown_input_bit(&mut expected_seed);
        let unknown_4 = next_unknown_input_bit(&mut expected_seed);
        let unknown_6 = next_unknown_input_bit(&mut expected_seed);
        let unknown_7 = next_unknown_input_bit(&mut expected_seed);
        let mut expected_first_byte = 0b0000_1001;
        if unknown_2 {
            expected_first_byte |= 1 << 2;
        }
        if unknown_4 {
            expected_first_byte |= 1 << 4;
        }
        if unknown_6 {
            expected_first_byte |= 1 << 6;
        }
        if unknown_7 {
            expected_first_byte |= 1 << 7;
        }

        let mut bytes = Vec::new();
        pack_input_bits(&mut ctx, 9, &mut bytes).expect("pack input bits");

        assert_eq!(bytes, vec![expected_first_byte, 0b0000_0001]);
        assert_eq!(ctx.int_state(STATE_RNG) as u32, expected_seed);

        let first_ptr = bytes.as_ptr();
        let first_capacity = bytes.capacity();
        ctx.set_input("in", InputValue::DigitalVector(vec![DigitalValue::zero()]));
        pack_input_bits(&mut ctx, 9, &mut bytes).expect("pack input bits");

        assert_eq!(bytes, vec![0, 0]);
        assert_eq!(bytes.as_ptr(), first_ptr);
        assert_eq!(bytes.capacity(), first_capacity);
    }

    #[test]
    fn d_process_time_zero_rollbackable_probe_does_not_reset_outputs_or_state() {
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.time = 0.0;
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        ctx.set_port_width("in", 0);
        ctx.set_port_width("out", 1);
        ctx.allocate_int_states(STATE_DOUT_START + 1);
        ctx.set_int_state(STATE_DOUT_START, STATE_ONE);
        ctx.set_output_digital_vector("out", vec![DigitalValue::one()], 0.0);
        ctx.take_pending_events();

        DigitalProcess
            .evaluate(&mut ctx)
            .expect("time-zero rollbackable probe evaluates");

        assert_eq!(
            ctx.int_state(STATE_DOUT_START),
            STATE_ONE,
            "time-zero rollbackable probe must not reset committed output state"
        );
        assert!(
            ctx.take_pending_events().is_empty(),
            "time-zero rollbackable probe must not schedule reset output events"
        );
    }

    #[test]
    fn d_process_time_zero_direct_evaluation_streams_unknown_outputs() {
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.time = 0.0;
        ctx.set_port_width("in", 0);
        ctx.set_port_width("out", 2);
        ctx.allocate_int_states(STATE_DOUT_START + 2);
        ctx.set_int_state(STATE_DOUT_START, STATE_ONE);
        ctx.set_int_state(STATE_DOUT_START + 1, STATE_ZERO);

        DigitalProcess
            .evaluate(&mut ctx)
            .expect("time-zero direct evaluation resets outputs");

        assert_eq!(ctx.int_state(STATE_DOUT_START), STATE_UNKNOWN);
        assert_eq!(ctx.int_state(STATE_DOUT_START + 1), STATE_UNKNOWN);
        let events = ctx.take_pending_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].port_name, "out");
        assert_eq!(events[0].delay, 0.0);
        assert_eq!(events[0].values, vec![unknown_high_z(), unknown_high_z()]);
    }

    #[test]
    fn d_process_failed_exchange_applies_zeroed_output_buffer_like_ngspice() {
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.time = 1.0e-9;
        ctx.set_port_width("in", 0);
        ctx.set_port_width("out", 1);
        ctx.allocate_int_states(STATE_DOUT_START + 1);
        ctx.set_int_state(STATE_PREV_CLK, STATE_ZERO);
        ctx.set_int_state(STATE_PREV_RESET, STATE_ZERO);
        ctx.set_int_state(STATE_RNG, D_PROCESS_RNG_SEED);
        ctx.set_int_state(STATE_DOUT_START, STATE_UNKNOWN);
        ctx.set_input_digital("clk", DigitalValue::one());
        ctx.set_input_digital("reset", DigitalValue::zero());
        let runtime: Arc<DigitalProcessRuntimeResource> =
            Arc::new(Mutex::new(Box::new(FailingProcessRuntime)));
        ctx.set_resource(RESOURCE_RUNTIME, runtime);
        ctx.set_resource(
            RESOURCE_IO_SCRATCH,
            Arc::new(DigitalProcessIoScratch::default()),
        );

        DigitalProcess
            .evaluate(&mut ctx)
            .expect("ngspice keeps evaluating d_process output buffer after exchange errors");

        assert_eq!(ctx.int_state(STATE_DOUT_START), STATE_ZERO);
        let events = ctx.take_pending_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].port_name, "out");
        assert_eq!(events[0].values, vec![DigitalValue::zero()]);
        assert_eq!(events[0].delay, 1.0e-9);
    }
}
