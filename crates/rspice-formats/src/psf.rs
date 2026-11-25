//! PSF ASCII declarations and finite numeric columns.
//!
//! Two layouts are read. RSpice writes one row per point, `(coordinate value value ...)`.
//! Spectre (`-format psfascii`) and Cadence's `psf` tool write blocks: a `TYPE` section naming
//! each value type (`COMPLEX DOUBLE` for an AC voltage), `SWEEP` and `TRACE` declarations that
//! may carry `PROP( ... )` property blocks and group traces (`"group" GROUP 3`), and a `VALUE`
//! section that names each value it gives, `"time" 1e-9` then `"out" 0.5` (or `"out" (re im)`),
//! a group's values following its name.

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
    /// Each signal's imaginary parts, when its type is complex.
    pub signal_imag: Vec<Option<Vec<f64>>>,
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
    if names_its_values(text) {
        decode_blocks(text, limits).map_err(PsfReadError::InvalidData)
    } else {
        decode_text(text, limits).map_err(PsfReadError::InvalidData)
    }
}

/// Whether the `VALUE` section names each value it gives, as Cadence's block layout does,
/// rather than giving one parenthesized row per point.
fn names_its_values(text: &str) -> bool {
    text.lines()
        .map(str::trim)
        .skip_while(|line| !line.eq_ignore_ascii_case("VALUE"))
        .skip(1)
        .find(|line| !line.is_empty())
        .is_some_and(|line| line.starts_with('"'))
}

/// One token of the block layout: a quoted name, a bare word or number, or a parenthesis.
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Text(String),
    Word(String),
    Open,
    Close,
}

/// The file's tokens, each with the line it is on, section by section.
fn block_sections(text: &str) -> Result<Vec<(String, Vec<(usize, Token)>)>, String> {
    let mut sections: Vec<(String, Vec<(usize, Token)>)> = Vec::new();
    for (index, raw_line) in text.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with("//") || line.starts_with('#') {
            continue;
        }
        let upper = line.to_ascii_uppercase();
        if matches!(
            upper.as_str(),
            "HEADER" | "TYPE" | "SWEEP" | "TRACE" | "VALUE" | "END"
        ) {
            sections.push((upper, Vec::new()));
            continue;
        }
        let Some((_, tokens)) = sections.last_mut() else {
            return Err(format!(
                "line {line_number} comes before the HEADER that opens a PSF file"
            ));
        };
        let mut word = String::new();
        let mut quoted: Option<String> = None;
        let mut chars = line.chars();
        while let Some(character) = chars.next() {
            if let Some(text) = quoted.as_mut() {
                match character {
                    '"' => {
                        tokens.push((line_number, Token::Text(quoted.take().unwrap_or_default())))
                    }
                    '\\' => text.push(
                        chars
                            .next()
                            .ok_or_else(|| format!("line {line_number} ends in an escape"))?,
                    ),
                    _ => text.push(character),
                }
                continue;
            }
            if character == '"' || character == '(' || character == ')' || character.is_whitespace()
            {
                if !word.is_empty() {
                    tokens.push((line_number, Token::Word(std::mem::take(&mut word))));
                }
                match character {
                    '"' => quoted = Some(String::new()),
                    '(' => tokens.push((line_number, Token::Open)),
                    ')' => tokens.push((line_number, Token::Close)),
                    _ => {}
                }
            } else {
                word.push(character);
            }
        }
        if quoted.is_some() {
            return Err(format!(
                "line {line_number} has an unterminated quoted token"
            ));
        }
        if !word.is_empty() {
            tokens.push((line_number, Token::Word(word)));
        }
    }
    Ok(sections)
}

/// A section's tokens with its `PROP( ... )` property blocks left out.
fn without_properties(tokens: &[(usize, Token)]) -> Vec<(usize, Token)> {
    let mut kept = Vec::with_capacity(tokens.len());
    let mut depth = 0_usize;
    let mut iter = tokens.iter().peekable();
    while let Some(token) = iter.next() {
        if depth > 0 {
            match token.1 {
                Token::Open => depth += 1,
                Token::Close => depth -= 1,
                _ => {}
            }
            continue;
        }
        if matches!(&token.1, Token::Word(word) if word.eq_ignore_ascii_case("PROP"))
            && matches!(iter.peek(), Some((_, Token::Open)))
        {
            iter.next();
            depth = 1;
            continue;
        }
        kept.push(token.clone());
    }
    kept
}

