//! Analog Behavioral Code Models
//!
//! Implements fundamental analog behavioral blocks used in mixed-signal simulation.

use crate::Value;
use crate::xspice::{CmContext, CmResult, CodeModel, ParamSpec, PortDirection, PortSpec, PortType};

//=============================================================================
// Gain Block
//=============================================================================

/// Gain code model: out = gain * (in + in_offset) + out_offset
///
/// # Parameters
/// - `gain` - Voltage gain (default: 1.0)
/// - `in_offset` - Input offset voltage (default: 0.0)
/// - `out_offset` - Output offset voltage (default: 0.0)
///
/// # Ports
/// - `in` - Analog voltage input
/// - `out` - Analog voltage output
#[derive(Debug, Default)]
pub struct Gain;

impl CodeModel for Gain {
    fn name(&self) -> &str {
        "gain"
    }

    fn description(&self) -> &str {
        "Voltage gain block: out = gain * (in + in_offset) + out_offset"
    }

    fn ports(&self) -> &[PortSpec] {
        static PORTS: [PortSpec; 2] = [
            PortSpec {
                name: String::new(), // Will be replaced
                direction: PortDirection::In,
                default_type: PortType::Voltage,
                allowed_types: vec![],
                is_vector: false,
                null_allowed: false,
                description: String::new(),
            },
            PortSpec {
                name: String::new(),
                direction: PortDirection::Out,
                default_type: PortType::Voltage,
                allowed_types: vec![],
                is_vector: false,
                null_allowed: false,
                description: String::new(),
            },
        ];

        // Use lazy_static pattern for proper port specs
        use std::sync::OnceLock;
        static REAL_PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        REAL_PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in", PortType::Voltage).with_description("Analog voltage input"),
                PortSpec::output("out", PortType::Voltage)
                    .with_description("Analog voltage output"),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("gain", 1.0).with_description("Voltage gain factor"),
                ParamSpec::real("in_offset", 0.0).with_description("Input offset voltage"),
                ParamSpec::real("out_offset", 0.0).with_description("Output offset voltage"),
            ]
        })
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let gain = ctx.param("gain");
        let in_offset = ctx.param("in_offset");
        let out_offset = ctx.param("out_offset");

        let v_in = ctx.input("in");
        let v_out = gain * (v_in + in_offset) + out_offset;

        // Provide both operating-point output and linearized gain.
        ctx.set_output_with_partial("out", v_out, gain);

        Ok(())
    }

    fn ac_gain(&self, ctx: &CmContext) -> Vec<Value> {
        vec![ctx.param("gain")]
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if output_port.eq_ignore_ascii_case("out") {
            vec![("in".to_string(), ctx.param("gain"))]
        } else {
            Vec::new()
        }
    }
}

//=============================================================================
// Summer
//=============================================================================

/// Analog summer: out = sum(in[i] * in_gain[i]) + out_offset
///
/// # Parameters
/// - `in_gain` - Gain for each input (default: 1.0)
/// - `out_offset` - Output offset (default: 0.0)
///
/// # Ports
/// - `in` - Vector analog voltage input
/// - `out` - Analog voltage output
#[derive(Debug, Default)]
pub struct Summer;

impl CodeModel for Summer {
    fn name(&self) -> &str {
        "summer"
    }

    fn description(&self) -> &str {
        "Analog summer: out = sum(in[i] * in_gain[i]) + out_offset"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::vector_input("in", PortType::Voltage)
                    .with_description("Vector of analog voltage inputs"),
                PortSpec::output("out", PortType::Voltage).with_description("Summed analog output"),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("in_gain", 1.0)
                    .with_description("Gain applied to each input (or array)"),
                ParamSpec::real("out_offset", 0.0).with_description("Output offset voltage"),
            ]
        })
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let in_gain = ctx.param("in_gain");
        let out_offset = ctx.param("out_offset");

        let inputs = ctx.input_vector("in");
        let sum: Value = inputs.iter().map(|&v| v * in_gain).sum();
        let v_out = sum + out_offset;

        ctx.set_output("out", v_out);
        Ok(())
    }
}

