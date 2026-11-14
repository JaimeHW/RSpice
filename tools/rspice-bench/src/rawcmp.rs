//! Rawfile comparison for paired runs: do two executables produce the same
//! result vectors for one deck?
//!
//! Only the plots' vectors are compared: plot name, flags, variable names
//! and types, point counts and the data. Header lines that name the run
//! rather than its result (`Title`, `Date`, `Command`) are ignored.

use serde::Serialize;

/// One plot of a rawfile.
#[derive(Debug, PartialEq)]
struct Plot {
    name: String,
    complex: bool,
    variables: Vec<String>,
    points: usize,
    /// Values in file order: point-major, then variable, then (for complex
    /// plots) real and imaginary parts.
    values: Vec<f64>,
    /// The values exactly as written, for the byte-identical test.
    bytes: Vec<u8>,
}

/// What comparing two rawfiles found.
#[derive(Debug, Clone, Serialize)]
pub struct Comparison {
    /// True when every compared value agrees within the tolerance (exactly,
    /// byte for byte, when no tolerance is given).
    pub matches: bool,
    /// Plots compared.
    pub plots: usize,
    /// Values compared.
    pub values: usize,
    /// Largest relative difference found (0 when identical).
    pub max_relative_difference: f64,
    /// Why the outputs disagree, when they do.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl Comparison {
    fn mismatch(detail: String) -> Self {
        Self {
            matches: false,
            plots: 0,
            values: 0,
            max_relative_difference: f64::INFINITY,
            detail: Some(detail),
        }
    }
}

fn header_value<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let (name, value) = line.split_once(':')?;
    name.trim()
        .eq_ignore_ascii_case(key)
        .then_some(value.trim())
}

fn read_line<'a>(bytes: &'a [u8], cursor: &mut usize) -> Option<&'a str> {
    if *cursor >= bytes.len() {
        return None;
    }
    let rest = &bytes[*cursor..];
    let end = rest
        .iter()
        .position(|&byte| byte == b'\n')
        .unwrap_or(rest.len());
    *cursor += (end + 1).min(rest.len());
    std::str::from_utf8(&rest[..end]).ok()
}

/// Parse every plot of a binary or ASCII rawfile.
fn parse(bytes: &[u8]) -> Result<Vec<Plot>, String> {
    let mut plots = Vec::new();
    let mut cursor = 0usize;
    while cursor < bytes.len() {
        let mut name = String::new();
        let mut complex = false;
        let mut variable_count = 0usize;
        let mut points = 0usize;
        let mut variables = Vec::new();
        let mut binary = None;
        while let Some(line) = read_line(bytes, &mut cursor) {
            let trimmed = line.trim_end_matches('\r');
            if trimmed.trim().is_empty() && name.is_empty() && variables.is_empty() {
                continue;
            }
            if let Some(value) = header_value(trimmed, "Plotname") {
                name = value.to_string();
            } else if let Some(value) = header_value(trimmed, "Flags") {
                complex = value.to_ascii_lowercase().contains("complex");
            } else if let Some(value) = header_value(trimmed, "No. Variables") {
                variable_count = value
                    .parse()
                    .map_err(|_| format!("bad variable count `{value}`"))?;
            } else if let Some(value) = header_value(trimmed, "No. Points") {
                points = value
                    .parse()
                    .map_err(|_| format!("bad point count `{value}`"))?;
            } else if trimmed.trim().eq_ignore_ascii_case("Variables:") {
                for _ in 0..variable_count {
                    let entry = read_line(bytes, &mut cursor)
                        .ok_or("rawfile ends inside its variable list")?;
                    let fields: Vec<&str> = entry.split_whitespace().collect();
                    variables.push(fields.get(1..).unwrap_or_default().join(" "));
                }
            } else if trimmed.trim().eq_ignore_ascii_case("Binary:") {
                binary = Some(true);
                break;
            } else if trimmed.trim().eq_ignore_ascii_case("Values:") {
                binary = Some(false);
                break;
            }
        }
        let Some(binary) = binary else {
            break;
        };
        let per_point = variable_count * if complex { 2 } else { 1 };
        let count = per_point * points;
        let (values, raw) = if binary {
            let length = count * 8;
            let end = cursor + length;
            if end > bytes.len() {
                return Err(format!("plot `{name}` is truncated"));
            }
            let raw = bytes[cursor..end].to_vec();
            cursor = end;
            let values = raw
                .chunks_exact(8)
                .map(|chunk| {
                    let mut word = [0u8; 8];
                    word.copy_from_slice(chunk);
                    f64::from_le_bytes(word)
                })
                .collect();
            (values, raw)
        } else {
            let mut values = Vec::with_capacity(count);
            let start = cursor;
            while values.len() < count {
                let line = read_line(bytes, &mut cursor)
                    .ok_or_else(|| format!("plot `{name}` ends early"))?;
                let mut fields = line.split_whitespace();
                // The first line of each point leads with its index.
                if values.len() % per_point.max(1) == 0 {
                    fields.next();
                }
                for field in fields {
                    for part in field.split(',') {
                        if part.is_empty() {
                            continue;
                        }
                        values.push(
                            part.parse::<f64>()
                                .map_err(|_| format!("bad value `{part}` in `{name}`"))?,
                        );
                    }
                }
            }
            (values, bytes[start..cursor].to_vec())
        };
        plots.push(Plot {
            name,
            complex,
            variables,
            points,
            values,
            bytes: raw,
        });
    }
    if plots.is_empty() {
        return Err("no plot found".to_string());
    }
    Ok(plots)
}

