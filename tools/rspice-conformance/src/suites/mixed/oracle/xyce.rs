//! O4 (and O5-O4): Xyce 7.10, run on an oracle deck that carries its own
//! `.PRINT` lines in standard format. The `.prn` table is read here rather than
//! through the Xyce suite's reader, which is private to that suite and also
//! interprets per-deck contracts this corpus does not use.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use super::{MAX_TRANSIENT_ROWS, OracleAbsent, Table, column_name, decimate};

pub const DEFAULT_EXE: &str = "C:/Users/James/Desktop/XyceNF-7.10.0/bin/Xyce.exe";

pub fn discover() -> Result<PathBuf, OracleAbsent> {
    super::discover(
        "Xyce (O4)",
        "RSPICE_XYCE_EXE",
        DEFAULT_EXE,
        "install Xyce 7.10 and point RSPICE_XYCE_EXE at Xyce.exe",
    )
}

pub fn version(executable: &Path) -> Result<String, String> {
    let output = Command::new(executable)
        .arg("-v")
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("Xyce -v: {error}"))?;
    let text = String::from_utf8_lossy(&output.stdout).to_string();
    text.lines()
        .map(str::trim)
        .find(|line| line.to_ascii_lowercase().contains("xyce"))
        .map(str::to_owned)
        .ok_or_else(|| "Xyce -v reported no version".into())
}

/// Run `deck` and return the `.prn` text of its single `.PRINT`.
pub fn run(executable: &Path, deck: &Path, timeout: Duration) -> Result<String, String> {
    let directory = deck.parent().ok_or("oracle deck has no directory")?;
    let scratch = tempfile::Builder::new()
        .prefix("rspice-mixed-xyce-")
        .tempdir()
        .map_err(|error| error.to_string())?;
    let prn = scratch.path().join("oracle");
    let log = scratch.path().join("xyce.log");
    let log_file = fs::File::create(&log).map_err(|error| error.to_string())?;
    let mut child = Command::new(executable)
        .arg("-o")
        .arg(&prn)
        .arg(deck.file_name().ok_or("unnamed deck")?)
        .current_dir(directory)
        .stdin(Stdio::null())
        .stdout(Stdio::from(
            log_file.try_clone().map_err(|error| error.to_string())?,
        ))
        .stderr(Stdio::from(log_file))
        .spawn()
        .map_err(|error| format!("spawn Xyce: {error}"))?;
    let started = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) if started.elapsed() > timeout => {
                let _ = child.kill();
                let _ = child.wait();
                break None;
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(50)),
            Err(error) => return Err(error.to_string()),
        }
    };
    // `.NOISE` also writes `<deck>_noise.dat` beside the deck whatever `-o`
    // says; the deck runs in place for its relative includes, so remove it.
    if let Some(name) = deck.file_name() {
        let mut side = name.to_os_string();
        side.push("_noise.dat");
        let _ = fs::remove_file(directory.join(side));
    }
    let Some(status) = status else {
        return Err(format!("Xyce timed out after {} s", timeout.as_secs()));
    };
    let log = fs::read_to_string(&log).unwrap_or_default();
    if !status.success() {
        let tail: Vec<&str> = log.lines().rev().take(8).collect();
        return Err(format!("Xyce exited with {status}: {}", tail.join(" | ")));
    }
    // `-o` names the base; Xyce writes `<base>.prn` for time and DC output,
    // `<base>.FD.prn` for AC and `<base>.NOISE.prn` for noise. The deck has
    // one `.PRINT`, so exactly one table is expected.
    let tables: Vec<PathBuf> = fs::read_dir(scratch.path())
        .map_err(|error| error.to_string())?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|extension| extension == "prn"))
        .collect();
    match tables.as_slice() {
        [table] => fs::read_to_string(table).map_err(|error| error.to_string()),
        [] => Err(format!(
            "Xyce wrote no .prn output: {}",
            log.lines().rev().take(4).collect::<Vec<_>>().join(" | ")
        )),
        many => Err(format!(
            "Xyce wrote {} .prn tables; an oracle deck prints once",
            many.len()
        )),
    }
}

