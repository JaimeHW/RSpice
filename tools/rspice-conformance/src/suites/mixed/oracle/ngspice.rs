//! O1 (and O5-O1): ngspice-46, run headless on an oracle deck.
//!
//! The capture appends its own `.control` block — `run`, `write` of the binary
//! rawfile, an `eprvcd` of the nets a `* rspice-events: <nets>` line in the deck
//! names, `quit` — so one deck is exactly one run of exactly its analysis card.
//! (`--batch` would run the deck's card and then the control block's `run`
//! again.) A deck that needs a control script of its own (`pre_osdi`, the Tian
//! loop-gain measurement) writes it inside `* rspice-control-begin` /
//! `* rspice-control-end` comment fences; the capture then uses those lines in
//! place of its `run`, before its `write`, so the script owns the runs. The
//! last plot the script leaves is the one written. `{ngspice_lib}` in a deck
//! names ngspice's own library directory, so no machine path is checked in.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use super::{MAX_TRANSIENT_ROWS, OracleAbsent, Producer, Table, column_name, decimate};
use crate::suites::ngspice::reference::live::{RawReferencePlot, parse_ngspice_raw_plots};

pub const DEFAULT_EXE: &str =
    "C:/Users/James/Desktop/ngspice-46-release/Spice64/bin/ngspice_con.exe";

pub fn discover() -> Result<PathBuf, OracleAbsent> {
    super::discover(
        "ngspice (O1)",
        "RSPICE_NGSPICE_EXE",
        DEFAULT_EXE,
        "install ngspice-46 and point RSPICE_NGSPICE_EXE at its console build (ngspice_con.exe on Windows, never the GUI ngspice.exe)",
    )
}

fn command(executable: &Path) -> Command {
    #[cfg_attr(not(windows), allow(unused_mut))]
    let mut command = Command::new(executable);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        // CREATE_NO_WINDOW: maintenance runs never surface a console window.
        command.creation_flags(0x0800_0000);
    }
    command
}

pub fn version(executable: &Path) -> Result<String, String> {
    let output = command(executable)
        .arg("--version")
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("ngspice --version: {error}"))?;
    let text = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    text.lines()
        .map(str::trim)
        .find(|line| line.to_ascii_lowercase().contains("ngspice-"))
        .map(|line| line.trim_matches('*').trim().to_owned())
        .ok_or_else(|| "ngspice --version reported no version".into())
}

/// What one ngspice run produced.
pub(crate) struct NgspiceRun {
    pub plots: Vec<RawReferencePlot>,
    pub vcd: Option<String>,
    /// Everything ngspice printed; O2 reads it for co-simulation load failures.
    pub log: String,
}

/// Run `deck` once with a generated control block.
pub(crate) fn run(executable: &Path, deck: &Path, timeout: Duration) -> Result<NgspiceRun, String> {
    let source =
        fs::read_to_string(deck).map_err(|error| format!("{}: {error}", deck.display()))?;
    let directory = deck.parent().ok_or("oracle deck has no directory")?;
    run_source(executable, directory, &source, timeout)
}

