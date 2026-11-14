//! The vendor corpus: each reviewed deck's admission and preparation
//! outcome under its vendor's execution profile, and the deck's source.

use crate::state::NetlistExecutionProfile;
use crate::workbench::workflows::netlist_workflow::bundle::decode_import_bytes;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

const CORPUS: &str = include_str!("testdata/vendor_profile_corpus_v1.tsv");

pub fn cases() -> Vec<(&'static str, bool, &'static str, NetlistExecutionProfile)> {
    let mut seen = BTreeSet::new();
    let cases = CORPUS
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let fields = line.split('\t').collect::<Vec<_>>();
            let [path, admission, preparation] = fields.as_slice() else {
                panic!("expected path, admission, and preparation: {line}");
            };
            assert!(seen.insert(*path), "duplicate corpus path: {path}");
            let accepted = match *admission {
                "accepted" => true,
                "blocked" => false,
                _ => panic!("unknown admission: {line}"),
            };
            assert_eq!(
                accepted,
                *preparation != "-",
                "only admitted decks can prepare: {line}"
            );
            assert!(
                !preparation.is_empty(),
                "missing preparation outcome: {line}"
            );
            let profile = match path.split('/').next().unwrap() {
                "HSPICE" => NetlistExecutionProfile::HspiceDeclarativeV1,
                "PSPICE" => NetlistExecutionProfile::PspiceDeclarativeV2,
                "SPECTRE" => NetlistExecutionProfile::SpectreSpiceV1,
                vendor => panic!("unmapped vendor {vendor}"),
            };
            (*path, accepted, *preparation, profile)
        })
        .collect::<Vec<_>>();
    assert_eq!(
        cases.len(),
        97,
        "review the inventory when adding or removing decks"
    );
    cases
}

pub fn source(path: &str) -> (String, PathBuf) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/xyce/Netlists/XDM")
        .join(path);
    let bytes = std::fs::read(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    (decode_import_bytes(&bytes).unwrap().0, path)
}
