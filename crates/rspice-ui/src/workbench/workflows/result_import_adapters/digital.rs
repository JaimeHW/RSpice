//! VCD and FST digital event-trace import adapters.

use super::*;

#[derive(Debug)]
struct DigitalSignal {
    name: String,
    width: usize,
}

#[derive(Debug)]
struct DigitalEvent {
    tick: u64,
    signal: usize,
    value: f64,
}

pub(in crate::workbench::workflows) fn parse_vcd(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let text = std::str::from_utf8(bytes)
        .map_err(|error| adapter_error(format, format_args!("source is not UTF-8: {error}")))?;
    let mut timescale = None;
    let mut scopes = Vec::new();
    let mut signals: Vec<DigitalSignal> = Vec::new();
    let mut identifiers: HashMap<String, usize> = HashMap::new();
    let mut aliases = Vec::new();
    let mut events = Vec::new();
    let mut current_tick = 0_u64;
    let mut in_definitions = true;
    let mut in_dumpvars = false;
    let mut directive = String::new();

    for (line_index, raw_line) in text.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "$dumpvars" || line == "$dumpon" {
            in_dumpvars = true;
            continue;
        }
        if in_dumpvars && line == "$end" {
            in_dumpvars = false;
            continue;
        }
        if line.starts_with('$') && !line.contains("$end") {
            directive.clear();
            directive.push_str(line);
            continue;
        }
        let owned;
        let line = if !directive.is_empty() {
            directive.push(' ');
            directive.push_str(line);
            owned = std::mem::take(&mut directive);
            owned.as_str()
        } else {
            line
        };
        if line.starts_with("$timescale") {
            let body = line
                .trim_start_matches("$timescale")
                .trim_end_matches("$end")
                .trim();
            if timescale
                .replace(parse_vcd_timescale(body, format)?)
                .is_some()
            {
                return Err(adapter_error(
                    format,
                    "VCD declares timescale more than once",
                ));
            }
        } else if line.starts_with("$scope") {
            let fields = line.split_whitespace().collect::<Vec<_>>();
            if fields.len() < 4 || fields.last() != Some(&"$end") {
                return Err(adapter_error(
                    format,
                    format_args!("malformed $scope at line {line_number}"),
                ));
            }
            validate_name(format, "scope", fields[2])?;
            scopes.push(fields[2].to_owned());
        } else if line.starts_with("$upscope") {
            if scopes.pop().is_none() {
                return Err(adapter_error(
                    format,
                    format_args!("unbalanced $upscope at line {line_number}"),
                ));
            }
        } else if line.starts_with("$var") {
            let fields = line.split_whitespace().collect::<Vec<_>>();
            if fields.len() < 6 || fields.last() != Some(&"$end") {
                return Err(adapter_error(
                    format,
                    format_args!("malformed $var at line {line_number}"),
                ));
            }
            let width = fields[2].parse::<usize>().map_err(|_| {
                adapter_error(
                    format,
                    format_args!("invalid $var width at line {line_number}"),
                )
            })?;
            if width == 0 || width > 53 {
                return Err(adapter_error(
                    format,
                    format_args!(
                        "VCD variable '{}' width {width} cannot be represented exactly as f64",
                        fields[4]
                    ),
                ));
            }
            let mut name = scopes.join(".");
            if !name.is_empty() {
                name.push('.');
            }
            name.push_str(fields[4]);
            validate_name(format, "signal", &name)?;
            let identifier = fields[3].to_owned();
            let signal_index = if let Some(existing) = identifiers.get(&identifier).copied() {
                if signals[existing].width != width {
                    return Err(adapter_error(format, "VCD alias changes declared width"));
                }
                aliases.push((existing, name));
                existing
            } else {
                if signals.len() >= MAX_RESULT_COLUMNS - 1 {
                    return Err(adapter_error(format, "VCD signal-count limit exceeded"));
                }
                let index = signals.len();
                signals.push(DigitalSignal { name, width });
                identifiers.insert(identifier, index);
                index
            };
            let _ = signal_index;
        } else if line.starts_with("$enddefinitions") {
            in_definitions = false;
        } else if line.starts_with('$') {
            continue;
        } else if in_definitions {
            return Err(adapter_error(
                format,
                format_args!("unexpected VCD definition text at line {line_number}"),
            ));
        } else if let Some(tick) = line.strip_prefix('#') {
            current_tick = tick.parse::<u64>().map_err(|_| {
                adapter_error(
                    format,
                    format_args!("invalid timestamp at line {line_number}"),
                )
            })?;
        } else {
            let (identifier, value) = parse_vcd_change(line, format, line_number)?;
            let signal = identifiers.get(identifier).copied().ok_or_else(|| {
                adapter_error(
                    format,
                    format_args!("line {line_number} changes unknown identifier '{identifier}'"),
                )
            })?;
            if events.len() >= MAX_RESULT_VALUES {
                return Err(adapter_error(format, "VCD event-count limit exceeded"));
            }
            if value > (2_f64.powi(signals[signal].width as i32) - 1.0) {
                return Err(adapter_error(
                    format,
                    format_args!("line {line_number} value exceeds declared width"),
                ));
            }
            events.push(DigitalEvent {
                tick: current_tick,
                signal,
                value,
            });
        }
    }
    if !directive.is_empty() {
        return Err(adapter_error(
            format,
            "truncated VCD directive at end of file",
        ));
    }
    if in_definitions {
        return Err(adapter_error(format, "VCD is missing $enddefinitions"));
    }
    let mut parsed = digital_events_to_dataset(
        format,
        timescale.ok_or_else(|| adapter_error(format, "VCD is missing $timescale"))?,
        signals,
        events,
    )?;
    append_digital_aliases(format, &mut parsed, aliases)?;
    Ok(parsed)
}

fn parse_vcd_timescale(raw: &str, format: ResultImportFormat) -> Result<f64, String> {
    let compact = raw.split_whitespace().collect::<String>();
    let split = compact
        .find(|character: char| !character.is_ascii_digit())
        .ok_or_else(|| adapter_error(format, format_args!("invalid VCD timescale '{raw}'")))?;
    let magnitude = compact[..split]
        .parse::<u32>()
        .map_err(|_| adapter_error(format, format_args!("invalid VCD timescale '{raw}'")))?;
    if !matches!(magnitude, 1 | 10 | 100) {
        return Err(adapter_error(
            format,
            "VCD timescale magnitude must be 1, 10, or 100",
        ));
    }
    let scale = match compact[split..].to_ascii_lowercase().as_str() {
        "s" => 1.0,
        "ms" => 1e-3,
        "us" => 1e-6,
        "ns" => 1e-9,
        "ps" => 1e-12,
        "fs" => 1e-15,
        unit => {
            return Err(adapter_error(
                format,
                format_args!("unsupported VCD timescale unit '{unit}'"),
            ));
        }
    };
    Ok(f64::from(magnitude) * scale)
}