/// Run deck text as if it sat in `directory` (relative includes resolve there).
pub(crate) fn run_source(
    executable: &Path,
    directory: &Path,
    source: &str,
    timeout: Duration,
) -> Result<NgspiceRun, String> {
    if cfg!(windows)
        && !executable
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.eq_ignore_ascii_case("ngspice_con.exe"))
    {
        return Err("Windows capture requires ngspice_con.exe; ngspice.exe opens a GUI".into());
    }
    let scratch = tempfile::Builder::new()
        .prefix("rspice-mixed-ngspice-")
        .tempdir()
        .map_err(|error| error.to_string())?;
    let raw = scratch.path().join("oracle.raw");
    let vcd = scratch.path().join("events.vcd");
    let nets = directive(source, "rspice-events");
    let control = fenced_control(source);
    let mut body: Vec<&str> = source.lines().collect();
    while body.last().is_some_and(|line| line.trim().is_empty()) {
        body.pop();
    }
    if body
        .last()
        .is_some_and(|line| line.trim().eq_ignore_ascii_case(".end"))
    {
        body.pop();
    }
    let library = ngspice_library(executable);
    let mut text = body.join("\n").replace(NGSPICE_LIB_PLACEHOLDER, &library);
    text.push_str("\n.control\n");
    if control.is_empty() {
        text.push_str("run\n");
    }
    for line in &control {
        text.push_str(&line.replace(NGSPICE_LIB_PLACEHOLDER, &library));
        text.push('\n');
    }
    // `write` saves the current plot, and a noise run leaves the integrated
    // totals current; the spectrum is the first noise plot.
    if super::super::design::deck_cards(source)
        .iter()
        .any(|(card, _)| card == ".noise")
    {
        text.push_str("setplot noise1\n");
    }
    // Binary: the console build writes ASCII rawfiles with CRLF line ends.
    text.push_str("set filetype=binary\n");
    // `* rspice-write: <vectors>` limits the written plot (a `.sens` plot
    // lists every model parameter of every device).
    let vectors = directive(source, "rspice-write").unwrap_or_default();
    text.push_str(&format!("write {} {vectors}\n", forward(&raw)));
    if let Some(nets) = &nets {
        text.push_str(&format!("eprvcd {nets} > {}\n", forward(&vcd)));
    }
    text.push_str("quit\n.endc\n.end\n");
    // The deck must sit beside the original so relative includes resolve. Its
    // name is unique per run, so captures running side by side in one
    // process never share it.
    static RUNS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let run = RUNS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let temporary = directory.join(format!(".rspice-capture-{}-{run}.cir", std::process::id()));
    fs::write(&temporary, text).map_err(|error| error.to_string())?;
    let log_path = scratch.path().join("ngspice.log");
    let log_file = fs::File::create(&log_path).map_err(|error| error.to_string())?;
    let spawned = command(executable)
        .arg("--no-spiceinit")
        .arg(temporary.file_name().expect("named"))
        .current_dir(directory)
        .env_remove("SPICE_SCRIPTS")
        .env_remove("ngspice_vpath")
        .stdin(Stdio::null())
        .stdout(Stdio::from(
            log_file.try_clone().map_err(|error| error.to_string())?,
        ))
        .stderr(Stdio::from(log_file))
        .spawn();
    let outcome = spawned
        .map_err(|error| format!("spawn ngspice: {error}"))
        .and_then(|mut child| {
            let started = Instant::now();
            loop {
                match child.try_wait() {
                    Ok(Some(status)) => break Ok(status),
                    Ok(None) if started.elapsed() > timeout => {
                        let _ = child.kill();
                        let _ = child.wait();
                        break Err(format!("ngspice timed out after {} s", timeout.as_secs()));
                    }
                    Ok(None) => std::thread::sleep(Duration::from_millis(50)),
                    Err(error) => break Err(error.to_string()),
                }
            }
        });
    let _ = fs::remove_file(&temporary);
    let log = fs::read_to_string(&log_path).unwrap_or_default();
    let status = outcome?;
    if !status.success() {
        return Err(format!("ngspice exited with {status}: {}", tail(&log)));
    }
    let bytes = fs::read(&raw)
        .map_err(|error| format!("ngspice wrote no rawfile ({error}): {}", tail(&log)))?;
    let plots = parse_ngspice_raw_plots(&bytes)?;
    let vcd = if nets.is_some() {
        Some(fs::read_to_string(&vcd).map_err(|error| format!("ngspice wrote no VCD: {error}"))?)
    } else {
        None
    };
    Ok(NgspiceRun { plots, vcd, log })
}

/// Spelled in a deck where it needs ngspice's own library directory (OSDI
/// models), so the checked-in deck carries no machine path.
pub const NGSPICE_LIB_PLACEHOLDER: &str = "{ngspice_lib}";

/// `<bin>/../lib/ngspice` of the discovered executable.
fn ngspice_library(executable: &Path) -> String {
    let library = executable
        .parent()
        .and_then(Path::parent)
        .map(|root| root.join("lib").join("ngspice"))
        .unwrap_or_default();
    forward(&library)
}

fn forward(path: &Path) -> String {
    path.display().to_string().replace('\\', "/")
}

fn tail(log: &str) -> String {
    let lines: Vec<&str> = log.lines().filter(|line| !line.trim().is_empty()).collect();
    lines[lines.len().saturating_sub(8)..].join(" | ")
}

/// The value of a `* <name>: value` comment directive.
pub fn directive(source: &str, name: &str) -> Option<String> {
    source.lines().find_map(|line| {
        line.trim()
            .strip_prefix('*')
            .map(str::trim)
            .and_then(|rest| rest.strip_prefix(name))
            .and_then(|rest| rest.trim_start().strip_prefix(':'))
            .map(|value| value.trim().to_owned())
    })
}

