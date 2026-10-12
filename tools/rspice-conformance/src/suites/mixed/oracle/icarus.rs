//! O3: live boundary replay through Icarus Verilog.
//!
//! The digital semantics of a module are checked with the analog loop cut at
//! its boundary: the runner takes the event trace RSpice produced on every
//! A/D input of the module under test, replays it as timed assignments in a
//! testbench, runs Icarus, and compares the module's outputs exactly with
//! RSpice's. Because the stimulus comes from the RSpice run, nothing can be
//! checked in; O3 runs where `iverilog` is found and is `unverified` elsewhere
//! (`RSPICE_MIXED_ORACLES_REQUIRED=1` makes absence a failure).
//!
//! `oracle/<variant>.o3/tb.v` is the template. Its header comments name the
//! sources and the replayed nets:
//!
//! ```text
//! // rspice-o3-sources: ../../net/i2c_ctrl.v ../../net/i2c_dev.v
//! // rspice-o3-inputs: scl sda_in
//! // rspice-o3-timescale: 1ps
//! ```
//!
//! and the line `// RSPICE-O3-STIMULUS` marks where the timed assignments go.
//! The testbench dumps the outputs with `$dumpfile("o3.vcd")`/`$dumpvars`.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use super::OracleAbsent;
use crate::suites::mixed::events::Trace;
use crate::suites::verilog::oracle::{OracleAvailability, VerilogEngine, detect, run_process};

pub const STIMULUS_MARKER: &str = "// RSPICE-O3-STIMULUS";

pub fn discover() -> Result<(String, String), OracleAbsent> {
    match detect(VerilogEngine::Icarus) {
        OracleAvailability::Available(tools) => {
            let compiler = tools
                .program("compiler")
                .map(|program| program.program.clone());
            let runtime = tools
                .program("runtime")
                .map(|program| program.program.clone());
            compiler.zip(runtime).ok_or_else(|| OracleAbsent {
                oracle: "Icarus Verilog (O3)",
                probed: vec!["iverilog/vvp".into()],
                hint: "install Icarus Verilog (iverilog + vvp)",
            })
        }
        other => Err(OracleAbsent {
            oracle: "Icarus Verilog (O3)",
            probed: vec![other.diagnostic()],
            hint: "install Icarus Verilog (iverilog + vvp)",
        }),
    }
}

/// Header directives of a template.
pub fn template_directive(template: &str, name: &str) -> Option<String> {
    template.lines().find_map(|line| {
        line.trim()
            .strip_prefix("//")
            .map(str::trim)
            .and_then(|rest| rest.strip_prefix(name))
            .and_then(|rest| rest.trim_start().strip_prefix(':'))
            .map(|value| value.trim().to_owned())
    })
}

/// Replace the marker with one `initial` block per replayed net: the trace's
/// values at their times, in units of `tick_seconds`.
pub fn render_testbench(
    template: &str,
    inputs: &BTreeMap<String, Trace>,
    tick_seconds: f64,
) -> Result<String, String> {
    if !template.contains(STIMULUS_MARKER) {
        return Err(format!("O3 template lacks {STIMULUS_MARKER}"));
    }
    let mut stimulus = String::new();
    for (net, trace) in inputs {
        let identifier = net.replace(['.', '[', ']'], "_");
        let _ = writeln!(stimulus, "  initial begin");
        let mut now: u64 = 0;
        for (time, value) in trace {
            let tick = (time / tick_seconds).round();
            if !(tick.is_finite() && tick >= 0.0) {
                return Err(format!("{net}: event time {time:e} is not replayable"));
            }
            let tick = tick as u64;
            let delay = tick.saturating_sub(now);
            now = tick;
            let literal = match value.as_str() {
                "0" | "1" | "x" | "z" => format!("1'b{value}"),
                other => other.to_owned(),
            };
            if delay == 0 {
                let _ = writeln!(stimulus, "    {identifier} = {literal};");
            } else {
                let _ = writeln!(stimulus, "    #{delay} {identifier} = {literal};");
            }
        }
        let _ = writeln!(stimulus, "  end");
    }
    Ok(template.replace(STIMULUS_MARKER, stimulus.trim_end()))
}

/// Compile and run a rendered testbench; returns the dumped VCD.
pub fn run(
    tools: &(String, String),
    template_dir: &Path,
    testbench: &str,
    sources: &[String],
    timeout_ms: u64,
) -> Result<String, String> {
    let scratch = tempfile::Builder::new()
        .prefix("rspice-mixed-o3-")
        .tempdir()
        .map_err(|error| error.to_string())?;
    let tb = scratch.path().join("tb.v");
    std::fs::write(&tb, testbench).map_err(|error| error.to_string())?;
    let image = scratch.path().join("o3.vvp");
    let mut args = vec![
        "-g2012".to_owned(),
        "-o".to_owned(),
        image.display().to_string(),
        tb.display().to_string(),
    ];
    for source in sources {
        args.push(template_dir.join(source).display().to_string());
    }
    let compile = run_process(&tools.0, &args, Some(scratch.path()), timeout_ms)
        .map_err(|error| error.to_string())?;
    if !compile.success {
        return Err(format!("iverilog failed: {}", compile.stderr.trim()));
    }
    let simulate = run_process(
        &tools.1,
        &[image.display().to_string()],
        Some(scratch.path()),
        timeout_ms,
    )
    .map_err(|error| error.to_string())?;
    if !simulate.success {
        return Err(format!("vvp failed: {}", simulate.stderr.trim()));
    }
    std::fs::read_to_string(scratch.path().join("o3.vcd"))
        .map_err(|error| format!("O3 testbench wrote no o3.vcd: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stimulus_replays_each_trace_with_relative_delays() {
        let template = "// rspice-o3-inputs: clk\n`timescale 1ps/1ps\nmodule tb; reg clk;\n// RSPICE-O3-STIMULUS\nendmodule\n";
        assert_eq!(
            template_directive(template, "rspice-o3-inputs").as_deref(),
            Some("clk")
        );
        let mut inputs = BTreeMap::new();
        inputs.insert(
            "clk".to_owned(),
            vec![
                (0.0, "0".to_owned()),
                (1e-9, "1".to_owned()),
                (2.5e-9, "0".to_owned()),
            ],
        );
        let rendered = render_testbench(template, &inputs, 1e-12).unwrap();
        assert!(
            rendered.contains("    clk = 1'b0;\n    #1000 clk = 1'b1;\n    #1500 clk = 1'b0;"),
            "{rendered}"
        );
        assert!(render_testbench("module tb; endmodule", &inputs, 1e-12).is_err());
    }
}
