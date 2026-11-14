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

const LEAF: &str = "`timescale 1ns/1ps\nmodule leaf(y);\n  output y;\n  reg y;\n  initial y = 1'b0;\n  always #5 y = ~y;\nendmodule\n";
const TOP: &str =
    "`timescale 1ns/1ps\nmodule top(q);\n  output q;\n  wire q;\n  leaf u1(.y(q));\nendmodule\n";

/// Two files the deck names with no macros are one compilation unit, as the
/// engine compiles them: a module of one instantiates a module of the
/// other, and both runtimes carry the unit's identity in their keys. Given
/// macros, a file compiles alone and cannot see the other's module.
#[test]
fn files_named_without_macros_compile_as_one_unit() {
    let scratch = Scratch::new("unit");
    scratch.write("rtl/top.v", TOP);
    scratch.write("lib/leaf.v", LEAF);
    let circuit = "t\n.hdl \"rtl/top.v\"\n.hdl \"lib/leaf.v\"\nX1 q top\nR1 q 0 1k\n.end\n";
    let sealed = seal(circuit, &scratch.deck());

    let keys = device_keys(&sealed);
    assert_eq!(keys.len(), 2, "{keys:?}");
    assert!(keys[0].ends_with("/leaf/leaf/leaf.v"), "{keys:?}");
    assert!(keys[1].ends_with("/top/top/top.v"), "{keys:?}");
    let identity = |key: &str| key.split('/').nth(2).map(str::to_owned);
    assert_eq!(
        identity(&keys[0]),
        identity(&keys[1]),
        "one unit, one identity"
    );
    let lines = sealed.source.lines().collect::<Vec<_>>();
    assert_eq!(lines[1], sealed_line(&keys[1], "top"));
    assert_eq!(lines[2], sealed_line(&keys[0], "leaf"));

    let refusal = refuse(
        "t\n.hdl \"rtl/top.v\" define=X\n.hdl \"lib/leaf.v\"\nX1 q top\n.end\n",
        &scratch.deck(),
    );
    assert_eq!(
        refusal.kind,
        ElaborationErrorKind::CompileRefusal,
        "{refusal}"
    );
    assert_eq!(refusal.line, 2);
    assert!(refusal.detail.contains("leaf"), "{refusal}");
}

/// A file of a unit that declares no module, only the macros the files
/// after it read, has no runtime; its line keeps its place as a comment.
#[test]
fn a_unit_file_of_directives_alone_keeps_its_line_as_a_comment() {
    let scratch = Scratch::new("directives");
    scratch.write("defs.v", "`define HALF 5\n");
    scratch.write("leaf.v", &LEAF.replace("#5", "#`HALF"));
    let circuit = "t\n.hdl \"defs.v\"\n.hdl \"leaf.v\"\nX1 q leaf\n.end\n";
    let sealed = seal(circuit, &scratch.deck());
    let [key] = device_keys(&sealed).try_into().expect("one runtime");
    let lines = sealed.source.lines().collect::<Vec<_>>();
    assert!(lines[1].starts_with("* "), "{}", sealed.source);
    assert_eq!(lines[2], sealed_line(&key, "leaf"));
}

