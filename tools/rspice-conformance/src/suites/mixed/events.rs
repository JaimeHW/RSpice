//! Digital event traces: RSpice's typed histories, ngspice `eprvcd` and Icarus
//! `$dumpvars` VCD files, and the checked-in `events.tsv` form, all reduced to
//! one normalized shape: per net, `(time_s, value)` with values `0 1 x z` (or a
//! real number for a real-valued net).
//!
//! XSPICE's twelve states map to four by state (0/1/X); a high-impedance
//! strength is `z`. Verilog values pass through unchanged.

use std::collections::BTreeMap;

pub const EVENT_TRACE_HEADER: &str = "# RSPICE-EVENT-TRACE 1";

pub type Trace = Vec<(f64, String)>;

/// The four-state letter for an XSPICE 12-state event code.
pub fn xspice_code_letter(code: u8) -> Option<&'static str> {
    use rspice_core::io::VcdBit;
    match rspice_core::execution::event_code_to_vcd_bit(Some(code))? {
        VcdBit::Zero => Some("0"),
        VcdBit::One => Some("1"),
        VcdBit::Unknown => Some("x"),
        VcdBit::HighImpedance => Some("z"),
    }
}

/// The four-state letter for a core digital value.
pub fn digital_value_letter(value: rspice_core::xspice::DigitalValue) -> &'static str {
    use rspice_core::io::VcdBit;
    match rspice_core::execution::digital_value_to_vcd_bit(value) {
        VcdBit::Zero => "0",
        VcdBit::One => "1",
        VcdBit::Unknown => "x",
        VcdBit::HighImpedance => "z",
    }
}

/// Drop repeated values: an event that does not change the net is not an edge.
pub fn collapse(trace: &[(f64, String)]) -> Trace {
    let mut out: Trace = Vec::with_capacity(trace.len());
    for (time, value) in trace {
        if out.last().is_some_and(|(_, last)| last == value) {
            continue;
        }
        // Equal-time updates keep only the settled value at that instant.
        if out.last().is_some_and(|(last_time, _)| last_time == time) {
            out.pop();
            if out.last().is_some_and(|(_, last)| last == value) {
                continue;
            }
        }
        out.push((*time, value.clone()));
    }
    out
}

