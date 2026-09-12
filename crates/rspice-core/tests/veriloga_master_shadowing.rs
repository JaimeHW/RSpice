//! Three instance-master namespaces, one name.
//!
//! A `.subckt`, a `.VERILOGA` include and the build-time generated catalog can
//! each define the master an `X` card names. A deck that spells one name twice
//! resolves it by precedence, and used to do so without saying a word; a deck
//! that misspells a master used to lose its typed refusal the moment a
//! `.VERILOGA` include appeared beside it.
#![cfg(feature = "veriloga")]

use std::path::PathBuf;

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

/// A two-terminal linear conductance. The resistance is a parameter so an
/// operating point proves which definition supplied the leg, not merely that
/// some master resolved.
fn conductance_module(name: &str) -> String {
    format!(
        "module {name}(p, n);\n\
         \x20   inout p, n;\n\
         \x20   electrical p, n;\n\
         \x20   parameter real rnom = 2000.0;\n\
         \x20   analog I(p, n) <+ V(p, n) / rnom;\n\
         endmodule\n"
    )
}

/// A Verilog-A source under a chosen file stem, in a directory of its own so
/// concurrent runs never contend for the path.
struct Library {
    path: PathBuf,
    directory: PathBuf,
}

impl Library {
    fn write(stem: &str, module: &str) -> Self {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock follows the Unix epoch")
            .as_nanos();
        let directory =
            std::env::temp_dir().join(format!("rspice_r34_{}_{nonce}", std::process::id()));
        std::fs::create_dir_all(&directory).expect("create the Verilog-A library directory");
        let path = directory.join(format!("{stem}.va"));
        std::fs::write(&path, conductance_module(module)).expect("write the Verilog-A library");
        Self { path, directory }
    }

    fn quoted(&self) -> String {
        self.path.display().to_string().replace('\\', "/")
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
        let _ = std::fs::remove_dir(&self.directory);
    }
}

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

/// The single shadowing warning a deck carries, or a panic naming what it
/// carried instead.
fn only_shadowing_warning(netlist: &Netlist) -> String {
    let rendered: Vec<String> = netlist
        .diagnostics
        .iter()
        .map(|diagnostic| {
            format!(
                "line {} [{}] {}",
                diagnostic.line, diagnostic.code, diagnostic.message
            )
        })
        .collect();
    assert_eq!(rendered.len(), 1, "{rendered:?}");
    assert_eq!(
        netlist.diagnostics[0].code, "shadowed-instance-master",
        "{rendered:?}"
    );
    netlist.diagnostics[0].message.clone()
}

/// A deck `.subckt` beside a `.VERILOGA` source of the same name: the deck's
/// definition wins, and the deck is told so.
#[test]
fn a_deck_subckt_shadowing_a_veriloga_source_warns_and_still_supplies_the_master() {
    let library = Library::write("diode", "diode");
    let netlist = Netlist::parse(&format!(
        "R3.4 deck subckt over Verilog-A\n\
         .va \"{}\"\n\
         .subckt diode p n\n\
         RS p n 3k\n\
         .ends diode\n\
         V1 in 0 1\n\
         R1 in out 1k\n\
         X1 out 0 diode\n\
         .op\n\
         .end\n",
        library.quoted()
    ))
    .expect("the shadowing deck parses");

    assert_eq!(
        only_shadowing_warning(&netlist),
        format!(
            "Instance master DIODE is defined in more than one namespace: \
             .subckt diode at line 3 and the .VERILOGA file stem of '{}' at line 2; \
             the deck's .subckt is used",
            library.quoted()
        )
    );
    assert_eq!(netlist.diagnostics[0].line, 3);

    // The warning describes the precedence; it does not change it.
    let out = engine()
        .run_dc_op(&netlist)
        .expect("the shadowing deck solves")
        .try_voltage_named("out")
        .expect("out is a solved node");
    assert!(
        (out - 0.75).abs() < 1e-9,
        "the deck's 3k .subckt must supply the leg, got {out}"
    );
}

/// A `.VERILOGA` file stem that is a generated built-in's name: the authored
/// source wins over the shipped catalog, and the deck is told so.
#[cfg(feature = "veriloga-builtins-base")]
#[test]
fn a_veriloga_file_stem_shadowing_a_generated_builtin_warns_and_still_binds_the_source() {
    assert!(
        rspice_core::device::veriloga_builtins::builtins::builtin_names()
            .iter()
            .any(|name| name.eq_ignore_ascii_case("DIODE_CMC")),
        "this pin needs the shipped CMC diode built-in"
    );
    let library = Library::write("DIODE_CMC", "DIODE_CMC");
    let netlist = Netlist::parse(&format!(
        "R3.4 Verilog-A over the generated catalog\n\
         .va \"{}\"\n\
         V1 in 0 1\n\
         R1 in out 1k\n\
         X1 out 0 DIODE_CMC\n\
         .op\n\
         .end\n",
        library.quoted()
    ))
    .expect("the built-in shadowing deck parses");

    assert_eq!(
        only_shadowing_warning(&netlist),
        format!(
            "Instance master DIODE_CMC is defined in more than one namespace: \
             the .VERILOGA file stem of '{}' at line 2 and \
             the generated built-in model DIODE_CMC; the .VERILOGA file stem is used",
            library.quoted()
        )
    );
    assert_eq!(netlist.diagnostics[0].line, 2);

    let out = engine()
        .run_dc_op(&netlist)
        .expect("the built-in shadowing deck solves")
        .try_voltage_named("out")
        .expect("out is a solved node");
    assert!(
        (out - 2.0 / 3.0).abs() < 1e-9,
        "the authored 2k Verilog-A source must supply the leg, got {out}"
    );
}

/// A deck whose masters are each defined once says nothing, however many
/// namespaces exist.
#[test]
fn a_deck_without_a_master_clash_emits_no_warning() {
    let library = Library::write("r34_no_clash_source", "r34_no_clash_module");
    let netlist = Netlist::parse(&format!(
        "R3.4 no clash\n\
         .va \"{}\"\n\
         .subckt probe p n\n\
         RS p n 3k\n\
         .ends probe\n\
         V1 in 0 1\n\
         R1 in out 1k\n\
         X1 out 0 probe\n\
         .op\n\
         .end\n",
        library.quoted()
    ))
    .expect("the clash-free deck parses");
    assert!(
        netlist.diagnostics.is_empty(),
        "{:?}",
        netlist
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.message.clone())
            .collect::<Vec<_>>()
    );
}
