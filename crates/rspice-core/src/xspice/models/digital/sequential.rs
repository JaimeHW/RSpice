use super::*;

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