/// Parse a standard-format `.prn` into a section for `kind`.
pub fn sections(kind: &str, prn: &str, deck: &str) -> Result<BTreeMap<String, Table>, String> {
    let mut lines = prn.lines().filter(|line| !line.trim().is_empty());
    let header: Vec<String> = lines
        .next()
        .ok_or("empty .prn")?
        .split_whitespace()
        .map(str::to_owned)
        .collect();
    if header.first().map(String::as_str) != Some("Index") || header.len() < 2 {
        return Err(format!("unexpected .prn header {header:?}"));
    }
    let mut columns: Vec<Vec<f64>> = vec![Vec::new(); header.len()];
    for line in lines {
        if line.starts_with("End of Xyce") {
            break;
        }
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() != header.len() {
            return Err(format!(
                ".prn row has {} fields, header {}",
                fields.len(),
                header.len()
            ));
        }
        for (column, field) in columns.iter_mut().zip(&fields) {
            let value: f64 = field
                .parse()
                .map_err(|_| format!("bad .prn number {field}"))?;
            if !value.is_finite() {
                return Err(format!("non-finite .prn value {field}"));
            }
            column.push(value);
        }
    }
    let kind = crate::suites::mixed::design::analysis_kind(kind).unwrap_or(kind);
    let (label, axis_name) = match kind {
        "op" => ("op", "op"),
        "dc" => ("dc", "sweep"),
        "tran" | "trnoise" => ("tran", "time"),
        "ac" => ("ac", "frequency"),
        "noise" => ("noise", "frequency"),
        "sens" | "tf" => (kind, "scalar"),
        other => return Err(format!("Xyce capture does not handle {other}")),
    };
    let mut table = Table {
        axis_name: axis_name.into(),
        ..Default::default()
    };
    let has_axis = !matches!(axis_name, "op" | "scalar");
    // A standard-format DC table prints only the requested outputs, not the
    // swept value, so the sweep comes from the deck's own `.dc` card.
    let swept = kind == "dc";
    let first = if has_axis && !swept { 2 } else { 1 };
    table.axis = if swept {
        dc_axis(deck, columns[0].len())?
    } else if has_axis {
        columns[1].clone()
    } else {
        vec![0.0]
    };
    let mut index = first;
    while index < header.len() {
        let name = header[index].to_ascii_lowercase();
        if kind == "ac" {
            // AC prints re/im pairs: `Re(V(OUT)) Im(V(OUT))`.
            if let Some(inner) = name
                .strip_prefix("re(")
                .and_then(|rest| rest.strip_suffix(')'))
            {
                let im = columns.get(index + 1).ok_or("Re column without Im")?;
                table.complex.insert(
                    column_name(inner),
                    columns[index]
                        .iter()
                        .zip(im)
                        .map(|(re, im)| [*re, *im])
                        .collect(),
                );
                index += 2;
                continue;
            }
        }
        let series = if has_axis {
            columns[index].clone()
        } else {
            vec![*columns[index].last().ok_or("empty column")?]
        };
        let key = if kind == "noise" {
            // Xyce prints amplitude densities; the corpus compares powers.
            let key = name
                .replace("_spectrum", "")
                .replace("v(", "")
                .replace(')', "");
            table
                .real
                .insert(key, series.iter().map(|value| value * value).collect());
            index += 1;
            continue;
        } else if matches!(kind, "sens" | "tf") {
            name
        } else {
            column_name(&name)
        };
        table.real.insert(key, series);
        index += 1;
    }
    if kind == "tran" || kind == "trnoise" {
        decimate(&mut table, MAX_TRANSIENT_ROWS);
    }
    let mut sections = BTreeMap::new();
    sections.insert(label.to_owned(), table);
    Ok(sections)
}

/// The sweep values of the deck's single linear `.dc` card.
fn dc_axis(deck: &str, rows: usize) -> Result<Vec<f64>, String> {
    let cards = crate::suites::mixed::design::deck_cards(deck);
    let [(_, card)] = cards.as_slice() else {
        return Err("a DC oracle deck needs exactly one .dc card".into());
    };
    let words: Vec<&str> = card.split_whitespace().collect();
    let value = |index: usize| -> Result<f64, String> {
        words
            .get(index)
            .and_then(|word| rspice_core::netlist::lexer::parse_spice_value_complete(word).ok())
            .ok_or_else(|| format!("cannot read the .dc card {card:?}"))
    };
    let (start, stop, step) = (value(2)?, value(3)?, value(4)?);
    let count = ((stop - start) / step).round() as usize + 1;
    if count != rows {
        return Err(format!(
            "the .dc card spans {count} points but Xyce printed {rows}"
        ));
    }
    Ok((0..count).map(|k| start + k as f64 * step).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_prn_tables_parse() {
        let prn = "Index   TIME   V(OUT)   I(VDD)\n0 0.0 1.0 -1e-3\n1 1e-9 1.5 -2e-3\nEnd of Xyce(TM) Simulation\n";
        let sections = sections("tran", prn, "").unwrap();
        let table = &sections["tran"];
        assert_eq!(table.axis, vec![0.0, 1e-9]);
        assert_eq!(table.real["v(out)"], vec![1.0, 1.5]);
        assert_eq!(table.real["i(vdd)"], vec![-1e-3, -2e-3]);
        let ac = "Index FREQ Re(V(OUT)) Im(V(OUT))\n0 10 1 -1\nEnd of Xyce(TM) Simulation\n";
        assert_eq!(
            super::sections("ac", ac, "").unwrap()["ac"].complex["v(out)"],
            vec![[1.0, -1.0]]
        );
        assert!(super::sections("tran", "Index TIME\n0 nan\n", "").is_err());
        let dc = "Index V(OUT)\n0 1.0\n1 1.5\n2 2.0\nEnd of Xyce(TM) Simulation\n";
        let table = &super::sections("dc-line", dc, "t\nV1 a 0 1\n.dc v1 1.4 1.6 0.1\n.end\n")
            .unwrap()["dc"];
        assert_eq!(table.axis.len(), 3);
        assert!((table.axis[2] - 1.6).abs() < 1e-12);
        assert_eq!(table.real["v(out)"], vec![1.0, 1.5, 2.0]);
    }
}
