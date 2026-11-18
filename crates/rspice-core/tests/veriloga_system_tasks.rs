//! Verilog-AMS system tasks and functions in the analog context (LRM 2.4
//! sections 9.4 to 9.13), run by the engine.

#![cfg(feature = "veriloga")]

#[path = "common/model_outcome.rs"]
mod model_outcome;

use model_outcome::RunWithOutcome;
use rspice_core::config::{HdlOutputCollector, HdlOutputRecord, HdlStream};
use rspice_core::{Engine, Netlist, NoAbort, SimulationOutcome};
use std::path::PathBuf;
use std::sync::Arc;

/// A scratch directory of its own for each test.
fn scratch(name: &str) -> PathBuf {
    let directory = std::env::temp_dir().join(format!(
        "rspice_va_system_tasks_{name}_{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).unwrap();
    directory
}

/// Run a transient of `body` in a one-port module across a ramp source, in
/// `directory`, and return the outcome and every HDL record.
fn run(
    directory: &std::path::Path,
    body: &str,
    stop: f64,
) -> (
    Result<SimulationOutcome<rspice_core::engine::TransientResult>, String>,
    Vec<HdlOutputRecord>,
) {
    run_with(directory, body, stop, |_| {})
}

/// [`run`], with the run's HDL options set by `hdl`.
fn run_with(
    directory: &std::path::Path,
    body: &str,
    stop: f64,
    hdl: impl FnOnce(&mut rspice_core::config::HdlRunConfig),
) -> (
    Result<SimulationOutcome<rspice_core::engine::TransientResult>, String>,
    Vec<HdlOutputRecord>,
) {
    let model = directory.join("tasks.va");
    std::fs::write(
        &model,
        format!(
            "`include \"disciplines.vams\"\nmodule tasks(x); inout x; electrical x;\n\
             integer fd, rc, k, seed, n, a, b, c, d, e, f, g, h, i, j; real r;\nanalog begin\n{body}\nI(x) <+ V(x) / 1k;\nend\nendmodule\n"
        ),
    )
    .unwrap();
    let deck = directory.join("deck.cir");
    std::fs::write(
        &deck,
        format!(
            "* system tasks\nvx s 0 pwl(0 0 {stop} 1)\nr1 s x 1k\nxa x tasks\n.hdl \"tasks.va\"\n.end\n"
        ),
    )
    .unwrap();
    let netlist = Netlist::parse_file(&deck).unwrap();
    let collector = Arc::new(HdlOutputCollector::new());
    let mut config = rspice_core::SimulationConfig::default();
    config.hdl.output = Some(collector.clone());
    config.hdl.deck_directory = Some(directory.to_path_buf());
    hdl(&mut config.hdl);
    let outcome = Engine::new(config)
        .run_with_outcome(&NoAbort, |engine, signal| {
            engine.run_tran_with_abort(&netlist, stop, stop / 100.0, signal)
        })
        .map_err(|error| error.to_string());
    (outcome, collector.take())
}

fn stdout(records: &[HdlOutputRecord]) -> Vec<String> {
    records
        .iter()
        .filter(|record| record.stream == HdlStream::Stdout)
        .map(|record| record.text.clone())
        .collect()
}

/// `$debug` prints, `$fdisplay` to the standard output prints there, and
/// `$fwrite` to the standard error lands there.
#[test]
fn debug_and_standard_stream_file_output_print_from_accepted_points() {
    let directory = scratch("streams");
    let (outcome, records) = run(
        &directory,
        "@(initial_step) begin $debug(\"debug\"); $fdisplay(32'h8000_0001, \"out %0d\", 7); \
         $fwrite(32'h8000_0002, \"err\"); end",
        1e-9,
    );
    outcome.unwrap();
    let printed = stdout(&records);
    assert!(printed.iter().any(|text| text == "debug\n"), "{printed:?}");
    assert!(printed.iter().any(|text| text == "out 7\n"), "{printed:?}");
    assert!(
        records
            .iter()
            .any(|record| record.stream == HdlStream::Stderr && record.text == "err"),
        "{records:?}"
    );
}

/// `$fatal` ends the run at the accepted point with its formatted message
/// and source line; `$stop` ends it as a stop (LRM 2.4 section 9.7).
#[test]
fn fatal_and_stop_end_the_run_with_their_source_line() {
    use rspice_core::ModelFinishTask;
    let directory = scratch("control");
    for (body, task, message) in [
        (
            "@(cross(V(x) - 0.25, +1)) $fatal(1, \"crossed at %g\", 0.25);",
            ModelFinishTask::Fatal,
            Some("crossed at 0.25"),
        ),
        (
            "@(cross(V(x) - 0.25, +1)) $stop;",
            ModelFinishTask::Stop,
            None,
        ),
    ] {
        let (outcome, records) = run(&directory, body, 10e-9);
        let SimulationOutcome::Finished { finish, .. } = outcome.unwrap() else {
            panic!("{body}: the run must end at the call");
        };
        assert_eq!(finish.task, task, "{body}");
        assert_eq!(finish.message.as_deref(), message, "{body}");
        let source = finish.source.expect("the call's source");
        assert!(source.file.ends_with("tasks.va"), "{source:?}");
        assert_eq!(source.line, 5);
        assert!(
            records
                .iter()
                .any(|record| matches!(record.stream, HdlStream::Finish { .. })),
            "{records:?}"
        );
    }
}

/// `$random` and `$dist_*` in analog code draw IEEE 1364-2005 section
/// 17.9.3's sequence: the same values Icarus Verilog 13.0 prints for
///
/// ```verilog
/// seed = 42; a = $random(seed); ... j = $dist_erlang(seed, 2, 5);
/// s = 3; x = $random(s);
/// ```
///
/// and `$arandom("global")`, the design's stream, starts from the run's
/// `hdlseed`, as the digital kernel's `$random` does.
#[test]
fn random_and_integer_distributions_draw_what_icarus_draws() {
    let directory = scratch("random");
    let (outcome, records) = run_with(
        &directory,
        "@(initial_step) begin seed = 42;\n\
         a = $random(seed); b = $random(seed); c = $random(seed);\n\
         d = $dist_uniform(seed, -10, 10); e = $dist_normal(seed, 5, 3);\n\
         f = $dist_exponential(seed, 4); g = $dist_poisson(seed, 6);\n\
         h = $dist_chi_square(seed, 3); i = $dist_t(seed, 4); j = $dist_erlang(seed, 2, 5);\n\
         $display(\"%0d %0d %0d %0d %0d %0d %0d %0d %0d %0d seed=%0d\", a, b, c, d, e, f, g, h, i, j, seed);\n\
         n = $arandom(\"global\"); $display(\"hdlseed %0d\", n); end",
        1e-9,
        |hdl| hdl.seed = 3,
    );
    outcome.unwrap();
    let printed = stdout(&records);
    assert_eq!(
        printed,
        [
            "-2144582656 646214477 38602500 -5 7 1 4 7 -3 6 seed=2039465103\n",
            "hdlseed -2147276288\n",
        ],
        "{records:?}"
    );
}

/// `$fopen`, `$fscanf`, `$test$plusargs` and `$value$plusargs` read the run's
/// files and plusargs, `$fdisplay` and `$fstrobe` write through the same
/// table, and `$fstrobe` writes once per accepted point. A path outside the
/// run directory opens as 0 with a warning, as the digital kernel's does.
#[test]
fn files_and_plusargs_are_the_runs() {
    let directory = scratch("files");
    std::fs::write(directory.join("in.txt"), "3 2.5\n").unwrap();
    let (outcome, records) = run_with(
        &directory,
        "@(initial_step) begin fd = $fopen(\"out.txt\", \"w\");\n\
         k = $fopen(\"in.txt\", \"r\"); rc = $fscanf(k, \"%d %g\", n, r);\n\
         a = $test$plusargs(\"fast\"); b = $value$plusargs(\"gain=%d\", c);\n\
         e = $fopen(\"../escape.txt\", \"w\");\n\
         $fdisplay(fd, \"n=%0d r=%g rc=%0d fast=%0d gain=%0d/%0d escape=%0d\", n, r, rc, a, b, c, e);\n\
         $fclose(k); end\n\
         $fstrobe(fd, \"t\");",
        10e-9,
        |hdl| hdl.plusargs = vec!["+gain=7".into(), "+fast".into()],
    );
    let SimulationOutcome::Completed(result) = outcome.unwrap() else {
        panic!("the run completes");
    };
    let written = std::fs::read_to_string(directory.join("out.txt")).unwrap();
    // `$fdisplay` writes once, at the accepted first point; `$fstrobe` once
    // per accepted point.
    let (displayed, strobed): (Vec<&str>, Vec<&str>) =
        written.lines().partition(|line| *line != "t");
    assert_eq!(
        displayed,
        ["n=3 r=2.5 rc=2 fast=1 gain=1/7 escape=0"],
        "{written}"
    );
    assert!(written.starts_with(displayed[0]), "{written}");
    assert_eq!(strobed.len(), result.time.len(), "{written}");
    assert!(
        records.iter().any(
            |record| matches!(record.stream, HdlStream::Diagnostic { .. })
                && record.text.contains("$fopen(\"../escape.txt\")")
        ),
        "{records:?}"
    );
    assert!(!directory.join("../escape.txt").exists());
}

/// `$fdisplay` writes as `$display` does, once per accepted point: an
/// `@(initial_step)` call writes one line though the first point takes
/// several Newton evaluations, which the `$debug` beside it counts. The
/// module reads its file back and drives the number of lines it holds, which
/// the browser worker's twin of this test reads.
#[test]
fn an_initial_step_file_display_writes_once_however_many_evaluations_the_point_takes() {
    let directory = scratch("once");
    std::fs::write(directory.join("once.va"), ONCE_PER_POINT).unwrap();
    let deck = directory.join("deck.cir");
    std::fs::write(
        &deck,
        "* one write per accepted point\n.hdl \"once.va\"\nxa q once\nr1 q 0 1k\n.end\n",
    )
    .unwrap();
    let netlist = Netlist::parse_file(&deck).unwrap();
    let collector = Arc::new(HdlOutputCollector::new());
    let mut config = rspice_core::SimulationConfig::default();
    config.hdl.output = Some(collector.clone());
    config.hdl.deck_directory = Some(directory.clone());
    let outcome = Engine::new(config)
        .run_with_outcome(&NoAbort, |engine, signal| {
            engine.run_tran_with_abort(&netlist, 1e-9, 1e-11, signal)
        })
        .unwrap();
    let SimulationOutcome::Completed(result) = outcome else {
        panic!("the run completes");
    };
    let records = collector.take();
    let evaluations = stdout(&records)
        .iter()
        .filter(|text| text.starts_with("evaluation"))
        .count();
    assert!(
        evaluations >= 2,
        "the first point takes several evaluations: {records:?}"
    );
    let written = std::fs::read_to_string(directory.join("once.txt")).unwrap();
    assert_eq!(written, "7\n");
    let q = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("q"))
        .unwrap();
    let lines = *result.voltages[q].last().unwrap();
    assert!((lines - 1.0).abs() < 1e-9, "V(q) = {lines}");
}

/// A current source into a diode across the deck's 1k resistor, a first
/// point Newton solves in several evaluations, until the module reads back
/// how many lines its `@(initial_step)` wrote and drives that count.
const ONCE_PER_POINT: &str = "`include \"disciplines.vams\"\nmodule once(q); inout q; electrical q;\n\
     integer fd, k, n, a, b, c;\nanalog begin\n\
     @(initial_step) begin fd = $fopen(\"once.txt\", \"w\"); $fdisplay(fd, \"%0d\", 7);\n\
     $debug(\"evaluation at %g\", $abstime); $fclose(fd); end\n\
     @(timer(0.5n)) begin k = $fopen(\"once.txt\", \"r\"); n = $fscanf(k, \"%d %d %d\", a, b, c);\n\
     $fclose(k); end\n\
     if (n > 0) V(q) <+ n;\n\
     else I(q) <+ 1e-14 * (limexp(V(q) / 0.02585) - 1) - 5m;\nend\nendmodule\n";

/// `$debug` and `$fdebug` write at every evaluation, a rejected point's
/// included (LRM 2.4 sections 9.4.6 and 9.5.9): the step that overshoots a
/// `cross` event is rejected for one that lands on it, and its evaluations
/// print all the same, in the order they ran.
#[test]
fn debug_writes_at_every_evaluation_a_rejected_points_included() {
    let directory = scratch("debug");
    let (outcome, records) = run(
        &directory,
        "@(initial_step) fd = $fopen(\"debug.txt\", \"w\");\n\
         @(cross(V(x) - 0.25, +1)) a = 1;\n\
         $debug(\"debug %.17g\", $abstime); $fdebug(fd, \"%.17g\", $abstime);",
        10e-9,
    );
    let SimulationOutcome::Completed(result) = outcome.unwrap() else {
        panic!("the run completes");
    };
    let printed: Vec<f64> = stdout(&records)
        .iter()
        .filter_map(|text| text.strip_prefix("debug "))
        .map(|time| time.trim().parse().unwrap())
        .collect();
    for time in &result.time {
        assert!(
            printed.contains(time),
            "accepted {time} printed: {printed:?}"
        );
    }
    let rejected: Vec<f64> = printed
        .iter()
        .copied()
        .filter(|time| !result.time.contains(time))
        .collect();
    assert!(
        !rejected.is_empty(),
        "a rejected point prints: {printed:?} accepted {:?}",
        result.time
    );
    let written: Vec<f64> = std::fs::read_to_string(directory.join("debug.txt"))
        .unwrap()
        .lines()
        .map(|time| time.parse().unwrap())
        .collect();
    assert_eq!(written, printed);
}

/// A module that draws at a timer every nanosecond and prints what it drew:
/// a seedless `$random`, the design's stream, and a constant seed scoped to
/// the design and to the instance.
const DRAWS: &str = "`include \"disciplines.vams\"\nmodule draws(x); inout x; electrical x;\n\
     integer r, g; real dg, di;\nanalog begin\n\
     @(timer(0, 1n)) begin r = $random; g = $arandom(\"global\");\n\
     dg = $rdist_normal(7, 0, 1, \"global\"); di = $rdist_normal(7, 0, 1, \"instance\");\n\
     $display(\"%m %0d %0d %.17g %.17g\", r, g, dg, di); end\n\
     I(x) <+ V(x) / 1k;\nend\nendmodule\n";

/// What each instance of [`DRAWS`] printed, by instance, in a transient of
/// `instances` (deck lines) to `stop`; with `checkpoint`, the run stops there
/// and a second run resumes from its checkpoint.
fn draws_by_instance(
    name: &str,
    instances: &str,
    stop: f64,
    checkpoint: Option<f64>,
) -> std::collections::BTreeMap<String, Vec<String>> {
    let directory = scratch(name);
    std::fs::write(directory.join("draws.va"), DRAWS).unwrap();
    let deck = directory.join("deck.cir");
    std::fs::write(
        &deck,
        format!("* draws\n.hdl \"draws.va\"\nv1 s 0 1\n{instances}.end\n"),
    )
    .unwrap();
    let netlist = Netlist::parse_file(&deck).unwrap();
    let collector = Arc::new(HdlOutputCollector::new());
    let mut config = rspice_core::SimulationConfig::default();
    config.hdl.output = Some(collector.clone());
    config.hdl.deck_directory = Some(directory.clone());
    config.hdl.seed = 11;
    let engine = Engine::new(config);
    let step = 1e-10;
    match checkpoint {
        None => {
            engine
                .run_tran_with_abort(&netlist, stop, step, &NoAbort)
                .unwrap();
        }
        Some(at) => {
            let (_, saved) = engine
                .run_tran_checkpointed_with_abort(&netlist, at, step, &NoAbort)
                .unwrap();
            let saved = saved.into_checkpoint().unwrap();
            let saved =
                rspice_core::engine::TransientCheckpoint::from_text(&saved.to_text()).unwrap();
            engine
                .run_tran_resume_with_abort(&netlist, &saved, stop, step, &NoAbort)
                .unwrap();
        }
    }
    let mut draws = std::collections::BTreeMap::<String, Vec<String>>::new();
    for text in stdout(&collector.take()) {
        let (instance, values) = text.trim_end().split_once(' ').unwrap();
        draws
            .entry(instance.to_ascii_lowercase())
            .or_default()
            .push(values.to_owned());
    }
    draws
}

/// A `$random` without a seed draws from a stream of the instance's own,
/// fixed by the run's seed and the instance's name: two instances of one
/// module draw different values, a rerun draws the same, and listing the
/// instances in another order changes nothing. A `"global"` type string
/// draws from the design's stream, the same in every instance, and scopes a
/// constant seed to the design; `"instance"` scopes it to the instance.
#[test]
fn seedless_draws_are_per_instance_and_global_draws_are_the_designs() {
    let forward = "xa s draws\nxb s draws\n";
    let reversed = "xb s draws\nxa s draws\n";
    let draws = draws_by_instance("order", forward, 5e-9, None);
    assert_eq!(draws_by_instance("rerun", forward, 5e-9, None), draws);
    assert_eq!(draws_by_instance("reversed", reversed, 5e-9, None), draws);
    let (a, b) = (&draws["xa"], &draws["xb"]);
    assert!(a.len() >= 5, "{draws:?}");
    assert_eq!(a.len(), b.len(), "{draws:?}");
    for (a, b) in a.iter().zip(b) {
        let a: Vec<&str> = a.split(' ').collect();
        let b: Vec<&str> = b.split(' ').collect();
        assert_ne!(a[0], b[0], "seedless draws differ between instances");
        assert_eq!(a[1], b[1], "\"global\" draws are the design's");
        assert_eq!(a[2], b[2], "a \"global\" seed is the design's");
        assert_ne!(a[3], b[3], "an \"instance\" seed is the instance's");
    }
}

/// A run resumed from a checkpoint draws exactly what the run it continues
/// would have: the streams and the internal seeds are in the checkpoint.
#[test]
fn a_resumed_run_draws_what_the_uninterrupted_run_draws() {
    let instances = "xa s draws\nxb s draws\n";
    let whole = draws_by_instance("whole", instances, 6e-9, None);
    let resumed = draws_by_instance("resumed", instances, 6e-9, Some(2.5e-9));
    assert_eq!(resumed, whole);
}

/// A module that opens files and reads one through names and formats held in
/// string parameters and string variables.
const STRING_NAMES: &str = "`include \"disciplines.vams\"\nmodule names(x); inout x; electrical x;\n\
     parameter string out_name = \"out.txt\"; parameter string in_name = \"in.txt\";\n\
     parameter string fmt = \"%d %d %d\"; parameter string other = \"other.txt\";\n\
     parameter integer pick = 1;\n\
     string s, f; integer fd, k, n, a, b, c;\nanalog begin\n@(initial_step) begin\n\
     fd = $fopen(out_name, \"w\"); $fdisplay(fd, \"parameter\"); $fclose(fd);\n\
     s = pick ? \"variable.txt\" : other;\n\
     fd = $fopen(s, \"w\"); $fdisplay(fd, \"variable\"); $fclose(fd);\n\
     f = fmt; k = $fopen(in_name, \"r\"); n = $fscanf(k, f, a, b, c); $fclose(k);\n\
     $display(\"n=%0d a=%0d b=%0d c=%0d\", n, a, b, c);\n\
     k = $fopen(in_name, \"r\"); n = $fscanf(k, fmt, a, b); $fclose(k);\n\
     $display(\"n=%0d a=%0d b=%0d\", n, a, b);\nend\n\
     I(x) <+ V(x) / 1k;\nend\nendmodule\n";

/// LRM 2.4 section 9.5.1: `$fopen`'s file name "is an expression that is a
/// string literal, string data type, or an integral data type containing a
/// character string". A name or a `$fscanf` format comes from a string
/// parameter, fixed when the module is elaborated, or from a string
/// variable, read when the call runs; an instance's override of the
/// parameter names its own file.
#[test]
fn file_names_and_formats_come_from_string_parameters_and_variables() {
    let directory = scratch("names");
    std::fs::write(directory.join("names.va"), STRING_NAMES).unwrap();
    std::fs::write(directory.join("in.txt"), "1 2 3\n").unwrap();
    let deck = directory.join("deck.cir");
    std::fs::write(
        &deck,
        "* string names\n.hdl \"names.va\"\nv1 x 0 1\nxa x names\n\
         xb x names out_name=\"override.txt\" other=\"chosen.txt\" pick=0\n.end\n",
    )
    .unwrap();
    let netlist = Netlist::parse_file(&deck).unwrap();
    let collector = Arc::new(HdlOutputCollector::new());
    let mut config = rspice_core::SimulationConfig::default();
    config.hdl.output = Some(collector.clone());
    config.hdl.deck_directory = Some(directory.clone());
    Engine::new(config).run_dc_op(&netlist).unwrap();
    let read = |file: &str| std::fs::read_to_string(directory.join(file)).unwrap();
    assert_eq!(read("out.txt"), "parameter\n");
    assert_eq!(read("variable.txt"), "variable\n");
    // `xb` names its overrides: one through the parameter, the other
    // through the variable the condition assigned it to.
    assert_eq!(read("override.txt"), "parameter\n");
    assert_eq!(read("chosen.txt"), "variable\n");
    assert_eq!(
        stdout(&collector.take()),
        [
            "n=3 a=1 b=2 c=3\n",
            "n=2 a=1 b=2\n",
            "n=3 a=1 b=2 c=3\n",
            "n=2 a=1 b=2\n"
        ]
    );
}

/// A module whose first point of every analysis opens a file descriptor and
/// a multichannel descriptor in write mode and writes a line to each.
const EVERY_ANALYSIS: &str = "`include \"disciplines.vams\"\nmodule log(x); inout x; electrical x;\n\
     integer fd, mcd;\nanalog begin\n\
     @(initial_step) begin fd = $fopen(\"log.txt\", \"w\"); mcd = $fopen(\"channel.txt\");\n\
     $fdisplay(fd, \"analysis\"); $fdisplay(mcd, \"analysis\"); end\n\
     I(x) <+ V(x) / 1k;\nend\nendmodule\n";

/// LRM 2.4 section 9.5.1.1: "If a file is opened in a write mode in the
/// first analysis and reopened in that write mode in following analysis,
/// then content written from the following analyses shall be appended to the
/// content written during the previous analyses." An `.op` and a `.tran`
/// on one engine write both their lines to each file; another engine is
/// another run, whose first analysis empties them.
#[test]
fn a_later_analysis_of_a_run_appends_to_what_an_earlier_one_wrote() {
    let directory = scratch("analyses");
    std::fs::write(directory.join("log.va"), EVERY_ANALYSIS).unwrap();
    let deck = directory.join("deck.cir");
    std::fs::write(
        &deck,
        "* every analysis writes\n.hdl \"log.va\"\nv1 s 0 1\nxa s log\n.end\n",
    )
    .unwrap();
    let netlist = Netlist::parse_file(&deck).unwrap();
    let mut config = rspice_core::SimulationConfig::default();
    config.hdl.deck_directory = Some(directory.clone());
    let read = |file: &str| std::fs::read_to_string(directory.join(file)).unwrap();
    let engine = Engine::new(config.clone());
    engine.run_dc_op(&netlist).unwrap();
    engine
        .run_tran_with_abort(&netlist, 1e-9, 1e-10, &NoAbort)
        .unwrap();
    assert_eq!(read("log.txt"), "analysis\nanalysis\n");
    assert_eq!(read("channel.txt"), "analysis\nanalysis\n");
    Engine::new(config).run_dc_op(&netlist).unwrap();
    assert_eq!(read("log.txt"), "analysis\n");
    assert_eq!(read("channel.txt"), "analysis\n");
}

/// A design whose digital half opens a file descriptor and a multichannel
/// descriptor, and whose analog half opens a file descriptor; each half
/// writes through every one of them.
const SHARED_DESCRIPTORS: &str = "`include \"disciplines.vams\"\n`timescale 1ns/1ns\n\
     module share(p, n); inout p, n; electrical p, n;\n\
     reg clk; integer count, fd, mcd, afd;\n\
     initial begin clk = 0; count = 0; fd = $fopen(\"digital_fd.txt\", \"w\");\n\
     mcd = $fopen(\"digital_mcd.txt\"); end\n\
     always #5 clk = ~clk;\n\
     always @(posedge clk) begin count = count + 1;\n\
     $fdisplay(fd, \"digital %0d\", count); $fdisplay(mcd, \"digital %0d\", count);\n\
     if (count > 1) $fdisplay(afd, \"digital %0d\", count); end\n\
     analog begin\n@(initial_step) afd = $fopen(\"analog_fd.txt\", \"w\");\n\
     @(timer(7n, 10n)) begin $fdisplay(fd, \"analog %g\", $abstime * 1e9);\n\
     $fdisplay(mcd, \"analog %g\", $abstime * 1e9); $fdisplay(afd, \"analog %g\", $abstime * 1e9); end\n\
     I(p, n) <+ V(p, n) / 1000000.0;\nend\nendmodule\n";

/// LRM 2.4 section 9.5.1.2: "The file I/O system functions and tasks in both
/// the analog and digital contexts can use file descriptors opened in either
/// context, if the file descriptors are opened for writing or appending."
/// Each file holds both halves' lines in the order the run wrote them.
#[test]
fn the_analog_and_digital_halves_write_through_each_others_descriptors() {
    let directory = scratch("shared");
    std::fs::write(directory.join("share.va"), SHARED_DESCRIPTORS).unwrap();
    let deck = directory.join("deck.cir");
    std::fs::write(
        &deck,
        "* shared descriptors\nx1 p 0 share\nrp p 0 1meg\n.va \"share.va\" share\n.end\n",
    )
    .unwrap();
    let netlist = Netlist::parse_file(&deck).unwrap();
    let collector = Arc::new(HdlOutputCollector::new());
    let mut config = rspice_core::SimulationConfig::default();
    config.hdl.output = Some(collector.clone());
    config.hdl.deck_directory = Some(directory.clone());
    Engine::new(config)
        .run_tran_with_abort(&netlist, 40e-9, 1e-9, &NoAbort)
        .unwrap();
    let records = collector.take();
    assert!(
        records
            .iter()
            .all(|record| !matches!(record.stream, HdlStream::Diagnostic { .. })),
        "{records:?}"
    );
    let read = |file: &str| std::fs::read_to_string(directory.join(file)).unwrap();
    let both =
        "digital 1\nanalog 7\ndigital 2\nanalog 17\ndigital 3\nanalog 27\ndigital 4\nanalog 37\n";
    assert_eq!(read("digital_fd.txt"), both);
    assert_eq!(read("digital_mcd.txt"), both);
    assert_eq!(
        read("analog_fd.txt"),
        both.strip_prefix("digital 1\n").unwrap()
    );
}
