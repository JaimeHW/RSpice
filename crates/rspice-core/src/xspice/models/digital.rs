//! Digital Code Models - Gates, Flip-Flops, Memory
//!
//! Provides digital logic primitives for mixed-signal simulation.

use crate::xspice::{
    CmContext, CmResult, CodeModel, DigitalState, DigitalStrength, DigitalValue, ParamSpec,
    PortSpec, PortType,
};

//=============================================================================
// Digital Source
//=============================================================================

/// Digital stimulus source from file
#[derive(Debug, Default)]
pub struct DigitalSource;

impl CodeModel for DigitalSource {
    fn name(&self) -> &str {
        "d_source"
    }
    fn description(&self) -> &str {
        "Digital stimulus from file"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| vec![PortSpec::vector_output("out", PortType::Digital)])
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| vec![ParamSpec::string("input_file", "").required()])
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }
    fn evaluate(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }
}

/// Digital state machine
#[derive(Debug, Default)]
pub struct DigitalStateMachine;

impl CodeModel for DigitalStateMachine {
    fn name(&self) -> &str {
        "d_state"
    }
    fn description(&self) -> &str {
        "Digital state machine"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::vector_input("in", PortType::Digital),
                PortSpec::input("clk", PortType::Digital),
                PortSpec::input("reset", PortType::Digital),
                PortSpec::vector_output("out", PortType::Digital),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| vec![ParamSpec::string("state_file", "").required()])
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }
    fn evaluate(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }
}

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

//=============================================================================
// Flip-Flops and Latches
//=============================================================================

/// D Flip-Flop
#[derive(Debug, Default)]
pub struct DFlipFlop;

impl CodeModel for DFlipFlop {
    fn name(&self) -> &str {
        "d_dff"
    }
    fn description(&self) -> &str {
        "D flip-flop"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("data", PortType::Digital),
                PortSpec::input("clk", PortType::Digital),
                PortSpec::input("set", PortType::Digital).nullable(),
                PortSpec::input("reset", PortType::Digital).nullable(),
                PortSpec::output("q", PortType::Digital),
                PortSpec::output("qn", PortType::Digital),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("clk_delay", 1e-9).with_min(0.0),
                ParamSpec::real("set_delay", 1e-9).with_min(0.0),
                ParamSpec::real("reset_delay", 1e-9).with_min(0.0),
                ParamSpec::boolean("ic", false),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(2); // [q_state, prev_clk]
        let ic = ctx.param("ic") > 0.5;
        ctx.set_int_state(0, if ic { 1 } else { 0 });
        ctx.set_int_state(1, 0);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let clk_delay = ctx.param("clk_delay");
        let set_delay = ctx.param("set_delay");
        let reset_delay = ctx.param("reset_delay");

        let data = ctx.input_digital("data").unwrap_or_default();
        let clk = ctx.input_digital("clk").unwrap_or_default();
        let set = ctx.input_digital("set");
        let reset = ctx.input_digital("reset");

        let prev_clk = ctx.int_state(1);
        let mut q = ctx.int_state(0);
        let clk_state = if clk.state.is_high() { 1 } else { 0 };

        // Async set/reset
        if let Some(s) = set
            && s.state.is_high()
        {
            q = 1;
        }
        if let Some(r) = reset
            && r.state.is_high()
        {
            q = 0;
        }

        // Rising edge clock
        if clk_state == 1 && prev_clk == 0 {
            q = if data.state.is_high() { 1 } else { 0 };
        }

        let delay = if set.is_some_and(|s| s.state.is_high()) {
            set_delay
        } else if reset.is_some_and(|r| r.state.is_high()) {
            reset_delay
        } else {
            clk_delay
        };

        ctx.set_int_state(0, q);
        ctx.set_int_state(1, clk_state);

        let q_val = if q == 1 {
            DigitalValue::one()
        } else {
            DigitalValue::zero()
        };
        let qn_val = q_val.invert();
        ctx.set_output_digital("q", q_val, delay);
        ctx.set_output_digital("qn", qn_val, delay);
        Ok(())
    }
}

/// JK Flip-Flop
#[derive(Debug, Default)]
pub struct JkFlipFlop;

