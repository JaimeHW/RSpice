//! Fail-closed Touchstone v1/v2 S-parameter reader.
//!
//! The reader accepts real/imaginary, magnitude/angle, and dB/angle network
//! data. Unsupported v2 constructs (mixed-mode and noise blocks) are rejected
//! explicitly so they can never be mistaken for ordinary network records.

use super::{SignalType, WaveformDataset, WaveformSignal};
use std::path::Path;

const MAX_TOUCHSTONE_PORTS: usize = 64;

#[derive(Debug, Clone, Copy)]
enum DataFormat {
    Ri,
    Ma,
    Db,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatrixFormat {
    Full,
    Lower,
    Upper,
}

#[derive(Debug, Clone, Copy)]
enum TwoPortOrder {
    TwentyOneTwelve,
    TwelveTwentyOne,
}

#[derive(Debug, Clone, Copy)]
struct Options {
    frequency_scale_hz: f64,
    data_format: DataFormat,
    reference_ohms: f64,
}

/// Parse one selected Touchstone artifact without touching project state.
pub(crate) fn read_touchstone_bytes(
    source_name: &str,
    bytes: &[u8],
) -> Result<WaveformDataset, String> {
    let text = std::str::from_utf8(bytes)
        .map_err(|error| format!("Touchstone source is not valid UTF-8: {error}"))?;
    let mut options = Options {
        // Touchstone v1 defaults.
        frequency_scale_hz: 1.0e9,
        data_format: DataFormat::Ma,
        reference_ohms: 50.0,
    };
    let mut version = 1_u32;
    let mut matrix_format = MatrixFormat::Full;
    let mut declared_two_port_order = None;
    let mut declared_ports = ports_from_extension(source_name);
    let mut declared_frequencies = None;
    let mut reference_values = None;
    // `[Reference]` arguments may span several lines, so the values collected
    // so far are held here until one per port has been read.
    let mut pending_reference: Option<Vec<f64>> = None;
    let mut expected_reference_len = 0usize;
    let mut numeric_tokens = Vec::new();
    let mut saw_network_data = false;
    let mut saw_end = false;
    let mut in_information = false;

    for (line_index, raw_line) in text.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line
            .split_once('!')
            .map_or(raw_line, |(before, _)| before);
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if saw_end {
            return Err(format!(
                "Touchstone line {line_number}: content after [End] is not allowed"
            ));
        }
        if in_information {
            if trimmed.eq_ignore_ascii_case("[End Information]") {
                in_information = false;
            }
            continue;
        }
        // `[Reference]` arguments may begin on the line following the keyword
        // and may span multiple lines, so plain numeric lines belong to it
        // until every port has an impedance. Anything else closes the list and
        // is parsed as the keyword or option line it is.
        if pending_reference.is_some() {
            if trimmed.starts_with('[') || trimmed.starts_with('#') {
                reference_values = pending_reference.take();
            } else {
                let mut values = pending_reference.take().unwrap_or_default();
                values.extend(parse_numeric_values(trimmed, line_number)?);
                if values.len() >= expected_reference_len {
                    reference_values = Some(values);
                } else {
                    pending_reference = Some(values);
                }
                continue;
            }
        }
        if trimmed.starts_with('#') {
            options = parse_option_line(trimmed, line_number)?;
            continue;
        }
        if trimmed.starts_with('[') {
            let (section, value) = parse_section_line(trimmed, line_number)?;
            match section.as_str() {
                "version" => {
                    let parsed = value.parse::<f64>().map_err(|_| {
                        format!("Touchstone line {line_number}: invalid [Version] '{value}'")
                    })?;
                    if !parsed.is_finite() || !(1.0..3.0).contains(&parsed) {
                        return Err(format!(
                            "Touchstone line {line_number}: unsupported [Version] '{value}'"
                        ));
                    }
                    version = parsed.floor() as u32;
                }
                "number of ports" => {
                    declared_ports = Some(parse_positive_usize(
                        value,
                        line_number,
                        "[Number of Ports]",
                    )?);
                }
                "number of frequencies" => {
                    declared_frequencies = Some(parse_positive_usize(
                        value,
                        line_number,
                        "[Number of Frequencies]",
                    )?);
                }
                "matrix format" => {
                    matrix_format = match value.to_ascii_lowercase().as_str() {
                        "full" => MatrixFormat::Full,
                        "lower" => MatrixFormat::Lower,
                        "upper" => MatrixFormat::Upper,
                        _ => {
                            return Err(format!(
                                "Touchstone line {line_number}: unsupported [Matrix Format] '{value}'"
                            ));
                        }
                    };
                }
                "two-port data order" => {
                    if declared_two_port_order.is_some() {
                        return Err(format!(
                            "Touchstone line {line_number}: [Two-Port Data Order] may appear only once"
                        ));
                    }
                    declared_two_port_order = Some(match value.to_ascii_lowercase().as_str() {
                        "21_12" => TwoPortOrder::TwentyOneTwelve,
                        "12_21" => TwoPortOrder::TwelveTwentyOne,
                        _ => {
                            return Err(format!(
                                "Touchstone line {line_number}: unsupported [Two-Port Data Order] '{value}'"
                            ));
                        }
                    });
                }
                "reference" => {
                    if reference_values.is_some() {
                        return Err(format!(
                            "Touchstone line {line_number}: [Reference] may appear only once"
                        ));
                    }
                    // The specification places [Reference] after [Number of
                    // Ports] precisely so the argument count is known here.
                    let ports = declared_ports.ok_or_else(|| {
                        format!(
                            "Touchstone line {line_number}: [Reference] requires a declared port count"
                        )
                    })?;
                    expected_reference_len = ports;
                    let values = if value.is_empty() {
                        Vec::new()
                    } else {
                        parse_numeric_values(value, line_number)?
                    };
                    if values.len() >= ports {
                        reference_values = Some(values);
                    } else {
                        pending_reference = Some(values);
                    }
                }
                "network data" => saw_network_data = true,
                "begin information" => in_information = true,
                "end information" => {
                    return Err(format!(
                        "Touchstone line {line_number}: [End Information] has no matching [Begin Information]"
                    ));
                }
                "mixed-mode order" => {
                    return Err(format!(
                        "Touchstone line {line_number}: mixed-mode Touchstone import is not available in this build"
                    ));
                }
                "noise data" | "number of noise frequencies" => {
                    return Err(format!(
                        "Touchstone line {line_number}: Touchstone noise-data import is not available in this build"
                    ));
                }
                "end" => {
                    if !value.is_empty() {
                        return Err(format!(
                            "Touchstone line {line_number}: [End] must not have trailing content"
                        ));
                    }
                    saw_end = true;
                }
                _ => {
                    return Err(format!(
                        "Touchstone line {line_number}: unsupported section '[{section}]'"
                    ));
                }
            }
            continue;
        }

        if version >= 2 && !saw_network_data {
            return Err(format!(
                "Touchstone line {line_number}: numeric data precedes [Network Data]"
            ));
        }
        for token in trimmed.split_whitespace().filter(|token| *token != "+") {
            numeric_tokens.push(parse_numeric_token(token).ok_or_else(|| {
                format!(
                    "Touchstone line {line_number}: expected a finite numeric token, got '{token}'"
                )
            })?);
        }
    }
    if let Some(values) = pending_reference.take() {
        // The file ended mid-list; the per-port count check below reports it.
        reference_values = Some(values);
    }
    if in_information {
        return Err("Touchstone [Begin Information] block is not terminated".to_owned());
    }
    if version >= 2 && !saw_end {
        return Err("Touchstone v2 source is missing the required [End] section".to_owned());
    }

