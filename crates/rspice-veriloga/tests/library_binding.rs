//! Library binding: library maps and configurations (IEEE 1364-2005 clause
//! 13).
//!
//! Every case binds a design whose `leaf` cell exists in two libraries, so
//! only the rule under test decides which one an instance gets. The bound
//! hierarchy is read from the compiled plan's scope table, where each module
//! instance is known by the `library.cell` that `%l` prints (section 13.6).
//! Expected bindings come from the clause text: Icarus skips `config`, and
//! Verilator's library maps are partial.

use rspice_veriloga::{
    CompileError, CompilerOptions, NoPipelineControl, PreparedRuntimeSource, ProviderCompileError,
    SourceProviderLimits, UnitLibraries, VerilogACompiler,
};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static NONCE: AtomicU64 = AtomicU64::new(0);

struct Sources(PathBuf);
impl Sources {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "rspice-library-binding-{}-{}",
            std::process::id(),
            NONCE.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn write(&self, name: &str, source: &str) -> PathBuf {
        let path = self.0.join(name);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, source).unwrap();
        path
    }
}
impl Drop for Sources {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn compiler() -> VerilogACompiler {
    VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..CompilerOptions::default()
    })
}

/// A leaf whose output tells the two libraries' cells apart.
fn leaf(value: u8) -> String {
    format!("module leaf(v); output [7:0] v; reg [7:0] v; initial v = 8'd{value}; endmodule\n")
}

const TOP: &str = "module top; wire [7:0] a, b; leaf u1(a); leaf u2(b); endmodule\n";

/// `top` in `work`, `leaf` in both `rtlLib` (1) and `gateLib` (2), the map
/// declaring `rtlLib` first, and `configs` appended to `top.v`.
fn prepare(configs: &str) -> Result<PreparedRuntimeSource, ProviderCompileError> {
    let files = Sources::new();
    let top = files.write("top.v", &format!("{TOP}{configs}"));
    let rtl = files.write("rtl/leaf.v", &leaf(1));
    let gate = files.write("gate/leaf.v", &leaf(2));
    prepare_files(
        &[(&top, "work"), (&rtl, "rtlLib"), (&gate, "gateLib")],
        &["rtlLib", "gateLib"],
    )
}

fn prepare_files(
    files: &[(&Path, &str)],
    declared: &[&str],
) -> Result<PreparedRuntimeSource, ProviderCompileError> {
    let paths: Vec<&Path> = files.iter().map(|(path, _)| *path).collect();
    compiler().prepare_library_unit_runtime_source_with_diagnostics(
        &paths,
        &UnitLibraries {
            files: files.iter().map(|(_, library)| library.to_string()).collect(),
            declared: declared.iter().map(|library| library.to_string()).collect(),
            configs: Vec::new(),
        },
        SourceProviderLimits::UNBOUNDED,
        &NoPipelineControl,
    )
}

/// Each module instance of the compiled root, as `path=library.cell`.
fn bound(prepared: &PreparedRuntimeSource, root: &str) -> Vec<String> {
    let built = prepared
        .compile_runtime(Some(root))
        .unwrap_or_else(|error| panic!("{root} compiles: {error}"));
    built
        .canonical_ir
        .digital
        .scopes
        .iter()
        .filter(|scope| {
            scope.kind == rspice_veriloga::canonical_ir::digital::DigitalScopeKind::Module
        })
        .map(|scope| format!("{}={}", scope.path, scope.module))
        .collect()
}

fn binding_error(configs: &str) -> String {
    let error = prepare(configs).expect_err("the binding must be refused");
    let ProviderCompileError::Compile { source, .. } = &error else {
        panic!("a compile refusal: {error}");
    };
    format!("{} [{}]", source, source.diagnostic_code())
}

/// Section 13.5.1: with no configuration, an instance binds to the first
/// library in the map's declaration order that declares its cell.
#[test]
fn without_a_configuration_the_map_order_binds() {
    let prepared = prepare("").unwrap();
    assert_eq!(
        bound(&prepared, "top"),
        ["top=top", "top.u1=rtlLib.leaf", "top.u2=rtlLib.leaf"]
    );
    // The shadowed gateLib cell is no module a deck could select.
    assert!(prepared.compile_runtime(Some("leaf")).is_ok());
    assert!(prepared.compile_runtime(Some("gateLib.leaf")).is_err());
    // A configuration's copies never make an unnamed file selection ambiguous.
    assert_eq!(
        prepared.unit_files()[0]
            .modules
            .iter()
            .map(|module| module.as_str())
            .collect::<Vec<_>>(),
        ["top"]
    );
}

