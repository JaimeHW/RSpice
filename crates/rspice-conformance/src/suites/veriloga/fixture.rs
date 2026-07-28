//! On-disk form of a captured golden fingerprint.
//!
//! One file per model, line-oriented, sorted, and free of any value the running
//! machine could influence. Two properties matter more than compactness:
//!
//! **Exactness.** Reals are written with Rust's shortest round-tripping float
//! formatting, so a parse of the rendered text returns the identical bit
//! pattern. A fixture that quietly rounded would turn a real numerical
//! regression into a passing test.
//!
//! **Readability.** During a backend rewrite the interesting question is never
//! "did anything change" — a digest answers that — but "which entry moved and
//! by how much". Hex-encoded bits would have been shorter and would have made
//! the fixture useless for the one job it exists to do.
//!
//! Matrix blocks are stored as explicit `row,col` pairs and only for entries the
//! device actually wrote. The index set is therefore part of the fixture: an
//! entry that used to be structurally absent and now is not shows up as an added
//! line, not as a zero that silently became nonzero.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use super::golden::{GOLDEN_FORMAT_VERSION, GoldenNoiseSample, GoldenRecord};
use rspice_core::Value;

/// A model's complete captured fingerprint.
#[derive(Debug, Clone, PartialEq)]
pub struct GoldenFixture {
    pub model_name: String,
    pub node_count: usize,
    pub branch_count: usize,
    pub cases: Vec<GoldenCase>,
}

/// One parameter configuration and every bias point captured under it.
#[derive(Debug, Clone, PartialEq)]
pub struct GoldenCase {
    /// Instance parameter overrides, sorted by name. Empty means model defaults.
    pub options: Vec<(String, Value)>,
    pub points: Vec<GoldenPoint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoldenPoint {
    pub unknowns: Vec<Value>,
    pub record: GoldenRecord,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixtureParseError {
    pub line: usize,
    pub detail: String,
}

impl std::fmt::Display for FixtureParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}: {}", self.line, self.detail)
    }
}

impl std::error::Error for FixtureParseError {}

impl GoldenCase {
    /// Stable identity of the parameter configuration, used as the `case` key.
    ///
    /// Sorted so a capture cannot depend on the order overrides were discovered
    /// in, which would otherwise make the fixture churn between runs.
    pub fn label(&self) -> String {
        if self.options.is_empty() {
            return "default".to_string();
        }
        let mut sorted = self.options.clone();
        sorted.sort_by(|left, right| left.0.cmp(&right.0));
        sorted
            .iter()
            .map(|(name, value)| format!("{name}={}", render_real(*value)))
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl GoldenFixture {
    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# RSpice Verilog-A golden fingerprint.\n\
             # Written by `rspice-bench veriloga-golden capture`. Do not edit by hand.\n",
        );
        let _ = writeln!(out, "format {GOLDEN_FORMAT_VERSION}");
        let _ = writeln!(out, "model {}", self.model_name);
        let _ = writeln!(out, "nodes {}", self.node_count);
        let _ = writeln!(out, "branches {}", self.branch_count);

        for case in &self.cases {
            let _ = writeln!(out, "case {}", case.label());
            for (index, point) in case.points.iter().enumerate() {
                let _ = writeln!(out, "point {index}");
                let _ = writeln!(out, "unknowns {}", render_vector(&point.unknowns));
                let _ = writeln!(out, "rhs {}", render_indexed(&point.record.rhs));
                let _ = writeln!(
                    out,
                    "jac {}",
                    render_matrix(&point.record.jacobian, self.size())
                );
                let _ = writeln!(
                    out,
                    "cap {}",
                    render_matrix(&point.record.capacitance, self.size())
                );
                for (source, sample) in point.record.noise.iter().enumerate() {
                    let _ = writeln!(
                        out,
                        "noise {source} {} {} {} {}",
                        sample.mechanism,
                        u8::from(sample.active),
                        render_real(sample.psd),
                        sample
                            .exponent
                            .map_or_else(|| "-".to_string(), render_real),
                    );
                }
            }
        }
        out
    }