//=============================================================================
// Multiplier
//=============================================================================

/// Analog multiplier: out = in0 * in1 * out_gain + out_offset
///
/// # Parameters
/// - `out_gain` - Output gain factor (default: 1.0)
/// - `out_offset` - Output offset (default: 0.0)
///
/// # Ports
/// - `in0`, `in1` - Analog voltage inputs
/// - `out` - Analog voltage output
#[derive(Debug, Default)]
pub struct Multiplier;

impl CodeModel for Multiplier {
    fn name(&self) -> &str {
        "mult"
    }

    fn description(&self) -> &str {
        "Analog multiplier: out = in0 * in1 * out_gain + out_offset"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in0", PortType::Voltage).with_description("First analog input"),
                PortSpec::input("in1", PortType::Voltage).with_description("Second analog input"),
                PortSpec::output("out", PortType::Voltage).with_description("Product output"),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("out_gain", 1.0).with_description("Output gain factor"),
                ParamSpec::real("out_offset", 0.0).with_description("Output offset voltage"),
            ]
        })
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let out_gain = ctx.param("out_gain");
        let out_offset = ctx.param("out_offset");

        let in0 = ctx.input("in0");
        let in1 = ctx.input("in1");
        let v_out = in0 * in1 * out_gain + out_offset;

        ctx.set_output("out", v_out);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if output_port.eq_ignore_ascii_case("out") {
            let out_gain = ctx.param("out_gain");
            vec![
                ("in0".to_string(), ctx.input("in1") * out_gain),
                ("in1".to_string(), ctx.input("in0") * out_gain),
            ]
        } else {
            Vec::new()
        }
    }
}

//=============================================================================
// Divider
//=============================================================================

/// Analog divider: out = (num / den) * out_gain + out_offset
///
/// # Parameters
/// - `out_gain` - Output gain (default: 1.0)
/// - `out_offset` - Output offset (default: 0.0)
/// - `den_lower_limit` - Minimum denominator magnitude (default: 1e-12)
///
/// # Ports
/// - `num` - Numerator input
/// - `den` - Denominator input
/// - `out` - Quotient output
#[derive(Debug, Default)]
pub struct Divider;

impl CodeModel for Divider {
    fn name(&self) -> &str {
        "divider"
    }

    fn description(&self) -> &str {
        "Analog divider: out = (num / den) * out_gain + out_offset"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("num", PortType::Voltage).with_description("Numerator input"),
                PortSpec::input("den", PortType::Voltage).with_description("Denominator input"),
                PortSpec::output("out", PortType::Voltage).with_description("Quotient output"),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("out_gain", 1.0).with_description("Output gain factor"),
                ParamSpec::real("out_offset", 0.0).with_description("Output offset voltage"),
                ParamSpec::real("den_lower_limit", 1e-12)
                    .with_min(0.0)
                    .with_description("Minimum denominator magnitude to prevent division by zero"),
            ]
        })
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let out_gain = ctx.param("out_gain");
        let out_offset = ctx.param("out_offset");
        let den_limit = ctx.param("den_lower_limit");

        let num = ctx.input("num");
        let den = ctx.input("den");

        // Prevent division by zero
        let safe_den = if den.abs() < den_limit {
            den_limit.copysign(den)
        } else {
            den
        };

        let v_out = (num / safe_den) * out_gain + out_offset;
        ctx.set_output("out", v_out);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }

        let out_gain = ctx.param("out_gain");
        let den_limit = ctx.param("den_lower_limit");
        let num = ctx.input("num");
        let den = ctx.input("den");
        let den_limited = den.abs() < den_limit;
        let safe_den = if den_limited {
            den_limit.copysign(den)
        } else {
            den
        };
        if safe_den == 0.0 || !safe_den.is_finite() {
            return Vec::new();
        }

        vec![
            ("num".to_string(), out_gain / safe_den),
            (
                "den".to_string(),
                if den_limited {
                    0.0
                } else {
                    -out_gain * num / (safe_den * safe_den)
                },
            ),
        ]
    }
}

//=============================================================================
// Limiter
//=============================================================================

