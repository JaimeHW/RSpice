use std::path::{Path, PathBuf};

use rspice_core::ElaborationErrorKind;

use super::{SealedDeckHdl, leading_location, seal_deck_hdl_sources};
use crate::netlist_preparation::IncludeSearchChain;
use crate::preparation::PreparationError;

/// A directory of its own under the system temporary directory, removed when
/// the test ends.
struct Scratch(PathBuf);

impl Scratch {
    fn new(name: &str) -> Self {
        let dir =
            std::env::temp_dir().join(format!("rspice-hdl-sources-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create the scratch directory");
        Self(dir)
    }

    fn write(&self, name: &str, text: &str) -> PathBuf {
        let path = self.0.join(name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("create the parent directory");
        }
        std::fs::write(&path, text).expect("write the file");
        path
    }

    fn deck(&self) -> PathBuf {
        self.0.join("deck.cir")
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn seal(source: &str, origin: &Path) -> SealedDeckHdl {
    seal_deck_hdl_sources(source, Some(origin), &IncludeSearchChain::default())
        .unwrap_or_else(|refusal| panic!("the deck seals: {refusal}"))
}

fn refuse(source: &str, origin: &Path) -> super::DeckHdlRefusal {
    match seal_deck_hdl_sources(source, Some(origin), &IncludeSearchChain::default()) {
        Ok(sealed) => panic!("the deck must be refused, sealed:\n{}", sealed.source),
        Err(refusal) => refusal,
    }
}

fn device_keys(sealed: &SealedDeckHdl) -> Vec<String> {
    let mut keys = sealed
        .runtimes
        .device_runtimes()
        .map(|runtime| runtime.source_key().to_owned())
        .collect::<Vec<_>>();
    keys.sort();
    keys
}

/// The sealed directive a key and alias take on a deck line.
fn sealed_line(key: &str, alias: &str) -> String {
    format!(".veriloga \"{key}\" {alias}")
}

const PAIR: &str = "module tb(c);\n  output c;\n  reg c;\n  initial c = 1'b0;\nendmodule\n\
                    module dut(c, q);\n  input c;\n  output q;\n  reg q;\n  initial q = 1'b0;\n  always @(posedge c) q <= ~q;\nendmodule\n";

#[test]
fn a_deck_naming_no_source_is_returned_as_written() {
    let source = "t\nR1 a 0 1k\n.end\n";
    let sealed = seal_deck_hdl_sources(source, None, &IncludeSearchChain::default())
        .expect("nothing to seal");
    assert_eq!(sealed.source, source);
    assert!(sealed.runtimes.is_empty());
    assert!(sealed.dependencies.is_empty());
}

/// One file the deck names twice, each line selecting another module, seals
/// a runtime for each under keys that name the module, the alias and the
/// file, and rewrites both lines in place.
#[test]
fn one_file_named_by_two_directives_seals_two_runtimes() {
    let scratch = Scratch::new("two-directives");
    scratch.write("pair.v", PAIR);
    let circuit =
        "t\n.hdl \"pair.v\" module=tb\n.hdl \"pair.v\" module=dut\nxtb c tb\nxdut c q dut\n.end\n";
    let sealed = seal(circuit, &scratch.deck());

    let keys = device_keys(&sealed);
    assert_eq!(keys.len(), 2, "{keys:?}");
    assert!(
        keys.iter()
            .all(|key| key.starts_with("__rspice_project__/deck/")),
        "{keys:?}"
    );
    assert!(keys[0].ends_with("/dut/dut/pair.v"), "{keys:?}");
    assert!(keys[1].ends_with("/tb/tb/pair.v"), "{keys:?}");
    let lines = sealed.source.lines().collect::<Vec<_>>();
    assert_eq!(lines.len(), circuit.lines().count(), "no line moves");
    assert_eq!(lines[1], sealed_line(&keys[1], "tb"));
    assert_eq!(lines[2], sealed_line(&keys[0], "dut"));
    assert_eq!(lines[3..], circuit.lines().collect::<Vec<_>>()[3..]);
    assert_eq!(sealed.dependencies.len(), 2, "each line's read is recorded");
    assert!(
        sealed.dependencies.iter().all(
            |dependency| dependency.source() == PAIR && dependency.requested_path() == "pair.v"
        )
    );
    assert_eq!(
        sealed
            .dependencies
            .iter()
            .map(|dependency| dependency.directive_line())
            .collect::<Vec<_>>(),
        [2, 3]
    );
}

/// `.ahdl_include` is a source line like `.hdl`, and sealing it is idempotent:
/// a line that already names a sealed key is not read again.
#[test]
fn an_ahdl_include_line_seals_and_a_sealed_deck_seals_to_itself() {
    let scratch = Scratch::new("ahdl-include");
    scratch.write("models/pair.v", PAIR);
    let circuit = "t\n.ahdl_include \"models/pair.v\" tb module=tb\nxtb c tb\n.end\n";
    let sealed = seal(circuit, &scratch.deck());
    let keys = device_keys(&sealed);
    assert_eq!(keys.len(), 1, "{keys:?}");
    assert!(keys[0].ends_with("/tb/tb/pair.v"), "{keys:?}");
    assert_eq!(
        sealed.source.lines().nth(1),
        Some(sealed_line(&keys[0], "tb").as_str())
    );

    let again = seal(&sealed.source, &scratch.deck());
    assert_eq!(again.source, sealed.source);
    assert!(again.runtimes.is_empty());
}

/// A file of connect rules and connect modules alone is a connection
/// library, bound under its file stem, while a file of modules beside it
/// seals as a device runtime.
#[test]
fn a_connect_only_source_seals_as_a_connection_library() {
    let scratch = Scratch::new("connect-only");
    scratch.write(
        "drv.vams",
        "module drv(p, q);\n  inout p;\n  electrical p;\n  output q;\n  reg q;\n  initial q = 1;\n  analog I(p) <+ 0;\nendmodule\n",
    );
    let mut rules = rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES
        .iter()
        .map(|(_, source)| *source)
        .collect::<String>();
    rules.push_str(
        "\nconnectrules Low; connect d2a #(.vsup(1.0)); endconnectrules\n\
         connectrules High; connect d2a #(.vsup(5.0)); endconnectrules\n",
    );
    scratch.write("rules.vams", &rules);
    std::fs::create_dir_all(scratch.0.join("deck")).expect("create the deck directory");
    let deck = scratch.0.join("deck").join("mx.tran.cir");
    let circuit = "connect-only rules\n.hdl \"../drv.vams\"\n.hdl \"../rules.vams\"\nV1 p 0 1\nX1 p q drv\nR1 q 0 1meg\n\
                   .options connectrules=High connectrules_source=rules\n.tran 0.2n 2n\n.end\n";
    let sealed = seal(circuit, &deck);

    assert_eq!(sealed.runtimes.device_runtimes().len(), 1);
    let connections = sealed
        .runtimes
        .sources()
        .filter(|binding| binding.provenance_label().contains("-connections:"))
        .collect::<Vec<_>>();
    let [library] = connections.as_slice() else {
        panic!("one connection library, saw {}", connections.len());
    };
    assert_eq!(library.netlist_alias(), "rules");
    assert_eq!(
        sealed.source.lines().nth(2),
        Some(sealed_line(library.source_key(), "rules").as_str())
    );
}

/// A source with neither modules nor connect rules selects nothing, as the
/// engine refuses it.
#[test]
fn an_empty_source_is_refused_as_an_unknown_module() {
    let scratch = Scratch::new("empty");
    scratch.write("empty.vams", "// no modules, no rules\n");
    let refusal = refuse("t\n.hdl \"empty.vams\"\n.end\n", &scratch.deck());
    assert_eq!(
        refusal.kind,
        ElaborationErrorKind::UnknownModule,
        "{refusal}"
    );
    assert_eq!(refusal.line, 2);
    assert_eq!(refusal.file_name(), "empty.vams");
    assert!(refusal.to_string().starts_with("line 2: "), "{refusal}");
}

/// A line's `define=` reaches the preprocessor, and what was compiled names
/// the key: the same file under other macros is another runtime.
#[test]
fn a_define_reaches_the_preprocessor_and_changes_the_key() {
    let scratch = Scratch::new("define");
    scratch.write(
        "gate.v",
        "module gate(q);\n  output q;\n  reg q;\n`ifdef FAST\n  initial q = 1'b1;\n`else\n  this is not verilog;\n`endif\nendmodule\n",
    );
    let deck = |define: &str| format!("t\n.hdl \"gate.v\" gate {define}\nx1 q gate\n.end\n");

    let fast = seal(&deck("define=FAST"), &scratch.deck());
    let fast_one = seal(&deck("define=FAST=1"), &scratch.deck());
    let [fast_key] = device_keys(&fast).try_into().expect("one runtime");
    let [fast_one_key] = device_keys(&fast_one).try_into().expect("one runtime");
    assert_ne!(fast_key, fast_one_key, "the macros are part of the key");

    let refusal = refuse(&deck(""), &scratch.deck());
    assert_eq!(
        refusal.kind,
        ElaborationErrorKind::CompileRefusal,
        "{refusal}"
    );
    assert_eq!(refusal.line, 2);
    // The source does not parse, so no module was selected yet, and the
    // parser's message carries an offset, not a `file:line`: the location
    // is the file.
    assert_eq!(refusal.module, None, "{refusal}");
    assert!(refusal.detail.contains("Parse error"), "{refusal}");
    assert_eq!(refusal.location(), refusal.path.display().to_string());
}

/// A file declaring two modules, named without `module=` or an alias, seals
/// each module the deck instantiates: the line names the first and the
/// second is named before `.end`. A deck instantiating neither cannot say
/// which it means.
#[test]
fn a_multi_module_source_seals_the_modules_the_deck_instantiates() {
    let scratch = Scratch::new("multi-module");
    scratch.write("pair.v", PAIR);
    let circuit = "t\n.hdl \"pair.v\"\nxtb c tb\nxdut c q dut\nR1 q 0 1k\n.end\n";
    let sealed = seal(circuit, &scratch.deck());

    let keys = device_keys(&sealed);
    assert_eq!(keys.len(), 2, "{keys:?}");
    let lines = sealed.source.lines().collect::<Vec<_>>();
    let named = lines
        .iter()
        .filter(|line| line.starts_with(".veriloga \"__rspice_project__/deck/"))
        .count();
    assert_eq!(named, 2, "{}", sealed.source);
    assert!(lines[1].starts_with(".veriloga \"__rspice_project__/deck/"));
    assert_eq!(lines[2..5], circuit.lines().collect::<Vec<_>>()[2..5]);
    assert_eq!(lines.last(), Some(&".end"));

    let refusal = refuse("t\n.hdl \"pair.v\"\nR1 q 0 1k\n.end\n", &scratch.deck());
    assert_eq!(
        refusal.kind,
        ElaborationErrorKind::ModuleNotSelected,
        "{refusal}"
    );
    assert_eq!(refusal.line, 2);
}

/// A file that cannot be read is refused by name, with its deck line, before
/// anything is compiled; preparation reports the same line.
#[test]
fn a_missing_source_is_refused_naming_the_line_and_the_file() {
    let scratch = Scratch::new("missing");
    scratch.write("pair.v", PAIR);
    let refusal = refuse(
        "t\n.hdl \"pair.v\" module=tb\nxtb c tb\n.hdl \"absent.va\" amp\n.end\n",
        &scratch.deck(),
    );
    assert_eq!(
        refusal.kind,
        ElaborationErrorKind::MissingSource,
        "{refusal}"
    );
    assert_eq!(refusal.line, 4);
    assert_eq!(refusal.file_name(), "absent.va");
    let text = refusal.to_string();
    assert!(
        text.starts_with("line 4: the Verilog-A source 'absent.va' cannot be read"),
        "{text}"
    );
    let error = PreparationError::from(refusal);
    assert_eq!(error.line(), Some(4));
}

/// An unsaved deck has no directory for a relative path, and says so.
#[test]
fn an_unsaved_deck_naming_a_relative_source_is_told_to_save() {
    let refusal = seal_deck_hdl_sources(
        "t\n.hdl \"pair.v\"\n.end\n",
        None,
        &IncludeSearchChain::default(),
    )
    .expect_err("a relative path with no deck file resolves nowhere");
    assert_eq!(refusal.kind, ElaborationErrorKind::MissingSource);
    assert!(refusal.detail.contains("save the deck"), "{refusal}");
}

/// A source line in a false `.if` branch names nothing, so its file is never
/// read.
#[test]
fn a_source_line_in_a_false_branch_is_left_alone() {
    let scratch = Scratch::new("false-branch");
    scratch.write("pair.v", PAIR);
    let circuit =
        "t\n.if 0\n.hdl \"absent.va\" amp\n.endif\n.hdl \"pair.v\" module=tb\nxtb c tb\n.end\n";
    let sealed = seal(circuit, &scratch.deck());
    assert_eq!(sealed.runtimes.device_runtimes().len(), 1);
    let lines = sealed.source.lines().collect::<Vec<_>>();
    assert_eq!(lines[2], ".hdl \"absent.va\" amp");
    assert!(lines[4].starts_with(".veriloga \"__rspice_project__/deck/"));
}

/// A rewritten line keeps its own line ending, so a CRLF deck stays CRLF.
#[test]
fn a_crlf_deck_keeps_its_line_endings() {
    let scratch = Scratch::new("crlf");
    scratch.write("pair.v", PAIR);
    let circuit = "t\r\n.hdl \"pair.v\" module=tb\r\nxtb c tb\r\n.end\r\n";
    let sealed = seal(circuit, &scratch.deck());
    let [key] = device_keys(&sealed).try_into().expect("one runtime");
    assert_eq!(
        sealed.source,
        format!("t\r\n{}\r\nxtb c tb\r\n.end\r\n", sealed_line(&key, "tb"))
    );
}

#[test]
fn the_compilers_leading_location_is_recovered() {
    assert_eq!(
        leading_location("sdq.vams:12:5: unsupported").as_deref(),
        Some("sdq.vams:12:5")
    );
    assert_eq!(leading_location("no location here"), None);
}
