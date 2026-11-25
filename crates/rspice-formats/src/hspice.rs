//! HSPICE output: the waveform files an analysis writes (`.tr#` transient, `.ac#` AC, `.sw#`
//! DC sweep) in the binary and ASCII forms `.option post` chooses, and the tables `.measure`
//! writes (`.mt#`, `.ma#`, `.ms#`).
//!
//! A waveform file opens with a header of fixed-width text fields: the column counts, the
//! output version, the title, and from byte 256 one type code per column, the column names and
//! the swept parameter's name, closed by `$&%#`. The values follow row by row, the scale first;
//! a scale value of 1e30 or more closes a table, and a swept analysis writes one table per sweep
//! value, each opening with that value. The binary form frames everything in records that state
//! their byte counts; the ASCII form wraps the header at 80 columns and writes the values as
//! fixed-width numbers with nothing between them. Version 9601 writes single precision, 2001
//! double precision, and 2013 a double-precision scale beside single-precision columns.

use crate::WaveformDomain;
use crate::numeric::{DecodedNumericDataset, DecodedNumericSignal};

/// Why an HSPICE file was refused, in words for the person who chose it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HspiceReadError(String);

impl HspiceReadError {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl std::fmt::Display for HspiceReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for HspiceReadError {}

/// One table of a waveform file: the whole analysis, or one value of its sweep.
#[derive(Debug)]
pub struct DecodedHspiceTable {
    /// The swept parameter and its value in this table, when the analysis was swept.
    pub sweep: Option<(String, f64)>,
    pub dataset: DecodedNumericDataset,
}

/// A waveform file, read.
#[derive(Debug)]
pub struct DecodedHspice {
    /// The title line of the netlist, as the file states it.
    pub title: String,
    pub tables: Vec<DecodedHspiceTable>,
}

/// A measurement table, read: one column per measurement, one row per sweep point or alter.
#[derive(Debug)]
pub struct DecodedHspiceMeasure {
    pub title: String,
    /// The measurements over `point`, the row number; a value HSPICE reports as `failed` is NaN.
    pub dataset: DecodedNumericDataset,
}

const HEADER_END: &[u8] = b"$&%#";
/// Where the column type codes start in the header.
const COLUMNS_AT: usize = 256;
/// A scale value at or above this closes a table: HSPICE writes 1e30, which single precision
/// stores as slightly more.
const TABLE_END: f64 = 9.0e29;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Layout {
    /// Versions 9007 and 9601.
    Single,
    /// Version 2001.
    Double,
    /// Version 2013: a double-precision scale beside single-precision columns.
    DoubleScale,
}

impl Layout {
    fn of(version: &str) -> Result<Self, HspiceReadError> {
        match version {
            "9007" | "9601" => Ok(Self::Single),
            "2001" => Ok(Self::Double),
            "2013" => Ok(Self::DoubleScale),
            other => Err(HspiceReadError::new(format!(
                "the header names output version '{other}'; this importer reads HSPICE versions \
                 9007, 9601, 2001 and 2013"
            ))),
        }
    }

    fn scale_bytes(self) -> usize {
        match self {
            Self::Single => 4,
            Self::Double | Self::DoubleScale => 8,
        }
    }

    fn column_bytes(self) -> usize {
        match self {
            Self::Single | Self::DoubleScale => 4,
            Self::Double => 8,
        }
    }