/// Limiter with gain: out = clamp(gain * (in + in_offset) + out_offset, out_lower, out_upper)
///
/// # Parameters
/// - `gain` - Gain in linear region (default: 1.0)
/// - `in_offset` - Input offset (default: 0.0)
/// - `out_offset` - Output offset (default: 0.0)
/// - `out_lower_limit` - Lower output limit (default: -1e12)
/// - `out_upper_limit` - Upper output limit (default: 1e12)
/// - `limit_range` - Smoothing range (default: 0.01)
///
/// # Ports
/// - `in` - Analog voltage input
/// - `out` - Analog voltage output (limited)
#[derive(Debug, Default)]
pub struct Limiter;

impl CodeModel for Limiter {
    fn name(&self) -> &str {
        "limit"
    }

    fn description(&self) -> &str {
        "Limiter with gain and offset"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in", PortType::Voltage).with_description("Analog voltage input"),
                PortSpec::output("out", PortType::Voltage)
                    .with_description("Limited analog output"),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("gain", 1.0).with_description("Gain in linear region"),
                ParamSpec::real("in_offset", 0.0).with_description("Input offset voltage"),
                ParamSpec::real("out_offset", 0.0).with_description("Output offset voltage"),
                ParamSpec::real("out_lower_limit", -1e12).with_description("Lower output limit"),
                ParamSpec::real("out_upper_limit", 1e12).with_description("Upper output limit"),
                ParamSpec::real("limit_range", 0.01)
                    .with_min(0.0)
                    .with_description("Smoothing range for soft limiting"),
            ]
        })
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let gain = ctx.param("gain");
        let in_offset = ctx.param("in_offset");
        let out_offset = ctx.param("out_offset");
        let lower = ctx.param("out_lower_limit");
        let upper = ctx.param("out_upper_limit");
        let range = ctx.param("limit_range");

        let v_in = ctx.input("in");
        let linear_out = gain * (v_in + in_offset) + out_offset;

        // Soft limiting (smooth clamp) using tanh-based smoothing
        let v_out = if range > 0.0 {
            soft_clamp(linear_out, lower, upper, range)
        } else {
            linear_out.clamp(lower, upper)
        };

        // Compute partial derivative for Newton-Raphson
        let partial = if linear_out < lower || linear_out > upper {
            0.0 // Output is clipped, derivative is zero
        } else {
            gain
        };

        ctx.set_output_with_partial("out", v_out, partial);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if output_port.eq_ignore_ascii_case("out") {
            vec![("in".to_string(), ctx.partial("out"))]
        } else {
            Vec::new()
        }
    }
}

//=============================================================================
// Controlled Limiter
//=============================================================================

/// Controlled limiter: out = clamp(gain * (in + in_offset), lower, upper)
///
/// The lower and upper limits are controlled by analog input ports, matching
/// ngspice's XSPICE `climit` code model.
#[derive(Debug, Default)]
pub struct ControlledLimiter;

impl CodeModel for ControlledLimiter {
    fn name(&self) -> &str {
        "climit"
    }

