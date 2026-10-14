//! What the navigator reads from a Verilog-A file: its outline, as the
//! compiler's parser sees it, and its includes, resolved where the
//! compile resolves them.
//!
//! The include cases are checked against a real compile of the same
//! closure, so the panel and the compiler cannot drift apart unnoticed.

use super::*;
use crate::state::{CellViewRef, ProjectSourceDependency, ProjectSourceFile, ProjectSourceOwner};

/// A cell view's bundle rooted at `root_path`, with `files` beside it.
fn bundle(root_path: &str, root: &str, files: &[(&str, &str)]) -> ProjectSourceBundle {
    let files = files
        .iter()
        .map(|(path, source)| ProjectSourceFile::try_new(*path, *source).unwrap())
        .collect::<Vec<_>>();
    let dependencies = files
        .iter()
        .map(|file| ProjectSourceDependency::try_new(root_path, file.logical_path()).unwrap())
        .collect::<Vec<_>>();
    ProjectSourceBundle::try_new(
        ProjectSourceOwner::cell_view(CellViewRef::new("behavioral", "gain", "veriloga")),
        ProjectSourceLanguage::VerilogA,
        root_path,
        root,
        files,
        dependencies,
    )
    .unwrap()
}

fn navigation(bundle: &ProjectSourceBundle, path: &str) -> SourceNavigation {
    document_navigation(ProjectId::new(), bundle, &mut None, path)
}

fn row<'a>(outline: &'a [SourceOutlineRow], name: &str) -> &'a SourceOutlineRow {
    outline
        .iter()
        .find(|row| row.name == name)
        .unwrap_or_else(|| panic!("no outline row {name}: {outline:#?}"))
}

const GAIN: &str = "`include \"disciplines.vams\"\n\
module gain(p, n);\n\
  inout p, n;\n\
  electrical p, n;\n\
  electrical mid;\n\
  parameter real g = 1m from (0:inf);\n\
  parameter integer stages = 2;\n\
  real acc;\n\
  branch (p, mid) drop;\n\
  analog function real twice;\n\
    input x;\n\
    real x;\n\
    twice = 2 * x;\n\
  endfunction\n\
  analog I(p, n) <+ g * V(p, n);\n\
endmodule\n";

/// A port is declared twice — `inout p` gives its direction, `electrical p`
/// its discipline — and was listed once per declaration. It is one row that
/// states both, and every other kind states what its declaration says.
#[test]
fn a_port_is_one_row_stating_its_direction_and_discipline() {
    let bundle = bundle("models/gain.va", GAIN, &[]);
    let outline = navigation(&bundle, "models/gain.va").outline;

    let ports = outline
        .iter()
        .filter(|row| row.group == Some(SourceOutlineGroup::Ports))
        .map(|row| (row.name.as_str(), row.detail.as_str(), row.line))
        .collect::<Vec<_>>();
    assert_eq!(
        ports,
        [("p", "inout electrical", 3), ("n", "inout electrical", 3)]
    );
    assert_eq!(row(&outline, "g").detail, "real = 1m from (0:inf)");
    assert_eq!(row(&outline, "g").line, 6);
    assert_eq!(row(&outline, "stages").detail, "integer = 2");
    assert_eq!(row(&outline, "acc").detail, "real");
    assert_eq!(row(&outline, "mid").detail, "electrical");
    assert_eq!(row(&outline, "drop").detail, "(p, mid)");
    assert_eq!(row(&outline, "twice").detail, "real (x)");
    assert_eq!(
        row(&outline, "gain").group,
        Some(SourceOutlineGroup::Module)
    );

    let groups = outline.iter().map(|row| row.group).collect::<Vec<_>>();
    let mut ordered = groups.clone();
    ordered.sort();
    assert_eq!(groups, ordered, "rows come grouped, in the groups' order");
}

/// The parser reads the file unexpanded, so a macro is quoted as written and
/// a directive inside the module, or a definition continued over lines,
/// moves no row off its line.
#[test]
fn macros_and_directives_keep_the_parsed_outline() {
    let source = "`define G_DEFAULT \\\n  1m\nmodule gain(p, n);\n  inout p, n;\n`ifdef WIDE\n  electrical p, n;\n`else\n  electrical p, n;\n`endif\n  parameter real g = `G_DEFAULT * `M_PI from (0:inf);\n  analog I(p, n) <+ `MAX(g, 0) * V(p, n);\nendmodule\n";
    let bundle = bundle("gain.va", source, &[]);
    let outline = navigation(&bundle, "gain.va").outline;

    assert_eq!(row(&outline, "p").detail, "inout electrical");
    assert_eq!(
        row(&outline, "g").detail,
        "real = `G_DEFAULT * `M_PI from (0:inf)"
    );
    assert_eq!(row(&outline, "g").line, 10);
    assert_eq!(row(&outline, "gain").line, 3);
}

