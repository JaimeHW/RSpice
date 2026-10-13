//! The selection run through Icarus Verilog itself: a check of this harness,
//! not of RSpice.
//!
//! The suite's gold files and `PASSED` lines were written against Icarus, so
//! every selected test must pass when Icarus produces the output and this
//! harness judges it. A test that does not points at the harness: a list
//! misread, a gold file looked for in the wrong place, a flag or plusarg not
//! passed through. Each test is compiled and run the way the suite's own
//! runners do it (`vvp_reg.pl` for the text lists, `vvp_reg.py` for the JSON
//! list), from the checkout, with its log assembled the same way.
//!
//! It also measures what the normalization leaves behind. A gold file is an
//! Icarus log, and Icarus's own messages in it that the fixed rule set does
//! not drop are text RSpice can never print. A gold test whose compiler output
//! (and, for the JSON list, whose run's standard error) still holds a line
//! after normalization is counted as `residual`: its gold file is out of
//! RSpice's reach as written. The text lists log the run's two streams as one,
//! so what the runtime itself prints there is not separable and not counted.

use std::collections::VecDeque;
use std::fs::{self, File};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex, mpsc};
use std::time::{Duration, Instant};

use super::judge::{self, Outcome, RunEnd};
use super::list::{Check, SourceList};
use super::runner;
use super::select::{Candidate, Class};
use crate::suites::verilog::oracle::OracleTools;

/// One test under Icarus.
#[derive(Debug, Clone)]
pub struct IcarusReport {
    pub name: String,
    pub outcome: Outcome,
    pub detail: String,
    /// Icarus text in its log that the normalization keeps.
    pub residual: bool,
    pub elapsed_s: f64,
}

/// Run the selected candidates through Icarus, `jobs` at a time.
pub fn run(
    root: &Path,
    tools: &OracleTools,
    candidates: &[Candidate],
    jobs: usize,
    budget: Duration,
    mut progress: impl FnMut(&IcarusReport),
) -> Result<Vec<IcarusReport>, String> {
    let compiler = tools
        .program("compiler")
        .ok_or("the Icarus toolchain has no compiler")?
        .program
        .clone();
    let runtime = tools
        .program("runtime")
        .ok_or("the Icarus toolchain has no runtime")?
        .program
        .clone();
    let scratch = tempfile::Builder::new()
        .prefix("rspice-ivtest-icarus-")
        .tempdir()
        .map_err(|error| format!("temporary directory: {error}"))?;
    let selected: VecDeque<&Candidate> = candidates
        .iter()
        .filter(|candidate| candidate.selected())
        .collect();
    let workers = jobs.max(1).min(selected.len());
    let queue = Arc::new(Mutex::new(selected));
    let (sender, receiver) = mpsc::channel();
    let mut reports = Vec::new();
    std::thread::scope(|scope| {
        for _ in 0..workers {
            let (sender, queue) = (sender.clone(), Arc::clone(&queue));
            let (compiler, runtime, scratch) = (&compiler, &runtime, scratch.path());
            scope.spawn(move || {
                loop {
                    let next = queue.lock().map(|mut queue| queue.pop_front());
                    let Ok(Some(candidate)) = next else {
                        break;
                    };
                    let tools = Tools {
                        compiler,
                        runtime,
                        root,
                        scratch,
                        budget,
                    };
                    if sender.send(run_one(&tools, candidate)).is_err() {
                        break;
                    }
                }
            });
        }
        drop(sender);
        reports.extend(receiver);
    });
    // The suite's own runners are serial, and a few tests share a scratch
    // file name under `work/` (`test_fscanf.bin`), so a test that did not
    // pass among concurrent ones runs again on its own before it counts.
    let tools = Tools {
        compiler: &compiler,
        runtime: &runtime,
        root,
        scratch: scratch.path(),
        budget,
    };
    for report in &mut reports {
        if report.outcome != Outcome::Pass
            && let Some(candidate) = candidates
                .iter()
                .find(|candidate| candidate.entry.name == report.name)
        {
            *report = run_one(&tools, candidate);
        }
    }
    reports.sort_by(|a, b| a.name.cmp(&b.name));
    for report in &reports {
        progress(report);
    }
    Ok(reports)
}

struct Tools<'a> {
    compiler: &'a str,
    runtime: &'a str,
    root: &'a Path,
    scratch: &'a Path,
    budget: Duration,
}

/// What one Icarus process wrote, its success, or why it never finished.
struct Ran {
    /// Standard output, or both streams in write order when merged.
    stdout: String,
    stderr: String,
    success: bool,
}