/// Section 13.3.1.1: the design statement names the top cell; the
/// configuration is selected by its own name.
#[test]
fn a_configuration_is_selected_by_its_name() {
    let prepared = prepare("config cfg; design work.top; endconfig\n").unwrap();
    assert_eq!(
        bound(&prepared, "cfg"),
        ["cfg=work.top", "cfg.u1=rtlLib.leaf", "cfg.u2=rtlLib.leaf"]
    );
    // A design cell without a library is the configuration's own library's.
    let prepared = prepare("config cfg; design top; endconfig\n").unwrap();
    assert_eq!(bound(&prepared, "cfg")[0], "cfg=work.top");
}

/// Section 13.3.1.2: the default liblist orders the search for every
/// instance no more specific clause selects.
#[test]
fn the_default_liblist_orders_the_search() {
    let prepared =
        prepare("config cfg; design work.top; default liblist gateLib rtlLib; endconfig\n")
            .unwrap();
    assert_eq!(
        bound(&prepared, "cfg"),
        ["cfg=work.top", "cfg.u1=gateLib.leaf", "cfg.u2=gateLib.leaf"]
    );
    // The default view is unchanged beside it.
    assert_eq!(bound(&prepared, "top")[1], "top.u1=rtlLib.leaf");
}

/// Section 13.3.1.4: a cell clause binds every instance of its cell.
#[test]
fn a_cell_clause_binds_every_instance_of_its_cell() {
    for rule in ["cell leaf use gateLib.leaf;", "cell leaf liblist gateLib;"] {
        let prepared = prepare(&format!("config cfg; design work.top; {rule} endconfig\n")).unwrap();
        assert_eq!(
            bound(&prepared, "cfg"),
            ["cfg=work.top", "cfg.u1=gateLib.leaf", "cfg.u2=gateLib.leaf"],
            "{rule}"
        );
    }
    // Naming a library selects only the instances that would otherwise bind
    // to that library's cell.
    let prepared = prepare(
        "config cfg; design work.top; cell gateLib.leaf use rtlLib.leaf; endconfig\n",
    )
    .unwrap();
    assert_eq!(bound(&prepared, "cfg")[1], "cfg.u1=rtlLib.leaf");
    let prepared = prepare(
        "config cfg; design work.top; default liblist gateLib; \
         cell gateLib.leaf use rtlLib.leaf; endconfig\n",
    )
    .unwrap();
    assert_eq!(bound(&prepared, "cfg")[1], "cfg.u1=rtlLib.leaf");
}

/// Section 13.3.1.3: an instance clause binds one instance, before any cell
/// clause, and its sibling keeps its own binding.
#[test]
fn an_instance_clause_binds_one_instance() {
    for rule in [
        "instance top.u2 use gateLib.leaf;",
        "instance top.u2 liblist gateLib;",
    ] {
        let prepared = prepare(&format!("config cfg; design work.top; {rule} endconfig\n")).unwrap();
        assert_eq!(
            bound(&prepared, "cfg"),
            ["cfg=work.top", "cfg.u1=rtlLib.leaf", "cfg.u2=gateLib.leaf"],
            "{rule}"
        );
    }
    let prepared = prepare(
        "config cfg; design work.top; cell leaf use gateLib.leaf; \
         instance top.u1 use rtlLib.leaf; endconfig\n",
    )
    .unwrap();
    assert_eq!(
        bound(&prepared, "cfg"),
        ["cfg=work.top", "cfg.u1=rtlLib.leaf", "cfg.u2=gateLib.leaf"]
    );
}

