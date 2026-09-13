//! Analysis-addressed references. Axis names alone do not identify an analysis.

use rspice_core::netlist::{AnalysisCommand, DcSweepMode, FreqVariation};
use std::collections::{BTreeMap, BTreeSet};

pub(super) const HEADER: &str = "# RSPICE-NGSPICE-ORACLE 2";

pub(super) fn source_fingerprint(source: &str) -> String {
    blake3::hash(source.replace("\r\n", "\n").as_bytes())
        .to_hex()
        .to_string()
}

/// A canonical, executable SPICE command, with every resolved argument retained.
/// The source fingerprint additionally binds options, models and output requests.
pub(super) fn analysis_key(analysis: &AnalysisCommand) -> Result<Option<String>, String> {
    let frequency = |kind, variation: &FreqVariation, points, start: &f64, stop: &f64| {
        let variation = match variation {
            FreqVariation::Lin => "lin",
            FreqVariation::Dec => "dec",
            FreqVariation::Oct => "oct",
        };
        format!(".{kind} {variation} {points} {start:.17e} {stop:.17e}")
    };
    let key = match analysis {
        AnalysisCommand::Op => ".op".to_string(),
        AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            mode,
            sweep2,
        } => {
            if *mode != DcSweepMode::Linear
                || sweep2
                    .as_ref()
                    .is_some_and(|outer| outer.mode != DcSweepMode::Linear)
            {
                return Err("analysis-addressed ngspice capture requires a linear DC sweep; non-linear sweep modes need their own execution contract".into());
            }
            let mut key = format!(
                ".dc {} {start:.17e} {stop:.17e} {step:.17e}",
                source.to_ascii_lowercase()
            );
            if let Some(outer) = sweep2 {
                key.push_str(&format!(
                    " {} {:.17e} {:.17e} {:.17e}",
                    outer.source.to_ascii_lowercase(),
                    outer.start,
                    outer.stop,
                    outer.step
                ));
            }
            key
        }
        AnalysisCommand::Ac {
            variation,
            points,
            start_freq,
            stop_freq,
        } => frequency("ac", variation, *points, start_freq, stop_freq),
        AnalysisCommand::Sp {
            variation,
            points,
            start_freq,
            stop_freq,
            do_noise,
        } => format!(
            "{} {}",
            frequency("sp", variation, *points, start_freq, stop_freq),
            u8::from(*do_noise)
        ),
        AnalysisCommand::Tran {
            step,
            stop,
            start,
            max_step,
            uic,
        } => {
            let mut key = format!(".tran {step:.17e} {stop:.17e}");
            if start.is_some() || max_step.is_some() {
                key.push_str(&format!(" {:.17e}", start.unwrap_or(0.0)));
            }
            if let Some(max_step) = max_step {
                key.push_str(&format!(" {max_step:.17e}"));
            }
            if *uic {
                key.push_str(" uic");
            }
            key
        }
        AnalysisCommand::Noise {
            output_node,
            reference_node,
            input_source,
            variation,
            points,
            start_freq,
            stop_freq,
        } => {
            let probe = reference_node.as_ref().map_or_else(
                || output_node.to_ascii_lowercase(),
                |node| {
                    format!(
                        "{},{}",
                        output_node.to_ascii_lowercase(),
                        node.to_ascii_lowercase()
                    )
                },
            );
            let grid = frequency("noise", variation, *points, start_freq, stop_freq);
            format!(
                ".noise v({probe}) {} {}",
                input_source.to_ascii_lowercase(),
                grid.trim_start_matches(".noise ")
            )
        }
        _ => return Ok(None),
    };
    Ok(Some(key))
}

fn axis(key: &str) -> &'static str {
    match key.split_whitespace().next().unwrap_or_default() {
        ".dc" => "v-sweep",
        ".tran" => "time",
        ".op" => "op",
        _ => "frequency",
    }
}

pub(super) enum OracleReferences {
    Legacy(String),
    Scoped(BTreeMap<String, String>),
}