    fn description(&self) -> &str {
        "Controlled limiter block with analog upper and lower limit inputs"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in", PortType::Voltage).with_description("Analog input"),
                PortSpec::input("cntl_upper", PortType::Voltage)
                    .with_description("Upper limit control input"),
                PortSpec::input("cntl_lower", PortType::Voltage)
                    .with_description("Lower limit control input"),
                PortSpec::output("out", PortType::Voltage)
                    .with_description("Controlled limited output"),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("in_offset", 0.0).with_description("Input offset voltage"),
                ParamSpec::real("gain", 1.0).with_description("Linear-region gain"),
                ParamSpec::real("upper_delta", 0.0)
                    .with_description("Delta subtracted from the upper control input"),
                ParamSpec::real("lower_delta", 0.0)
                    .with_description("Delta added to the lower control input"),
                ParamSpec::real("limit_range", 1.0e-6)
                    .with_description("Smoothing range near the controlled limits"),
                ParamSpec::boolean("fraction", false)
                    .with_description("Treat limit_range as a fraction of the limit span"),
            ]
        })
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let (out, pout_pin, _pout_pcntl_lower, _pout_pcntl_upper) = climit_transfer(
            ctx.input("in"),
            ctx.param("in_offset"),
            ctx.input("cntl_upper"),
            ctx.input("cntl_lower"),
            ctx.param("lower_delta"),
            ctx.param("upper_delta"),
            ctx.param("limit_range"),
            ctx.param("gain"),
            ctx.param("fraction") > 0.5,
        );

        ctx.set_output_with_partial("out", out, pout_pin);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if output_port.eq_ignore_ascii_case("out") {
            vec![("in".to_string(), ctx.partial("out"))]
        } else {
            Vec::new()
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn climit_transfer(
    input: Value,
    in_offset: Value,
    cntl_upper: Value,
    cntl_lower: Value,
    lower_delta: Value,
    upper_delta: Value,
    mut limit_range: Value,
    gain: Value,
    fraction: bool,
) -> (Value, Value, Value, Value) {
    let out_lower_limit = cntl_lower + lower_delta;
    let out_upper_limit = cntl_upper - upper_delta;

    if fraction {
        limit_range *= out_upper_limit - out_lower_limit;
    }
    if !limit_range.is_finite() {
        limit_range = 0.0;
    }

    let threshold_upper = out_upper_limit - limit_range;
    let threshold_lower = out_lower_limit + limit_range;
    if threshold_upper < threshold_lower {
        return (0.0, 0.0, 0.0, 0.0);
    }

    let raw = gain * (in_offset + input);
    if raw < threshold_lower {
        if raw > out_lower_limit - limit_range && limit_range > 0.0 {
            let (limited_out, mut pout_pin) =
                smooth_corner(raw, out_lower_limit, out_lower_limit, limit_range, 0.0, 1.0);
            pout_pin *= gain;
            let (pout_pcntl_lower, _) =
                smooth_discontinuity(raw, out_lower_limit, 1.0, threshold_lower, 0.0);
            (limited_out, pout_pin, pout_pcntl_lower, 0.0)
        } else {
            (out_lower_limit, 0.0, 1.0, 0.0)
        }
    } else if raw > threshold_upper {
        if raw < out_upper_limit + limit_range && limit_range > 0.0 {
            let (limited_out, mut pout_pin) =
                smooth_corner(raw, out_upper_limit, out_upper_limit, limit_range, 1.0, 0.0);
            pout_pin *= gain;
            let (pout_pcntl_upper, _) =
                smooth_discontinuity(raw, threshold_upper, 0.0, out_upper_limit, 1.0);
            (limited_out, pout_pin, 0.0, pout_pcntl_upper)
        } else {
            (out_upper_limit, 0.0, 0.0, 1.0)
        }
    } else {
        (raw, gain, 0.0, 0.0)
    }
}

fn smooth_corner(
    x_input: Value,
    x_center: Value,
    y_center: Value,
    domain: Value,
    lower_slope: Value,
    upper_slope: Value,
) -> (Value, Value) {
    let x_upper = x_center + domain;
    let y_upper = y_center + upper_slope * domain;
    let a = ((upper_slope - lower_slope) / 4.0) / domain;
    let b = upper_slope - 2.0 * a * x_upper;
    let c = y_upper - a * x_upper * x_upper - b * x_upper;
    let y_output = a * x_input * x_input + b * x_input + c;
    let dy_dx = 2.0 * a * x_input + b;
    (y_output, dy_dx)
}

fn smooth_discontinuity(
    x_input: Value,
    x_lower: Value,
    y_lower: Value,
    x_upper: Value,
    y_upper: Value,
) -> (Value, Value) {
    let x_center = (x_upper + x_lower) / 2.0;
    let y_center = (y_upper + y_lower) / 2.0;
    let center_slope = 2.0 * (y_upper - y_lower) / (x_upper - x_lower);

    if x_input < x_lower {
        (y_lower, 0.0)
    } else if x_input < x_center {
        let a = center_slope / (x_upper - x_lower);
        let b = center_slope - 2.0 * a * x_center;
        let c = y_center - a * x_center * x_center - b * x_center;
        (
            a * x_input * x_input + b * x_input + c,
            2.0 * a * x_input + b,
        )
    } else if x_input < x_upper {
        let a = -center_slope / (x_upper - x_lower);
        let b = -2.0 * a * x_upper;
        let c = y_upper - a * x_upper * x_upper - b * x_upper;
        (
            a * x_input * x_input + b * x_input + c,
            2.0 * a * x_input + b,
        )
    } else {
        (y_upper, 0.0)
    }
}

/// Soft clamp function for smooth limiting
fn soft_clamp(x: f64, lower: f64, upper: f64, range: f64) -> f64 {
    let mid = (lower + upper) / 2.0;
    let half_range = (upper - lower) / 2.0;

    if half_range <= 0.0 {
        return mid;
    }

    // Scale and apply tanh for smoothness
    let scaled = (x - mid) / half_range;
    mid + half_range * scaled.clamp(-1.0 + range, 1.0 - range)
}

//=============================================================================
// Integrator
//=============================================================================

/// Continuous-time integrator: out = gain * integral(in) + out_ic
///
/// # Parameters
/// - `gain` - Integration gain (default: 1.0)
/// - `out_ic` - Initial condition (default: 0.0)
/// - `out_lower_limit` - Lower saturation (default: -1e12)
/// - `out_upper_limit` - Upper saturation (default: 1e12)
///
/// # Ports
/// - `in` - Input to integrate
/// - `out` - Integrated output
#[derive(Debug, Default)]
pub struct Integrator;

impl CodeModel for Integrator {
    fn name(&self) -> &str {
        "integrator"
    }

    fn description(&self) -> &str {
        "Continuous-time integrator with saturation limits"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in", PortType::Voltage)
                    .with_description("Input voltage to integrate"),
                PortSpec::output("out", PortType::Voltage).with_description("Integrated output"),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("gain", 1.0).with_description("Integration gain (1/time_constant)"),
                ParamSpec::real("out_ic", 0.0).with_description("Initial output value"),
                ParamSpec::real("out_lower_limit", -1e12)
                    .with_description("Lower output saturation limit"),
                ParamSpec::real("out_upper_limit", 1e12)
                    .with_description("Upper output saturation limit"),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        // State layout:
        // state[0] = integrated output
        // state[1] = previous input sample for trapezoidal integration
        ctx.allocate_states(2);

        // Set initial condition
        let ic = ctx.param("out_ic");
        ctx.set_state(0, ic);
        ctx.set_state(1, 0.0);
        ctx.advance_state();

        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let gain = ctx.param("gain");
        let lower_limit = ctx.param("out_lower_limit");
        let upper_limit = ctx.param("out_upper_limit");
        let (lower, upper) = if lower_limit <= upper_limit {
            (lower_limit, upper_limit)
        } else {
            (upper_limit, lower_limit)
        };

        let v_in_raw = ctx.input("in");
        let v_in = if v_in_raw.is_finite() { v_in_raw } else { 0.0 };
        let dt = ctx.timestep;
        let prev_out = ctx.state_prev(0);
        let prev_in = ctx.state_prev(1);

        // Trapezoidal integration:
        // y[n] = y[n-1] + gain * dt * (x[n] + x[n-1]) / 2
        let delta = if dt.is_finite() && dt > 0.0 {
            0.5 * gain * dt * (v_in + prev_in)
        } else {
            0.0
        };
        let new_out = (prev_out + delta).clamp(lower, upper);

        ctx.set_state(0, new_out);
        ctx.set_state(1, v_in);
        ctx.set_output("out", new_out);

        Ok(())
    }
}

//=============================================================================
// Differentiator
//=============================================================================

/// Continuous-time differentiator: out = gain * d(in)/dt
///
/// # Parameters
/// - `gain` - Differentiation gain (default: 1.0)
/// - `out_offset` - Output offset (default: 0.0)
/// - `out_lower_limit` - Lower saturation (default: -1e12)
/// - `out_upper_limit` - Upper saturation (default: 1e12)
///
/// # Ports
/// - `in` - Input to differentiate
/// - `out` - Differentiated output
#[derive(Debug, Default)]
pub struct Differentiator;

impl CodeModel for Differentiator {
    fn name(&self) -> &str {
        "differentiator"
    }

    fn description(&self) -> &str {
        "Continuous-time differentiator with saturation limits"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in", PortType::Voltage)
                    .with_description("Input voltage to differentiate"),
                PortSpec::output("out", PortType::Voltage)
                    .with_description("Differentiated output"),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("gain", 1.0).with_description("Differentiation gain"),
                ParamSpec::real("out_offset", 0.0).with_description("Output offset voltage"),
                ParamSpec::real("out_lower_limit", -1e12)
                    .with_description("Lower output saturation limit"),
                ParamSpec::real("out_upper_limit", 1e12)
                    .with_description("Upper output saturation limit"),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        // Allocate state for previous input
        ctx.allocate_states(1);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let gain = ctx.param("gain");
        let out_offset = ctx.param("out_offset");
        let lower = ctx.param("out_lower_limit");
        let upper = ctx.param("out_upper_limit");

        let v_in = ctx.input("in");
        let dt = ctx.timestep;
        let prev_in = ctx.state_prev(0);

        // Backward difference: dy/dt ≈ (y[n] - y[n-1]) / dt
        let derivative = if dt > 1e-18 {
            (v_in - prev_in) / dt
        } else {
            0.0
        };

        let v_out = (gain * derivative + out_offset).clamp(lower, upper);

        ctx.set_state(0, v_in);
        ctx.set_output("out", v_out);

        Ok(())
    }
}

//=============================================================================
// Official Aliases
//=============================================================================

macro_rules! analog_model_alias {
    ($alias:ident, $target:ident, $name:literal) => {
        #[derive(Debug, Default)]
        pub struct $alias;

        impl CodeModel for $alias {
            fn name(&self) -> &str {
                $name
            }

            fn description(&self) -> &str {
                $target.description()
            }

            fn ports(&self) -> &[PortSpec] {
                $target.ports()
            }

            fn parameters(&self) -> &[ParamSpec] {
                $target.parameters()
            }

            fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
                $target.init(ctx)
            }

            fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
                $target.evaluate(ctx)
            }

            fn ac_gain(&self, ctx: &CmContext) -> Vec<Value> {
                $target.ac_gain(ctx)
            }

            fn output_input_partials(
                &self,
                ctx: &CmContext,
                output_port: &str,
            ) -> Vec<(String, Value)> {
                $target.output_input_partials(ctx, output_port)
            }
        }
    };
}