/// A file the parser cannot read — mid-edit, or leaning on a macro only the
/// preprocessor expands — keeps the language index's outline of its names
/// rather than going blank, and still lists a port once rather than again as
/// the net its discipline declaration names.
#[test]
fn an_unparsed_file_keeps_its_names() {
    let source = "module gain(p, n);\n  inout p, n;\n  electrical p, n;\n  parameter real g = `UNDEFINED;\n  analog I(p, n) <+ g * V(p, n)\nendmodule\n";
    let bundle = bundle("gain.va", source, &[]);
    let outline = navigation(&bundle, "gain.va").outline;

    assert_eq!(
        outline
            .iter()
            .map(|row| (row.group, row.name.as_str(), row.detail.as_str(), row.line))
            .collect::<Vec<_>>(),
        [
            (Some(SourceOutlineGroup::Module), "gain", "", 1),
            (Some(SourceOutlineGroup::Ports), "p", "", 2),
            (Some(SourceOutlineGroup::Parameters), "g", "", 4),
        ]
    );
}

const INCLUDING: &str = "`include \"disciplines.vams\"\n\
`include \"constants.vams\"\n\
`include \"lib/helpers.vams\"\n\
module gain(p, n);\n\
  inout p, n;\n\
  electrical p, n;\n\
  parameter real g = `GAIN * `UNIT from (0:inf);\n\
  analog I(p, n) <+ g * V(p, n);\n\
endmodule\n";

fn including_bundle() -> ProjectSourceBundle {
    bundle(
        "models/gain.va",
        INCLUDING,
        &[
            // A project file named like a standard header is the one an
            // include of that name gets.
            ("models/constants.vams", "`define GAIN 2\n"),
            (
                "models/lib/helpers.vams",
                "`include \"units.vams\"\n`include \"disciplines.vams\"\n",
            ),
            ("models/lib/units.vams", "`define UNIT 1\n"),
        ],
    )
}

/// Every include of the closure, each where the compiler resolves it: the
/// cell's own files first, nested includes too, and the built-in headers
/// only for a name the bundle does not hold.
#[test]
fn includes_resolve_project_first_then_built_in() {
    let includes = navigation(&including_bundle(), "models/gain.va").includes;

    assert_eq!(
        includes
            .iter()
            .map(|include| (include.requested.as_str(), include.target.clone()))
            .collect::<Vec<_>>(),
        [
            (
                "disciplines.vams",
                SourceIncludeTarget::BuiltIn("@rspice/stdlib/disciplines.vams")
            ),
            (
                "constants.vams",
                SourceIncludeTarget::Project("models/constants.vams".to_owned())
            ),
            (
                "lib/helpers.vams",
                SourceIncludeTarget::Project("models/lib/helpers.vams".to_owned())
            ),
            (
                "units.vams",
                SourceIncludeTarget::Project("models/lib/units.vams".to_owned())
            ),
        ],
        "helpers.vams's own include of disciplines.vams is the same header, listed once"
    );
}

/// The panel's resolution is the compiler's: the same closure, the same
/// documents, in the same order.
#[test]
fn includes_agree_with_the_compile() {
    let bundle = including_bundle();
    let compilation =
        super::super::veriloga::compile_project_bundle_virtual_for_provenance(&bundle, "gain")
            .expect("the closure compiles");
    let mut compiled = Vec::new();
    for edge in &compilation.include_graph {
        if !compiled.contains(&edge.included_path) {
            compiled.push(edge.included_path.clone());
        }
    }

    let listed = navigation(&bundle, "models/gain.va")
        .includes
        .into_iter()
        .map(|include| match include.target {
            SourceIncludeTarget::Project(path) => path,
            SourceIncludeTarget::BuiltIn(identity) => identity.to_owned(),
            SourceIncludeTarget::Missing { .. } => panic!("{include:?} resolved in the compile"),
        })
        .collect::<Vec<_>>();

    assert_eq!(listed, compiled);
}

/// A name nothing holds is listed at its directive's line, and listing it
/// does not stop the walk: the includes after it are still listed. A
/// directive in a comment is not one.
#[test]
fn a_missing_include_is_listed_at_its_line_and_the_walk_goes_on() {
    let source = "// `include \"commented.vams\"\n/* `include \"blocked.vams\" */\n`include \"missing.vams\"\n`include \"disciplines.vams\"\n`include \"absent/other.vams\"\nmodule gain;\nendmodule\n";
    let bundle = bundle("gain.va", source, &[]);
    let includes = navigation(&bundle, "gain.va").includes;

    assert_eq!(
        includes
            .iter()
            .map(|include| (include.requested.as_str(), include.target.clone()))
            .collect::<Vec<_>>(),
        [
            (
                "missing.vams",
                SourceIncludeTarget::Missing {
                    including: "gain.va".to_owned(),
                    line: 3,
                }
            ),
            (
                "disciplines.vams",
                SourceIncludeTarget::BuiltIn("@rspice/stdlib/disciplines.vams")
            ),
            (
                "absent/other.vams",
                SourceIncludeTarget::Missing {
                    including: "gain.va".to_owned(),
                    line: 5,
                }
            ),
        ]
    );
}
