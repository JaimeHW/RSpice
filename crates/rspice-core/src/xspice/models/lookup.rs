//! XSPICE lookup-table code models.
//!
//! Implements the official `pwl` and `pwlts` analog lookup blocks.

use crate::Value;
use crate::xspice::{
    CmContext, CmError, CmResult, CodeModel, ParamSpec, PortDirection, PortSpec, PortType,
};
use std::sync::OnceLock;

const INPUT_DOMAIN_MIN: Value = 1.0e-12;
const INPUT_DOMAIN_MAX: Value = 0.5;

#[derive(Debug, Clone, Copy)]
struct LookupPoint {
    x: Value,
    y: Value,
}

#[derive(Debug, Clone, Copy)]
struct LookupResult {
    value: Value,
    slope: Value,
}

fn lookup_params() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real_vector("x_array", Vec::new())
                .required()
                .with_vector_min_len(2)
                .with_description("Monotonically increasing x-axis lookup points"),
            ParamSpec::real_vector("y_array", Vec::new())
                .required()
                .with_vector_min_len(2)
                .with_description("Lookup output values"),
            ParamSpec::real("input_domain", 0.01).with_description(
                "Smoothing domain around internal breakpoints, clamped to official limits",
            ),
            ParamSpec::boolean("fraction", true)
                .with_description("Treat input_domain as a fraction of adjacent x spacing"),
            ParamSpec::boolean("limit", false)
                .with_description("Clamp outside the table instead of extrapolating"),
        ]
    })
}

fn pwl_input_port() -> PortSpec {
    PortSpec {
        name: "in".to_string(),
        direction: PortDirection::In,
        default_type: PortType::Voltage,
        allowed_types: vec![
            PortType::Voltage,
            PortType::DifferentialVoltage,
            PortType::Current,
            PortType::VoltageName,
        ],
        is_vector: false,
        null_allowed: false,
        vector_min_len: None,
        vector_max_len: None,
        description: "Lookup input".to_string(),
    }
}

fn pwl_output_port() -> PortSpec {
    PortSpec {
        name: "out".to_string(),
        direction: PortDirection::Out,
        default_type: PortType::Voltage,
        allowed_types: vec![
            PortType::Voltage,
            PortType::DifferentialVoltage,
            PortType::Current,
        ],
        is_vector: false,
        null_allowed: false,
        vector_min_len: None,
        vector_max_len: None,
        description: "Lookup output".to_string(),
    }
}

fn lookup_table(ctx: &CmContext) -> CmResult<Vec<LookupPoint>> {
    let x_values = ctx
        .real_vector_param("x_array")
        .ok_or_else(|| missing_param("x_array"))?;
    let y_values = ctx
        .real_vector_param("y_array")
        .ok_or_else(|| missing_param("y_array"))?;
    validate_lookup_table(x_values, y_values)
}

fn missing_param(name: &str) -> CmError {
    CmError::MissingParameter(name.to_string())
}

fn invalid_param(name: &str, message: impl Into<String>) -> CmError {
    CmError::InvalidParameter {
        name: name.to_string(),
        message: message.into(),
    }
}

fn validate_lookup_table(x_values: &[Value], y_values: &[Value]) -> CmResult<Vec<LookupPoint>> {
    if x_values.len() != y_values.len() {
        return Err(invalid_param(
            "x_array/y_array",
            format!(
                "x_array length {} must match y_array length {}",
                x_values.len(),
                y_values.len()
            ),
        ));
    }
    if x_values.len() < 2 {
        return Err(invalid_param(
            "x_array/y_array",
            format!(
                "x_array and y_array require at least 2 points, got {}",
                x_values.len()
            ),
        ));
    }

    let mut points: Vec<LookupPoint> = Vec::with_capacity(x_values.len());
    for (idx, (&x, &y)) in x_values.iter().zip(y_values).enumerate() {
        if !x.is_finite() {
            return Err(invalid_param(
                "x_array",
                format!("point {idx} must be finite, got {x}"),
            ));
        }
        if !y.is_finite() {
            return Err(invalid_param(
                "y_array",
                format!("point {idx} must be finite, got {y}"),
            ));
        }
        if let Some(previous) = points.last()
            && x <= previous.x
        {
            return Err(invalid_param(
                "x_array",
                format!("points must be strictly increasing; point {idx} has x={x}"),
            ));
        }
        points.push(LookupPoint { x, y });
    }

    Ok(points)
}

fn effective_input_domain(input_domain: Value) -> CmResult<Value> {
    if !input_domain.is_finite() {
        return Err(invalid_param(
            "input_domain",
            format!("value must be finite, got {input_domain}"),
        ));
    }

    Ok(input_domain.clamp(INPUT_DOMAIN_MIN, INPUT_DOMAIN_MAX))
}

fn validate_smoothing_domain(
    _points: &[LookupPoint],
    input_domain: Value,
    _fraction: bool,
) -> CmResult<()> {
    effective_input_domain(input_domain)?;
    Ok(())
}

fn segment_slope(left: LookupPoint, right: LookupPoint) -> Value {
    (right.y - left.y) / (right.x - left.x)
}

fn breakpoint_domain(
    points: &[LookupPoint],
    index: usize,
    input_domain: Value,
    fraction: bool,
) -> Value {
    if !fraction {
        return input_domain;
    }
    let lower = points[index].x - points[index - 1].x;
    let upper = points[index + 1].x - points[index].x;
    input_domain * lower.min(upper)
}