/// Section 13.3.1.5: a liblist a clause binds with is inherited by the
/// instances below the one it bound.
#[test]
fn a_clause_liblist_is_inherited_below() {
    let files = Sources::new();
    let top = files.write(
        "top.v",
        "module top; wire [7:0] a; mid u(a); endmodule\n\
         module mid(v); output [7:0] v; wire [7:0] v; leaf l(v); endmodule\n\
         config cfg; design work.top; cell mid liblist work gateLib; endconfig\n",
    );
    let rtl = files.write("rtl/leaf.v", &leaf(1));
    let gate = files.write("gate/leaf.v", &leaf(2));
    let prepared = prepare_files(
        &[(&top, "work"), (&rtl, "rtlLib"), (&gate, "gateLib")],
        &["rtlLib", "gateLib"],
    )
    .unwrap();
    assert_eq!(
        bound(&prepared, "cfg"),
        ["cfg=work.top", "cfg.u=work.mid", "cfg.u.l=gateLib.leaf"]
    );
    assert_eq!(
        bound(&prepared, "top"),
        ["top=top", "top.u=mid", "top.u.l=rtlLib.leaf"]
    );
}

/// Section 13.3.1.6: a `use` clause without a library takes the parent
/// cell's library, whatever the default liblist says.
#[test]
fn a_use_clause_without_a_library_takes_the_parents() {
    let files = Sources::new();
    let rtl_top = files.write(
        "rtl/top2.v",
        "module top2; wire [7:0] a; leaf u1(a); endmodule\n",
    );
    let rtl = files.write("rtl/leaf.v", &leaf(1));
    let gate = files.write("gate/leaf.v", &leaf(2));
    let configs = files.write(
        "cfg.v",
        "config cfg; design rtlLib.top2; default liblist gateLib; \
         instance top2.u1 use leaf; endconfig\n\
         config plain; design rtlLib.top2; default liblist gateLib; endconfig\n",
    );
    let prepared = prepare_files(
        &[
            (&rtl_top, "rtlLib"),
            (&rtl, "rtlLib"),
            (&gate, "gateLib"),
            (&configs, "work"),
        ],
        &["rtlLib", "gateLib"],
    )
    .unwrap();
    assert_eq!(
        bound(&prepared, "cfg"),
        ["cfg=rtlLib.top2", "cfg.u1=rtlLib.leaf"]
    );
    assert_eq!(
        bound(&prepared, "plain"),
        ["plain=rtlLib.top2", "plain.u1=gateLib.leaf"]
    );
}

/// A cell bound two ways in one hierarchy is two modules; one bound the same
/// way twice is one.
#[test]
fn bindings_share_a_module_only_when_they_agree() {
    let prepared = prepare(
        "config one; design work.top; instance top.u2 use gateLib.leaf; endconfig\n\
         config two; design work.top; default liblist gateLib; endconfig\n",
    )
    .unwrap();
    assert_eq!(
        bound(&prepared, "one"),
        ["one=work.top", "one.u1=rtlLib.leaf", "one.u2=gateLib.leaf"]
    );
    assert_eq!(
        bound(&prepared, "two"),
        ["two=work.top", "two.u1=gateLib.leaf", "two.u2=gateLib.leaf"]
    );
}

/// A generate branch the default parameters leave unexpanded still binds by
/// the configuration: an override expands it later.
#[test]
fn an_unexpanded_generate_branch_binds_by_the_configuration() {
    let files = Sources::new();
    let top = files.write(
        "top.v",
        "module top; wire [7:0] a; sel #(.FAST(0)) u(a); endmodule\n\
         module sel(v); parameter FAST = 1; output [7:0] v; wire [7:0] v;\n\
           generate if (FAST) begin : g fixed f(v); end else begin : g leaf l(v); end endgenerate\n\
         endmodule\n\
         module fixed(v); output [7:0] v; reg [7:0] v; initial v = 8'd9; endmodule\n\
         config cfg; design work.top; default liblist gateLib work; endconfig\n",
    );
    let rtl = files.write("rtl/leaf.v", &leaf(1));
    let gate = files.write("gate/leaf.v", &leaf(2));
    let prepared = prepare_files(
        &[(&top, "work"), (&rtl, "rtlLib"), (&gate, "gateLib")],
        &["rtlLib", "gateLib"],
    )
    .unwrap();
    let leaves = |root: &str| {
        bound(&prepared, root)
            .into_iter()
            .filter(|scope| scope.contains("leaf"))
            .map(|scope| scope.split_once('=').unwrap().1.to_owned())
            .collect::<Vec<_>>()
    };
    assert_eq!(leaves("cfg"), ["gateLib.leaf"]);
    assert_eq!(leaves("top"), ["rtlLib.leaf"]);
}

