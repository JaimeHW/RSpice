use super::*;
use crate::Value;
use crate::xspice::{CmError, EvaluationPhase};

//=============================================================================
// Memory
//=============================================================================

/// RAM model
#[derive(Debug, Default)]
pub struct DigitalRam;

const D_RAM_INITIALIZED: usize = 0;
const D_RAM_PREV_WRITE_EN: usize = 1;
const D_RAM_PREV_SELECT: usize = 2;
const D_RAM_ADDRESS_START: usize = 3;
const D_RAM_SELECT_VALUE_MIN: i64 = 0;
const D_RAM_SELECT_VALUE_MAX: i64 = 32_767;
const D_RAM_IC_MIN: i64 = 0;
const D_RAM_IC_MAX: i64 = 2;
const D_RAM_READ_DELAY_MIN: Value = 1.0e-12;

#[derive(Debug, Clone, Copy)]
struct DMemoryShape {
    address_width: usize,
    word_width: usize,
    select_width: usize,
    memory_start: usize,
    memory_bits: usize,
}

fn d_ram_data_start(address_width: usize) -> usize {
    D_RAM_ADDRESS_START + address_width
}

fn d_ram_memory_start(address_width: usize, word_width: usize) -> usize {
    d_ram_data_start(address_width) + word_width
}

fn d_ram_error(message: impl Into<String>) -> CmError {
    CmError::EvaluationError(format!("d_ram: {}", message.into()))
}

fn d_ram_read_delay(ctx: &CmContext) -> Value {
    ctx.param_or("read_delay", 100.0e-9)
        .max(D_RAM_READ_DELAY_MIN)
}

fn d_ram_integer_param(ctx: &CmContext, name: &str, default: Value, min: i64, max: i64) -> i64 {
    let value = ctx.param_or(name, default).round();
    if value.is_finite() {
        (value as i64).clamp(min, max)
    } else {
        (default.round() as i64).clamp(min, max)
    }
}

fn d_ram_shape(ctx: &CmContext) -> CmResult<DMemoryShape> {
    let address_width = ctx.port_width("address");
    let word_width = ctx.port_width("data_in");
    let output_width = ctx.port_width("data_out");
    let select_width = ctx.port_width("select");

    if address_width == 0 {
        return Err(d_ram_error("address port must have at least one bit"));
    }
    if word_width == 0 {
        return Err(d_ram_error("data_in port must have at least one bit"));
    }
    if output_width != word_width {
        return Err(d_ram_error(format!(
            "data_out width {output_width} does not match data_in width {word_width}"
        )));
    }
    if select_width == 0 {
        return Err(d_ram_error("select port must have at least one bit"));
    }
    if select_width > 16 {
        return Err(d_ram_error(format!(
            "select width {select_width} exceeds ngspice d_ram maximum of 16"
        )));
    }

    if address_width >= usize::BITS as usize {
        return Err(d_ram_error(format!(
            "address width {address_width} is too large"
        )));
    }

    let word_count = 1usize << address_width;
    let memory_bits = word_count
        .checked_mul(word_width)
        .ok_or_else(|| d_ram_error("memory size overflows usize"))?;
    let memory_start = d_ram_memory_start(address_width, word_width);
    memory_start
        .checked_add(memory_bits)
        .ok_or_else(|| d_ram_error("state size overflows usize"))?;

    Ok(DMemoryShape {
        address_width,
        word_width,
        select_width,
        memory_start,
        memory_bits,
    })
}

fn d_ram_state_code(value: DigitalValue) -> i64 {
    match value.state {
        DigitalState::Zero | DigitalState::ZeroR | DigitalState::ZeroZ => 0,
        DigitalState::One | DigitalState::OneR | DigitalState::OneZ => 1,
        _ => 2,
    }
}

fn d_ram_value_from_code(code: i64, strength: DigitalStrength) -> DigitalValue {
    let state = match code {
        0 => DigitalState::Zero,
        1 => DigitalState::One,
        _ => DigitalState::Unknown,
    };
    DigitalValue::new(state, strength)
}

