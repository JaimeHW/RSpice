use super::*;

//=============================================================================
// Basic Gates
//=============================================================================

macro_rules! define_gate {
    ($name:ident, $spice_name:expr, $desc:expr, $op:expr) => {
        #[derive(Debug, Default)]
        pub struct $name;

        impl CodeModel for $name {
            fn name(&self) -> &str {
                $spice_name
            }
            fn description(&self) -> &str {
                $desc
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
                        ParamSpec::real("rise_delay", 1e-9).with_min(0.0),
                        ParamSpec::real("fall_delay", 1e-9).with_min(0.0),
                    ]
                })
            }

            fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
                ctx.allocate_int_states(1);
                Ok(())
            }

            fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
                let rise = ctx.param("rise_delay");
                let fall = ctx.param("fall_delay");
                let inputs = ctx.input_digital_vector("in");
                let prev = ctx.int_state(0);

                let result: DigitalState = $op(&inputs);
                let new_state = if result.is_high() {
                    1
                } else if result.is_low() {
                    0
                } else {
                    -1
                };

                if new_state != prev {
                    let val = DigitalValue::new(result, DigitalStrength::Strong);
                    let delay = if new_state == 1 { rise } else { fall };
                    ctx.set_output_digital("out", val, delay);
                }
                ctx.set_int_state(0, new_state);
                Ok(())
            }
        }
    };
}

fn and_op(inputs: &[DigitalValue]) -> DigitalState {
    inputs
        .iter()
        .fold(DigitalState::One, |a, b| a.and(&b.state))
}

fn or_op(inputs: &[DigitalValue]) -> DigitalState {
    inputs
        .iter()
        .fold(DigitalState::Zero, |a, b| a.or(&b.state))
}

fn xor_op(inputs: &[DigitalValue]) -> DigitalState {
    inputs
        .iter()
        .fold(DigitalState::Zero, |a, b| a.xor(&b.state))
}

define_gate!(DigitalAnd, "d_and", "AND gate", and_op);
define_gate!(DigitalOr, "d_or", "OR gate", or_op);
define_gate!(DigitalXor, "d_xor", "XOR gate", xor_op);

/// NAND gate
#[derive(Debug, Default)]
pub struct DigitalNand;

impl CodeModel for DigitalNand {
    fn name(&self) -> &str {
        "d_nand"
    }
    fn description(&self) -> &str {
        "NAND gate"
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
                ParamSpec::real("rise_delay", 1e-9).with_min(0.0),
                ParamSpec::real("fall_delay", 1e-9).with_min(0.0),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(1);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let rise = ctx.param("rise_delay");
        let fall = ctx.param("fall_delay");
        let inputs = ctx.input_digital_vector("in");
        let prev = ctx.int_state(0);

        let result = and_op(&inputs).invert();
        let new_state = if result.is_high() {
            1
        } else if result.is_low() {
            0
        } else {
            -1
        };

        if new_state != prev {
            let val = DigitalValue::new(result, DigitalStrength::Strong);
            let delay = if new_state == 1 { rise } else { fall };
            ctx.set_output_digital("out", val, delay);
        }
        ctx.set_int_state(0, new_state);
        Ok(())
    }
}

/// NOR gate
#[derive(Debug, Default)]
pub struct DigitalNor;

impl CodeModel for DigitalNor {
    fn name(&self) -> &str {
        "d_nor"
    }
    fn description(&self) -> &str {
        "NOR gate"
    }
    fn ports(&self) -> &[PortSpec] {
        DigitalNand.ports()
    }
    fn parameters(&self) -> &[ParamSpec] {
        DigitalNand.parameters()
    }
    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(1);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let rise = ctx.param("rise_delay");
        let fall = ctx.param("fall_delay");
        let inputs = ctx.input_digital_vector("in");
        let prev = ctx.int_state(0);
        let result = or_op(&inputs).invert();
        let new_state = if result.is_high() {
            1
        } else if result.is_low() {
            0
        } else {
            -1
        };
        if new_state != prev {
            ctx.set_output_digital(
                "out",
                DigitalValue::new(result, DigitalStrength::Strong),
                if new_state == 1 { rise } else { fall },
            );
        }
        ctx.set_int_state(0, new_state);
        Ok(())
    }
}

/// XNOR gate
#[derive(Debug, Default)]
pub struct DigitalXnor;