analog_model_alias!(DivideAlias, Divider, "divide");
analog_model_alias!(IntegratorAlias, Integrator, "int");
analog_model_alias!(DifferentiatorAlias, Differentiator, "d_dt");

//=============================================================================
// Analog Switch
//=============================================================================

/// Analog switch controlled by voltage
///
/// # Parameters
/// - `cntl_on` - Control voltage to turn on (default: 1.0)
/// - `cntl_off` - Control voltage to turn off (default: 0.0)
/// - `r_on` - On-state resistance (default: 1.0)
/// - `r_off` - Off-state resistance (default: 1e12)
/// - `log` - Use logarithmic transition (default: true)
///
/// # Ports
/// - `cntl` - Control voltage input
/// - `ps` - Positive switch terminal
/// - `ns` - Negative switch terminal
#[derive(Debug, Default)]
pub struct AnalogSwitch;

impl CodeModel for AnalogSwitch {
    fn name(&self) -> &str {
        "aswitch"
    }

    fn description(&self) -> &str {
        "Analog switch with voltage control"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("cntl", PortType::Voltage).with_description("Control voltage"),
                PortSpec {
                    name: "ps".to_string(),
                    direction: PortDirection::InOut,
                    default_type: PortType::Voltage,
                    allowed_types: vec![PortType::Voltage],
                    is_vector: false,
                    null_allowed: false,
                    description: "Positive switch terminal".to_string(),
                },
                PortSpec {
                    name: "ns".to_string(),
                    direction: PortDirection::InOut,
                    default_type: PortType::Voltage,
                    allowed_types: vec![PortType::Voltage],
                    is_vector: false,
                    null_allowed: false,
                    description: "Negative switch terminal".to_string(),
                },
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("cntl_on", 1.0).with_description("Control voltage for fully on"),
                ParamSpec::real("cntl_off", 0.0).with_description("Control voltage for fully off"),
                ParamSpec::real("r_on", 1.0)
                    .with_min(1e-9)
                    .with_description("On-state resistance"),
                ParamSpec::real("r_off", 1e12)
                    .with_min(1e-9)
                    .with_description("Off-state resistance"),
                ParamSpec::boolean("log", true)
                    .with_description("Use logarithmic resistance transition"),
            ]
        })
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let cntl_on = ctx.param("cntl_on");
        let cntl_off = ctx.param("cntl_off");
        let r_on = ctx.param("r_on");
        let r_off = ctx.param("r_off");
        let log = ctx.param("log") > 0.5;

        let v_cntl = ctx.input("cntl");

        // Calculate interpolation factor (0 = off, 1 = on)
        let t = if (cntl_on - cntl_off).abs() < 1e-15 {
            if v_cntl >= cntl_on { 1.0 } else { 0.0 }
        } else {
            ((v_cntl - cntl_off) / (cntl_on - cntl_off)).clamp(0.0, 1.0)
        };

        // Interpolate resistance
        let resistance = if log {
            // Logarithmic interpolation
            (r_off.ln() + t * (r_on.ln() - r_off.ln())).exp()
        } else {
            // Linear interpolation
            r_off + t * (r_on - r_off)
        };

        let conductance = 1.0 / resistance;

        // Stamp two-terminal conductance between switch nodes.
        let ps_node = ctx.port_node("ps").unwrap_or(0);
        let ns_node = ctx.port_node("ns").unwrap_or(0);
        let ps = ps_node.checked_sub(1);
        let ns = ns_node.checked_sub(1);

        if let Some(p) = ps {
            ctx.stamp_conductance(p, p, conductance);
        }
        if let Some(n) = ns {
            ctx.stamp_conductance(n, n, conductance);
        }
        if let (Some(p), Some(n)) = (ps, ns) {
            ctx.stamp_conductance(p, n, -conductance);
            ctx.stamp_conductance(n, p, -conductance);
        }

        // Keep a scalar diagnostic output for unit tests and probing.
        ctx.set_output("conductance", conductance);

        Ok(())
    }
}

