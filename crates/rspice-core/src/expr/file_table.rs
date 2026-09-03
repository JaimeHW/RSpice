//! Build-time resolution for behavioral lookup functions.

use super::ast::{Expr, Function, LookupInterpolation, LookupTable};
use super::{Context, Vm, compile};
use crate::Value;
use crate::config::ExpressionDialect;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Resolve Xyce inline and file-backed table functions into VM-ready lookup data.
pub fn resolve_file_lookup_functions(
    expr: Expr,
    source_path: Option<&Path>,
) -> Result<Expr, String> {
    resolve_file_lookup_functions_with_limits(
        expr,
        source_path,
        crate::resource::ResourceLimits::default(),
    )
}

/// Resolve lookup functions under an explicit resource policy.
pub fn resolve_file_lookup_functions_with_limits(
    expr: Expr,
    source_path: Option<&Path>,
    resource_limits: crate::resource::ResourceLimits,
) -> Result<Expr, String> {
    resolve_expr(expr, source_path, resource_limits)
}

fn resolve_expr(
    expr: Expr,
    source_path: Option<&Path>,
    resource_limits: crate::resource::ResourceLimits,
) -> Result<Expr, String> {
    match expr {
        Expr::Function { func, args } => {
            let resolved_args = args
                .into_iter()
                .map(|arg| resolve_expr(arg, source_path, resource_limits))
                .collect::<Result<Vec<_>, _>>()?;

            if let Some((transient_breakpoints, interpolation)) = lookup_function_mode(func) {
                if matches!(resolved_args.first(), Some(Expr::StringLiteral(_))) {
                    return resolve_table_file_function(
                        func,
                        resolved_args,
                        source_path,
                        transient_breakpoints,
                        interpolation,
                        resource_limits,
                    );
                }
                if inline_lookup_supported(func) {
                    return resolve_inline_lookup_function(
                        func,
                        resolved_args,
                        transient_breakpoints,
                        interpolation,
                        resource_limits,
                    );
                }
                return Err(format!(
                    "{} requires a file path as its first argument",
                    function_name(func)
                ));
            }

            Ok(Expr::Function {
                func,
                args: resolved_args,
            })
        }
        Expr::Unary { op, operand } => Ok(Expr::Unary {
            op,
            operand: Box::new(resolve_expr(*operand, source_path, resource_limits)?),
        }),
        Expr::Binary { op, left, right } => Ok(Expr::Binary {
            op,
            left: Box::new(resolve_expr(*left, source_path, resource_limits)?),
            right: Box::new(resolve_expr(*right, source_path, resource_limits)?),
        }),
        Expr::Const(_)
        | Expr::NodeVoltage(_)
        | Expr::BranchCurrent(_)
        | Expr::StringLiteral(_)
        | Expr::Time
        | Expr::Frequency
        | Expr::Temperature
        | Expr::ThermalVoltage
        | Expr::Gmin => Ok(expr),
        Expr::LookupTable { input, table } => Ok(Expr::LookupTable {
            input: Box::new(resolve_expr(*input, source_path, resource_limits)?),
            table,
        }),
    }
}

#[derive(Debug, Clone, Copy)]
enum InterpolationKind {
    Linear,
    NaturalCubic,
    Akima,
    Wodicka,
    Barycentric,
}

fn lookup_function_mode(func: Function) -> Option<(bool, InterpolationKind)> {
    match func {
        Function::Table | Function::TableFile => Some((true, InterpolationKind::Linear)),
        Function::FastTable | Function::FastTableFile => Some((false, InterpolationKind::Linear)),
        Function::Cubic | Function::CubicFile => Some((false, InterpolationKind::NaturalCubic)),
        Function::Akima | Function::AkimaFile => Some((false, InterpolationKind::Akima)),
        Function::Wodicka | Function::WodickaFile => Some((false, InterpolationKind::Wodicka)),
        Function::Barycentric | Function::BarycentricFile => {
            Some((false, InterpolationKind::Barycentric))
        }
        _ => None,
    }
}

fn inline_lookup_supported(func: Function) -> bool {
    matches!(func, Function::Table | Function::Akima)
}

fn resolve_table_file_function(
    func: Function,
    args: Vec<Expr>,
    source_path: Option<&Path>,
    transient_breakpoints: bool,
    interpolation: InterpolationKind,
    resource_limits: crate::resource::ResourceLimits,
) -> Result<Expr, String> {
    let Some(Expr::StringLiteral(path)) = args.first() else {
        unreachable!("file lookup resolution requires a string path")
    };
    if args.len() > 3 {
        return Err(format!(
            "{} file lookup accepts a path, optional sample count, and optional log-scale flag; got {} arguments",
            function_name(func),
            args.len()
        ));
    }

    let path = resolve_table_path(path, source_path);
    let mut points = load_lookup_points(&path, resource_limits)?;
    if let Some(sample_count) = args.get(1) {
        let Expr::Const(sample_count) = sample_count else {
            return Err(format!(
                "{} sample count must be constant after parameter expansion",
                function_name(func)
            ));
        };
        let sample_count = exact_sample_count(*sample_count, func)?;
        let log_scale = match args.get(2) {
            None => false,
            Some(Expr::Const(value)) => *value > 0.0,
            Some(_) => {
                return Err(format!(
                    "{} log-scale flag must be constant after parameter expansion",
                    function_name(func)
                ));
            }
        };
        if sample_count > 0 {
            points = gradient_density_downsample(&points, sample_count, log_scale)?;
        }
    }
    let table = build_lookup_table(
        func,
        points,
        transient_breakpoints,
        interpolation,
        resource_limits,
        true,
    )?;
    Ok(Expr::LookupTable {
        input: Box::new(Expr::Time),
        table,
    })
}

