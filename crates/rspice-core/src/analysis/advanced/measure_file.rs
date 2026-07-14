//! Production readers for Xyce comparison tables consumed by `.MEASURE ERROR`.

use crate::Value;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Delimiter {
    Whitespace,
    Comma,
}

pub(super) fn read_error_comparison_column(
    file: &str,
    dependent_column: usize,
) -> Result<Vec<Value>, String> {
    let extension = Path::new(file)
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
        .ok_or_else(|| "comparison filename has no UTF-8 extension".to_string())?;
    let contents = crate::xspice::read_data_file_to_string(file)?;
    match extension.as_str() {
        "prn" | "csv" => parse_prn_or_csv_column(&contents, dependent_column),
        "csd" => parse_csd_column(&contents, dependent_column),
        _ => Err(format!(
            "unsupported comparison format '.{extension}'; expected PRN, CSV, or CSD"
        )),
    }
}

fn parse_prn_or_csv_column(content: &str, column: usize) -> Result<Vec<Value>, String> {
    let lines = content
        .lines()
        .enumerate()
        .map(|(index, line)| (index + 1, line.trim()))
        .filter(|(_, line)| !line.is_empty())
        .collect::<Vec<_>>();
    if lines.is_empty() {
        return Err("comparison table is empty".to_string());
    }

    let (header_index, header_line, delimiter, column_count) = lines
        .iter()
        .enumerate()
        .find_map(|(index, (line_number, line))| {
            header_shape(line).map(|(delimiter, count)| (index, *line_number, delimiter, count))
        })
        .ok_or_else(|| "comparison table has no recognizable header".to_string())?;
    if column >= column_count {
        return Err(format!(
            "dependent column {column} does not exist; table has {column_count} columns"
        ));
    }

    let mut values = Vec::new();
    for (line_number, line) in lines.iter().skip(header_index + 1).copied() {
        if line.to_ascii_lowercase().starts_with("end of xyce") || is_footer(line) {
            break;
        }
        if is_separator(line) {
            continue;
        }
        if let Some((repeated_delimiter, repeated_count)) = header_shape(line) {
            if repeated_delimiter == delimiter && repeated_count == column_count {
                continue;
            }
            return Err(format!(
                "comparison table changes columns at line {line_number}"
            ));
        }
        let fields = split_fields(line, delimiter)
            .map_err(|error| format!("invalid comparison row at line {line_number}: {error}"))?;
        if fields.len() != column_count {
            return Err(format!(
                "comparison row at line {line_number} has {} columns; expected {column_count}",
                fields.len()
            ));
        }
        values.push(parse_finite_value(&fields[column], line_number)?);
    }
    if values.is_empty() {
        return Err(format!(
            "comparison table after header line {header_line} has no data rows"
        ));
    }
    Ok(values)
}

fn header_shape(line: &str) -> Option<(Delimiter, usize)> {
    let delimiter = if line.contains(',') {
        Delimiter::Comma
    } else {
        Delimiter::Whitespace
    };
    let fields = split_fields(line, delimiter).ok()?;
    if fields.len() < 2 || fields.iter().all(|field| parse_numeric(field).is_ok()) {
        return None;
    }
    Some((delimiter, fields.len()))
}

fn split_fields(line: &str, delimiter: Delimiter) -> Result<Vec<String>, String> {
    match delimiter {
        Delimiter::Whitespace => Ok(line.split_whitespace().map(str::to_string).collect()),
        Delimiter::Comma => split_csv_record(line),
    }
}

fn split_csv_record(line: &str) -> Result<Vec<String>, String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut chars = line.chars().peekable();
    while let Some(character) = chars.next() {
        match character {
            '"' if quoted && chars.peek() == Some(&'"') => {
                field.push('"');
                chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => {
                fields.push(field.trim().to_string());
                field.clear();
            }
            _ => field.push(character),
        }
    }
    if quoted {
        return Err("unterminated quoted CSV field".to_string());
    }
    fields.push(field.trim().to_string());
    Ok(fields)
}

