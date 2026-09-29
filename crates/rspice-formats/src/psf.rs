//! PSF ASCII declarations and finite numeric columns.

use crate::WaveformDomain;

#[derive(Debug, Clone, Copy)]
pub struct PsfReadLimits {
    pub max_columns: usize,
    pub max_rows: usize,
}

/// Decoded columns, before consumer-specific coordinate and identity validation.
#[derive(Debug)]
pub struct DecodedPsfWaveforms {
    pub domain: WaveformDomain,
    pub coordinate_name: String,
    pub coordinate: Vec<f64>,
    pub signal_names: Vec<String>,
    pub signal_values: Vec<Vec<f64>>,
}

#[derive(Debug)]
pub enum PsfReadError {
    InvalidData(String),
    Utf8(std::str::Utf8Error),
}

impl std::fmt::Display for PsfReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidData(detail) => f.write_str(detail),
            Self::Utf8(source) => write!(f, "source is not UTF-8: {source}"),
        }
    }
}

impl std::error::Error for PsfReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidData(_) => None,
            Self::Utf8(source) => Some(source),
        }
    }
}

/// Decode an already byte-bounded source without assigning result ownership.
pub fn decode_psf_ascii(
    bytes: &[u8],
    limits: PsfReadLimits,
) -> Result<DecodedPsfWaveforms, PsfReadError> {
    let text = std::str::from_utf8(bytes).map_err(PsfReadError::Utf8)?;
    decode_text(text, limits).map_err(PsfReadError::InvalidData)
}

fn decode_text(text: &str, limits: PsfReadLimits) -> Result<DecodedPsfWaveforms, String> {
    let mut analysis = None;
    let mut coordinate_name = None;
    let mut signal_names = Vec::new();
    let mut value_lines = Vec::new();
    let mut section = "";
    for (index, raw_line) in text.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with("//") || line.starts_with('#') {
            continue;
        }
        match line.to_ascii_uppercase().as_str() {
            "HEADER" | "TYPE" | "SWEEP" | "TRACE" | "VALUE" | "END" => {
                section = line;
                continue;
            }
            _ => {}
        }
        if section.eq_ignore_ascii_case("HEADER") {
            let fields = psf_tokens(line, line_number)?;
            if fields.len() >= 2
                && matches!(
                    fields[0].to_ascii_lowercase().as_str(),
                    "analysis" | "type" | "sweepmode"
                )
            {
                analysis = Some(
                    fields[1]
                        .parse::<WaveformDomain>()
                        .map_err(|error| error.to_string())?,
                );
            }
        } else if section.eq_ignore_ascii_case("SWEEP") {
            let fields = psf_tokens(line, line_number)?;
            if fields.is_empty() {
                continue;
            }
            if coordinate_name.replace(fields[0].clone()).is_some() {
                return Err("PSF ASCII declares multiple sweep axes".to_owned());
            }
        } else if section.eq_ignore_ascii_case("TRACE") {
            let fields = psf_tokens(line, line_number)?;
            if fields.is_empty() {
                continue;
            }
            signal_names.push(fields[0].clone());
        } else if section.eq_ignore_ascii_case("VALUE") {
            value_lines.push((line_number, line));
        }
    }
    let coordinate_name = coordinate_name
        .ok_or_else(|| "PSF ASCII is missing a SWEEP axis declaration".to_owned())?;
    if signal_names.is_empty() {
        return Err("PSF ASCII is missing TRACE declarations".to_owned());
    }
    if signal_names.len() + 1 > limits.max_columns {
        return Err("PSF ASCII trace-count limit exceeded".to_owned());
    }
    let mut coordinate = Vec::new();
    let mut components = vec![Vec::new(); signal_names.len()];
    for (line_number, line) in value_lines {
        if coordinate.len() >= limits.max_rows {
            return Err("PSF ASCII row limit exceeded".to_owned());
        }
        let fields = psf_tokens(line, line_number)?;
        if fields.len() != signal_names.len() + 1 {
            return Err(format!(
                "VALUE row {line_number} has {} fields; expected {}",
                fields.len(),
                signal_names.len() + 1
            ));
        }
        coordinate.push(parse_psf_number(&fields[0], line_number, &coordinate_name)?);
        for (index, signal) in signal_names.iter().enumerate() {
            components[index].push(parse_psf_number(&fields[index + 1], line_number, signal)?);
        }
    }
    Ok(DecodedPsfWaveforms {
        domain: analysis.unwrap_or_else(|| WaveformDomain::from_coordinate_name(&coordinate_name)),
        coordinate_name,
        coordinate,
        signal_names,
        signal_values: components,
    })
}