fn resolve_inline_lookup_function(
    func: Function,
    mut args: Vec<Expr>,
    transient_breakpoints: bool,
    interpolation: InterpolationKind,
    resource_limits: crate::resource::ResourceLimits,
) -> Result<Expr, String> {
    if args.len() < 3 || args.len().is_multiple_of(2) {
        return Err(format!(
            "{} inline lookup requires an input followed by one or more x/y pairs; got {} arguments",
            function_name(func),
            args.len()
        ));
    }

    let input = args.remove(0);
    let mut points = Vec::with_capacity(args.len() / 2);
    for (pair_index, pair) in args.chunks_exact(2).enumerate() {
        let (Some(x), Some(y)) = (
            constant_lookup_point_value(&pair[0]),
            constant_lookup_point_value(&pair[1]),
        ) else {
            if matches!(func, Function::Table) {
                let mut original_args = Vec::with_capacity(args.len() + 1);
                original_args.push(input);
                original_args.extend(args);
                return Ok(Expr::Function {
                    func,
                    args: original_args,
                });
            }
            return Err(format!(
                "{} inline lookup point {} must be constant after parameter expansion",
                function_name(func),
                pair_index + 1
            ));
        };
        if !x.is_finite() || !y.is_finite() {
            return Err(format!(
                "{} inline lookup point {} must contain finite values, got ({x}, {y})",
                function_name(func),
                pair_index + 1
            ));
        }
        points.push((x, y));
    }

    points.sort_by(|left, right| left.0.total_cmp(&right.0));
    if let Some(duplicate) = points.windows(2).find(|pair| pair[0].0 == pair[1].0) {
        return Err(format!(
            "{} inline lookup requires distinct x values; duplicate x={}",
            function_name(func),
            duplicate[0].0
        ));
    }

    let table = build_lookup_table(
        func,
        points,
        transient_breakpoints,
        interpolation,
        resource_limits,
        false,
    )?;
    Ok(Expr::LookupTable {
        input: Box::new(input),
        table,
    })
}

fn constant_lookup_point_value(expr: &Expr) -> Option<Value> {
    if let Expr::Const(value) = expr {
        return Some(*value);
    }
    if !lookup_point_expression_is_pure(expr) {
        return None;
    }

    let program = compile(expr);
    let evaluate = |dialect| {
        Vm::new().execute(
            &program,
            &Context::dc(&[], &[]).with_expression_dialect(dialect),
        )
    };
    let ngspice = evaluate(ExpressionDialect::Ngspice);
    let xyce = evaluate(ExpressionDialect::Xyce);
    (ngspice.is_finite() && xyce.is_finite() && ngspice.to_bits() == xyce.to_bits()).then_some(xyce)
}

fn lookup_point_expression_is_pure(expr: &Expr) -> bool {
    match expr {
        Expr::Const(_) => true,
        Expr::Unary { operand, .. } => lookup_point_expression_is_pure(operand),
        Expr::Binary { left, right, .. } => {
            lookup_point_expression_is_pure(left) && lookup_point_expression_is_pure(right)
        }
        Expr::Function { func, args } => {
            !matches!(
                func,
                Function::Sdt
                    | Function::SpicePulse
                    | Function::SpiceSin
                    | Function::SpiceExp
                    | Function::SpiceSffm
            ) && args.iter().all(lookup_point_expression_is_pure)
        }
        Expr::LookupTable { input, .. } => lookup_point_expression_is_pure(input),
        Expr::NodeVoltage(_)
        | Expr::BranchCurrent(_)
        | Expr::StringLiteral(_)
        | Expr::Time
        | Expr::Frequency
        | Expr::Temperature
        | Expr::ThermalVoltage
        | Expr::Gmin => false,
    }
}

fn build_lookup_table(
    func: Function,
    points: Vec<(Value, Value)>,
    transient_breakpoints: bool,
    interpolation: InterpolationKind,
    resource_limits: crate::resource::ResourceLimits,
    can_downsample: bool,
) -> Result<LookupTable, String> {
    const MAX_BARYCENTRIC_POINTS: usize = 4_096;
    if matches!(interpolation, InterpolationKind::Barycentric)
        && points.len() > MAX_BARYCENTRIC_POINTS
    {
        let remedy = if can_downsample {
            "provide a sample count to downsample"
        } else {
            "use a file lookup with a sample count to downsample"
        };
        return Err(format!(
            "{} table has {} points; barycentric interpolation is limited to {MAX_BARYCENTRIC_POINTS} points to bound quadratic setup work ({remedy})",
            function_name(func),
            points.len()
        ));
    }
    let retained_values = match interpolation {
        InterpolationKind::Linear => points.len().saturating_mul(2),
        InterpolationKind::NaturalCubic | InterpolationKind::Barycentric => {
            points.len().saturating_mul(3)
        }
        InterpolationKind::Akima | InterpolationKind::Wodicka => points
            .len()
            .saturating_mul(2)
            .saturating_add(points.len().saturating_sub(1).saturating_mul(3)),
    };
    crate::resource::ResourceLimitError::ensure(
        crate::resource::ResourceKind::ExternalDataValues,
        retained_values,
        resource_limits.max_external_data_values,
    )
    .map_err(|error| error.to_string())?;
    let interpolation = match interpolation {
        InterpolationKind::Linear => LookupInterpolation::Linear,
        InterpolationKind::NaturalCubic => LookupInterpolation::NaturalCubic {
            second_derivatives: Arc::from(
                natural_cubic_second_derivatives(&points).into_boxed_slice(),
            ),
        },
        InterpolationKind::Akima => LookupInterpolation::Akima {
            coefficients: Arc::from(akima_coefficients(&points).into_boxed_slice()),
        },
        InterpolationKind::Wodicka => LookupInterpolation::Wodicka {
            coefficients: Arc::from(wodicka_coefficients(&points).into_boxed_slice()),
        },
        InterpolationKind::Barycentric => LookupInterpolation::Barycentric {
            weights: Arc::from(barycentric_weights(&points).into_boxed_slice()),
        },
    };
    Ok(LookupTable {
        points: Arc::from(points.into_boxed_slice()),
        interpolation,
        transient_breakpoints,
    })
}