impl OracleReferences {
    pub(super) fn parse(
        content: &str,
        expanded_source: &str,
        analyses: &[AnalysisCommand],
    ) -> Result<Self, String> {
        let expected: BTreeSet<String> = analyses
            .iter()
            .map(analysis_key)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();
        let content = content.trim_start_matches('\u{feff}').trim_start();
        if content
            .lines()
            .next()
            .is_none_or(|line| line.trim() != HEADER)
        {
            if content.lines().next().is_some_and(|line| {
                line.starts_with("# RSPICE-NGSPICE-ORACLE ")
                    && line.trim() != "# RSPICE-NGSPICE-ORACLE 1"
            }) {
                return Err("unsupported execution-oracle format version".into());
            }
            if content.lines().any(|line| {
                line.trim().starts_with("# analysis:") || line.trim() == "# end-analysis"
            }) {
                return Err("analysis sections require execution-oracle format 2".into());
            }
            let mut axes = BTreeSet::new();
            for key in &expected {
                if !axes.insert(axis(key)) {
                    return Err(format!(
                        "legacy oracle has ambiguous analysis identity on axis {}; recapture separate analyses with format 2",
                        axis(key)
                    ));
                }
            }
            return Ok(Self::Legacy(content.to_string()));
        }

        let mut fingerprint = None;
        let mut producer = None;
        let mut sections = BTreeMap::new();
        let mut current: Option<(String, String)> = None;
        for line in content.lines().skip(1) {
            if let Some(key) = line.strip_prefix("# analysis: ") {
                if current.is_some() || key.is_empty() {
                    return Err("nested or empty oracle analysis section".into());
                }
                current = Some((key.to_string(), String::new()));
            } else if line == "# end-analysis" {
                let (key, body) = current
                    .take()
                    .ok_or("oracle analysis terminator without a section")?;
                validate_section(&key, &body)?;
                if sections.insert(key.clone(), body).is_some() {
                    return Err(format!("duplicate oracle analysis identity: {key}"));
                }
            } else if let Some((_, body)) = &mut current {
                body.push_str(line);
                body.push('\n');
            } else if let Some(value) = line.strip_prefix("# input-blake3: ") {
                if fingerprint.replace(value).is_some() {
                    return Err("duplicate oracle source fingerprint".into());
                }
            } else if let Some(value) = line.strip_prefix("# ngspice: ") {
                if value.is_empty() || producer.replace(value).is_some() {
                    return Err("missing or duplicate oracle producer".into());
                }
            } else if !line.trim().is_empty() {
                return Err(format!("unrecognized oracle bundle content: {line}"));
            }
        }
        if current.is_some() {
            return Err("unterminated oracle analysis section".into());
        }
        if producer.is_none() {
            return Err("oracle producer is missing".into());
        }
        if fingerprint != Some(source_fingerprint(expanded_source).as_str()) {
            return Err("oracle source fingerprint differs from the expanded circuit".into());
        }
        let found: BTreeSet<String> = sections.keys().cloned().collect();
        if found != expected || found.is_empty() {
            return Err(format!(
                "oracle analysis coverage differs: missing {:?}; unexpected {:?}",
                expected.difference(&found).collect::<Vec<_>>(),
                found.difference(&expected).collect::<Vec<_>>()
            ));
        }
        Ok(Self::Scoped(sections))
    }

    pub(super) fn for_analysis(&self, analysis: &AnalysisCommand) -> Result<Option<&str>, String> {
        let Some(key) = analysis_key(analysis)? else {
            return Ok(None);
        };
        match self {
            Self::Legacy(content) => Ok(Some(content)),
            Self::Scoped(sections) => sections
                .get(&key)
                .map(|content| Some(content.as_str()))
                .ok_or_else(|| format!("missing oracle analysis: {key}")),
        }
    }
}

/// Format 2 contains one complete finite table per analysis. Validate its
/// row/column structure before the legacy numeric readers can skip bad rows.
fn validate_section(key: &str, body: &str) -> Result<(), String> {
    let mut lines = body.lines().map(str::trim).filter(|line| !line.is_empty());
    let header = lines
        .next()
        .ok_or_else(|| format!("oracle analysis {key} has no output"))?;
    let mut names = BTreeSet::new();
    if axis(key) == "op" {
        if header != "Node Voltage" || lines.next() != Some("---- -------") {
            return Err(format!("{key}: invalid operating-point table header"));
        }
        for line in lines {
            let fields: Vec<_> = line.split_whitespace().collect();
            if fields.len() != 2
                || !names.insert(fields[0].to_ascii_lowercase())
                || !finite_number(fields[1])
            {
                return Err(format!("{key}: invalid operating-point reference row"));
            }
        }
        if names.is_empty() {
            return Err(format!("{key}: operating-point reference has no values"));
        }
    } else {
        let fields: Vec<_> = header.split_whitespace().collect();
        if fields.len() < 3
            || fields[0] != "Index"
            || fields[1] != axis(key)
            || fields[2..]
                .iter()
                .any(|name| !names.insert(name.to_ascii_lowercase()))
        {
            return Err(format!("{key}: invalid analysis reference table header"));
        }
        let mut rows = 0;
        for (index, line) in lines.enumerate() {
            let values: Vec<_> = line.split_whitespace().collect();
            if values.len() != fields.len()
                || values[0].parse::<usize>().ok() != Some(index)
                || values[1..].iter().any(|value| !finite_number(value))
            {
                return Err(format!(
                    "{key}: incomplete, nonfinite or out-of-order reference row {index}"
                ));
            }
            rows += 1;
        }
        if rows == 0 {
            return Err(format!("{key}: analysis reference has no samples"));
        }
    }
    Ok(())
}

