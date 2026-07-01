use super::*;
use crate::Value;
use crate::xspice::{CmError, EvaluationPhase};
use std::sync::Arc;

const D_LUT_INITIAL_STATE: i64 = i64::MIN;
const D_LOOKUP_DELAY_MIN: Value = 1.0e-12;
const D_LUT_TABLE_RESOURCE: &str = "xspice.d_lut.table_values";
const D_GENLUT_TABLE_RESOURCE: &str = "xspice.d_genlut.table_values";
const D_GENLUT_DELAY_PLAN_RESOURCE: &str = "xspice.d_genlut.delay_plan";

#[derive(Debug, Default)]
pub struct DigitalLookupTable;

#[derive(Debug, Default)]
pub struct DigitalGenericLookupTable;

#[derive(Debug, Clone)]
struct DLookupTableBytes {
    table_revision: u64,
    bytes: Arc<[u8]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DGenlutDelaySignature {
    input_width: usize,
    output_width: usize,
    rise_revision: u64,
    fall_revision: u64,
    input_revision: u64,
}

#[derive(Debug, Clone)]
struct DGenlutDelayPlan {
    signature: DGenlutDelaySignature,
    rise_delays: Arc<[Value]>,
    fall_delays: Arc<[Value]>,
    input_delays: Arc<[Value]>,
}

fn d_lut_error(message: impl Into<String>) -> CmError {
    CmError::EvaluationError(format!("d_lut: {}", message.into()))
}

fn d_genlut_error(message: impl Into<String>) -> CmError {
    CmError::EvaluationError(format!("d_genlut: {}", message.into()))
}

fn d_lut_state_code(state: DigitalState) -> i64 {
    match state.logic_level() {
        Some(false) => 0,
        Some(true) => 1,
        None => -1,
    }
}

fn d_lut_state_from_code(code: i64) -> DigitalState {
    match code {
        0 => DigitalState::Zero,
        1 => DigitalState::One,
        _ => DigitalState::Unknown,
    }
}

fn d_lookup_table_bytes(ctx: &mut CmContext, resource_key: &str) -> CmResult<Arc<[u8]>> {
    let table = ctx
        .string_param("table_values")
        .ok_or_else(|| CmError::MissingParameter("table_values".to_string()))?;
    let table_revision = ctx.string_param_revision("table_values").unwrap_or(0);
    if let Some(resource) = ctx.resource::<DLookupTableBytes>(resource_key)
        && resource.table_revision == table_revision
    {
        return Ok(Arc::clone(&resource.bytes));
    }

    let bytes: Arc<[u8]> = Arc::from(table.as_bytes());
    ctx.set_resource(
        resource_key,
        Arc::new(DLookupTableBytes {
            table_revision,
            bytes: Arc::clone(&bytes),
        }),
    );
    Ok(bytes)
}

fn d_genlut_delay_signature(
    ctx: &CmContext,
    input_width: usize,
    output_width: usize,
) -> DGenlutDelaySignature {
    DGenlutDelaySignature {
        input_width,
        output_width,
        rise_revision: ctx.real_vector_param_revision("rise_delay").unwrap_or(0),
        fall_revision: ctx.real_vector_param_revision("fall_delay").unwrap_or(0),
        input_revision: ctx.real_vector_param_revision("input_delay").unwrap_or(0),
    }
}

fn d_genlut_expand_param_values(
    ctx: &CmContext,
    name: &str,
    width: usize,
    default: Value,
    clamp_delay: bool,
) -> Arc<[Value]> {
    let values = ctx.real_vector_param(name).unwrap_or(&[]);
    let fallback = values.last().copied().unwrap_or(default);
    (0..width)
        .map(|index| {
            let value = values.get(index).copied().unwrap_or(fallback);
            if clamp_delay {
                d_lookup_delay(value)
            } else {
                value
            }
        })
        .collect::<Vec<_>>()
        .into()
}

fn d_genlut_delay_plan(
    ctx: &mut CmContext,
    input_width: usize,
    output_width: usize,
) -> Arc<DGenlutDelayPlan> {
    let signature = d_genlut_delay_signature(ctx, input_width, output_width);
    if let Some(resource) = ctx.resource::<DGenlutDelayPlan>(D_GENLUT_DELAY_PLAN_RESOURCE)
        && resource.signature == signature
    {
        return resource;
    }

    let plan = Arc::new(DGenlutDelayPlan {
        signature,
        rise_delays: d_genlut_expand_param_values(ctx, "rise_delay", output_width, 1.0e-9, true),
        fall_delays: d_genlut_expand_param_values(ctx, "fall_delay", output_width, 1.0e-9, true),
        input_delays: d_genlut_expand_param_values(ctx, "input_delay", input_width, 0.0, false),
    });
    ctx.set_resource(D_GENLUT_DELAY_PLAN_RESOURCE, Arc::clone(&plan));
    plan
}

fn d_lut_table_state(table: &[u8], index: usize) -> DigitalState {
    match table.get(index).copied() {
        Some(b'0') => DigitalState::Zero,
        Some(b'1') => DigitalState::One,
        _ => DigitalState::Unknown,
    }
}

fn d_genlut_value_code(value: DigitalValue) -> (i64, i64) {
    let state = d_lut_state_code(value.state);
    let strength = match value.strength {
        DigitalStrength::Undetermined => 0,
        DigitalStrength::HighZ => 1,
        DigitalStrength::Resistive => 2,
        DigitalStrength::Strong => 3,
    };
    (state, strength)
}

fn d_genlut_strength_from_code(code: i64) -> DigitalStrength {
    match code {
        1 => DigitalStrength::HighZ,
        2 => DigitalStrength::Resistive,
        3 => DigitalStrength::Strong,
        _ => DigitalStrength::Undetermined,
    }
}

fn d_genlut_lookup_value(table: &[u8], index: usize) -> DigitalValue {
    match table.get(index).copied() {
        Some(b'0') => DigitalValue::new(DigitalState::Zero, DigitalStrength::Strong),
        Some(b'1') => DigitalValue::new(DigitalState::One, DigitalStrength::Strong),
        Some(b'z') => DigitalValue::new(DigitalState::Unknown, DigitalStrength::HighZ),
        _ => DigitalValue::new(DigitalState::Unknown, DigitalStrength::Undetermined),
    }
}

fn d_genlut_unknown_value() -> DigitalValue {
    DigitalValue::new(DigitalState::Unknown, DigitalStrength::Undetermined)
}

fn d_lut_index_for_width(inputs: &[DigitalValue], input_width: usize) -> CmResult<Option<usize>> {
    if input_width >= usize::BITS as usize {
        return Err(d_lut_error(format!(
            "input vector width {} is too large",
            input_width
        )));
    }

    let mut index = 0usize;
    for bit in 0..input_width {
        let input = inputs.get(bit).copied().unwrap_or_default();
        match input.state.logic_level() {
            Some(false) => {}
            Some(true) => {
                let mask = 1usize.checked_shl(bit as u32).ok_or_else(|| {
                    d_lut_error(format!("input vector width {input_width} is too large"))
                })?;
                index |= mask;
            }
            None => return Ok(None),
        }
    }

    Ok(Some(index))
}

fn d_lut_index(inputs: &[DigitalValue]) -> CmResult<Option<usize>> {
    d_lut_index_for_width(inputs, inputs.len())
}

fn d_genlut_shape(ctx: &CmContext) -> CmResult<(usize, usize)> {
    let input_width = ctx.port_width("in");
    let output_width = ctx.port_width("out");
    if input_width >= usize::BITS as usize {
        return Err(d_genlut_error(format!(
            "input vector width {input_width} is too large"
        )));
    }
    Ok((input_width, output_width))
}

fn d_genlut_param_value(ctx: &CmContext, name: &str, index: usize, default: Value) -> Value {
    match ctx.real_vector_param(name) {
        Some(values) if index < values.len() => values[index],
        Some(values) if !values.is_empty() => *values.last().unwrap(),
        _ => default,
    }
}

fn d_lookup_delay(value: Value) -> Value {
    value.max(D_LOOKUP_DELAY_MIN)
}

fn d_genlut_output_delay(
    ctx: &CmContext,
    delay_plan: &DGenlutDelayPlan,
    output_index: usize,
    new_state: i64,
    previous_state: i64,
    input_delay: Value,
) -> Value {
    let edge_delay = match new_state {
        0 => delay_plan.fall_delays[output_index],
        1 => delay_plan.rise_delays[output_index],
        _ if previous_state == 0 => delay_plan.rise_delays[output_index],
        _ => delay_plan.fall_delays[output_index],
    };

    if ctx.time == 0.0 || previous_state == D_LUT_INITIAL_STATE {
        0.0
    } else {
        input_delay + edge_delay
    }
}

fn d_genlut_strength_delay(
    ctx: &CmContext,
    delay_plan: &DGenlutDelayPlan,
    output_index: usize,
    new_strength: i64,
    previous_state: i64,
) -> Value {
    if ctx.time == 0.0 || previous_state == D_LUT_INITIAL_STATE {
        return 0.0;
    }

    if new_strength == 3 {
        if previous_state == 0 {
            delay_plan.fall_delays[output_index]
        } else {
            delay_plan.rise_delays[output_index]
        }
    } else if previous_state == 0 {
        delay_plan.rise_delays[output_index]
    } else {
        delay_plan.fall_delays[output_index]
    }
}

fn d_genlut_previous_input_start(_input_width: usize, _output_width: usize) -> usize {
    0
}

fn d_genlut_previous_state_start(input_width: usize, _output_width: usize) -> usize {
    input_width
}

fn d_genlut_previous_strength_start(input_width: usize, output_width: usize) -> usize {
    input_width + output_width
}

fn d_genlut_total_state_count(input_width: usize, output_width: usize) -> usize {
    input_width + 2 * output_width
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct DGenlutInputScan {
    index: Option<usize>,
    max_delay: Value,
    one_bits: usize,
    unknown_bits: usize,
}

fn d_genlut_scan_inputs(
    ctx: &CmContext,
    inputs: &[DigitalValue],
    input_width: usize,
    input_start: usize,
) -> DGenlutInputScan {
    let input_delays = d_genlut_expand_param_values(ctx, "input_delay", input_width, 0.0, false);
    d_genlut_scan_inputs_with_delays(ctx, inputs, input_width, input_start, &input_delays)
}

fn d_genlut_scan_inputs_with_delays(
    ctx: &CmContext,
    inputs: &[DigitalValue],
    input_width: usize,
    input_start: usize,
    input_delays: &[Value],
) -> DGenlutInputScan {
    let mut max_delay = 0.0;
    let mut index = Some(0usize);
    let mut one_bits = 0usize;
    let mut unknown_bits = 0usize;

    for bit in 0..input_width {
        let input = inputs.get(bit).copied().unwrap_or_default();
        let input_code = d_lut_state_code(input.state);
        if input_code != ctx.int_state(input_start + bit) {
            max_delay = f64::max(max_delay, input_delays.get(bit).copied().unwrap_or(0.0));
        }
        let bit_mask = 1usize << bit;
        match input.state.logic_level() {
            Some(true) => {
                if let Some(value) = index {
                    index = Some(value | bit_mask);
                }
                one_bits |= bit_mask;
            }
            Some(false) => {}
            None => {
                index = None;
                unknown_bits |= bit_mask;
            }
        }
    }

    DGenlutInputScan {
        index,
        max_delay,
        one_bits,
        unknown_bits,
    }
}

fn d_genlut_commit_input_scan(
    ctx: &mut CmContext,
    input_width: usize,
    input_start: usize,
    scan: DGenlutInputScan,
) {
    for bit in 0..input_width {
        let bit_mask = 1usize << bit;
        let code = if scan.unknown_bits & bit_mask != 0 {
            -1
        } else if scan.one_bits & bit_mask != 0 {
            1
        } else {
            0
        };
        d_lookup_set_int_state(ctx, input_start + bit, code);
    }
}

fn d_lut_delay(
    ctx: &CmContext,
    new_code: i64,
    previous_code: i64,
    rise_delay: Value,
    fall_delay: Value,
) -> Value {
    if ctx.time == 0.0 || previous_code == D_LUT_INITIAL_STATE {
        return 0.0;
    }

    match new_code {
        0 => fall_delay,
        1 => rise_delay,
        _ if previous_code == 0 => rise_delay,
        _ => fall_delay,
    }
}

fn d_lookup_set_int_state(ctx: &mut CmContext, index: usize, value: i64) {
    if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
        ctx.set_int_state(index, value);
    }
}

impl CodeModel for DigitalLookupTable {
    fn name(&self) -> &str {
        "d_lut"
    }