    let num_ports = match declared_ports {
        Some(ports) => ports,
        None => infer_ports(&numeric_tokens, matrix_format)?
            .ok_or_else(|| "Unable to determine Touchstone port count".to_owned())?,
    };
    if num_ports == 0 || num_ports > MAX_TOUCHSTONE_PORTS {
        return Err(format!(
            "Touchstone port count {num_ports} is outside the supported range 1..={MAX_TOUCHSTONE_PORTS}"
        ));
    }
    // The keyword is required of a two-port file and permitted of no other
    // (Touchstone 2.0 specification, "[Two-Port Data Order]").
    if declared_two_port_order.is_some() && num_ports != 2 {
        return Err(format!(
            "[Two-Port Data Order] is permitted only where the file declares two ports, not {num_ports}"
        ));
    }
    let two_port_order = declared_two_port_order.unwrap_or(TwoPortOrder::TwentyOneTwelve);
    if !matches!(two_port_order, TwoPortOrder::TwentyOneTwelve)
        && matrix_format != MatrixFormat::Full
    {
        return Err("[Two-Port Data Order] 12_21 requires a full two-port matrix".to_owned());
    }

    let record_width = values_per_frequency(num_ports, matrix_format)
        .ok_or_else(|| "Touchstone matrix dimensions overflow".to_owned())?;
    if numeric_tokens.is_empty() || !numeric_tokens.len().is_multiple_of(record_width) {
        return Err(format!(
            "Touchstone numeric data length {} is not divisible by record width {record_width}",
            numeric_tokens.len()
        ));
    }
    let frequency_count = numeric_tokens.len() / record_width;
    if let Some(expected) = declared_frequencies
        && expected != frequency_count
    {
        return Err(format!(
            "Touchstone [Number of Frequencies]={expected} but parsed {frequency_count} records"
        ));
    }
    let reference_by_port = resolve_reference_values(
        num_ports,
        options.reference_ohms,
        reference_values.as_deref(),
    )?;