fn exact_sample_count(value: Value, func: Function) -> Result<usize, String> {
    if !value.is_finite() || value < 0.0 || value.fract() != 0.0 {
        return Err(format!(
            "{} sample count must be a finite non-negative integer, got {value}",
            function_name(func)
        ));
    }
    if value > usize::MAX as Value {
        return Err(format!("{} sample count is too large", function_name(func)));
    }
    let count = value as usize;
    if count == 1 {
        return Err(format!(
            "{} sample count must be zero (disabled) or at least two",
            function_name(func)
        ));
    }
    Ok(count)
}

/// Xyce's gradient-density table reduction (Zhao/Tsiotras mesh density).
///
/// The absolute derivative of an Akima interpolant is itself sampled at the
/// original knots and Akima-integrated into a normalized density CDF. Uniform
/// samples of that CDF are mapped back through another Akima interpolant. The
/// final ordinate scale preserves the original trapezoidal integral.
fn gradient_density_downsample(
    points: &[(Value, Value)],
    sample_count: usize,
    log_scale: bool,
) -> Result<Vec<(Value, Value)>, String> {
    let filtered = points
        .iter()
        .copied()
        .filter(|(_, value)| *value != 0.0)
        .collect::<Vec<_>>();
    if filtered.len() < 2 {
        return Err("gradient-density downsampling needs at least two non-zero samples".into());
    }
    if log_scale && filtered.iter().any(|(_, value)| *value <= 0.0) {
        return Err("log-scale gradient-density downsampling requires positive values".into());
    }

    let density_points = if log_scale {
        filtered
            .iter()
            .map(|(x, y)| (*x, y.log10()))
            .collect::<Vec<_>>()
    } else {
        filtered.clone()
    };
    let source_coefficients = akima_coefficients(&filtered);
    let density_coefficients = akima_coefficients(&density_points);
    let derivative_magnitudes = density_points
        .iter()
        .enumerate()
        .map(|(index, (x, _))| {
            akima_derivative_at(&density_points, &density_coefficients, index, *x).abs()
        })
        .collect::<Vec<_>>();
    let derivative_points = filtered
        .iter()
        .zip(derivative_magnitudes)
        .map(|((x, _), derivative)| (*x, derivative))
        .collect::<Vec<_>>();
    let derivative_coefficients = akima_coefficients(&derivative_points);

    let mut cumulative = Vec::with_capacity(filtered.len());
    cumulative.push((0.0, filtered[0].0));
    let mut total = 0.0;
    for index in 0..filtered.len() - 1 {
        let span = filtered[index + 1].0 - filtered[index].0;
        let [p1, p2, p3] = derivative_coefficients[index];
        total += span
            * (derivative_points[index].1
                + span * (0.5 * p1 + span * (p2 / 3.0 + 0.25 * p3 * span)));
        cumulative.push((total, filtered[index + 1].0));
    }
    if !total.is_finite() || total <= 0.0 {
        return Err("gradient-density downsampling requires a non-constant table".into());
    }
    for (cdf, _) in &mut cumulative {
        *cdf /= total;
    }
    if cumulative.windows(2).any(|pair| pair[1].0 <= pair[0].0) {
        return Err("gradient-density CDF is not strictly increasing".into());
    }
    let inverse_coefficients = akima_coefficients(&cumulative);

    let mut reduced = Vec::with_capacity(sample_count);
    for index in 0..sample_count {
        let fraction = index as Value / (sample_count - 1) as Value;
        let time = akima_value_at(&cumulative, &inverse_coefficients, fraction);
        let value = akima_value_at(&filtered, &source_coefficients, time);
        if time.is_finite() && value.is_finite() {
            reduced.push((time, value));
        }
    }
    if reduced.len() < 2 {
        return Err("gradient-density downsampling produced fewer than two finite samples".into());
    }

    let original_integral = trapezoidal_integral(&filtered);
    let reduced_integral = trapezoidal_integral(&reduced);
    if reduced_integral != 0.0 {
        let scale = original_integral / reduced_integral;
        for (_, value) in &mut reduced {
            *value *= scale;
        }
    }
    Ok(reduced)
}

fn akima_value_at(points: &[(Value, Value)], coefficients: &[[Value; 3]], x: Value) -> Value {
    let index = points
        .partition_point(|point| point.0 <= x)
        .saturating_sub(1)
        .min(points.len() - 2);
    let delta = x - points[index].0;
    let [p1, p2, p3] = coefficients[index];
    points[index].1 + delta * (p1 + delta * (p2 + p3 * delta))
}

fn akima_derivative_at(
    points: &[(Value, Value)],
    coefficients: &[[Value; 3]],
    index: usize,
    x: Value,
) -> Value {
    let segment = index.min(points.len() - 2);
    let delta = x - points[segment].0;
    let [p1, p2, p3] = coefficients[segment];
    p1 + delta * (2.0 * p2 + 3.0 * p3 * delta)
}

fn trapezoidal_integral(points: &[(Value, Value)]) -> Value {
    points
        .windows(2)
        .map(|pair| 0.5 * (pair[0].1 + pair[1].1) * (pair[1].0 - pair[0].0))
        .sum()
}

fn natural_cubic_second_derivatives(points: &[(Value, Value)]) -> Vec<Value> {
    let count = points.len();
    let mut second_derivatives = vec![0.0; count];
    let mut decomposition = vec![0.0; count - 1];

    for index in 1..count - 1 {
        let previous_span = points[index].0 - points[index - 1].0;
        let combined_span = points[index + 1].0 - points[index - 1].0;
        let next_span = points[index + 1].0 - points[index].0;
        let sigma = previous_span / combined_span;
        let pivot = sigma * second_derivatives[index - 1] + 2.0;
        second_derivatives[index] = (sigma - 1.0) / pivot;
        let slope_change = (points[index + 1].1 - points[index].1) / next_span
            - (points[index].1 - points[index - 1].1) / previous_span;
        decomposition[index] =
            (6.0 * slope_change / combined_span - sigma * decomposition[index - 1]) / pivot;
    }

    for index in (0..count - 1).rev() {
        second_derivatives[index] =
            second_derivatives[index] * second_derivatives[index + 1] + decomposition[index];
    }
    second_derivatives
}

