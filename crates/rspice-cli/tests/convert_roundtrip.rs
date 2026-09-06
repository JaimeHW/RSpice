//! Integration tests for `rspice convert`: every supported input format must
//! be readable, conversions must preserve values, and `--variables` /
//! `--start` / `--stop` must subset the data.

mod common;

use common::test_dir;

use std::path::{Path, PathBuf};
use std::process::Command;

const TRAN_DECK: &str = "* transient convert test
v1 in 0 sin(0 1 1k)
r1 in out 1k
c1 out 0 1u
.tran 10u 200u
.end
";

const AC_DECK: &str = "* ac convert test
v1 in 0 dc 0 ac 1
r1 in out 1k
c1 out 0 1n
.ac dec 2 100k 1meg
.end
";

/// A digital bridge and a real event node, so a conversion has both kinds of
/// event timeline to carry.
const XSPICE_EVENT_DECK: &str = "* xspice event convert test
v1 in 0 pulse(0 5 0 1n 1n 5n 10n)
abridge1 [in] [d] adc
adac [d] [out] dac
aobs out rnode obs
rout out 0 1k
.model adc adc_bridge(in_low=1 in_high=4)
.model dac dac_bridge(out_low=0 out_high=5 out_undef=2.5)
.model obs v_to_real(gain=2)
.tran 1n 20n
.end
";

/// A mixed Verilog-AMS module with a two-bit discrete output.
///
/// This is the one shape in the tree that makes a *run* declare a bus: the
/// boundary binds one deck node per bit and publishes the range its author
/// wrote, which nothing else on the command line's inputs can say.
const VECTOR_PORT_MODEL: &str = r#"
`include "disciplines.vams"
module vector_mixed(p, n, count);
    inout p, n;
    electrical p, n;
    output [1:0] count;
    reg [1:0] count;
    initial count = 2'b00;
    always #5 count = count + 2'b01;
    analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#;

/// The deck that instantiates it, with the model written beside it.
///
/// A vector boundary port is one deck node per bit, declared MSB first, so
/// `x1.count [1:0]` is carried by `COUNT#1` and `COUNT#0`. Fifteen
/// nanoseconds is four counter states: `00 01 10 11`, one every `#5`.
fn bus_deck(dir: &Path) -> String {
    let model = dir.join("vector_mixed.va");
    std::fs::write(&model, VECTOR_PORT_MODEL).expect("write the model");
    format!(
        "* a two-bit discrete boundary, one deck node per bit\n\
         x1 p 0 count#1 count#0 vector_mixed\n\
         rp p 0 1meg\n\
         .va \"{}\" vector_mixed\n\
         .tran 1n 15n\n\
         .end\n",
        model.display().to_string().replace('\\', "/")
    )
}

