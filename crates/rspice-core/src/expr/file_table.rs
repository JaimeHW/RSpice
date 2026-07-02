//! Build-time resolution for file-backed behavioral lookup functions.

use super::ast::{Expr, Function, LookupTable};
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

            if let Some(transient_breakpoints) = table_file_breakpoint_mode(func) {
                return resolve_table_file_function(
                    func,
                    resolved_args,
                    source_path,
                    transient_breakpoints,
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

fn table_file_breakpoint_mode(func: Function) -> Option<bool> {
    match func {
        Function::Table | Function::TableFile => Some(true),
        Function::FastTable | Function::FastTableFile => Some(false),
        _ => None,
    }
}

fn resolve_table_file_function(
    func: Function,
    args: Vec<Expr>,
    source_path: Option<&Path>,
    transient_breakpoints: bool,
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
    Ok(Expr::LookupTable(LookupTable {
        points: Arc::from(points.into_boxed_slice()),
        transient_breakpoints,
    }))
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

    fn unique_temp_dir(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("rspice-{label}-{unique}"))
    }
}