fn akima_coefficients(points: &[(Value, Value)]) -> Vec<[Value; 3]> {
    let count = points.len();
    if count < 2 {
        return Vec::new();
    }
    let slopes = extended_akima_slopes(points);
    let mut tangents = vec![0.0; count];
    for index in 0..count {
        let forward_weight = (slopes[index + 3] - slopes[index + 2]).abs();
        let backward_weight = (slopes[index + 1] - slopes[index]).abs();
        let weight = forward_weight + backward_weight;
        tangents[index] = if weight == 0.0 {
            0.5 * (slopes[index + 1] + slopes[index + 2])
        } else {
            (forward_weight * slopes[index + 1] + backward_weight * slopes[index + 2]) / weight
        };
    }

    (0..count - 1)
        .map(|index| {
            hermite_coefficients(
                points[index + 1].0 - points[index].0,
                slopes[index + 2],
                tangents[index],
                tangents[index + 1],
            )
        })
        .collect()
}

fn extended_akima_slopes(points: &[(Value, Value)]) -> Vec<Value> {
    let count = points.len();
    let mut slopes = vec![0.0; count + 3];
    for index in 0..count - 1 {
        slopes[index + 2] =
            (points[index + 1].1 - points[index].1) / (points[index + 1].0 - points[index].0);
    }

    slopes[0] = 3.0 * slopes[2] - 2.0 * slopes[3];
    slopes[1] = 2.0 * slopes[2] - slopes[3];
    slopes[count + 1] = 2.0 * slopes[count] - slopes[count - 1];
    slopes[count + 2] = 3.0 * slopes[count] - 2.0 * slopes[count - 1];
    slopes
}

fn wodicka_coefficients(points: &[(Value, Value)]) -> Vec<[Value; 3]> {
    if points.len() < 2 {
        return Vec::new();
    }
    let slopes = extended_akima_slopes(points);
    (0..points.len() - 1)
        .map(|index| {
            let denominator = (slopes[index + 3] - slopes[index + 2]).abs()
                + (slopes[index + 1] - slopes[index]).abs();
            if denominator == 0.0 {
                return [slopes[index + 2], 0.0, 0.0];
            }

            let next_denominator = (slopes[index + 4] - slopes[index + 3]).abs()
                + (slopes[index + 2] - slopes[index + 1]).abs();
            let alpha = (slopes[index + 1] - slopes[index]).abs() / denominator;
            let left_tangent = (1.0 - alpha) * slopes[index + 1] + alpha * slopes[index + 2];
            let right_tangent = if next_denominator == 0.0 {
                slopes[index + 2]
            } else {
                let next_alpha = (slopes[index + 2] - slopes[index + 1]).abs() / next_denominator;
                (1.0 - next_alpha) * slopes[index + 2] + next_alpha * slopes[index + 3]
            };
            hermite_coefficients(
                points[index + 1].0 - points[index].0,
                slopes[index + 2],
                left_tangent,
                right_tangent,
            )
        })
        .collect()
}

fn hermite_coefficients(
    span: Value,
    secant: Value,
    left_tangent: Value,
    right_tangent: Value,
) -> [Value; 3] {
    [
        left_tangent,
        (3.0 * secant - 2.0 * left_tangent - right_tangent) / span,
        (left_tangent + right_tangent - 2.0 * secant) / span.powi(2),
    ]
}

fn barycentric_weights(points: &[(Value, Value)]) -> Vec<Value> {
    (0..points.len())
        .map(|point_index| {
            let mut denominator = 1.0;
            for other_index in 0..points.len() {
                if other_index != point_index {
                    denominator *= points[point_index].0 - points[other_index].0;
                }
            }
            1.0 / denominator
        })
        .collect()
}

fn resolve_table_path(path: &str, source_path: Option<&Path>) -> PathBuf {
    let raw = Path::new(path);
    if raw.is_absolute() {
        return raw.to_path_buf();
    }
    source_path
        .and_then(Path::parent)
        .unwrap_or_else(|| Path::new("."))
        .join(raw)
}

fn load_lookup_points(
    path: &Path,
    resource_limits: crate::resource::ResourceLimits,
) -> Result<Vec<(Value, Value)>, String> {
    let content = crate::resource::read_utf8_file_limited(
        path,
        crate::resource::ResourceKind::ExternalDataBytes,
        resource_limits.max_external_data_bytes,
    )
    .map_err(|err| {
        format!(
            "failed to read behavioral table file '{}': {err}",
            path.display()
        )
    })?;
    let mut points = Vec::new();
    for (line_index, line) in content.lines().enumerate() {
        let line = strip_lookup_comment(line).trim();
        if line.is_empty() || line.starts_with('*') || line.starts_with('#') {
            continue;
        }
        let mut fields = line.split_whitespace();
        let x_text = fields.next().ok_or_else(|| {
            format!(
                "table file '{}' row {} has no x value",
                path.display(),
                line_index + 1
            )
        })?;
        let y_text = fields.next().ok_or_else(|| {
            format!(
                "table file '{}' row {} has no y value",
                path.display(),
                line_index + 1
            )
        })?;
        let x = parse_table_value(path, line_index + 1, "x", x_text)?;
        let y = parse_table_value(path, line_index + 1, "y", y_text)?;
        crate::resource::ResourceLimitError::ensure(
            crate::resource::ResourceKind::ExternalDataValues,
            points.len().saturating_add(1).saturating_mul(2),
            resource_limits.max_external_data_values,
        )
        .map_err(|error| error.to_string())?;
        points.try_reserve(1).map_err(|error| {
            format!(
                "unable to reserve behavioral table row {} from '{}': {error}",
                line_index + 1,
                path.display()
            )
        })?;
        points.push((x, y));
    }

    if points.len() < 2 {
        return Err(format!(
            "behavioral table file '{}' must contain at least two numeric rows",
            path.display()
        ));
    }
    points.sort_by(|left, right| left.0.total_cmp(&right.0));
    points.dedup_by(|left, right| {
        if left.0 == right.0 {
            right.1 = left.1;
            true
        } else {
            false
        }
    });
    if points.len() < 2 {
        return Err(format!(
            "behavioral table file '{}' must contain at least two distinct x values",
            path.display()
        ));
    }
    Ok(points)
}