impl CodeModel for JkFlipFlop {
    fn name(&self) -> &str {
        "d_jkff"
    }
    fn description(&self) -> &str {
        "JK flip-flop"
    }
    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("j", PortType::Digital),
                PortSpec::input("k", PortType::Digital),
                PortSpec::input("clk", PortType::Digital),
                PortSpec::input("set", PortType::Digital).nullable(),
                PortSpec::input("reset", PortType::Digital).nullable(),
                PortSpec::output("q", PortType::Digital),
                PortSpec::output("qn", PortType::Digital),
            ]
        })
    }
    fn parameters(&self) -> &[ParamSpec] {
        DFlipFlop.parameters()
    }
    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        DFlipFlop.init(ctx)
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let delay = ctx.param("clk_delay");
        let j = ctx.input_digital("j").unwrap_or_default();
        let k = ctx.input_digital("k").unwrap_or_default();
        let clk = ctx.input_digital("clk").unwrap_or_default();

        let prev_clk = ctx.int_state(1);
        let mut q = ctx.int_state(0);
        let clk_state = if clk.state.is_high() { 1 } else { 0 };

        if clk_state == 1 && prev_clk == 0 {
            let jh = j.state.is_high();
            let kh = k.state.is_high();
            q = match (jh, kh) {
                (false, false) => q,
                (true, false) => 1,
                (false, true) => 0,
                (true, true) => 1 - q, // Toggle
            };
        }

        ctx.set_int_state(0, q);
        ctx.set_int_state(1, clk_state);

        let q_val = if q == 1 {
            DigitalValue::one()
        } else {
            DigitalValue::zero()
        };
        ctx.set_output_digital("q", q_val, delay);
        ctx.set_output_digital("qn", q_val.invert(), delay);
        Ok(())
    }
}

/// T Flip-Flop
#[derive(Debug, Default)]
pub struct TFlipFlop;

impl CodeModel for TFlipFlop {
    fn name(&self) -> &str {
        "d_tff"
    }
    fn description(&self) -> &str {
        "T flip-flop"
    }
    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("t", PortType::Digital),
                PortSpec::input("clk", PortType::Digital),
                PortSpec::output("q", PortType::Digital),
                PortSpec::output("qn", PortType::Digital),
            ]
        })
    }
    fn parameters(&self) -> &[ParamSpec] {
        DFlipFlop.parameters()
    }
    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(2);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let delay = ctx.param("clk_delay");
        let t = ctx.input_digital("t").unwrap_or_default();
        let clk = ctx.input_digital("clk").unwrap_or_default();

        let prev_clk = ctx.int_state(1);
        let mut q = ctx.int_state(0);
        let clk_state = if clk.state.is_high() { 1 } else { 0 };

        if clk_state == 1 && prev_clk == 0 && t.state.is_high() {
            q = 1 - q; // Toggle
        }

        ctx.set_int_state(0, q);
        ctx.set_int_state(1, clk_state);

        let q_val = if q == 1 {
            DigitalValue::one()
        } else {
            DigitalValue::zero()
        };
        ctx.set_output_digital("q", q_val, delay);
        ctx.set_output_digital("qn", q_val.invert(), delay);
        Ok(())
    }
}

/// SR Flip-Flop
#[derive(Debug, Default)]
pub struct SrFlipFlop;

impl CodeModel for SrFlipFlop {
    fn name(&self) -> &str {
        "d_srff"
    }
    fn description(&self) -> &str {
        "SR flip-flop"
    }
    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("s", PortType::Digital),
                PortSpec::input("r", PortType::Digital),
                PortSpec::input("clk", PortType::Digital),
                PortSpec::output("q", PortType::Digital),
                PortSpec::output("qn", PortType::Digital),
            ]
        })
    }
    fn parameters(&self) -> &[ParamSpec] {
        DFlipFlop.parameters()
    }
    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(2);
        Ok(())
    }
    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let delay = ctx.param("clk_delay");
        let s = ctx.input_digital("s").unwrap_or_default();
        let r = ctx.input_digital("r").unwrap_or_default();
        let clk = ctx.input_digital("clk").unwrap_or_default();

        let prev_clk = ctx.int_state(1);
        let mut q = ctx.int_state(0);
        let clk_state = if clk.state.is_high() { 1 } else { 0 };

        if clk_state == 1 && prev_clk == 0 {
            if s.state.is_high() && !r.state.is_high() {
                q = 1;
            } else if r.state.is_high() && !s.state.is_high() {
                q = 0;
            }
        }

        ctx.set_int_state(0, q);
        ctx.set_int_state(1, clk_state);
        let q_val = if q == 1 {
            DigitalValue::one()
        } else {
            DigitalValue::zero()
        };
        ctx.set_output_digital("q", q_val, delay);
        ctx.set_output_digital("qn", q_val.invert(), delay);
        Ok(())
    }
}