fn parse_csd_column(content: &str, column: usize) -> Result<Vec<Value>, String> {
    let lines = content
        .lines()
        .enumerate()
        .map(|(index, line)| (index + 1, line.trim()))
        .filter(|(_, line)| !line.is_empty())
        .collect::<Vec<_>>();
    if lines.is_empty() {
        return Err("CSDF comparison table is empty".to_string());
    }

    let mut values = Vec::new();
    let mut index = 0usize;
    while index < lines.len() {
        let (line_number, line) = lines[index];
        if !line.eq_ignore_ascii_case("#H") {
            return Err(format!(
                "CSDF section must begin with #H at line {line_number}"
            ));
        }
        index += 1;
        let mut complex_values = false;
        while index < lines.len() && !lines[index].1.eq_ignore_ascii_case("#N") {
            for field in lines[index].1.split_whitespace() {
                if let Some((key, value)) = field.split_once('=')
                    && key.eq_ignore_ascii_case("COMPLEXVALUES")
                    && value.trim_matches(['\'', '"']).eq_ignore_ascii_case("YES")
                {
                    complex_values = true;
                }
            }
            index += 1;
        }
        if complex_values {
            return Err("complex-valued CSDF comparison tables are not valid for DC ERROR".into());
        }
        if index >= lines.len() {
            return Err("CSDF section has no #N column block".to_string());
        }
        index += 1;
        let Some((column_line_number, column_line)) = lines.get(index).copied() else {
            return Err("CSDF #N block has no column-name line".to_string());
        };
        let named_column_count = parse_csd_columns(column_line)
            .map_err(|error| format!("invalid CSDF column line {column_line_number}: {error}"))?;
        let logical_column_count = named_column_count
            .checked_add(1)
            .ok_or_else(|| "CSDF column count overflows usize".to_string())?;
        if column >= logical_column_count {
            return Err(format!(
                "dependent column {column} does not exist; CSDF rows expose {logical_column_count} columns including the sweep variable"
            ));
        }
        index += 1;

        while index < lines.len() {
            let (row_line_number, row_header) = lines[index];
            if row_header.eq_ignore_ascii_case("#;") {
                index += 1;
                break;
            }
            if row_header.eq_ignore_ascii_case("#H") {
                break;
            }
            let (sweep, expected_values) = parse_csd_row_header(row_header).map_err(|error| {
                format!("invalid CSDF row header at line {row_line_number}: {error}")
            })?;
            if expected_values != named_column_count {
                return Err(format!(
                    "CSDF row at line {row_line_number} declares {expected_values} values; expected {named_column_count}"
                ));
            }
            index += 1;
            let mut row = Vec::with_capacity(logical_column_count);
            row.push(sweep);
            while row.len() < logical_column_count {
                let Some((data_line_number, data_line)) = lines.get(index).copied() else {
                    return Err(format!(
                        "CSDF row at line {row_line_number} ended before all values were read"
                    ));
                };
                if data_line.starts_with('#') {
                    return Err(format!(
                        "CSDF row at line {row_line_number} ended at line {data_line_number} before all values were read"
                    ));
                }
                for token in data_line.split_whitespace() {
                    if row.len() >= logical_column_count {
                        return Err(format!(
                            "CSDF row at line {row_line_number} contains too many values"
                        ));
                    }
                    let expected_position = row.len();
                    row.push(parse_csd_value(token, expected_position, data_line_number)?);
                }
                index += 1;
            }
            values.push(row[column]);
        }
    }
    if values.is_empty() {
        return Err("CSDF comparison table has no data rows".to_string());
    }
    Ok(values)
}

fn parse_csd_columns(line: &str) -> Result<usize, String> {
    if !line.contains('\'') {
        let count = line.split_whitespace().count();
        return (count > 0)
            .then_some(count)
            .ok_or_else(|| "column list is empty".to_string());
    }
    let mut count = 0usize;
    let mut rest = line;
    loop {
        let Some(start) = rest.find('\'') else {
            if rest.trim().is_empty() {
                break;
            }
            return Err(format!("unexpected unquoted text '{}'", rest.trim()));
        };
        if !rest[..start].trim().is_empty() {
            return Err(format!(
                "unexpected text before quoted column '{}'",
                rest[..start].trim()
            ));
        }
        let after_start = &rest[start + 1..];
        let Some(end) = after_start.find('\'') else {
            return Err("unterminated quoted column".to_string());
        };
        count += 1;
        rest = &after_start[end + 1..];
    }
    (count > 0)
        .then_some(count)
        .ok_or_else(|| "column list is empty".to_string())
}