/// Parse a VCD file into per-net traces. Scoped names are joined with `.`
/// below the top scope; vectors expand to one net per bit (`name[3]`).
pub fn parse_vcd(text: &str) -> Result<BTreeMap<String, Trace>, String> {
    let mut timescale = 1e-12f64;
    let mut ids: BTreeMap<String, Vec<(String, usize)>> = BTreeMap::new();
    let mut scopes: Vec<String> = Vec::new();
    let mut traces: BTreeMap<String, Trace> = BTreeMap::new();
    let mut now = 0.0f64;
    let mut tokens = text.split_whitespace().peekable();
    let mut in_definitions = true;
    while let Some(token) = tokens.next() {
        if in_definitions {
            match token {
                "$timescale" => {
                    let mut spec = String::new();
                    for part in tokens.by_ref() {
                        if part == "$end" {
                            break;
                        }
                        spec.push_str(part);
                    }
                    timescale = parse_timescale(&spec)?;
                }
                "$scope" => {
                    let _kind = tokens.next();
                    let name = tokens.next().unwrap_or_default().to_owned();
                    scopes.push(name);
                    skip_to_end(&mut tokens);
                }
                "$upscope" => {
                    scopes.pop();
                    skip_to_end(&mut tokens);
                }
                "$var" => {
                    let _kind = tokens.next();
                    let width: usize = tokens
                        .next()
                        .and_then(|width| width.parse().ok())
                        .ok_or("VCD $var without a width")?;
                    let id = tokens.next().ok_or("VCD $var without an id")?.to_owned();
                    let mut reference = tokens.next().ok_or("VCD $var without a name")?.to_owned();
                    let mut range = None;
                    for part in tokens.by_ref() {
                        if part == "$end" {
                            break;
                        }
                        range = Some(part.to_owned());
                    }
                    if let Some(range) = range {
                        reference.push_str(&range);
                    }
                    let name = scoped_name(&scopes, &reference.to_ascii_lowercase());
                    let bits = if width == 1 {
                        vec![(name, 0)]
                    } else {
                        (0..width)
                            .map(|bit| {
                                (
                                    format!(
                                        "{}[{}]",
                                        name.split('[').next().unwrap_or(&name),
                                        width - 1 - bit
                                    ),
                                    bit,
                                )
                            })
                            .collect()
                    };
                    ids.entry(id).or_default().extend(bits);
                }
                "$enddefinitions" => {
                    skip_to_end(&mut tokens);
                    in_definitions = false;
                }
                _ if token.starts_with('$') => skip_to_end(&mut tokens),
                _ => {}
            }
            continue;
        }
        if token.starts_with('$') {
            if matches!(
                token,
                "$dumpvars" | "$dumpon" | "$dumpoff" | "$dumpall" | "$end"
            ) {
                continue;
            }
            skip_to_end(&mut tokens);
            continue;
        }
        if let Some(time) = token.strip_prefix('#') {
            let ticks: f64 = time.parse().map_err(|_| format!("bad VCD time {token}"))?;
            now = ticks * timescale;
            continue;
        }
        let first = token.chars().next().unwrap_or(' ');
        match first {
            '0' | '1' | 'x' | 'X' | 'z' | 'Z' => {
                let id = &token[1..];
                if let Some(nets) = ids.get(id) {
                    for (net, _) in nets {
                        traces
                            .entry(net.clone())
                            .or_default()
                            .push((now, first.to_ascii_lowercase().to_string()));
                    }
                }
            }
            'b' | 'B' => {
                let value = &token[1..];
                let id = tokens.next().ok_or("VCD vector value without an id")?;
                if let Some(nets) = ids.get(id) {
                    let width = nets.len();
                    let padded = pad_vector(value, width);
                    for (net, bit) in nets {
                        let letter = padded.chars().nth(*bit).unwrap_or('x').to_ascii_lowercase();
                        traces
                            .entry(net.clone())
                            .or_default()
                            .push((now, letter.to_string()));
                    }
                }
            }
            'r' | 'R' => {
                let value = &token[1..];
                let id = tokens.next().ok_or("VCD real value without an id")?;
                if let Some(nets) = ids.get(id) {
                    for (net, _) in nets {
                        traces
                            .entry(net.clone())
                            .or_default()
                            .push((now, value.to_owned()));
                    }
                }
            }
            _ => return Err(format!("unrecognized VCD value change {token}")),
        }
    }
    Ok(traces
        .into_iter()
        .map(|(net, trace)| (net, collapse(&trace)))
        .collect())
}

fn skip_to_end<'a>(tokens: &mut impl Iterator<Item = &'a str>) {
    for part in tokens.by_ref() {
        if part == "$end" {
            break;
        }
    }
}

fn scoped_name(scopes: &[String], reference: &str) -> String {
    // The top scope is the testbench or ngspice's own wrapper; nets below it
    // keep their hierarchical path.
    if scopes.len() <= 1 {
        reference.to_owned()
    } else {
        let mut name = scopes[1..].join(".").to_ascii_lowercase();
        name.push('.');
        name.push_str(reference);
        name
    }
}

fn pad_vector(value: &str, width: usize) -> String {
    if value.len() >= width {
        return value[value.len() - width..].to_owned();
    }
    let fill = match value.chars().next() {
        Some('x' | 'X') => 'x',
        Some('z' | 'Z') => 'z',
        _ => '0',
    };
    let mut padded: String = std::iter::repeat_n(fill, width - value.len()).collect();
    padded.push_str(value);
    padded
}

fn parse_timescale(spec: &str) -> Result<f64, String> {
    let spec = spec.trim();
    let digits: String = spec.chars().take_while(|c| c.is_ascii_digit()).collect();
    let unit = spec[digits.len()..].trim();
    let magnitude: f64 = digits
        .parse()
        .map_err(|_| format!("bad VCD timescale {spec}"))?;
    let scale = match unit {
        "s" => 1.0,
        "ms" => 1e-3,
        "us" => 1e-6,
        "ns" => 1e-9,
        "ps" => 1e-12,
        "fs" => 1e-15,
        _ => return Err(format!("bad VCD timescale unit {unit}")),
    };
    Ok(magnitude * scale)
}