fn parse_vcd_change(
    line: &str,
    format: ResultImportFormat,
    line_number: usize,
) -> Result<(&str, f64), String> {
    let first = line.as_bytes()[0] as char;
    if matches!(first, '0' | '1' | 'x' | 'X' | 'z' | 'Z') {
        if matches!(first, 'x' | 'X' | 'z' | 'Z') {
            return Err(adapter_error(
                format,
                format_args!(
                    "line {line_number} contains X/Z state that cannot be losslessly mapped to an analog trace"
                ),
            ));
        }
        let identifier = line.get(1..).unwrap_or("").trim();
        if identifier.is_empty() {
            return Err(adapter_error(
                format,
                format_args!("missing identifier at line {line_number}"),
            ));
        }
        return Ok((identifier, if first == '1' { 1.0 } else { 0.0 }));
    }
    let mut fields = line.split_whitespace();
    let value = fields.next().unwrap_or("");
    let identifier = fields.next().unwrap_or("");
    if identifier.is_empty() || fields.next().is_some() {
        return Err(adapter_error(
            format,
            format_args!("malformed value change at line {line_number}"),
        ));
    }
    if let Some(bits) = value.strip_prefix(['b', 'B']) {
        return Ok((identifier, logic_bits_to_f64(bits.as_bytes(), format)?));
    }
    if let Some(real) = value.strip_prefix(['r', 'R']) {
        let real = real.parse::<f64>().map_err(|_| {
            adapter_error(
                format,
                format_args!("invalid real change at line {line_number}"),
            )
        })?;
        if !real.is_finite() {
            return Err(adapter_error(
                format,
                format_args!("non-finite real change at line {line_number}"),
            ));
        }
        return Ok((identifier, real));
    }
    Err(adapter_error(
        format,
        format_args!("unsupported value change at line {line_number}"),
    ))
}

fn logic_bits_to_f64(bits: &[u8], format: ResultImportFormat) -> Result<f64, String> {
    if bits.is_empty() || bits.len() > 53 {
        return Err(adapter_error(
            format,
            "digital vector is empty or wider than 53 exact bits",
        ));
    }
    let mut value = 0_u64;
    for bit in bits {
        value = value
            .checked_mul(2)
            .ok_or_else(|| adapter_error(format, "digital vector overflow"))?;
        match bit {
            b'0' => {}
            b'1' => value += 1,
            _ => {
                return Err(adapter_error(
                    format,
                    "digital vector contains X/Z/U/W/- state that cannot be losslessly mapped to an analog trace",
                ));
            }
        }
    }
    Ok(value as f64)
}

#[derive(Debug, Clone, Copy)]
struct FstHeaderPreflight {
    var_count: usize,
    max_handle: usize,
    value_change_sections: usize,
}