fn linear_value(point: LookupPoint, slope: Value, x: Value) -> LookupResult {
    LookupResult {
        value: point.y + slope * (x - point.x),
        slope,
    }
}

fn smooth_corner(
    x: Value,
    x0: Value,
    y0: Value,
    left_slope: Value,
    right_slope: Value,
    domain: Value,
) -> LookupResult {
    let x_left = x0 - domain;
    let span = 2.0 * domain;
    let t = ((x - x_left) / span).clamp(0.0, 1.0);
    let t2 = t * t;
    let t3 = t2 * t;

    let y_left = y0 - left_slope * domain;
    let y_right = y0 + right_slope * domain;
    let m_left = left_slope * span;
    let m_right = right_slope * span;

    let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
    let h10 = t3 - 2.0 * t2 + t;
    let h01 = -2.0 * t3 + 3.0 * t2;
    let h11 = t3 - t2;

    let value = h00 * y_left + h10 * m_left + h01 * y_right + h11 * m_right;

    let dh00 = 6.0 * t2 - 6.0 * t;
    let dh10 = 3.0 * t2 - 4.0 * t + 1.0;
    let dh01 = -6.0 * t2 + 6.0 * t;
    let dh11 = 3.0 * t2 - 2.0 * t;
    let slope = (dh00 * y_left + dh10 * m_left + dh01 * y_right + dh11 * m_right) / span;

    LookupResult { value, slope }
}

fn evaluate_lookup(
    points: &[LookupPoint],
    x: Value,
    input_domain: Value,
    fraction: bool,
    limit: bool,
) -> LookupResult {
    let first = points[0];
    let last = points[points.len() - 1];

    if x <= first.x {
        if limit {
            return LookupResult {
                value: first.y,
                slope: 0.0,
            };
        }
        return linear_value(first, segment_slope(points[0], points[1]), x);
    }
    if x >= last.x {
        if limit {
            return LookupResult {
                value: last.y,
                slope: 0.0,
            };
        }
        return linear_value(
            last,
            segment_slope(points[points.len() - 2], points[points.len() - 1]),
            x,
        );
    }

    for index in 1..points.len() - 1 {
        let domain = breakpoint_domain(points, index, input_domain, fraction);
        if domain > 0.0 && x >= points[index].x - domain && x <= points[index].x + domain {
            return smooth_corner(
                x,
                points[index].x,
                points[index].y,
                segment_slope(points[index - 1], points[index]),
                segment_slope(points[index], points[index + 1]),
                domain,
            );
        }
    }

    let upper = points
        .partition_point(|point| point.x < x)
        .clamp(1, points.len() - 1);
    let lower = upper - 1;
    linear_value(
        points[lower],
        segment_slope(points[lower], points[upper]),
        x,
    )
}

fn evaluate_lookup_context(ctx: &CmContext, x: Value) -> CmResult<LookupResult> {
    let points = lookup_table(ctx)?;
    let input_domain = effective_input_domain(ctx.param("input_domain"))?;
    let fraction = ctx.param("fraction") > 0.5;
    let limit = ctx.param("limit") > 0.5;

    validate_smoothing_domain(&points, input_domain, fraction)?;
    Ok(evaluate_lookup(&points, x, input_domain, fraction, limit))
}

/// Piecewise-linear controlled source.
#[derive(Debug, Default)]
pub struct PiecewiseLinear;

impl CodeModel for PiecewiseLinear {
    fn name(&self) -> &str {
        "pwl"
    }

    fn description(&self) -> &str {
        "Piecewise-linear controlled source"
    }

    fn ports(&self) -> &[PortSpec] {
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| vec![pwl_input_port(), pwl_output_port()])
    }

    fn parameters(&self) -> &[ParamSpec] {
        lookup_params()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        let points = lookup_table(ctx)?;
        validate_smoothing_domain(
            &points,
            ctx.param("input_domain"),
            ctx.param("fraction") > 0.5,
        )
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let result = evaluate_lookup_context(ctx, ctx.input("in"))?;
        ctx.set_output_with_partial("out", result.value, result.slope);
        Ok(())
    }

    fn ac_gain(&self, ctx: &CmContext) -> Vec<Value> {
        match evaluate_lookup_context(ctx, ctx.input("in")) {
            Ok(result) => vec![result.slope],
            Err(_) => vec![0.0],
        }
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }
        match evaluate_lookup_context(ctx, ctx.input("in")) {
            Ok(result) => vec![("in".to_string(), result.slope)],
            Err(_) => Vec::new(),
        }
    }
}

/// Piecewise-linear time-series source.
#[derive(Debug, Default)]
pub struct PiecewiseLinearTimeSeries;

impl CodeModel for PiecewiseLinearTimeSeries {
    fn name(&self) -> &str {
        "pwlts"
    }

    fn description(&self) -> &str {
        "Piecewise-linear source driven by simulation time"
    }

    fn ports(&self) -> &[PortSpec] {
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| vec![pwl_output_port()])
    }

    fn parameters(&self) -> &[ParamSpec] {
        lookup_params()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        let points = lookup_table(ctx)?;
        validate_smoothing_domain(
            &points,
            ctx.param("input_domain"),
            ctx.param("fraction") > 0.5,
        )
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let result = evaluate_lookup_context(ctx, ctx.time)?;
        ctx.set_output("out", result.value);
        Ok(())
    }

    fn ac_gain(&self, _ctx: &CmContext) -> Vec<Value> {
        vec![0.0]
    }
}
