use super::*;
use crate::Value;
use crate::xspice::CmError;

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

fn d_ram_sized_input_vector(ctx: &CmContext, name: &str, width: usize) -> Vec<DigitalValue> {
    let mut values = ctx.input_digital_vector(name);
    values.resize(width, DigitalValue::default());
    values.truncate(width);
    values
}

fn d_ram_select_code(ctx: &CmContext, shape: DMemoryShape) -> i64 {
    let select = d_ram_sized_input_vector(ctx, "select", shape.select_width);
    let select_value = ctx.param_or("select_value", 1.0) as i64;

    for (bit_idx, value) in select.iter().enumerate() {
        let expected = (select_value >> bit_idx) & 1;
        if d_ram_state_code(*value) != expected {
            return 0;
        }
    }

    1
}

fn d_ram_address_codes(ctx: &CmContext, shape: DMemoryShape) -> Vec<i64> {
    d_ram_sized_input_vector(ctx, "address", shape.address_width)
        .into_iter()
        .map(d_ram_state_code)
        .collect()
}

fn d_ram_data_codes(ctx: &CmContext, shape: DMemoryShape) -> Vec<i64> {
    d_ram_sized_input_vector(ctx, "data_in", shape.word_width)
        .into_iter()
        .map(d_ram_state_code)
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

fn d_ram_read_word(ctx: &CmContext, shape: DMemoryShape, address: &[i64]) -> Vec<i64> {
    let Some(address_index) = d_ram_address_index(address) else {
        return vec![2; shape.word_width];
    };

    (0..shape.word_width)
        .map(|bit| ctx.int_state(d_ram_memory_index(shape, address_index, bit)))
        .collect()
}

fn d_ram_write_word(ctx: &mut CmContext, shape: DMemoryShape, address: &[i64], data: &[i64]) {
    let Some(address_index) = d_ram_address_index(address) else {
        for bit in 0..shape.memory_bits {
            ctx.set_int_state(shape.memory_start + bit, 2);
        }
        return;
    };

    for (bit, code) in data.iter().copied().enumerate().take(shape.word_width) {
        ctx.set_int_state(d_ram_memory_index(shape, address_index, bit), code);
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
    shape: DMemoryShape,
    write_en: i64,
    select: i64,
    address: &[i64],
    data: &[i64],
) {
    ctx.set_int_state(D_RAM_PREV_WRITE_EN, write_en);
    ctx.set_int_state(D_RAM_PREV_SELECT, select);
    for (idx, code) in address
        .iter()
        .copied()
        .enumerate()
        .take(shape.address_width)
    {
        ctx.set_int_state(D_RAM_ADDRESS_START + idx, code);
    }
    let data_start = d_ram_data_start(shape.address_width);
    for (idx, code) in data.iter().copied().enumerate().take(shape.word_width) {
        ctx.set_int_state(data_start + idx, code);
    }
}

fn d_ram_previous_address_changed(ctx: &CmContext, shape: DMemoryShape, address: &[i64]) -> bool {
    address
        .iter()
        .copied()
        .enumerate()
        .take(shape.address_width)
        .any(|(idx, code)| ctx.int_state(D_RAM_ADDRESS_START + idx) != code)
}

fn d_ram_previous_data_changed(ctx: &CmContext, shape: DMemoryShape, data: &[i64]) -> bool {
    let data_start = d_ram_data_start(shape.address_width);
    data.iter()
        .copied()
        .enumerate()
        .take(shape.word_width)
        .any(|(idx, code)| ctx.int_state(data_start + idx) != code)
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
                PortSpec::vector_input("data_in", PortType::Digital),
                PortSpec::vector_output("data_out", PortType::Digital),
                PortSpec::vector_input("address", PortType::Digital),
                PortSpec::input("write_en", PortType::Digital),
                PortSpec::vector_input("select", PortType::Digital),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::integer("select_value", 1).with_range(0.0, 32767.0),
                ParamSpec::integer("ic", 2).with_range(0.0, 2.0),
                ParamSpec::real("read_delay", 100.0e-9).with_min(1.0e-12),
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
        let ic = (ctx.param_or("ic", 2.0) as i64).clamp(0, 2);
        let read_delay = ctx.param_or("read_delay", 100.0e-9);

        if ctx.time == 0.0 || ctx.int_state(D_RAM_INITIALIZED) == 0 {
            for bit in 0..shape.memory_bits {
                ctx.set_int_state(shape.memory_start + bit, ic);
            }

            if select == 1 && write_en == 0 {
                let codes = vec![ic; shape.word_width];
                d_ram_set_outputs(ctx, &codes, DigitalStrength::Strong, 0.0);
            } else {
                let codes = vec![2; shape.word_width];
                d_ram_set_outputs(ctx, &codes, DigitalStrength::HighZ, 0.0);
            }

            ctx.set_int_state(D_RAM_INITIALIZED, 1);
            d_ram_store_previous(ctx, shape, write_en, select, &address, &data);
            return Ok(());
        }

        let select_changed = select != ctx.int_state(D_RAM_PREV_SELECT);
        let write_changed = write_en != ctx.int_state(D_RAM_PREV_WRITE_EN);
        let address_changed = d_ram_previous_address_changed(ctx, shape, &address);
        let data_changed = d_ram_previous_data_changed(ctx, shape, &data);

        if select_changed {
            if select == 1 {
                if write_en == 0 {
                    let word = d_ram_read_word(ctx, shape, &address);
                    d_ram_set_outputs(ctx, &word, DigitalStrength::Strong, read_delay);
                } else {
                    d_ram_write_word(ctx, shape, &address, &data);
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
                    d_ram_write_word(ctx, shape, &address, &data);
                    let output = if d_ram_address_index(&address).is_some() {
                        data.clone()
                    } else {
                        vec![2; shape.word_width]
                    };
                    d_ram_set_outputs(ctx, &output, DigitalStrength::HighZ, read_delay);
                }
            } else if select == 1 {
                let word = d_ram_read_word(ctx, shape, &address);
                d_ram_set_outputs(ctx, &word, DigitalStrength::Strong, read_delay);
            }
        }

        d_ram_store_previous(ctx, shape, write_en, select, &address, &data);
        Ok(())
    }
}
