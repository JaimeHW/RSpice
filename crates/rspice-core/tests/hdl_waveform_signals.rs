#![cfg(feature = "veriloga")]
//! HDL signals in a run's waveforms, on the C12 testbench (`full`): the
//! internal variables `.ESAVE` names, published as ordinary traces under
//! their instance paths.
//!
//! The testbench loads codes 0 to 15, one every 100 us, and 99 us into each
//! period reads V(vout) into `vk[k]`, then `mv = vk[k]`; `meas` is driven by
//! `assign meas = mv`, and `pass` is set to 1 at time zero and only cleared
//! by a step outside its window, which this design never takes.

use rspice_core::engine::{HdlSourceLine, HdlTraceSource, TransientResult};
use rspice_core::xspice::DigitalState;
use rspice_core::{Engine, Netlist};

const STOP: f64 = 1.65e-3;
const STEP: f64 = 0.5e-6;

/// The C12 `full` deck with its `.ESAVE` list replaced by `esave`, parsed
/// where the deck is so that its includes resolve.
fn c12_full(esave: &str) -> Netlist {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/mixed-corpus/c12-ldo-tb/deck/full.tran.cir");
    let text = std::fs::read_to_string(&path).expect("the deck reads");
    assert!(text.contains(".esave xtb.ok__event_0\n"), "the deck's list");
    let text = text.replace(".esave xtb.ok__event_0\n", &format!("{esave}\n"));
    Netlist::parse_with_path(&text, &path).unwrap_or_else(|error| panic!("parses: {error}"))
}

fn run(esave: &str) -> TransientResult {
    Engine::default()
        .run_tran(&c12_full(esave), STOP, STEP)
        .unwrap_or_else(|error| panic!("{esave}: {error}"))
}

fn logic(result: &TransientResult, name: &str) -> Vec<(f64, DigitalState)> {
    result
        .digital_trace_named(name)
        .unwrap_or_else(|| panic!("no digital trace {name}"))
        .iter()
        .map(|point| (point.time, point.value.state))
        .collect()
}

fn real(result: &TransientResult, name: &str) -> Vec<(f64, f64)> {
    result
        .real_trace_named(name)
        .unwrap_or_else(|| panic!("no real trace {name}"))
        .iter()
        .map(|point| (point.time, point.value))
        .collect()
}

/// The bit a logic trace holds at `time`: its last point at or before it.
fn level_at(trace: &[(f64, DigitalState)], time: f64) -> DigitalState {
    trace
        .iter()
        .rev()
        .find(|(at, _)| *at <= time)
        .map(|(_, state)| *state)
        .expect("a value by then")
}

/// `.esave xtb.pass xtb.mv` publishes the testbench's verdict register and
/// its real variable as traces named by instance path. `D(xtb.pass)` is 1
/// from time zero on; `E(xtb.mv)` steps to each code's reading 99 us into
/// its period, as `meas`, which `assign meas = mv` drives, does point for
/// point. A vector variable traces as an engine-declared bus over its bits,
/// and `xtrim.sh` holds code k half way through period k.
#[test]
fn esave_publishes_internal_hdl_variables_as_traces() {
    let result = run(".esave xtb.ok__event_0 xtb.pass xtb.mv meas xtrim.sh");

    assert_eq!(logic(&result, "xtb.pass"), [(0.0, DigitalState::One)]);

    let mv = real(&result, "xtb.mv");
    let meas = real(&result, "meas");
    assert_eq!(
        mv.len(),
        17,
        "the value at time zero, then one per code: {mv:?}"
    );
    assert_eq!(mv[0], (0.0, 0.0));
    for (k, (time, _)) in mv.iter().skip(1).enumerate() {
        let expected = k as f64 * 100e-6 + 99e-6;
        assert!(
            (time - expected).abs() <= 1e-15,
            "code {k} is read at {expected:e} s, the trace steps at {time:e} s"
        );
    }
    assert!(
        mv.windows(2).all(|pair| pair[0].1 != pair[1].1),
        "every point is a change: {mv:?}"
    );
    let bits = |trace: &[(f64, f64)]| -> Vec<(u64, u64)> {
        trace
            .iter()
            .map(|(time, value)| (time.to_bits(), value.to_bits()))
            .collect()
    };
    assert_eq!(bits(&mv), bits(&meas), "mv and meas step together, exactly");

    let bus = result
        .digital_buses
        .iter()
        .find(|bus| bus.name == "xtrim.sh")
        .expect("the vector is a bus");
    assert_eq!((bus.msb, bus.lsb), (3, 0));
    assert_eq!(
        bus.members,
        ["xtrim.sh[3]", "xtrim.sh[2]", "xtrim.sh[1]", "xtrim.sh[0]"]
    );
    assert_eq!(bus.source, rspice_core::engine::DigitalBusSource::Engine);
    let members: Vec<_> = bus
        .members
        .iter()
        .map(|member| logic(&result, member))
        .collect();
    for k in 0..16u32 {
        let time = f64::from(k) * 100e-6 + 50e-6;
        let word = members.iter().fold(0, |word, member| {
            word << 1 | u32::from(level_at(member, time) == DigitalState::One)
        });
        assert_eq!(word, k, "xtrim.sh at {time:e} s");
    }
}