fn rspice(args: &[&str]) {
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--quiet")
        .args(args)
        .output()
        .expect("run rspice");
    assert!(
        output.status.success(),
        "rspice {:?} failed: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
}

fn simulate(dir: &Path, deck: &str, format: &str, out_name: &str) -> PathBuf {
    let deck_path = dir.join("deck.sp");
    std::fs::write(&deck_path, deck).expect("write deck");
    let out = dir.join(out_name);
    rspice(&[
        "run",
        deck_path.to_str().unwrap(),
        "-o",
        out.to_str().unwrap(),
        "-f",
        format,
    ]);
    out
}

fn convert(input: &Path, output: &Path, to: &str, extra: &[&str]) {
    let mut args = vec![
        "convert",
        input.to_str().unwrap(),
        output.to_str().unwrap(),
        "--to",
        to,
    ];
    args.extend_from_slice(extra);
    rspice(&args);
}

/// Parse a CSV file into (header, rows).
fn read_csv(path: &Path) -> (Vec<String>, Vec<Vec<f64>>) {
    let text = std::fs::read_to_string(path).expect("read csv");
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let header: Vec<String> = lines
        .next()
        .expect("csv header")
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let rows = lines
        .map(|line| {
            line.split(',')
                .map(|field| field.trim().parse::<f64>().expect("numeric field"))
                .collect()
        })
        .collect();
    (header, rows)
}

#[test]
fn binary_raw_to_csv_preserves_values() {
    let dir = test_dir("raw_to_csv");
    let raw = simulate(&dir, TRAN_DECK, "raw", "tran.raw");

    let csv = dir.join("tran.csv");
    convert(&raw, &csv, "csv", &[]);

    let (header, rows) = read_csv(&csv);
    assert_eq!(header[0], "time");
    assert!(
        header.iter().any(|h| h.eq_ignore_ascii_case("V(OUT)")),
        "V(OUT) column missing: {header:?}"
    );
    assert!(rows.len() > 10, "too few data rows: {}", rows.len());

    // Time axis must be increasing and end near 200us
    let last = rows.last().unwrap()[0];
    assert!(
        (last - 200e-6).abs() < 20e-6,
        "unexpected final time {last}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn full_format_cycle_preserves_values() {
    let dir = test_dir("cycle");
    let raw = simulate(&dir, TRAN_DECK, "raw", "tran.raw");

    // raw -> json -> hdf5 -> csv, then compare against direct raw -> csv
    let json = dir.join("tran.json");
    let h5 = dir.join("tran.h5");
    let cycled = dir.join("cycled.csv");
    let direct = dir.join("direct.csv");
    convert(&raw, &json, "json", &[]);
    convert(&json, &h5, "hdf5", &[]);
    convert(&h5, &cycled, "csv", &[]);
    convert(&raw, &direct, "csv", &[]);

    let (header_a, rows_a) = read_csv(&direct);
    let (header_b, rows_b) = read_csv(&cycled);
    assert_eq!(header_a, header_b, "headers diverged through the cycle");
    assert_eq!(rows_a.len(), rows_b.len(), "row count diverged");
    for (ra, rb) in rows_a.iter().zip(&rows_b) {
        for (a, b) in ra.iter().zip(rb) {
            assert!(
                (a - b).abs() <= 1e-12 * a.abs().max(1.0),
                "value diverged through cycle: {a} vs {b}"
            );
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn complex_ac_survives_csv_and_raw_round_trip() {
    let dir = test_dir("ac_complex");
    let csv = simulate(&dir, AC_DECK, "csv", "ac.csv");

    let (header, _) = read_csv(&csv);
    assert!(
        header.iter().any(|h| h.starts_with("Re(")) && header.iter().any(|h| h.starts_with("Im(")),
        "AC csv must carry Re/Im columns: {header:?}"
    );

    // csv -> raw (complex) -> csv must keep the Re/Im pairing
    let raw = dir.join("ac.raw");
    let back = dir.join("ac_back.csv");
    convert(&csv, &raw, "raw", &[]);
    let raw_text = std::fs::read(&raw).expect("read raw");
    let head = String::from_utf8_lossy(&raw_text[..raw_text.len().min(400)]).to_string();
    assert!(
        head.contains("Flags: complex"),
        "complex flag lost in rawfile header: {head}"
    );

    convert(&raw, &back, "csv", &[]);
    let (header_back, rows) = read_csv(&back);
    assert_eq!(header, header_back, "Re/Im columns diverged");
    assert!(!rows.is_empty());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn csv_reader_accepts_quoted_comma_signal_names() {
    let dir = test_dir("quoted_signal_name");
    let input = dir.join("diff.csv");
    let output = dir.join("diff.json");
    std::fs::write(&input, "time,\"V(in,out)\"\n0,2.5\n1e-9,2.5\n").expect("write quoted csv");

    convert(&input, &output, "json", &[]);

    let json = std::fs::read_to_string(&output).expect("json output");
    assert!(
        json.contains("\"name\": \"V(in,out)\""),
        "quoted comma-bearing signal name should survive conversion: {json}"
    );
    assert!(
        json.contains("2.5"),
        "signal values should survive conversion: {json}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn variable_and_range_filters_apply() {
    let dir = test_dir("filters");
    let raw = simulate(&dir, TRAN_DECK, "raw", "tran.raw");

    let filtered = dir.join("filtered.csv");
    convert(
        &raw,
        &filtered,
        "csv",
        &[
            "--variables",
            "V(OUT)",
            "--start",
            "50e-6",
            "--stop",
            "150e-6",
        ],
    );

    let (header, rows) = read_csv(&filtered);
    assert_eq!(header.len(), 2, "expected scale + one column: {header:?}");
    assert!(header[1].eq_ignore_ascii_case("V(OUT)"));
    assert!(!rows.is_empty());
    for row in &rows {
        assert!(
            row[0] >= 50e-6 - 1e-12 && row[0] <= 150e-6 + 1e-12,
            "row outside requested range: t={}",
            row[0]
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn unknown_variable_filter_is_an_error() {
    let dir = test_dir("bad_filter");
    let raw = simulate(&dir, TRAN_DECK, "raw", "tran.raw");

    let out = dir.join("out.csv");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "convert",
            raw.to_str().unwrap(),
            out.to_str().unwrap(),
            "--to",
            "csv",
            "--variables",
            "V(NOSUCHNODE)",
        ])
        .output()
        .expect("run rspice");
    assert!(!output.status.success(), "unknown variable must fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("available variables"),
        "error should list available variables: {stderr}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn malformed_json_shape_is_rejected_before_conversion() {
    let dir = test_dir("bad_json_shape");
    let input = dir.join("bad.json");
    let output_path = dir.join("out.csv");
    std::fs::write(
        &input,
        r#"{
  "analysis": "transient",
  "scale": { "name": "time", "values": [0.0, 1e-6] },
  "signals": [
    { "name": "V(OUT)", "values": [1.0] },
    { "name": "I(V1)", "real": [0.0, 0.1], "imag": [0.0] }
  ]
}
"#,
    )
    .expect("write malformed json");

    let command_output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "convert",
            input.to_str().unwrap(),
            output_path.to_str().unwrap(),
            "--to",
            "csv",
        ])
        .output()
        .expect("run rspice");
    assert!(
        !command_output.status.success(),
        "mismatched JSON waveform lengths must fail"
    );
    let stderr = String::from_utf8_lossy(&command_output.stderr);
    assert!(
        stderr.contains("V(OUT)") && stderr.contains("expected 2"),
        "error should name the first malformed signal and expected length: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// Every artifact that carries the event timelines converts to the same dump.
///
/// The rawfile's event plots and the typed result document's `digitalTraces` /
/// `realTraces` are two spellings of one history, and `run -f vcd` projects
/// that history directly. All three must agree byte for byte, or "convert to
/// VCD" would mean something different depending on which file was kept.
#[test]
fn every_carrier_of_the_event_history_converts_to_the_same_dump() {
    let dir = test_dir("vcd_sources");
    let published = simulate(&dir, XSPICE_EVENT_DECK, "vcd", "run.vcd");
    let raw = simulate(&dir, XSPICE_EVENT_DECK, "raw", "run.raw");
    let ascii = simulate(&dir, XSPICE_EVENT_DECK, "ascii", "run.ascii.raw");
    let json = simulate(&dir, XSPICE_EVENT_DECK, "json", "run.json");

    let expected = std::fs::read(&published).expect("read the published dump");
    for (source, tag) in [(raw, "raw"), (ascii, "ascii"), (json, "json")] {
        let converted = dir.join(format!("from_{tag}.vcd"));
        convert(&source, &converted, "vcd", &[]);
        assert_eq!(
            std::fs::read(&converted).expect("read the converted dump"),
            expected,
            "converting the {tag} artifact produced a different dump"
        );
    }

    // Reading a dump and writing it back normalises it and changes nothing
    // else, so the round trip is a fixed point.
    let normalised = dir.join("normalised.vcd");
    convert(&published, &normalised, "vcd", &[]);
    assert_eq!(
        std::fs::read(&normalised).expect("read the normalised dump"),
        expected
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// A run that declares a bus publishes it as one vector, on every carrier.
///
/// The declaration is the only thing about a vector port that the member node
/// names cannot state: `COUNT#1` and `COUNT#0` are two conductors until
/// something says they are one two-bit word with bit one first. The rawfile
/// and the typed document have carried that declaration since the contract
/// landed; the dump used to drop it and write two scalar wires, which made one
/// run's three exports disagree about what a word is — and made the command
/// line disagree with `TransientResult.to_vcd()` and `WasmResultHandle.toVcd()`,
/// which are the same core projection.
#[test]
fn a_declared_bus_reaches_every_carrier_as_one_vector() {
    let dir = test_dir("vcd_bus_sources");
    let deck = bus_deck(&dir);
    let published = simulate(&dir, &deck, "vcd", "run.vcd");
    let expected = std::fs::read_to_string(&published).expect("read the published dump");

    assert!(
        expected.contains("$var wire 2 ! x1.count [1:0] $end"),
        "the bus is one vector variable of its own width: {expected}"
    );
    assert!(
        !expected.contains("COUNT#") ,
        "a member's content is in the vector, so its scalar is not beside it: {expected}"
    );
    for word in ["b00 !", "b01 !", "b10 !", "b11 !"] {
        assert!(
            expected.contains(word),
            "the two-bit counter counts through {word}: {expected}"
        );
    }

    // Every artifact of the same run converts to the same dump. Before this,
    // only the dump differed — and it was the one a logic viewer reads.
    for (format, name) in [
        ("raw", "run.raw"),
        ("ascii", "run.ascii.raw"),
        ("json", "run.json"),
    ] {
        let source = simulate(&dir, &deck, format, name);
        let converted = dir.join(format!("from_{format}.vcd"));
        convert(&source, &converted, "vcd", &[]);
        assert_eq!(
            std::fs::read_to_string(&converted).expect("read the converted dump"),
            expected,
            "the {format} artifact's declaration reached the dump differently"
        );
    }

    // Reading a vector back and writing it again is a fixed point: the `$var`
    // becomes an imported declaration over synthesized `name[k]` members,
    // which project back to the same vector.
    let normalised = dir.join("normalised.vcd");
    convert(&published, &normalised, "vcd", &[]);
    assert_eq!(
        std::fs::read_to_string(&normalised).expect("read the normalised dump"),
        expected,
        "a dump carrying a vector is not a fixed point of convert"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// A table format keeps a bus as its members, and only a dump keeps the word.
///
/// A `D(node)` column is one conductor per column and always was; converting a
/// bus-declaring artifact to CSV therefore writes the flattened member columns
/// the grid already holds, and the declaration is simply not a thing a table
/// has a place for. The one column that does carry a whole word is the one
/// `load_vcd_table` packs from a dump's own vector, and it is an unsigned
/// number rather than a declaration. Both are documented in `convert --help`;
/// this is what they mean.
#[test]
fn a_bus_converts_to_a_table_as_its_member_columns_and_to_a_dump_as_a_word() {
    let dir = test_dir("vcd_bus_table");
    let deck = bus_deck(&dir);
    let raw = simulate(&dir, &deck, "raw", "run.raw");

    let grid = dir.join("grid.csv");
    convert(&raw, &grid, "csv", &[]);
    let (header, _) = read_csv(&grid);
    for member in ["D(COUNT#1)", "D(COUNT#0)"] {
        assert!(
            header.iter().any(|column| column.eq_ignore_ascii_case(member)),
            "a table carries the members, one column each: {header:?}"
        );
    }
    assert!(
        !header.iter().any(|column| column.contains("[1:0]")),
        "a table has no place for a declaration: {header:?}"
    );

    // Out through the dump, the word is one column holding its unsigned value.
    let dump = simulate(&dir, &deck, "vcd", "run.vcd");
    let packed = dir.join("packed.csv");
    convert(&dump, &packed, "csv", &[]);
    let (header, rows) = read_csv(&packed);
    let word = header
        .iter()
        .position(|column| column.eq_ignore_ascii_case("D(x1.count [1:0])"))
        .unwrap_or_else(|| panic!("the vector is one packed column: {header:?}"));
    assert_eq!(
        rows.iter().map(|row| row[word]).collect::<Vec<_>>(),
        vec![0.0, 1.0, 2.0, 3.0],
        "a two-bit counter counts 0, 1, 2, 3"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// `--variables` reaches a bus by its name, its range, or one of its bits.
///
/// The bare bus name is not one of the vector variable's own spellings — it is
/// declared `x1.count [1:0]` — so without the bus grammar the obvious thing to
/// type would have failed. A bit-select does select, and takes the whole word
/// with it, because a `$var` is as wide as it is declared and no dump carries
/// one bit of one; that is said out loud rather than done quietly.
#[test]
fn a_bus_is_selected_by_its_name_its_range_or_any_of_its_bits() {
    let dir = test_dir("vcd_bus_variables");
    let deck = bus_deck(&dir);
    let published = simulate(&dir, &deck, "vcd", "run.vcd");

    for (tag, want) in [
        ("name", "x1.count"),
        ("range", "x1.count[1:0]"),
        ("spaced", "x1.count [1:0]"),
        ("column", "D(x1.count [1:0])"),
    ] {
        let filtered = dir.join(format!("by_{tag}.vcd"));
        convert(&published, &filtered, "vcd", &["--variables", want]);
        let text = std::fs::read_to_string(&filtered).expect("read the filtered dump");
        assert!(
            text.contains("$var wire 2 ! x1.count [1:0] $end"),
            "'{want}' must select the vector: {text}"
        );
    }

    // One bit keeps the word, and says why.
    let one_bit = dir.join("by_bit.vcd");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "convert",
            published.to_str().unwrap(),
            one_bit.to_str().unwrap(),
            "--to",
            "vcd",
            "--variables",
            "x1.count[0]",
        ])
        .output()
        .expect("run rspice");
    assert!(output.status.success(), "a bit of a declared bus is a variable");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("x1.count[0]") && stderr.contains("whole"),
        "widening the selection must be stated: {stderr}"
    );
    let text = std::fs::read_to_string(&one_bit).expect("read the dump");
    assert!(
        text.contains("$var wire 2 ! x1.count [1:0] $end"),
        "the vector is kept whole: {text}"
    );

    // An index the range does not cover is still an unknown variable.
    let missing = dir.join("by_missing_bit.vcd");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "convert",
            published.to_str().unwrap(),
            missing.to_str().unwrap(),
            "--to",
            "vcd",
            "--variables",
            "x1.count[7]",
        ])
        .output()
        .expect("run rspice");
    assert!(!output.status.success(), "bit 7 of a two-bit bus is nothing");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("available variables"),
        "the refusal should list what the dump has"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// A rawfile's event plots survive the trip out through a dump and back into a
/// table: the same nodes, the same times, the same levels.
///
/// The drive band does not, and that is the documented cost: VCD has four bit
/// states and no strength, so every level comes back strongly driven.
#[test]
fn event_plots_reach_a_table_through_a_dump_with_only_the_drive_band_lost() {
    use rspice_core::execution::{digital_value_from_vcd_bit, digital_value_to_vcd_bit};
    use rspice_core::io::{VcdSignalKind, VcdValue};
    use rspice_core::xspice::DigitalStrength;

    let dir = test_dir("vcd_event_roundtrip");
    let raw = simulate(&dir, XSPICE_EVENT_DECK, "raw", "run.raw");
    let history = rspice_core::execution::decode_event_plots(
        &rspice_core::io::parse_raw_plots_file_with_limits(
            &raw,
            rspice_core::ResourceLimits::default(),
        )
        .expect("read the rawfile's plots"),
    )
    .expect("decode the event plots");

    let dump = dir.join("events.vcd");
    convert(&raw, &dump, "vcd", &[]);
    let document = rspice_core::io::parse_vcd_file(&dump).expect("read the dump");

    for trace in &history.digital_traces {
        let signal = document
            .signals
            .iter()
            .find(|signal| {
                signal.variables[0]
                    .name
                    .eq_ignore_ascii_case(&trace.node_name)
            })
            .unwrap_or_else(|| panic!("'{}' is missing from the dump", trace.node_name));
        assert_eq!(signal.kind, VcdSignalKind::Logic);
        assert_eq!(signal.changes.len(), trace.points.len());
        for (change, point) in signal.changes.iter().zip(&trace.points) {
            let VcdValue::Logic(bits) = &change.value else {
                panic!("a digital node is logic: {change:?}");
            };
            assert_eq!(bits.as_slice(), [digital_value_to_vcd_bit(point.value)]);
            let recovered = digital_value_from_vcd_bit(bits[0]);
            assert_eq!(
                recovered.to_bool(),
                point.value.to_bool(),
                "the level must survive the dump"
            );
            assert!(
                matches!(
                    recovered.strength,
                    DigitalStrength::Strong | DigitalStrength::HighZ
                ),
                "the drive band is not in the file; a level comes back driven: {recovered:?}"
            );
        }
    }

    // The table form is the dump's own ticks, held between changes, under the
    // column spelling every other RSpice surface uses.
    let table = dir.join("events.csv");
    convert(&dump, &table, "csv", &[]);
    let (header, rows) = read_csv(&table);
    assert_eq!(header[0], "time");
    let digital = header
        .iter()
        .position(|column| column.eq_ignore_ascii_case("D(d)"))
        .unwrap_or_else(|| panic!("the digital node is a D() column: {header:?}"));
    let real = header
        .iter()
        .position(|column| column.eq_ignore_ascii_case("E(rnode)"))
        .unwrap_or_else(|| panic!("the real node is an E() column: {header:?}"));

    let ticks: std::collections::BTreeSet<u64> = document
        .signals
        .iter()
        .flat_map(|signal| signal.changes.iter().map(|change| change.tick))
        .collect();
    assert_eq!(
        rows.len(),
        ticks.len(),
        "one row per distinct tick: {header:?}"
    );
    let period = document.timescale.seconds();
    for (row, tick) in rows.iter().zip(&ticks) {
        assert_eq!(row[0], *tick as f64 * period, "time is tick times period");
    }
    assert!(
        rows.iter().any(|row| row[digital] == 0.0)
            && rows.iter().any(|row| row[digital] == 1.0)
            && rows.iter().any(|row| row[digital] == 0.5),
        "the bridged node is low, high, and unknown in turn"
    );
    assert!(
        rows.iter().any(|row| row[real] != 0.0),
        "the real node's values are carried as themselves"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// A source with no event section converts from the grid columns instead, and
/// says what that costs.
#[test]
fn a_grid_digital_column_converts_to_a_dump_one_change_at_a_time() {
    use rspice_core::io::{VcdBit, VcdSignalKind, VcdValue};

    let dir = test_dir("vcd_from_grid");
    let input = dir.join("grid.csv");
    std::fs::write(
        &input,
        "time,V(out),D(clk),E(level)\n\
         0,1.0,0,0.25\n\
         1e-9,1.0,0,0.25\n\
         2e-9,1.0,1,0.5\n\
         3e-9,1.0,0.5,0.5\n",
    )
    .expect("write a grid table");

    let dump = dir.join("grid.vcd");
    convert(&input, &dump, "vcd", &[]);
    let document = rspice_core::io::parse_vcd_file(&dump).expect("read the dump");

    assert_eq!(
        document.signals.len(),
        2,
        "the analog column is not an event timeline: {document:?}"
    );
    let clk = document
        .signals
        .iter()
        .find(|signal| signal.variables[0].name == "clk")
        .expect("the digital column");
    assert_eq!(clk.kind, VcdSignalKind::Logic);
    assert_eq!(
        clk.changes
            .iter()
            .map(|change| (change.tick, change.value.clone()))
            .collect::<Vec<_>>(),
        vec![
            (0, VcdValue::Logic(vec![VcdBit::Zero])),
            (2, VcdValue::Logic(vec![VcdBit::One])),
            (3, VcdValue::Logic(vec![VcdBit::Unknown])),
        ],
        "0, 1 and 0.5 become 0, 1 and x, once per level held"
    );

    let level = document
        .signals
        .iter()
        .find(|signal| signal.variables[0].name == "level")
        .expect("the real column");
    assert_eq!(level.kind, VcdSignalKind::Real);
    assert_eq!(
        level
            .changes
            .iter()
            .map(|change| (change.tick, change.value.clone()))
            .collect::<Vec<_>>(),
        vec![(0, VcdValue::Real(0.25)), (2, VcdValue::Real(0.5)),]
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// `--variables` and `--start`/`--stop` subset a dump the way they subset a
/// table.
#[test]
fn variable_and_range_filters_apply_to_a_dump() {
    let dir = test_dir("vcd_filters");
    let published = simulate(&dir, XSPICE_EVENT_DECK, "vcd", "run.vcd");

    let filtered = dir.join("filtered.vcd");
    convert(
        &published,
        &filtered,
        "vcd",
        &["--variables", "d", "--start", "5e-9"],
    );
    let document = rspice_core::io::parse_vcd_file(&filtered).expect("read the filtered dump");
    assert_eq!(document.signals.len(), 1, "only one node was asked for");
    assert!(
        document.signals[0].variables[0]
            .name
            .eq_ignore_ascii_case("d")
    );
    let period = document.timescale.seconds();
    assert!(
        !document.signals[0].changes.is_empty(),
        "the node still changes after 5 ns"
    );
    for change in &document.signals[0].changes {
        assert!(
            change.tick as f64 * period >= 5e-9,
            "a change before the requested start survived: {change:?}"
        );
    }

    let out = dir.join("unknown.vcd");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "convert",
            published.to_str().unwrap(),
            out.to_str().unwrap(),
            "--to",
            "vcd",
            "--variables",
            "nosuchnode",
        ])
        .output()
        .expect("run rspice");
    assert!(!output.status.success(), "an unknown node must fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("available variables") && stderr.contains("D(D)"),
        "the refusal should list the dump's own columns: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// A table with no event column has no dump to write, and says so rather than
/// publishing an empty one.
#[test]
fn an_analog_only_table_has_no_dump_to_write() {
    let dir = test_dir("vcd_no_events");
    let csv = simulate(&dir, TRAN_DECK, "csv", "tran.csv");
    let out = dir.join("tran.vcd");

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "convert",
            csv.to_str().unwrap(),
            out.to_str().unwrap(),
            "--to",
            "vcd",
        ])
        .output()
        .expect("run rspice");
    assert!(!output.status.success(), "there is nothing to dump");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("D(node)/E(node)"),
        "the refusal should say what a dump is made of: {stderr}"
    );
    assert!(!out.exists(), "a refused conversion leaves no file behind");

    let _ = std::fs::remove_dir_all(&dir);
}
