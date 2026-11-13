//! O8: QSPICE (`QSPICE64.exe`, batch), the simulator the default dialect
//! follows, run on a reduced oracle deck of the parts QSPICE models: no XSPICE
//! code models and no Verilog-AMS, so a digital block enters the deck as the
//! sources it holds at the analysis point.
//!
//! QSPICE writes `<deck>.qraw` beside the deck: an ASCII header (`Title:`,
//! `Plotname:`, `Flags:`, `No. Variables:`, `No. Points:`, `Command:`, then
//! `.param` and `.alias` lines), a `Variables:` list and `Binary:` doubles, one
//! record of `No. Variables` values per point. A subcircuit node is named
//! innermost first with byte 0x95 between levels: `V(outintp<0x95>xmod)` is
//! `v(xmod.outintp)`.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use super::{OracleAbsent, Table, column_name};

pub const DEFAULT_EXE: &str = "C:/Program Files/QSPICE/QSPICE64.exe";

/// The byte QSPICE writes between the levels of a hierarchical name.
const HIERARCHY_SEPARATOR: char = '\u{95}';

pub fn discover() -> Result<PathBuf, OracleAbsent> {
    super::discover(
        "QSPICE (O8)",
        "RSPICE_QSPICE_EXE",
        super::Fallback::Files(vec![PathBuf::from(DEFAULT_EXE)]),
        "install QSPICE and point RSPICE_QSPICE_EXE at QSPICE64.exe",
    )
}

/// What QSPICE wrote for one deck: its raw file and its console log.
pub struct Run {
    pub raw: Vec<u8>,
    pub log: String,
}

/// Run `deck`, expanded, in a fresh temporary directory: QSPICE writes its
/// `.qraw` beside the deck it is given.
pub fn run(executable: &Path, deck: &Path, timeout: Duration) -> Result<Run, String> {
    let source =
        fs::read_to_string(deck).map_err(|error| format!("{}: {error}", deck.display()))?;
    let mut processor =
        rspice_core::netlist::IncludeProcessor::new(deck.parent().unwrap_or(Path::new(".")));
    let expanded = processor
        .expand_content(&source, deck)
        .map_err(|error| format!("{}: {error}", deck.display()))?;
    let scratch = tempfile::Builder::new()
        .prefix("rspice-mixed-qspice-")
        .tempdir()
        .map_err(|error| error.to_string())?;
    let copy = scratch.path().join("oracle.cir");
    fs::write(&copy, expanded).map_err(|error| error.to_string())?;
    let log = scratch.path().join("qspice.log");
    let log_file = fs::File::create(&log).map_err(|error| error.to_string())?;
    let mut child = Command::new(executable)
        .arg(&copy)
        .current_dir(scratch.path())
        .stdin(Stdio::null())
        .stdout(Stdio::from(
            log_file.try_clone().map_err(|error| error.to_string())?,
        ))
        .stderr(Stdio::from(log_file))
        .spawn()
        .map_err(|error| format!("spawn QSPICE: {error}"))?;
    let started = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if started.elapsed() > timeout => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(format!("QSPICE timed out after {} s", timeout.as_secs()));
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(50)),
            Err(error) => return Err(error.to_string()),
        }
    };
    let log = fs::read_to_string(&log).unwrap_or_default();
    if !status.success() {
        return Err(format!("QSPICE exited with {status}: {}", log.trim()));
    }
    let raw = fs::read(scratch.path().join("oracle.qraw"))
        .map_err(|error| format!("QSPICE wrote no .qraw ({error}): {}", log.trim()))?;
    Ok(Run { raw, log })
}

/// A QSPICE run votes only on a deck it supports whole: a log line saying it
/// met something it does not know, or skipped a card, refuses the run.
pub fn refusal(log: &str) -> Option<String> {
    log.lines()
        .map(str::trim)
        .find(|line| {
            let lower = line.to_ascii_lowercase();
            lower.contains("unknown") || lower.contains("ignoring")
        })
        .map(str::to_owned)
}

/// The header text of a raw file (read byte for character, so 0x95 stays one
/// character) and its binary body.
fn split(raw: &[u8]) -> Result<(String, &[u8]), String> {
    let marker = b"Binary:\n";
    let at = raw
        .windows(marker.len())
        .position(|window| window == marker)
        .ok_or("QSPICE raw file has no Binary: section")?;
    let header = raw[..at]
        .iter()
        .map(|&byte| char::from(byte))
        .filter(|&character| character != '\r')
        .collect();
    Ok((header, &raw[at + marker.len()..]))
}

/// The build QSPICE names on its raw file's `Command:` line.
pub fn version(raw: &[u8]) -> Result<String, String> {
    let (header, _) = split(raw)?;
    header
        .lines()
        .find_map(|line| line.strip_prefix("Command:"))
        .map(|command| command.trim().to_owned())
        .ok_or_else(|| "QSPICE raw file names no Command".into())
}

