//! The published exit-code table is derived from the code, not maintained
//! beside it.
//!
//! `README.md` documents an exit status per failure category, and automation
//! branches on those numbers. A table that drifts from [`exit_code_for`] is
//! worse than no table: a script written against it silently stops matching
//! the process it is watching. This test reconstructs the mapping from the one
//! source of truth — every [`FailureCategory`], including every engine
//! category the core declares — and requires the published table to be exactly
//! that mapping: same codes, same category spellings, nothing extra on either
//! side.
//!
//! `tests/exit_codes.rs` proves the codes are *reachable* by driving real
//! decks through the binary. This proves they are *documented*.

use std::collections::{BTreeMap, BTreeSet};

use rspice_core::SimulationErrorCategory;

use super::error::{ExitCode, FailureCategory, exit_code_for};

/// The published table. Documentation, not source: the engineering rule bars
/// asserting on production `.rs` text, and the whole point here is to check a
/// document against behaviour.
const README: &str = include_str!("../../README.md");

/// Every category the CLI can report, engine and frontend alike.
fn every_category() -> Vec<FailureCategory> {
    let mut categories: Vec<FailureCategory> = SimulationErrorCategory::ALL
        .iter()
        .map(|category| FailureCategory::Engine(*category))
        .collect();
    categories.extend([
        FailureCategory::InputNotFound,
        FailureCategory::Io,
        FailureCategory::Usage,
        FailureCategory::Verification,
        FailureCategory::Internal,
        FailureCategory::Compilation,
        FailureCategory::Conversion,
    ]);
    categories
}

/// Code to the set of category spellings that produce it.
fn mapping_from_code() -> BTreeMap<u8, BTreeSet<String>> {
    let mut mapping = BTreeMap::<u8, BTreeSet<String>>::new();
    for category in every_category() {
        mapping
            .entry(exit_code_for(category) as u8)
            .or_default()
            .insert(category.as_str().to_owned());
    }
    // Success is documented too, and no failure category produces it.
    mapping.entry(ExitCode::Success as u8).or_default();
    mapping
}

/// Parse the `## Exit Codes` table out of the README.
fn documented_table() -> BTreeMap<u8, BTreeSet<String>> {
    let section = README
        .split_once("## Exit Codes")
        .expect("README documents an exit-code section")
        .1;
    let section = section
        .split_once("\n## ")
        .map_or(section, |(before, _)| before);

    let mut documented = BTreeMap::<u8, BTreeSet<String>>::new();
    for line in section.lines() {
        let line = line.trim();
        let Some(row) = line.strip_prefix('|').and_then(|row| row.strip_suffix('|')) else {
            continue;
        };
        let cells = row.split('|').map(str::trim).collect::<Vec<_>>();
        if cells.len() != 3 {
            continue;
        }
        // The header row and its separator carry no numeric code.
        let Ok(code) = cells[0].parse::<u8>() else {
            continue;
        };
        let categories = if cells[1] == "—" {
            BTreeSet::new()
        } else {
            cells[1]
                .split(',')
                .map(|category| category.trim().trim_matches('`').to_owned())
                .filter(|category| !category.is_empty())
                .collect()
        };
        assert!(
            !cells[2].is_empty(),
            "exit code {code} is documented without a meaning"
        );
        assert!(
            documented.insert(code, categories).is_none(),
            "exit code {code} is documented twice"
        );
    }
    documented
}

#[test]
fn the_published_exit_code_table_is_exactly_the_category_mapping() {
    let expected = mapping_from_code();
    let documented = documented_table();

    let expected_codes = expected.keys().copied().collect::<BTreeSet<_>>();
    let documented_codes = documented.keys().copied().collect::<BTreeSet<_>>();
    assert_eq!(
        documented_codes,
        expected_codes,
        "the README's exit codes and the codes `exit_code_for` produces disagree; \
         missing from the README: {:?}; documented but unreachable: {:?}",
        expected_codes
            .difference(&documented_codes)
            .collect::<Vec<_>>(),
        documented_codes
            .difference(&expected_codes)
            .collect::<Vec<_>>(),
    );

    for (code, categories) in &expected {
        assert_eq!(
            documented.get(code),
            Some(categories),
            "exit code {code} documents {:?} but is produced by {categories:?}",
            documented.get(code)
        );
    }
}

#[test]
fn the_table_documents_every_engine_category_by_its_stable_spelling() {
    let documented = documented_table();
    let spellings = documented
        .values()
        .flatten()
        .cloned()
        .collect::<BTreeSet<_>>();
    for category in SimulationErrorCategory::ALL {
        assert!(
            spellings.contains(category.as_str()),
            "engine category '{}' has an exit code but no documented row; automation \
             branching on the published table would not know what it means",
            category.as_str()
        );
    }
    // Nothing in the table may be a spelling the CLI never emits.
    let emitted = every_category()
        .into_iter()
        .map(|category| category.as_str().to_owned())
        .collect::<BTreeSet<_>>();
    assert!(
        spellings.is_subset(&emitted),
        "the README documents categories the CLI never reports: {:?}",
        spellings.difference(&emitted).collect::<Vec<_>>()
    );
}
