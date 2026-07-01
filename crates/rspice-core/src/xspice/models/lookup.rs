//! XSPICE lookup-table code models.
//!
//! Implements the official `pwl` and `pwlts` analog lookup blocks.

use crate::Value;
use crate::xspice::{
    CmContext, CmError, CmResult, CodeModel, ParamSpec, PortDirection, PortSpec, PortType,
};
use std::sync::{Arc, OnceLock};

const INPUT_DOMAIN_MIN: Value = 1.0e-12;
const INPUT_DOMAIN_MAX: Value = 0.5;
const LOOKUP_TABLE_RESOURCE: &str = "xspice.lookup.table";

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

#[derive(Debug, Clone, PartialEq)]
struct LookupTableSignature {
    x_values: Vec<Value>,
    y_values: Vec<Value>,
}

#[derive(Debug, Clone)]
struct LookupTableResource {
    signature: LookupTableSignature,
    points: CmResult<Arc<Vec<LookupPoint>>>,
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
            PortType::DifferentialCurrent,
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
            PortType::DifferentialCurrent,
        ],
        is_vector: false,
        null_allowed: false,
        vector_min_len: None,
        vector_max_len: None,
        description: "Lookup output".to_string(),
    }
}

fn lookup_table_signature(ctx: &CmContext) -> LookupTableSignature {
    LookupTableSignature {
        x_values: ctx.real_vector_param("x_array").unwrap_or(&[]).to_vec(),
        y_values: ctx.real_vector_param("y_array").unwrap_or(&[]).to_vec(),
    }
}

fn lookup_table_signature_matches(ctx: &CmContext, signature: &LookupTableSignature) -> bool {
    ctx.real_vector_param("x_array").unwrap_or(&[]) == signature.x_values.as_slice()
        && ctx.real_vector_param("y_array").unwrap_or(&[]) == signature.y_values.as_slice()
}

fn lookup_table_uncached(ctx: &CmContext) -> CmResult<Vec<LookupPoint>> {
    let x_values = ctx
        .real_vector_param("x_array")
        .ok_or_else(|| missing_param("x_array"))?;
    let y_values = ctx
        .real_vector_param("y_array")
        .ok_or_else(|| missing_param("y_array"))?;
    validate_lookup_table(x_values, y_values)
}

fn cache_lookup_table(ctx: &mut CmContext) {
    if let Some(resource) = ctx.resource::<LookupTableResource>(LOOKUP_TABLE_RESOURCE)
        && lookup_table_signature_matches(ctx, &resource.signature)
    {
        return;
    }

    let signature = lookup_table_signature(ctx);
    let points = lookup_table_uncached(ctx).map(Arc::new);
    ctx.set_resource(
        LOOKUP_TABLE_RESOURCE,
        Arc::new(LookupTableResource { signature, points }),
    );
}

fn lookup_table(ctx: &CmContext) -> CmResult<Arc<Vec<LookupPoint>>> {
    if let Some(resource) = ctx.resource::<LookupTableResource>(LOOKUP_TABLE_RESOURCE)
        && lookup_table_signature_matches(ctx, &resource.signature)
    {
        return resource.points.clone();
    }
    lookup_table_uncached(ctx).map(Arc::new)
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
    lower: LookupPoint,
    center: LookupPoint,
    upper: LookupPoint,
    input_domain: Value,
    fraction: bool,
) -> Value {
    if !fraction {
        return input_domain;
    }
    let lower_width = center.x - lower.x;
    let upper_width = upper.x - center.x;
    input_domain * lower_width.min(upper_width)
}

fn expanded_lookup_point(points: &[LookupPoint], index: usize, limit: bool) -> LookupPoint {
    let original_len = points.len();
    match index {
        0 => {
            let first = points[0];
            let second = points[1];
            LookupPoint {
                x: 2.0 * first.x - second.x,
                y: if limit {
                    first.y
                } else {
                    2.0 * first.y - second.y
                },
            }
        }
        index if index == original_len + 1 => {
            let last = points[original_len - 1];
            let previous = points[original_len - 2];
            LookupPoint {
                x: 2.0 * last.x - previous.x,
                y: if limit {
                    last.y
                } else {
                    2.0 * last.y - previous.y
                },
            }
        }
        _ => points[index - 1],
    }
}

fn lookup_midpoint(left: LookupPoint, right: LookupPoint) -> Value {
    (left.x + right.x) / 2.0
}