fn parse_csd_row_header(line: &str) -> Result<(Value, usize), String> {
    let fields = line.split_whitespace().collect::<Vec<_>>();
    if fields.len() != 3 || !fields[0].eq_ignore_ascii_case("#C") {
        return Err("expected '#C <sweep-value> <value-count>'".to_string());
    }
    let sweep = parse_numeric(fields[1])?;
    if !sweep.is_finite() {
        return Err("sweep value is non-finite".to_string());
    }
    let count = fields[2]
        .parse::<usize>()
        .map_err(|error| format!("invalid value count '{}': {error}", fields[2]))?;
    Ok((sweep, count))
}

fn parse_csd_value(token: &str, position: usize, line: usize) -> Result<Value, String> {
    let (value, encoded_position) = token
        .split_once(':')
        .ok_or_else(|| format!("CSDF token '{token}' at line {line} lacks a position suffix"))?;
    let encoded_position = encoded_position.parse::<usize>().map_err(|error| {
        format!("invalid CSDF position in token '{token}' at line {line}: {error}")
    })?;
    if encoded_position != position {
        return Err(format!(
            "CSDF token '{token}' at line {line} has position {encoded_position}; expected {position}"
        ));
    }
    let (real, imaginary) = value.split_once('/').unwrap_or((value, "0"));
    let real = parse_numeric(real)?;
    let imaginary = parse_numeric(imaginary)?;
    if !real.is_finite() || !imaginary.is_finite() {
        return Err(format!("CSDF token '{token}' at line {line} is non-finite"));
    }
    if imaginary != 0.0 {
        return Err(format!(
            "CSDF token '{token}' at line {line} has a nonzero imaginary component"
        ));
    }
    Ok(real)
}

fn parse_finite_value(token: &str, line: usize) -> Result<Value, String> {
    let value = parse_numeric(token)
        .map_err(|error| format!("invalid numeric token '{token}' at line {line}: {error}"))?;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(format!(
            "numeric token '{token}' at line {line} is non-finite"
        ))
    }
}

fn parse_numeric(token: &str) -> Result<Value, String> {
    let normalized = token.trim().trim_end_matches(',');
    normalized
        .parse::<Value>()
        .or_else(|_| normalized.replace(['D', 'd'], "e").parse::<Value>())
        .map_err(|error| error.to_string())
}

fn is_separator(line: &str) -> bool {
    line.chars()
        .all(|character| character == '-' || character == '=' || character.is_whitespace())
}

fn is_footer(line: &str) -> bool {
    let normalized = line.to_ascii_lowercase();
    normalized.starts_with("cpu time")
        || normalized.starts_with("total cpu time")
        || normalized.starts_with("current dynamic memory usage")
        || normalized.starts_with("dynamic memory limit")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prn_and_csv_columns_preserve_format_column_numbering() {
        let prn = "Index A B\n0 1 2\n1 3 4\nEnd of Xyce(TM) Simulation\n";
        assert_eq!(parse_prn_or_csv_column(prn, 2).unwrap(), vec![2.0, 4.0]);

        let csv = "A,B\n1,2\n3,4\n";
        assert_eq!(parse_prn_or_csv_column(csv, 1).unwrap(), vec![2.0, 4.0]);
    }

    #[test]
    fn csd_columns_include_the_sweep_variable_at_zero() {
        let csd = "#H\nCOMPLEXVALUES='NO'\n#N\n'A' 'B'\n#C 5 2\n1:1 2:2\n#C 4 2\n3:1 4:2\n#;\n";
        assert_eq!(parse_csd_column(csd, 0).unwrap(), vec![5.0, 4.0]);
        assert_eq!(parse_csd_column(csd, 2).unwrap(), vec![2.0, 4.0]);
    }

    #[test]
    fn malformed_rows_and_nonfinite_values_fail_closed() {
        assert!(parse_prn_or_csv_column("Index A\n0 NaN\n", 1).is_err());
        assert!(parse_prn_or_csv_column("Index A\n0\n", 1).is_err());
        assert!(parse_csd_column("#H\n#N\n'A'\n#C 0 1\n1:2\n#;\n", 1).is_err());
    }
}
