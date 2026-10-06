use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use rspice_core::Netlist;

struct TempDeckDir(PathBuf);

impl TempDeckDir {
    fn new(test_name: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock is after UNIX epoch")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "rspice_netlist_include_encoding_{}_{}_{}",
            test_name,
            std::process::id(),
            nonce
        ));
        fs::create_dir_all(&dir).expect("create temp deck dir");
        Self(dir)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDeckDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn write(path: &Path, bytes: impl AsRef<[u8]>) {
    fs::write(path, bytes).unwrap_or_else(|err| panic!("write {}: {err}", path.display()));
}

fn assert_model_exists(netlist: &Netlist, name: &str) {
    assert!(
        netlist
            .models
            .iter()
            .any(|model| model.name.eq_ignore_ascii_case(name)),
        "expected model `{name}` in {:?}",
        netlist
            .models
            .iter()
            .map(|model| model.name.as_str())
            .collect::<Vec<_>>()
    );
}

fn utf16le_with_bom(text: &str) -> Vec<u8> {
    let mut bytes = vec![0xFF, 0xFE];
    for unit in text.encode_utf16() {
        bytes.extend(unit.to_le_bytes());
    }
    bytes
}

#[test]
fn parsed_include_inventory_retains_empty_nested_and_selected_library_sources() {
    let dir = TempDeckDir::new("source_inventory");
    let deck = dir.path().join("top.cir");
    let library = dir.path().join("corners.lib");
    let empty = dir.path().join("empty.inc");
    let nested = dir.path().join("nested.inc");
    write(&empty, "");
    write(&nested, ".include empty.inc\n");
    write(
        &library,
        ".lib tt\n.include nested.inc\n.param resistance=1000\n.endl tt\n",
    );
    write(
        &deck,
        "inventory\n.lib corners.lib tt\n.include empty.inc\nV1 1 0 1\nR1 1 0 {resistance}\n.end\n",
    );
    let mut expected = [
        library.canonicalize().unwrap(),
        empty.canonicalize().unwrap(),
        nested.canonicalize().unwrap(),
    ];
    expected.sort();
    for netlist in [
        Netlist::parse_file(&deck).unwrap(),
        Netlist::parse_file_with_search_paths(&deck, &[dir.path().to_path_buf()]).unwrap(),
    ] {
        assert_eq!(netlist.included_source_paths(), expected);
        assert_eq!(netlist.clone().included_source_paths(), expected);
    }
    assert!(
        Netlist::parse("memory\nR1 1 0 1k\n.end\n")
            .unwrap()
            .included_source_paths()
            .is_empty()
    );
}

#[test]
fn sealed_include_inventory_does_not_require_filesystem_members() {
    use rspice_core::netlist::{NetlistParseOptions, SealedSourceBundle, SealedSourceEdge};
    let dir = TempDeckDir::new("sealed_inventory");
    let deck = dir.path().join("missing.cir");
    let include = dir.path().join("missing.inc");
    let source = "sealed\n.include missing.inc\nR1 1 0 1k\n.end\n";
    let bundle = SealedSourceBundle::try_new_with_edges(
        [(deck.clone(), source.into()), (include.clone(), "".into())],
        [SealedSourceEdge {
            owner: deck.clone(),
            requested_path: "missing.inc".into(),
            target: include.clone(),
        }],
    )
    .unwrap();
    let netlist = Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
        source,
        &deck,
        bundle,
        NetlistParseOptions::default(),
        &rspice_core::NoAbort,
    )
    .unwrap();
    assert_eq!(netlist.included_source_paths(), [include]);
    assert!(!deck.exists());
}

#[test]
fn include_expansion_strips_utf8_bom_like_top_level_parse_file() {
    let dir = TempDeckDir::new("utf8_bom_include");
    let deck = dir.path().join("top.cir");
    let include = dir.path().join("diode.inc");

    write(
        &deck,
        "include utf8 bom\n.include \"diode.inc\"\nD1 in 0 dbom\nV1 in 0 1\n.op\n.end\n",
    );
    write(&include, b"\xEF\xBB\xBF.model dbom d is=1e-12\n");

    let netlist = Netlist::parse_file(&deck).expect("deck parses with UTF-8 BOM include");

    assert_model_exists(&netlist, "dbom");
}

#[test]
fn include_expansion_decodes_utf16le_bom_like_top_level_parse_file() {
    let dir = TempDeckDir::new("utf16le_bom_include");
    let deck = dir.path().join("top.cir");
    let include = dir.path().join("diode.inc");

    write(
        &deck,
        "include utf16le bom\n.include \"diode.inc\"\nD1 in 0 dutf16\nV1 in 0 1\n.op\n.end\n",
    );
    write(&include, utf16le_with_bom(".model dutf16 d is=2e-12\n"));

    let netlist = Netlist::parse_file(&deck).expect("deck parses with UTF-16 LE BOM include");

    assert_model_exists(&netlist, "dutf16");
}

#[test]
fn lib_expansion_uses_latin1_fallback_like_top_level_parse_file() {
    let dir = TempDeckDir::new("latin1_lib");
    let deck = dir.path().join("top.cir");
    let lib = dir.path().join("models.lib");

    write(
        &deck,
        "include latin1 lib\n.lib \"models.lib\" TT\nD1 in 0 dlatin\nV1 in 0 1\n.op\n.end\n",
    );
    write(
        &lib,
        b".lib TT\n* vendor caf\xE9 comment\n.model dlatin d is=3e-12\n.endl TT\n",
    );

    let netlist = Netlist::parse_file(&deck).expect("deck parses with Latin-1 .lib section");

    assert_model_exists(&netlist, "dlatin");
}
