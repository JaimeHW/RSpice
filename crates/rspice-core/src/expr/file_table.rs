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
        | Expr::Temperature => Ok(expr),
    }
}

#[derive(Debug, Clone, Copy)]
enum FileInterpolation {
    Linear,
    NaturalCubic,
    Akima,
}

fn table_file_mode(func: Function) -> Option<(bool, FileInterpolation)> {
    match func {
        Function::Table | Function::TableFile => Some((true, FileInterpolation::Linear)),
        Function::FastTable | Function::FastTableFile => Some((false, FileInterpolation::Linear)),
        Function::Cubic | Function::CubicFile => Some((false, FileInterpolation::NaturalCubic)),
        Function::Akima | Function::AkimaFile => Some((false, FileInterpolation::Akima)),
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
    if args.len() > 1 {
        return Err(format!(
            "{} file lookup currently accepts exactly one file argument, got {}",
            function_name(func),
            args.len()
        ));
    }

    let path = resolve_table_path(path, source_path);
    let points = load_lookup_points(&path)?;
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
    };
    Ok(Expr::LookupTable(LookupTable {
        points: Arc::from(points.into_boxed_slice()),
        interpolation,
        transient_breakpoints,
    }))
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
    let mut slopes = vec![0.0; count + 3];
    for index in 0..count - 1 {
        slopes[index + 2] =
            (points[index + 1].1 - points[index].1) / (points[index + 1].0 - points[index].0);
    }

    slopes[0] = 3.0 * slopes[2] - 2.0 * slopes[3];
    slopes[1] = 2.0 * slopes[2] - slopes[3];
    slopes[count + 1] = 2.0 * slopes[count] - slopes[count - 1];
    slopes[count + 2] = 3.0 * slopes[count] - 2.0 * slopes[count - 1];

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
            let span = points[index + 1].0 - points[index].0;
            let left_tangent = tangents[index];
            let right_tangent = tangents[index + 1];
            [
                left_tangent,
                (3.0 * slopes[index + 2] - 2.0 * left_tangent - right_tangent) / span,
                (left_tangent + right_tangent - 2.0 * slopes[index + 2]) / span.powi(2),
            ]
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
    fn natural_cubic_requires_exactly_one_file_argument() {
        let too_few = parse_expression_strict("cubic()")
            .expect_err("cubic without a path must fail")
            .to_string();
        assert!(too_few.contains("at least 1 argument"), "{too_few}");

        let too_many = parse_expression_strict("cubic(\"curve.dat\", 4)")
            .expect_err("unsupported resampling arguments must fail")
            .to_string();
        assert!(too_many.contains("at most 1 arguments"), "{too_many}");
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

    fn unique_temp_dir(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("rspice-{label}-{unique}"))
    }
}