    pub fn size(&self) -> usize {
        self.node_count + self.branch_count
    }

    pub fn parse(text: &str) -> Result<Self, FixtureParseError> {
        let mut fixture = GoldenFixture {
            model_name: String::new(),
            node_count: 0,
            branch_count: 0,
            cases: Vec::new(),
        };
        let mut format_version = None;
        let mut pending: Option<GoldenPoint> = None;

        for (offset, raw) in text.lines().enumerate() {
            let line_number = offset + 1;
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let fail = |detail: String| FixtureParseError {
                line: line_number,
                detail,
            };
            let (keyword, rest) = match line.split_once(' ') {
                Some((keyword, rest)) => (keyword, rest.trim()),
                None => (line, ""),
            };

            match keyword {
                "format" => {
                    let version: u32 = rest
                        .parse()
                        .map_err(|_| fail(format!("malformed format version '{rest}'")))?;
                    if version != GOLDEN_FORMAT_VERSION {
                        return Err(fail(format!(
                            "fixture format {version} but this build reads {GOLDEN_FORMAT_VERSION}"
                        )));
                    }
                    format_version = Some(version);
                }
                "model" => fixture.model_name = rest.to_string(),
                "nodes" => {
                    fixture.node_count = rest
                        .parse()
                        .map_err(|_| fail(format!("malformed node count '{rest}'")))?
                }
                "branches" => {
                    fixture.branch_count = rest
                        .parse()
                        .map_err(|_| fail(format!("malformed branch count '{rest}'")))?
                }
                "case" => {
                    flush_point(&mut fixture, &mut pending);
                    fixture.cases.push(GoldenCase {
                        options: parse_options(rest).map_err(&fail)?,
                        points: Vec::new(),
                    });
                }
                "point" => {
                    flush_point(&mut fixture, &mut pending);
                    if fixture.cases.is_empty() {
                        return Err(fail("point before any case".to_string()));
                    }
                    pending = Some(GoldenPoint {
                        unknowns: Vec::new(),
                        record: GoldenRecord {
                            jacobian: Vec::new(),
                            rhs: Vec::new(),
                            capacitance: Vec::new(),
                            noise: Vec::new(),
                        },
                    });
                }
                "unknowns" | "rhs" | "jac" | "cap" | "noise" => {
                    let size = fixture.size();
                    let point = pending
                        .as_mut()
                        .ok_or_else(|| fail(format!("'{keyword}' outside a point")))?;
                    match keyword {
                        "unknowns" => point.unknowns = parse_vector(rest).map_err(&fail)?,
                        "rhs" => point.record.rhs = parse_indexed(rest, size).map_err(&fail)?,
                        "jac" => {
                            point.record.jacobian = parse_matrix(rest, size).map_err(&fail)?
                        }
                        "cap" => {
                            point.record.capacitance = parse_matrix(rest, size).map_err(&fail)?
                        }
                        _ => point.record.noise.push(parse_noise(rest).map_err(&fail)?),
                    }
                }
                other => return Err(fail(format!("unknown keyword '{other}'"))),
            }
        }
        flush_point(&mut fixture, &mut pending);

        if format_version.is_none() {
            return Err(FixtureParseError {
                line: 0,
                detail: "fixture has no format line".to_string(),
            });
        }
        Ok(fixture)
    }
}

fn flush_point(fixture: &mut GoldenFixture, pending: &mut Option<GoldenPoint>) {
    if let Some(point) = pending.take()
        && let Some(case) = fixture.cases.last_mut()
    {
        case.points.push(point);
    }
}

/// Shortest representation that parses back to the identical bit pattern.
fn render_real(value: Value) -> String {
    if value == 0.0 && value.is_sign_negative() {
        // `-0.0` renders as `-0` and parses back as `-0.0`, but being explicit
        // costs one character and removes any doubt when reading a diff.
        return "-0".to_string();
    }
    format!("{value:e}")
}