fn d_ram_input_state_code(ctx: &CmContext, name: &str, index: usize) -> i64 {
    ctx.input_digital_vector_values(name)
        .and_then(|values| values.get(index).copied())
        .map(d_ram_state_code)
        .unwrap_or_else(|| d_ram_state_code(DigitalValue::default()))
}

fn d_ram_select_code(ctx: &CmContext, shape: DMemoryShape) -> i64 {
    let select_value = d_ram_integer_param(
        ctx,
        "select_value",
        1.0,
        D_RAM_SELECT_VALUE_MIN,
        D_RAM_SELECT_VALUE_MAX,
    );

    for bit_idx in 0..shape.select_width {
        let expected = (select_value >> bit_idx) & 1;
        if d_ram_input_state_code(ctx, "select", bit_idx) != expected {
            return 0;
        }
    }

    1
}

fn d_ram_address_codes(ctx: &CmContext, shape: DMemoryShape) -> Vec<i64> {
    (0..shape.address_width)
        .map(|index| d_ram_input_state_code(ctx, "address", index))
        .collect()
}

fn d_ram_data_codes(ctx: &CmContext, shape: DMemoryShape) -> Vec<i64> {
    (0..shape.word_width)
        .map(|index| d_ram_input_state_code(ctx, "data_in", index))
        .collect()
}

fn d_ram_address_index(address: &[i64]) -> Option<usize> {
    let mut index = 0usize;
    for (bit_idx, code) in address.iter().copied().enumerate() {
        match code {
            0 => {}
            1 => index |= 1usize << bit_idx,
            _ => return None,
        }
    }
    Some(index)
}

fn d_ram_memory_index(shape: DMemoryShape, address_index: usize, bit: usize) -> usize {
    shape.memory_start + address_index * shape.word_width + bit
}

fn d_ram_state_len(shape: DMemoryShape) -> usize {
    shape.memory_start + shape.memory_bits
}

fn d_ram_state_snapshot(ctx: &CmContext, shape: DMemoryShape) -> Vec<i64> {
    (0..d_ram_state_len(shape))
        .map(|index| ctx.int_state(index))
        .collect()
}

fn d_ram_state(ctx: &CmContext, scratch_state: Option<&[i64]>, index: usize) -> i64 {
    scratch_state
        .and_then(|state| state.get(index).copied())
        .unwrap_or_else(|| ctx.int_state(index))
}

fn d_ram_set_state(
    ctx: &mut CmContext,
    scratch_state: &mut Option<Vec<i64>>,
    index: usize,
    value: i64,
) {
    if let Some(state) = scratch_state.as_mut() {
        if index < state.len() {
            state[index] = value;
        }
    } else {
        ctx.set_int_state(index, value);
    }
}

fn d_ram_read_word(
    ctx: &CmContext,
    scratch_state: Option<&[i64]>,
    shape: DMemoryShape,
    address: &[i64],
) -> Vec<i64> {
    let Some(address_index) = d_ram_address_index(address) else {
        return vec![2; shape.word_width];
    };

    (0..shape.word_width)
        .map(|bit| {
            d_ram_state(
                ctx,
                scratch_state,
                d_ram_memory_index(shape, address_index, bit),
            )
        })
        .collect()
}

fn d_ram_write_word(
    ctx: &mut CmContext,
    scratch_state: &mut Option<Vec<i64>>,
    shape: DMemoryShape,
    address: &[i64],
    data: &[i64],
) {
    let Some(address_index) = d_ram_address_index(address) else {
        for bit in 0..shape.memory_bits {
            d_ram_set_state(ctx, scratch_state, shape.memory_start + bit, 2);
        }
        return;
    };

    for (bit, code) in data.iter().copied().enumerate().take(shape.word_width) {
        d_ram_set_state(
            ctx,
            scratch_state,
            d_ram_memory_index(shape, address_index, bit),
            code,
        );
    }
}