fn finite_number(value: &str) -> bool {
    value.parse::<f64>().is_ok_and(f64::is_finite)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::Netlist;

    const SOURCE: &str =
        "reference identity\nV1 a 0 1\nV2 b 0 2\nR1 a b 1k\n.dc v1 0 1 1 v2 2 3 1\n.end\n";

    fn bundle(source: &str) -> (Vec<AnalysisCommand>, String, String) {
        let analyses = Netlist::parse(source).unwrap().analyses;
        let key = analysis_key(&analyses[0]).unwrap().unwrap();
        let header = format!(
            "{HEADER}\n# ngspice: analytic fixture\n# input-blake3: {}\n",
            source_fingerprint(source)
        );
        let section = format!(
            "# analysis: {key}\nIndex v-sweep v(a)\n0 0 0\n1 1 1\n2 0 0\n3 1 1\n# end-analysis\n"
        );
        (analyses, header, section)
    }

    #[test]
    fn analysis_bundle_binds_source_and_all_outer_sweep_arguments() {
        let (analyses, header, section) = bundle(SOURCE);
        let content = format!("{header}{section}");
        OracleReferences::parse(&content, SOURCE, &analyses).unwrap();
        OracleReferences::parse(&content, &SOURCE.replace('\n', "\r\n"), &analyses).unwrap();
        for changed in [
            SOURCE.replace("R1 a b 1k", "R1 a b 2k"),
            SOURCE.replace("v2 2 3 1", "v2 4 5 1"),
            SOURCE.replace("v2 2 3 1", "v1 2 3 1"),
        ] {
            let altered = Netlist::parse(&changed).unwrap().analyses;
            assert!(OracleReferences::parse(&content, &changed, &altered).is_err());
            // Even a replaced source hash cannot disguise wrong outer coordinates.
            let restamped =
                content.replace(&source_fingerprint(SOURCE), &source_fingerprint(&changed));
            if analysis_key(&altered[0]).unwrap() != analysis_key(&analyses[0]).unwrap() {
                assert!(OracleReferences::parse(&restamped, &changed, &altered).is_err());
            }
        }
    }

    #[test]
    fn analysis_bundle_rejects_incomplete_duplicate_and_nonfinite_sections() {
        let (analyses, header, section) = bundle(SOURCE);
        for content in [
            header.clone(),
            format!("{header}{section}{section}"),
            format!("{header}{}", section.replace("# end-analysis\n", "")),
            format!("{header}{}", section.replace(".dc v1", ".dc wrong")),
            format!("{header}{}", section.replace("1 1 1", "1 1 NaN")),
            format!("{header}{}", section.replace("2 0 0", "2 0")),
            format!("{header}{}", section.replace("3 1 1", "9 1 1")),
            format!(
                "{header}{}",
                section.replace("Index v-sweep v(a)", "Index frequency v(a)")
            ),
            format!("{header}{section}").replace(HEADER, "# RSPICE-NGSPICE-ORACLE 1"),
            format!("{header}{section}").replace(HEADER, "# RSPICE-NGSPICE-ORACLE 999"),
        ] {
            assert!(
                OracleReferences::parse(&content, SOURCE, &analyses).is_err(),
                "{content}"
            );
        }
    }

    #[test]
    fn canonical_capture_commands_round_trip_resolved_arguments() {
        for command in [
            ".op",
            ".dc V1 -2 2 .5 V2 3 1 -1",
            ".ac oct 3 1 8",
            ".sp lin 2 1 2 0",
            ".tran 1n 10n 2n .5n uic",
            ".noise v(a,b) V1 dec 10 1 1k",
        ] {
            let source = format!("round trip\nV1 a 0 1\nV2 b 0 2\n{command}\n.end\n");
            let parsed = Netlist::parse(&source).unwrap();
            let key = analysis_key(&parsed.analyses[0]).unwrap().unwrap();
            let reparsed =
                Netlist::parse(&format!("round trip\nV1 a 0 1\nV2 b 0 2\n{key}\n.end\n")).unwrap();
            assert_eq!(
                analysis_key(&reparsed.analyses[0]).unwrap().as_ref(),
                Some(&key)
            );
        }
    }

    #[test]
    fn legacy_reference_rejects_distinct_analyses_on_the_same_axis() {
        let source = "ambiguous\nV1 a 0 1\n.dc v1 0 1 1\n.dc v1 0 2 1\n.end\n";
        let parsed = Netlist::parse(source).unwrap();
        assert!(
            OracleReferences::parse(
                "Index v-sweep v(a)\n0 0 0\n1 1 1\n",
                source,
                &parsed.analyses
            )
            .is_err()
        );
        OracleReferences::parse(
            "Index v-sweep v(a)\n0 0 0\n1 1 1\n",
            source,
            &parsed.analyses[..1],
        )
        .unwrap();
    }
}