/// What a configuration cannot bind is an error that names it; what RSpice
/// does not bind is refused by name.
#[test]
fn an_unbindable_configuration_is_refused_by_name() {
    for (configs, expected) in [
        (
            "config cfg; design work.top; default liblist nowhere; endconfig\n",
            "library 'nowhere', which is not declared",
        ),
        (
            "config cfg; design work.absent; endconfig\n",
            "design cell 'work.absent'",
        ),
        (
            "config cfg; design work.top; instance top.u1 use gateLib.missing; endconfig\n",
            "'gateLib.missing', which is not declared",
        ),
        (
            "config cfg; design work.top; instance top.u1 liblist work; endconfig\n",
            "none of whose libraries declares it",
        ),
        (
            "config top; design work.top; endconfig\n",
            "has the name of a module",
        ),
        (
            "config cfg; design work.top; default liblist gateLib; \
             default liblist rtlLib; endconfig\n",
            "two `default` clauses",
        ),
        (
            "config cfg; design work.top; instance other.u1 liblist gateLib; endconfig\n",
            "does not start at its design cell",
        ),
        ("config cfg; design; endconfig\n", "names no design cell"),
    ] {
        let message = binding_error(configs);
        assert!(
            message.contains(expected) && message.contains("VA-SEM-LIBRARY-BINDING"),
            "{configs}: {message}"
        );
    }
    for (configs, expected) in [
        (
            "config cfg; design work.top; instance top.u1 use gateLib.leaf:config; endconfig\n",
            "hierarchical configuration",
        ),
        (
            "config cfg; design work.top work.leaf; endconfig\n",
            "one top cell per configuration",
        ),
        (
            "config cfg; design work.top; default liblist; endconfig\n",
            "an empty liblist",
        ),
    ] {
        let message = binding_error(configs);
        assert!(
            message.contains(expected) && message.contains("VA-SEM-UNSUPPORTED-FEATURE"),
            "{configs}: {message}"
        );
    }
}

/// A library declares a cell name once; two libraries may each declare it.
#[test]
fn a_module_name_is_declared_once_per_library() {
    let files = Sources::new();
    let first = files.write("a/leaf.v", &leaf(1));
    let second = files.write("b/leaf.v", &leaf(2));
    let error = prepare_files(&[(&first, "rtlLib"), (&second, "rtlLib")], &["rtlLib"])
        .expect_err("one library, one leaf");
    let ProviderCompileError::Compile {
        source: CompileError::ModuleSelection(message),
        ..
    } = &error
    else {
        panic!("a module-selection refusal: {error}");
    };
    assert!(
        message.contains("'leaf'") && message.contains("one library 'rtlLib'"),
        "{message}"
    );
    prepare_files(&[(&first, "rtlLib"), (&second, "gateLib")], &["rtlLib", "gateLib"])
        .expect("two libraries may each declare leaf");
}

/// A library is named by a simple identifier.
#[test]
fn a_library_name_is_an_identifier() {
    let files = Sources::new();
    let top = files.write("top.v", TOP);
    let error = prepare_files(&[(&top, "not a name")], &[])
        .expect_err("a library name with spaces");
    assert!(error.to_string().contains("not a library name"), "{error}");
}

/// A configuration written in a single file of `work` needs no map.
#[test]
fn a_configuration_needs_no_library_map() {
    let source = "module leaf(v); output [7:0] v; reg [7:0] v; initial v = 8'd1; endmodule\n\
                  module leaf2(v); output [7:0] v; reg [7:0] v; initial v = 8'd2; endmodule\n\
                  module top; wire [7:0] a, b; leaf u1(a); leaf u2(b); endmodule\n\
                  config cfg; design top; instance top.u2 use leaf2; endconfig\n";
    let artifact = compiler()
        .compile_canonical_ir_module(source, Some("cfg"))
        .unwrap_or_else(|error| panic!("cfg compiles: {error}"));
    let scopes: Vec<String> = artifact
        .digital
        .scopes
        .iter()
        .map(|scope| format!("{}={}", scope.path, scope.module))
        .collect();
    // `leaf2` binds as the default view binds it, so the configuration shares
    // that module; `%l` prints `work.leaf2` for it either way.
    assert_eq!(scopes, ["cfg=work.top", "cfg.u1=leaf", "cfg.u2=leaf2"]);
}