#[derive(Debug)]
struct FstGeometryPreflight {
    widths: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
struct FstDataPreflight {
    block_type: u8,
    section_start: usize,
    section_end: usize,
    section_length: usize,
    memory_required: usize,
}

/// Validate every allocation-relevant FST framing field before handing the
/// bytes to `fst-reader`. That crate trusts several sizes with `Vec` capacity
/// reservations and contains arithmetic assertions intended for trusted
/// files, so the public import boundary cannot delegate this job to it.
fn preflight_fst(bytes: &[u8], format: ResultImportFormat) -> Result<FstGeometryPreflight, String> {
    if bytes.len() as u64 > MAX_RESULT_DATASET_BYTES {
        return Err(adapter_error(format, "FST input exceeds the byte limit"));
    }
    let mut cursor = 0_usize;
    let mut block_count = 0_usize;
    let mut header = None;
    let mut geometry = None;
    let mut hierarchy_seen = false;
    let mut blackout_seen = false;
    let mut data_sections = Vec::new();
    let mut terminated = false;

    while cursor < bytes.len() {
        block_count = block_count
            .checked_add(1)
            .ok_or_else(|| adapter_error(format, "FST block-count accounting overflow"))?;
        if block_count > MAX_FST_TOP_LEVEL_BLOCKS {
            return Err(adapter_error(
                format,
                "FST top-level block-count limit exceeded",
            ));
        }
        let block_offset = cursor;
        let block_type = *bytes
            .get(cursor)
            .ok_or_else(|| adapter_error(format, "truncated FST block type"))?;
        cursor += 1;
        let section_start = cursor;
        let section_length_u64 = fst_be_u64(bytes, section_start, format, "section length")?;

        if block_type == 255 && section_length_u64 == 0 {
            cursor = section_start + 8;
            if cursor != bytes.len() {
                return Err(adapter_error(
                    format,
                    "FST contains trailing bytes after its end marker",
                ));
            }
            terminated = true;
            break;
        }
        if section_length_u64 < 8 {
            return Err(adapter_error(
                format,
                format_args!(
                    "FST block at byte {block_offset} declares a section shorter than its length field"
                ),
            ));
        }
        let section_length = fst_bounded_size(
            section_length_u64,
            format,
            format_args!("block at byte {block_offset} section"),
        )?;
        let section_end = section_start
            .checked_add(section_length)
            .ok_or_else(|| adapter_error(format, "FST section offset overflow"))?;
        if section_end > bytes.len() {
            return Err(adapter_error(
                format,
                format_args!("truncated FST block at byte {block_offset}"),
            ));
        }

        match block_type {
            0 => {
                if header.is_some() {
                    return Err(adapter_error(format, "FST repeats its header block"));
                }
                if section_length_u64 != FST_HEADER_SECTION_BYTES {
                    return Err(adapter_error(
                        format,
                        format_args!(
                            "FST header length is {section_length_u64}; expected {FST_HEADER_SECTION_BYTES}"
                        ),
                    ));
                }
                let body = section_start + 8;
                let start_time = fst_be_u64(bytes, body, format, "header start time")?;
                let end_time = fst_be_u64(bytes, body + 8, format, "header end time")?;
                if end_time < start_time {
                    return Err(adapter_error(
                        format,
                        "FST header end time precedes its start time",
                    ));
                }
                let endian_marker: [u8; 8] = bytes
                    .get(body + 16..body + 24)
                    .ok_or_else(|| {
                        adapter_error(format, "truncated FST floating-point endian marker")
                    })?
                    .try_into()
                    .expect("eight-byte slice");
                if endian_marker != std::f64::consts::E.to_le_bytes()
                    && endian_marker != std::f64::consts::E.to_be_bytes()
                {
                    return Err(adapter_error(
                        format,
                        "FST header has an invalid floating-point endian marker",
                    ));
                }
                let _scope_count = fst_count(
                    fst_be_u64(bytes, body + 32, format, "header scope count")?,
                    MAX_RESULT_COLUMNS,
                    format,
                    "scope",
                )?;
                let var_count = fst_count(
                    fst_be_u64(bytes, body + 40, format, "header variable count")?,
                    MAX_RESULT_COLUMNS - 1,
                    format,
                    "variable",
                )?;
                let max_handle = fst_count(
                    fst_be_u64(bytes, body + 48, format, "header signal count")?,
                    MAX_RESULT_COLUMNS - 1,
                    format,
                    "unique signal",
                )?;
                let value_change_sections = fst_count(
                    fst_be_u64(bytes, body + 56, format, "header data-block count")?,
                    MAX_FST_TOP_LEVEL_BLOCKS,
                    format,
                    "data block",
                )?;
                if var_count == 0
                    || max_handle == 0
                    || max_handle > var_count
                    || value_change_sections == 0
                {
                    return Err(adapter_error(
                        format,
                        "FST header declares inconsistent scope, variable, signal, or data-block counts",
                    ));
                }
                header = Some(FstHeaderPreflight {
                    var_count,
                    max_handle,
                    value_change_sections,
                });
            }
            1 | 5 | 8 => {
                if section_length < 32 {
                    return Err(adapter_error(format, "truncated FST value-change header"));
                }
                let memory_required = fst_bounded_size(
                    fst_be_u64(
                        bytes,
                        section_start + 24,
                        format,
                        "value-change allocation size",
                    )?,
                    format,
                    "value-change allocation",
                )?;
                data_sections.push(FstDataPreflight {
                    block_type,
                    section_start,
                    section_end,
                    section_length,
                    memory_required,
                });
            }
            2 => {
                if blackout_seen {
                    return Err(adapter_error(format, "FST repeats its blackout block"));
                }
                preflight_fst_blackout(bytes, section_start, section_end, format)?;
                blackout_seen = true;
            }
            3 => {
                if geometry.is_some() {
                    return Err(adapter_error(format, "FST repeats its geometry block"));
                }
                geometry = Some(preflight_fst_geometry(
                    bytes,
                    section_start,
                    section_end,
                    section_length,
                    format,
                )?);
            }
            4 | 6 | 7 => {
                if hierarchy_seen {
                    return Err(adapter_error(format, "FST repeats its hierarchy block"));
                }
                preflight_fst_hierarchy(
                    bytes,
                    block_type,
                    section_start,
                    section_end,
                    section_length,
                    format,
                )?;
                hierarchy_seen = true;
            }
            254 => {
                if section_length < 16 {
                    return Err(adapter_error(
                        format,
                        "truncated FST whole-file gzip wrapper",
                    ));
                }
                let expanded = fst_bounded_size(
                    fst_be_u64(
                        bytes,
                        section_start + 8,
                        format,
                        "gzip wrapper expanded size",
                    )?,
                    format,
                    "gzip wrapper expanded allocation",
                )?;
                return Err(adapter_error(
                    format,
                    format_args!(
                        "whole-file gzip-wrapped FST ({expanded} declared expanded bytes) is rejected because nested framing cannot be preflighted before fst-reader decompresses it"
                    ),
                ));
            }
            255 => {}
            other => {
                return Err(adapter_error(
                    format,
                    format_args!("unknown FST top-level block type {other}"),
                ));
            }
        }
        cursor = section_end;
    }

    if !terminated && cursor != bytes.len() {
        return Err(adapter_error(
            format,
            "FST framing did not end at the input boundary",
        ));
    }
    let header = header.ok_or_else(|| adapter_error(format, "FST header block is missing"))?;
    let geometry =
        geometry.ok_or_else(|| adapter_error(format, "FST geometry block is missing"))?;
    if !hierarchy_seen {
        return Err(adapter_error(format, "FST hierarchy block is missing"));
    }
    if geometry.widths.len() != header.max_handle {
        return Err(adapter_error(
            format,
            "FST geometry signal count disagrees with its header",
        ));
    }
    if data_sections.len() != header.value_change_sections {
        return Err(adapter_error(
            format,
            "FST data-block count disagrees with its header",
        ));
    }
    if header.var_count < geometry.widths.len() {
        return Err(adapter_error(
            format,
            "FST variable count is smaller than its unique signal count",
        ));
    }
    for section in data_sections {
        preflight_fst_data_section(bytes, section, &geometry.widths, format)?;
    }
    Ok(geometry)
}

#[cfg(test)]
pub(super) fn preflight_fst_for_test(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<(), String> {
    preflight_fst(bytes, format).map(drop)
}

fn fst_be_u64(
    bytes: &[u8],
    offset: usize,
    format: ResultImportFormat,
    field: impl std::fmt::Display,
) -> Result<u64, String> {
    let end = offset
        .checked_add(8)
        .ok_or_else(|| adapter_error(format, format_args!("{field} offset overflow")))?;
    let raw = bytes
        .get(offset..end)
        .ok_or_else(|| adapter_error(format, format_args!("truncated FST {field}")))?;
    Ok(u64::from_be_bytes(
        raw.try_into().expect("eight-byte slice"),
    ))
}

fn fst_bounded_size(
    value: u64,
    format: ResultImportFormat,
    field: impl std::fmt::Display,
) -> Result<usize, String> {
    if value > MAX_RESULT_DATASET_BYTES {
        return Err(adapter_error(
            format,
            format_args!(
                "FST {field} declares {value} bytes; the limit is {MAX_RESULT_DATASET_BYTES}"
            ),
        ));
    }
    usize::try_from(value)
        .map_err(|_| adapter_error(format, format_args!("FST {field} does not fit this target")))
}

fn fst_count(
    value: u64,
    maximum: usize,
    format: ResultImportFormat,
    field: &str,
) -> Result<usize, String> {
    let value = usize::try_from(value)
        .map_err(|_| adapter_error(format, format_args!("FST {field} count overflow")))?;
    if value > maximum {
        Err(adapter_error(
            format,
            format_args!("FST {field} count {value} exceeds the limit {maximum}"),
        ))
    } else {
        Ok(value)
    }
}

fn fst_uleb(
    bytes: &[u8],
    cursor: &mut usize,
    limit: usize,
    maximum_bits: u32,
    format: ResultImportFormat,
    field: &str,
) -> Result<(u64, usize), String> {
    let start = *cursor;
    let max_bytes = maximum_bits.div_ceil(7) as usize;
    let mut value = 0_u128;
    for index in 0..max_bytes {
        if *cursor >= limit {
            return Err(adapter_error(format, format_args!("truncated FST {field}")));
        }
        let byte = bytes[*cursor];
        *cursor += 1;
        value |= u128::from(byte & 0x7f) << (7 * index);
        if byte & 0x80 == 0 {
            let maximum = if maximum_bits == 64 {
                u128::from(u64::MAX)
            } else {
                (1_u128 << maximum_bits) - 1
            };
            if value > maximum {
                return Err(adapter_error(format, format_args!("FST {field} overflow")));
            }
            return Ok((value as u64, *cursor - start));
        }
    }
    Err(adapter_error(
        format,
        format_args!("FST {field} uses an overlong integer"),
    ))
}

fn fst_sleb_i64(
    bytes: &[u8],
    cursor: &mut usize,
    limit: usize,
    format: ResultImportFormat,
    field: &str,
) -> Result<i64, String> {
    let mut value = 0_i128;
    for index in 0..10_usize {
        if *cursor >= limit {
            return Err(adapter_error(format, format_args!("truncated FST {field}")));
        }
        let byte = bytes[*cursor];
        *cursor += 1;
        let shift = 7 * index;
        value |= i128::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            if byte & 0x40 != 0 {
                value |= (!0_i128) << (shift + 7);
            }
            return i64::try_from(value)
                .map_err(|_| adapter_error(format, format_args!("FST {field} overflow")));
        }
    }
    Err(adapter_error(
        format,
        format_args!("FST {field} uses an overlong integer"),
    ))
}

fn preflight_fst_blackout(
    bytes: &[u8],
    section_start: usize,
    section_end: usize,
    format: ResultImportFormat,
) -> Result<(), String> {
    let mut cursor = section_start + 8;
    let (count, _) = fst_uleb(
        bytes,
        &mut cursor,
        section_end,
        32,
        format,
        "blackout count",
    )?;
    let count = fst_count(count, MAX_RESULT_ROWS, format, "blackout")?;
    let mut time = 0_u64;
    for _ in 0..count {
        if cursor >= section_end {
            return Err(adapter_error(format, "truncated FST blackout entry"));
        }
        cursor += 1;
        let (delta, _) = fst_uleb(
            bytes,
            &mut cursor,
            section_end,
            64,
            format,
            "blackout time delta",
        )?;
        time = time
            .checked_add(delta)
            .ok_or_else(|| adapter_error(format, "FST blackout time overflow"))?;
    }
    if cursor != section_end {
        return Err(adapter_error(
            format,
            "FST blackout section has inconsistent framing",
        ));
    }
    Ok(())
}

fn preflight_fst_geometry(
    bytes: &[u8],
    section_start: usize,
    section_end: usize,
    section_length: usize,
    format: ResultImportFormat,
) -> Result<FstGeometryPreflight, String> {
    if section_length < 24 {
        return Err(adapter_error(format, "truncated FST geometry block"));
    }
    let uncompressed = fst_bounded_size(
        fst_be_u64(bytes, section_start + 8, format, "geometry expanded size")?,
        format,
        "geometry expanded allocation",
    )?;
    let handle_count = fst_count(
        fst_be_u64(bytes, section_start + 16, format, "geometry signal count")?,
        MAX_RESULT_COLUMNS - 1,
        format,
        "geometry signal",
    )?;
    if handle_count == 0 {
        return Err(adapter_error(format, "FST geometry declares no signals"));
    }
    let compressed = section_length - 24;
    if compressed > MAX_RESULT_DATASET_BYTES as usize {
        return Err(adapter_error(
            format,
            "FST geometry compressed-size limit exceeded",
        ));
    }
    // fst-reader's geometry inflater exposes only the resulting SignalInfo
    // vector, after which a malicious encoded width could cause a much larger
    // frame allocation. Without an independently preflightable payload, the
    // safe boundary is to accept the format's permitted uncompressed geometry
    // representation and fail closed on compressed geometry.
    if uncompressed != compressed {
        return Err(adapter_error(
            format,
            "compressed FST geometry is rejected because signal widths cannot be bounded before decompression",
        ));
    }
    let mut cursor = section_start + 24;
    let mut widths = Vec::with_capacity(handle_count);
    for _ in 0..handle_count {
        let (encoded, _) = fst_uleb(
            bytes,
            &mut cursor,
            section_end,
            32,
            format,
            "geometry signal width",
        )?;
        let width = if encoded == 0 {
            8 // FST's geometry marker for an IEEE-754 real signal.
        } else if encoded == u64::from(u32::MAX) {
            return Err(adapter_error(
                format,
                "FST variable-length signal records are not supported",
            ));
        } else {
            let width = usize::try_from(encoded)
                .map_err(|_| adapter_error(format, "FST signal width overflow"))?;
            if width > 53 {
                return Err(adapter_error(
                    format,
                    format_args!(
                        "FST digital signal width {width} exceeds the 53-bit lossless import limit"
                    ),
                ));
            }
            width
        };
        widths.push(width);
    }
    if cursor != section_end {
        return Err(adapter_error(
            format,
            "FST geometry payload has inconsistent signal-count framing",
        ));
    }
    Ok(FstGeometryPreflight { widths })
}

fn preflight_fst_hierarchy(
    bytes: &[u8],
    block_type: u8,
    section_start: usize,
    section_end: usize,
    section_length: usize,
    format: ResultImportFormat,
) -> Result<(), String> {
    if section_length < 16 {
        return Err(adapter_error(format, "truncated FST hierarchy block"));
    }
    let expanded = fst_bounded_size(
        fst_be_u64(bytes, section_start + 8, format, "hierarchy expanded size")?,
        format,
        "hierarchy expanded allocation",
    )?;
    let compressed = section_length - 16;
    if expanded == 0 || compressed == 0 {
        return Err(adapter_error(format, "FST hierarchy block is empty"));
    }
    if compressed > MAX_RESULT_DATASET_BYTES as usize {
        return Err(adapter_error(
            format,
            "FST hierarchy compressed-size limit exceeded",
        ));
    }
    if block_type == 4 {
        let payload = bytes
            .get(section_start + 16..section_end)
            .ok_or_else(|| adapter_error(format, "truncated FST gzip hierarchy payload"))?;
        if payload.len() < 10 || payload[0..2] != [0x1f, 0x8b] || payload[2] != 8 || payload[3] != 0
        {
            return Err(adapter_error(
                format,
                "FST gzip hierarchy has an unsupported or truncated header",
            ));
        }
    }
    if block_type == 7 {
        let mut cursor = section_start + 16;
        let (first_stage, encoded_bytes) = fst_uleb(
            bytes,
            &mut cursor,
            section_end,
            64,
            format,
            "LZ4-duo first-stage size",
        )?;
        let first_stage = fst_bounded_size(first_stage, format, "LZ4-duo first-stage allocation")?;
        if first_stage == 0 || encoded_bytes > compressed || cursor >= section_end {
            return Err(adapter_error(
                format,
                "FST LZ4-duo hierarchy has inconsistent compressed framing",
            ));
        }
    }
    Ok(())
}

fn preflight_fst_data_section(
    bytes: &[u8],
    section: FstDataPreflight,
    widths: &[usize],
    format: ResultImportFormat,
) -> Result<(), String> {
    let mut cursor = section.section_start + 32;
    let (frame_expanded, _) = fst_uleb(
        bytes,
        &mut cursor,
        section.section_end,
        64,
        format,
        "initial-frame expanded size",
    )?;
    let frame_expanded =
        fst_bounded_size(frame_expanded, format, "initial-frame expanded allocation")?;
    let (frame_compressed, _) = fst_uleb(
        bytes,
        &mut cursor,
        section.section_end,
        64,
        format,
        "initial-frame compressed size",
    )?;
    let frame_compressed = fst_bounded_size(
        frame_compressed,
        format,
        "initial-frame compressed allocation",
    )?;
    let (frame_handles, _) = fst_uleb(
        bytes,
        &mut cursor,
        section.section_end,
        64,
        format,
        "initial-frame signal count",
    )?;
    let frame_handles = fst_count(
        frame_handles,
        MAX_RESULT_COLUMNS - 1,
        format,
        "initial-frame signal",
    )?;
    if frame_handles != widths.len() {
        return Err(adapter_error(
            format,
            "FST initial-frame signal count disagrees with geometry",
        ));
    }
    let minimum_frame_bytes = widths.iter().try_fold(0_usize, |total, width| {
        total
            .checked_add(*width)
            .ok_or_else(|| adapter_error(format, "FST initial-frame width accounting overflow"))
    })?;
    if minimum_frame_bytes != frame_expanded {
        return Err(adapter_error(
            format,
            "FST initial-frame size disagrees with its declared signal widths",
        ));
    }
    if frame_compressed == 0 {
        return Err(adapter_error(format, "FST initial frame is empty"));
    }
    if frame_compressed != frame_expanded && bytes.get(cursor).copied() != Some(0x78) {
        return Err(adapter_error(
            format,
            "FST compressed initial frame does not have zlib framing",
        ));
    }
    cursor = cursor
        .checked_add(frame_compressed)
        .ok_or_else(|| adapter_error(format, "FST initial-frame offset overflow"))?;
    if cursor > section.section_end {
        return Err(adapter_error(format, "truncated FST initial frame"));
    }

    let (data_handles, _) = fst_uleb(
        bytes,
        &mut cursor,
        section.section_end,
        64,
        format,
        "value-change signal count",
    )?;
    let data_handles = fst_count(
        data_handles,
        MAX_RESULT_COLUMNS - 1,
        format,
        "value-change signal",
    )?;
    if data_handles != widths.len() {
        return Err(adapter_error(
            format,
            "FST value-change signal count disagrees with geometry",
        ));
    }
    let value_change_start = cursor;
    let pack_type = *bytes
        .get(cursor)
        .ok_or_else(|| adapter_error(format, "truncated FST value-change packing type"))?;
    cursor += 1;
    let value_payload_start = cursor;

    if section.section_length < 24 {
        return Err(adapter_error(format, "truncated FST time-table metadata"));
    }
    let time_meta_start = section.section_end - 24;
    let time_expanded = fst_bounded_size(
        fst_be_u64(bytes, time_meta_start, format, "time-table expanded size")?,
        format,
        "time-table expanded allocation",
    )?;
    let time_compressed = fst_bounded_size(
        fst_be_u64(
            bytes,
            time_meta_start + 8,
            format,
            "time-table compressed size",
        )?,
        format,
        "time-table compressed allocation",
    )?;
    let time_count = fst_count(
        fst_be_u64(bytes, time_meta_start + 16, format, "time-table item count")?,
        MAX_RESULT_ROWS,
        format,
        "time-table item",
    )?;
    if time_count > time_expanded {
        return Err(adapter_error(
            format,
            "FST time table declares more items than its expanded byte stream can contain",
        ));
    }
    let time_data_start = time_meta_start
        .checked_sub(time_compressed)
        .ok_or_else(|| adapter_error(format, "FST time-table offset underflow"))?;
    if (time_expanded == 0) != (time_compressed == 0) {
        return Err(adapter_error(
            format,
            "FST time table has inconsistent empty framing",
        ));
    }
    if time_compressed != 0
        && time_compressed != time_expanded
        && bytes.get(time_data_start).copied() != Some(0x78)
    {
        return Err(adapter_error(
            format,
            "FST compressed time table does not have zlib framing",
        ));
    }
    if time_compressed == time_expanded {
        let mut time_cursor = time_data_start;
        let mut time = 0_u64;
        for _ in 0..time_count {
            let (delta, _) = fst_uleb(
                bytes,
                &mut time_cursor,
                time_meta_start,
                64,
                format,
                "time-table delta",
            )?;
            time = time
                .checked_add(delta)
                .ok_or_else(|| adapter_error(format, "FST time-table value overflow"))?;
        }
        if time_cursor != time_meta_start {
            return Err(adapter_error(
                format,
                "FST uncompressed time table has inconsistent item-count framing",
            ));
        }
    }
    let chain_length_offset = time_data_start
        .checked_sub(8)
        .ok_or_else(|| adapter_error(format, "FST offset-table length underflow"))?;
    if chain_length_offset < value_payload_start {
        return Err(adapter_error(
            format,
            "FST time table overlaps its value-change payload",
        ));
    }
    let offset_table_bytes = fst_bounded_size(
        fst_be_u64(
            bytes,
            chain_length_offset,
            format,
            "offset-table compressed size",
        )?,
        format,
        "offset-table allocation",
    )?;
    let offset_table_start = chain_length_offset
        .checked_sub(offset_table_bytes)
        .ok_or_else(|| adapter_error(format, "FST offset-table start underflow"))?;
    if offset_table_start < value_payload_start {
        return Err(adapter_error(
            format,
            "FST offset table overlaps its value-change header",
        ));
    }
    let last_payload_offset = offset_table_start
        .checked_sub(value_change_start)
        .ok_or_else(|| adapter_error(format, "FST value-change offset underflow"))?;
    if last_payload_offset > u32::MAX as usize {
        return Err(adapter_error(format, "FST value-change offsets exceed u32"));
    }
    let ranges = preflight_fst_offset_table(
        bytes,
        section.block_type,
        offset_table_start,
        chain_length_offset,
        widths.len(),
        last_payload_offset,
        format,
    )?;
    let mut actual_memory = 0_usize;
    for (offset, length) in ranges {
        let signal_start = value_change_start
            .checked_add(offset)
            .ok_or_else(|| adapter_error(format, "FST signal payload offset overflow"))?;
        let signal_end = signal_start
            .checked_add(length)
            .ok_or_else(|| adapter_error(format, "FST signal payload length overflow"))?;
        if signal_start < value_payload_start || signal_end > offset_table_start {
            return Err(adapter_error(
                format,
                "FST signal payload points outside the value-change region",
            ));
        }
        let mut signal_cursor = signal_start;
        let (declared_expanded, marker_bytes) = fst_uleb(
            bytes,
            &mut signal_cursor,
            signal_end,
            32,
            format,
            "packed signal expanded size",
        )?;
        let compressed_bytes = match pack_type {
            b'4' | b'F' => length
                .checked_sub(marker_bytes)
                .ok_or_else(|| adapter_error(format, "FST packed signal length underflow"))?,
            _ => length,
        };
        if compressed_bytes > MAX_RESULT_DATASET_BYTES as usize {
            return Err(adapter_error(
                format,
                "FST packed signal compressed-size limit exceeded",
            ));
        }
        let expanded_bytes = if declared_expanded == 0 {
            length
                .checked_sub(marker_bytes)
                .ok_or_else(|| adapter_error(format, "FST direct signal length underflow"))?
        } else {
            let expanded = fst_bounded_size(
                declared_expanded,
                format,
                match pack_type {
                    b'4' => "LZ4 signal expanded allocation",
                    b'F' => "FastLZ signal expanded allocation",
                    _ => "zlib signal expanded allocation",
                },
            )?;
            if pack_type != b'4'
                && pack_type != b'F'
                && bytes.get(signal_cursor).copied() != Some(0x78)
            {
                return Err(adapter_error(
                    format,
                    "FST packed zlib signal does not have zlib framing",
                ));
            }
            expanded
        };
        actual_memory = actual_memory
            .checked_add(expanded_bytes)
            .ok_or_else(|| adapter_error(format, "FST signal allocation accounting overflow"))?;
        if actual_memory > MAX_RESULT_DATASET_BYTES as usize {
            return Err(adapter_error(
                format,
                "FST aggregate expanded signal allocation exceeds the byte limit",
            ));
        }
    }
    if actual_memory > section.memory_required {
        return Err(adapter_error(
            format,
            "FST value-change allocation is larger than its section memory declaration",
        ));
    }
    Ok(())
}

fn preflight_fst_offset_table(
    bytes: &[u8],
    block_type: u8,
    table_start: usize,
    table_end: usize,
    signal_count: usize,
    payload_end_offset: usize,
    format: ResultImportFormat,
) -> Result<Vec<(usize, usize)>, String> {
    let mut cursor = table_start;
    let mut signal_index = 0_usize;
    let mut offsets = Vec::with_capacity(signal_count);
    let mut direct_signals = Vec::with_capacity(signal_count);
    let mut current_offset = 0_usize;
    let mut previous_alias = None;

    while cursor < table_end {
        if block_type == 8 {
            let kind = bytes[cursor];
            if kind & 1 == 1 {
                let encoded = fst_sleb_i64(
                    bytes,
                    &mut cursor,
                    table_end,
                    format,
                    "dynamic-alias offset",
                )?;
                let value = encoded >> 1;
                match value.cmp(&0) {
                    std::cmp::Ordering::Greater => {
                        let delta = usize::try_from(value).map_err(|_| {
                            adapter_error(format, "FST dynamic-alias offset overflow")
                        })?;
                        current_offset = current_offset.checked_add(delta).ok_or_else(|| {
                            adapter_error(format, "FST dynamic-alias offset overflow")
                        })?;
                        offsets.push(current_offset);
                        direct_signals.push(true);
                        signal_index = signal_index.checked_add(1).ok_or_else(|| {
                            adapter_error(format, "FST offset-table count overflow")
                        })?;
                    }
                    std::cmp::Ordering::Less => {
                        let alias = value
                            .checked_neg()
                            .and_then(|value| value.checked_sub(1))
                            .and_then(|value| usize::try_from(value).ok())
                            .ok_or_else(|| {
                                adapter_error(format, "FST dynamic alias index overflow")
                            })?;
                        if alias >= signal_index || !direct_signals[alias] {
                            return Err(adapter_error(
                                format,
                                "FST dynamic alias does not refer to an earlier direct signal",
                            ));
                        }
                        previous_alias = Some(alias);
                        direct_signals.push(false);
                        signal_index += 1;
                    }
                    std::cmp::Ordering::Equal => {
                        if previous_alias.is_none() {
                            return Err(adapter_error(
                                format,
                                "FST repeated dynamic alias has no preceding alias",
                            ));
                        }
                        direct_signals.push(false);
                        signal_index += 1;
                    }
                }
            } else {
                let (encoded, _) = fst_uleb(
                    bytes,
                    &mut cursor,
                    table_end,
                    32,
                    format,
                    "dynamic-alias empty-signal run",
                )?;
                let empty = usize::try_from(encoded >> 1)
                    .map_err(|_| adapter_error(format, "FST empty-signal count overflow"))?;
                if empty == 0 {
                    return Err(adapter_error(
                        format,
                        "FST offset table contains an empty zero-length run",
                    ));
                }
                let next_signal_index = signal_index
                    .checked_add(empty)
                    .ok_or_else(|| adapter_error(format, "FST offset-table count overflow"))?;
                if next_signal_index > signal_count {
                    return Err(adapter_error(
                        format,
                        "FST offset table declares more signals than geometry",
                    ));
                }
                direct_signals.resize(next_signal_index, false);
                signal_index = next_signal_index;
            }
        } else {
            let (raw, _) = fst_uleb(
                bytes,
                &mut cursor,
                table_end,
                32,
                format,
                "offset-table entry",
            )?;
            let raw = u32::try_from(raw)
                .map_err(|_| adapter_error(format, "FST offset-table entry overflow"))?;
            if raw == 0 {
                let (alias, _) =
                    fst_uleb(bytes, &mut cursor, table_end, 32, format, "signal alias")?;
                let alias = usize::try_from(alias)
                    .ok()
                    .and_then(|alias| alias.checked_sub(1))
                    .ok_or_else(|| adapter_error(format, "FST signal alias index underflow"))?;
                if alias >= signal_index || !direct_signals[alias] {
                    return Err(adapter_error(
                        format,
                        "FST signal alias does not refer to an earlier direct signal",
                    ));
                }
                direct_signals.push(false);
                signal_index += 1;
            } else if raw & 1 == 1 {
                let delta = (raw >> 1) as usize;
                if delta == 0 {
                    return Err(adapter_error(format, "FST signal offset does not advance"));
                }
                current_offset = current_offset
                    .checked_add(delta)
                    .ok_or_else(|| adapter_error(format, "FST signal offset overflow"))?;
                offsets.push(current_offset);
                direct_signals.push(true);
                signal_index += 1;
            } else {
                let empty = (raw >> 1) as usize;
                if empty == 0 {
                    return Err(adapter_error(
                        format,
                        "FST offset table contains an empty zero-length run",
                    ));
                }
                let next_signal_index = signal_index
                    .checked_add(empty)
                    .ok_or_else(|| adapter_error(format, "FST offset-table count overflow"))?;
                if next_signal_index > signal_count {
                    return Err(adapter_error(
                        format,
                        "FST offset table declares more signals than geometry",
                    ));
                }
                direct_signals.resize(next_signal_index, false);
                signal_index = next_signal_index;
            }
        }
        if signal_index > signal_count {
            return Err(adapter_error(
                format,
                "FST offset table declares more signals than geometry",
            ));
        }
    }
    if signal_index != signal_count {
        return Err(adapter_error(
            format,
            "FST offset-table signal count disagrees with geometry",
        ));
    }
    if offsets
        .last()
        .is_some_and(|offset| *offset >= payload_end_offset)
    {
        return Err(adapter_error(
            format,
            "FST signal offset points outside the value-change payload",
        ));
    }
    let mut ranges = Vec::with_capacity(offsets.len());
    for (index, offset) in offsets.iter().copied().enumerate() {
        let next = offsets
            .get(index + 1)
            .copied()
            .unwrap_or(payload_end_offset);
        let length = next
            .checked_sub(offset)
            .ok_or_else(|| adapter_error(format, "FST signal offsets are not ordered"))?;
        if length == 0 || length > u32::MAX as usize {
            return Err(adapter_error(
                format,
                "FST signal payload length is zero or exceeds u32",
            ));
        }
        ranges.push((offset, length));
    }
    Ok(ranges)
}

pub(in crate::workbench::workflows) fn parse_fst(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let geometry = preflight_fst(bytes, format)?;
    let cursor = Cursor::new(bytes);
    let mut reader = fst_reader::FstReader::open(BufReader::new(cursor))
        .map_err(|error| adapter_error(format, format_args!("invalid FST container: {error}")))?;
    let header = reader.get_header();
    if header.var_count as usize > MAX_RESULT_COLUMNS - 1 {
        return Err(adapter_error(format, "FST signal-count limit exceeded"));
    }
    if !(-30..=30).contains(&header.timescale_exponent) {
        return Err(adapter_error(
            format,
            "FST timescale exponent is outside the supported finite range",
        ));
    }
    let mut scopes = Vec::new();
    let mut scope_identity_bytes = 0_usize;
    let mut by_handle: BTreeMap<usize, Vec<DigitalSignal>> = BTreeMap::new();
    let mut hierarchy_entries = 0_usize;
    let mut hierarchy_signals = 0_usize;
    let mut hierarchy_error = None;
    let maximum_handle = usize::try_from(header.max_handle)
        .map_err(|_| adapter_error(format, "FST header signal count does not fit this target"))?;
    reader
        .read_hierarchy(|entry| {
            if hierarchy_error.is_some() {
                return;
            }
            hierarchy_entries += 1;
            if hierarchy_entries > MAX_RESULT_VALUES {
                hierarchy_error = Some("FST hierarchy-entry limit exceeded".to_owned());
                return;
            }
            match entry {
                fst_reader::FstHierarchyEntry::Scope { name, .. } => {
                    let separator = usize::from(!scopes.is_empty());
                    let Some(next_identity_bytes) = scope_identity_bytes
                        .checked_add(separator)
                        .and_then(|length| length.checked_add(name.len()))
                    else {
                        hierarchy_error = Some("FST hierarchy identity length overflow".to_owned());
                        return;
                    };
                    if name.is_empty() || name.len() > MAX_SIGNAL_NAME_BYTES {
                        hierarchy_error = Some(format!(
                            "FST scope identity exceeds {MAX_SIGNAL_NAME_BYTES} bytes or is empty"
                        ));
                    } else if next_identity_bytes > MAX_SIGNAL_NAME_BYTES {
                        hierarchy_error = Some(format!(
                            "FST scope path exceeds {MAX_SIGNAL_NAME_BYTES} bytes"
                        ));
                    } else if scopes.len() >= MAX_RESULT_COLUMNS {
                        hierarchy_error = Some("FST hierarchy-depth limit exceeded".to_owned());
                    } else {
                        scope_identity_bytes = next_identity_bytes;
                        scopes.push(name);
                    }
                }
                fst_reader::FstHierarchyEntry::UpScope => {
                    if let Some(name) = scopes.pop() {
                        scope_identity_bytes = if scopes.is_empty() {
                            0
                        } else {
                            scope_identity_bytes.saturating_sub(name.len() + 1)
                        };
                    } else {
                        hierarchy_error =
                            Some("FST hierarchy closes a scope that was not open".to_owned());
                    }
                }
                fst_reader::FstHierarchyEntry::Var {
                    name,
                    length,
                    handle,
                    ..
                } => {
                    hierarchy_signals += 1;
                    if hierarchy_signals > MAX_RESULT_COLUMNS - 1 {
                        hierarchy_error =
                            Some("FST hierarchy signal/alias limit exceeded".to_owned());
                        return;
                    }
                    let handle = handle.get_index();
                    if handle >= maximum_handle {
                        hierarchy_error = Some(
                            "FST hierarchy references an out-of-range signal handle".to_owned(),
                        );
                        return;
                    }
                    if geometry.widths[handle] != length as usize {
                        hierarchy_error = Some(format!(
                            "FST hierarchy width {length} for '{name}' disagrees with geometry width {}",
                            geometry.widths[handle]
                        ));
                        return;
                    }
                    if length == 0 || length > 53 {
                        hierarchy_error = Some(format!(
                            "FST hierarchy signal '{name}' has unsupported width {length}"
                        ));
                        return;
                    }
                    if name.is_empty() || name.len() > MAX_SIGNAL_NAME_BYTES {
                        hierarchy_error = Some(format!(
                            "FST signal identity exceeds {MAX_SIGNAL_NAME_BYTES} bytes or is empty"
                        ));
                        return;
                    }
                    let mut full_name = scopes.join(".");
                    if !full_name.is_empty() {
                        full_name.push('.');
                    }
                    full_name.push_str(&name);
                    if full_name.len() > MAX_SIGNAL_NAME_BYTES {
                        hierarchy_error = Some(format!(
                            "FST hierarchy signal identity exceeds {MAX_SIGNAL_NAME_BYTES} bytes"
                        ));
                        return;
                    }
                    by_handle.entry(handle).or_default().push(DigitalSignal {
                        name: full_name,
                        width: length as usize,
                    });
                }
                _ => {}
            }
        })
        .map_err(|error| {
            adapter_error(
                format,
                format_args!("could not read FST hierarchy: {error}"),
            )
        })?;
    if let Some(error) = hierarchy_error {
        return Err(adapter_error(format, error));
    }
    if !scopes.is_empty() {
        return Err(adapter_error(format, "FST hierarchy has unclosed scopes"));
    }
    if by_handle.len() != maximum_handle {
        return Err(adapter_error(
            format,
            "FST hierarchy unique-signal count disagrees with geometry",
        ));
    }
    let total_signals = by_handle.values().map(Vec::len).sum::<usize>();
    if total_signals == 0 || total_signals > MAX_RESULT_COLUMNS - 1 {
        return Err(adapter_error(
            format,
            "FST contains no signals or too many aliases",
        ));
    }
    let handles = by_handle
        .keys()
        .map(|index| fst_reader::FstSignalHandle::from_index(*index))
        .collect::<Vec<_>>();
    let handle_order = by_handle.keys().copied().collect::<Vec<_>>();
    let handle_to_signal = handle_order
        .iter()
        .enumerate()
        .map(|(signal, handle)| (*handle, signal))
        .collect::<HashMap<_, _>>();
    let canonical_signals = handle_order
        .iter()
        .map(|handle| DigitalSignal {
            name: by_handle[handle][0].name.clone(),
            width: by_handle[handle][0].width,
        })
        .collect::<Vec<_>>();
    let mut events = Vec::new();
    let callback_result = reader.read_signals(
        &fst_reader::FstFilter::filter_signals(handles),
        |tick, handle, value| {
            if events.len() >= MAX_RESULT_VALUES {
                return Err("FST event-count limit exceeded".to_owned());
            }
            let signal = *handle_to_signal
                .get(&handle.get_index())
                .ok_or_else(|| "FST returned an undeclared signal handle".to_owned())?;
            let value = match value {
                fst_reader::FstSignalValue::String(bits) => logic_bits_to_f64(bits, format)?,
                fst_reader::FstSignalValue::Real(value) if value.is_finite() => value,
                fst_reader::FstSignalValue::Real(_) => {
                    return Err("FST contains a non-finite real value".to_owned());
                }
            };
            events.push(DigitalEvent {
                tick,
                signal,
                value,
            });
            Ok(())
        },
    );
    callback_result.map_err(|error| {
        adapter_error(format, format_args!("could not read FST events: {error:?}"))
    })?;
    let timescale = 10_f64.powi(header.timescale_exponent as i32);
    let parsed = digital_events_to_dataset(format, timescale, canonical_signals, events)?;

    // Materialize aliases after canonical decoding. They retain independent
    // identities while sharing exact samples and axes.
    let mut aliases = Vec::new();
    for (canonical_index, handle) in handle_order.iter().enumerate() {
        for alias in by_handle[handle].iter().skip(1) {
            aliases.push((canonical_index, alias.name.clone()));
        }
    }
    let mut parsed = parsed;
    append_digital_aliases(format, &mut parsed, aliases)?;
    Ok(parsed)
}

pub(in crate::workbench::workflows) fn looks_like_fst(bytes: &[u8]) -> bool {
    let mut cursor = Cursor::new(bytes);
    fst_reader::is_fst_file(&mut cursor)
}

fn digital_events_to_dataset(
    format: ResultImportFormat,
    timescale_seconds: f64,
    signals: Vec<DigitalSignal>,
    mut events: Vec<DigitalEvent>,
) -> Result<ParsedResultDataset, String> {
    if !timescale_seconds.is_finite() || timescale_seconds <= 0.0 {
        return Err(adapter_error(
            format,
            "digital timescale is not finite and positive",
        ));
    }
    if signals.is_empty() {
        return Err(adapter_error(format, "digital source declares no signals"));
    }
    for signal in &signals {
        validate_name(format, "signal", &signal.name)?;
        if signal.width == 0 || signal.width > 53 {
            return Err(adapter_error(
                format,
                format_args!(
                    "signal '{}' width {} cannot be represented exactly",
                    signal.name, signal.width
                ),
            ));
        }
    }
    events.sort_by_key(|event| event.tick);
    let ticks = events
        .iter()
        .map(|event| event.tick)
        .collect::<BTreeSet<_>>();
    if ticks.len() < MIN_RESULT_ROWS || ticks.len() > MAX_RESULT_ROWS {
        return Err(adapter_error(
            format,
            format_args!(
                "digital trace has {} distinct event times; expected {MIN_RESULT_ROWS}..={MAX_RESULT_ROWS}",
                ticks.len()
            ),
        ));
    }
    let mut states = vec![None; signals.len()];
    let mut values = vec![Vec::with_capacity(ticks.len()); signals.len()];
    let mut coordinate = Vec::with_capacity(ticks.len());
    let mut events = events.into_iter().peekable();
    for tick in ticks {
        if tick > MAX_EXACT_F64_INTEGER {
            return Err(adapter_error(
                format,
                format_args!("digital timestamp tick {tick} cannot be represented exactly as f64"),
            ));
        }
        while events.peek().is_some_and(|event| event.tick == tick) {
            let event = events.next().expect("peeked event exists");
            if event.signal >= states.len() {
                return Err(adapter_error(
                    format,
                    "digital event references an unknown signal",
                ));
            }
            states[event.signal] = Some(event.value);
        }
        if states.iter().any(Option::is_none) {
            return Err(adapter_error(
                format,
                format_args!(
                    "not every digital signal has a known 0/1/vector value at initial tick {tick}"
                ),
            ));
        }
        let time = (tick as f64) * timescale_seconds;
        if !time.is_finite() {
            return Err(adapter_error(
                format,
                "scaled digital timestamp is not finite",
            ));
        }
        coordinate.push(time);
        for (column, state) in values.iter_mut().zip(&states) {
            column.push(state.expect("all states checked"));
        }
    }
    let signals = signals
        .into_iter()
        .zip(values)
        .map(|(signal, real)| ImportedSignal {
            name: signal.name,
            real,
            imag: None,
            unit: None,
        })
        .collect();
    finish_dataset(format, AnalysisType::Transient, "time", coordinate, signals)
}

fn append_digital_aliases(
    format: ResultImportFormat,
    parsed: &mut ParsedResultDataset,
    aliases: Vec<(usize, String)>,
) -> Result<(), String> {
    if parsed.waveforms.len().saturating_add(aliases.len()) > MAX_RESULT_COLUMNS - 1 {
        return Err(adapter_error(
            format,
            "digital aliases exceed the signal-count limit",
        ));
    }
    let mut known = parsed
        .waveforms
        .iter()
        .map(|waveform| waveform.name.to_ascii_lowercase())
        .collect::<HashSet<_>>();
    let mut alias_waveforms = Vec::with_capacity(aliases.len());
    for (canonical_index, name) in aliases {
        validate_name(format, "signal", &name)?;
        if !known.insert(name.to_ascii_lowercase()) {
            return Err(adapter_error(
                format,
                format_args!("duplicate digital signal identity '{name}'"),
            ));
        }
        let canonical = parsed.waveforms.get(canonical_index).ok_or_else(|| {
            adapter_error(
                format,
                "digital alias references an unknown canonical signal",
            )
        })?;
        alias_waveforms.push(WaveformData::new(
            name,
            Arc::clone(&canonical.x),
            Arc::clone(&canonical.y),
            trace_color(parsed.waveforms.len() + alias_waveforms.len()),
        ));
    }
    parsed.waveforms.extend(alias_waveforms);
    Ok(())
}