    /// The width of one number in the ASCII form: `0.12345E+01` beside `0.123456E+001`.
    fn ascii_width(self) -> usize {
        match self {
            Self::Single => 11,
            Self::Double | Self::DoubleScale => 13,
        }
    }
}

struct Column {
    name: String,
    kind: u32,
}

struct Header {
    title: String,
    layout: Layout,
    domain: WaveformDomain,
    /// The scale first, then the signals.
    columns: Vec<Column>,
    sweep: Option<String>,
}

impl Header {
    /// AC voltages and currents are written as a real and an imaginary part.
    fn is_complex(&self, column: &Column) -> bool {
        self.domain == WaveformDomain::Ac && matches!(column.kind, 1 | 8 | 15)
    }
}

/// Read an HSPICE waveform file, binary or ASCII.
pub fn decode_hspice(bytes: &[u8]) -> Result<DecodedHspice, HspiceReadError> {
    let (header, values) = match binary_byte_order(bytes) {
        Some(order) => {
            let (header, data) = binary_records(bytes, order)?;
            let header = parse_header(&header)?;
            let values = binary_values(&data, order, &header)?;
            (header, values)
        }
        None => {
            let (header, data) = ascii_parts(bytes)?;
            let header = parse_header(&header)?;
            let values = ascii_values(&data, header.layout)?;
            (header, values)
        }
    };
    let tables = split_tables(&header, values)?;
    Ok(DecodedHspice {
        title: header.title,
        tables,
    })
}

#[derive(Debug, Clone, Copy)]
enum ByteOrder {
    Little,
    Big,
}

impl ByteOrder {
    fn u32(self, bytes: [u8; 4]) -> u32 {
        match self {
            Self::Little => u32::from_le_bytes(bytes),
            Self::Big => u32::from_be_bytes(bytes),
        }
    }

    fn word(self, bytes: &[u8], at: usize) -> Option<u32> {
        let word: [u8; 4] = bytes.get(at..at.checked_add(4)?)?.try_into().ok()?;
        Some(self.u32(word))
    }