/// A name that is neither an event net nor a net or variable of an HDL
/// instance is refused by name, as `.ESAVE` refuses a node that is not an
/// event node.
#[test]
fn esave_refuses_a_name_no_instance_declares() {
    let error = Engine::default()
        .run_tran(&c12_full(".esave xtb.ok__event_0 xtb.pas"), STOP, STEP)
        .expect_err("a misspelt variable is refused")
        .to_string()
        .to_ascii_lowercase();
    assert!(
        error.contains("'xtb.pas' is not an event node or an hdl variable"),
        "{error}"
    );
}

/// A counter whose register and real advance every 20 ns, behind the
/// built-in connect modules, so that every converter in the run can be
/// checkpointed. Its `.ESAVE` list names the two variables.
fn counter_deck(directory: &std::path::Path) -> Netlist {
    use rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES;
    let mut source = String::from(
        "`include \"disciplines.vams\"\n\
         `timescale 1ns/1ps\n\
         module cnt(o);\n  output o; reg o;\n  reg [3:0] q;\n  real r;\n\
         \x20 initial begin o = 1'b0; q = 4'd0; r = 0.0; end\n\
         \x20 always #20 begin q = q + 4'd1; r = r + 0.25; o = q[0]; end\nendmodule\n",
    );
    for (_, module) in BUILTIN_CONNECT_MODULES {
        source.push_str(module);
    }
    source.push_str("connectrules deck;\n    connect a2d;\n    connect d2a;\nendconnectrules\n");
    let path = directory.join("cnt.vams");
    std::fs::write(&path, source).expect("the module writes");
    let path = path.display().to_string().replace('\\', "/");
    Netlist::parse(&format!(
        "* counter\n.hdl \"{path}\" module=cnt\n.options connectrules=deck\n\
         x1 out cnt\nr1 out 0 1k\n.esave x1.q x1.r\n.end\n"
    ))
    .expect("the deck parses")
}

/// A run resumed from a checkpoint taken between two counts continues the
/// variables' traces exactly: its first point is each variable's value at
/// the seam, and every later point is the uninterrupted run's, bit for bit.
#[test]
fn a_resumed_run_continues_the_esave_traces_exactly() {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("the clock follows the Unix epoch")
        .as_nanos();
    let directory =
        std::env::temp_dir().join(format!("rspice-esave-{}-{nonce}", std::process::id()));
    std::fs::create_dir(&directory).expect("a test directory");
    let netlist = counter_deck(&directory);
    let (stop, step, seam) = (400e-9, 1e-9, 130e-9);
    let engine = Engine::default();
    let whole = engine
        .run_tran(&netlist, stop, step)
        .expect("the whole run");
    let (_, checkpoint) = engine
        .run_tran_checkpointed(&netlist, seam, step)
        .expect("the first part checkpoints");
    let (resumed, _) = engine
        .run_tran_resume(&netlist, &checkpoint, stop, step)
        .expect("the rest resumes");
    let _ = std::fs::remove_dir_all(&directory);
    let start = resumed.time[0];
    assert!((start - seam).abs() <= 1e-15, "resumed at {start:e}");

    let whole_r = real(&whole, "x1.r");
    assert!(
        whole_r.len() >= 20,
        "0 and a count every 20 ns: {whole_r:?}"
    );
    assert_eq!(whole_r[6].1, 1.5);
    assert!((whole_r[6].0 - 120e-9).abs() <= 1e-15, "{whole_r:?}");
    let mut expected = vec![(start, 1.5)];
    expected.extend(whole_r.iter().filter(|(time, _)| *time > start));
    let bits = |trace: &[(f64, f64)]| -> Vec<(u64, u64)> {
        trace
            .iter()
            .map(|(time, value)| (time.to_bits(), value.to_bits()))
            .collect()
    };
    assert_eq!(bits(&real(&resumed, "x1.r")), bits(&expected));

    for bit in 0..4 {
        let name = format!("x1.q[{bit}]");
        let whole_q = logic(&whole, &name);
        let mut expected = vec![(start, level_at(&whole_q, start))];
        expected.extend(whole_q.iter().filter(|(time, _)| *time > start));
        assert_eq!(logic(&resumed, &name), expected, "{name}");
    }
    assert_eq!(
        level_at(&logic(&resumed, "x1.q[1]"), 150e-9),
        DigitalState::One
    );
}