    let mut frequencies = Vec::with_capacity(frequency_count);
    let mut matrix_real = vec![vec![vec![0.0; frequency_count]; num_ports]; num_ports];
    let mut matrix_imag = vec![vec![vec![0.0; frequency_count]; num_ports]; num_ports];
    let mut offset = 0;
    for frequency_index in 0..frequency_count {
        let frequency_hz = numeric_tokens[offset] * options.frequency_scale_hz;
        offset += 1;
        if !frequency_hz.is_finite() || frequency_hz <= 0.0 {
            return Err(format!(
                "Touchstone frequency point {frequency_index} must be finite and positive"
            ));
        }
        if frequencies
            .last()
            .is_some_and(|previous| *previous >= frequency_hz)
        {
            return Err(format!(
                "Touchstone frequency point {frequency_index} is not strictly increasing"
            ));
        }
        frequencies.push(frequency_hz);

        let positions = matrix_positions(num_ports, matrix_format, two_port_order);
        for (row, column) in positions {
            let (real, imag) = pair_to_complex(
                numeric_tokens[offset],
                numeric_tokens[offset + 1],
                options.data_format,
            )
            .map_err(|error| format!("Touchstone frequency point {frequency_index}: {error}"))?;
            offset += 2;
            matrix_real[row][column][frequency_index] = real;
            matrix_imag[row][column][frequency_index] = imag;
        }
        if matrix_format != MatrixFormat::Full {
            for row in 0..num_ports {
                for column in 0..num_ports {
                    let absent = match matrix_format {
                        MatrixFormat::Lower => row < column,
                        MatrixFormat::Upper => row > column,
                        MatrixFormat::Full => false,
                    };
                    if absent {
                        matrix_real[row][column][frequency_index] =
                            matrix_real[column][row][frequency_index];
                        matrix_imag[row][column][frequency_index] =
                            matrix_imag[column][row][frequency_index];
                    }
                }
            }
        }
    }

    let mut dataset = WaveformDataset::new(
        Path::new(source_name)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("touchstone"),
    );
    dataset.analysis = "S-Parameter".to_owned();
    dataset
        .metadata
        .insert("format".to_owned(), "touchstone".to_owned());
    dataset
        .metadata
        .insert("touchstone_version".to_owned(), version.to_string());
    dataset
        .metadata
        .insert("num_ports".to_owned(), num_ports.to_string());
    dataset.metadata.insert(
        "z0_ports".to_owned(),
        reference_by_port
            .iter()
            .map(f64::to_string)
            .collect::<Vec<_>>()
            .join(","),
    );
    let mut x_signal = WaveformSignal::new("frequency", SignalType::Frequency);
    x_signal.data = frequencies;
    dataset.set_x(x_signal);
    for row in 1..=num_ports {
        for column in 1..=num_ports {
            let name = if num_ports <= 9 {
                format!("S{row}{column}")
            } else {
                format!("S{row}_{column}")
            };
            let mut real = WaveformSignal::new(format!("{name}_RE"), SignalType::SParameter);
            real.data = matrix_real[row - 1][column - 1].clone();
            dataset.add_signal(real);
            let mut imag = WaveformSignal::new(format!("{name}_IM"), SignalType::SParameter);
            imag.data = matrix_imag[row - 1][column - 1].clone();
            dataset.add_signal(imag);
        }
    }
    Ok(dataset)
}

fn ports_from_extension(source_name: &str) -> Option<usize> {
    let extension = Path::new(source_name)
        .extension()?
        .to_str()?
        .to_ascii_lowercase();
    (extension.starts_with('s')
        && extension.ends_with('p')
        && extension.len() > 2
        && extension[1..extension.len() - 1]
            .bytes()
            .all(|byte| byte.is_ascii_digit()))
    .then(|| extension[1..extension.len() - 1].parse().ok())
    .flatten()
}

