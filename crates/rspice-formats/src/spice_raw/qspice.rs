//! QSPICE's `.qraw`: a rawfile with QSPICE's own header lines beside the usual ones, every value
//! in double precision whatever its flags say, and two layouts of its own. A complex plot writes
//! each point's frequency once, as a real value, before the point's complex values; a stepped
//! plot states each run's parameter values in `parameter` columns rather than in a log.
//!
//! Each plot is restated as the rawfile it would be without those differences and read by the
//! rawfile reader, and each run's parameter columns become the run's step values.

use super::{DecodedRawPlot, RawReadError};

/// Read every analysis plot of a `.qraw`, in file order: each run of a stepped plot is a dataset
/// named by its parameter values.
pub fn decode_qspice_raw_plots(
    bytes: &[u8],
    limits: rspice_core::ResourceLimits,
) -> Result<Vec<DecodedRawPlot>, RawReadError> {
    let mut restated = Vec::with_capacity(bytes.len());
    let mut parameters = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        if bytes[at..].iter().all(u8::is_ascii_whitespace) {
            break;
        }
        let plot = restate_plot(&bytes[at..], &mut restated)?;
        parameters.extend(plot.parameters);
        at += plot.consumed;
    }
    let mut decoded = super::decode_spice_raw_plots_with_log(&restated, None, limits)?;
    for plot in &mut decoded {
        let Some(step) = &mut plot.step else {
            continue;
        };
        let mut coordinate = Vec::new();
        plot.dataset.signals.retain(|signal| {
            if !parameters.contains(&signal.name) {
                return true;
            }
            if let Some(&value) = signal.real.first() {
                coordinate.push((signal.name.clone(), value));
            }
            false
        });
        step.coordinate = coordinate;
    }
    Ok(decoded)
}

struct Restated {
    /// The source bytes the plot took.
    consumed: usize,
    /// The names of its `parameter` columns.
    parameters: Vec<String>,
}

fn layout(message: impl Into<String>) -> RawReadError {
    RawReadError::Column(message.into())
}

/// One plot of `source`, from its first header line, restated onto `out`.
fn restate_plot(source: &[u8], out: &mut Vec<u8>) -> Result<Restated, RawReadError> {
    let mut lines = Vec::new();
    let mut at = 0;
    let binary = loop {
        let Some(length) = source[at..].iter().position(|&byte| byte == b'\n') else {
            return Err(layout(
                "the header ends without a 'Binary:' or 'Values:' line, so the file has no values",
            ));
        };
        let raw = &source[at..at + length];
        at += length + 1;
        let raw = raw.strip_suffix(b"\r").unwrap_or(raw);
        // QSPICE writes names and titles in the system code page; a byte that is not UTF-8 is
        // read as Latin-1.
        let line = match std::str::from_utf8(raw) {
            Ok(line) => line.to_owned(),
            Err(_) => raw.iter().map(|&byte| char::from(byte)).collect(),
        };
        match line.trim() {
            "Binary:" => break true,
            "Values:" => break false,
            _ => lines.push(line),
        }
    };
    let field = |key: &str| {
        lines.iter().find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.trim()
                .eq_ignore_ascii_case(key)
                .then(|| value.trim().to_owned())
        })
    };
    let flags = field("Flags").unwrap_or_default();
    let complex = flags
        .split_whitespace()
        .any(|flag| flag.eq_ignore_ascii_case("complex"));
    let variables = field("No. Variables")
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|&variables| variables > 0)
        .ok_or_else(|| layout("the header states no variable count"))?;
    let stated_points = field("No. Points")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0);
    let parameters = lines
        .iter()
        .skip_while(|line| !line.trim().eq_ignore_ascii_case("Variables:"))
        .skip(1)
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let (_, name, kind) = (parts.next()?, parts.next()?, parts.next()?);
            kind.eq_ignore_ascii_case("parameter")
                .then(|| name.to_owned())
        })
        .collect::<Vec<_>>();

    let fastaccess = flags
        .split_whitespace()
        .any(|flag| flag.eq_ignore_ascii_case("fastaccess"));
    let (values, consumed, points, qspice_rows) = if binary {
        let rows = binary_rows(&source[at..], variables, stated_points, complex)?;
        let payload = &source[at..at + rows.length];
        let values = if rows.qspice && complex {
            restate_complex(payload, rows.points, variables, fastaccess)
        } else {
            payload.to_vec()
        };
        (values, at + rows.length, rows.points, rows.qspice)
    } else {
        let (values, end, points) = restate_ascii(&source[at..], complex);
        // A point count the header states stands; one it leaves out is the count of points read.
        let points = if stated_points > 0 {
            stated_points
        } else {
            points
        };
        (values, at + end, points, false)
    };

    for line in &lines {
        let restated = match line.split_once(':') {
            Some((key, value)) if qspice_rows && key.trim().eq_ignore_ascii_case("Flags") => {
                let mut flags = value.split_whitespace().collect::<Vec<_>>();
                if !flags.iter().any(|flag| flag.eq_ignore_ascii_case("double")) {
                    flags.push("double");
                }
                format!("Flags: {}", flags.join(" "))
            }
            Some((key, _)) if key.trim().eq_ignore_ascii_case("No. Points") => {
                format!("No. Points: {points}")
            }
            _ => line.clone(),
        };
        out.extend_from_slice(restated.as_bytes());
        out.push(b'\n');
    }
    out.extend_from_slice(if binary { b"Binary:\n" } else { b"Values:\n" });
    out.extend_from_slice(&values);
    Ok(Restated {
        consumed,
        parameters,
    })
}