//=============================================================================
// Sample and Hold
//=============================================================================

/// Sample and hold circuit
///
/// # Parameters
/// - `cntl_th` - Control threshold for sampling (default: 0.5)
/// - `out_ic` - Initial output value (default: 0.0)
///
/// # Ports
/// - `cntl` - Control input (sample when high)
/// - `in` - Analog input to sample
/// - `out` - Held output value
#[derive(Debug, Default)]
pub struct SampleHold;

impl CodeModel for SampleHold {
    fn name(&self) -> &str {
        "s_h"
    }

    fn description(&self) -> &str {
        "Sample and hold circuit"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("cntl", PortType::Voltage)
                    .with_description("Control (sample when above threshold)"),
                PortSpec::input("in", PortType::Voltage).with_description("Analog input to sample"),
                PortSpec::output("out", PortType::Voltage).with_description("Held output value"),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("cntl_th", 0.5).with_description("Control threshold for sampling"),
                ParamSpec::real("out_ic", 0.0).with_description("Initial output value"),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_states(1);
        let ic = ctx.param("out_ic");
        ctx.set_state(0, ic);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let cntl_th = ctx.param("cntl_th");

        let v_cntl = ctx.input("cntl");
        let v_in = ctx.input("in");

        // Sample when control is above threshold
        let held_value = if v_cntl >= cntl_th {
            v_in // Sample new value
        } else {
            ctx.state(0) // Hold previous value
        };

        ctx.set_state(0, held_value);
        ctx.set_output("out", held_value);

        Ok(())
    }
}

//=============================================================================
// Tests
//=============================================================================
