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

const RESOURCE_RUNTIME: &str = "d_process.runtime";
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

fn set_unknown_outputs(ctx: &mut CmContext, output_width: usize) {
    ctx.set_output_digital_vector_from_context_fn("out", output_width, 0.0, |_, _| {
        unknown_high_z()
    });
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

fn next_unknown_input_bit(ctx: &mut CmContext) -> bool {
    let mut seed = ctx.int_state(STATE_RNG) as u32;
    if seed == 0 {
        seed = D_PROCESS_RNG_SEED as u32;
    }
    seed ^= seed << 13;
    seed ^= seed >> 17;
    seed ^= seed << 5;
    ctx.set_int_state(STATE_RNG, seed as i64);
    seed & 1 != 0
}

fn pack_input_bits(ctx: &mut CmContext, input_width: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; packed_byte_len(input_width)];

    for bit_index in 0..input_width {
        let value = ctx
            .input_digital_vector_values("in")
            .and_then(|inputs| inputs.get(bit_index).copied())
            .unwrap_or_default();
        let bit = match digital_state_code(value) {
            STATE_ZERO => false,
            STATE_ONE => true,
            _ => next_unknown_input_bit(ctx),
        };
        if bit {
            bytes[bit_index >> 3] |= 1u8 << (bit_index & 7);
        }
    }

    bytes
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
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| CmError::InvalidParameter {
                name: "process_file".to_string(),
                message: "must not be empty".to_string(),
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
        set_unknown_outputs(ctx, output_width);
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
            set_unknown_outputs(ctx, output_width);
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
            let input_bytes = pack_input_bits(ctx, input_width);
            let mut output_bytes = vec![0u8; packed_byte_len(output_width)];
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
                if let Err(err) = runtime.exchange(signed_time, &input_bytes, &mut output_bytes) {
                    log::warn!(
                        "d_process external exchange failed at {signed_time:.16e}; continuing with current output packet like ngspice: {err}"
                    );
                }
            }

            let clk_delay = ctx.param_or("clk_delay", 1.0e-9);
            for index in 0..output_width {
                let new_code = output_code(&output_bytes, index);
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
}