/// Where a binary plot's values end, and in which layout they are written.
struct BinaryRows {
    points: usize,
    length: usize,
    /// QSPICE's own rows: every value a double, a complex plot's frequency written once.
    qspice: bool,
}

/// Find the layout that ends a binary plot exactly where the file or the next plot begins:
/// QSPICE's own first, then the two a rawfile uses (which a file that only carries the `.qraw`
/// name may), which the rawfile reader then reads as they are.
fn binary_rows(
    source: &[u8],
    variables: usize,
    stated_points: usize,
    complex: bool,
) -> Result<BinaryRows, RawReadError> {
    let others = variables - 1;
    let qspice_row = if complex {
        others
            .checked_mul(16)
            .and_then(|bytes| bytes.checked_add(8))
    } else {
        variables.checked_mul(8)
    }
    .ok_or_else(|| layout("the header states more variables than a row can hold"))?;
    let rawfile_rows = if complex {
        [
            variables.checked_mul(16),
            others
                .checked_mul(8)
                .and_then(|bytes| bytes.checked_add(16)),
        ]
    } else {
        [
            variables.checked_mul(8),
            others.checked_mul(4).and_then(|bytes| bytes.checked_add(8)),
        ]
    };
    // QSPICE states the count once the run ends; a file it is still writing states none.
    let points = if stated_points > 0 {
        stated_points
    } else if source.len().is_multiple_of(qspice_row) {
        source.len() / qspice_row
    } else {
        0
    };
    let fits = |row: usize| {
        let length = points.checked_mul(row).filter(|&length| length > 0)?;
        let rest = source.get(length..)?;
        (rest.is_empty() || rest.starts_with(b"Title:") || rest.starts_with(b"Plotname:"))
            .then_some(length)
    };
    if let Some(length) = fits(qspice_row) {
        return Ok(BinaryRows {
            points,
            length,
            qspice: true,
        });
    }
    if let Some(length) = rawfile_rows.into_iter().flatten().find_map(fits) {
        return Ok(BinaryRows {
            points,
            length,
            qspice: false,
        });
    }
    Err(layout(format!(
        "the values are cut short: {points} points of {variables} variables need {} bytes, and \
         the file holds {}",
        points.saturating_mul(qspice_row),
        source.len()
    )))
}

/// A complex plot in QSPICE's rows restated as a rawfile's: each frequency, written once as a
/// real double, becomes a complex double with no imaginary part. A `fastaccess` plot holds its
/// columns one after another rather than its rows.
fn restate_complex(payload: &[u8], points: usize, variables: usize, fastaccess: bool) -> Vec<u8> {
    let mut values = Vec::with_capacity(points * variables * 16);
    if fastaccess {
        let (frequencies, rest) = payload.split_at(points * 8);
        for frequency in frequencies.chunks_exact(8) {
            values.extend_from_slice(frequency);
            values.extend_from_slice(&[0; 8]);
        }
        values.extend_from_slice(rest);
    } else {
        for row in payload.chunks_exact(8 + (variables - 1) * 16) {
            values.extend_from_slice(&row[..8]);
            values.extend_from_slice(&[0; 8]);
            values.extend_from_slice(&row[8..]);
        }
    }
    values
}

/// The ASCII values of one plot, up to the next plot's header or the end of the file, and the
/// points they hold. A point's first line opens with its index; a complex plot gives its
/// frequency there as a real value, which a rawfile writes as a complex one.
fn restate_ascii(source: &[u8], complex: bool) -> (Vec<u8>, usize, usize) {
    let mut end = 0;
    let mut values = Vec::new();
    let mut points = 0;
    while end < source.len() {
        let length = source[end..]
            .iter()
            .position(|&byte| byte == b'\n')
            .map_or(source.len() - end, |length| length + 1);
        let line = &source[end..end + length];
        if line.starts_with(b"Title:") || line.starts_with(b"Plotname:") {
            break;
        }
        end += length;
        let text = String::from_utf8_lossy(line);
        let text = text.trim_end();
        let mut words = text.split_whitespace();
        let (Some(first), second) = (words.next(), words.next()) else {
            continue;
        };
        let opens_point = second.is_some() && first.parse::<usize>().is_ok();
        points += usize::from(opens_point);
        values.extend_from_slice(text.as_bytes());
        if complex && opens_point && words.next().is_none() && !text.contains(',') {
            values.extend_from_slice(b",0");
        }
        values.push(b'\n');
    }
    (values, end, points)
}

#[cfg(test)]
#[path = "qspice_tests.rs"]
mod tests;