/// A source's own `include` of a header beside it is read from the host,
/// recorded as a dependency of the run, and sealed by content with the
/// source: the header's bytes are part of what the key names.
#[test]
fn a_local_include_header_is_sealed_with_its_source() {
    let scratch = Scratch::new("header");
    scratch.write("models/half.vh", "`define HALF 5\n");
    scratch.write(
        "models/leaf.v",
        &format!("`include \"half.vh\"\n{}", LEAF.replace("#5", "#`HALF")),
    );
    let circuit = "t\n.hdl \"models/leaf.v\"\nX1 q leaf\n.end\n";
    let sealed = seal(circuit, &scratch.deck());
    let [key] = device_keys(&sealed).try_into().expect("one runtime");
    let header = sealed
        .dependencies
        .iter()
        .find(|dependency| dependency.resolved_path().ends_with("half.vh"))
        .unwrap_or_else(|| panic!("the header is a dependency: {:?}", sealed.dependencies));
    assert_eq!(header.source(), "`define HALF 5\n");
    assert!(header.owner_path().ends_with("leaf.v"), "{header:?}");
    assert_eq!(header.directive_line(), 1);

    scratch.write("models/half.vh", "`define HALF 7\n");
    let [changed] = device_keys(&seal(circuit, &scratch.deck()))
        .try_into()
        .expect("one runtime");
    assert_ne!(key, changed, "the header's bytes are part of the key");

    std::fs::remove_file(scratch.0.join("models/half.vh")).expect("remove the header");
    let refusal = refuse(circuit, &scratch.deck());
    assert_eq!(
        refusal.kind,
        ElaborationErrorKind::CompileRefusal,
        "{refusal}"
    );
    assert!(refusal.detail.contains("half.vh"), "{refusal}");
}

/// A source line an included file writes is read relative to that file and
/// sealed by that file and its line; the deck's own text is not touched,
/// and a refusal names the included file.
#[test]
fn a_source_line_of_an_included_file_is_sealed_where_it_stands() {
    let scratch = Scratch::new("included-line");
    scratch.write("models/leaf.v", LEAF);
    let include = scratch.write("models/leaf.inc", "* leaf\n.hdl \"leaf.v\"\n");
    let circuit = "t\n.include \"models/leaf.inc\"\nX1 q leaf\nR1 q 0 1k\n.end\n";
    let sealed = seal(circuit, &scratch.deck());
    assert_eq!(sealed.source, circuit);
    let [key] = device_keys(&sealed).try_into().expect("one runtime");
    let canonical = include.canonicalize().expect("canonical include");
    assert_eq!(
        sealed.included.get(&(canonical.clone(), 2)),
        Some(&sealed_line(&key, "leaf"))
    );
    let location = rspice_core::netlist::NetlistSourceLocation {
        path: Some(include.clone()),
        line: 2,
    };
    assert_eq!(
        sealed.included_line(&location),
        Some(sealed_line(&key, "leaf"))
    );
    let read = sealed
        .dependencies
        .iter()
        .find(|dependency| dependency.resolved_path().ends_with("leaf.v"))
        .expect("the source is a dependency");
    assert_eq!(read.owner_path(), canonical);
    assert_eq!(read.directive_line(), 2);

    scratch.write("models/leaf.inc", "* leaf\n.hdl \"leaf.v\" module=absent\n");
    let refusal = refuse(circuit, &scratch.deck());
    assert_eq!(refusal.line, 2);
    assert!(
        refusal.to_string().starts_with("line 2 of '") && refusal.to_string().contains("leaf.inc'"),
        "{refusal}"
    );
    assert_eq!(PreparationError::from(refusal).line(), None);
}

/// A library map places a unit's files in libraries, which no sealed run
/// carries; the deck is refused by the option.
#[test]
fn a_deck_with_a_library_map_is_refused() {
    let scratch = Scratch::new("libmap");
    scratch.write("rtl/top.v", TOP);
    scratch.write("lib/leaf.v", LEAF);
    let refusal = refuse(
        "t\n.options hdl_libmap=lib.map\n.hdl \"rtl/top.v\"\n.hdl \"lib/leaf.v\"\nX1 q top\n.end\n",
        &scratch.deck(),
    );
    assert_eq!(refusal.line, 3);
    assert!(refusal.detail.contains("hdl_libmap"), "{refusal}");
}

#[test]
fn the_compilers_leading_location_is_recovered() {
    assert_eq!(
        leading_location("sdq.vams:12:5: unsupported").as_deref(),
        Some("sdq.vams:12:5")
    );
    assert_eq!(leading_location("no location here"), None);
}
