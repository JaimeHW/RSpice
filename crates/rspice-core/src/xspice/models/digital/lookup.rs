use super::*;
use crate::Value;
use crate::xspice::{CmError, EvaluationPhase};

const D_LUT_INITIAL_STATE: i64 = i64::MIN;
const D_LOOKUP_DELAY_MIN: Value = 1.0e-12;

#[derive(Debug, Default)]
pub struct DigitalLookupTable;

#[derive(Debug, Default)]
pub struct DigitalGenericLookupTable;

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

fn d_lut_table_state(table: &str, index: usize) -> DigitalState {
    match table.as_bytes().get(index).copied() {
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

fn d_genlut_lookup_value(table: &str, index: usize) -> DigitalValue {
    match table.as_bytes().get(index).copied() {
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

fn d_genlut_delay_param_value(ctx: &CmContext, name: &str, index: usize, default: Value) -> Value {
    d_lookup_delay(d_genlut_param_value(ctx, name, index, default))
}

fn d_genlut_output_delay(
    ctx: &CmContext,
    output_index: usize,
    new_state: i64,
    previous_state: i64,
    input_delay: Value,
) -> Value {
    let default = 1.0e-9;
    let edge_delay = match new_state {
        0 => d_genlut_delay_param_value(ctx, "fall_delay", output_index, default),
        1 => d_genlut_delay_param_value(ctx, "rise_delay", output_index, default),
        _ if previous_state == 0 => {
            d_genlut_delay_param_value(ctx, "rise_delay", output_index, default)
        }
        _ => d_genlut_delay_param_value(ctx, "fall_delay", output_index, default),
    };

    if ctx.time == 0.0 || previous_state == D_LUT_INITIAL_STATE {
        0.0
    } else {
        input_delay + edge_delay
    }
}

fn d_genlut_strength_delay(
    ctx: &CmContext,
    output_index: usize,
    new_strength: i64,
    previous_state: i64,
) -> Value {
    let default = 1.0e-9;
    if ctx.time == 0.0 || previous_state == D_LUT_INITIAL_STATE {
        return 0.0;
    }

    if new_strength == 3 {
        if previous_state == 0 {
            d_genlut_delay_param_value(ctx, "fall_delay", output_index, default)
        } else {
            d_genlut_delay_param_value(ctx, "rise_delay", output_index, default)
        }
    } else if previous_state == 0 {
        d_genlut_delay_param_value(ctx, "rise_delay", output_index, default)
    } else {
        d_genlut_delay_param_value(ctx, "fall_delay", output_index, default)
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
        let table = ctx
            .string_param("table_values")
            .ok_or_else(|| CmError::MissingParameter("table_values".to_string()))?;
        let input_width = ctx.port_width("in");
        let inputs = ctx.input_digital_vector_values("in").unwrap_or(&[]);
        let output_state = match d_lut_index_for_width(inputs, input_width)? {
            Some(index) => d_lut_table_state(table, index),
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
        if ctx.string_param("table_values").is_none() {
            return Err(CmError::MissingParameter("table_values".to_string()));
        }
        let input_start = d_genlut_previous_input_start(input_width, output_width);
        let state_start = d_genlut_previous_state_start(input_width, output_width);
        let strength_start = d_genlut_previous_strength_start(input_width, output_width);

        let mut max_input_delay = 0.0;
        let mut input_index = Some(0usize);
        for bit in 0..input_width {
            let input = {
                let inputs = ctx.input_digital_vector_values("in").unwrap_or(&[]);
                inputs.get(bit).copied().unwrap_or_default()
            };
            let input_code = d_lut_state_code(input.state);
            if input_code != ctx.int_state(input_start + bit) {
                max_input_delay = f64::max(
                    max_input_delay,
                    d_genlut_param_value(ctx, "input_delay", bit, 0.0),
                );
            }
            match (input_index, input.state.logic_level()) {
                (Some(index), Some(true)) => {
                    input_index = Some(index | (1usize << bit));
                }
                (Some(index), Some(false)) => {
                    input_index = Some(index);
                }
                _ => input_index = None,
            }
            d_lookup_set_int_state(ctx, input_start + bit, input_code);
        }

        let entry_len = 1usize << input_width;
        for output_index in 0..output_width {
            let value = {
                let table = ctx.string_param("table_values").unwrap_or("");
                match input_index {
                    Some(index) => d_genlut_lookup_value(table, index + output_index * entry_len),
                    None => d_genlut_unknown_value(),
                }
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
                        output_index,
                        state_code,
                        previous_state,
                        max_input_delay,
                    ),
                );
            } else if strength_code != previous_strength {
                ctx.set_output_digital_vector_element(
                    "out",
                    output_index,
                    value,
                    d_genlut_strength_delay(ctx, output_index, strength_code, previous_state),
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

    #[test]
    fn d_lut_short_tables_default_missing_entries_to_unknown() {
        assert_eq!(d_lut_table_state("01", 0), DigitalState::Zero);
        assert_eq!(d_lut_table_state("01", 1), DigitalState::One);
        assert_eq!(d_lut_table_state("01", 2), DigitalState::Unknown);
    }

    #[test]
    fn d_lut_non_binary_table_characters_are_unknown() {
        assert_eq!(d_lut_table_state("0x1", 1), DigitalState::Unknown);
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
            d_genlut_lookup_value("z", 0),
            DigitalValue::new(DigitalState::Unknown, DigitalStrength::HighZ)
        );
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
