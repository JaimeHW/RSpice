//! The vendor corpus exercises production import and preparation boundaries.

use super::*;
use crate::workbench::documents::netlist_document::NetlistImportIssueSeverity;
use crate::workbench::workflows::netlist_workflow::fixtures::vendor_profiles::{cases, source};

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