fn lookup_breakpoint_times(
    points: &[LookupPoint],
    input_domain: Value,
    fraction: bool,
) -> Vec<Value> {
    let expanded_len = points.len() + 2;
    let mut breakpoints = Vec::with_capacity(points.len() * 3);
    for index in 1..expanded_len - 1 {
        let lower = expanded_lookup_point(points, index - 1, false);
        let center = expanded_lookup_point(points, index, false);
        let upper = expanded_lookup_point(points, index + 1, false);
        let domain = breakpoint_domain(lower, center, upper, input_domain, fraction);
        if domain > 0.0 && center.x - domain > 0.0 {
            breakpoints.push(center.x - domain);
            breakpoints.push(center.x);
            breakpoints.push(center.x + domain);
        }
    }
    breakpoints
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
    let expanded_len = points.len() + 2;
    let first_bound = expanded_lookup_point(points, 0, limit);
    let first = expanded_lookup_point(points, 1, limit);
    let last = expanded_lookup_point(points, expanded_len - 2, limit);
    let last_bound = expanded_lookup_point(points, expanded_len - 1, limit);

    if x <= lookup_midpoint(first_bound, first) {
        return linear_value(first_bound, segment_slope(first_bound, first), x);
    }
    if x >= lookup_midpoint(last, last_bound) {
        return linear_value(last, segment_slope(last, last_bound), x);
    }

    for index in 1..expanded_len - 1 {
        let center = expanded_lookup_point(points, index, limit);
        let upper = expanded_lookup_point(points, index + 1, limit);
        if x < lookup_midpoint(center, upper) {
            let lower = expanded_lookup_point(points, index - 1, limit);
            let left_slope = segment_slope(lower, center);
            let right_slope = segment_slope(center, upper);
            let domain = breakpoint_domain(lower, center, upper, input_domain, fraction);
            if domain > 0.0 && x >= center.x - domain && x < center.x + domain {
                return smooth_corner(x, center.x, center.y, left_slope, right_slope, domain);
            }
            let slope = if x < center.x - domain {
                left_slope
            } else {
                right_slope
            };
            return linear_value(center, slope, x);
        }
    }

    linear_value(last, segment_slope(last, last_bound), x)
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
        cache_lookup_table(ctx);
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
        cache_lookup_table(ctx);
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

    fn transient_breakpoints(&self, ctx: &CmContext) -> CmResult<Vec<Value>> {
        let points = lookup_table(ctx)?;
        let input_domain = effective_input_domain(ctx.param("input_domain"))?;
        let fraction = ctx.param("fraction") > 0.5;
        validate_smoothing_domain(&points, input_domain, fraction)?;
        Ok(lookup_breakpoint_times(&points, input_domain, fraction))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_near(actual: Value, expected: Value) {
        assert!(
            (actual - expected).abs() <= 1.0e-12,
            "expected {expected:e}, got {actual:e}"
        );
    }

    #[test]
    fn lookup_table_cache_reloads_when_params_change() {
        let mut ctx = CmContext::new();
        ctx.set_real_vector_param("x_array", vec![0.0, 1.0]);
        ctx.set_real_vector_param("y_array", vec![0.0, 10.0]);
        cache_lookup_table(&mut ctx);

        let before = evaluate_lookup_context(&ctx, 0.5).expect("initial table");
        assert_near(before.value, 5.0);
        assert_near(before.slope, 10.0);

        ctx.set_real_vector_param("y_array", vec![0.0, 20.0]);
        let after = evaluate_lookup_context(&ctx, 0.5).expect("updated table");
        assert_near(after.value, 10.0);
        assert_near(after.slope, 20.0);
    }

    #[test]
    fn lookup_evaluation_preserves_first_overlapping_breakpoint() {
        let points = validate_lookup_table(&[0.0, 0.5, 1.0, 1.5], &[0.0, 10.0, 30.0, 60.0])
            .expect("strictly increasing lookup table");
        let x = 0.7;

        let result = evaluate_lookup(&points, x, 0.4, false, false);
        let first_breakpoint_result = smooth_corner(
            x,
            points[1].x,
            points[1].y,
            segment_slope(points[0], points[1]),
            segment_slope(points[1], points[2]),
            0.4,
        );
        let second_breakpoint_result = smooth_corner(
            x,
            points[2].x,
            points[2].y,
            segment_slope(points[1], points[2]),
            segment_slope(points[2], points[3]),
            0.4,
        );

        assert_near(result.value, first_breakpoint_result.value);
        assert_near(result.slope, first_breakpoint_result.slope);
        assert!(
            (result.value - second_breakpoint_result.value).abs() > 1.0e-6,
            "test fixture should distinguish the overlapping breakpoint choices"
        );
    }

    #[test]
    fn lookup_limit_smooths_synthetic_edge_clamps() {
        let points = validate_lookup_table(&[0.0, 1.0, 2.0], &[0.0, 10.0, 30.0])
            .expect("strictly increasing lookup table");

        let lower = evaluate_lookup(&points, -0.005, 0.01, false, true);
        assert_near(lower.value, 0.00625);
        assert_near(lower.slope, 2.5);

        let upper = evaluate_lookup(&points, 2.005, 0.01, false, true);
        assert_near(upper.value, 29.9875);
        assert_near(upper.slope, 5.0);
    }

    #[test]
    fn pwlts_breakpoints_cover_smoothed_table_corners() {
        let points = validate_lookup_table(&[0.0, 1.0e-9, 2.0e-9], &[0.0, 10.0, 30.0])
            .expect("strictly increasing lookup table");

        let breakpoints = lookup_breakpoint_times(&points, 0.01, true);
        let expected = [0.99e-9, 1.0e-9, 1.01e-9, 1.99e-9, 2.0e-9, 2.01e-9];
        assert_eq!(breakpoints.len(), expected.len());
        for (actual, expected) in breakpoints.into_iter().zip(expected) {
            assert_near(actual, expected);
        }
    }
}