/// `V(outintp<0x95>xmod)` is `v(xmod.outintp)`.
fn signal_name(raw: &str) -> String {
    let Some(inner) = raw
        .strip_prefix("V(")
        .and_then(|rest| rest.strip_suffix(')'))
    else {
        return column_name(raw);
    };
    let mut levels: Vec<&str> = inner.split(HIERARCHY_SEPARATOR).collect();
    levels.reverse();
    column_name(&format!("v({})", levels.join(".")))
}

/// The node voltages of an operating-point raw file as an `op` table.
pub fn sections(analysis: &str, raw: &[u8]) -> Result<BTreeMap<String, Table>, String> {
    let kind = crate::suites::mixed::design::analysis_kind(analysis).unwrap_or(analysis);
    if kind != "op" {
        return Err(format!(
            "the QSPICE capture reads operating points, not {kind}"
        ));
    }
    let (header, body) = split(raw)?;
    let count = |label: &str| -> Result<usize, String> {
        header
            .lines()
            .find_map(|line| line.strip_prefix(label))
            .and_then(|value| value.trim().parse().ok())
            .ok_or_else(|| format!("QSPICE raw file lacks {label}"))
    };
    let variables = count("No. Variables:")?;
    if count("No. Points:")? != 1 {
        return Err("an operating-point raw file holds one point".into());
    }
    let names: Vec<&str> = header
        .split_once("\nVariables:\n")
        .ok_or("QSPICE raw file has no Variables: list")?
        .1
        .lines()
        .take(variables)
        .map(|line| line.split('\t').nth(2).unwrap_or_default())
        .collect();
    if names.len() != variables || body.len() < 8 * variables {
        return Err("QSPICE raw file is short".into());
    }
    let mut table = Table {
        axis_name: "op".into(),
        axis: vec![0.0],
        ..Default::default()
    };
    for (index, name) in names.iter().enumerate() {
        if !name.starts_with("V(") {
            continue;
        }
        let bytes: [u8; 8] = body[8 * index..8 * index + 8]
            .try_into()
            .expect("eight bytes");
        let value = f64::from_le_bytes(bytes);
        if !value.is_finite() {
            return Err(format!("{name}: non-finite oracle value"));
        }
        table.real.insert(signal_name(name), vec![value]);
    }
    Ok(BTreeMap::from([("op".to_owned(), table)]))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A raw file as QSPICE lays one out, with two nodes in a subcircuit.
    fn raw(values: &[f64]) -> Vec<u8> {
        let mut raw = b"Title: * probe\nDate: Wed Nov 11 00:00:00 2026\n\
            Plotname: Operating Point\nFlags: real\nNo. Variables: 4\n\
            No. Points: 1                    \n\
            Command: QSPICE64, Build Oct 27 2026 09:06:44\n.param temp=27\n\
            .alias I(R1\x95X1) (0.002mho*V(a,b\x95x1))\nVariables:\n\
            \t0\tV(a)\tvoltage\n\t1\tV(b\x95x1)\tvoltage\n\
            \t2\tV(c\x95x2\x95x1)\tvoltage\n\t3\tI(V1)\tcurrent\nBinary:\n"
            .to_vec();
        for value in values {
            raw.extend_from_slice(&value.to_le_bytes());
        }
        raw
    }

    #[test]
    fn an_operating_point_reads_every_node_by_its_dotted_name() {
        let sections = sections("op", &raw(&[1.0, 0.25, -0.5, 2e-3])).unwrap();
        let table = &sections["op"];
        assert_eq!(table.axis_name, "op");
        let names: Vec<&str> = table.real.keys().map(String::as_str).collect();
        assert_eq!(names, ["v(a)", "v(x1.b)", "v(x1.x2.c)"]);
        assert_eq!(table.real["v(x1.b)"], [0.25]);
        assert_eq!(table.real["v(x1.x2.c)"], [-0.5]);
        assert_eq!(
            version(&raw(&[0.0; 4])).unwrap(),
            "QSPICE64, Build Oct 27 2026 09:06:44"
        );
    }

    #[test]
    fn a_short_or_foreign_raw_file_is_refused() {
        assert!(sections("op", &raw(&[1.0, 0.25])).is_err());
        assert!(sections("tran", &raw(&[0.0; 4])).is_err());
        assert!(sections("op", b"Title: none\n").is_err());
        assert!(sections("op", &raw(&[f64::NAN, 0.0, 0.0, 0.0])).is_err());
    }

    #[test]
    fn a_log_that_ignores_a_card_refuses_the_run() {
        assert_eq!(
            refusal("deck.cir\n\nTotal elapsed time: 0.04 seconds.\n"),
            None
        );
        assert!(refusal("Unknown device type: A1\n").is_some());
    }
}