    fn value(self, bytes: &[u8], at: usize, width: usize) -> Option<f64> {
        let bytes = bytes.get(at..at.checked_add(width)?)?;
        Some(match (width, self) {
            (4, Self::Little) => f64::from(f32::from_le_bytes(bytes.try_into().ok()?)),
            (4, Self::Big) => f64::from(f32::from_be_bytes(bytes.try_into().ok()?)),
            (_, Self::Little) => f64::from_le_bytes(bytes.try_into().ok()?),
            (_, Self::Big) => f64::from_be_bytes(bytes.try_into().ok()?),
        })
    }
}

/// A binary file opens with a record header whose first and third words are 4: which byte order
/// reads them as 4 is the file's byte order.
fn binary_byte_order(bytes: &[u8]) -> Option<ByteOrder> {
    [ByteOrder::Little, ByteOrder::Big]
        .into_iter()
        .find(|order| order.word(bytes, 0) == Some(4) && order.word(bytes, 8) == Some(4))
}

/// The header text and the value bytes of a binary file, out of their records: each record is
/// four words (4, a count, 4, the body's length in bytes), the body, and the body's length again.
fn binary_records(bytes: &[u8], order: ByteOrder) -> Result<(Vec<u8>, Vec<u8>), HspiceReadError> {
    let mut header = Vec::new();
    let mut data = Vec::new();
    let mut header_done = false;
    let mut at = 0;
    let mut record = 0_usize;
    while at < bytes.len() {
        record += 1;
        let cut_short =
            || HspiceReadError::new(format!("the file is cut short inside record {record}"));
        let (Some(first), Some(third), Some(length)) = (
            order.word(bytes, at),
            order.word(bytes, at + 8),
            order.word(bytes, at + 12),
        ) else {
            return Err(cut_short());
        };
        if first != 4 || third != 4 {
            return Err(HspiceReadError::new(format!(
                "record {record} does not open as an HSPICE binary record does"
            )));
        }
        let length = usize::try_from(length).map_err(|_| cut_short())?;
        let body_start = at + 16;
        let body_end = body_start.checked_add(length).ok_or_else(cut_short)?;
        let body = bytes.get(body_start..body_end).ok_or_else(cut_short)?;
        let trailer = order.word(bytes, body_end).ok_or_else(cut_short)?;
        if usize::try_from(trailer).ok() != Some(length) {
            return Err(HspiceReadError::new(format!(
                "record {record} states {length} bytes at its start and {trailer} at its end"
            )));
        }
        at = body_end + 4;
        if header_done {
            data.extend_from_slice(body);
        } else {
            header.extend_from_slice(body);
            if let Some(end) = find(&header, HEADER_END) {
                header.truncate(end);
                header_done = true;
            }
        }
    }
    if !header_done {
        return Err(HspiceReadError::new(
            "the header never ends: the file has no `$&%#` after its column names",
        ));
    }
    Ok((header, data))
}

/// The header text and the value text of an ASCII file. The header is wrapped at 80 columns,
/// so its line breaks are not part of its fields; the values start on the line after `$&%#`.
fn ascii_parts(bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>), HspiceReadError> {
    let mut header = Vec::new();
    let mut end = None;
    for (index, &byte) in bytes.iter().enumerate() {
        if matches!(byte, b'\r' | b'\n') {
            continue;
        }
        header.push(byte);
        if header.ends_with(HEADER_END) {
            header.truncate(header.len() - HEADER_END.len());
            end = Some(index + 1);
            break;
        }
    }
    let Some(end) = end else {
        return Err(HspiceReadError::new(
            "the file is neither a binary HSPICE file nor an ASCII one: no `$&%#` closes a header",
        ));
    };
    let rest = &bytes[end..];
    let values_start = rest
        .iter()
        .position(|&byte| byte == b'\n')
        .map_or(rest.len(), |line_end| line_end + 1);
    let values = rest[values_start..]
        .iter()
        .copied()
        .filter(|byte| !matches!(byte, b'\r' | b'\n'))
        .collect();
    Ok((header, values))
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn latin1(bytes: &[u8]) -> String {
    bytes.iter().map(|&byte| char::from(byte)).collect()
}

fn parse_header(text: &[u8]) -> Result<Header, HspiceReadError> {
    let field = |from: usize, to: usize| latin1(text.get(from..to.min(text.len())).unwrap_or(&[]));
    let count = |from: usize, name: &str| -> Result<usize, HspiceReadError> {
        let value = field(from, from + 4);
        value.trim().parse::<usize>().map_err(|_| {
            HspiceReadError::new(format!(
                "the header's {name} count, '{}', is not a number",
                value.trim()
            ))
        })
    };
    let variables = count(0, "variable")?;
    let probes = count(4, "probe")?;
    let sweeps = count(8, "sweep")?;
    let version = field(16, 24);
    let version = version.trim();
    let version = version
        .char_indices()
        .rev()
        .nth(3)
        .map_or(version, |(at, _)| &version[at..]);
    let layout = Layout::of(version)?;
    if sweeps > 1 {
        return Err(HspiceReadError::new(format!(
            "the file sweeps {sweeps} parameters; this importer reads a file that sweeps one or none"
        )));
    }
    let columns = variables
        .checked_add(probes)
        .filter(|&columns| columns > 0)
        .ok_or_else(|| HspiceReadError::new("the header declares no columns"))?;
    let words = latin1(text.get(COLUMNS_AT..).unwrap_or(&[]));
    let mut words = words.split_whitespace();
    let mut kinds = Vec::new();
    for index in 0..columns {
        let word = words.next().ok_or_else(|| {
            HspiceReadError::new(format!(
                "the header declares {columns} columns but types only {index}"
            ))
        })?;
        kinds.push(word.parse::<u32>().map_err(|_| {
            HspiceReadError::new(format!(
                "the type of column {} is '{word}', not a number",
                index + 1
            ))
        })?);
    }
    let mut parsed = Vec::with_capacity(kinds.len());
    for (index, kind) in kinds.into_iter().enumerate() {
        let name = words.next().ok_or_else(|| {
            HspiceReadError::new(format!(
                "the header declares {columns} columns but names only {index}"
            ))
        })?;
        parsed.push(Column {
            name: closed_name(name),
            kind,
        });
    }
    let sweep = if sweeps == 1 {
        Some(
            words
                .next()
                .ok_or_else(|| HspiceReadError::new("the header names no swept parameter"))?
                .to_owned(),
        )
    } else {
        None
    };
    let domain = match parsed[0].kind {
        1 => WaveformDomain::Transient,
        2 => WaveformDomain::Ac,
        _ => WaveformDomain::DcSweep,
    };
    Ok(Header {
        title: field(24, 88).trim().to_owned(),
        layout,
        domain,
        columns: parsed,
        sweep,
    })
}

/// HSPICE leaves the closing parenthesis off a name such as `v(out`.
fn closed_name(name: &str) -> String {
    let open = name.matches('(').count();
    let close = name.matches(')').count();
    let mut name = name.to_owned();
    for _ in close..open {
        name.push(')');
    }
    name
}

struct Cursor<'a> {
    data: &'a [u8],
    at: usize,
    order: ByteOrder,
}

impl Cursor<'_> {
    fn next(&mut self, width: usize) -> Result<f64, HspiceReadError> {
        let value = self
            .order
            .value(self.data, self.at, width)
            .ok_or_else(ends_inside_table)?;
        self.at += width;
        Ok(value)
    }
}

