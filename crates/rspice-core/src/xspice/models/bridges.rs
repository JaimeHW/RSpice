//! A/D and D/A Bridge Code Models

use crate::xspice::{CmContext, CmResult, CodeModel, DigitalValue, ParamSpec, PortSpec, PortType};

/// Analog to digital converter bridge
#[derive(Debug, Default)]
pub struct AdcBridge;

impl CodeModel for AdcBridge {
    fn name(&self) -> &str {
        "adc_bridge"
    }

    fn description(&self) -> &str {
        "Analog to digital converter"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in", PortType::Voltage),
                PortSpec::output("out", PortType::Digital),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("in_low", 0.8),
                ParamSpec::real("in_high", 2.0),
                ParamSpec::real("rise_delay", 1e-9).with_min(0.0),
                ParamSpec::real("fall_delay", 1e-9).with_min(0.0),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(1);
        ctx.set_int_state(0, -1);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let in_low = ctx.param("in_low");
        let in_high = ctx.param("in_high");
        let rise_delay = ctx.param("rise_delay");
        let fall_delay = ctx.param("fall_delay");
        let v_in = ctx.input("in");
        let prev = ctx.int_state(0);

        let new_state = if v_in >= in_high {
            1
        } else if v_in <= in_low {
            0
        } else if prev == 0 || prev == 1 {
            prev
        } else {
            -1
        };

        if new_state != prev {
            let val = match new_state {
                0 => DigitalValue::zero(),
                1 => DigitalValue::one(),
                _ => DigitalValue::unknown(),
            };
            let delay = if new_state == 1 {
                rise_delay
            } else {
                fall_delay
            };
            ctx.set_output_digital("out", val, delay);
        }
        ctx.set_int_state(0, new_state);
        Ok(())
    }
}

/// Digital to analog converter bridge
#[derive(Debug, Default)]
pub struct DacBridge;

impl CodeModel for DacBridge {
    fn name(&self) -> &str {
        "dac_bridge"
    }

    fn description(&self) -> &str {
        "Digital to analog converter"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in", PortType::Digital),
                PortSpec::output("out", PortType::Voltage),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("out_low", 0.0),
                ParamSpec::real("out_high", 5.0),
                ParamSpec::real("out_undef", 2.5),
                ParamSpec::real("t_rise", 1e-9).with_min(0.0),
                ParamSpec::real("t_fall", 1e-9).with_min(0.0),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_states(2);
        let undef = ctx.param("out_undef");
        ctx.set_state(0, undef);
        ctx.set_state(1, undef);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let out_low = ctx.param("out_low");
        let out_high = ctx.param("out_high");
        let out_undef = ctx.param("out_undef");
        let t_rise = ctx.param("t_rise");
        let t_fall = ctx.param("t_fall");

        let d_in = ctx.input_digital("in").unwrap_or_default();
        let v_target = if d_in.state.is_high() {
            out_high
        } else if d_in.state.is_low() {
            out_low
        } else {
            out_undef
        };

        let v_prev = ctx.state(0);
        let dt = ctx.timestep;

        let v_out = if (v_target - v_prev).abs() < 1e-12 {
            v_target
        } else if v_target > v_prev {
            let rate = (out_high - out_low) / t_rise.max(1e-15);
            (v_prev + rate * dt).min(v_target)
        } else {
            let rate = (out_high - out_low) / t_fall.max(1e-15);
            (v_prev - rate * dt).max(v_target)
        };

        ctx.set_state(0, v_out);
        ctx.set_state(1, v_target);
        ctx.set_output("out", v_out);
        Ok(())
    }
}

