//! Source-drift detection and the states it puts a library in.

use super::*;

use crate::product::{ContentDigest, ModelSourceId};
use crate::state::model_library::{ModelSourceContent, ModelSourcePin};
use crate::workbench::app_state::AppState;

fn digest(bytes: &[u8]) -> ContentDigest {
    use sha2::{Digest as _, Sha256};
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

/// A retained library whose pin names `pinned` and whose bytes are `held`.
fn retained(name: &str, pinned: &[u8], held: &[u8]) -> ModelLibrary {
    let mut library = ModelLibrary::new(name);
    let root = PathBuf::from(format!("/retained/{name}/models.lib"));
    library.root_path = Some(root.clone());
    library.source_authority = ModelSourceAuthority::RetainedImport {
        source_id: ModelSourceId::new(),
        digest: digest(pinned),
    };
    library.source_closure = vec![ModelSourcePin {
        path: root.clone(),
        digest: digest(pinned),
    }];
    library.source_contents = vec![ModelSourceContent {
        path: root,
        bytes: held.to_vec(),
    }];
    library
}

#[test]
fn a_retained_source_that_no_longer_hashes_to_its_pin_is_a_finding() {
    let mut state = AppState::default();
    state.model_library_manager.clear();
    state.model_library_manager.add_library(retained(
        "clean",
        b".model a nmos\n",
        b".model a nmos\n",
    ));
    state.model_library_manager.add_library(retained(
        "moved",
        b".model a nmos\n",
        b".model a pmos\n",
    ));

    assert!(
        needs_scan(&state),
        "nothing has been scanned, which is not the same as nothing being wrong"
    );
    scan(&mut state);
    assert!(
        !needs_scan(&state),
        "the report describes this project and this catalogue, so an idle \
         repaint rescans nothing"
    );

    assert!(
        findings_for(&state, "clean").is_empty(),
        "a source that still hashes to its pin says nothing"
    );
    let findings = findings_for(&state, "moved");
    assert_eq!(findings.len(), 1);
    let finding = &findings[0];
    assert_eq!(
        finding.pinned,
        short_digest(&digest(b".model a nmos\n").to_string())
    );
    assert_eq!(
        finding.on_disk.as_deref(),
        Some(short_digest(&digest(b".model a pmos\n").to_string()).as_str()),
        "the finding names both digests, not just that they differ"
    );
    assert!(
        finding.change.is_none(),
        "a retained library keeps one copy, so the accepted bytes are gone and \
         nothing honest can be said about what changed"
    );
    assert!(
        state
            .workbench
            .models_view
            .source_drift
            .scanned_at
            .is_some(),
        "a finding says when it was found"
    );
}

/// Opening a project in-session replaces the workspace and the catalogue
/// wholesale and touches no view state, so the latch cannot be a revision.
///
/// `apply_loaded_project_authorized` assigns `state.workspace` and
/// `state.model_library_manager` and leaves `models_view` exactly as it was.
/// The report is `#[serde(skip)]`, so a *session restore* re-arms the scan by
/// starting empty — but an in-session open does not, and the revision a
/// project file brings is whatever it was saved with. Two projects at equal
/// revisions is ordinary rather than exotic, and at equal revisions the old
/// latch held: project A's library names and digests kept painting over
/// project B until something unrelated moved the number.
#[test]
fn a_project_opened_at_the_same_revision_cannot_wear_the_previous_project_s_drift_verdict() {
    let mut open = AppState::default();
    open.model_library_manager.clear();
    open.model_library_manager.add_library(retained(
        "alpha",
        b".model a nmos\n",
        b".model a pmos\n",
    ));
    scan(&mut open);
    assert_eq!(
        findings_for(&open, "alpha").len(),
        1,
        "project A really has a finding to carry over"
    );
    assert!(!needs_scan(&open), "and its report describes project A");

    // A different project, whose file happens to have been saved at the same
    // revision. Asserted rather than assumed, so the collision this guards is
    // real and stays real if the default revision ever changes.
    let mut opened = AppState::default();
    opened.model_library_manager.clear();
    opened.model_library_manager.add_library(retained(
        "beta",
        b".model b pmos\n",
        b".model b pmos\n",
    ));
    assert_eq!(
        open.workspace.project.revision().get(),
        opened.workspace.project.revision().get(),
        "the two projects collide on revision, which is what makes this a test"
    );

    // Exactly what opening a project does: both members replaced, no view
    // state touched.
    open.workspace = opened.workspace;
    open.model_library_manager = opened.model_library_manager;

    assert!(
        findings_for(&open, "alpha").iter().count() == 1,
        "the stale verdict is still sitting in the view state — this is the \
         state a reader would be shown"
    );
    assert!(
        needs_scan(&open),
        "so the latch must re-arm on the catalogue's own content, which a \
         replacement cannot present without having earned it"
    );

    scan(&mut open);
    assert!(
        findings_for(&open, "alpha").is_empty(),
        "and the rescan drops the previous project's findings"
    );
    assert!(findings_for(&open, "beta").is_empty());
}

#[test]
fn a_built_in_catalog_has_no_pin_to_fail() {
    let mut state = AppState::default();
    state.model_library_manager.clear();
    let mut library = ModelLibrary::new("builtin");
    library.source_authority = ModelSourceAuthority::BuiltIn;
    library.source_closure = vec![ModelSourcePin {
        path: PathBuf::from("/builtin/models.lib"),
        digest: digest(b"whatever"),
    }];
    state.model_library_manager.add_library(library);
    scan(&mut state);
    assert!(findings_for(&state, "builtin").is_empty());
}

/// An external library keeps both copies, so its finding can say what moved.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_external_source_edited_on_disk_reports_how_much_of_it_moved() {
    let directory = std::env::temp_dir().join(format!(
        "rspice-drift-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|elapsed| elapsed.as_nanos())
            .unwrap_or_default()
    ));
    std::fs::create_dir_all(&directory).expect("a scratch directory for the fixture source");
    let path = directory.join("models.lib");
    let accepted = b".model a nmos\n.model b pmos\n";
    std::fs::write(&path, accepted).expect("write the accepted source");

    let mut library = ModelLibrary::new("external");
    library.root_path = Some(path.clone());
    library.source_authority = ModelSourceAuthority::External;
    library.source_closure = vec![ModelSourcePin {
        path: path.clone(),
        digest: digest(accepted),
    }];
    library.source_contents = vec![ModelSourceContent {
        path: path.clone(),
        bytes: accepted.to_vec(),
    }];

    let mut state = AppState::default();
    state.model_library_manager.clear();
    state.model_library_manager.add_library(library);
    scan(&mut state);
    assert!(
        findings_for(&state, "external").is_empty(),
        "the file on disk is the file that was accepted"
    );

    // Somebody edits the PDK under the project's feet.
    std::fs::write(&path, b".model a nmos\n.model b pmos\n.model c nmos\n")
        .expect("rewrite the source");
    state.workbench.models_view.source_drift = Default::default();
    scan(&mut state);
    let findings = findings_for(&state, "external");
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].change.as_deref(), Some("+1 −0 lines"));

    // And a source that has been removed outright is drift too, of a kind the
    // digest cannot describe.
    std::fs::remove_file(&path).expect("remove the source");
    state.workbench.models_view.source_drift = Default::default();
    scan(&mut state);
    let findings = findings_for(&state, "external");
    assert_eq!(findings.len(), 1);
    assert!(findings[0].on_disk.is_none());

    let _ = std::fs::remove_dir_all(&directory);
}