fn fenced_control(source: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut inside = false;
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed == "* rspice-control-begin" {
            inside = true;
            continue;
        }
        if trimmed == "* rspice-control-end" {
            inside = false;
            continue;
        }
        if inside {
            lines.push(trimmed.strip_prefix("* ").unwrap_or(trimmed).to_owned());
        }
    }
    lines
}

/// An analysis ngspice reported it could not complete although it exited 0.
/// Its reference records the refusal and only O5 applies (design §3 C5).
pub fn refusal(analysis: &str, log: &str) -> Option<String> {
    let kind = crate::suites::mixed::design::analysis_kind(analysis).unwrap_or(analysis);
    (kind == "pz")
        .then(|| {
            log.lines()
                .map(str::trim)
                .find(|line| line.contains("Pole-zero iteration limit reached"))
                .map(|line| format!("ngspice {line}"))
        })
        .flatten()
}

/// Convert ngspice plots into reference sections for `analysis` (the kind).
pub(crate) fn sections(
    analysis: &str,
    plots: &[RawReferencePlot],
) -> Result<BTreeMap<String, Table>, String> {
    let mut sections = BTreeMap::new();
    let plot = |predicate: &dyn Fn(&str) -> bool| {
        plots
            .iter()
            .rev()
            .find(|plot| predicate(&plot.plotname.to_ascii_lowercase()))
            .ok_or_else(|| {
                format!(
                    "no matching plot among {:?}",
                    plots
                        .iter()
                        .map(|plot| plot.plotname.clone())
                        .collect::<Vec<_>>()
                )
            })
    };
    let kind = crate::suites::mixed::design::analysis_kind(analysis).unwrap_or(analysis);
    match kind {
        "op" => {
            let plot = plot(&|name| name.contains("operating point"))?;
            sections.insert("op".into(), real_table("op", plot, false)?);
        }
        "dc" => {
            let plot = plot(&|name| name.contains("dc transfer"))?;
            sections.insert("dc".into(), real_table("sweep", plot, true)?);
        }
        "tran" | "trnoise" => {
            let plot = plot(&|name| name.contains("transient"))?;
            let mut table = real_table("time", plot, true)?;
            decimate(&mut table, MAX_TRANSIENT_ROWS);
            sections.insert("tran".into(), table);
        }
        "ac" => {
            let plot = plot(&|name| name.contains("ac analysis"))?;
            sections.insert("ac".into(), complex_table(plot)?);
        }
        "stb" => {
            // The deck's Tian script leaves the loop gain as `loopgain`.
            let plot = plot(&|name| name.contains("ac analysis"))?;
            let mut table = complex_table(plot)?;
            let loopgain = table
                .complex
                .remove(&column_name("loopgain"))
                .ok_or("the Tian script left no loopgain vector")?;
            table.complex.clear();
            table.complex.insert("loopgain".into(), loopgain);
            sections.insert("stb".into(), table);
        }
        "noise" => {
            let plot = plot(&|name| name.contains("noise spectral density"))?;
            let mut table = real_table("frequency", plot, true)?;
            // ngspice reports amplitude densities (V/sqrt(Hz)); the corpus
            // compares power densities.
            let mut powers = BTreeMap::new();
            for (name, series) in &table.real {
                let name = name
                    .trim_start_matches("v(")
                    .trim_end_matches(')')
                    .replace("_spectrum", "");
                powers.insert(name, series.iter().map(|value| value * value).collect());
            }
            table.real = powers;
            sections.insert("noise".into(), table);
        }
        "tf" => {
            let plot = plot(&|name| name.contains("transfer function"))?;
            let mut table = Table {
                axis_name: "scalar".into(),
                axis: vec![0.0],
                ..Default::default()
            };
            for (index, variable) in plot.variables.iter().enumerate() {
                let lower = variable.to_ascii_lowercase();
                let name = if lower.contains("transfer_function") {
                    "gain"
                } else if lower.contains("output_impedance") {
                    "zout"
                } else if lower.contains("input_impedance") {
                    "zin"
                } else {
                    continue;
                };
                table
                    .real
                    .insert(name.into(), vec![first_value(plot, index)?.re]);
            }
            sections.insert("tf".into(), table);
        }
        "sens" => {
            let plot = plot(&|name| name.contains("sensitivity"))?;
            let mut table = Table {
                axis_name: "scalar".into(),
                axis: vec![0.0],
                ..Default::default()
            };
            // One column per element: ngspice also lists every model and
            // instance parameter (`rb0:r`, `cout_m`, thousands for BSIM4).
            for (index, variable) in plot.variables.iter().enumerate() {
                let name = variable
                    .to_ascii_lowercase()
                    .replace("v(", "")
                    .replace(')', "");
                if name.contains(':') || name.contains('_') || name.contains('#') {
                    continue;
                }
                table.real.insert(name, vec![first_value(plot, index)?.re]);
            }
            sections.insert("sens".into(), table);
        }
        "pz" => {
            let plot = plot(&|name| name.contains("pole-zero") || name.contains("pole zero"))?;
            for (label, prefix) in [("pz-poles", "pole"), ("pz-zeros", "zero")] {
                let mut re = Vec::new();
                let mut im = Vec::new();
                for (index, variable) in plot.variables.iter().enumerate() {
                    let lower = variable.to_ascii_lowercase();
                    if lower.trim_start_matches("v(").starts_with(prefix) {
                        let root = first_value(plot, index)?;
                        re.push(root.re);
                        im.push(root.im);
                    }
                }
                let mut table = Table {
                    axis_name: "root".into(),
                    axis: (0..re.len()).map(|k| k as f64).collect(),
                    ..Default::default()
                };
                table.real.insert("re".into(), re);
                table.real.insert("im".into(), im);
                sections.insert(label.into(), table);
            }
        }
        other => return Err(format!("ngspice capture does not handle {other}")),
    }
    Ok(sections)
}