fn parse_real(text: &str) -> Result<Value, String> {
    text.parse::<Value>()
        .map_err(|_| format!("malformed real '{text}'"))
}

fn render_vector(values: &[Value]) -> String {
    values
        .iter()
        .map(|value| render_real(*value))
        .collect::<Vec<_>>()
        .join(" ")
}

fn parse_vector(text: &str) -> Result<Vec<Value>, String> {
    text.split_whitespace().map(parse_real).collect()
}

/// `index:value`, omitting zeros.
fn render_indexed(values: &[Value]) -> String {
    values
        .iter()
        .enumerate()
        .filter(|(_, value)| **value != 0.0)
        .map(|(index, value)| format!("{index}:{}", render_real(*value)))
        .collect::<Vec<_>>()
        .join(" ")
}

fn parse_indexed(text: &str, size: usize) -> Result<Vec<Value>, String> {
    let mut values = vec![0.0; size];
    for token in text.split_whitespace() {
        let (index, value) = token
            .split_once(':')
            .ok_or_else(|| format!("malformed indexed entry '{token}'"))?;
        let index: usize = index
            .parse()
            .map_err(|_| format!("malformed index in '{token}'"))?;
        if index >= size {
            return Err(format!("index {index} outside a vector of {size}"));
        }
        values[index] = parse_real(value)?;
    }
    Ok(values)
}

/// `row,col:value`, omitting zeros. Row-major dense on the way back in.
fn render_matrix(values: &[Value], size: usize) -> String {
    let mut entries = Vec::new();
    for row in 0..size {
        for col in 0..size {
            let value = values.get(row * size + col).copied().unwrap_or(0.0);
            if value != 0.0 {
                entries.push(format!("{row},{col}:{}", render_real(value)));
            }
        }
    }
    entries.join(" ")
}

fn parse_matrix(text: &str, size: usize) -> Result<Vec<Value>, String> {
    let mut values = vec![0.0; size * size];
    for token in text.split_whitespace() {
        let (position, value) = token
            .split_once(':')
            .ok_or_else(|| format!("malformed matrix entry '{token}'"))?;
        let (row, col) = position
            .split_once(',')
            .ok_or_else(|| format!("malformed matrix position in '{token}'"))?;
        let row: usize = row
            .parse()
            .map_err(|_| format!("malformed row in '{token}'"))?;
        let col: usize = col
            .parse()
            .map_err(|_| format!("malformed column in '{token}'"))?;
        if row >= size || col >= size {
            return Err(format!("entry ({row},{col}) outside a {size}x{size} block"));
        }
        values[row * size + col] = parse_real(value)?;
    }
    Ok(values)
}

/// `<source-index> <mechanism> <active> <psd> <exponent|->`
///
/// The source index is written for readability and checked on the way back in;
/// order is what actually identifies a source, matching the evaluation order the
/// device reports.
fn parse_noise(text: &str) -> Result<GoldenNoiseSample, String> {
    let fields: Vec<&str> = text.split_whitespace().collect();
    let [_index, mechanism, active, psd, exponent] = fields.as_slice() else {
        return Err(format!("noise line needs 5 fields, found {}", fields.len()));
    };
    Ok(GoldenNoiseSample {
        // Mechanism names come from `&'static str` descriptors in the running
        // binary; a parsed fixture carries the captured spelling instead, which
        // is what a comparison needs to notice a renamed or reordered source.
        mechanism: Box::leak(mechanism.to_string().into_boxed_str()),
        active: match *active {
            "0" => false,
            "1" => true,
            other => return Err(format!("malformed noise active flag '{other}'")),
        },
        psd: parse_real(psd)?,
        exponent: if *exponent == "-" {
            None
        } else {
            Some(parse_real(exponent)?)
        },
    })
}

