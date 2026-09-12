//! What a deck gets back when its Verilog-A or mixed instance cannot be bound.
//!
//! Every refusal on the `.VERILOGA`/X-card seam is one
//! [`rspice_core::ElaborationError`]: the instance, the master, a typed
//! [`rspice_core::ElaborationErrorKind`], the source span where one is known,
//! and the sentence the site wrote. That is the whole point of these tests —
//! a workbench places the failure on a symbol and decides what to offer from
//! the kind, and neither of those may depend on the prose.
//!
//! The seam used to report through two untyped `SimulationError` variants,
//! picked per site rather than per meaning, so a wrong terminal count was a
//! circuit error while an unreadable `.va` was a netlist error and the CLI
//! gave them different exit statuses. These pin the classification that
//! replaced that, one deck per kind.
//!
//! Two kinds have no deck. `CacheCorrupt` needs a compiled artifact whose
//! digest disagrees with its payload and `Internal` needs the engine to fail a
//! step it expected to complete; a deck that produced either would be a bug
//! report, not a fixture. Both are still classified — see
//! [`the_two_kinds_no_deck_reaches_are_still_reported_as_engine_failures`] —
//! and the sites that raise them are held by the source scan in
//! `engine::builder::elaboration_scan`.
#![cfg(feature = "veriloga")]

use rspice_core::{ElaborationError, ElaborationErrorKind, Engine, Netlist, SimulationError};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static MODEL_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// A `.va` written to a unique path, deleted when the guard drops.
///
/// The uniqueness matters: the engine's Verilog-A cache is keyed by canonical
/// path, so two tests sharing a filename would share a cache entry.
struct ModelFile(PathBuf);

impl ModelFile {
    fn new(name: &str, source: &str) -> Self {
        let sequence = MODEL_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "rspice_elaboration_{name}_{}_{sequence}.va",
            std::process::id()
        ));
        let mut file = std::fs::File::create(&path).expect("create model file");
        file.write_all(source.as_bytes()).expect("write model");
        Self(path)
    }

    fn deck_path(&self) -> String {
        self.0.display().to_string().replace('\\', "/")
    }
}

impl Drop for ModelFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

const ANALOG_RESISTOR: &str = r#"
`include "disciplines.vams"
module ares(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;

const MIXED_DIVIDER: &str = r#"
`include "disciplines.vams"
module mdiv(p, n, q);
    inout p, n;
    electrical p, n;
    output q;
    reg q;
    parameter real r = 1000.0;
    initial q = 1'b0;
    always #5 q = ~q;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;