/// Every value of a binary file, in file order. The stream is walked as the tables lay it out,
/// since in version 2013 a scale value is wider than a column value: a sweep value when the
/// analysis was swept, then rows until a scale value closes the table.
fn binary_values(
    data: &[u8],
    order: ByteOrder,
    header: &Header,
) -> Result<Vec<f64>, HspiceReadError> {
    let (scale_width, column_width) = (header.layout.scale_bytes(), header.layout.column_bytes());
    let row_parts = row_parts(header);
    let mut cursor = Cursor { data, at: 0, order };
    let mut values = Vec::with_capacity(data.len() / column_width);
    loop {
        if header.sweep.is_some() {
            values.push(cursor.next(column_width)?);
        }
        loop {
            let scale = cursor.next(scale_width)?;
            values.push(scale);
            if scale >= TABLE_END {
                break;
            }
            for _ in 0..row_parts {
                values.push(cursor.next(column_width)?);
            }
        }
        // A swept file's tables follow one another to its end; an unswept file holds one.
        if header.sweep.is_none() || cursor.at >= data.len() {
            break;
        }
    }
    Ok(values)
}

/// Every value of an ASCII file, in file order: fixed-width numbers with nothing between them.
fn ascii_values(data: &[u8], layout: Layout) -> Result<Vec<f64>, HspiceReadError> {
    let data = data.trim_ascii_end();
    let width = layout.ascii_width();
    let mut values = Vec::with_capacity(data.len() / width);
    for (index, chunk) in data.chunks(width).enumerate() {
        let value = std::str::from_utf8(chunk)
            .ok()
            .and_then(|text| text.trim().parse::<f64>().ok())
            .ok_or_else(|| {
                HspiceReadError::new(format!(
                    "value {} of the data, '{}', is not a number {width} characters wide",
                    index + 1,
                    latin1(chunk)
                ))
            })?;
        values.push(value);
    }
    Ok(values)
}

fn ends_inside_table() -> HspiceReadError {
    HspiceReadError::new("the file ends inside a table, before the value that closes it")
}

/// The values one row holds after its scale value.
fn row_parts(header: &Header) -> usize {
    header.columns[1..]
        .iter()
        .map(|column| if header.is_complex(column) { 2 } else { 1 })
        .sum()
}

fn split_tables(
    header: &Header,
    values: Vec<f64>,
) -> Result<Vec<DecodedHspiceTable>, HspiceReadError> {
    let mut stream = values.into_iter();
    let mut tables = Vec::new();
    loop {
        let sweep = match &header.sweep {
            Some(name) => match stream.next() {
                Some(value) => Some((name.clone(), value)),
                None if !tables.is_empty() => break,
                None => return Err(ends_inside_table()),
            },
            None => None,
        };
        let mut coordinate = Vec::new();
        let mut signals: Vec<DecodedNumericSignal> = header.columns[1..]
            .iter()
            .map(|column| DecodedNumericSignal {
                name: column.name.clone(),
                real: Vec::new(),
                imag: header.is_complex(column).then(Vec::new),
                unit: signal_unit(column.kind),
            })
            .collect();
        loop {
            let scale = stream.next().ok_or_else(ends_inside_table)?;
            if scale >= TABLE_END {
                break;
            }
            coordinate.push(scale);
            for signal in &mut signals {
                signal
                    .real
                    .push(stream.next().ok_or_else(ends_inside_table)?);
                if let Some(imag) = &mut signal.imag {
                    imag.push(stream.next().ok_or_else(ends_inside_table)?);
                }
            }
        }
        if coordinate.is_empty() {
            return Err(HspiceReadError::new(match &sweep {
                Some((name, value)) => format!("the table for {name} = {value} holds no rows"),
                None => "the file holds no rows".to_owned(),
            }));
        }
        tables.push(DecodedHspiceTable {
            sweep,
            dataset: DecodedNumericDataset {
                domain: header.domain,
                coordinate_name: header.columns[0].name.clone(),
                coordinate_unit: match header.domain {
                    WaveformDomain::Transient => Some("s".to_owned()),
                    WaveformDomain::Ac => Some("Hz".to_owned()),
                    WaveformDomain::DcSweep => None,
                },
                coordinate,
                signals,
            },
        });
        if header.sweep.is_none() {
            break;
        }
    }
    Ok(tables)
}