/// D Latch
#[derive(Debug, Default)]
pub struct DLatch;

impl CodeModel for DLatch {
    fn name(&self) -> &str {
        "d_dlatch"
    }
    fn description(&self) -> &str {
        "D latch (level-sensitive)"
    }
    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("data", PortType::Digital),
                PortSpec::input("enable", PortType::Digital),
                PortSpec::output("q", PortType::Digital),
                PortSpec::output("qn", PortType::Digital),
            ]
        })
    }
    fn parameters(&self) -> &[ParamSpec] {
        DFlipFlop.parameters()
    }
    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(1);
        Ok(())
    }
    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let delay = ctx.param("clk_delay");
        let data = ctx.input_digital("data").unwrap_or_default();
        let enable = ctx.input_digital("enable").unwrap_or_default();
        let mut q = ctx.int_state(0);

        if enable.state.is_high() {
            q = if data.state.is_high() { 1 } else { 0 };
        }

        ctx.set_int_state(0, q);
        let q_val = if q == 1 {
            DigitalValue::one()
        } else {
            DigitalValue::zero()
        };
        ctx.set_output_digital("q", q_val, delay);
        ctx.set_output_digital("qn", q_val.invert(), delay);
        Ok(())
    }
}

/// SR Latch
#[derive(Debug, Default)]
pub struct SrLatch;

impl CodeModel for SrLatch {
    fn name(&self) -> &str {
        "d_srlatch"
    }
    fn description(&self) -> &str {
        "SR latch"
    }
    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("s", PortType::Digital),
                PortSpec::input("r", PortType::Digital),
                PortSpec::input("enable", PortType::Digital),
                PortSpec::output("q", PortType::Digital),
                PortSpec::output("qn", PortType::Digital),
            ]
        })
    }
    fn parameters(&self) -> &[ParamSpec] {
        DFlipFlop.parameters()
    }
    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(1);
        Ok(())
    }
    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let delay = ctx.param("clk_delay");
        let s = ctx.input_digital("s").unwrap_or_default();
        let r = ctx.input_digital("r").unwrap_or_default();
        let enable = ctx.input_digital("enable").unwrap_or_default();
        let mut q = ctx.int_state(0);

        if enable.state.is_high() {
            if s.state.is_high() && !r.state.is_high() {
                q = 1;
            } else if r.state.is_high() && !s.state.is_high() {
                q = 0;
            }
        }

        ctx.set_int_state(0, q);
        let q_val = if q == 1 {
            DigitalValue::one()
        } else {
            DigitalValue::zero()
        };
        ctx.set_output_digital("q", q_val, delay);
        ctx.set_output_digital("qn", q_val.invert(), delay);
        Ok(())
    }
}

//=============================================================================
// Memory
//=============================================================================

/// RAM model
#[derive(Debug, Default)]
pub struct DigitalRam;

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
                PortSpec::input("select", PortType::Digital),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("read_delay", 1e-9).with_min(0.0),
                ParamSpec::real("write_delay", 1e-9).with_min(0.0),
                ParamSpec::integer("address_width", 8),
                ParamSpec::integer("data_width", 8),
            ]
        })
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }
    fn evaluate(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }
}

/// ROM model
#[derive(Debug, Default)]
pub struct DigitalRom;