const BIDIRECTIONAL_MIXED: &str = r#"
`include "disciplines.vams"
module bidi_mixed(p, n, io);
    inout p, n;
    electrical p, n;
    inout io;
    reg io;
    initial io = 1'b0;
    always #5 io = ~io;
    analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#;

/// The typed refusal a deck produces, or a panic naming what came back instead.
///
/// Going through `build_circuit` rather than an analysis is deliberate: every
/// one of these is decided while the circuit is being built, and an analysis
/// would only add a second chance for something else to fail first.
fn elaboration_error(deck: &str) -> ElaborationError {
    let netlist = Netlist::parse(deck).expect("the deck parses");
    let error = Engine::default()
        .build_circuit(&netlist)
        .err()
        .expect("the deck must be refused");
    match error {
        SimulationError::Elaboration(error) => *error,
        other => panic!(
            "the seam must report a typed elaboration error, got {}: {other}",
            other.descriptor().code
        ),
    }
}

/// The span, as the engine renders it, or `"<none>"`.
fn span_of(error: &ElaborationError) -> String {
    error
        .span
        .as_ref()
        .map_or_else(|| "<none>".to_owned(), ToString::to_string)
}

#[test]
fn a_missing_verilog_a_source_is_a_missing_source_at_that_file() {
    let error = elaboration_error(
        "* a .VERILOGA naming a file that is not there\n\
         v1 a 0 1\n\
         r1 a 0 1k\n\
         x1 a 0 absent_master\n\
         .va \"C:/rspice/elaboration/definitely_absent.va\" absent_master\n\
         .op\n\
         .end\n",
    );
    assert_eq!(error.kind, ElaborationErrorKind::MissingSource);
    // Line 0 is this seam's "the file itself": the compiler reports its own
    // offsets inside the detail, and the deck line that authored the
    // `.VERILOGA` card is not retained by the parsed netlist.
    let span = error.span.as_ref().expect("a missing file names itself");
    assert_eq!(span.line, 0);
    assert_eq!(
        span.path
            .as_deref()
            .and_then(std::path::Path::file_name)
            .and_then(std::ffi::OsStr::to_str),
        Some("definitely_absent.va"),
        "the span must name the file the deck asked for: {span}"
    );
    assert!(
        error
            .to_string()
            .contains("does not exist or is unreadable"),
        "{error}"
    );
}

#[test]
fn a_source_declaring_nothing_is_an_unknown_module_at_that_file() {
    let source = ModelFile::new("declares_nothing", "// no module and no connectrules\n");
    let error = elaboration_error(&format!(
        "* a .va with neither a device module nor a connection library\n\
         v1 a 0 1\n\
         r1 a 0 1k\n\
         .va \"{}\" nothing\n\
         .op\n\
         .end\n",
        source.deck_path()
    ));
    assert_eq!(error.kind, ElaborationErrorKind::UnknownModule);
    assert_eq!(error.span.as_ref().map(|span| span.line), Some(0));
    assert_eq!(error.instance, None, "no instance was reached: {error}");
    assert!(error.module.is_some(), "the source names itself: {error}");
}

#[test]
fn a_name_two_sources_claim_is_module_not_selected_on_the_instance() {
    let first = ModelFile::new(
        "ambiguous_first",
        "`include \"disciplines.vams\"\n\
         module amb(p, n);\n inout p, n;\n electrical p, n;\n\
         analog I(p, n) <+ V(p, n) / 1000.0;\nendmodule\n",
    );
    let second = ModelFile::new(
        "ambiguous_second",
        "`include \"disciplines.vams\"\n\
         module amb(p, n);\n inout p, n;\n electrical p, n;\n\
         analog I(p, n) <+ V(p, n) / 2000.0;\nendmodule\n",
    );
    let error = elaboration_error(&format!(
        "* two sources declare the same module name\n\
         v1 a 0 1\n\
         x1 a 0 amb\n\
         r1 a 0 1k\n\
         .va \"{}\"\n\
         .va \"{}\"\n\
         .op\n\
         .end\n",
        first.deck_path(),
        second.deck_path()
    ));
    assert_eq!(error.kind, ElaborationErrorKind::ModuleNotSelected);
    assert_eq!(error.instance.as_deref(), Some("x1"));
    assert_eq!(error.module.as_deref(), Some("amb"));
    // Nothing points at a line here and the error says so rather than
    // inventing one: a parsed element carries no source location.
    assert_eq!(span_of(&error), "<none>");
}

#[test]
fn a_parameter_the_master_does_not_declare_is_parameter_unknown() {
    let model = ModelFile::new("unknown_parameter", MIXED_DIVIDER);
    let error = elaboration_error(&format!(
        "* an instance parameter the module never declared\n\
         v1 a 0 1\n\
         x1 a 0 q mdiv zzz=1\n\
         rq q 0 1k\n\
         r1 a 0 1k\n\
         .va \"{}\" mdiv\n\
         .tran 1n 20n\n\
         .end\n",
        model.deck_path()
    ));
    assert_eq!(error.kind, ElaborationErrorKind::ParameterUnknown);
    assert_eq!(error.instance.as_deref(), Some("x1"));
    assert_eq!(error.module.as_deref(), Some("mdiv"));
    assert!(error.detail.contains("'zzz'"), "{error}");
}

#[test]
fn a_value_outside_its_domain_is_parameter_value_on_both_routes() {
    let analog = ModelFile::new("analog_multiplicity", ANALOG_RESISTOR);
    let analog_error = elaboration_error(&format!(
        "* a negative multiplicity on the analog route\n\
         v1 a 0 1\n\
         x1 a 0 ares m=-2\n\
         r1 a 0 1k\n\
         .va \"{}\" ares\n\
         .op\n\
         .end\n",
        analog.deck_path()
    ));
    assert_eq!(analog_error.kind, ElaborationErrorKind::ParameterValue);
    assert_eq!(analog_error.instance.as_deref(), Some("x1"));

    // The mixed route reaches the same verdict for the same mistake, which is
    // the property that matters: adding a process to a module must not change
    // the vocabulary its parameter mistakes are reported in.
    let mixed = ModelFile::new("mixed_multiplicity", MIXED_DIVIDER);
    let mixed_error = elaboration_error(&format!(
        "* a zero multiplicity on the mixed route\n\
         v1 a 0 1\n\
         x1 a 0 q mdiv m=0\n\
         rq q 0 1k\n\
         r1 a 0 1k\n\
         .va \"{}\" mdiv\n\
         .tran 1n 20n\n\
         .end\n",
        mixed.deck_path()
    ));
    assert_eq!(mixed_error.kind, ElaborationErrorKind::ParameterValue);
    assert_eq!(mixed_error.instance.as_deref(), Some("x1"));
}