/// The single value of a one-point plot's vector.
fn first_value(plot: &RawReferencePlot, index: usize) -> Result<num_complex::Complex64, String> {
    plot.data
        .get(index)
        .and_then(|series| series.first())
        .copied()
        .filter(|value| value.re.is_finite() && value.im.is_finite())
        .ok_or_else(|| format!("{}: no finite value", plot.variables[index]))
}

fn real_table(axis_name: &str, plot: &RawReferencePlot, has_axis: bool) -> Result<Table, String> {
    let mut table = Table {
        axis_name: axis_name.into(),
        ..Default::default()
    };
    let start = usize::from(has_axis);
    if has_axis {
        table.axis = plot
            .data
            .first()
            .ok_or("plot has no axis")?
            .iter()
            .map(|value| value.re)
            .collect();
    } else {
        table.axis = vec![0.0];
    }
    for (index, variable) in plot.variables.iter().enumerate().skip(start) {
        let series: Vec<f64> = plot.data[index].iter().map(|value| value.re).collect();
        if series.iter().any(|value| !value.is_finite()) {
            return Err(format!("{variable}: non-finite oracle samples"));
        }
        table.real.insert(column_name(variable), series);
    }
    Ok(table)
}

fn complex_table(plot: &RawReferencePlot) -> Result<Table, String> {
    let mut table = Table {
        axis_name: "frequency".into(),
        axis: plot
            .data
            .first()
            .ok_or("plot has no axis")?
            .iter()
            .map(|value| value.re)
            .collect(),
        ..Default::default()
    };
    for (index, variable) in plot.variables.iter().enumerate().skip(1) {
        let series: Vec<[f64; 2]> = plot.data[index]
            .iter()
            .map(|value| [value.re, value.im])
            .collect();
        if series
            .iter()
            .any(|[re, im]| !re.is_finite() || !im.is_finite())
        {
            return Err(format!("{variable}: non-finite oracle samples"));
        }
        table.complex.insert(column_name(variable), series);
    }
    Ok(table)
}

pub const PRODUCER: Producer = Producer::Ngspice;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_directives_and_control_fences_are_read_from_comments() {
        let deck = "title\n* rspice-events: dv dclk\nR1 a 0 1\n* rspice-control-begin\n* let x = 1\n* rspice-control-end\n.end\n";
        assert_eq!(directive(deck, "rspice-events").as_deref(), Some("dv dclk"));
        assert_eq!(directive(deck, "rspice-other"), None);
        assert_eq!(fenced_control(deck), vec!["let x = 1".to_owned()]);
    }
}