/// Compare the result vectors of two rawfiles. With `tolerance` `None` the
/// data must be byte-identical; with `Some(rel)` each pair of values must
/// satisfy `|a - b| <= rel * max(|a|, |b|, 1e-12 * peak)`, where `peak` is
/// the largest magnitude in that variable, so values near a zero crossing
/// are judged on the signal's scale.
pub fn compare(a: &[u8], b: &[u8], tolerance: Option<f64>) -> Comparison {
    let (plots_a, plots_b) = match (parse(a), parse(b)) {
        (Ok(a), Ok(b)) => (a, b),
        (Err(error), _) => return Comparison::mismatch(format!("A's rawfile: {error}")),
        (_, Err(error)) => return Comparison::mismatch(format!("B's rawfile: {error}")),
    };
    if plots_a.len() != plots_b.len() {
        return Comparison::mismatch(format!("{} plots against {}", plots_a.len(), plots_b.len()));
    }
    let mut values = 0usize;
    let mut max_relative = 0.0_f64;
    for (a, b) in plots_a.iter().zip(&plots_b) {
        if a.name != b.name
            || a.complex != b.complex
            || a.variables != b.variables
            || a.points != b.points
        {
            return Comparison::mismatch(format!(
                "plot `{}` ({} variables, {} points) against `{}` ({} variables, {} points)",
                a.name,
                a.variables.len(),
                a.points,
                b.name,
                b.variables.len(),
                b.points
            ));
        }
        values += a.values.len();
        if a.bytes == b.bytes {
            continue;
        }
        let Some(tolerance) = tolerance else {
            let first = a
                .values
                .iter()
                .zip(&b.values)
                .position(|(x, y)| x.to_bits() != y.to_bits())
                .unwrap_or(0);
            let width = a.variables.len().max(1) * if a.complex { 2 } else { 1 };
            return Comparison::mismatch(format!(
                "plot `{}` differs first at point {}, `{}`: {:e} against {:e}",
                a.name,
                first / width,
                a.variables
                    .get((first % width) / if a.complex { 2 } else { 1 })
                    .map_or("?", String::as_str),
                a.values.get(first).copied().unwrap_or(f64::NAN),
                b.values.get(first).copied().unwrap_or(f64::NAN),
            ));
        };
        let width = a.variables.len().max(1) * if a.complex { 2 } else { 1 };
        let mut peak = vec![0.0_f64; width];
        for (index, value) in a.values.iter().chain(&b.values).enumerate() {
            let slot = index % width;
            peak[slot] = peak[slot].max(value.abs());
        }
        for (index, (&x, &y)) in a.values.iter().zip(&b.values).enumerate() {
            if x.to_bits() == y.to_bits() {
                continue;
            }
            let scale = x.abs().max(y.abs()).max(1e-12 * peak[index % width]);
            let relative = if scale > 0.0 && (x - y).is_finite() {
                (x - y).abs() / scale
            } else {
                f64::INFINITY
            };
            max_relative = max_relative.max(relative);
        }
        if max_relative > tolerance {
            return Comparison {
                matches: false,
                plots: plots_a.len(),
                values,
                max_relative_difference: max_relative,
                detail: Some(format!(
                    "plot `{}` differs by {max_relative:.3e} relative, beyond {tolerance:e}",
                    a.name
                )),
            };
        }
    }
    Comparison {
        matches: true,
        plots: plots_a.len(),
        values,
        max_relative_difference: max_relative,
        detail: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn binary(title: &str, values: &[f64]) -> Vec<u8> {
        let mut bytes = format!(
            "Title: {title}\nDate: now\nPlotname: DC OP\nFlags: real\nNo. Variables: 2\n\
             No. Points: 1\nVariables:\n\t0\tV(A)\tvoltage\n\t1\tI(V1)\tcurrent\nBinary:\n"
        )
        .into_bytes();
        for value in values {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        bytes
    }

    #[test]
    fn identical_vectors_match_whatever_the_run_header_says() {
        let comparison = compare(
            &binary("one", &[1.0, -2e-3]),
            &binary("two", &[1.0, -2e-3]),
            None,
        );
        assert!(comparison.matches, "{comparison:?}");
        assert_eq!(comparison.values, 2);
    }

    #[test]
    fn a_one_ulp_difference_fails_exact_and_passes_a_tolerance() {
        let a = binary("t", &[1.0, -2e-3]);
        let b = binary("t", &[f64::from_bits(1.0_f64.to_bits() + 1), -2e-3]);
        assert!(!compare(&a, &b, None).matches);
        let tolerant = compare(&a, &b, Some(1e-12));
        assert!(tolerant.matches, "{tolerant:?}");
        assert!(tolerant.max_relative_difference > 0.0);
        assert!(!compare(&a, &binary("t", &[1.1, -2e-3]), Some(1e-3)).matches);
    }

    #[test]
    fn ascii_rawfiles_parse_too() {
        let text = b"Title: t\nPlotname: AC Analysis\nFlags: complex\nNo. Variables: 2\n\
No. Points: 2\nVariables:\n\t0\tfrequency\tfrequency\n\t1\tv(out)\tvoltage\nValues:\n\
 0\t1,0\n\t0.5,-0.5\n\n 1\t10,0\n\t0.1,-0.3\n";
        let comparison = compare(text, text, None);
        assert!(comparison.matches, "{comparison:?}");
        assert_eq!(comparison.values, 8);
    }

    #[test]
    fn a_different_variable_list_is_a_mismatch() {
        let mut b = binary("t", &[1.0, 2.0]);
        let at = b.windows(4).position(|w| w == b"V(A)").expect("name");
        b[at + 2] = b'B';
        assert!(!compare(&binary("t", &[1.0, 2.0]), &b, Some(1.0)).matches);
    }
}