fn parse_positive_usize(value: &str, line: usize, field: &str) -> Result<usize, String> {
    let parsed = value
        .parse::<usize>()
        .map_err(|_| format!("Touchstone line {line}: invalid {field} '{value}'"))?;
    if parsed == 0 {
        Err(format!("Touchstone line {line}: {field} must be positive"))
    } else {
        Ok(parsed)
    }
}

fn values_per_frequency(ports: usize, matrix: MatrixFormat) -> Option<usize> {
    let pairs = match matrix {
        MatrixFormat::Full => ports.checked_mul(ports)?,
        MatrixFormat::Lower | MatrixFormat::Upper => {
            ports.checked_mul(ports.checked_add(1)?)?.checked_div(2)?
        }
    };
    pairs.checked_mul(2)?.checked_add(1)
}

fn infer_ports(tokens: &[f64], matrix: MatrixFormat) -> Result<Option<usize>, String> {
    let candidates = (1..=MAX_TOUCHSTONE_PORTS)
        .filter(|ports| {
            values_per_frequency(*ports, matrix).is_some_and(|width| {
                tokens.len() >= width
                    && tokens.len().is_multiple_of(width)
                    && (0..tokens.len() / width).all(|index| tokens[index * width] > 0.0)
            })
        })
        .collect::<Vec<_>>();
    match candidates.as_slice() {
        [] => Ok(None),
        [ports] => Ok(Some(*ports)),
        _ => Err(format!(
            "Touchstone port count is ambiguous ({candidates:?}); use an .sNp extension or [Number of Ports]"
        )),
    }
}

/// Where each network-data pair of one frequency record belongs in the matrix.
///
/// Touchstone arranges network data row-wise: a Full matrix lists every
/// element of matrix row *r* before any element of row *r+1*, a Lower matrix
/// gives row *r* its columns 1..=*r*, and an Upper matrix gives row *r* its
/// columns *r*..=*n* (Touchstone 2.0 specification, "3-port and 4-port
/// Networks" and "5-port and Above Networks").
///
/// A full two-port matrix is the format's one documented exception: it carries
/// S11 S21 S12 S22 unless `[Two-Port Data Order] 12_21` says otherwise.
fn matrix_positions(
    ports: usize,
    matrix: MatrixFormat,
    two_port_order: TwoPortOrder,
) -> Vec<(usize, usize)> {
    if ports == 2 && matrix == MatrixFormat::Full {
        return match two_port_order {
            TwoPortOrder::TwentyOneTwelve => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
            TwoPortOrder::TwelveTwentyOne => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        };
    }
    let mut positions = Vec::new();
    for row in 0..ports {
        match matrix {
            MatrixFormat::Full => positions.extend((0..ports).map(|column| (row, column))),
            MatrixFormat::Lower => positions.extend((0..=row).map(|column| (row, column))),
            MatrixFormat::Upper => positions.extend((row..ports).map(|column| (row, column))),
        }
    }
    positions
}

fn parse_option_line(line: &str, line_number: usize) -> Result<Options, String> {
    let fields = line[1..].split_whitespace().collect::<Vec<_>>();
    if fields.len() < 3 {
        return Err(format!(
            "Touchstone line {line_number}: option line requires frequency unit, parameter type, and data format"
        ));
    }
    let frequency_scale_hz = match fields[0].to_ascii_lowercase().as_str() {
        "hz" => 1.0,
        "khz" => 1.0e3,
        "mhz" => 1.0e6,
        "ghz" => 1.0e9,
        other => {
            return Err(format!(
                "Touchstone line {line_number}: unsupported frequency unit '{other}'"
            ));
        }
    };
    if !fields[1].eq_ignore_ascii_case("s") {
        return Err(format!(
            "Touchstone line {line_number}: only S-parameter data is supported (found '{}')",
            fields[1]
        ));
    }
    let data_format = match fields[2].to_ascii_lowercase().as_str() {
        "ri" => DataFormat::Ri,
        "ma" => DataFormat::Ma,
        "db" => DataFormat::Db,
        other => {
            return Err(format!(
                "Touchstone line {line_number}: unsupported data format '{other}'"
            ));
        }
    };
    let reference_ohms = match fields.get(3..) {
        Some([]) | None => 50.0,
        Some([marker, value]) if marker.eq_ignore_ascii_case("r") => parse_numeric_token(value)
            .ok_or_else(|| {
                format!("Touchstone line {line_number}: invalid reference impedance '{value}'")
            })?,
        _ => {
            return Err(format!(
                "Touchstone line {line_number}: unexpected tokens in option line"
            ));
        }
    };
    if reference_ohms <= 0.0 {
        return Err(format!(
            "Touchstone line {line_number}: reference impedance must be positive"
        ));
    }
    Ok(Options {
        frequency_scale_hz,
        data_format,
        reference_ohms,
    })
}