    fn description(&self) -> &str {
        "Digital n-input lookup table gate"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::vector_input("in", PortType::Digital),
                PortSpec::output("out", PortType::Digital),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("rise_delay", 1.0e-9)
                    .with_description("Rise propagation delay, clamped to official lower limit"),
                ParamSpec::real("fall_delay", 1.0e-9)
                    .with_description("Fall propagation delay, clamped to official lower limit"),
                ParamSpec::real("input_load", 1.0e-12),
                ParamSpec::string("table_values", "").required(),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(1);
        ctx.set_int_state(0, D_LUT_INITIAL_STATE);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let table = d_lookup_table_bytes(ctx, D_LUT_TABLE_RESOURCE)?;
        let input_width = ctx.port_width("in");
        let inputs = ctx.input_digital_vector_values("in").unwrap_or(&[]);
        let output_state = match d_lut_index_for_width(inputs, input_width)? {
            Some(index) => d_lut_table_state(table.as_ref(), index),
            None => DigitalState::Unknown,
        };
        let output_code = d_lut_state_code(output_state);
        let previous_code = ctx.int_state(0);

        if output_code != previous_code {
            let delay = d_lut_delay(
                ctx,
                output_code,
                previous_code,
                d_lookup_delay(ctx.param("rise_delay")),
                d_lookup_delay(ctx.param("fall_delay")),
            );
            ctx.set_output_digital(
                "out",
                DigitalValue::new(output_state, DigitalStrength::Strong),
                delay,
            );
        }

        d_lookup_set_int_state(ctx, 0, output_code);
        Ok(())
    }
}

impl CodeModel for DigitalGenericLookupTable {
    fn name(&self) -> &str {
        "d_genlut"
    }