/// A voltage column is type 1 and a current 8 (a branch) or 15 (a terminal).
fn signal_unit(kind: u32) -> Option<String> {
    match kind {
        1 => Some("V".to_owned()),
        8 | 15 => Some("A".to_owned()),
        _ => None,
    }
}

/// Read an HSPICE measurement table (`.mt#`, `.ma#`, `.ms#`): a `DATA1` line, a `.TITLE` line,
/// the measurement names, then their values, one row per sweep point or alter, the rows wrapped
/// across as many lines as they need.
pub fn decode_hspice_measure(bytes: &[u8]) -> Result<DecodedHspiceMeasure, HspiceReadError> {
    let text = latin1(bytes);
    let mut lines = text.lines();
    let first = lines.next().unwrap_or_default();
    if !first
        .trim_start()
        .trim_start_matches('$')
        .starts_with("DATA1")
    {
        return Err(HspiceReadError::new(
            "the file does not open with the DATA1 line of an HSPICE measurement table",
        ));
    }
    let mut title = String::new();
    let mut words = Vec::new();
    for line in lines {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix(".TITLE") {
            title = rest.trim().trim_matches('\'').trim().to_owned();
            continue;
        }
        words.extend(trimmed.split_whitespace().map(str::to_owned));
    }
    let names = words
        .iter()
        .take_while(|word| measured_value(word).is_none())
        .cloned()
        .collect::<Vec<_>>();
    if names.is_empty() {
        return Err(HspiceReadError::new("the table names no measurements"));
    }
    let values = &words[names.len()..];
    if values.is_empty() || values.len() % names.len() != 0 {
        return Err(HspiceReadError::new(format!(
            "the table names {} measurements but holds {} values, which is not a whole number \
             of rows",
            names.len(),
            values.len()
        )));
    }
    let rows = values.len() / names.len();
    let mut signals = names
        .into_iter()
        .map(|name| DecodedNumericSignal {
            name,
            real: Vec::with_capacity(rows),
            imag: None,
            unit: None,
        })
        .collect::<Vec<_>>();
    let columns = signals.len();
    for (index, word) in values.iter().enumerate() {
        let value = measured_value(word).ok_or_else(|| {
            HspiceReadError::new(format!(
                "value {} of the table, '{word}', is neither a number nor `failed`",
                index + 1
            ))
        })?;
        signals[index % columns].real.push(value);
    }
    Ok(DecodedHspiceMeasure {
        title,
        dataset: DecodedNumericDataset {
            domain: WaveformDomain::DcSweep,
            coordinate_name: "point".to_owned(),
            coordinate_unit: Some("1".to_owned()),
            coordinate: (0..rows).map(|row| row as f64).collect(),
            signals,
        },
    })
}

/// A value of a measurement table: a number, or NaN where HSPICE reports the measurement
/// `failed`.
fn measured_value(word: &str) -> Option<f64> {
    if word.eq_ignore_ascii_case("failed") {
        return Some(f64::NAN);
    }
    // A name such as `inf` or `nan` is a measurement's name, not its value.
    if !word.starts_with(|first: char| first.is_ascii_digit() || matches!(first, '-' | '+' | '.')) {
        return None;
    }
    word.parse::<f64>().ok()
}

#[cfg(test)]
#[path = "hspice/tests.rs"]
mod tests;