fn parse_options(text: &str) -> Result<Vec<(String, Value)>, String> {
    if text == "default" {
        return Ok(Vec::new());
    }
    let mut options = BTreeMap::new();
    for token in text.split(',') {
        let (name, value) = token
            .split_once('=')
            .ok_or_else(|| format!("malformed option '{token}'"))?;
        options.insert(name.to_string(), parse_real(value)?);
    }
    Ok(options.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_fixture() -> GoldenFixture {
        GoldenFixture {
            model_name: "fixture_model".to_string(),
            node_count: 2,
            branch_count: 0,
            cases: vec![
                GoldenCase {
                    options: Vec::new(),
                    points: vec![GoldenPoint {
                        unknowns: vec![0.35, -0.125],
                        record: GoldenRecord {
                            jacobian: vec![1.5, -2.5, 0.0, 1.0e-14],
                            rhs: vec![0.0, 7.5e-9],
                            capacitance: vec![0.0, 0.0, 0.0, 3.25e-15],
                            noise: vec![GoldenNoiseSample {
                                mechanism: "WHITE_ID",
                                active: true,
                                psd: 4.0e-21,
                                exponent: None,
                            }],
                        },
                    }],
                },
                GoldenCase {
                    options: vec![("SHMOD".to_string(), 1.0)],
                    points: vec![GoldenPoint {
                        unknowns: vec![0.0, 0.0],
                        record: GoldenRecord {
                            jacobian: vec![0.0; 4],
                            rhs: vec![0.0; 2],
                            capacitance: vec![0.0; 4],
                            noise: vec![GoldenNoiseSample {
                                mechanism: "FLICKER_ID",
                                active: false,
                                psd: 0.0,
                                exponent: Some(1.25),
                            }],
                        },
                    }],
                },
            ],
        }
    }

    #[test]
    fn rendering_round_trips_exactly() {
        let fixture = sample_fixture();
        let parsed = GoldenFixture::parse(&fixture.render()).expect("fixture parses");
        assert_eq!(parsed, fixture);
    }

    #[test]
    fn rendering_is_stable() {
        let fixture = sample_fixture();
        assert_eq!(fixture.render(), fixture.render());
    }

    #[test]
    fn subnormal_and_extreme_reals_survive_a_round_trip() {
        let values = [
            f64::MIN_POSITIVE,
            f64::MIN_POSITIVE / 2.0,
            f64::MAX,
            -f64::MAX,
            1.0 / 3.0,
            -0.0,
            f64::INFINITY,
            f64::NEG_INFINITY,
        ];
        for value in values {
            let text = render_real(value);
            let parsed = parse_real(&text).expect("rendered real parses");
            assert_eq!(
                parsed.to_bits(),
                value.to_bits(),
                "{value} rendered as '{text}' and came back as {parsed}"
            );
        }
    }

    #[test]
    fn nan_is_recorded_rather_than_lost() {
        let text = render_real(f64::NAN);
        assert!(parse_real(&text).expect("NaN parses").is_nan());
    }

    #[test]
    fn a_structurally_new_matrix_entry_changes_the_text() {
        let size = 2;
        let before = render_matrix(&[1.0, 0.0, 0.0, 1.0], size);
        let after = render_matrix(&[1.0, 1.0e-30, 0.0, 1.0], size);
        assert_ne!(before, after);
        assert!(after.contains("0,1:"));
    }

    #[test]
    fn a_stale_format_version_is_refused() {
        let text = format!("format {}\nmodel x\n", GOLDEN_FORMAT_VERSION + 1);
        let error = GoldenFixture::parse(&text).expect_err("a future format must not be read");
        assert!(error.detail.contains("format"));
    }

    #[test]
    fn a_missing_format_line_is_refused() {
        let error = GoldenFixture::parse("model x\nnodes 1\n")
            .expect_err("an unversioned fixture must not be read");
        assert!(error.detail.contains("no format line"));
    }

    #[test]
    fn case_labels_are_order_independent() {
        let forward = GoldenCase {
            options: vec![("B".to_string(), 1.0), ("A".to_string(), 0.0)],
            points: Vec::new(),
        };
        let reverse = GoldenCase {
            options: vec![("A".to_string(), 0.0), ("B".to_string(), 1.0)],
            points: Vec::new(),
        };
        assert_eq!(forward.label(), reverse.label());
    }
}