impl CodeModel for DigitalXnor {
    fn name(&self) -> &str {
        "d_xnor"
    }
    fn description(&self) -> &str {
        "XNOR gate"
    }
    fn ports(&self) -> &[PortSpec] {
        DigitalNand.ports()
    }
    fn parameters(&self) -> &[ParamSpec] {
        DigitalNand.parameters()
    }
    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(1);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let rise = ctx.param("rise_delay");
        let fall = ctx.param("fall_delay");
        let inputs = ctx.input_digital_vector("in");
        let prev = ctx.int_state(0);
        let result = xor_op(&inputs).invert();
        let new_state = if result.is_high() {
            1
        } else if result.is_low() {
            0
        } else {
            -1
        };
        if new_state != prev {
            ctx.set_output_digital(
                "out",
                DigitalValue::new(result, DigitalStrength::Strong),
                if new_state == 1 { rise } else { fall },
            );
        }
        ctx.set_int_state(0, new_state);
        Ok(())
    }
}

/// Inverter
#[derive(Debug, Default)]
pub struct DigitalInverter;

impl CodeModel for DigitalInverter {
    fn name(&self) -> &str {
        "d_inverter"
    }
    fn description(&self) -> &str {
        "Digital inverter"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in", PortType::Digital),
                PortSpec::output("out", PortType::Digital),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        DigitalNand.parameters()
    }
    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(1);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let rise = ctx.param("rise_delay");
        let fall = ctx.param("fall_delay");
        let input = ctx.input_digital("in").unwrap_or_default();
        let prev = ctx.int_state(0);
        let result = input.state.invert();
        let new_state = if result.is_high() {
            1
        } else if result.is_low() {
            0
        } else {
            -1
        };
        if new_state != prev {
            ctx.set_output_digital(
                "out",
                DigitalValue::new(result, DigitalStrength::Strong),
                if new_state == 1 { rise } else { fall },
            );
        }
        ctx.set_int_state(0, new_state);
        Ok(())
    }
}

/// Buffer
#[derive(Debug, Default)]
pub struct DigitalBuffer;

impl CodeModel for DigitalBuffer {
    fn name(&self) -> &str {
        "d_buffer"
    }
    fn description(&self) -> &str {
        "Digital buffer"
    }
    fn ports(&self) -> &[PortSpec] {
        DigitalInverter.ports()
    }
    fn parameters(&self) -> &[ParamSpec] {
        DigitalInverter.parameters()
    }
    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(1);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let rise = ctx.param("rise_delay");
        let fall = ctx.param("fall_delay");
        let input = ctx.input_digital("in").unwrap_or_default();
        let prev = ctx.int_state(0);
        let result = input.state;
        let new_state = if result.is_high() {
            1
        } else if result.is_low() {
            0
        } else {
            -1
        };
        if new_state != prev {
            ctx.set_output_digital(
                "out",
                DigitalValue::new(result, DigitalStrength::Strong),
                if new_state == 1 { rise } else { fall },
            );
        }
        ctx.set_int_state(0, new_state);
        Ok(())
    }
}

/// Tristate buffer
#[derive(Debug, Default)]
pub struct DigitalTristate;

impl CodeModel for DigitalTristate {
    fn name(&self) -> &str {
        "d_tristate"
    }
    fn description(&self) -> &str {
        "Tri-state buffer"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in", PortType::Digital),
                PortSpec::input("enable", PortType::Digital),
                PortSpec::output("out", PortType::Digital),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        DigitalInverter.parameters()
    }
    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let delay = ctx.param("rise_delay");
        let input = ctx.input_digital("in").unwrap_or_default();
        let enable = ctx.input_digital("enable").unwrap_or_default();

        let result = if enable.state.is_high() {
            input
        } else {
            DigitalValue::high_z()
        };
        ctx.set_output_digital("out", result, delay);
        Ok(())
    }
}

/// Pull-up resistor
#[derive(Debug, Default)]
pub struct DigitalPullup;

impl CodeModel for DigitalPullup {
    fn name(&self) -> &str {
        "d_pullup"
    }
    fn description(&self) -> &str {
        "Pull-up resistor"
    }
    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| vec![PortSpec::output("out", PortType::Digital)])
    }
    fn parameters(&self) -> &[ParamSpec] {
        &[]
    }
    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }
    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.set_output_digital(
            "out",
            DigitalValue::new(DigitalState::OneR, DigitalStrength::Resistive),
            0.0,
        );
        Ok(())
    }
}

/// Pull-down resistor
#[derive(Debug, Default)]
pub struct DigitalPulldown;

impl CodeModel for DigitalPulldown {
    fn name(&self) -> &str {
        "d_pulldown"
    }
    fn description(&self) -> &str {
        "Pull-down resistor"
    }
    fn ports(&self) -> &[PortSpec] {
        DigitalPullup.ports()
    }
    fn parameters(&self) -> &[ParamSpec] {
        &[]
    }
    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }
    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.set_output_digital(
            "out",
            DigitalValue::new(DigitalState::ZeroR, DigitalStrength::Resistive),
            0.0,
        );
        Ok(())
    }
}
