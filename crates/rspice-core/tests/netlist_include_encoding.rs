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