fn d_ram_output_values(codes: &[i64], strength: DigitalStrength) -> Vec<DigitalValue> {
    codes
        .iter()
        .copied()
        .map(|code| d_ram_value_from_code(code, strength))
        .collect()
}

fn d_ram_set_outputs(ctx: &mut CmContext, codes: &[i64], strength: DigitalStrength, delay: Value) {
    ctx.set_output_digital_vector("data_out", d_ram_output_values(codes, strength), delay);
}

fn d_ram_store_previous(
    ctx: &mut CmContext,
    scratch_state: &mut Option<Vec<i64>>,
    shape: DMemoryShape,
    write_en: i64,
    select: i64,
    address: &[i64],
    data: &[i64],
) {
    d_ram_set_state(ctx, scratch_state, D_RAM_PREV_WRITE_EN, write_en);
    d_ram_set_state(ctx, scratch_state, D_RAM_PREV_SELECT, select);
    for (idx, code) in address
        .iter()
        .copied()
        .enumerate()
        .take(shape.address_width)
    {
        d_ram_set_state(ctx, scratch_state, D_RAM_ADDRESS_START + idx, code);
    }
    let data_start = d_ram_data_start(shape.address_width);
    for (idx, code) in data.iter().copied().enumerate().take(shape.word_width) {
        d_ram_set_state(ctx, scratch_state, data_start + idx, code);
    }
}

fn d_ram_previous_address_changed(
    ctx: &CmContext,
    scratch_state: Option<&[i64]>,
    shape: DMemoryShape,
    address: &[i64],
) -> bool {
    address
        .iter()
        .copied()
        .enumerate()
        .take(shape.address_width)
        .any(|(idx, code)| d_ram_state(ctx, scratch_state, D_RAM_ADDRESS_START + idx) != code)
}

fn d_ram_previous_data_changed(
    ctx: &CmContext,
    scratch_state: Option<&[i64]>,
    shape: DMemoryShape,
    data: &[i64],
) -> bool {
    let data_start = d_ram_data_start(shape.address_width);
    data.iter()
        .copied()
        .enumerate()
        .take(shape.word_width)
        .any(|(idx, code)| d_ram_state(ctx, scratch_state, data_start + idx) != code)
}

impl CodeModel for DigitalRam {
    fn name(&self) -> &str {
        "d_ram"
    }
    fn description(&self) -> &str {
        "Random access memory"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::vector_input("data_in", PortType::Digital).with_vector_min_len(1),
                PortSpec::vector_output("data_out", PortType::Digital).with_vector_min_len(1),
                PortSpec::vector_input("address", PortType::Digital).with_vector_min_len(1),
                PortSpec::input("write_en", PortType::Digital),
                PortSpec::vector_input("select", PortType::Digital).with_vector_len_range(1, 16),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::integer("select_value", 1)
                    .with_description("Active select value, clamped to official range"),
                ParamSpec::integer("ic", 2)
                    .with_description("Initial bit state, clamped to official range"),
                ParamSpec::real("read_delay", 100.0e-9)
                    .with_description("Read propagation delay, clamped to official lower limit"),
                ParamSpec::real("data_load", 1.0e-12),
                ParamSpec::real("address_load", 1.0e-12),
                ParamSpec::real("select_load", 1.0e-12),
                ParamSpec::real("enable_load", 1.0e-12),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        let shape = d_ram_shape(ctx)?;
        ctx.allocate_int_states(shape.memory_start + shape.memory_bits);
        ctx.set_int_state(D_RAM_INITIALIZED, 0);
        Ok(())
    }
    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let shape = d_ram_shape(ctx)?;
        let write_en = d_ram_state_code(ctx.input_digital("write_en").unwrap_or_default());
        let select = d_ram_select_code(ctx, shape);
        let address = d_ram_address_codes(ctx, shape);
        let data = d_ram_data_codes(ctx, shape);
        let ic = d_ram_integer_param(ctx, "ic", 2.0, D_RAM_IC_MIN, D_RAM_IC_MAX);
        let read_delay = d_ram_read_delay(ctx);
        let mut scratch_state = (ctx.evaluation_phase() == EvaluationPhase::RollbackableProbe)
            .then(|| d_ram_state_snapshot(ctx, shape));

