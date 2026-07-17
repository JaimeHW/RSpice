//! Build-time resolution for file-backed behavioral lookup functions.

use super::ast::{Expr, Function, LookupInterpolation, LookupTable};
use crate::Value;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Resolve Xyce file-backed table functions into VM-ready lookup data.
pub fn resolve_file_lookup_functions(
    expr: Expr,
    source_path: Option<&Path>,
) -> Result<Expr, String> {
    resolve_expr(expr, source_path)
}

fn resolve_expr(expr: Expr, source_path: Option<&Path>) -> Result<Expr, String> {
    match expr {
        Expr::Function { func, args } => {
            let resolved_args = args
                .into_iter()
                .map(|arg| resolve_expr(arg, source_path))
                .collect::<Result<Vec<_>, _>>()?;

            if let Some((transient_breakpoints, interpolation)) = table_file_mode(func) {
                return resolve_table_file_function(
                    func,
                    resolved_args,
                    source_path,
                    transient_breakpoints,
                    interpolation,
                );
            }

            Ok(Expr::Function {
                func,
                args: resolved_args,
            })
        }
        Expr::Unary { op, operand } => Ok(Expr::Unary {
            op,
            operand: Box::new(resolve_expr(*operand, source_path)?),
        }),
        Expr::Binary { op, left, right } => Ok(Expr::Binary {
            op,
            left: Box::new(resolve_expr(*left, source_path)?),
            right: Box::new(resolve_expr(*right, source_path)?),
        }),
        Expr::Const(_)
        | Expr::NodeVoltage(_)
        | Expr::BranchCurrent(_)
        | Expr::StringLiteral(_)
        | Expr::LookupTable(_)
        | Expr::Time
        | Expr::Frequency
        | Expr::Temperature
        | Expr::ThermalVoltage
        | Expr::Gmin => Ok(expr),
    }
}

#[derive(Debug, Clone, Copy)]
enum FileInterpolation {
    Linear,
    NaturalCubic,
    Akima,
    Wodicka,
    Barycentric,
}

fn table_file_mode(func: Function) -> Option<(bool, FileInterpolation)> {
    match func {
        Function::Table | Function::TableFile => Some((true, FileInterpolation::Linear)),
        Function::FastTable | Function::FastTableFile => Some((false, FileInterpolation::Linear)),
        Function::Cubic | Function::CubicFile => Some((false, FileInterpolation::NaturalCubic)),
        Function::Akima | Function::AkimaFile => Some((false, FileInterpolation::Akima)),
        Function::Wodicka | Function::WodickaFile => Some((false, FileInterpolation::Wodicka)),
        Function::Barycentric | Function::BarycentricFile => {
            Some((false, FileInterpolation::Barycentric))
        }
        _ => None,
    }
}

fn resolve_table_file_function(
    func: Function,
    args: Vec<Expr>,
    source_path: Option<&Path>,
    transient_breakpoints: bool,
    interpolation: FileInterpolation,
) -> Result<Expr, String> {
    let Some(Expr::StringLiteral(path)) = args.first() else {
        if matches!(func, Function::Table) {
            return Ok(Expr::Function { func, args });
        }
        return Err(format!(
            "{} requires a file path as its first argument",
            function_name(func)
        ));
    };
    if args.len() > 3 {
        return Err(format!(
            "{} file lookup accepts a path, optional sample count, and optional log-scale flag; got {} arguments",
            function_name(func),
            args.len()
        ));
    }

    let path = resolve_table_path(path, source_path);
    let mut points = load_lookup_points(&path)?;
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
    let interpolation = match interpolation {
        FileInterpolation::Linear => LookupInterpolation::Linear,
        FileInterpolation::NaturalCubic => LookupInterpolation::NaturalCubic {
            second_derivatives: Arc::from(
                natural_cubic_second_derivatives(&points).into_boxed_slice(),
            ),
        },
        FileInterpolation::Akima => LookupInterpolation::Akima {
            coefficients: Arc::from(akima_coefficients(&points).into_boxed_slice()),
        },
        FileInterpolation::Wodicka => LookupInterpolation::Wodicka {
            coefficients: Arc::from(wodicka_coefficients(&points).into_boxed_slice()),
        },
        FileInterpolation::Barycentric => LookupInterpolation::Barycentric {
            weights: Arc::from(barycentric_weights(&points).into_boxed_slice()),
        },
    };
    Ok(Expr::LookupTable(LookupTable {
        points: Arc::from(points.into_boxed_slice()),
        interpolation,
        transient_breakpoints,
    }))
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

fn load_lookup_points(path: &Path) -> Result<Vec<(Value, Value)>, String> {
    let content = std::fs::read_to_string(path).map_err(|err| {
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
            let Expr::LookupTable(table) = &resolved else {
                panic!("cubic file should resolve into lookup data");
            };
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
        let Expr::LookupTable(table) = resolved else {
            panic!("file table should resolve into lookup data");
        };
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
            let Expr::LookupTable(table) = &resolved else {
                panic!("Akima file should resolve into lookup data");
            };
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
            let Expr::LookupTable(table) = &resolved else {
                panic!("Wodicka file should resolve into lookup data");
            };
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
            let Expr::LookupTable(table) = &resolved else {
                panic!("barycentric file should resolve into lookup data");
            };
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

    fn unique_temp_dir(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("rspice-{label}-{unique}"))
    }
}