#[test]
fn the_drifted_library_reaches_the_reader_as_a_banner_with_both_digests() {
    let mut state = AppState::default();
    state.model_library_manager.clear();
    state.model_library_manager.add_library(retained(
        "moved",
        b".model a nmos\n",
        b".model a pmos\n",
    ));
    state
        .model_library_manager
        .select_library("moved")
        .expect("the fixture library is loaded");
    scan(&mut state);
    let finding = findings_for(&state, "moved")[0].clone();

    let ctx = egui::Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut pending = Vec::new();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1000.0, 600.0),
            )),
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut app = ManagerRenderContext {
                    state: &mut state,
                    pending_actions: &mut pending,
                };
                detail_banner(ui, &mut app, "moved");
            });
        },
    );
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("the banner publishes an access tree")
        .nodes;
    let labels = nodes
        .iter()
        .filter_map(|(_, node)| node.label())
        .collect::<Vec<_>>();
    assert!(
        labels.iter().any(|label| label.contains(&finding.pinned)
            && label.contains(finding.on_disk.as_deref().unwrap_or_default())),
        "the banner names both digests: {labels:?}"
    );
    assert!(
        labels.iter().any(|label| label.contains("detected")),
        "and when it was found: {labels:?}"
    );
    assert!(
        labels.iter().any(|label| *label == DRIFT_EFFECT),
        "and what it means for a run: {labels:?}"
    );
    assert!(
        nodes
            .iter()
            .any(|(_, node)| node.role() == egui::accesskit::Role::Button
                && node.label() == Some("Resolve drift…")),
        "and offers the flow that lists every finding"
    );
}

/// A clean library is silent — the banner is the exception, not the header.
#[test]
fn a_library_whose_sources_all_match_paints_no_banner() {
    let mut state = AppState::default();
    state.model_library_manager.clear();
    state.model_library_manager.add_library(retained(
        "clean",
        b".model a nmos\n",
        b".model a nmos\n",
    ));
    scan(&mut state);

    let ctx = egui::Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut pending = Vec::new();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1000.0, 600.0),
            )),
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut app = ManagerRenderContext {
                    state: &mut state,
                    pending_actions: &mut pending,
                };
                detail_banner(ui, &mut app, "clean");
            });
        },
    );
    assert!(
        output
            .platform_output
            .accesskit_update
            .expect("an access tree")
            .nodes
            .iter()
            .all(|(_, node)| node.label() != Some(DRIFT_EFFECT))
    );
}