fn parse_section_line(line: &str, line_number: usize) -> Result<(String, &str), String> {
    let end = line.find(']').ok_or_else(|| {
        format!("Touchstone line {line_number}: malformed section header '{line}'")
    })?;
    let section = line[1..end].trim().to_ascii_lowercase();
    if section.is_empty() {
        return Err(format!(
            "Touchstone line {line_number}: empty section header"
        ));
    }
    Ok((section, line[end + 1..].trim()))
}

fn parse_numeric_values(value: &str, line_number: usize) -> Result<Vec<f64>, String> {
    let values = value
        .split(|character: char| character.is_whitespace() || character == ',')
        .filter(|token| !token.is_empty())
        .map(|token| {
            parse_numeric_token(token).ok_or_else(|| {
                format!("Touchstone line {line_number}: invalid numeric value '{token}'")
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if values.is_empty() {
        Err(format!(
            "Touchstone line {line_number}: [Reference] requires at least one value"
        ))
    } else {
        Ok(values)
    }
}

fn parse_numeric_token(token: &str) -> Option<f64> {
    let parsed = token.replace(['D', 'd'], "e").parse::<f64>().ok()?;
    parsed.is_finite().then_some(parsed)
}

fn resolve_reference_values(
    ports: usize,
    default: f64,
    declared: Option<&[f64]>,
) -> Result<Vec<f64>, String> {
    let values = match declared {
        Some([one]) => vec![*one; ports],
        Some(values) if values.len() == ports => values.to_vec(),
        Some(values) => {
            return Err(format!(
                "Touchstone [Reference] count {} does not match port count {ports}",
                values.len()
            ));
        }
        None => vec![default; ports],
    };
    if values
        .iter()
        .any(|value| !value.is_finite() || *value <= 0.0)
    {
        return Err("Touchstone reference impedances must be finite and positive".to_owned());
    }
    Ok(values)
}

fn pair_to_complex(first: f64, second: f64, format: DataFormat) -> Result<(f64, f64), String> {
    let pair = match format {
        DataFormat::Ri => (first, second),
        DataFormat::Ma => {
            if first < 0.0 {
                return Err(format!(
                    "negative MA magnitude {first} is invalid; magnitude must be non-negative"
                ));
            }
            let angle = second.to_radians();
            (first * angle.cos(), first * angle.sin())
        }
        DataFormat::Db => {
            let magnitude = 10.0_f64.powf(first / 20.0);
            let angle = second.to_radians();
            (magnitude * angle.cos(), magnitude * angle.sin())
        }
    };
    if pair.0.is_finite() && pair.1.is_finite() {
        Ok(pair)
    } else {
        Err("S-parameter conversion produced a non-finite value".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One matrix entry, as the reader materialized it.
    fn entry(dataset: &WaveformDataset, row: usize, column: usize, index: usize) -> (f64, f64) {
        let name = format!("S{row}{column}");
        let component = |suffix: &str| {
            dataset
                .get_signal(&format!("{name}_{suffix}"))
                .unwrap_or_else(|| panic!("{name}_{suffix} is missing"))
                .data[index]
        };
        (component("RE"), component("IM"))
    }

    /// Touchstone lays a Full matrix out row-wise for every network of three
    /// ports and above: `<freq> <N11> <N12> <N13>`, then `<N21> <N22> <N23>`,
    /// then `<N31> <N32> <N33>` (Touchstone 2.0 specification, "3-port and
    /// 4-port Networks"). Every entry in this fixture names its own position,
    /// so a column-wise reading returns S21 where S12 was written.
    #[test]
    fn a_three_port_full_matrix_is_read_row_wise() {
        let dataset = read_touchstone_bytes(
            "network.s3p",
            b"!3-port S-parameter data, one matrix row per line\n\
              # Hz S RI R 50\n\
              1.0 0.11 0.01 0.12 0.02 0.13 0.03\n\
              0.21 0.04 0.22 0.05 0.23 0.06\n\
              0.31 0.07 0.32 0.08 0.33 0.09\n",
        )
        .expect("a spec-shaped 3-port file parses");

        assert_eq!(dataset.point_count(), 1);
        for (row, column, real, imag) in [
            (1, 1, 0.11, 0.01),
            (1, 2, 0.12, 0.02),
            (1, 3, 0.13, 0.03),
            (2, 1, 0.21, 0.04),
            (2, 2, 0.22, 0.05),
            (2, 3, 0.23, 0.06),
            (3, 1, 0.31, 0.07),
            (3, 2, 0.32, 0.08),
            (3, 3, 0.33, 0.09),
        ] {
            assert_eq!(
                entry(&dataset, row, column, 0),
                (real, imag),
                "S{row}{column}"
            );
        }
    }

    /// A 4-port file carries four pairs per matrix row, in the same row-wise
    /// order. The reader must not care how the rows are broken into lines.
    #[test]
    fn a_four_port_full_matrix_is_read_row_wise() {
        let dataset = read_touchstone_bytes(
            "network.s4p",
            b"# Hz S RI R 50\n\
              1.0 0.11 0 0.12 0 0.13 0 0.14 0\n\
              0.21 0 0.22 0 0.23 0 0.24 0\n\
              0.31 0 0.32 0 0.33 0 0.34 0\n\
              0.41 0 0.42 0 0.43 0 0.44 0\n",
        )
        .expect("a spec-shaped 4-port file parses");

        for row in 1..=4 {
            for column in 1..=4 {
                let expected = (row as f64 * 10.0 + column as f64) / 100.0;
                assert_eq!(
                    entry(&dataset, row, column, 0).0,
                    expected,
                    "S{row}{column}"
                );
            }
        }
    }

    /// `[Matrix Format] Lower` and `Upper` are still row-wise: Lower gives
    /// row *r* its columns 1..=*r*, Upper gives row *r* its columns *r*..=*n*
    /// (Touchstone 2.0 specification, "3-port and 4-port Networks"). The
    /// absent half is the reciprocal mirror of the half that is present.
    #[test]
    fn triangular_matrix_formats_are_read_row_wise() {
        let lower = read_touchstone_bytes(
            "lower.ts",
            b"[Version] 2.0\n\
              # Hz S RI R 50\n\
              [Number of Ports] 3\n\
              [Number of Frequencies] 1\n\
              [Matrix Format] Lower\n\
              [Network Data]\n\
              1.0 0.11 0.01\n\
              0.21 0.04 0.22 0.05\n\
              0.31 0.07 0.32 0.08 0.33 0.09\n\
              [End]\n",
        )
        .expect("a spec-shaped Lower file parses");
        let upper = read_touchstone_bytes(
            "upper.ts",
            b"[Version] 2.0\n\
              # Hz S RI R 50\n\
              [Number of Ports] 3\n\
              [Number of Frequencies] 1\n\
              [Matrix Format] Upper\n\
              [Network Data]\n\
              1.0 0.11 0.01 0.12 0.02 0.13 0.03\n\
              0.22 0.05 0.23 0.06\n\
              0.33 0.09\n\
              [End]\n",
        )
        .expect("a spec-shaped Upper file parses");

        for (row, column, real, imag) in [
            (1, 1, 0.11, 0.01),
            (2, 1, 0.21, 0.04),
            (2, 2, 0.22, 0.05),
            (3, 1, 0.31, 0.07),
            (3, 2, 0.32, 0.08),
            (3, 3, 0.33, 0.09),
        ] {
            assert_eq!(
                entry(&lower, row, column, 0),
                (real, imag),
                "S{row}{column}"
            );
            // The Lower half is mirrored into the Upper half.
            assert_eq!(
                entry(&lower, column, row, 0),
                (real, imag),
                "S{column}{row}"
            );
        }
        for (row, column, real, imag) in [
            (1, 1, 0.11, 0.01),
            (1, 2, 0.12, 0.02),
            (1, 3, 0.13, 0.03),
            (2, 2, 0.22, 0.05),
            (2, 3, 0.23, 0.06),
            (3, 3, 0.33, 0.09),
        ] {
            assert_eq!(
                entry(&upper, row, column, 0),
                (real, imag),
                "S{row}{column}"
            );
            assert_eq!(
                entry(&upper, column, row, 0),
                (real, imag),
                "S{column}{row}"
            );
        }
    }

    /// Examples 5 and 6 of the Touchstone 2.0 specification describe the same
    /// 4-port network twice: once as a Full matrix, once as a Lower one with
    /// the `[Reference]` arguments split across two lines. Reproduced here
    /// verbatim apart from the `[End]` line the specification's abridged
    /// examples omit, they must import to the same matrix.
    #[test]
    fn the_spec_full_and_lower_examples_describe_one_network() {
        let full = read_touchstone_bytes(
            "example5.ts",
            b"! 4-port S-parameter data\n\
              ! Default impedance is overridden by the [Reference] keyword arguments\n\
              ! Data cannot be represented using 1.0 syntax\n\
              [Version] 2.0\n\
              # GHz S MA R 50\n\
              [Number of Ports] 4\n\
              [Number of Frequencies] 1\n\
              [Reference] 50 75 0.01 0.01\n\
              [Matrix Format] Full\n\
              [Network Data]\n\
              5.00000 0.60 161.24 0.40 -42.20 0.42 -66.58 0.53 -79.34 !row 1\n\
              0.40 -42.20 0.60 161.20 0.53 -79.34 0.42 -66.58 !row 2\n\
              0.42 -66.58 0.53 -79.34 0.60 161.24 0.40 -42.20 !row 3\n\
              0.53 -79.34 0.42 -66.58 0.40 -42.20 0.60 161.24 !row 4\n\
              [End]\n",
        )
        .expect("specification example 5 parses");
        let lower = read_touchstone_bytes(
            "example6.ts",
            b"! 4-port S-parameter data\n\
              ! Note that [Reference] arguments are split across two lines\n\
              [Version] 2.0\n\
              # GHz S MA R 50\n\
              [Number of Ports] 4\n\
              [Number of Frequencies] 1\n\
              [Reference] 50 75\n\
              0.01 0.01\n\
              [Matrix Format] Lower\n\
              [Network Data]\n\
              5.00000 0.60 161.24 !row 1\n\
              0.40 -42.20 0.60 161.20 !row 2\n\
              0.42 -66.58 0.53 -79.34 0.60 161.24 !row 3\n\
              0.53 -79.34 0.42 -66.58 0.40 -42.20 0.60 161.24 !row 4\n\
              [End]\n",
        )
        .expect("specification example 6 parses");

        assert_eq!(full.metadata["z0_ports"], lower.metadata["z0_ports"]);
        assert_eq!(lower.metadata["z0_ports"], "50,75,0.01,0.01");
        for row in 1..=4 {
            for column in 1..=4 {
                let (full_re, full_im) = entry(&full, row, column, 0);
                let (lower_re, lower_im) = entry(&lower, row, column, 0);
                assert!(
                    (full_re - lower_re).abs() < 1e-12 && (full_im - lower_im).abs() < 1e-12,
                    "S{row}{column}: Full ({full_re}, {full_im}) vs Lower ({lower_re}, {lower_im})"
                );
            }
        }
        // Spot-check one off-diagonal entry against the specification text:
        // row 1 column 4 is 0.53 at -79.34 degrees.
        let (real, imag) = entry(&full, 1, 4, 0);
        let expected_angle: f64 = -79.34_f64.to_radians();
        assert!((real - 0.53 * expected_angle.cos()).abs() < 1e-12, "{real}");
        assert!((imag - 0.53 * expected_angle.sin()).abs() < 1e-12, "{imag}");
    }

    /// `[Reference]` arguments may begin on the line after the keyword
    /// (Touchstone 2.0 specification, "[Reference]", and its Example 4).
    #[test]
    fn reference_arguments_may_start_on_the_following_line() {
        let dataset = read_touchstone_bytes(
            "example4.ts",
            b"! Note that the [Reference] keyword arguments appear on a separate line\n\
              [Version] 2.0\n\
              # GHz S MA R 50\n\
              [Number of Ports] 4\n\
              [Reference]\n\
              50 75 0.01 0.01\n\
              [Number of Frequencies] 1\n\
              [Network Data]\n\
              5.00000 0.60 161.24 0.40 -42.20 0.42 -66.58 0.53 -79.34\n\
              0.40 -42.20 0.60 161.20 0.53 -79.34 0.42 -66.58\n\
              0.42 -66.58 0.53 -79.34 0.60 161.24 0.40 -42.20\n\
              0.53 -79.34 0.42 -66.58 0.40 -42.20 0.60 161.24\n\
              [End]\n",
        )
        .expect("specification example 4 parses");

        assert_eq!(dataset.metadata["z0_ports"], "50,75,0.01,0.01");
        assert_eq!(dataset.point_count(), 1);
    }

    /// The keyword is permitted only when the file declares two ports
    /// (Touchstone 2.0 specification, "[Two-Port Data Order]"). Accepting it
    /// elsewhere would let a file assert an ordering the reader then ignores.
    #[test]
    fn two_port_data_order_is_refused_outside_two_port_files() {
        let error = read_touchstone_bytes(
            "three.ts",
            b"[Version] 2.0\n\
              # Hz S RI R 50\n\
              [Number of Ports] 3\n\
              [Two-Port Data Order] 21_12\n\
              [Number of Frequencies] 1\n\
              [Network Data]\n\
              1.0 0.11 0 0.12 0 0.13 0\n\
              0.21 0 0.22 0 0.23 0\n\
              0.31 0 0.32 0 0.33 0\n\
              [End]\n",
        )
        .expect_err("[Two-Port Data Order] is not permitted on a 3-port file");

        assert!(error.contains("Two-Port Data Order"), "{error}");
    }

    /// A passive reciprocal 3-port measures the same power from port *i* to
    /// port *j* as from *j* to *i*, so the imported matrix must be symmetric.
    /// A transposing reader keeps that property, which is exactly why this
    /// guard is paired with the position-naming fixtures above rather than
    /// standing in for them.
    #[test]
    fn a_reciprocal_three_port_imports_symmetrically() {
        let dataset = read_touchstone_bytes(
            "divider.s3p",
            b"!Passive reciprocal 3-port divider\n\
              # GHz S RI R 50\n\
              1.0 0.10 0.00 0.55 -0.05 0.45 -0.04\n\
              0.55 -0.05 0.20 0.00 0.35 -0.03\n\
              0.45 -0.04 0.35 -0.03 0.30 0.00\n",
        )
        .expect("a reciprocal 3-port file parses");

        for row in 1..=3 {
            for column in 1..=3 {
                let forward = entry(&dataset, row, column, 0);
                let reverse = entry(&dataset, column, row, 0);
                assert!(
                    (forward.0 - reverse.0).abs() < 1e-12 && (forward.1 - reverse.1).abs() < 1e-12,
                    "S{row}{column} {forward:?} != S{column}{row} {reverse:?}"
                );
                // Passive: no entry delivers more than the incident wave.
                assert!(forward.0.hypot(forward.1) <= 1.0, "S{row}{column}");
            }
        }
        assert_eq!(entry(&dataset, 1, 2, 0), (0.55, -0.05));
        assert_eq!(entry(&dataset, 1, 3, 0), (0.45, -0.04));
        assert_eq!(entry(&dataset, 2, 3, 0), (0.35, -0.03));
    }

    #[test]
    fn reads_v2_per_port_references_and_alternate_two_port_order() {
        let dataset = read_touchstone_bytes(
            "network.ts",
            b"[Version] 2.0\n[Number of Ports] 2\n[Number of Frequencies] 2\n[Two-Port Data Order] 12_21\n# MHz S RI R 50\n[Reference] 50 75\n[Network Data]\n1 0.1 0 0.2 0 0.3 0 0.4 0\n2 0.5 0 0.6 0 0.7 0 0.8 0\n[End]\n",
        )
        .expect("valid Touchstone v2");
        assert_eq!(dataset.point_count(), 2);
        assert_eq!(dataset.metadata["z0_ports"], "50,75");
        assert_eq!(dataset.get_signal("S12_RE").unwrap().data, [0.2, 0.6]);
        assert_eq!(dataset.get_signal("S21_RE").unwrap().data, [0.3, 0.7]);
    }

    #[test]
    fn rejects_unsupported_blocks_nonfinite_values_and_data_after_end() {
        for (name, source, expected) in [
            (
                "mixed.ts",
                "[Version] 2.0\n[Number of Ports] 2\n[Mixed-Mode Order] D1,2\n[End]\n",
                "mixed-mode",
            ),
            ("bad.s1p", "# Hz S RI R 50\n1 NaN 0\n", "finite"),
            (
                "after.ts",
                "[Version] 2.0\n[Number of Ports] 1\n[Network Data]\n1 0 0\n[End]\n2 0 0\n",
                "after [End]",
            ),
        ] {
            let error = read_touchstone_bytes(name, source.as_bytes()).expect_err("must reject");
            assert!(error.contains(expected), "{error}");
        }
    }
}