/// Read the block layout Spectre and Cadence's `psf` tool write.
fn decode_blocks(text: &str, limits: PsfReadLimits) -> Result<DecodedPsfWaveforms, String> {
    let sections = block_sections(text)?;
    let section = |name: &str| {
        sections
            .iter()
            .find(|(section, _)| section == name)
            .map(|(_, tokens)| without_properties(tokens))
            .unwrap_or_default()
    };
    // The analysis the header names, when it is one a waveform is drawn over.
    let header = section("HEADER");
    let analysis = header.windows(2).find_map(|pair| match pair {
        [(_, Token::Text(key)), (_, Token::Text(value))]
            if key.eq_ignore_ascii_case("analysis type") =>
        {
            value.parse::<WaveformDomain>().ok()
        }
        _ => None,
    });
    // Which value types are complex.
    let mut complex_types = std::collections::HashSet::new();
    let types = section("TYPE");
    let mut current: Option<&str> = None;
    for (_, token) in &types {
        match token {
            Token::Text(name) => current = Some(name),
            Token::Word(word) if word.eq_ignore_ascii_case("COMPLEX") => {
                if let Some(name) = current {
                    complex_types.insert(name.to_owned());
                }
            }
            _ => {}
        }
    }
    let sweep = section("SWEEP");
    let Some((_, Token::Text(coordinate_name))) = sweep.first() else {
        return Err("PSF ASCII is missing a SWEEP axis declaration".to_owned());
    };
    // The traces in the order the VALUE section gives them: a group's members under its name.
    let trace = section("TRACE");
    let mut entries: Vec<(String, Vec<(String, bool)>)> = Vec::new();
    let mut at = 0;
    while at < trace.len() {
        let (line, Token::Text(name)) = &trace[at] else {
            return Err(format!(
                "line {} of the TRACE section does not open with a quoted name",
                trace[at].0
            ));
        };
        match trace.get(at + 1..at + 3) {
            Some([(_, Token::Word(group)), (_, Token::Word(count))])
                if group.eq_ignore_ascii_case("GROUP") =>
            {
                let count = count
                    .parse::<usize>()
                    .map_err(|_| format!("line {line} gives group '{name}' no member count"))?;
                let mut members = Vec::with_capacity(count.min(limits.max_columns));
                at += 3;
                for _ in 0..count {
                    match trace.get(at..at + 2) {
                        Some([(_, Token::Text(member)), (_, Token::Text(kind))]) => {
                            members.push((member.clone(), complex_types.contains(kind)));
                            at += 2;
                        }
                        _ => {
                            return Err(format!(
                                "group '{name}' declares {count} traces and lists fewer"
                            ));
                        }
                    }
                }
                entries.push((name.clone(), members));
            }
            _ => match trace.get(at + 1) {
                Some((_, Token::Text(kind))) => {
                    entries.push((
                        name.clone(),
                        vec![(name.clone(), complex_types.contains(kind))],
                    ));
                    at += 2;
                }
                _ => return Err(format!("line {line} gives trace '{name}' no type")),
            },
        }
    }
    let signal_names = entries
        .iter()
        .flat_map(|(_, members)| members.iter().map(|(name, _)| name.clone()))
        .collect::<Vec<_>>();
    let complex = entries
        .iter()
        .flat_map(|(_, members)| members.iter().map(|(_, complex)| *complex))
        .collect::<Vec<_>>();
    if signal_names.is_empty() {
        return Err("PSF ASCII is missing TRACE declarations".to_owned());
    }
    if signal_names.len() + 1 > limits.max_columns {
        return Err("PSF ASCII trace-count limit exceeded".to_owned());
    }
    let value = section("VALUE");
    let mut tokens = value.iter().peekable();
    let mut coordinate = Vec::new();
    let mut real = vec![Vec::new(); signal_names.len()];
    let mut imag = complex
        .iter()
        .map(|complex| complex.then(Vec::new))
        .collect::<Vec<_>>();
    let number = |tokens: &mut std::iter::Peekable<std::slice::Iter<'_, (usize, Token)>>,
                  identity: &str|
     -> Result<(f64, Option<f64>), String> {
        match tokens.next() {
            Some((line, Token::Word(word))) => Ok((parse_psf_number(word, *line, identity)?, None)),
            Some((line, Token::Open)) => match (tokens.next(), tokens.next(), tokens.next()) {
                (
                    Some((_, Token::Word(re))),
                    Some((_, Token::Word(im))),
                    Some((_, Token::Close)),
                ) => Ok((
                    parse_psf_number(re, *line, identity)?,
                    Some(parse_psf_number(im, *line, identity)?),
                )),
                _ => Err(format!(
                    "line {line} gives '{identity}' a complex value that is not (real imaginary)"
                )),
            },
            Some((line, _)) => Err(format!("line {line} gives '{identity}' no number")),
            None => Err(format!(
                "the VALUE section ends before a value of '{identity}'"
            )),
        }
    };
    while let Some((line, token)) = tokens.next() {
        let Token::Text(name) = token else {
            return Err(format!(
                "line {line} of the VALUE section does not open with a quoted name"
            ));
        };
        if name != coordinate_name {
            return Err(format!(
                "line {line} names '{name}' where a point opens with '{coordinate_name}'"
            ));
        }
        if coordinate.len() >= limits.max_rows {
            return Err("PSF ASCII row limit exceeded".to_owned());
        }
        coordinate.push(number(&mut tokens, coordinate_name)?.0);
        let mut column = 0;
        for (entry, members) in &entries {
            match tokens.next() {
                Some((_, Token::Text(given))) if given == entry => {}
                Some((line, _)) => {
                    return Err(format!(
                        "line {line} does not give '{entry}' where the TRACE section has it"
                    ));
                }
                None => {
                    return Err(format!(
                        "the VALUE section ends before a value of '{entry}'"
                    ));
                }
            }
            for (member, _) in members {
                let (re, im) = number(&mut tokens, member)?;
                real[column].push(re);
                match (&mut imag[column], im) {
                    (Some(parts), Some(im)) => parts.push(im),
                    (Some(parts), None) => parts.push(0.0),
                    (None, None) => {}
                    (None, Some(_)) => {
                        return Err(format!(
                            "'{member}' is declared real and given a complex value"
                        ));
                    }
                }
                column += 1;
            }
        }
    }
    let coordinate_name = coordinate_name.clone();
    Ok(DecodedPsfWaveforms {
        domain: analysis.unwrap_or_else(|| WaveformDomain::from_coordinate_name(&coordinate_name)),
        coordinate_name,
        coordinate,
        signal_names,
        signal_values: real,
        signal_imag: imag,
    })
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
        signal_imag: vec![None; signal_names.len()],
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
    if crate::numeric::decimal_underflowed(token, value) {
        return Err(format!(
            "line {line}: numeric token '{token}' for '{identity}' underflows at binary64 precision"
        ));
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    fn sample(coordinate: &str, value: &str) -> String {
        format!(
            "HEADER\n\"analysis\" \"tran\"\nSWEEP\n\"time\" \"s\"\nTRACE\n\"V(out)\" \"V\"\nVALUE\n({coordinate} {value})\nEND\n"
        )
    }

    /// A transient as Spectre writes it with `-format psfascii`: property blocks on the
    /// declarations, a group of traces, and every value named.
    const SPECTRE_TRAN: &str = r#"HEADER
"PSFversion" "1.00"
"PSF style" 7
"simulator" "spectre"
"analysis type" "tran"
"analysis description" "Transient Analysis `tran': time = (0 s -> 2 ns)"
TYPE
"sweep" FLOAT DOUBLE PROP(
"key" "sweep"
)
"V" FLOAT DOUBLE PROP(
"units" "V"
"key" "node"
)
"I" FLOAT DOUBLE PROP(
"units" "A"
"key" "branch"
)
SWEEP
"time" "sweep" PROP(
"key" "sweep"
)
TRACE
"group" GROUP 2
"out" "V"
"V1:p" "I"
"in" "V"
VALUE
"time" 0.000000000000000e+00
"group" 0.000000000000000e+00
-1.000000000000000e-03
"in" 1.000000000000000e+00
"time" 1.000000000000000e-09
"group" 5.000000000000000e-01
-5.000000000000000e-04
"in" 1.000000000000000e+00
END
"#;

    /// An AC analysis: complex node voltages as `(real imaginary)`.
    const SPECTRE_AC: &str = r#"HEADER
"PSFversion" "1.00"
"analysis type" "ac"
TYPE
"sweep" FLOAT DOUBLE
"V" COMPLEX DOUBLE PROP(
"units" "V"
)
SWEEP
"freq" "sweep"
TRACE
"out" "V"
VALUE
"freq" 1.000000000000000e+03
"out" (9.999e-01 -6.283e-03)
"freq" 1.000000000000000e+06
"out" (2.533e-02 -1.565e-01)
END
"#;

    fn wide() -> PsfReadLimits {
        PsfReadLimits {
            max_columns: 8,
            max_rows: 8,
        }
    }

    #[test]
    fn spectre_block_layout_reads_groups_properties_and_named_values() {
        let decoded = decode_psf_ascii(SPECTRE_TRAN.as_bytes(), wide()).unwrap();
        assert_eq!(decoded.domain, WaveformDomain::Transient);
        assert_eq!(decoded.coordinate_name, "time");
        assert_eq!(decoded.coordinate, [0.0, 1e-9]);
        assert_eq!(decoded.signal_names, ["out", "V1:p", "in"]);
        assert_eq!(decoded.signal_values[0], [0.0, 0.5]);
        assert_eq!(decoded.signal_values[1], [-1e-3, -5e-4]);
        assert_eq!(decoded.signal_values[2], [1.0, 1.0]);
        assert!(decoded.signal_imag.iter().all(Option::is_none));

        let decoded = decode_psf_ascii(SPECTRE_AC.as_bytes(), wide()).unwrap();
        assert_eq!(decoded.domain, WaveformDomain::Ac);
        assert_eq!(decoded.coordinate, [1e3, 1e6]);
        assert_eq!(decoded.signal_values[0], [9.999e-1, 2.533e-2]);
        assert_eq!(
            decoded.signal_imag[0].as_deref(),
            Some([-6.283e-3, -1.565e-1].as_slice())
        );

        let shuffled = SPECTRE_TRAN.replacen("\"in\" 1.0", "\"out\" 1.0", 1);
        let error = decode_psf_ascii(shuffled.as_bytes(), wide())
            .unwrap_err()
            .to_string();
        assert_eq!(
            error,
            "line 32 does not give 'in' where the TRACE section has it"
        );
    }

    #[test]
    fn coordinates_and_samples_refuse_decimal_underflow_with_source_context() {
        let limits = PsfReadLimits {
            max_columns: 2,
            max_rows: 1,
        };
        for literal in ["1e-999", "-1e-999", "2e-324", "-2e-324"] {
            for (source, identity) in [
                (sample(literal, "1"), "time"),
                (sample("0", literal), "V(out)"),
            ] {
                let error = decode_psf_ascii(source.as_bytes(), limits)
                    .unwrap_err()
                    .to_string();
                for expected in ["underflow", "line 8", identity, literal] {
                    assert!(error.contains(expected), "{error}");
                }
            }
        }
    }

    #[test]
    fn authored_zero_and_representable_subnormal_samples_remain_exact() {
        let limits = PsfReadLimits {
            max_columns: 2,
            max_rows: 1,
        };
        for (literal, expected) in [
            ("0e999", 0.0_f64),
            ("-0e-999", -0.0_f64),
            ("5e-324", 5e-324_f64),
            ("-5e-324", -5e-324_f64),
        ] {
            let source = sample(literal, literal);
            let data = decode_psf_ascii(source.as_bytes(), limits).unwrap();
            assert_eq!(data.coordinate[0].to_bits(), expected.to_bits());
            assert_eq!(data.signal_values[0][0].to_bits(), expected.to_bits());
        }
    }

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