fn strip_lookup_comment(line: &str) -> &str {
    let comment_start = line
        .char_indices()
        .filter_map(|(index, ch)| matches!(ch, ';' | '#').then_some(index))
        .min();
    match comment_start {
        Some(index) => &line[..index],
        None => line,
    }
}

fn parse_table_value(path: &Path, line: usize, label: &str, text: &str) -> Result<Value, String> {
    let value = text.parse::<Value>().map_err(|err| {
        format!(
            "invalid {label} value '{text}' in behavioral table file '{}' row {line}: {err}",
            path.display()
        )
    })?;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(format!(
            "non-finite {label} value '{text}' in behavioral table file '{}' row {line}",
            path.display()
        ))
    }
}

fn function_name(func: Function) -> &'static str {
    match func {
        Function::Table => "table",
        Function::TableFile => "tablefile",
        Function::FastTable => "fasttable",
        Function::FastTableFile => "fasttablefile",
        Function::Cubic => "cubic",
        Function::CubicFile => "cubicfile",
        Function::Akima => "akima",
        Function::AkimaFile => "akimafile",
        Function::Wodicka => "wodicka",
        Function::WodickaFile => "wodickafile",
        Function::Barycentric => "bli",
        Function::BarycentricFile => "blifile",
        _ => "function",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Context, Vm, compile, parse_expression_strict};
    use std::time::{SystemTime, UNIX_EPOCH};

    /// The closed form an inline lookup expression must reproduce, evaluated
    /// at the expression's input.
    type InlineLookupOracle = fn(Value) -> Value;

    #[test]
    fn file_table_resolution_enforces_runtime_data_limits() {
        let dir = unique_temp_dir("file-table-resource-limits");
        std::fs::create_dir_all(&dir).expect("create temp table directory");
        let contents = "0 0\n1 1\n";
        std::fs::write(dir.join("wave.dat"), contents).expect("write table data");
        let deck_path = dir.join("deck.cir");
        let expression =
            || parse_expression_strict("tablefile(\"wave.dat\")").expect("table expression parses");

        let byte_limits = crate::resource::ResourceLimits {
            max_external_data_bytes: contents.len() - 1,
            ..Default::default()
        };
        let byte_error =
            resolve_file_lookup_functions_with_limits(expression(), Some(&deck_path), byte_limits)
                .expect_err("oversized table file must fail");
        assert!(byte_error.contains("external_data_bytes limit exceeded"));

        let value_limits = crate::resource::ResourceLimits {
            max_external_data_values: 3,
            ..Default::default()
        };
        let value_error =
            resolve_file_lookup_functions_with_limits(expression(), Some(&deck_path), value_limits)
                .expect_err("oversized table value set must fail");
        assert!(value_error.contains("external_data_values limit exceeded"));

        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn barycentric_file_tables_bound_quadratic_setup_work() {
        let dir = unique_temp_dir("barycentric-table-work-limit");
        std::fs::create_dir_all(&dir).expect("create temp table directory");
        let mut contents = String::new();
        for index in 0..=4_096 {
            use std::fmt::Write as _;
            writeln!(&mut contents, "{index} {index}").expect("write table row");
        }
        std::fs::write(dir.join("wave.dat"), contents).expect("write table data");
        let deck_path = dir.join("deck.cir");
        let expression =
            parse_expression_strict("blifile(\"wave.dat\")").expect("table expression parses");

        let error = resolve_file_lookup_functions(expression, Some(&deck_path))
            .expect_err("oversized barycentric setup must fail");
        assert!(error.contains("barycentric interpolation is limited to 4096 points"));

        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn file_table_resolution_accepts_xyce_comment_styles() {
        let dir = unique_temp_dir("xyce-file-table-comments");
        std::fs::create_dir_all(&dir).expect("create temp table directory");
        let table_path = dir.join("wave.dat");
        std::fs::write(
            &table_path,
            "* full-line star comment\n\
             0 1\n\
             1 3 ; semicolon comment\n\
             ; full-line semicolon comment\n\
             2 5 # hash comment\n",
        )
        .expect("write table data");
        let deck_path = dir.join("deck.cir");

        let ast = parse_expression_strict("table(\"wave.dat\")").expect("table expression parses");
        let resolved =
            resolve_file_lookup_functions(ast, Some(&deck_path)).expect("table file resolves");
        let program = compile(&resolved);
        let mut vm = Vm::new();

        assert_eq!(
            vm.execute(&program, &Context::transient(&[], &[], 0.0)),
            1.0
        );
        assert_eq!(
            vm.execute(&program, &Context::transient(&[], &[], 1.5)),
            4.0
        );
    }

    #[test]
    fn natural_cubic_matches_xyce_spline_and_clamps_endpoints() {
        let dir = unique_temp_dir("xyce-natural-cubic");
        std::fs::create_dir_all(&dir).expect("create temp table directory");
        std::fs::write(dir.join("curve.dat"), "0 0\n1 1\n2 0\n").expect("write spline data");
        let deck_path = dir.join("deck.cir");

        for function in ["cubic", "cubicfile"] {
            let ast = parse_expression_strict(&format!("{function}(\"curve.dat\")"))
                .expect("cubic expression parses");
            let resolved =
                resolve_file_lookup_functions(ast, Some(&deck_path)).expect("cubic file resolves");
            let Expr::LookupTable { input, table } = &resolved else {
                panic!("cubic file should resolve into lookup data");
            };
            assert!(matches!(input.as_ref(), Expr::Time));
            assert!(!table.transient_breakpoints);
            assert!(matches!(
                table.interpolation,
                LookupInterpolation::NaturalCubic { .. }
            ));

            let program = compile(&resolved);
            let mut vm = Vm::new();
            for (time, expected) in [
                (-1.0, 0.0),
                (0.0, 0.0),
                (0.5, 0.6875),
                (1.0, 1.0),
                (1.5, 0.6875),
                (2.0, 0.0),
                (3.0, 0.0),
            ] {
                let actual = vm.execute(&program, &Context::transient(&[], &[], time));
                assert!((actual - expected).abs() < 1.0e-14, "time={time}");
            }
        }
    }

    #[test]
    fn natural_cubic_requires_a_path_and_at_most_two_downsampling_arguments() {
        let too_few = parse_expression_strict("cubic()")
            .expect_err("cubic without a path must fail")
            .to_string();
        assert!(too_few.contains("at least 1 argument"), "{too_few}");

        let too_many = parse_expression_strict("cubic(\"curve.dat\", 4, 0, 1)")
            .expect_err("unsupported resampling arguments must fail")
            .to_string();
        assert!(too_many.contains("at most 3 arguments"), "{too_many}");
    }

    #[test]
    fn file_tables_use_xyce_gradient_density_downsampling_and_preserve_area() {
        let dir = unique_temp_dir("xyce-gradient-density");
        std::fs::create_dir_all(&dir).expect("create temp table directory");
        std::fs::write(
            dir.join("pulse.dat"),
            "0 0.1\n1 0.1\n2 0.2\n3 4\n4 0.2\n5 0.1\n6 0.1\n",
        )
        .expect("write table data");
        let deck_path = dir.join("deck.cir");

        let ast =
            parse_expression_strict("table(\"pulse.dat\", 5)").expect("downsampled table parses");
        let resolved = resolve_file_lookup_functions(ast, Some(&deck_path))
            .expect("downsampled table resolves");
        let Expr::LookupTable { input, table } = resolved else {
            panic!("file table should resolve into lookup data");
        };
        assert!(matches!(input.as_ref(), Expr::Time));
        assert_eq!(table.points.len(), 5);
        assert!((table.points[0].0 - 0.0).abs() < 1.0e-14);
        assert!((table.points[4].0 - 6.0).abs() < 1.0e-12);
        assert!(
            table.points[1].0 > 1.5 && table.points[1].0 < 3.0,
            "{:?}",
            table.points
        );
        assert!(
            table.points[3].0 > 3.0 && table.points[3].0 < 4.5,
            "{:?}",
            table.points
        );
        let original = vec![
            (0.0, 0.1),
            (1.0, 0.1),
            (2.0, 0.2),
            (3.0, 4.0),
            (4.0, 0.2),
            (5.0, 0.1),
            (6.0, 0.1),
        ];
        assert!(
            (trapezoidal_integral(&table.points) - trapezoidal_integral(&original)).abs() < 1.0e-12
        );
    }

    #[test]
    fn log_scale_changes_density_domain_and_validates_positive_ordinates() {
        let points = vec![(0.0, 0.01), (1.0, 0.1), (2.0, 10.0), (3.0, 100.0)];
        let linear = gradient_density_downsample(&points, 4, false).expect("linear density");
        let logarithmic =
            gradient_density_downsample(&points, 4, true).expect("logarithmic density");
        assert_ne!(linear, logarithmic);

        let error = gradient_density_downsample(&[(0.0, -1.0), (1.0, 2.0)], 3, true)
            .expect_err("logarithmic density rejects negative values");
        assert!(error.contains("requires positive values"), "{error}");
    }

    #[test]
    fn akima_matches_original_local_slope_construction() {
        let dir = unique_temp_dir("xyce-akima");
        std::fs::create_dir_all(&dir).expect("create temp table directory");
        std::fs::write(dir.join("curve.dat"), "0 0\n1 1\n2 0\n3 1\n").expect("write spline data");
        let deck_path = dir.join("deck.cir");

        for function in ["akima", "akimafile", "spline", "splinefile"] {
            let ast = parse_expression_strict(&format!("{function}(\"curve.dat\")"))
                .expect("Akima expression parses");
            let resolved =
                resolve_file_lookup_functions(ast, Some(&deck_path)).expect("Akima file resolves");
            let Expr::LookupTable { input, table } = &resolved else {
                panic!("Akima file should resolve into lookup data");
            };
            assert!(matches!(input.as_ref(), Expr::Time));
            assert!(!table.transient_breakpoints);
            assert!(matches!(
                table.interpolation,
                LookupInterpolation::Akima { .. }
            ));

            let program = compile(&resolved);
            let mut vm = Vm::new();
            for (time, expected) in [
                (-1.0, 0.0),
                (0.0, 0.0),
                (0.5, 0.75),
                (1.5, 0.5),
                (2.5, 0.25),
                (3.0, 1.0),
                (4.0, 1.0),
            ] {
                let actual = vm.execute(&program, &Context::transient(&[], &[], time));
                assert!((actual - expected).abs() < 1.0e-14, "time={time}");
            }
        }

        std::fs::write(dir.join("two-points.dat"), "0 0\n1 1\n")
            .expect("write two-point spline data");
        let ast = parse_expression_strict("akima(\"two-points.dat\")")
            .expect("two-point Akima expression parses");
        let resolved = resolve_file_lookup_functions(ast, Some(&deck_path))
            .expect("two-point Akima file resolves");
        let program = compile(&resolved);
        let mut vm = Vm::new();
        let midpoint = vm.execute(&program, &Context::transient(&[], &[], 0.5));
        assert!((midpoint - 0.625).abs() < 1.0e-14);
    }

    #[test]
    fn constant_inline_lookups_sort_xy_pairs_and_use_the_explicit_input() {
        let cases: [(&str, InlineLookupOracle); 3] = [
            ("table(v(a),1,3,0.5,2,0,1)", |x: Value| 1.0 + 2.0 * x),
            ("akima(v(a),1,4,0.5,2.25,0,1)", |x: Value| (1.0 + x).powi(2)),
            ("spline(v(a),1,4,0.5,2.25,0,1)", |x: Value| {
                (1.0 + x).powi(2)
            }),
        ];
        for (expression, expected_at) in cases {
            let ast = parse_expression_strict(expression)
                .unwrap_or_else(|error| panic!("inline lookup should parse: {error}"));
            let resolved = resolve_file_lookup_functions(ast, None)
                .unwrap_or_else(|error| panic!("inline lookup should resolve: {error}"));
            let Expr::LookupTable { input, table } = &resolved else {
                panic!("constant inline lookup should lower to precomputed data");
            };
            assert!(
                matches!(input.as_ref(), Expr::NodeVoltage(node) if node.eq_ignore_ascii_case("A"))
            );
            assert_eq!(
                table.points.iter().map(|point| point.0).collect::<Vec<_>>(),
                vec![0.0, 0.5, 1.0]
            );

            let program = compile(&resolved);
            let mut vm = Vm::new();
            for x in [0.0, 0.1, 0.5, 0.9, 1.0] {
                let actual = vm.execute(&program, &Context::dc(&[x], &[]));
                let expected = expected_at(x);
                assert!(
                    (actual - expected).abs() < 2.0e-14,
                    "{expression} at x={x}: expected {expected}, got {actual}"
                );
            }
        }
    }

    #[test]
    fn inline_lookup_points_accept_dialect_invariant_constant_expressions() {
        let cases: [(&str, InlineLookupOracle); 2] = [
            ("table(v(a),1,3,-(1),-1,1-1,1)", |x: Value| 1.0 + 2.0 * x),
            ("akima(v(a),1,4,-(1),0,2-2,1)", |x: Value| (1.0 + x).powi(2)),
        ];
        for (expression, expected_at) in cases {
            let ast = parse_expression_strict(expression)
                .unwrap_or_else(|error| panic!("constant expression should parse: {error}"));
            let resolved = resolve_file_lookup_functions(ast, None)
                .unwrap_or_else(|error| panic!("constant points should resolve: {error}"));
            let Expr::LookupTable { table, .. } = &resolved else {
                panic!("constant point expressions should lower to precomputed data");
            };
            assert_eq!(
                table.points.as_ref(),
                [
                    (-1.0, expected_at(-1.0)),
                    (0.0, expected_at(0.0)),
                    (1.0, expected_at(1.0))
                ]
            );

            let program = compile(&resolved);
            for x in [-1.0, -0.5, 0.25, 1.0] {
                let actual = Vm::new().execute(&program, &Context::dc(&[x], &[]));
                let expected = expected_at(x);
                assert!(
                    (actual - expected).abs() < 2.0e-14,
                    "{expression} at x={x}: expected {expected}, got {actual}"
                );
            }
        }
    }

    #[test]
    fn xyce_lookup_values_clamp_while_exterior_spline_derivatives_extrapolate() {
        let points = vec![(0.0, 1.0), (0.5, 2.25), (1.0, 4.0)];
        let cases = [
            (
                Function::Table,
                InterpolationKind::Linear,
                [0.0, 0.0, 0.0, 0.0],
                [false; 4],
            ),
            (
                Function::Cubic,
                InterpolationKind::NaturalCubic,
                [2.4375, 2.25, 3.75, 3.5625],
                [false; 4],
            ),
            (
                Function::Akima,
                InterpolationKind::Akima,
                [1.5, 2.0, 4.0, 4.5],
                [false; 4],
            ),
            (
                Function::Wodicka,
                InterpolationKind::Wodicka,
                [1.5, 2.0, 4.0, 4.5],
                [false; 4],
            ),
            (
                Function::Barycentric,
                InterpolationKind::Barycentric,
                [1.5, Value::NAN, Value::NAN, 4.5],
                [false, true, true, false],
            ),
        ];
        for (function, interpolation, expected_derivatives, derivative_is_nan) in cases {
            let table = build_lookup_table(
                function,
                points.clone(),
                false,
                interpolation,
                crate::resource::ResourceLimits::default(),
                false,
            )
            .expect("qualified lookup table builds");
            for (index, (input, expected_value)) in
                [(-0.25_f64, 1.0_f64), (0.0, 1.0), (1.0, 4.0), (1.25, 4.0)]
                    .into_iter()
                    .enumerate()
            {
                let (value, derivative) = crate::expr::lookup_table_interpolate_with_derivative(
                    input,
                    &table,
                    ExpressionDialect::Xyce,
                );
                assert_eq!(value.to_bits(), expected_value.to_bits());
                if derivative_is_nan[index] {
                    assert!(derivative.is_nan(), "{function:?} at x={input}");
                } else {
                    assert!(
                        (derivative - expected_derivatives[index]).abs() < 2.0e-14,
                        "{function:?} at x={input}: expected derivative {}, got {derivative}",
                        expected_derivatives[index]
                    );
                }
            }
        }
    }

    #[test]
    fn xyce_two_point_table_derivative_preserves_the_ordinate_bound_defect() {
        let table = build_lookup_table(
            Function::Table,
            vec![(10.0, 0.0), (20.0, 100.0)],
            true,
            InterpolationKind::Linear,
            crate::resource::ResourceLimits::default(),
            false,
        )
        .expect("two-point table builds");
        for (input, expected_value) in [(5.0_f64, 0.0_f64), (30.0, 100.0)] {
            let (value, derivative) = crate::expr::lookup_table_interpolate_with_derivative(
                input,
                &table,
                ExpressionDialect::Xyce,
            );
            assert_eq!(value.to_bits(), expected_value.to_bits());
            assert_eq!(derivative.to_bits(), 10.0f64.to_bits());
        }
    }

    #[test]
    fn inline_lookup_resolution_fails_closed_on_invalid_precomputed_points() {
        let duplicate = parse_expression_strict("akima(v(a),0,1,0,2)")
            .expect("duplicate-point expression parses");
        let error = resolve_file_lookup_functions(duplicate, None)
            .expect_err("duplicate abscissas must fail")
            .to_string();
        assert!(error.contains("distinct x values"), "{error}");

        let runtime_knot = parse_expression_strict("akima(v(a),v(knot),1,1,2)")
            .expect("runtime-knot expression parses");
        let error = resolve_file_lookup_functions(runtime_knot, None)
            .expect_err("runtime spline knots must fail")
            .to_string();
        assert!(
            error.contains("constant after parameter expansion"),
            "{error}"
        );

        let expression = parse_expression_strict("akima(v(a),0,1,0.5,2.25,1,4)")
            .expect("resource-limited expression parses");
        let limits = crate::resource::ResourceLimits {
            max_external_data_values: 11,
            ..Default::default()
        };
        let error = resolve_file_lookup_functions_with_limits(expression, None, limits)
            .expect_err("precomputed coefficients must honor resource limits")
            .to_string();
        assert!(
            error.contains("external_data_values limit exceeded"),
            "{error}"
        );
    }

    #[test]
    fn wodicka_uses_its_distinct_rounded_corner_construction() {
        let dir = unique_temp_dir("xyce-wodicka");
        std::fs::create_dir_all(&dir).expect("create temp table directory");
        std::fs::write(dir.join("curve.dat"), "0 0\n1 0\n2 0\n3 2\n4 4\n5 4\n")
            .expect("write spline data");
        let deck_path = dir.join("deck.cir");

        for function in ["wodicka", "wodickafile"] {
            let ast = parse_expression_strict(&format!("{function}(\"curve.dat\")"))
                .expect("Wodicka expression parses");
            let resolved = resolve_file_lookup_functions(ast, Some(&deck_path))
                .expect("Wodicka file resolves");
            let Expr::LookupTable { input, table } = &resolved else {
                panic!("Wodicka file should resolve into lookup data");
            };
            assert!(matches!(input.as_ref(), Expr::Time));
            assert!(!table.transient_breakpoints);
            assert!(matches!(
                table.interpolation,
                LookupInterpolation::Wodicka { .. }
            ));

            let program = compile(&resolved);
            let mut vm = Vm::new();
            for (time, expected) in [
                (-1.0, 0.0),
                (0.5, 0.0),
                (1.5, 0.0),
                (2.5, 1.0),
                (3.5, 3.0),
                (4.5, 4.375),
                (6.0, 4.0),
            ] {
                let actual = vm.execute(&program, &Context::transient(&[], &[], time));
                assert!((actual - expected).abs() < 1.0e-14, "time={time}");
            }
        }

        let akima = parse_expression_strict("akima(\"curve.dat\")")
            .expect("Akima comparison expression parses");
        let akima = resolve_file_lookup_functions(akima, Some(&deck_path))
            .expect("Akima comparison resolves");
        let mut vm = Vm::new();
        let midpoint = vm.execute(&compile(&akima), &Context::transient(&[], &[], 2.5));
        assert!((midpoint - 0.875).abs() < 1.0e-14);
    }

    #[test]
    fn barycentric_uses_xyce_first_form_and_exact_knots() {
        let dir = unique_temp_dir("xyce-barycentric");
        std::fs::create_dir_all(&dir).expect("create temp table directory");
        std::fs::write(dir.join("curve.dat"), "-1 3\n0 4\n1 9\n")
            .expect("write interpolation data");
        let deck_path = dir.join("deck.cir");

        for function in ["bli", "blifile"] {
            let ast = parse_expression_strict(&format!("{function}(\"curve.dat\")"))
                .expect("barycentric expression parses");
            let resolved = resolve_file_lookup_functions(ast, Some(&deck_path))
                .expect("barycentric file resolves");
            let Expr::LookupTable { input, table } = &resolved else {
                panic!("barycentric file should resolve into lookup data");
            };
            assert!(matches!(input.as_ref(), Expr::Time));
            assert!(!table.transient_breakpoints);
            let LookupInterpolation::Barycentric { weights } = &table.interpolation else {
                panic!("BLI should use barycentric interpolation");
            };
            assert_eq!(weights.as_ref(), &[0.5, -1.0, 0.5]);

            let program = compile(&resolved);
            let mut vm = Vm::new();
            for (time, expected) in [
                (-2.0, 3.0),
                (-1.0, 3.0),
                (-0.5, 3.0),
                (0.0, 4.0),
                (0.25, 4.875),
                (0.5, 6.0),
                (1.0, 9.0),
                (2.0, 9.0),
            ] {
                let actual = vm.execute(&program, &Context::transient(&[], &[], time));
                assert!((actual - expected).abs() < 1.0e-14, "time={time}");
            }
        }
    }

    #[test]
    fn barycentric_preserves_xyce_first_form_numerical_order() {
        let points = (0..40)
            .map(|index| {
                let ordinate = if index % 2 == 0 { 1.0 } else { -1.0 };
                (index as f64, ordinate)
            })
            .collect::<Vec<_>>();
        let table = build_lookup_table(
            Function::Barycentric,
            points,
            false,
            InterpolationKind::Barycentric,
            crate::resource::ResourceLimits::default(),
            false,
        )
        .expect("barycentric table builds");

        let (value, _) = crate::expr::lookup_table_interpolate_with_derivative(
            38.75,
            &table,
            ExpressionDialect::Xyce,
        );
        assert_eq!(value.to_bits(), 0x41e1_d375_9cf0_bf9f);
    }

    fn unique_temp_dir(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("rspice-{label}-{unique}"))
    }
}
