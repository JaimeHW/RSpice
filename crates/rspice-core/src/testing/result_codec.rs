use super::{TestResult, ValueMismatch};

const RESULT_HEADER: &str = "RSPICE_TEST_RESULT_V1";

pub fn encode_test_result(result: &TestResult) -> String {
    let mut output = String::new();
    output.push_str(RESULT_HEADER);
    output.push('\n');
    push_field(&mut output, "name", &encode_text(&result.name));
    push_field(&mut output, "passed", if result.passed { "1" } else { "0" });
    push_field(&mut output, "duration_ms", &result.duration_ms.to_string());
    push_optional_field(&mut output, "analysis_type", result.analysis_type.as_deref());
    push_optional_field(&mut output, "error", result.error.as_deref());
    push_field(&mut output, "mismatches", &result.mismatches.len().to_string());

    for mismatch in &result.mismatches {
        output.push_str("m\t");
        output.push_str(&format_float(mismatch.x_value));
        output.push('\t');
        output.push_str(&encode_text(&mismatch.node));
        output.push('\t');
        output.push_str(&format_float(mismatch.expected));
        output.push('\t');
        output.push_str(&format_float(mismatch.actual));
        output.push('\t');
        output.push_str(&format_float(mismatch.relative_error));
        output.push('\n');
    }

    output
}

pub fn decode_test_result(content: &str) -> Result<TestResult, String> {
    let mut lines = content.lines();
    match lines.next() {
        Some(RESULT_HEADER) => {}
        Some(other) => return Err(format!("invalid result header '{other}'")),
        None => return Err("empty result file".to_string()),
    }

    let name = decode_required_text(lines.next(), "name")?;
    let passed = decode_required_bool(lines.next(), "passed")?;
    let duration_ms = decode_required_u128(lines.next(), "duration_ms")?;
    let analysis_type = decode_optional_text(lines.next(), "analysis_type")?;
    let error = decode_optional_text(lines.next(), "error")?;
    let expected_mismatches = decode_required_usize(lines.next(), "mismatches")?;
    let mut mismatches = Vec::with_capacity(expected_mismatches);

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() != 6 || parts[0] != "m" {
            return Err(format!("invalid mismatch row '{line}'"));
        }
        mismatches.push(ValueMismatch {
            x_value: parse_float(parts[1], "mismatch x")?,
            node: decode_text(parts[2])?,
            expected: parse_float(parts[3], "mismatch expected")?,
            actual: parse_float(parts[4], "mismatch actual")?,
            relative_error: parse_float(parts[5], "mismatch relative error")?,
        });
    }

    if mismatches.len() != expected_mismatches {
        return Err(format!(
            "expected {expected_mismatches} mismatch row(s), decoded {}",
            mismatches.len()
        ));
    }

    Ok(TestResult {
        name,
        passed,
        error,
        mismatches,
        duration_ms,
        analysis_type,
    })
}

fn push_field(output: &mut String, key: &str, value: &str) {
    output.push_str(key);
    output.push('\t');
    output.push_str(value);
    output.push('\n');
}

fn push_optional_field(output: &mut String, key: &str, value: Option<&str>) {
    match value {
        Some(value) => {
            output.push_str(key);
            output.push_str("\t1\t");
            output.push_str(&encode_text(value));
            output.push('\n');
        }
        None => {
            output.push_str(key);
            output.push_str("\t0\n");
        }
    }
}

fn decode_required_text(line: Option<&str>, key: &str) -> Result<String, String> {
    let parts = split_keyed_line(line, key)?;
    if parts.len() != 2 {
        return Err(format!("invalid '{key}' field"));
    }
    decode_text(parts[1])
}

fn decode_required_bool(line: Option<&str>, key: &str) -> Result<bool, String> {
    let parts = split_keyed_line(line, key)?;
    if parts.len() != 2 {
        return Err(format!("invalid '{key}' field"));
    }
    match parts[1] {
        "0" => Ok(false),
        "1" => Ok(true),
        other => Err(format!("invalid bool for '{key}': {other}")),
    }
}

fn decode_required_u128(line: Option<&str>, key: &str) -> Result<u128, String> {
    let parts = split_keyed_line(line, key)?;
    if parts.len() != 2 {
        return Err(format!("invalid '{key}' field"));
    }
    parts[1]
        .parse::<u128>()
        .map_err(|err| format!("invalid integer for '{key}': {err}"))
}

fn decode_required_usize(line: Option<&str>, key: &str) -> Result<usize, String> {
    let value = decode_required_u128(line, key)?;
    usize::try_from(value).map_err(|_| format!("'{key}' value is too large"))
}

fn decode_optional_text(line: Option<&str>, key: &str) -> Result<Option<String>, String> {
    let parts = split_keyed_line(line, key)?;
    match parts.as_slice() {
        [_, "0"] => Ok(None),
        [_, "1", value] => decode_text(value).map(Some),
        _ => Err(format!("invalid optional '{key}' field")),
    }
}

fn split_keyed_line<'a>(line: Option<&'a str>, key: &str) -> Result<Vec<&'a str>, String> {
    let line = line.ok_or_else(|| format!("missing '{key}' field"))?;
    let parts: Vec<&str> = line.split('\t').collect();
    if parts.first().copied() != Some(key) {
        return Err(format!("expected '{key}' field, got '{line}'"));
    }
    Ok(parts)
}

fn format_float(value: f64) -> String {
    if value.is_nan() {
        "NaN".to_string()
    } else if value == f64::INFINITY {
        "Infinity".to_string()
    } else if value == f64::NEG_INFINITY {
        "-Infinity".to_string()
    } else {
        format!("{value:.17e}")
    }
}

fn parse_float(value: &str, label: &str) -> Result<f64, String> {
    match value {
        "NaN" => Ok(f64::NAN),
        "Infinity" => Ok(f64::INFINITY),
        "-Infinity" => Ok(f64::NEG_INFINITY),
        _ => value
            .parse::<f64>()
            .map_err(|err| format!("invalid {label}: {err}")),
    }
}

fn encode_text(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'.' | b'-' | b'_' => {
                encoded.push(char::from(byte));
            }
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }
    encoded
}

fn decode_text(value: &str) -> Result<String, String> {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut idx = 0;
    while idx < bytes.len() {
        if bytes[idx] == b'%' {
            if idx + 2 >= bytes.len() {
                return Err("truncated percent escape".to_string());
            }
            let hex = std::str::from_utf8(&bytes[idx + 1..idx + 3])
                .map_err(|err| format!("invalid escape bytes: {err}"))?;
            let byte = u8::from_str_radix(hex, 16)
                .map_err(|err| format!("invalid percent escape '%{hex}': {err}"))?;
            decoded.push(byte);
            idx += 3;
        } else {
            decoded.push(bytes[idx]);
            idx += 1;
        }
    }
    String::from_utf8(decoded).map_err(|err| format!("decoded text is not UTF-8: {err}"))
}