#[test]
fn the_wrong_number_of_nets_is_port_count_on_both_routes() {
    let analog = ModelFile::new("analog_port_count", ANALOG_RESISTOR);
    let analog_error = elaboration_error(&format!(
        "* three nets on a two-terminal master\n\
         v1 a 0 1\n\
         x1 a 0 b ares\n\
         r1 a 0 1k\n\
         .va \"{}\" ares\n\
         .op\n\
         .end\n",
        analog.deck_path()
    ));
    assert_eq!(analog_error.kind, ElaborationErrorKind::PortCount);
    assert_eq!(analog_error.module.as_deref(), Some("ares"));

    let mixed = ModelFile::new("mixed_port_count", MIXED_DIVIDER);
    let mixed_error = elaboration_error(&format!(
        "* the discrete port left unconnected\n\
         v1 a 0 1\n\
         x1 a 0 mdiv\n\
         r1 a 0 1k\n\
         .va \"{}\" mdiv\n\
         .tran 1n 20n\n\
         .end\n",
        mixed.deck_path()
    ));
    assert_eq!(mixed_error.kind, ElaborationErrorKind::PortCount);
    assert_eq!(mixed_error.module.as_deref(), Some("mdiv"));
}

#[test]
fn a_port_this_route_cannot_bridge_is_port_discipline() {
    let bidirectional = ModelFile::new("bidirectional", BIDIRECTIONAL_MIXED);
    let declared = elaboration_error(&format!(
        "* a bidirectional discrete boundary\n\
         v1 a 0 1\n\
         x1 a 0 io bidi_mixed\n\
         rio io 0 1k\n\
         r1 a 0 1k\n\
         .va \"{}\" bidi_mixed\n\
         .tran 1n 20n\n\
         .end\n",
        bidirectional.deck_path()
    ));
    assert_eq!(declared.kind, ElaborationErrorKind::PortDiscipline);
    assert_eq!(declared.instance.as_deref(), Some("x1"));
    assert!(declared.detail.contains("'io'"), "{declared}");

    // The same kind for a port the deck wired somewhere it cannot go, because
    // both are answered at the same terminal of the same symbol.
    let mixed = ModelFile::new("port_to_ground", MIXED_DIVIDER);
    let connected = elaboration_error(&format!(
        "* a discrete boundary tied to ground\n\
         v1 a 0 1\n\
         x1 a 0 0 mdiv\n\
         r1 a 0 1k\n\
         .va \"{}\" mdiv\n\
         .tran 1n 20n\n\
         .end\n",
        mixed.deck_path()
    ));
    assert_eq!(connected.kind, ElaborationErrorKind::PortDiscipline);
    assert!(connected.detail.contains("ground"), "{connected}");
}

#[test]
fn selecting_connect_rules_that_do_not_exist_is_a_connect_rule_refusal() {
    let model = ModelFile::new("connect_rules", MIXED_DIVIDER);
    let error = elaboration_error(&format!(
        "* a connectrules block the design does not declare\n\
         v1 a 0 1\n\
         x1 a 0 q mdiv\n\
         rq q 0 1k\n\
         r1 a 0 1k\n\
         .options connectrules=NotThere\n\
         .va \"{}\" mdiv\n\
         .tran 1n 20n\n\
         .end\n",
        model.deck_path()
    ));
    assert_eq!(error.kind, ElaborationErrorKind::ConnectRule);
    // Selection happens once for the design, before any X-card is bound, so
    // there is deliberately no instance to mark on a schematic.
    assert_eq!(error.instance, None);
    assert!(error.detail.contains("NotThere"), "{error}");
}

#[test]
fn a_source_the_compiler_refuses_is_a_compile_refusal_at_that_file() {
    let broken = ModelFile::new("unparsable", "module broken(a); this is not verilog;\n");
    let error = elaboration_error(&format!(
        "* a .va the front end cannot parse\n\
         v1 a 0 1\n\
         x1 a 0 broken\n\
         r1 a 0 1k\n\
         .va \"{}\" broken\n\
         .op\n\
         .end\n",
        broken.deck_path()
    ));
    assert_eq!(error.kind, ElaborationErrorKind::CompileRefusal);
    let span = error
        .span
        .as_ref()
        .expect("the refused source names itself");
    assert_eq!(span.line, 0);
    assert_eq!(
        span.path
            .as_deref()
            .and_then(std::path::Path::file_name)
            .and_then(std::ffi::OsStr::to_str),
        broken.0.file_name().and_then(std::ffi::OsStr::to_str),
        "the span must name the source the compiler refused: {span}"
    );
}

