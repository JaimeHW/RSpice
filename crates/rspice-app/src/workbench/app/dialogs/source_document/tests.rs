//! Source document dialog tests: include search roots, role labels, the lifecycle
//! transactions, and the creation form a project with no deck opens.

use super::*;
use crate::state::{ProjectDescriptor, ProjectSourceOwner};
use crate::workbench::code::{CodeFeature, tests::harness};
use crate::workbench::documents::code_workspace::CodeWorkspacePage;
use rspice_simulation::netlist_preparation::IncludeSearchChain;
use std::path::{Path, PathBuf};

#[test]
fn a_project_places_its_relative_entries_against_its_own_folder() {
    let mut project = ProjectDescriptor::default();
    project.path = Some(PathBuf::from("/projects/mixer/mixer.rspiceproj"));
    project
        .set_include_search_paths(vec![PathBuf::from("models"), PathBuf::from("/opt/pdk/lib")])
        .expect("chain is accepted");

    let directories = rspice_simulation::netlist_preparation::IncludeSearchChain::resolve(
        project.include_search_paths(),
        project.data_root(),
    )
    .directories();
    assert_eq!(directories.len(), 2);
    assert_eq!(directories[0], Path::new("/projects/mixer").join("models"));
    assert_eq!(directories[1], PathBuf::from("/opt/pdk/lib"));
}

#[test]
fn relative_entries_resolve_against_the_project_data_root() {
    let root = crate::fixture_root::canonical_temp_dir().join(format!(
        "rspice-include-chain-relative-{}",
        std::process::id()
    ));
    let models = root.join("models");
    std::fs::create_dir_all(&models).expect("create chain fixture");

    let chain = IncludeSearchChain::resolve(
        &[PathBuf::from("models"), PathBuf::from("absent")],
        Some(root.as_path()),
    );

    assert_eq!(chain.entries().len(), 2);
    assert_eq!(chain.entries()[0].resolved(), models.as_path());
    assert!(chain.entries()[0].exists());
    assert_eq!(chain.entries()[1].resolved(), root.join("absent"));
    assert!(!chain.entries()[1].exists());
    std::fs::remove_dir_all(&root).ok();
}

/// The choice of engine entry point lives here and nowhere else, so it is
/// asserted by its effect: the same deck refuses to resolve without a
/// chain and resolves through it with one.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_chain_decides_which_engine_entry_point_a_host_parse_takes() {
    let root = crate::fixture_root::canonical_temp_dir().join(format!(
        "rspice-include-chain-entry-point-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos()
    ));
    let deck_dir = root.join("deck");
    let library_dir = root.join("pdk");
    std::fs::create_dir_all(&deck_dir).expect("create deck directory");
    std::fs::create_dir_all(&library_dir).expect("create library directory");
    std::fs::write(
        library_dir.join("device.lib"),
        ".model DCHAIN D(Is=1e-14)\n",
    )
    .expect("write library");
    let deck_path = deck_dir.join("top.cir");
    let source = "chain entry point\n.include device.lib\nD1 out 0 DCHAIN\n.end\n";
    let options = rspice_core::netlist::NetlistParseOptions::default();

    let without = IncludeSearchChain::default().parse_with_abort(
        source,
        Some(&deck_path),
        options,
        &rspice_core::abort_signal::NoAbort,
    );
    assert!(
        without.is_err(),
        "without a chain the include is nowhere the resolver looks"
    );

    let chain = IncludeSearchChain::resolve(&[library_dir.clone()], Some(root.as_path()));
    let with = chain
        .parse_with_abort(
            source,
            Some(&deck_path),
            options,
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("the chain resolves the include");
    assert_eq!(with.source_path.as_deref(), Some(deck_path.as_path()));

    std::fs::remove_dir_all(&root).ok();
}

/// Line endings are reported from the retained bytes. A whole-file rewrite
/// normalizes a mixed document, so the dialog has to be able to say so.
fn english() -> MessageCatalog {
    MessageCatalog::new(crate::workbench::UiTextLocale::EnglishUnitedStates)
}

pub(crate) fn facts_of(
    workspace: Workspace,
    model: &Model,
    code: &CodeWorkspaceRuntimeState,
    platform: &PlatformState,
) -> Option<DocumentFacts> {
    document_facts(workspace, model, code, platform, english())
}

#[test]
fn line_endings_are_measured_rather_than_assumed() {
    let english = english();
    let summary = |content| english.text(line_ending_summary(content));
    assert_eq!(summary("a\nb\n"), "LF");
    assert_eq!(summary("a\r\nb\r\n"), "CRLF");
    assert_eq!(summary("a\r\nb\n"), "mixed LF and CRLF");
    assert_eq!(summary("single"), "none \u{00b7} single line");
}

/// The role picker resolves a selection by matching the label it drew, so
/// every assignable role has to round-trip through its own text in every
/// locale — including the one whose expansion rewrites every vowel.
#[test]
fn every_assignable_role_round_trips_through_its_label_in_every_locale() {
    for locale in crate::workbench::UiTextLocale::ALL {
        let messages = MessageCatalog::new(locale);
        let mut drawn = std::collections::BTreeSet::new();
        for role in ASSIGNABLE_AUTOMATION_ROLES {
            let label = role_label(messages, role);
            assert!(
                drawn.insert(label.clone()),
                "two roles share the label {label:?} in {locale:?}"
            );
            assert_eq!(
                ASSIGNABLE_AUTOMATION_ROLES
                    .iter()
                    .find(|candidate| role_label(messages, **candidate) == label)
                    .copied(),
                Some(role)
            );
        }
    }
}

/// The environment lock refuses graph mutation and its refusal tells the
/// user to reassign the role first. That instruction was impossible to
/// follow, because no control could set a role. Assert the refusal, the
/// reassignment, and that the refusal then lifts.
#[test]
fn reassigning_a_role_lifts_the_environment_lock_refusal() {
    let mut harness = harness();
    harness.state.code_workspace.page = CodeWorkspacePage::Automation;
    let language = ProjectSourceLanguage::RSpiceAutomation;
    let owner = ProjectSourceOwner::code_workspace(language);
    let locked = harness
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&owner)
        .and_then(|bundle| {
            bundle
                .paths_for_role(crate::state::ProjectSourceRole::AutomationEnvironmentLock)
                .next()
                .map(str::to_owned)
        })
        .unwrap_or_default();
    assert!(!locked.is_empty(), "the fixture bundle pins an environment");
    harness.state.code_workspace.automation.selected_file = Some(locked.clone());

    let before = facts_of(
        harness.wb.workspace,
        &harness.model,
        &harness.state.code_workspace,
        &harness.platform,
    )
    .expect("the lock is a document");
    assert!(before.is_environment_lock);
    assert!(
        refusal_reason(&before, CodeSourceFileAction::Rename).is_some(),
        "the lock refuses graph mutation"
    );

    apply_lifecycle_action(
        &mut CodeFeature::workspace_parts(&mut harness.cx()),
        LifecycleAction::AssignRole(None),
        before.language,
        &before.logical_path,
    );

    let after = facts_of(
        harness.wb.workspace,
        &harness.model,
        &harness.state.code_workspace,
        &harness.platform,
    )
    .expect("the document survives the role change");
    assert!(!after.is_environment_lock, "the role was reassigned");
    assert!(
        refusal_reason(&after, CodeSourceFileAction::Rename).is_none(),
        "the refusal the service states is now followable"
    );
}
