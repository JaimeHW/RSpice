//! The SystemVerilog front end: the keyword set a file's name or
//! `` `begin_keywords `` selects (IEEE 1800-2017 section 22.14), packages and
//! imports (clause 26), and real nettypes (section 6.6.7).

use rspice_veriloga::{
    CompilerOptions, NoPipelineControl, PreparedRuntimeSource, ProviderCompileError,
    SourceCompileDiagnostic, SourceProviderLimits, VerilogACompiler,
};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static NONCE: AtomicU64 = AtomicU64::new(0);

/// A directory of source files, removed afterwards.
struct Sources(PathBuf);

impl Sources {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "rspice-sv-grammar-{}-{}",
            std::process::id(),
            NONCE.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }

    fn write(&self, name: &str, source: &str) -> PathBuf {
        let path = self.0.join(name);
        std::fs::write(&path, source).unwrap();
        path
    }

    /// The files, already written, prepared as one compilation unit.
    fn unit(&self, names: &[&str]) -> Result<PreparedRuntimeSource, ProviderCompileError> {
        let paths: Vec<PathBuf> = names.iter().map(|name| self.0.join(name)).collect();
        let paths: Vec<&std::path::Path> = paths.iter().map(PathBuf::as_path).collect();
        compiler().prepare_unit_runtime_source_with_diagnostics(
            &paths,
            SourceProviderLimits::UNBOUNDED,
            &NoPipelineControl,
        )
    }

    /// The first error diagnostic of preparing `names` and compiling `top`.
    fn refusal(&self, names: &[&str], top: &str) -> SourceCompileDiagnostic {
        let diagnostics = match self.unit(names) {
            Err(ProviderCompileError::Compile { diagnostics, .. }) => diagnostics,
            Err(ProviderCompileError::Source(error)) => panic!("preprocessing failed: {error}"),
            Ok(prepared) => match prepared.compile_runtime(Some(top)) {
                Ok(_) => panic!("{top} compiled"),
                Err(error) => prepared.diagnostics_for_error(&error),
            },
        };
        diagnostics
            .into_iter()
            .find(|diagnostic| {
                diagnostic.severity == rspice_veriloga::CompileDiagnosticSeverity::Error
            })
            .expect("an error diagnostic")
    }

    /// The preprocessor's refusal of `names`.
    fn preprocessor_refusal(&self, names: &[&str]) -> rspice_veriloga::PreprocessorError {
        match self.unit(names) {
            Err(ProviderCompileError::Source(error)) => error,
            Err(error) => panic!("not a preprocessor refusal: {error}"),
            Ok(_) => panic!("prepared"),
        }
    }

    /// `top`, compiled from `names` as one unit.
    fn compiles(&self, names: &[&str], top: &str) {
        let prepared = self.unit(names).unwrap_or_else(|error| panic!("{error}"));
        if let Err(error) = prepared.compile_runtime(Some(top)) {
            panic!("{error}: {:?}", prepared.diagnostics_for_error(&error));
        }
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

fn file_of(diagnostic: &SourceCompileDiagnostic) -> &str {
    diagnostic
        .path
        .as_deref()
        .and_then(|path| path.rsplit(['/', '\\']).next())
        .unwrap_or_default()
}

/// The words of Verilog-AMS are names in SystemVerilog, so an `analog` block
/// in a `.sv` file is refused as what it is, on the line it is written on.
#[test]
fn analog_content_in_a_systemverilog_file_is_refused_by_name() {
    let files = Sources::new();
    files.write(
        "m.sv",
        "// header\nmodule m;\n  analog begin\n  end\nendmodule\n",
    );
    let refusal = files.refusal(&["m.sv"], "m");
    assert_eq!(refusal.code, "VA-SV-ANALOG", "{refusal:?}");
    assert!(
        refusal.message.contains("module `m`") && refusal.message.contains(".vams"),
        "{}",
        refusal.message
    );
    assert_eq!(
        (file_of(&refusal), refusal.line, refusal.column),
        ("m.sv", Some(3), Some(3))
    );

    // The wrapper that selects the set sits on the file's first line, which
    // keeps both its line and its columns.
    files.write("one.sv", "module one; analog begin end endmodule\n");
    let refusal = files.refusal(&["one.sv"], "one");
    assert_eq!(refusal.code, "VA-SV-ANALOG");
    assert_eq!(
        (file_of(&refusal), refusal.line, refusal.column),
        ("one.sv", Some(1), Some(13))
    );
}

/// `` `begin_keywords "1800-2017" `` selects SystemVerilog in a file of any
/// name, and `` `end_keywords `` returns to Verilog-AMS for what follows.
#[test]
fn begin_keywords_selects_systemverilog_in_a_verilog_file() {
    let files = Sources::new();
    files.write(
        "m.v",
        "`begin_keywords \"1800-2017\"\nmodule sv_side;\n  analog begin end\nendmodule\n\
         `end_keywords\n",
    );
    let refusal = files.refusal(&["m.v"], "sv_side");
    assert_eq!(refusal.code, "VA-SV-ANALOG");
    assert_eq!((file_of(&refusal), refusal.line), ("m.v", Some(3)));

    files.write(
        "both.vams",
        "`include \"disciplines.vams\"\n`begin_keywords \"1800-2017\"\nmodule sv_side;\n\
         endmodule\n`end_keywords\nmodule ams(p);\n  inout p;\n  electrical p;\n  analog \
         V(p) <+ 0.0;\nendmodule\n",
    );
    files.compiles(&["both.vams"], "ams");
}

/// The set a file selects, or its directives leave, ends with the file: an
/// unbalanced `` `begin_keywords `` in one file of a unit does not leave the
/// next file in SystemVerilog.
#[test]
fn an_unclosed_keyword_region_ends_with_its_file() {
    let files = Sources::new();
    files.write(
        "open.v",
        "`begin_keywords \"1800-2017\"\nmodule open_sv;\nendmodule\n",
    );
    files.write(
        "ams.vams",
        "`include \"disciplines.vams\"\nmodule ams(p);\n  inout p;\n  electrical p;\n  analog \
         V(p) <+ 0.0;\nendmodule\n",
    );
    files.compiles(&["open.v", "ams.vams"], "ams");
    files.write("first.sv", "module first;\nendmodule\n");
    files.compiles(&["first.sv", "ams.vams"], "ams");
}

/// IEEE 1800-2017 section 22.14: the keyword set changes only between design
/// elements. A SystemVerilog file included inside a Verilog-AMS module is
/// refused, naming the file; the same file included between modules, or
/// inside a SystemVerilog module, changes nothing it may not.
#[test]
fn a_systemverilog_include_inside_a_verilog_ams_module_is_refused_naming_it() {
    let files = Sources::new();
    files.write("decls.svh", "  real r;\n");
    files.write(
        "top.vams",
        "module top;\n`include \"decls.svh\"\nendmodule\n",
    );
    let refusal = files.refusal(&["top.vams"], "top");
    assert_eq!(
        refusal.code, "VA-LEX-KEYWORDS-INSIDE-ELEMENT",
        "{refusal:?}"
    );
    assert!(
        refusal.message.contains("decls.svh") && refusal.message.contains("module `top`"),
        "{}",
        refusal.message
    );

    files.write(
        "sv_top.sv",
        "module sv_top;\n`include \"decls.svh\"\nendmodule\n",
    );
    files.compiles(&["sv_top.sv"], "sv_top");

    files.write(
        "inside.v",
        "module inside;\n`begin_keywords \"1800-2017\"\n`end_keywords\nendmodule\n",
    );
    let refusal = files.refusal(&["inside.v"], "inside");
    assert_eq!(refusal.code, "VA-LEX-KEYWORDS-INSIDE-ELEMENT");
    assert_eq!((file_of(&refusal), refusal.line), ("inside.v", Some(2)));
}

/// The keyword directives are read, not ignored: a version this compiler
/// does not carry, a specifier no standard defines and an `` `end_keywords ``
/// with nothing to close are refused, and the 1364 ones warn no more.
#[test]
fn keyword_directives_are_checked_and_no_longer_warn() {
    let files = Sources::new();
    files.write(
        "v2012.v",
        "`begin_keywords \"1800-2012\"\nmodule m;\nendmodule\n",
    );
    let error = files.preprocessor_refusal(&["v2012.v"]);
    assert!(error.message.contains("1800-2012"), "{}", error.message);
    assert_eq!(error.line, 1);
    files.write(
        "unknown.v",
        "`begin_keywords \"1364-2008\"\nmodule m;\nendmodule\n",
    );
    assert!(
        files
            .preprocessor_refusal(&["unknown.v"])
            .message
            .contains("names no keyword set")
    );
    files.write("stray.v", "module m;\nendmodule\n`end_keywords\n");
    assert_eq!(files.preprocessor_refusal(&["stray.v"]).line, 3);

    files.write(
        "classic.v",
        "`begin_keywords \"1364-2005\"\nmodule classic;\n  reg r;\n  initial r = 1'b0;\n\
         endmodule\n`end_keywords\n",
    );
    let prepared = files.unit(&["classic.v"]).unwrap();
    let warned: Vec<_> = prepared
        .diagnostics()
        .iter()
        .filter(|diagnostic| diagnostic.code == "VA-PARSE-IGNORED-DIRECTIVE")
        .collect();
    assert!(warned.is_empty(), "{warned:?}");
    prepared.compile_runtime(Some("classic")).unwrap();
}

/// The discipline header is Verilog-AMS, so a SystemVerilog file that
/// includes it is told so instead of misparsing it.
#[test]
fn the_discipline_header_is_refused_in_systemverilog_text() {
    let files = Sources::new();
    files.write(
        "m.sv",
        "`include \"disciplines.vams\"\nmodule m;\nendmodule\n",
    );
    let error = files.preprocessor_refusal(&["m.sv"]);
    assert!(
        error.message.contains("disciplines.vams") && error.message.contains(".vams"),
        "{}",
        error.message
    );
    assert_eq!(error.line, 1);
}

/// `` `begin_keywords "1800-2017" `` in a `.v` file makes `package`, `import`
/// and `nettype` the keywords they are; without it they are names, and a
/// package is no declaration at all.
#[test]
fn begin_keywords_in_a_verilog_file_makes_package_parse() {
    let files = Sources::new();
    let body = "package p;\n  nettype real rnet;\n  localparam real K = 1.5;\nendpackage\nmodule m;\n  \
                import p::*;\n  rnet n;\n  assign n = K;\nendmodule\n";
    files.write(
        "selected.v",
        &format!("`begin_keywords \"1800-2017\"\n{body}`end_keywords\n"),
    );
    files.compiles(&["selected.v"], "m");
    files.write("plain.v", body);
    let refusal = files.refusal(&["plain.v"], "m");
    assert_eq!((file_of(&refusal), refusal.line), ("plain.v", Some(1)));
}

/// Nettype nets and ports, ANSI and not (IEEE 1800-2017 sections 6.6.7 and
/// 23.2.2), through a package, a scoped name and an alias.
#[test]
fn nettype_nets_and_ports_compile() {
    let files = Sources::new();
    files.write("pk.sv", "package pk;\n  nettype real rnet;\nendpackage\n");
    files.write(
        "ansi.sv",
        "module ansi import pk::*; (input rnet x, output pk::rnet y);\n  assign y = 2.0 * x;\n\
         endmodule\n",
    );
    files.compiles(&["pk.sv", "ansi.sv"], "ansi");
    files.write(
        "plain.sv",
        "module plain(x, y);\n  import pk::*;\n  nettype rnet alias_net;\n  input rnet x;\n  \
         output alias_net y;\n  assign y = x;\nendmodule\n",
    );
    files.compiles(&["plain.sv", "pk.sv"], "plain");
}

/// What the subset does not read yet is refused by name: a resolution
/// function, struct and integral nettype data, package variables and
/// `export`; and every other SystemVerilog word with one refusal that names
/// it.
#[test]
fn constructs_outside_the_subset_are_refused_by_name() {
    let files = Sources::new();
    let refusal = |name: &str, source: &str| {
        files.write(name, source);
        files.refusal(&[name], "m")
    };
    let with = refusal(
        "with.sv",
        "module m;\n  nettype real r with resolve;\nendmodule\n",
    );
    assert_eq!(with.code, "VA-PARSE-SV-UNSUPPORTED");
    assert!(
        with.message.contains("resolution functions"),
        "{}",
        with.message
    );
    assert_eq!(with.line, Some(2));
    let structured = refusal(
        "struct.sv",
        "module m;\n  nettype struct { real v; } s;\nendmodule\n",
    );
    assert!(
        structured.message.contains("struct data"),
        "{}",
        structured.message
    );
    let integral = refusal("logic.sv", "module m;\n  nettype logic l;\nendmodule\n");
    assert!(
        integral.message.contains("integral data"),
        "{}",
        integral.message
    );
    let variable = refusal(
        "variable.sv",
        "package p;\n  real v;\nendpackage\nmodule m;\nendmodule\n",
    );
    assert!(
        variable.message.contains("package variables"),
        "{}",
        variable.message
    );
    let export = refusal(
        "export.sv",
        "package p;\n  export q::*;\nendpackage\nmodule m;\nendmodule\n",
    );
    assert!(export.message.contains("`export`"), "{}", export.message);
    let outside = refusal("outside.sv", "module m;\n  logic l;\nendmodule\n");
    assert_eq!(outside.code, "VA-PARSE-SV-OUTSIDE-SUBSET");
    assert!(
        outside.message.contains("`logic`")
            && outside.message.contains("outside the RSpice SV-RNM subset"),
        "{}",
        outside.message
    );
    assert_eq!((outside.line, outside.column), (Some(2), Some(3)));
}

/// A Verilog-AMS discipline is analog content wherever it types a net of a
/// SystemVerilog module, and `wreal` is told it is Verilog-AMS's.
#[test]
fn disciplines_and_wreal_are_not_nettypes() {
    let files = Sources::new();
    files.write(
        "disc.sv",
        "module m(p);\n  inout p;\n  electrical p;\nendmodule\n",
    );
    let refusal = files.refusal(&["disc.sv"], "m");
    assert_eq!(refusal.code, "VA-SV-ANALOG", "{refusal:?}");
    assert_eq!(refusal.line, Some(3));
    files.write("wreal.sv", "module m;\n  wreal w;\nendmodule\n");
    let refusal = files.refusal(&["wreal.sv"], "m");
    assert_eq!(refusal.code, "VA-SEM-SV-NETTYPE");
    assert!(
        refusal.message.contains("Verilog-AMS net type"),
        "{}",
        refusal.message
    );
    files.write("unknown.sv", "module m;\n  missing_type w;\nendmodule\n");
    let refusal = files.refusal(&["unknown.sv"], "m");
    assert!(
        refusal.message.contains("no nettype `missing_type`"),
        "{}",
        refusal.message
    );
}

/// IEEE 1800-2017 section 26.2: a package name is declared once in the
/// compilation unit, and the refusal names both files that declare it.
#[test]
fn a_duplicate_package_names_both_files() {
    let files = Sources::new();
    files.write(
        "one.sv",
        "package p;\n  localparam real K = 1.0;\nendpackage\n",
    );
    files.write(
        "two.sv",
        "// again\npackage p;\n  localparam real K = 2.0;\nendpackage\n",
    );
    files.write("m.sv", "module m;\nendmodule\n");
    let refusal = files.refusal(&["one.sv", "two.sv", "m.sv"], "m");
    assert_eq!(refusal.code, "VA-UNIT-DUPLICATE-PACKAGE");
    assert!(
        refusal.message.contains("one.sv") && refusal.message.contains("two.sv"),
        "{}",
        refusal.message
    );
    assert_eq!((file_of(&refusal), refusal.line), ("two.sv", Some(2)));
}