/// The entry a run's source map has for `trace`.
fn source<'a>(result: &'a TransientResult, trace: &str) -> &'a HdlTraceSource {
    result
        .hdl_sources
        .iter()
        .find(|source| source.trace.eq_ignore_ascii_case(trace))
        .unwrap_or_else(|| panic!("no source for {trace}: {:#?}", result.hdl_sources))
}

/// A source line as `file:line`, the file by its name alone.
fn place(line: &Option<HdlSourceLine>) -> String {
    let line = line.as_ref().expect("a placed line");
    let file = std::path::Path::new(&line.file)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(&line.file);
    format!("{file}:{}", line.line)
}

/// Each driver as `kind file:line`.
fn drivers(source: &HdlTraceSource) -> Vec<String> {
    source
        .drivers
        .iter()
        .map(|driver| format!("{:?} {}", driver.kind, place(&driver.source)))
        .collect()
}

/// The run's source map on the C12 testbench. `xtb.ok` is declared on line
/// 12 of the testbench and driven only by its `initial` block at line 32,
/// whether it is traced as the port's event net or as a variable;
/// `xtrim.tr3` is driven by the `initial` block on line 9 of `spi_trim.v`
/// and by the `always @(posedge csn)` on line 11. A replay of the cached
/// artifacts gives the same map.
#[test]
fn the_source_map_places_declarations_and_drivers() {
    let esave = ".esave xtb.ok__event_0 xtb.ok xtrim.tr3";
    let result = run(esave);
    assert_eq!(result.hdl_sources.len(), 3, "{:#?}", result.hdl_sources);
    for trace in ["xtb.ok__event_0", "xtb.ok"] {
        let ok = source(&result, trace);
        assert!(ok.instance.eq_ignore_ascii_case("xtb"), "{ok:?}");
        assert_eq!(ok.identifier, "ok");
        assert_eq!(place(&ok.declaration), "tb_ldo_full.vams:12");
        assert_eq!(drivers(ok), ["Initial tb_ldo_full.vams:32"]);
    }
    let tr3 = source(&result, "xtrim.tr3");
    assert_eq!(tr3.identifier, "tr3");
    assert_eq!(place(&tr3.declaration), "spi_trim.v:7");
    assert_eq!(
        drivers(tr3),
        ["Initial spi_trim.v:9", "Always spi_trim.v:11"]
    );

    // The map is read from the compiled designs, so it is the same whether a
    // run compiled them or replayed the compile cache's records. With no
    // room in memory a run compiles them afresh and empties the in-memory
    // cache, so the run after it reads back the records the first run left.
    let mut config = rspice_core::SimulationConfig::default();
    config.resource_limits.max_shared_cache_bytes = 1;
    let before = rspice_core::veriloga_cache_telemetry();
    let compiled = Engine::new(config)
        .run_tran(&c12_full(esave), STOP, STEP)
        .expect("the compiled run");
    let after = rspice_core::veriloga_cache_telemetry();
    assert!(
        after.compilations_succeeded > before.compilations_succeeded,
        "{before:?} {after:?}"
    );
    assert_eq!(compiled.hdl_sources, result.hdl_sources);
    let before = after;
    let replayed = run(esave);
    let after = rspice_core::veriloga_cache_telemetry();
    assert!(after.disk_hits > before.disk_hits, "{before:?} {after:?}");
    assert_eq!(replayed.hdl_sources, result.hdl_sources);
}