        if ctx.time == 0.0 || d_ram_state(ctx, scratch_state.as_deref(), D_RAM_INITIALIZED) == 0 {
            for bit in 0..shape.memory_bits {
                d_ram_set_state(ctx, &mut scratch_state, shape.memory_start + bit, ic);
            }

            if select == 1 && write_en == 0 {
                let codes = vec![ic; shape.word_width];
                d_ram_set_outputs(ctx, &codes, DigitalStrength::Strong, 0.0);
            } else {
                let codes = vec![2; shape.word_width];
                d_ram_set_outputs(ctx, &codes, DigitalStrength::HighZ, 0.0);
            }

            d_ram_set_state(ctx, &mut scratch_state, D_RAM_INITIALIZED, 1);
            d_ram_store_previous(
                ctx,
                &mut scratch_state,
                shape,
                write_en,
                select,
                &address,
                &data,
            );
            return Ok(());
        }

        let select_changed =
            select != d_ram_state(ctx, scratch_state.as_deref(), D_RAM_PREV_SELECT);
        let write_changed =
            write_en != d_ram_state(ctx, scratch_state.as_deref(), D_RAM_PREV_WRITE_EN);
        let address_changed =
            d_ram_previous_address_changed(ctx, scratch_state.as_deref(), shape, &address);
        let data_changed = d_ram_previous_data_changed(ctx, scratch_state.as_deref(), shape, &data);

        if select_changed {
            if select == 1 {
                if write_en == 0 {
                    let word = d_ram_read_word(ctx, scratch_state.as_deref(), shape, &address);
                    d_ram_set_outputs(ctx, &word, DigitalStrength::Strong, read_delay);
                } else {
                    d_ram_write_word(ctx, &mut scratch_state, shape, &address, &data);
                    let output = if d_ram_address_index(&address).is_some() {
                        data.clone()
                    } else {
                        vec![2; shape.word_width]
                    };
                    d_ram_set_outputs(ctx, &output, DigitalStrength::HighZ, read_delay);
                }
            } else if write_en == 0 {
                let codes = vec![2; shape.word_width];
                d_ram_set_outputs(ctx, &codes, DigitalStrength::HighZ, read_delay);
            }
        } else if write_changed || address_changed || data_changed {
            if write_en == 1 {
                if select == 1 {
                    d_ram_write_word(ctx, &mut scratch_state, shape, &address, &data);
                    let output = if d_ram_address_index(&address).is_some() {
                        data.clone()
                    } else {
                        vec![2; shape.word_width]
                    };
                    d_ram_set_outputs(ctx, &output, DigitalStrength::HighZ, read_delay);
                }
            } else if select == 1 {
                let word = d_ram_read_word(ctx, scratch_state.as_deref(), shape, &address);
                d_ram_set_outputs(ctx, &word, DigitalStrength::Strong, read_delay);
            }
        }