impl CodeModel for DigitalRom {
    fn name(&self) -> &str {
        "d_rom"
    }
    fn description(&self) -> &str {
        "Read-only memory"
    }
    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::vector_input("address", PortType::Digital),
                PortSpec::input("select", PortType::Digital),
                PortSpec::vector_output("data_out", PortType::Digital),
            ]
        })
    }
    fn parameters(&self) -> &[ParamSpec] {
        DigitalRam.parameters()
    }
    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }
    fn evaluate(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverter() {
        let model = DigitalInverter;
        let mut ctx = CmContext::new();
        ctx.set_param("rise_delay", 1e-9);
        ctx.set_param("fall_delay", 1e-9);
        model.init(&mut ctx).unwrap();

        // Set previous state to 1 so output of 0 triggers a state change
        ctx.set_int_state(0, 1);

        ctx.set_input_digital("in", DigitalValue::one());
        model.evaluate(&mut ctx).unwrap();
        let events = ctx.take_pending_events();
        assert!(
            !events.is_empty(),
            "Inverter should schedule event on state change"
        );
        assert!(events[0].1.state.is_low(), "Inverter of 1 should output 0");
    }

    #[test]
    fn test_and_gate() {
        let model = DigitalAnd;
        let mut ctx = CmContext::new();
        ctx.set_param("rise_delay", 1e-9);
        ctx.set_param("fall_delay", 1e-9);
        model.init(&mut ctx).unwrap();

        use crate::xspice::context::InputValue;
        ctx.set_input(
            "in",
            InputValue::DigitalVector(vec![DigitalValue::one(), DigitalValue::one()]),
        );
        model.evaluate(&mut ctx).unwrap();
        let events = ctx.take_pending_events();
        assert!(!events.is_empty());
        assert!(events[0].1.state.is_high());
    }

    #[test]
    fn test_dff() {
        let model = DFlipFlop;
        let mut ctx = CmContext::new();
        ctx.set_param("clk_delay", 1e-9);
        ctx.set_param("set_delay", 1e-9);
        ctx.set_param("reset_delay", 1e-9);
        ctx.set_param("ic", 0.0);
        model.init(&mut ctx).unwrap();

        ctx.set_input_digital("data", DigitalValue::one());
        ctx.set_input_digital("clk", DigitalValue::zero());
        model.evaluate(&mut ctx).unwrap();

        // Rising edge
        ctx.set_input_digital("clk", DigitalValue::one());
        model.evaluate(&mut ctx).unwrap();

        let events = ctx.take_pending_events();
        assert!(!events.is_empty());
    }

    #[test]
    fn test_or_gate() {
        let model = DigitalOr;
        let mut ctx = CmContext::new();
        ctx.set_param("rise_delay", 1e-9);
        ctx.set_param("fall_delay", 1e-9);
        model.init(&mut ctx).unwrap();

        use crate::xspice::context::InputValue;

        // Test 0 OR 0 = 0
        ctx.set_int_state(0, 1); // Force state change
        ctx.set_input(
            "in",
            InputValue::DigitalVector(vec![DigitalValue::zero(), DigitalValue::zero()]),
        );
        model.evaluate(&mut ctx).unwrap();
        let events = ctx.take_pending_events();
        assert!(!events.is_empty(), "OR gate should schedule event");
        assert!(events[0].1.state.is_low(), "0 OR 0 = 0");

        // Test 1 OR 0 = 1
        ctx.set_int_state(0, 0);
        ctx.set_input(
            "in",
            InputValue::DigitalVector(vec![DigitalValue::one(), DigitalValue::zero()]),
        );
        model.evaluate(&mut ctx).unwrap();
        let events = ctx.take_pending_events();
        assert!(!events.is_empty());
        assert!(events[0].1.state.is_high(), "1 OR 0 = 1");
    }

    #[test]
    fn test_xor_gate() {
        let model = DigitalXor;
        let mut ctx = CmContext::new();
        ctx.set_param("rise_delay", 1e-9);
        ctx.set_param("fall_delay", 1e-9);
        model.init(&mut ctx).unwrap();

        use crate::xspice::context::InputValue;

        // Test 1 XOR 0 = 1
        ctx.set_input(
            "in",
            InputValue::DigitalVector(vec![DigitalValue::one(), DigitalValue::zero()]),
        );
        model.evaluate(&mut ctx).unwrap();
        let events = ctx.take_pending_events();
        assert!(!events.is_empty());
        assert!(events[0].1.state.is_high(), "1 XOR 0 = 1");

        // Test 1 XOR 1 = 0
        ctx.set_int_state(0, 1);
        ctx.set_input(
            "in",
            InputValue::DigitalVector(vec![DigitalValue::one(), DigitalValue::one()]),
        );
        model.evaluate(&mut ctx).unwrap();
        let events = ctx.take_pending_events();
        assert!(!events.is_empty());
        assert!(events[0].1.state.is_low(), "1 XOR 1 = 0");
    }

    #[test]
    fn test_nand_gate() {
        let model = DigitalNand;
        let mut ctx = CmContext::new();
        ctx.set_param("rise_delay", 1e-9);
        ctx.set_param("fall_delay", 1e-9);
        model.init(&mut ctx).unwrap();

        use crate::xspice::context::InputValue;

        // Test 1 NAND 1 = 0
        ctx.set_int_state(0, 1);
        ctx.set_input(
            "in",
            InputValue::DigitalVector(vec![DigitalValue::one(), DigitalValue::one()]),
        );
        model.evaluate(&mut ctx).unwrap();
        let events = ctx.take_pending_events();
        assert!(!events.is_empty());
        assert!(events[0].1.state.is_low(), "1 NAND 1 = 0");

        // Test 1 NAND 0 = 1
        ctx.set_int_state(0, 0);
        ctx.set_input(
            "in",
            InputValue::DigitalVector(vec![DigitalValue::one(), DigitalValue::zero()]),
        );
        model.evaluate(&mut ctx).unwrap();
        let events = ctx.take_pending_events();
        assert!(!events.is_empty());
        assert!(events[0].1.state.is_high(), "1 NAND 0 = 1");
    }

    #[test]
    fn test_nor_gate() {
        let model = DigitalNor;
        let mut ctx = CmContext::new();
        ctx.set_param("rise_delay", 1e-9);
        ctx.set_param("fall_delay", 1e-9);
        model.init(&mut ctx).unwrap();

        use crate::xspice::context::InputValue;

        // Test 0 NOR 0 = 1
        ctx.set_input(
            "in",
            InputValue::DigitalVector(vec![DigitalValue::zero(), DigitalValue::zero()]),
        );
        model.evaluate(&mut ctx).unwrap();
        let events = ctx.take_pending_events();
        assert!(!events.is_empty());
        assert!(events[0].1.state.is_high(), "0 NOR 0 = 1");

        // Test 1 NOR 0 = 0
        ctx.set_int_state(0, 1);
        ctx.set_input(
            "in",
            InputValue::DigitalVector(vec![DigitalValue::one(), DigitalValue::zero()]),
        );
        model.evaluate(&mut ctx).unwrap();
        let events = ctx.take_pending_events();
        assert!(!events.is_empty());
        assert!(events[0].1.state.is_low(), "1 NOR 0 = 0");
    }

    #[test]
    fn test_buffer() {
        let model = DigitalBuffer;
        let mut ctx = CmContext::new();
        ctx.set_param("rise_delay", 1e-9);
        ctx.set_param("fall_delay", 1e-9);
        model.init(&mut ctx).unwrap();

        // Set previous state different from expected
        ctx.set_int_state(0, 0);
        ctx.set_input_digital("in", DigitalValue::one());
        model.evaluate(&mut ctx).unwrap();
        let events = ctx.take_pending_events();
        assert!(!events.is_empty(), "Buffer should schedule event");
        assert!(events[0].1.state.is_high(), "Buffer of 1 = 1");
    }

    #[test]
    fn test_tristate() {
        let model = DigitalTristate;
        let mut ctx = CmContext::new();
        ctx.set_param("delay", 1e-9);
        model.init(&mut ctx).unwrap();

        // Enable = 1, input = 1 -> output = 1
        ctx.set_int_state(0, 0);
        ctx.set_input_digital("in", DigitalValue::one());
        ctx.set_input_digital("enable", DigitalValue::one());
        model.evaluate(&mut ctx).unwrap();
        let events = ctx.take_pending_events();
        assert!(!events.is_empty());
        assert!(events[0].1.state.is_high());

        // Enable = 0 -> output = high-Z
        ctx.set_int_state(0, 1);
        ctx.set_input_digital("enable", DigitalValue::zero());
        model.evaluate(&mut ctx).unwrap();
        let events = ctx.take_pending_events();
        assert!(!events.is_empty());
        assert!(
            events[0].1.state.is_high_z(),
            "Disabled tristate should output high-Z"
        );
    }

    #[test]
    fn test_jk_flipflop() {
        let model = JkFlipFlop;
        let mut ctx = CmContext::new();
        ctx.set_param("clk_delay", 1e-9);
        ctx.set_param("set_delay", 1e-9);
        ctx.set_param("reset_delay", 1e-9);
        ctx.set_param("ic", 0.0);
        model.init(&mut ctx).unwrap();

        // J=1, K=0: Set on rising edge
        ctx.set_input_digital("j", DigitalValue::one());
        ctx.set_input_digital("k", DigitalValue::zero());
        ctx.set_input_digital("clk", DigitalValue::zero());
        ctx.set_input_digital("set", DigitalValue::zero());
        ctx.set_input_digital("reset", DigitalValue::zero());
        model.evaluate(&mut ctx).unwrap();

        // Rising edge
        ctx.set_input_digital("clk", DigitalValue::one());
        model.evaluate(&mut ctx).unwrap();
        let events = ctx.take_pending_events();
        assert!(
            !events.is_empty(),
            "JK flip-flop should output on clock edge"
        );
    }

    #[test]
    fn test_t_flipflop() {
        let model = TFlipFlop;
        let mut ctx = CmContext::new();
        ctx.set_param("clk_delay", 1e-9);
        ctx.set_param("set_delay", 1e-9);
        ctx.set_param("reset_delay", 1e-9);
        ctx.set_param("ic", 0.0);
        model.init(&mut ctx).unwrap();

        // T=1: Toggle on rising edge
        ctx.set_input_digital("t", DigitalValue::one());
        ctx.set_input_digital("clk", DigitalValue::zero());
        ctx.set_input_digital("set", DigitalValue::zero());
        ctx.set_input_digital("reset", DigitalValue::zero());
        model.evaluate(&mut ctx).unwrap();

        // Rising edge should toggle
        ctx.set_input_digital("clk", DigitalValue::one());
        model.evaluate(&mut ctx).unwrap();
        let events = ctx.take_pending_events();
        assert!(
            !events.is_empty(),
            "T flip-flop should toggle on clock edge when T=1"
        );
    }

    #[test]
    fn test_sr_latch() {
        let model = SrLatch;
        let mut ctx = CmContext::new();
        ctx.set_param("clk_delay", 1e-9); // SR latch uses DFlipFlop parameters
        ctx.set_param("ic", 0.0);
        model.init(&mut ctx).unwrap();

        // Set enable high (required for SR latch to respond)
        ctx.set_input_digital("enable", DigitalValue::one());
        // S=1, R=0: Set (use correct port names: s and r)
        ctx.set_input_digital("s", DigitalValue::one());
        ctx.set_input_digital("r", DigitalValue::zero());
        model.evaluate(&mut ctx).unwrap();
        let events = ctx.take_pending_events();
        assert!(!events.is_empty(), "SR latch should set when S=1");
        // Find the 'q' output event (first one should be q)
        let q_event = events.iter().find(|e| e.0 == "q");
        assert!(q_event.is_some(), "Should have a 'q' output event");
        assert!(
            q_event.unwrap().1.state.is_high(),
            "SR latch Q should be high when set"
        );
    }

    #[test]
    fn test_pullup() {
        let model = DigitalPullup;
        let mut ctx = CmContext::new();
        model.init(&mut ctx).unwrap();
        model.evaluate(&mut ctx).unwrap();
        // Pullup should always output weak high
        let events = ctx.take_pending_events();
        assert!(!events.is_empty());
        assert!(events[0].1.state.is_high());
    }

    #[test]
    fn test_pulldown() {
        let model = DigitalPulldown;
        let mut ctx = CmContext::new();
        model.init(&mut ctx).unwrap();
        model.evaluate(&mut ctx).unwrap();
        // Pulldown should always output weak low
        let events = ctx.take_pending_events();
        assert!(!events.is_empty());
        assert!(events[0].1.state.is_low());
    }
}