fn psf_tokens(line: &str, line_number: usize) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut token = String::new();
    let mut quoted = false;
    let mut chars = line.chars();
    while let Some(character) = chars.next() {
        if quoted {
            match character {
                '"' => quoted = false,
                '\\' => {
                    let escaped = chars
                        .next()
                        .ok_or_else(|| format!("line {line_number} ends in an escape"))?;
                    token.push(escaped);
                }
                _ => token.push(character),
            }
        } else if character == '"' {
            quoted = true;
        } else if character.is_whitespace() || character == '(' || character == ')' {
            if !token.is_empty() {
                tokens.push(std::mem::take(&mut token));
            }
        } else {
            token.push(character);
        }
    }
    if quoted {
        return Err(format!(
            "line {line_number} has an unterminated quoted token"
        ));
    }
    if !token.is_empty() {
        tokens.push(token);
    }
    Ok(tokens)
}

fn parse_psf_number(token: &str, line: usize, identity: &str) -> Result<f64, String> {
    let value = token
        .parse::<f64>()
        .map_err(|_| format!("line {line} has invalid numeric token '{token}' for '{identity}'"))?;
    if !value.is_finite() {
        return Err(format!("line {line} has non-finite value for '{identity}'"));
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    #[test]
    fn quoted_columns_limits_and_source_errors_survive_decoding() {
        let source = b"HEADER\n\"analysis\" \"tran\"\nSWEEP\n\"time\" \"s\"\nTRACE\n\"V(out)\" \"V\"\nVALUE\n(0 1)\n(1e-9 -0)\nEND\n";
        let limits = PsfReadLimits {
            max_columns: 2,
            max_rows: 2,
        };
        let decoded = decode_psf_ascii(source, limits).unwrap();
        assert_eq!(decoded.domain, WaveformDomain::Transient);
        assert_eq!(decoded.coordinate_name, "time");
        assert_eq!(decoded.coordinate, [0.0, 1e-9]);
        assert_eq!(decoded.signal_names, ["V(out)"]);
        assert_eq!(decoded.signal_values[0][0], 1.0);
        assert_eq!(decoded.signal_values[0][1].to_bits(), (-0.0_f64).to_bits());

        for (limits, expected) in [
            (
                PsfReadLimits {
                    max_columns: 1,
                    ..limits
                },
                "PSF ASCII trace-count limit exceeded",
            ),
            (
                PsfReadLimits {
                    max_rows: 1,
                    ..limits
                },
                "PSF ASCII row limit exceeded",
            ),
        ] {
            let error = decode_psf_ascii(source, limits).unwrap_err();
            assert!(matches!(error, PsfReadError::InvalidData(_)));
            assert_eq!(error.to_string(), expected);
            assert!(error.source().is_none());
        }
        let error = decode_psf_ascii(b"\xff", limits).unwrap_err();
        assert!(matches!(error, PsfReadError::Utf8(_)));
        assert!(
            error
                .source()
                .unwrap()
                .downcast_ref::<std::str::Utf8Error>()
                .is_some()
        );
        let error = decode_psf_ascii(b"HEADER\nanalysis unknown\n", limits).unwrap_err();
        assert_eq!(error.to_string(), "unsupported analysis domain 'unknown'");
    }
}