        d_ram_store_previous(
            ctx,
            &mut scratch_state,
            shape,
            write_en,
            select,
            &address,
            &data,
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::context::InputValue;
    use crate::xspice::{AnalysisType, EvaluationPhase};

    fn ram_context() -> CmContext {
        let mut ctx = CmContext::new();
        ctx.set_port_width("data_in", 1);
        ctx.set_port_width("data_out", 1);
        ctx.set_port_width("address", 1);
        ctx.set_port_width("select", 1);
        ctx.set_param("select_value", 1.0);
        ctx.set_param("ic", 1.0);
        ctx.set_param("read_delay", 2.0e-9);
        ctx.set_input(
            "data_in",
            InputValue::DigitalVector(vec![DigitalValue::zero()]),
        );
        ctx.set_input(
            "address",
            InputValue::DigitalVector(vec![DigitalValue::zero()]),
        );
        ctx.set_input_digital("write_en", DigitalValue::zero());
        ctx.set_input(
            "select",
            InputValue::DigitalVector(vec![DigitalValue::one()]),
        );
        ctx
    }

    fn evaluate_ram_with_phase(ctx: &mut CmContext, time: Value, phase: EvaluationPhase) {
        ctx.time = time;
        ctx.analysis = AnalysisType::Transient;
        ctx.set_evaluation_phase(phase);
        DigitalRam.evaluate(ctx).expect("d_ram evaluates");
    }

    fn evaluate_ram(ctx: &mut CmContext, time: Value) {
        evaluate_ram_with_phase(ctx, time, EvaluationPhase::DirectEvaluation);
    }

    fn take_data_out(ctx: &mut CmContext) -> (DigitalValue, Value) {
        let mut events = ctx.take_pending_events();
        let event = events
            .drain(..)
            .find(|event| event.port_name == "data_out")
            .expect("data_out event is scheduled");
        assert_eq!(event.values.len(), 1);
        (event.values[0], event.delay)
    }

    #[test]
    fn d_ram_write_read_and_unknown_address_follow_ngspice_source_branches() {
        let mut ctx = ram_context();
        DigitalRam.init(&mut ctx).expect("d_ram initializes");

        evaluate_ram(&mut ctx, 0.0);
        let (initial, initial_delay) = take_data_out(&mut ctx);
        assert_eq!(initial, DigitalValue::one());
        assert_eq!(initial_delay, 0.0);

        ctx.set_input_digital("write_en", DigitalValue::one());
        ctx.set_input(
            "data_in",
            InputValue::DigitalVector(vec![DigitalValue::zero()]),
        );
        evaluate_ram(&mut ctx, 1.0e-9);
        let (write_output, write_delay) = take_data_out(&mut ctx);
        assert_eq!(
            write_output,
            DigitalValue::new(DigitalState::Zero, DigitalStrength::HighZ),
            "ngspice drives written data state with HI_IMPEDANCE strength during writes"
        );
        assert_eq!(write_delay, 2.0e-9);

        ctx.set_input_digital("write_en", DigitalValue::zero());
        evaluate_ram(&mut ctx, 2.0e-9);
        let (read_output, read_delay) = take_data_out(&mut ctx);
        assert_eq!(
            read_output,
            DigitalValue::zero(),
            "readback should return the written zero with STRONG strength"
        );
        assert_eq!(read_delay, 2.0e-9);

        ctx.set_input_digital("write_en", DigitalValue::one());
        ctx.set_input(
            "address",
            InputValue::DigitalVector(vec![DigitalValue::unknown()]),
        );
        ctx.set_input(
            "data_in",
            InputValue::DigitalVector(vec![DigitalValue::one()]),
        );
        evaluate_ram(&mut ctx, 3.0e-9);
        let (unknown_write, _) = take_data_out(&mut ctx);
        assert_eq!(
            unknown_write,
            DigitalValue::new(DigitalState::Unknown, DigitalStrength::HighZ),
            "ngspice poisons the entire RAM and outputs UNKNOWN/HI_Z on unknown-address writes"
        );

        ctx.set_input_digital("write_en", DigitalValue::zero());
        ctx.set_input(
            "address",
            InputValue::DigitalVector(vec![DigitalValue::zero()]),
        );
        evaluate_ram(&mut ctx, 4.0e-9);
        let (poisoned_read, _) = take_data_out(&mut ctx);
        assert_eq!(
            poisoned_read,
            DigitalValue::unknown(),
            "reading any valid address after an unknown-address write returns poisoned UNKNOWN"
        );
    }

    #[test]
    fn d_ram_unknown_write_enable_is_branch_dependent_like_ngspice() {
        let mut ctx = ram_context();
        ctx.set_param("ic", 0.0);
        ctx.set_input(
            "select",
            InputValue::DigitalVector(vec![DigitalValue::zero()]),
        );
        ctx.set_input_digital("write_en", DigitalValue::unknown());
        ctx.set_input(
            "data_in",
            InputValue::DigitalVector(vec![DigitalValue::one()]),
        );
        DigitalRam.init(&mut ctx).expect("d_ram initializes");

        evaluate_ram(&mut ctx, 0.0);
        let (initial, _) = take_data_out(&mut ctx);
        assert_eq!(
            initial,
            DigitalValue::new(DigitalState::Unknown, DigitalStrength::HighZ)
        );

        ctx.set_input(
            "select",
            InputValue::DigitalVector(vec![DigitalValue::one()]),
        );
        evaluate_ram(&mut ctx, 1.0e-9);
        let (selected_unknown_write_en, _) = take_data_out(&mut ctx);
        assert_eq!(
            selected_unknown_write_en,
            DigitalValue::new(DigitalState::One, DigitalStrength::HighZ),
            "ngspice treats UNKNOWN write_en as a write when select changes active"
        );

        ctx.set_input(
            "data_in",
            InputValue::DigitalVector(vec![DigitalValue::zero()]),
        );
        evaluate_ram(&mut ctx, 2.0e-9);
        let (already_selected_unknown_write_en, _) = take_data_out(&mut ctx);
        assert_eq!(
            already_selected_unknown_write_en,
            DigitalValue::one(),
            "ngspice treats UNKNOWN write_en as a read on non-select input changes"
        );

        ctx.set_input_digital("write_en", DigitalValue::zero());
        evaluate_ram(&mut ctx, 3.0e-9);
        let (readback, _) = take_data_out(&mut ctx);
        assert_eq!(
            readback,
            DigitalValue::one(),
            "the non-select UNKNOWN write_en branch must not overwrite stored data"
        );
    }

    #[test]
    fn d_ram_rollbackable_probe_does_not_commit_memory_or_previous_inputs() {
        let mut ctx = ram_context();
        DigitalRam.init(&mut ctx).expect("d_ram initializes");

        evaluate_ram(&mut ctx, 0.0);
        let (initial, _) = take_data_out(&mut ctx);
        assert_eq!(initial, DigitalValue::one());

        let shape = d_ram_shape(&ctx).expect("test RAM shape is valid");
        let cell = d_ram_memory_index(shape, 0, 0);
        assert_eq!(ctx.int_state(cell), 1);

        ctx.set_input_digital("write_en", DigitalValue::one());
        ctx.set_input(
            "data_in",
            InputValue::DigitalVector(vec![DigitalValue::zero()]),
        );
        evaluate_ram_with_phase(&mut ctx, 1.0e-9, EvaluationPhase::RollbackableProbe);
        let (trial_write, trial_delay) = take_data_out(&mut ctx);
        assert_eq!(
            trial_write,
            DigitalValue::new(DigitalState::Zero, DigitalStrength::HighZ)
        );
        assert_eq!(trial_delay, 2.0e-9);

        assert_eq!(
            ctx.int_state(cell),
            1,
            "rollbackable write probe must not alter stored RAM contents"
        );
        assert_eq!(
            ctx.int_state(D_RAM_PREV_WRITE_EN),
            0,
            "rollbackable probe must not commit edge-tracking state"
        );

        ctx.set_input_digital("write_en", DigitalValue::zero());
        ctx.set_input(
            "select",
            InputValue::DigitalVector(vec![DigitalValue::zero()]),
        );
        evaluate_ram(&mut ctx, 2.0e-9);
        let _ = take_data_out(&mut ctx);

        ctx.set_input(
            "select",
            InputValue::DigitalVector(vec![DigitalValue::one()]),
        );
        evaluate_ram(&mut ctx, 3.0e-9);
        let (readback, _) = take_data_out(&mut ctx);
        assert_eq!(
            readback,
            DigitalValue::one(),
            "accepted read after rollbackable write probe must see the committed memory"
        );
    }
}