    fn description(&self) -> &str {
        "Digital n-input by m-output lookup table gate"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::vector_input("in", PortType::Digital),
                PortSpec::vector_output("out", PortType::Digital),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real_vector("rise_delay", Vec::new()).with_description(
                    "Rise delays, defaulting each output to official 1.0e-9 and clamped to official lower limit",
                ),
                ParamSpec::real_vector("fall_delay", Vec::new()).with_description(
                    "Fall delays, defaulting each output to official 1.0e-9 and clamped to official lower limit",
                ),
                ParamSpec::real_vector("input_load", Vec::new())
                    .with_description("Input loads, defaulting each input to official 1.0e-12"),
                ParamSpec::real_vector("input_delay", Vec::new())
                    .with_description("Input delays, defaulting each input to official 0.0"),
                ParamSpec::string("table_values", "").required(),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        let (input_width, output_width) = d_genlut_shape(ctx)?;
        ctx.allocate_int_states(d_genlut_total_state_count(input_width, output_width));
        let input_start = d_genlut_previous_input_start(input_width, output_width);
        let state_start = d_genlut_previous_state_start(input_width, output_width);
        let strength_start = d_genlut_previous_strength_start(input_width, output_width);

        for bit in 0..input_width {
            ctx.set_int_state(input_start + bit, D_LUT_INITIAL_STATE);
        }
        for bit in 0..output_width {
            ctx.set_int_state(state_start + bit, D_LUT_INITIAL_STATE);
            ctx.set_int_state(strength_start + bit, D_LUT_INITIAL_STATE);
        }
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let (input_width, output_width) = d_genlut_shape(ctx)?;
        let table = d_lookup_table_bytes(ctx, D_GENLUT_TABLE_RESOURCE)?;
        let delay_plan = d_genlut_delay_plan(ctx, input_width, output_width);
        let input_start = d_genlut_previous_input_start(input_width, output_width);
        let state_start = d_genlut_previous_state_start(input_width, output_width);
        let strength_start = d_genlut_previous_strength_start(input_width, output_width);

        let scan = {
            let inputs = ctx.input_digital_vector_values("in").unwrap_or(&[]);
            d_genlut_scan_inputs_with_delays(
                ctx,
                inputs,
                input_width,
                input_start,
                &delay_plan.input_delays,
            )
        };
        d_genlut_commit_input_scan(ctx, input_width, input_start, scan);

        let entry_len = 1usize << input_width;
        for output_index in 0..output_width {
            let value = match scan.index {
                Some(index) => {
                    d_genlut_lookup_value(table.as_ref(), index + output_index * entry_len)
                }
                None => d_genlut_unknown_value(),
            };
            let (state_code, strength_code) = d_genlut_value_code(value);
            let previous_state = ctx.int_state(state_start + output_index);
            let previous_strength = ctx.int_state(strength_start + output_index);

            if state_code != previous_state {
                ctx.set_output_digital_vector_element(
                    "out",
                    output_index,
                    value,
                    d_genlut_output_delay(
                        ctx,
                        &delay_plan,
                        output_index,
                        state_code,
                        previous_state,
                        scan.max_delay,
                    ),
                );
            } else if strength_code != previous_strength {
                ctx.set_output_digital_vector_element(
                    "out",
                    output_index,
                    value,
                    d_genlut_strength_delay(
                        ctx,
                        &delay_plan,
                        output_index,
                        strength_code,
                        previous_state,
                    ),
                );
            }

            d_lookup_set_int_state(ctx, state_start + output_index, state_code);
            d_lookup_set_int_state(ctx, strength_start + output_index, strength_code);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::EvaluationPhase;
    use crate::xspice::context::InputValue;
    use crate::xspice::{ParamType, PortDirection};

    fn param_summary(model: &dyn CodeModel) -> Vec<(&str, ParamType, Value, Option<&str>, bool)> {
        model
            .parameters()
            .iter()
            .map(|param| {
                (
                    param.name.as_str(),
                    param.param_type,
                    param.default,
                    param.string_default.as_deref(),
                    param.required,
                )
            })
            .collect()
    }

    fn assert_digital_ports(
        model: &dyn CodeModel,
        expected: &[(&str, PortDirection, bool, Option<usize>, Option<usize>)],
    ) {
        let ports = model.ports();
        assert_eq!(
            ports
                .iter()
                .map(|port| port.name.as_str())
                .collect::<Vec<_>>(),
            expected
                .iter()
                .map(|(name, _, _, _, _)| *name)
                .collect::<Vec<_>>()
        );
        for (port, (_, direction, is_vector, min_len, max_len)) in ports.iter().zip(expected) {
            assert_eq!(port.direction, *direction, "{} direction", port.name);
            assert_eq!(
                port.default_type,
                PortType::Digital,
                "{} default type",
                port.name
            );
            assert_eq!(
                port.allowed_types,
                vec![PortType::Digital],
                "{} allowed types",
                port.name
            );
            assert_eq!(port.is_vector, *is_vector, "{} vector flag", port.name);
            assert!(!port.null_allowed, "{} nullability", port.name);
            assert_eq!(port.vector_min_len, *min_len, "{} min length", port.name);
            assert_eq!(port.vector_max_len, *max_len, "{} max length", port.name);
        }
    }

    #[test]
    fn d_lut_metadata_matches_ngspice46_interface() {
        assert_digital_ports(
            &DigitalLookupTable,
            &[
                ("in", PortDirection::In, true, None, None),
                ("out", PortDirection::Out, false, None, None),
            ],
        );
        assert_eq!(
            param_summary(&DigitalLookupTable),
            vec![
                ("rise_delay", ParamType::Real, 1.0e-9, None, false),
                ("fall_delay", ParamType::Real, 1.0e-9, None, false),
                ("input_load", ParamType::Real, 1.0e-12, None, false),
                ("table_values", ParamType::String, 0.0, Some(""), true),
            ]
        );
    }

    #[test]
    fn d_genlut_metadata_matches_ngspice46_interface() {
        assert_digital_ports(
            &DigitalGenericLookupTable,
            &[
                ("in", PortDirection::In, true, None, None),
                ("out", PortDirection::Out, true, None, None),
            ],
        );
        assert_eq!(
            param_summary(&DigitalGenericLookupTable),
            vec![
                ("rise_delay", ParamType::RealVector, 0.0, None, false),
                ("fall_delay", ParamType::RealVector, 0.0, None, false),
                ("input_load", ParamType::RealVector, 0.0, None, false),
                ("input_delay", ParamType::RealVector, 0.0, None, false),
                ("table_values", ParamType::String, 0.0, Some(""), true),
            ]
        );
    }

    #[test]
    fn d_lut_short_tables_default_missing_entries_to_unknown() {
        assert_eq!(d_lut_table_state(b"01", 0), DigitalState::Zero);
        assert_eq!(d_lut_table_state(b"01", 1), DigitalState::One);
        assert_eq!(d_lut_table_state(b"01", 2), DigitalState::Unknown);
    }

    #[test]
    fn d_lut_non_binary_table_characters_are_unknown() {
        assert_eq!(d_lut_table_state(b"0x1", 1), DigitalState::Unknown);
    }

    #[test]
    fn d_lut_uses_first_input_as_low_order_index_bit() {
        let inputs = [
            DigitalValue::one(),
            DigitalValue::zero(),
            DigitalValue::one(),
        ];

        assert_eq!(d_lut_index(&inputs).unwrap(), Some(5));
    }

    #[test]
    fn d_lut_unknown_input_produces_unknown_index() {
        let inputs = [DigitalValue::one(), DigitalValue::unknown()];

        assert_eq!(d_lut_index(&inputs).unwrap(), None);
    }

    #[test]
    fn d_lut_index_uses_declared_width_for_bounds() {
        assert!(d_lut_index_for_width(&[], usize::BITS as usize).is_err());
    }

    #[test]
    fn d_lut_index_uses_declared_width_with_default_event_node_values() {
        let inputs = [DigitalValue::one()];

        assert_eq!(d_lut_index_for_width(&inputs, 3).unwrap(), Some(1));
    }

    #[test]
    fn d_lut_zero_input_width_is_constant_table_like_ngspice() {
        let mut ctx = CmContext::new();
        ctx.set_port_width("in", 0);
        ctx.set_string_param("table_values", "1");

        DigitalLookupTable
            .init(&mut ctx)
            .expect("d_lut zero-input init");
        DigitalLookupTable
            .evaluate(&mut ctx)
            .expect("d_lut zero-input evaluate");

        assert_eq!(ctx.output_digital_vector("out"), vec![DigitalValue::one()]);
    }

    #[test]
    fn d_lut_rejects_unaddressable_input_widths_even_when_all_bits_are_zero() {
        let inputs = vec![DigitalValue::zero(); usize::BITS as usize];

        assert!(d_lut_index(&inputs).is_err());
    }

    #[test]
    fn d_lut_rejects_unaddressable_declared_width_even_with_short_input_buffer() {
        let mut ctx = CmContext::new();
        ctx.set_port_width("in", usize::BITS as usize);
        ctx.set_string_param("table_values", "0");

        DigitalLookupTable
            .init(&mut ctx)
            .expect("d_lut init does not inspect table width");
        let err = DigitalLookupTable
            .evaluate(&mut ctx)
            .expect_err("declared d_lut width must drive addressability checks");

        assert!(
            format!("{err}").contains("input vector width"),
            "unexpected d_lut error: {err}"
        );
    }

    #[test]
    fn d_lut_code_round_trip_preserves_logic_states() {
        assert_eq!(d_lut_state_from_code(0), DigitalState::Zero);
        assert_eq!(d_lut_state_from_code(1), DigitalState::One);
        assert_eq!(d_lut_state_from_code(-1), DigitalState::Unknown);
    }

    #[test]
    fn d_genlut_z_table_entries_are_high_impedance_unknowns() {
        assert_eq!(
            d_genlut_lookup_value(b"z", 0),
            DigitalValue::new(DigitalState::Unknown, DigitalStrength::HighZ)
        );
    }

    #[test]
    fn d_lookup_table_byte_cache_reloads_when_table_param_changes() {
        let mut ctx = CmContext::new();
        ctx.set_string_param("table_values", "01");

        let first =
            d_lookup_table_bytes(&mut ctx, D_LUT_TABLE_RESOURCE).expect("table bytes cache");
        let reused =
            d_lookup_table_bytes(&mut ctx, D_LUT_TABLE_RESOURCE).expect("table bytes reuse");
        assert!(Arc::ptr_eq(&first, &reused));
        assert_eq!(first.as_ref(), b"01");

        ctx.set_string_param("other", "ignored");
        let reused_after_unrelated_string_change =
            d_lookup_table_bytes(&mut ctx, D_LUT_TABLE_RESOURCE)
                .expect("unrelated string params do not invalidate table bytes");
        assert!(Arc::ptr_eq(&first, &reused_after_unrelated_string_change));

        ctx.set_string_param("table_values", "10");
        let updated =
            d_lookup_table_bytes(&mut ctx, D_LUT_TABLE_RESOURCE).expect("table bytes reload");
        assert!(!Arc::ptr_eq(&first, &updated));
        assert_eq!(updated.as_ref(), b"10");
    }

    #[test]
    fn d_genlut_vector_params_extend_the_last_supplied_value() {
        let mut ctx = CmContext::new();
        ctx.set_real_vector_param("rise_delay", vec![1.0e-9, 2.0e-9]);

        assert_eq!(d_genlut_param_value(&ctx, "rise_delay", 0, 9.0), 1.0e-9);
        assert_eq!(d_genlut_param_value(&ctx, "rise_delay", 1, 9.0), 2.0e-9);
        assert_eq!(d_genlut_param_value(&ctx, "rise_delay", 2, 9.0), 2.0e-9);
    }

    #[test]
    fn d_genlut_input_scan_packs_known_bits_unknowns_and_changed_delay() {
        let mut ctx = CmContext::new();
        ctx.allocate_int_states(4);
        ctx.set_int_state(0, 0);
        ctx.set_int_state(1, 0);
        ctx.set_int_state(2, 0);
        ctx.set_int_state(3, 1);
        ctx.set_real_vector_param("input_delay", vec![1.0e-9, 2.0e-9, 3.0e-9, 4.0e-9]);

        let inputs = [
            DigitalValue::one(),
            DigitalValue::unknown(),
            DigitalValue::zero(),
        ];
        let scan = d_genlut_scan_inputs(&ctx, &inputs, 4, 0);

        assert_eq!(scan.index, None);
        assert_eq!(scan.one_bits, 0b0001);
        assert_eq!(scan.unknown_bits, 0b0010);
        assert_eq!(scan.max_delay, 4.0e-9);

        d_genlut_commit_input_scan(&mut ctx, 4, 0, scan);
        assert_eq!(ctx.int_state(0), 1);
        assert_eq!(ctx.int_state(1), -1);
        assert_eq!(ctx.int_state(2), 0);
        assert_eq!(ctx.int_state(3), 0);
    }

    #[test]
    fn d_genlut_zero_input_width_is_constant_table_like_ngspice() {
        let mut ctx = CmContext::new();
        ctx.set_port_width("in", 0);
        ctx.set_port_width("out", 1);
        ctx.set_string_param("table_values", "1");

        DigitalGenericLookupTable
            .init(&mut ctx)
            .expect("ngspice d_genlut accepts zero input bits");
        DigitalGenericLookupTable
            .evaluate(&mut ctx)
            .expect("zero-input d_genlut evaluates as a constant lookup");

        assert_eq!(ctx.output_digital_vector("out"), vec![DigitalValue::one()]);
    }

    #[test]
    fn d_genlut_strength_codes_round_trip() {
        assert_eq!(
            d_genlut_strength_from_code(0),
            DigitalStrength::Undetermined
        );
        assert_eq!(d_genlut_strength_from_code(1), DigitalStrength::HighZ);
        assert_eq!(d_genlut_strength_from_code(2), DigitalStrength::Resistive);
        assert_eq!(d_genlut_strength_from_code(3), DigitalStrength::Strong);
    }

    #[test]
    fn d_genlut_multi_output_lookup_and_unknown_input_update_all_state() {
        let mut ctx = CmContext::new();
        ctx.set_port_width("in", 2);
        ctx.set_port_width("out", 3);
        ctx.set_string_param("table_values", "0101z0z1x1zz");
        ctx.set_input(
            "in",
            InputValue::DigitalVector(vec![DigitalValue::one(), DigitalValue::zero()]),
        );

        DigitalGenericLookupTable
            .init(&mut ctx)
            .expect("d_genlut init");
        DigitalGenericLookupTable
            .evaluate(&mut ctx)
            .expect("d_genlut indexed evaluate");

        assert_eq!(
            ctx.output_digital_vector("out"),
            vec![
                DigitalValue::one(),
                DigitalValue::zero(),
                DigitalValue::one()
            ]
        );

        ctx.set_input(
            "in",
            InputValue::DigitalVector(vec![DigitalValue::unknown(), DigitalValue::zero()]),
        );
        DigitalGenericLookupTable
            .evaluate(&mut ctx)
            .expect("d_genlut unknown-input evaluate");

        let unknown = DigitalValue::new(DigitalState::Unknown, DigitalStrength::Undetermined);
        assert_eq!(ctx.output_digital_vector("out"), vec![unknown; 3]);
        assert_eq!(ctx.int_state(d_genlut_previous_input_start(2, 3)), -1);
        assert_eq!(ctx.int_state(d_genlut_previous_state_start(2, 3)), -1);
        assert_eq!(ctx.int_state(d_genlut_previous_state_start(2, 3) + 2), -1);
    }

    #[test]
    fn d_genlut_uses_max_changed_input_delay_after_input_scan() {
        let mut ctx = CmContext::new();
        ctx.set_port_width("in", 3);
        ctx.set_port_width("out", 1);
        ctx.set_string_param("table_values", "00000010");
        ctx.set_real_vector_param("input_delay", vec![1.0e-9, 2.0e-9, 3.0e-9]);
        ctx.set_real_vector_param("rise_delay", vec![1.0e-9]);
        ctx.set_input(
            "in",
            InputValue::DigitalVector(vec![
                DigitalValue::zero(),
                DigitalValue::zero(),
                DigitalValue::zero(),
            ]),
        );

        DigitalGenericLookupTable
            .init(&mut ctx)
            .expect("d_genlut init");
        DigitalGenericLookupTable
            .evaluate(&mut ctx)
            .expect("d_genlut initial evaluate");
        let _ = ctx.take_pending_events();

        ctx.time = 1.0e-9;
        ctx.set_input(
            "in",
            InputValue::DigitalVector(vec![
                DigitalValue::zero(),
                DigitalValue::one(),
                DigitalValue::one(),
            ]),
        );
        DigitalGenericLookupTable
            .evaluate(&mut ctx)
            .expect("d_genlut changed-input evaluate");

        let events = ctx.take_pending_events();
        assert!(
            events
                .iter()
                .any(|event| event.delay == 4.0e-9 && event.values == vec![DigitalValue::one()]),
            "d_genlut should add the largest changed input delay to the output delay, got {events:?}"
        );
        let input_start = d_genlut_previous_input_start(3, 1);
        assert_eq!(ctx.int_state(input_start), 0);
        assert_eq!(ctx.int_state(input_start + 1), 1);
        assert_eq!(ctx.int_state(input_start + 2), 1);
    }

    #[test]
    fn d_lut_rollbackable_probe_does_not_commit_previous_output_state() {
        let mut ctx = CmContext::new();
        ctx.set_port_width("in", 1);
        ctx.set_string_param("table_values", "01");
        ctx.set_param("rise_delay", 1.0e-9);
        ctx.set_param("fall_delay", 1.0e-9);
        ctx.set_input("in", InputValue::DigitalVector(vec![DigitalValue::zero()]));
        DigitalLookupTable.init(&mut ctx).expect("d_lut init");

        DigitalLookupTable
            .evaluate(&mut ctx)
            .expect("d_lut initial evaluate");
        let _ = ctx.take_pending_events();
        assert_eq!(ctx.int_state(0), 0);

        ctx.time = 1.0e-9;
        ctx.set_input("in", InputValue::DigitalVector(vec![DigitalValue::one()]));
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        DigitalLookupTable
            .evaluate(&mut ctx)
            .expect("d_lut rollback probe evaluates");
        let events = ctx.take_pending_events();
        assert!(
            events
                .iter()
                .any(|event| event.delay == 1.0e-9 && event.values == vec![DigitalValue::one()]),
            "rollbackable lookup probe should expose the trial output event, got {events:?}"
        );
        assert_eq!(
            ctx.int_state(0),
            0,
            "rollbackable lookup probe must not commit previous output state"
        );

        ctx.set_evaluation_phase(EvaluationPhase::DirectEvaluation);
        DigitalLookupTable
            .evaluate(&mut ctx)
            .expect("d_lut direct evaluate after probe");
        assert_eq!(ctx.int_state(0), 1);
    }

    #[test]
    fn d_genlut_rollbackable_probe_does_not_commit_previous_io_state() {
        let mut ctx = CmContext::new();
        ctx.set_port_width("in", 1);
        ctx.set_port_width("out", 1);
        ctx.set_string_param("table_values", "01");
        ctx.set_input("in", InputValue::DigitalVector(vec![DigitalValue::zero()]));
        DigitalGenericLookupTable
            .init(&mut ctx)
            .expect("d_genlut init");

        DigitalGenericLookupTable
            .evaluate(&mut ctx)
            .expect("d_genlut initial evaluate");
        let _ = ctx.take_pending_events();
        let input_start = d_genlut_previous_input_start(1, 1);
        let state_start = d_genlut_previous_state_start(1, 1);
        assert_eq!(ctx.int_state(input_start), 0);
        assert_eq!(ctx.int_state(state_start), 0);

        ctx.time = 1.0e-9;
        ctx.set_input("in", InputValue::DigitalVector(vec![DigitalValue::one()]));
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        DigitalGenericLookupTable
            .evaluate(&mut ctx)
            .expect("d_genlut rollback probe evaluates");
        let events = ctx.take_pending_events();
        assert!(
            events
                .iter()
                .any(|event| event.delay == 1.0e-9 && event.values == vec![DigitalValue::one()]),
            "rollbackable genlut probe should expose the trial output event, got {events:?}"
        );
        assert_eq!(
            ctx.int_state(input_start),
            0,
            "rollbackable genlut probe must not commit previous input state"
        );
        assert_eq!(
            ctx.int_state(state_start),
            0,
            "rollbackable genlut probe must not commit previous output state"
        );

        ctx.set_evaluation_phase(EvaluationPhase::DirectEvaluation);
        DigitalGenericLookupTable
            .evaluate(&mut ctx)
            .expect("d_genlut direct evaluate after probe");
        assert_eq!(ctx.int_state(input_start), 1);
        assert_eq!(ctx.int_state(state_start), 1);
    }
}