/// Every kind a deck reaches is a netlist failure, and the two it cannot are
/// simulation failures.
///
/// This is the classification a CLI turns into an exit status and a Python
/// binding into an exception class. It used to follow the variant each site
/// happened to pick, which is why a wrong terminal count exited as a failed
/// simulation and a missing file as bad input.
#[test]
fn the_two_kinds_no_deck_reaches_are_still_reported_as_engine_failures() {
    for kind in [
        ElaborationErrorKind::CacheCorrupt,
        ElaborationErrorKind::Internal,
    ] {
        let error = SimulationError::from(ElaborationError {
            instance: Some("x1".to_owned()),
            module: Some("counter".to_owned()),
            kind,
            span: None,
            detail: "an artifact or a binding step the deck cannot influence".to_owned(),
        });
        let descriptor = error.descriptor();
        assert_eq!(
            descriptor.category.as_str(),
            "simulation",
            "{kind:?} is not the deck author's to fix"
        );
    }

    for kind in [
        ElaborationErrorKind::MissingSource,
        ElaborationErrorKind::UnknownModule,
        ElaborationErrorKind::ModuleNotSelected,
        ElaborationErrorKind::ParameterUnknown,
        ElaborationErrorKind::ParameterValue,
        ElaborationErrorKind::PortCount,
        ElaborationErrorKind::PortDiscipline,
        ElaborationErrorKind::ConnectRule,
        ElaborationErrorKind::CompileRefusal,
    ] {
        let error = SimulationError::from(ElaborationError {
            instance: None,
            module: None,
            kind,
            span: None,
            detail: "authored input".to_owned(),
        });
        assert_eq!(
            error.descriptor().category.as_str(),
            "netlist",
            "{kind:?} is fixed by editing the deck or its sources"
        );
    }
}

/// The rendering: one prefix, built once, in front of the site's own sentence.
#[test]
fn the_rendering_names_the_span_the_instance_and_the_master_before_the_reason() {
    let full = ElaborationError {
        instance: Some("x1".to_owned()),
        module: Some("counter".to_owned()),
        kind: ElaborationErrorKind::PortCount,
        span: Some(rspice_core::netlist::NetlistSourceLocation::in_file(
            "counter.va",
            0,
        )),
        detail: "the master declares at most 3 terminal(s) and the card connects 4".to_owned(),
    };
    assert_eq!(
        full.to_string(),
        "Elaboration error at counter.va:0: instance 'x1' (module 'counter'): the instance does \
         not connect the nets the master declares: the master declares at most 3 terminal(s) and \
         the card connects 4"
    );

    // Each optional part is simply absent rather than rendered empty.
    let bare = ElaborationError {
        instance: None,
        module: None,
        kind: ElaborationErrorKind::ConnectRule,
        span: None,
        detail: "Unknown connectrules 'Low'".to_owned(),
    };
    assert_eq!(
        bare.to_string(),
        "Elaboration error: the connect rules do not settle this boundary: Unknown connectrules \
         'Low'"
    );
}

/// The kind tokens are a wire contract: the UI, the Python binding and any
/// regression suite branch on them, so they may not drift with a rename.
#[test]
fn every_kind_has_a_distinct_stable_token() {
    let tokens = [
        (ElaborationErrorKind::MissingSource, "missing_source"),
        (ElaborationErrorKind::UnknownModule, "unknown_module"),
        (
            ElaborationErrorKind::ModuleNotSelected,
            "module_not_selected",
        ),
        (ElaborationErrorKind::ParameterUnknown, "parameter_unknown"),
        (ElaborationErrorKind::ParameterValue, "parameter_value"),
        (ElaborationErrorKind::PortCount, "port_count"),
        (ElaborationErrorKind::PortDiscipline, "port_discipline"),
        (ElaborationErrorKind::ConnectRule, "connect_rule"),
        (ElaborationErrorKind::CompileRefusal, "compile_refusal"),
        (ElaborationErrorKind::CacheCorrupt, "cache_corrupt"),
        (ElaborationErrorKind::Internal, "internal"),
    ];
    let mut seen = std::collections::BTreeSet::new();
    for (kind, token) in tokens {
        assert_eq!(kind.as_str(), token);
        assert!(seen.insert(token), "duplicate kind token: {token}");
    }
    assert_eq!(seen.len(), 11);
}