/// Serialize traces as `events.tsv`.
pub fn write_events_tsv(header: &[String], traces: &BTreeMap<String, Trace>) -> String {
    let mut out = String::from(EVENT_TRACE_HEADER);
    out.push('\n');
    for line in header {
        out.push_str("# ");
        out.push_str(line);
        out.push('\n');
    }
    for (net, trace) in traces {
        for (time, value) in trace {
            out.push_str(&format!("{time:.15e}\t{net}\t{value}\n"));
        }
    }
    out
}

/// Parse `events.tsv`; header comment lines are returned separately.
pub fn read_events_tsv(text: &str) -> Result<(Vec<String>, BTreeMap<String, Trace>), String> {
    let mut lines = text.lines();
    if lines.next().map(str::trim) != Some(EVENT_TRACE_HEADER) {
        return Err("event trace header is missing".into());
    }
    let mut header = Vec::new();
    let mut traces: BTreeMap<String, Trace> = BTreeMap::new();
    for line in lines {
        if let Some(comment) = line.strip_prefix("# ") {
            header.push(comment.to_owned());
            continue;
        }
        if line.trim().is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        let [time, net, value] = fields.as_slice() else {
            return Err(format!("malformed event row {line:?}"));
        };
        let time: f64 = time.parse().map_err(|_| format!("bad event time {time}"))?;
        if !time.is_finite() {
            return Err(format!("non-finite event time in {line:?}"));
        }
        traces
            .entry((*net).to_owned())
            .or_default()
            .push((time, (*value).to_owned()));
    }
    Ok((header, traces))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vcd_scalars_vectors_and_scopes_normalize() {
        let vcd = "$timescale 1ps $end\n$scope module top $end\n$var wire 1 ! clk $end\n$scope module dut $end\n$var reg 2 \" q [1:0] $end\n$upscope $end\n$upscope $end\n$enddefinitions $end\n#0\n$dumpvars\n0!\nbx \"\n$end\n#1000\n1!\nb10 \"\n#2000\n0!\nb1 \"\n";
        let traces = parse_vcd(vcd).unwrap();
        assert_eq!(
            traces["clk"],
            vec![(0.0, "0".into()), (1e-9, "1".into()), (2e-9, "0".into())]
        );
        assert_eq!(
            traces["dut.q[1]"],
            vec![(0.0, "x".into()), (1e-9, "1".into()), (2e-9, "0".into())]
        );
        assert_eq!(
            traces["dut.q[0]"],
            vec![(0.0, "x".into()), (1e-9, "0".into()), (2e-9, "1".into())]
        );
    }

    #[test]
    fn collapse_drops_repeats_and_keeps_the_settled_equal_time_value() {
        let trace = vec![
            (0.0, "0".to_owned()),
            (1.0, "0".to_owned()),
            (2.0, "1".to_owned()),
            (2.0, "0".to_owned()),
            (3.0, "1".to_owned()),
        ];
        assert_eq!(collapse(&trace), vec![(0.0, "0".into()), (3.0, "1".into())]);
    }

    #[test]
    fn events_tsv_round_trips() {
        let mut traces = BTreeMap::new();
        traces.insert(
            "dv".to_owned(),
            vec![(1e-9, "1".to_owned()), (3e-9, "0".to_owned())],
        );
        let text = write_events_tsv(&["ngspice: test".to_owned()], &traces);
        let (header, read) = read_events_tsv(&text).unwrap();
        assert_eq!(header, vec!["ngspice: test".to_owned()]);
        assert_eq!(read, traces);
        assert!(read_events_tsv("no header\n").is_err());
    }

    #[test]
    fn xspice_codes_map_to_four_states() {
        use rspice_core::xspice::{DigitalState, DigitalStrength, DigitalValue};
        let one = DigitalValue::new(DigitalState::One, DigitalStrength::Strong);
        assert_eq!(digital_value_letter(one), "1");
        assert_eq!(xspice_code_letter(one.event_code()), Some("1"));
        assert_eq!(digital_value_letter(DigitalValue::high_z()), "z");
    }
}
