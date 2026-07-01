use super::*;
use crate::Value;
use crate::xspice::{CmError, EvaluationPhase};
use std::sync::{Arc, Mutex};

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
const D_RAM_SCRATCH_STATE_RESOURCE: &str = "d_ram.scratch_state";
const D_RAM_SHAPE_RESOURCE: &str = "d_ram.shape";

type DRamScratchStateResource = Mutex<Vec<i64>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DMemoryShapeSignature {
    address_width: usize,
    word_width: usize,
    output_width: usize,
    select_width: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DMemoryShape {
    address_width: usize,
    word_width: usize,
    select_width: usize,
    memory_start: usize,
    memory_bits: usize,
}

#[derive(Debug, Clone)]
struct DMemoryShapeResource {
    signature: DMemoryShapeSignature,
    shape: DMemoryShape,
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

fn d_ram_shape_signature(ctx: &CmContext) -> DMemoryShapeSignature {
    DMemoryShapeSignature {
        address_width: ctx.port_width("address"),
        word_width: ctx.port_width("data_in"),
        output_width: ctx.port_width("data_out"),
        select_width: ctx.port_width("select"),
    }
}

fn d_ram_shape_from_signature(signature: DMemoryShapeSignature) -> CmResult<DMemoryShape> {
    if signature.address_width == 0 {
        return Err(d_ram_error("address port must have at least one bit"));
    }
    if signature.word_width == 0 {
        return Err(d_ram_error("data_in port must have at least one bit"));
    }
    if signature.output_width != signature.word_width {
        return Err(d_ram_error(format!(
            "data_out width {} does not match data_in width {}",
            signature.output_width, signature.word_width
        )));
    }
    if signature.select_width == 0 {
        return Err(d_ram_error("select port must have at least one bit"));
    }
    if signature.select_width > 16 {
        return Err(d_ram_error(format!(
            "select width {} exceeds ngspice d_ram maximum of 16",
            signature.select_width
        )));
    }

    if signature.address_width >= usize::BITS as usize {
        return Err(d_ram_error(format!(
            "address width {} is too large",
            signature.address_width
        )));
    }

    let word_count = 1usize << signature.address_width;
    let memory_bits = word_count
        .checked_mul(signature.word_width)
        .ok_or_else(|| d_ram_error("memory size overflows usize"))?;
    let memory_start = d_ram_memory_start(signature.address_width, signature.word_width);
    memory_start
        .checked_add(memory_bits)
        .ok_or_else(|| d_ram_error("state size overflows usize"))?;

    Ok(DMemoryShape {
        address_width: signature.address_width,
        word_width: signature.word_width,
        select_width: signature.select_width,
        memory_start,
        memory_bits,
    })
}

fn d_ram_shape(ctx: &CmContext) -> CmResult<DMemoryShape> {
    d_ram_shape_from_signature(d_ram_shape_signature(ctx))
}

fn d_ram_cached_shape(ctx: &mut CmContext) -> CmResult<DMemoryShape> {
    let signature = d_ram_shape_signature(ctx);
    if let Some(resource) = ctx.resource::<DMemoryShapeResource>(D_RAM_SHAPE_RESOURCE)
        && resource.signature == signature
    {
        return Ok(resource.shape);
    }

    let shape = d_ram_shape_from_signature(signature)?;
    ctx.set_resource(
        D_RAM_SHAPE_RESOURCE,
        Arc::new(DMemoryShapeResource { signature, shape }),
    );
    Ok(shape)
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

fn d_ram_address_index_from_codes(address: impl Iterator<Item = i64>) -> Option<usize> {
    let mut index = 0usize;
    for (bit_idx, code) in address.enumerate() {
        match code {
            0 => {}
            1 => index |= 1usize << bit_idx,
            _ => return None,
        }
    }
    Some(index)
}

fn d_ram_address_index(ctx: &CmContext, shape: DMemoryShape) -> Option<usize> {
    d_ram_address_index_from_codes(
        (0..shape.address_width).map(|index| d_ram_input_state_code(ctx, "address", index)),
    )
}

fn d_ram_memory_index(shape: DMemoryShape, address_index: usize, bit: usize) -> usize {
    shape.memory_start + address_index * shape.word_width + bit
}

fn d_ram_state_len(shape: DMemoryShape) -> usize {
    shape.memory_start + shape.memory_bits
}

fn d_ram_fill_state_snapshot(ctx: &CmContext, shape: DMemoryShape, state: &mut Vec<i64>) {
    let len = d_ram_state_len(shape);
    state.resize(len, 0);
    for (index, slot) in state.iter_mut().enumerate() {
        *slot = ctx.int_state(index);
    }
}

fn d_ram_state(ctx: &CmContext, scratch_state: Option<&[i64]>, index: usize) -> i64 {
    scratch_state
        .and_then(|state| state.get(index).copied())
        .unwrap_or_else(|| ctx.int_state(index))
}

fn d_ram_set_state(
    ctx: &mut CmContext,
    scratch_state: &mut Option<&mut [i64]>,
    index: usize,
    value: i64,
) {
    if let Some(state) = scratch_state.as_deref_mut() {
        if index < state.len() {
            state[index] = value;
        }
    } else {
        ctx.set_int_state(index, value);
    }
}

fn d_ram_write_word(
    ctx: &mut CmContext,
    scratch_state: &mut Option<&mut [i64]>,
    shape: DMemoryShape,
    address_index: Option<usize>,
) {
    let Some(address_index) = address_index else {
        for bit in 0..shape.memory_bits {
            d_ram_set_state(ctx, scratch_state, shape.memory_start + bit, 2);
        }
        return;
    };

    for bit in 0..shape.word_width {
        let code = d_ram_input_state_code(ctx, "data_in", bit);
        d_ram_set_state(
            ctx,
            scratch_state,
            d_ram_memory_index(shape, address_index, bit),
            code,
        );
    }
}

fn d_ram_set_outputs(
    ctx: &mut CmContext,
    scratch_state: Option<&[i64]>,
    shape: DMemoryShape,
    address_index: Option<usize>,
    strength: DigitalStrength,
    delay: Value,
) {
    ctx.set_output_digital_vector_from_context_fn(
        "data_out",
        shape.word_width,
        delay,
        |source_ctx, bit| {
            let code = address_index
                .map(|address_index| {
                    d_ram_state(
                        source_ctx,
                        scratch_state,
                        d_ram_memory_index(shape, address_index, bit),
                    )
                })
                .unwrap_or(2);
            d_ram_value_from_code(code, strength)
        },
    );
}

fn d_ram_set_uniform_outputs(
    ctx: &mut CmContext,
    shape: DMemoryShape,
    code: i64,
    strength: DigitalStrength,
    delay: Value,
) {
    let value = d_ram_value_from_code(code, strength);
    ctx.set_output_digital_vector_from_context_fn("data_out", shape.word_width, delay, |_, _| {
        value
    });
}

fn d_ram_set_input_data_outputs(
    ctx: &mut CmContext,
    shape: DMemoryShape,
    address_index: Option<usize>,
    strength: DigitalStrength,
    delay: Value,
) {
    ctx.set_output_digital_vector_from_context_fn(
        "data_out",
        shape.word_width,
        delay,
        |source_ctx, bit| {
            let code = if address_index.is_some() {
                d_ram_input_state_code(source_ctx, "data_in", bit)
            } else {
                2
            };
            d_ram_value_from_code(code, strength)
        },
    );
}

fn d_ram_store_previous(
    ctx: &mut CmContext,
    scratch_state: &mut Option<&mut [i64]>,
    shape: DMemoryShape,
    write_en: i64,
    select: i64,
) {
    d_ram_set_state(ctx, scratch_state, D_RAM_PREV_WRITE_EN, write_en);
    d_ram_set_state(ctx, scratch_state, D_RAM_PREV_SELECT, select);
    for idx in 0..shape.address_width {
        let code = d_ram_input_state_code(ctx, "address", idx);
        d_ram_set_state(ctx, scratch_state, D_RAM_ADDRESS_START + idx, code);
    }
    let data_start = d_ram_data_start(shape.address_width);
    for idx in 0..shape.word_width {
        let code = d_ram_input_state_code(ctx, "data_in", idx);
        d_ram_set_state(ctx, scratch_state, data_start + idx, code);
    }
}

fn d_ram_previous_address_changed(
    ctx: &CmContext,
    scratch_state: Option<&[i64]>,
    shape: DMemoryShape,
) -> bool {
    (0..shape.address_width).any(|idx| {
        d_ram_state(ctx, scratch_state, D_RAM_ADDRESS_START + idx)
            != d_ram_input_state_code(ctx, "address", idx)
    })
}

fn d_ram_previous_data_changed(
    ctx: &CmContext,
    scratch_state: Option<&[i64]>,
    shape: DMemoryShape,
) -> bool {
    let data_start = d_ram_data_start(shape.address_width);
    (0..shape.word_width).any(|idx| {
        d_ram_state(ctx, scratch_state, data_start + idx)
            != d_ram_input_state_code(ctx, "data_in", idx)
    })
}

fn d_ram_evaluate_with_state(
    ctx: &mut CmContext,
    shape: DMemoryShape,
    write_en: i64,
    select: i64,
    address_index: Option<usize>,
    ic: i64,
    read_delay: Value,
    mut scratch_state: Option<&mut [i64]>,
) -> CmResult<()> {
    if ctx.time == 0.0 || d_ram_state(ctx, scratch_state.as_deref(), D_RAM_INITIALIZED) == 0 {
        for bit in 0..shape.memory_bits {
            d_ram_set_state(ctx, &mut scratch_state, shape.memory_start + bit, ic);
        }

        if select == 1 && write_en == 0 {
            d_ram_set_uniform_outputs(ctx, shape, ic, DigitalStrength::Strong, 0.0);
        } else {
            d_ram_set_uniform_outputs(ctx, shape, 2, DigitalStrength::HighZ, 0.0);
        }

        d_ram_set_state(ctx, &mut scratch_state, D_RAM_INITIALIZED, 1);
        d_ram_store_previous(ctx, &mut scratch_state, shape, write_en, select);
        return Ok(());
    }

    let select_changed = select != d_ram_state(ctx, scratch_state.as_deref(), D_RAM_PREV_SELECT);
    let write_changed = write_en != d_ram_state(ctx, scratch_state.as_deref(), D_RAM_PREV_WRITE_EN);
    let address_changed = d_ram_previous_address_changed(ctx, scratch_state.as_deref(), shape);
    let data_changed = d_ram_previous_data_changed(ctx, scratch_state.as_deref(), shape);

    if select_changed {
        if select == 1 {
            if write_en == 0 {
                d_ram_set_outputs(
                    ctx,
                    scratch_state.as_deref(),
                    shape,
                    address_index,
                    DigitalStrength::Strong,
                    read_delay,
                );
            } else {
                d_ram_write_word(ctx, &mut scratch_state, shape, address_index);
                d_ram_set_input_data_outputs(
                    ctx,
                    shape,
                    address_index,
                    DigitalStrength::HighZ,
                    read_delay,
                );
            }
        } else if write_en == 0 {
            d_ram_set_uniform_outputs(ctx, shape, 2, DigitalStrength::HighZ, read_delay);
        }
    } else if write_changed || address_changed || data_changed {
        if write_en == 1 {
            if select == 1 {
                d_ram_write_word(ctx, &mut scratch_state, shape, address_index);
                d_ram_set_input_data_outputs(
                    ctx,
                    shape,
                    address_index,
                    DigitalStrength::HighZ,
                    read_delay,
                );
            }
        } else if select == 1 {
            d_ram_set_outputs(
                ctx,
                scratch_state.as_deref(),
                shape,
                address_index,
                DigitalStrength::Strong,
                read_delay,
            );
        }
    }

    d_ram_store_previous(ctx, &mut scratch_state, shape, write_en, select);
    Ok(())
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
        let shape = d_ram_cached_shape(ctx)?;
        ctx.allocate_int_states(shape.memory_start + shape.memory_bits);
        ctx.set_int_state(D_RAM_INITIALIZED, 0);
        ctx.set_resource(
            D_RAM_SCRATCH_STATE_RESOURCE,
            Arc::new(Mutex::new(Vec::<i64>::new())),
        );
        Ok(())
    }
    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let shape = d_ram_cached_shape(ctx)?;
        let write_en = d_ram_state_code(ctx.input_digital("write_en").unwrap_or_default());
        let select = d_ram_select_code(ctx, shape);
        let address_index = d_ram_address_index(ctx, shape);
        let ic = d_ram_integer_param(ctx, "ic", 2.0, D_RAM_IC_MIN, D_RAM_IC_MAX);
        let read_delay = d_ram_read_delay(ctx);

        if ctx.evaluation_phase() == EvaluationPhase::RollbackableProbe {
            let scratch = ctx
                .resource::<DRamScratchStateResource>(D_RAM_SCRATCH_STATE_RESOURCE)
                .ok_or_else(|| d_ram_error("scratch state is not initialized"))?;
            let mut scratch = scratch
                .lock()
                .map_err(|_| d_ram_error("scratch state lock is poisoned"))?;
            d_ram_fill_state_snapshot(ctx, shape, &mut scratch);
            return d_ram_evaluate_with_state(
                ctx,
                shape,
                write_en,
                select,
                address_index,
                ic,
                read_delay,
                Some(scratch.as_mut_slice()),
            );
        }

        d_ram_evaluate_with_state(
            ctx,
            shape,
            write_en,
            select,
            address_index,
            ic,
            read_delay,
            None,
        )
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

    fn take_data_out_vector(ctx: &mut CmContext) -> (Vec<DigitalValue>, Value) {
        let mut events = ctx.take_pending_events();
        let event = events
            .drain(..)
            .find(|event| event.port_name == "data_out")
            .expect("data_out event is scheduled");
        (event.values, event.delay)
    }

    #[test]
    fn d_ram_shape_cache_reuses_and_refreshes_for_port_width_changes() {
        let mut ctx = ram_context();

        let first_shape = d_ram_cached_shape(&mut ctx).expect("d_ram shape caches");
        let first_resource = ctx
            .resource::<DMemoryShapeResource>(D_RAM_SHAPE_RESOURCE)
            .expect("shape resource is installed");
        let second_shape = d_ram_cached_shape(&mut ctx).expect("d_ram shape reuses cache");
        let second_resource = ctx
            .resource::<DMemoryShapeResource>(D_RAM_SHAPE_RESOURCE)
            .expect("shape resource remains installed");

        assert_eq!(first_shape, second_shape);
        assert!(
            std::sync::Arc::ptr_eq(&first_resource, &second_resource),
            "unchanged d_ram port widths should reuse the cached memory shape"
        );

        ctx.set_port_width("address", 2);
        let updated_shape = d_ram_cached_shape(&mut ctx).expect("d_ram shape refreshes");
        let updated_resource = ctx
            .resource::<DMemoryShapeResource>(D_RAM_SHAPE_RESOURCE)
            .expect("updated shape resource is installed");

        assert_eq!(updated_shape.address_width, 2);
        assert_eq!(updated_shape.memory_bits, 4);
        assert!(
            !std::sync::Arc::ptr_eq(&first_resource, &updated_resource),
            "changed d_ram port widths must refresh the cached memory shape"
        );
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
    fn d_ram_multi_bit_word_uses_streamed_inputs_for_write_and_readback() {
        let mut ctx = CmContext::new();
        ctx.set_port_width("data_in", 2);
        ctx.set_port_width("data_out", 2);
        ctx.set_port_width("address", 2);
        ctx.set_port_width("select", 1);
        ctx.set_param("select_value", 1.0);
        ctx.set_param("ic", 0.0);
        ctx.set_param("read_delay", 3.0e-9);
        ctx.set_input(
            "address",
            InputValue::DigitalVector(vec![DigitalValue::one(), DigitalValue::zero()]),
        );
        ctx.set_input(
            "data_in",
            InputValue::DigitalVector(vec![DigitalValue::zero(), DigitalValue::zero()]),
        );
        ctx.set_input_digital("write_en", DigitalValue::zero());
        ctx.set_input(
            "select",
            InputValue::DigitalVector(vec![DigitalValue::one()]),
        );

        DigitalRam.init(&mut ctx).expect("d_ram initializes");
        evaluate_ram(&mut ctx, 0.0);
        let _ = take_data_out_vector(&mut ctx);

        ctx.set_input_digital("write_en", DigitalValue::one());
        ctx.set_input(
            "data_in",
            InputValue::DigitalVector(vec![DigitalValue::one(), DigitalValue::zero()]),
        );
        evaluate_ram(&mut ctx, 1.0e-9);
        let (write_output, write_delay) = take_data_out_vector(&mut ctx);
        assert_eq!(
            write_output,
            vec![
                DigitalValue::new(DigitalState::One, DigitalStrength::HighZ),
                DigitalValue::new(DigitalState::Zero, DigitalStrength::HighZ)
            ]
        );
        assert_eq!(write_delay, 3.0e-9);

        ctx.set_input_digital("write_en", DigitalValue::zero());
        evaluate_ram(&mut ctx, 2.0e-9);
        let (readback, read_delay) = take_data_out_vector(&mut ctx);
        assert_eq!(readback, vec![DigitalValue::one(), DigitalValue::zero()]);
        assert_eq!(read_delay, 3.0e-9);
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

    #[test]
    fn d_ram_rollbackable_probe_reuses_snapshot_buffer() {
        let mut ctx = ram_context();
        DigitalRam.init(&mut ctx).expect("d_ram initializes");

        evaluate_ram(&mut ctx, 0.0);
        let _ = take_data_out(&mut ctx);

        let shape = d_ram_shape(&ctx).expect("test RAM shape is valid");
        let cell = d_ram_memory_index(shape, 0, 0);
        assert_eq!(ctx.int_state(cell), 1);

        ctx.set_input_digital("write_en", DigitalValue::one());
        ctx.set_input(
            "data_in",
            InputValue::DigitalVector(vec![DigitalValue::zero()]),
        );
        evaluate_ram_with_phase(&mut ctx, 1.0e-9, EvaluationPhase::RollbackableProbe);
        let _ = take_data_out(&mut ctx);

        let scratch = ctx
            .resource::<DRamScratchStateResource>(D_RAM_SCRATCH_STATE_RESOURCE)
            .expect("d_ram scratch state is installed");
        let guard = scratch.lock().expect("d_ram scratch state locks");
        assert_eq!(guard.len(), d_ram_state_len(shape));
        let first_ptr = guard.as_ptr();
        let first_capacity = guard.capacity();
        drop(guard);

        evaluate_ram_with_phase(&mut ctx, 2.0e-9, EvaluationPhase::RollbackableProbe);
        let guard = scratch.lock().expect("d_ram scratch state locks again");
        assert_eq!(guard.as_ptr(), first_ptr);
        assert_eq!(guard.capacity(), first_capacity);
        assert_eq!(
            ctx.int_state(cell),
            1,
            "rollbackable probes must continue to leave committed RAM untouched"
        );
    }
}