fn run_one(tools: &Tools<'_>, candidate: &Candidate) -> IcarusReport {
    let started = Instant::now();
    let entry = &candidate.entry;
    let class = candidate.class.unwrap_or(Class::Passed);
    let json = entry.list == SourceList::Vvp;
    let object = tools.scratch.join(format!("{}.vvp", entry.name));
    let mut compile = vec!["-o".to_owned(), object.to_string_lossy().into_owned()];
    // `vvp_reg.pl` defines this for every test it runs; `vvp_reg.py` does not.
    if !json {
        compile.push("-D__ICARUS_UNSIZED__".to_owned());
    }
    compile.extend(entry.args.iter().cloned());
    if let Some(module) = &entry.module {
        compile.extend(["-s".to_owned(), module.clone()]);
    }
    compile.push(format!(
        "./{}",
        entry.source.to_string_lossy().replace('\\', "/")
    ));
    // The text lists judge one log of every stream in write order; the JSON
    // list judges the run's standard output alone.
    let compiled = match spawn(
        tools,
        tools.compiler,
        &compile,
        &entry.name,
        "compile",
        !json,
    ) {
        Ok(ran) => ran,
        Err(end) => return finish(tools, candidate, end, String::new(), started),
    };
    let compiler_text = format!("{}{}", compiled.stdout, compiled.stderr);
    if !compiled.success {
        let end = RunEnd::CompileError {
            message: compiler_text.clone(),
        };
        return finish(tools, candidate, end, compiler_text, started);
    }
    if !class.runs() {
        let end = RunEnd::Completed {
            end: "compiled".to_owned(),
            stdout: String::new(),
            files: Vec::new(),
        };
        return finish(tools, candidate, end, compiler_text, started);
    }
    let (before, after) = &entry.runtime_flags;
    let mut simulate = before.clone();
    simulate.push(object.to_string_lossy().into_owned());
    simulate.extend(after.iter().cloned());
    simulate.extend(entry.plusargs.iter().cloned());
    let ran = match spawn(tools, tools.runtime, &simulate, &entry.name, "run", !json) {
        Ok(ran) => ran,
        Err(end) => return finish(tools, candidate, end, compiler_text, started),
    };
    let _ = fs::remove_file(&object);
    let mut residual = compiler_text;
    residual.push_str(&ran.stderr);
    let end = if ran.success {
        let stdout = if json {
            ran.stdout
        } else {
            format!("{}{}{}", compiled.stdout, compiled.stderr, ran.stdout)
        };
        let files = match &entry.check {
            Check::Diff { produced, .. } => fs::read(tools.root.join(produced))
                .map(|bytes| {
                    vec![judge::ProducedFile {
                        path: produced.clone(),
                        text: String::from_utf8_lossy(&bytes).into_owned(),
                    }]
                })
                .unwrap_or_default(),
            _ => Vec::new(),
        };
        RunEnd::Completed {
            end: "finish".to_owned(),
            stdout,
            files,
        }
    } else {
        RunEnd::RuntimeError {
            message: format!("{}{}", ran.stdout, ran.stderr),
        }
    };
    finish(tools, candidate, end, residual, started)
}

fn finish(
    tools: &Tools<'_>,
    candidate: &Candidate,
    end: RunEnd,
    icarus_text: String,
    started: Instant,
) -> IcarusReport {
    let class = candidate.class.unwrap_or(Class::Passed);
    let (outcome, detail) = match runner::expectation(tools.root, candidate) {
        Err(detail) => (Outcome::Fail, detail),
        Ok(expectation) => judge::combine(judge::judge(class, &expectation, &end), None),
    };
    let residual = class == Class::Gold
        && judge::normalize(&icarus_text)
            .iter()
            .any(|line| !line.trim().is_empty());
    IcarusReport {
        name: candidate.entry.name.clone(),
        outcome,
        detail,
        residual,
        elapsed_s: started.elapsed().as_secs_f64(),
    }
}

/// Run one Icarus program from the checkout under the budget. With `merged`
/// both streams go to one file, in the order they were written, as the
/// suite's Perl runner logs them.
fn spawn(
    tools: &Tools<'_>,
    program: &str,
    args: &[String],
    name: &str,
    step: &str,
    merged: bool,
) -> Result<Ran, RunEnd> {
    let crash = |message: String| RunEnd::Crash { message };
    let out_path = tools.scratch.join(format!("{name}.{step}.out"));
    let err_path = tools.scratch.join(format!("{name}.{step}.err"));
    let out = File::create(&out_path).map_err(|error| crash(error.to_string()))?;
    let err = if merged {
        out.try_clone()
    } else {
        File::create(&err_path)
    }
    .map_err(|error| crash(error.to_string()))?;
    let mut child = Command::new(program)
        .args(args)
        .current_dir(tools.root)
        .stdin(Stdio::null())
        .stdout(Stdio::from(out))
        .stderr(Stdio::from(err))
        .spawn()
        .map_err(|error| crash(format!("{program}: {error}")))?;
    let started = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if started.elapsed() > tools.budget => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(RunEnd::Timeout {
                    seconds: tools.budget.as_secs_f64(),
                });
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(5)),
            Err(error) => return Err(crash(error.to_string())),
        }
    };
    let read = |path: &Path| {
        fs::read(path)
            .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
            .unwrap_or_default()
    };
    let ran = Ran {
        stdout: read(&out_path),
        stderr: if merged {
            String::new()
        } else {
            read(&err_path)
        },
        success: status.success(),
    };
    let _ = fs::remove_file(&out_path);
    let _ = fs::remove_file(&err_path);
    Ok(ran)
}
