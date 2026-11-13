//! `define=NAME[=VALUE]` on a Verilog-A source line.
//!
//! The macros a line defines reach that line's source alone: they decide what
//! it compiles to and which headers it reads, two lines loading one file under
//! different macros compile it twice, and an unchanged deck compiles nothing.
//!
//! The cache counters are process-wide, so this binary holds exactly one test
//! that compiles a model.

#![cfg(feature = "veriloga")]

#[path = "common/veriloga_source_fixture.rs"]
mod fixture;

use fixture::Fixture;
use rspice_core::abort_signal::NoAbort;
use rspice_core::netlist::parse_veriloga_source_directive;
use rspice_core::{Engine, Netlist, veriloga_cache_telemetry};

/// A resistor of `OHMS` (1000 when undefined) that reads `extra.vh` only
/// under `WITH_HEADER`.
const GAIN_MODULE: &str = r#"`include "disciplines.vams"
module dgain(p, n);
    inout p, n;
    electrical p, n;
`ifdef WITH_HEADER
`include "extra.vh"
`endif
`ifdef OHMS
    analog I(p, n) <+ V(p, n) / `OHMS;
`else
    analog I(p, n) <+ V(p, n) / 1000.0;
`endif
endmodule
"#;

/// Two dividers whose lower legs are the same file under two lines.
fn deck(fast: &str, slow: &str) -> String {
    format!(
        "* two lines load one file under different macros\n\
         V1 in 0 1\n\
         R1 in a 1k\n\
         X1 a 0 fast\n\
         R2 in b 1k\n\
         X2 b 0 slow\n\
         .hdl \"gain.va\" fast {fast}\n\
         .hdl \"gain.va\" slow {slow}\n\
         .end\n"
    )
}

#[test]
fn a_define_is_parsed_per_line_and_a_malformed_one_is_refused() {
    let include = parse_veriloga_source_directive(
        r#".hdl "gain.va" fast module=dgain define="OHMS=3000.0" DEFINE=WITH_HEADER define='MSG=a b'"#,
    )
    .expect("a line with defines parses");
    assert_eq!(include.model_name.as_deref(), Some("fast"));
    assert_eq!(include.selected_module.as_deref(), Some("dgain"));
    assert_eq!(
        include.defines,
        [
            ("OHMS".to_owned(), Some("3000.0".to_owned())),
            ("WITH_HEADER".to_owned(), None),
            ("MSG".to_owned(), Some("a b".to_owned())),
        ]
    );
    let include = parse_veriloga_source_directive(".va gain.va define=E= define=_x$1=2")
        .expect("an empty value and a '$' in a name parse");
    assert_eq!(
        include.defines,
        [
            ("E".to_owned(), Some(String::new())),
            ("_x$1".to_owned(), Some("2".to_owned())),
        ]
    );
    for line in [
        ".va gain.va define=",
        ".va gain.va define==1",
        ".va gain.va define=1BAD",
        ".va gain.va define=\"A B=1\"",
        ".va gain.va define=\"unterminated",
        ".va gain.va define=A alias",
    ] {
        assert!(parse_veriloga_source_directive(line).is_none(), "{line}");
        let deck = format!("* malformed define\n{line}\n.end\n");
        let error = Netlist::parse_validated(&deck).expect_err(line).to_string();
        assert!(error.contains("[define=NAME[=VALUE]]"), "{line}: {error}");
    }
}

#[test]
fn a_define_reaches_only_its_lines_source_and_keys_the_compile_cache() {
    let fixture = Fixture::new("defines");
    // A fresh source on every run, so no disk record of an earlier run can
    // answer for it.
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    fixture.write("gain.va", &format!("// run {nonce}\n{GAIN_MODULE}"));
    let header = fixture.write("extra.vh", "// read only under WITH_HEADER\n");
    let run = |text: &str| {
        let deck = fixture.write("deck.cir", text);
        let netlist = Netlist::parse_file(&deck).expect("the deck parses");
        let before = veriloga_cache_telemetry();
        let result = Engine::default()
            .run_dc_op(&netlist)
            .expect("the deck solves");
        let after = veriloga_cache_telemetry();
        let voltage = |node: &str| result.try_voltage_named(node).expect(node);
        (
            voltage("a"),
            voltage("b"),
            after.compilations_succeeded - before.compilations_succeeded,
            after.memory_hits - before.memory_hits,
            netlist,
        )
    };

    // Each line compiles the file under its own macros: 3000 ohms beside
    // the 1k reads 0.75, the undefined default reads 0.5.
    let first = deck("define=\"OHMS=3000.0\" define=WITH_HEADER", "");
    let (a, b, compiled, _, netlist) = run(&first);
    assert!(
        (a - 0.75).abs() < 1e-9,
        "the defined line reads 0.75, got {a}"
    );
    assert!((b - 0.5).abs() < 1e-9, "the plain line reads 0.5, got {b}");
    assert_eq!(
        compiled, 2,
        "one file under two sets of macros compiles twice"
    );

    // The dependency scan reads each line with its own macros: the header
    // only `WITH_HEADER` includes is a dependency of this deck, and of no
    // deck that leaves it undefined.
    let header = header.canonicalize().unwrap();
    let dependencies = Engine::default()
        .veriloga_source_dependencies_with_abort(&netlist, &NoAbort)
        .unwrap();
    assert!(
        dependencies
            .iter()
            .any(|path| path.canonicalize().is_ok_and(|path| path == header)),
        "{dependencies:?}"
    );
    let plain = Netlist::parse_file(&fixture.write("plain.cir", &deck("", ""))).unwrap();
    let dependencies = Engine::default()
        .veriloga_source_dependencies_with_abort(&plain, &NoAbort)
        .unwrap();
    assert!(
        !dependencies
            .iter()
            .any(|path| path.canonicalize().is_ok_and(|path| path == header)),
        "{dependencies:?}"
    );

    // Nothing changed: both lines are cache hits and nothing compiles.
    let (a, b, compiled, hits, _) = run(&first);
    assert!((a - 0.75).abs() < 1e-9 && (b - 0.5).abs() < 1e-9, "{a} {b}");
    assert_eq!(
        (compiled, hits),
        (0, 2),
        "an unchanged deck compiles nothing"
    );

    // A changed define recompiles its line alone: 500 ohms reads 1/3.
    let (a, b, compiled, hits, _) = run(&deck(
        "define=\"OHMS=3000.0\" define=WITH_HEADER",
        "define=\"OHMS=500.0\"",
    ));
    assert!((a - 0.75).abs() < 1e-9, "{a}");
    assert!(
        (b - 1.0 / 3.0).abs() < 1e-9,
        "the redefined line reads 1/3, got {b}"
    );
    assert_eq!((compiled, hits), (1, 1), "only the changed line recompiles");

    // A registered runtime was compiled before the deck was read; a define
    // on its line is refused rather than silently dropped.
    let sealed = Netlist::parse(
        "* a define on a registered runtime\n\
         V1 in 0 1\n\
         X1 in 0 cell\n\
         .hdl \"__rspice_project__/p/cell.va\" cell define=FAST\n\
         .end\n",
    )
    .unwrap();
    let error = Engine::default()
        .run_dc_op(&sealed)
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("define= cannot reach its preprocessor"),
        "{error}"
    );

    // A file loaded with define= compiles on its own, so a module it
    // instantiates from another deck file is undefined in it, and so is one
    // of its modules another file instantiates. Both were refused as
    // "Undefined module: 'leaf'"; the refusal now says why.
    let structural = |name: &str, body: &str| {
        format!(
            "`include \"disciplines.vams\"\nmodule {name}(p, n);\n    inout p, n;\n    electrical p, n;\n    {body}\nendmodule\n"
        )
    };
    fixture.write(
        "leaf.va",
        &structural("leaf", "analog I(p, n) <+ V(p, n) / 1000.0;"),
    );
    fixture.write("top.va", &structural("top", "leaf inner(p, n);"));
    fixture.write(
        "spare.va",
        &structural("spare", "analog I(p, n) <+ V(p, n) / 2000.0;"),
    );
    let refusal = |lines: &str| {
        let deck = fixture.write(
            "cross.cir",
            &format!("* cross-file instances\nV1 in 0 1\nR1 in a 1k\nX1 a 0 top\nR2 in b 1k\nX2 b 0 spare\n{lines}.end\n"),
        );
        let netlist = Netlist::parse_file(&deck).expect("the deck parses");
        Engine::default()
            .run_dc_op(&netlist)
            .expect_err("a module of another unit is not visible")
            .to_string()
    };
    let error = refusal(".hdl \"top.va\" define=FAST\n.hdl \"leaf.va\"\n.hdl \"spare.va\"\n");
    assert!(
        error.contains("module 'leaf' is declared in '")
            && error.contains(
                "leaf.va', another file of the deck, but this file was loaded with define= and so compiles on its own"
            )
            && !error.contains("Undefined module"),
        "{error}"
    );
    // The instantiating files are one compilation unit here.
    let error = refusal(".hdl \"top.va\"\n.hdl \"leaf.va\" define=FAST\n.hdl \"spare.va\"\n");
    assert!(
        error.contains("module 'leaf' is declared in '")
            && error.contains("leaf.va', which was loaded with define= and so compiles on its own")
            && !error.contains("Undefined module"),
        "{error}"
    );
}
