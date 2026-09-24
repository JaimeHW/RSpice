//! The vendor corpus exercises production import and preparation boundaries.

use super::*;
use crate::state::NetlistExecutionProfile;
use crate::workbench::documents::netlist_document::NetlistImportIssueSeverity;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

const CORPUS: &str =
    include_str!("../../../../state/workspace/testdata/vendor_profile_corpus_v1.tsv");

fn cases() -> Vec<(&'static str, bool, &'static str, NetlistExecutionProfile)> {
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

fn source(path: &str) -> (String, PathBuf) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/xyce/Netlists/XDM")
        .join(path);
    let bytes = std::fs::read(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    (decode_import_bytes(&bytes).unwrap().0, path)
}

#[test]
fn vendor_profile_corpus_preserves_reviewed_admission_boundaries() {
    let mut failures = Vec::new();
    for (relative, expected, _, profile) in cases() {
        let (source, path) = source(relative);
        let issues = validate_import_candidate(&source, Some(&path), Some(profile));
        let accepted = !issues
            .iter()
            .any(|issue| issue.severity == NetlistImportIssueSeverity::Blocking);
        if accepted != expected {
            failures.push(format!(
                "{relative}: expected import acceptance={expected}, got {accepted}: {issues:?}"
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn vendor_admitted_decks_have_explicit_preparation_outcomes() {
    let mut failures = Vec::new();
    for (relative, accepted, expected, profile) in cases() {
        if !accepted {
            continue;
        }
        let (source, path) = source(relative);
        let mut app = RSpiceApp::test_instance();
        assert!(apply_imported_netlist(
            &mut app.state,
            source.clone(),
            Some(path),
            relative
        ));
        let descriptor = app.state.workspace.netlist_descriptor.as_mut().unwrap();
        descriptor.imported_dialect = Some(profile.source_dialect());
        descriptor.execution_profile = Some(profile);
        descriptor.compatibility_reviewed = true;
        let ready = validate_visible_netlist_source(&mut app);
        let error = app
            .state
            .ui
            .netlist
            .validation_error
            .as_deref()
            .unwrap_or_default();
        if (expected == "ready" && !ready)
            || (expected != "ready" && (ready || !error.contains(expected)))
        {
            failures.push(format!(
                "{relative}: expected preparation {expected:?}, got ready={ready}, {error}"
            ));
        }
        assert_eq!(
            app.state.workspace.netlist_source.as_deref(),
            Some(source.as_str())
        );
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
