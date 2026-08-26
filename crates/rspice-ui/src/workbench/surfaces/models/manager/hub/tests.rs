//! What the pack ledger claims, checked against what it paints.
//!
//! The projection above is a pure function of the session hub and the
//! project, so most of this asserts on it directly. The rest renders one
//! fragment headless and reads the accessibility tree, because a phrase
//! painted by the table painter publishes no node of its own and a cell
//! nobody can hear is a cell that is not there.

use super::*;

use crate::workbench::app_state::AppState;

/// Renders one models-workspace fragment and returns its access tree.
fn accessibility_nodes(
    state: &mut AppState,
    size: egui::Vec2,
    render: impl FnOnce(&mut Ui, &mut ManagerRenderContext<'_>),
) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
    let ctx = egui::Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut pending = Vec::new();
    // `run_ui` takes an `FnMut`, so the one-shot renderer is parked where
    // the closure can take it rather than consuming a captured value.
    let mut render = Some(render);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut context = ManagerRenderContext {
                    state,
                    pending_actions: &mut pending,
                };
                if let Some(render) = render.take() {
                    render(ui, &mut context);
                }
            });
        },
    );
    output
        .platform_output
        .accesskit_update
        .expect("the models workspace publishes an access tree")
        .nodes
}

fn button(
    nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)],
    label: &str,
) -> Option<egui::accesskit::Node> {
    nodes
        .iter()
        .find(|(_, node)| {
            node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
        })
        .map(|(_, node)| node.clone())
}

fn labelled(nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)], label: &str) -> bool {
    nodes
        .iter()
        .any(|(_, node)| node.label().is_some_and(|found| found == label))
}

fn release(missing: &[&str]) -> PackReleaseConfirmation {
    PackReleaseConfirmation {
        name: "RSpice proving pack".to_owned(),
        version: "1.0.0".to_owned(),
        spdx: "LicenseRef-RSpice-Models".to_owned(),
        archive_length: 2 * 1024 * 1024,
        parts: 12,
        capabilities: vec!["subckt".to_owned(), "resistor".to_owned()],
        missing: missing.iter().map(|value| (*value).to_owned()).collect(),
        part: None,
        replaces: None,
    }
}

fn row(name: &str, version: &str, state: HubPackState) -> HubPackRow {
    HubPackRow {
        pack_id: format!("rspice-{}", name.to_ascii_lowercase()),
        name: name.to_owned(),
        category: "proving".to_owned(),
        version: version.to_owned(),
        state,
        spdx: "LicenseRef-RSpice-Models".to_owned(),
        archive_length: 1024 * 1024,
        parts: 4,
        capabilities: vec!["subckt".to_owned()],
        archive: None,
    }
}

/// One installed release, with whatever the startup sweep concluded.
fn held(version: &str, archive: Option<ArchiveEvidence>) -> InstalledRelease {
    InstalledRelease {
        version: version.to_owned(),
        archive,
        archive_sha256: "a".repeat(64),
    }
}

/// One project pin naming a release of the proving pack.
fn pin(version: &str, archive: &str) -> PackPartPin {
    PackPartPin {
        pack_id: "rspice-proving".to_owned(),
        pack_version: version.to_owned(),
        archive_sha256: archive.to_owned(),
        part_id: "opa2".to_owned(),
    }
}

/// One ledger line, assembled the way the projection assembles one.
fn pack(releases: Vec<HubPackRow>, installed: Option<InstalledRelease>) -> HubLedgerRow {
    pinned_pack(releases, installed, &[])
}

fn pinned_pack(
    releases: Vec<HubPackRow>,
    installed: Option<InstalledRelease>,
    pins: &[PackPartPin],
) -> HubLedgerRow {
    recalled_pack(releases, installed, pins, None)
}

fn recalled_pack(
    mut releases: Vec<HubPackRow>,
    installed: Option<InstalledRelease>,
    pins: &[PackPartPin],
    recalled: Option<(&str, &str)>,
) -> HubLedgerRow {
    releases.sort_by(newest_release_first);
    let pack_id = releases
        .first()
        .map_or_else(|| "rspice-proving".to_owned(), |row| row.pack_id.clone());
    ledger_row(
        &pack_id,
        releases,
        installed,
        pins,
        recalled.map(|(version, reason)| Recalled {
            version: version.to_owned(),
            reason: reason.to_owned(),
        }),
    )
}

/// A catalog holding the given packs, signed and verified.
fn catalog(packs: Vec<HubLedgerRow>, age_days: Option<u64>, stale: bool) -> HubCatalog {
    HubCatalog {
        packs,
        age_days,
        signed: Some("2026-08-14".to_owned()),
        unavailable: None,
        stale,
        expired: None,
        cache_discarded: false,
        identity: None,
        signing_key: "7ce1".to_owned(),
        licences: vec!["LicenseRef-RSpice-Models".to_owned()],
        host: browser::Host::default(),
        storage: crate::state::model_hub::durable::PackStorageStanding::NotApplicable,
    }
}

/// One catalog-listed pack publishing the given releases, all runnable.
fn listed(versions: &[&str]) -> SnapshotPack {
    SnapshotPack {
        id: "rspice-proving".to_owned(),
        name: "RSpice proving pack".to_owned(),
        category: "proving".to_owned(),
        releases: versions
            .iter()
            .map(|version| rspice_pack::SnapshotRelease {
                version: (*version).to_owned(),
                archive_sha256: "0".repeat(64),
                archive_length: 1024 * 1024,
                capabilities: vec!["subckt".to_owned()],
                spdx: "LicenseRef-RSpice-Models".to_owned(),
                parts: Vec::new(),
            })
            .collect(),
    }
}

fn state_of(rows: &[HubPackRow], version: &str) -> HubPackState {
    rows.iter()
        .find(|row| row.version == version)
        .unwrap_or_else(|| panic!("the table lists {version}"))
        .state
        .clone()
}

#[test]
fn a_two_digit_major_release_supersedes_the_single_digit_one_installed() {
    // Byte order ranks `9.0.0` above `10.0.0`, so the shelf both picked
    // the wrong release as newest and refused the comparison that would
    // have caught it. A machine on 9.0.0 was told it was current.
    let rows = pack_rows(&listed(&["9.0.0", "10.0.0"]), Some("9.0.0"));
    assert_eq!(state_of(&rows, "9.0.0"), HubPackState::Installed);
    assert_eq!(
        state_of(&rows, "10.0.0"),
        HubPackState::UpdateAvailable {
            installed: "9.0.0".to_owned(),
        },
        "the newest release the catalog publishes is the one that updates"
    );
}

#[test]
fn a_pre_release_never_supersedes_the_release_it_precedes() {
    // The same byte order ranks `1.2.0-rc.2` above `1.2.0`, which offered
    // an update that walks a machine backwards onto a candidate build.
    let rows = pack_rows(&listed(&["1.2.0", "1.2.0-rc.2"]), Some("1.2.0"));
    assert_eq!(state_of(&rows, "1.2.0"), HubPackState::Installed);
    assert_eq!(state_of(&rows, "1.2.0-rc.2"), HubPackState::Available);

    // And the inspector orders a pack's releases the same way, so the one
    // it puts first is the one the ledger calls newest.
    let mut ordered = ["1.2.0-rc.1", "1.2.0", "1.10.0", "1.9.0"]
        .map(|version| row("Proving", version, HubPackState::Available))
        .to_vec();
    ordered.sort_by(newest_release_first);
    assert_eq!(
        ordered
            .iter()
            .map(|row| row.version.as_str())
            .collect::<Vec<_>>(),
        ["1.10.0", "1.9.0", "1.2.0", "1.2.0-rc.1"]
    );
}

/// Render the whole install confirmation, footer included.
///
/// Its two actions moved into the design system's modal footer when this
/// workspace adopted `Dialog`, so a fragment render of the body alone can no
/// longer see them: the dialog is what has to be rendered for "are both
/// actions reachable" to mean anything.
fn confirm_pack_nodes(
    state: &mut AppState,
    confirmation: &PackReleaseConfirmation,
) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
    state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::ConfirmPack {
        pack_id: "rspice-proving".to_owned(),
        attach: true,
        release: Some(Box::new(confirmation.clone())),
    });
    let catalog = HubCatalog::default();
    let ctx = egui::Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut pending = Vec::new();
    // A modal registers its layer during the pass that draws it, so the first
    // pass renders it against a workspace that has not yet been told a modal
    // owns interaction and reports every control disabled. Three passes is the
    // settled shape for reading a dialog headless.
    let mut output = None;
    for _ in 0..3 {
        pending.clear();
        output = Some(ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1024.0, 760.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let mut context = ManagerRenderContext {
                        state,
                        pending_actions: &mut pending,
                    };
                    super::super::dialogs::render_dialog(ui, &mut context, &catalog);
                });
            },
        ));
    }
    output
        .expect("three passes")
        .platform_output
        .accesskit_update
        .expect("the models workspace publishes an access tree")
        .nodes
}

#[test]
fn the_release_confirmation_states_its_cost_and_exposes_both_actions() {
    let mut state = AppState::default();
    let confirmation = release(&[]);
    let nodes = confirm_pack_nodes(&mut state, &confirmation);
    assert!(button(&nodes, "Cancel").is_some(), "cancel is reachable");
    let install = button(&nodes, "Install pack").expect("the primary action is reachable");
    assert!(
        !install.is_disabled(),
        "a compatible release is installable"
    );
    // The cost the dialog paints comes from the confirmation it captured,
    // which is what the request it dispatches is built from too. Asserting
    // the projection rather than the painted glyphs is deliberate: this
    // workspace's property rows are painter text and expose no
    // accessibility node, so a tree assertion here would prove nothing.
    assert_eq!(byte_size(confirmation.archive_length), "2.00 MiB");
    assert_eq!(confirmation.spdx, "LicenseRef-RSpice-Models");
    assert_eq!(confirmation.parts, 12);
    assert_eq!(
        release_request("rspice-proving", &confirmation),
        ModelHubRequest::InstallPack {
            pack_id: "rspice-proving".to_owned(),
            version: "1.0.0".to_owned(),
            part: None,
        }
    );
}

#[test]
fn a_confirmed_update_replaces_the_release_it_names() {
    let mut confirmation = release(&[]);
    confirmation.version = "1.1.0".to_owned();
    confirmation.replaces = Some("1.0.0".to_owned());
    assert_eq!(
        release_request("rspice-proving", &confirmation),
        ModelHubRequest::UpdatePack {
            pack_id: "rspice-proving".to_owned(),
            installed: "1.0.0".to_owned(),
            latest: "1.1.0".to_owned(),
        }
    );
}

#[test]
fn an_incompatible_release_is_described_and_refused_rather_than_hidden() {
    let mut state = AppState::default();
    let confirmation = release(&["nonexistent-capability"]);
    let nodes = confirm_pack_nodes(&mut state, &confirmation);
    let install = button(&nodes, "Install pack").expect("the action is still present");
    assert!(
        install.is_disabled(),
        "an incompatible release cannot be installed"
    );
    assert_eq!(
        plain_list(&confirmation.missing),
        "nonexistent-capability",
        "the reason offered is the plain capability name"
    );
}

#[test]
fn every_pack_state_is_reachable_and_offers_the_action_it_earns() {
    // One row per pack now, so the states are states of a *pack*: nothing
    // held, held and current, held and superseded, and offered but
    // unrunnable. Each earns exactly the controls it can honour.
    let offered = pack(vec![row("Proving", "1.0.0", HubPackState::Available)], None);
    let current = pack(
        vec![row("Proving", "1.0.0", HubPackState::Installed)],
        Some(held("1.0.0", Some(ArchiveEvidence::MatchesCatalog))),
    );
    let superseded = pack(
        vec![
            row("Proving", "1.0.0", HubPackState::Installed),
            row(
                "Proving",
                "1.1.0",
                HubPackState::UpdateAvailable {
                    installed: "1.0.0".to_owned(),
                },
            ),
        ],
        Some(held("1.0.0", Some(ArchiveEvidence::MatchesCatalog))),
    );
    let unrunnable = pack(
        vec![row(
            "Proving",
            "1.0.0",
            HubPackState::Incompatible {
                missing: vec!["nonexistent-capability".to_owned()],
            },
        )],
        None,
    );

    for (ledger, offers, refuses) in [
        (offered, vec!["Install"], vec![]),
        (current, vec!["Verify installed", "Remove"], vec![]),
        (
            superseded,
            vec!["Update", "Verify installed", "Remove"],
            vec![],
        ),
        (unrunnable, vec![], vec!["Install"]),
    ] {
        let mut app_state = AppState::default();
        app_state.workbench.models_view.selected_pack = Some(ledger.pack_id.clone());
        let listed = catalog(vec![ledger], Some(2), false);
        let nodes =
            accessibility_nodes(&mut app_state, egui::vec2(1100.0, 760.0), move |ui, app| {
                packs_page(ui, app, &listed);
            });
        for label in offers {
            let node = button(&nodes, label)
                .unwrap_or_else(|| panic!("{label} is reachable on this pack"));
            assert!(!node.is_disabled(), "{label} is offered, not refused");
        }
        for label in refuses {
            let node = button(&nodes, label)
                .unwrap_or_else(|| panic!("{label} states its refusal rather than vanishing"));
            assert!(node.is_disabled(), "{label} refuses with its reason");
        }
    }
}

/// The chips and the table are one predicate over one projection.
///
/// The mockup shipped this defect first: facet counts derived from a
/// second walk over the corpus could — and did — disagree with the rows
/// the table then showed. `Installed` and `Available` are complements, so
/// they must account for every pack exactly once, and every chip's count
/// must equal the number of rows its own facet admits.
#[test]
fn the_facet_counts_partition_the_pack_total() {
    let named = |name: &str, state: HubPackState| {
        let mut listed = row(name, "1.0.0", state);
        listed.pack_id = format!("rspice-{}", name.to_ascii_lowercase());
        vec![listed]
    };
    let packs = vec![
        // Held, current, and nothing has re-proved it: needs attention.
        pack(
            named("Alpha", HubPackState::Installed),
            Some(held("1.0.0", Some(ArchiveEvidence::MatchesCatalog))),
        ),
        // Offered and runnable — an offer, which is also attention.
        pack(named("Beta", HubPackState::Available), None),
        // Held, matching, and pinned by this project.
        pinned_pack(
            named("Gamma", HubPackState::Installed),
            Some(held("1.0.0", Some(ArchiveEvidence::MatchesCatalog))),
            &[pin("1.0.0", &"a".repeat(64))],
        ),
        // Offered, and this build cannot run it.
        pack(
            named(
                "Delta",
                HubPackState::Incompatible {
                    missing: vec!["nonexistent-capability".to_owned()],
                },
            ),
            None,
        ),
    ];
    let hub = catalog(packs, Some(1), false);
    let mut state = AppState::default();
    let mut pending = Vec::new();
    let app = ManagerRenderContext {
        state: &mut state,
        pending_actions: &mut pending,
    };

    let counts = ledger_facet_counts(&hub, &app);
    let count = |facet: ModelHubFacet| {
        counts[ModelHubFacet::ALL
            .iter()
            .position(|candidate| *candidate == facet)
            .expect("every facet is in the registry")]
    };
    assert_eq!(count(ModelHubFacet::All), hub.packs.len());
    assert_eq!(
        count(ModelHubFacet::Installed) + count(ModelHubFacet::Available),
        count(ModelHubFacet::All),
        "installed and available are complements, so they partition the ledger"
    );
    assert_eq!(count(ModelHubFacet::Installed), 2);
    assert_eq!(count(ModelHubFacet::Pinned), 1);

    // And every chip's count is the number of rows its facet admits, which
    // is what makes a chip that lies impossible rather than unlikely.
    for facet in ModelHubFacet::ALL {
        let admitted = hub
            .packs
            .iter()
            .filter(|row| ledger_matches(row, pack_attention(row, None).as_ref(), facet))
            .count();
        assert_eq!(count(facet), admitted, "{} chip", facet.label());
    }
}

/// The ladder reports the most decisive exception, and only that one.
#[test]
fn the_attention_ladder_reports_the_most_decisive_exception_first() {
    let phrase = |row: &HubLedgerRow, proof: Option<&PackReProof>| {
        pack_attention(row, proof).map(|attention| attention.phrase)
    };

    // Nothing held and something on offer: nothing to decide. The offer is
    // on the Install control, which names the version it would fetch; a
    // row that shouted "update 1.0.0" at every pack nobody installed would
    // make the whole column noise.
    let offered = pack(vec![row("Proving", "1.0.0", HubPackState::Available)], None);
    assert_eq!(phrase(&offered, None), None);

    // Held and re-proved this session: the silent state. The verdict is
    // reported in the inspector, where somebody asking about this one pack
    // will find it — a table that shouts "verified" on every healthy row
    // buries the rows that are not.
    let current = pack(
        vec![row("Proving", "1.0.0", HubPackState::Installed)],
        Some(held("1.0.0", Some(ArchiveEvidence::MatchesCatalog))),
    );
    assert_eq!(phrase(&current, Some(&PackReProof::Verified)), None);
    assert_eq!(phrase(&current, None), Some("never re-proved".to_owned()));
    assert_eq!(
        phrase(&current, Some(&PackReProof::Failed("truncated".to_owned()))),
        Some("re-proof failed".to_owned())
    );

    // The startup comparison outranks every later verdict: bytes that no
    // longer hash to the published digest are not this release.
    let replaced = pack(
        vec![row("Proving", "1.0.0", HubPackState::Installed)],
        Some(held("1.0.0", Some(ArchiveEvidence::DiffersFromCatalog))),
    );
    assert_eq!(
        phrase(&replaced, Some(&PackReProof::Verified)),
        Some("archive differs".to_owned())
    );

    // A pin naming an archive this machine no longer has outranks a failed
    // re-proof: the re-proof can be run again, and the bytes cannot.
    let repinned = pinned_pack(
        vec![row("Proving", "1.0.0", HubPackState::Installed)],
        Some(held("1.0.0", Some(ArchiveEvidence::MatchesCatalog))),
        &[pin("1.0.0", &"b".repeat(64))],
    );
    assert_eq!(
        phrase(
            &repinned,
            Some(&PackReProof::Failed("truncated".to_owned()))
        ),
        Some("pinned archive replaced".to_owned())
    );

    // A pin whose release the catalog no longer publishes is worth saying
    // and is not urgent: the retained sources still execute, and only the
    // attribution is unprovable.
    let withdrawn = pinned_pack(Vec::new(), None, &[pin("0.9.0", &"a".repeat(64))]);
    assert_eq!(
        phrase(&withdrawn, None),
        Some("pin not installed".to_owned())
    );

    // A recall outranks the whole ladder, including the archive comparison
    // that outranks everything else: those bytes are not what was signed and
    // the reader can re-download them, whereas a recalled release is one the
    // publisher has said to stop reaching for at all.
    const REASON: &str = "the output stage mismodels saturation above 85 C.";
    let recalled = recalled_pack(
        vec![row("Proving", "1.0.0", HubPackState::Installed)],
        Some(held("1.0.0", Some(ArchiveEvidence::DiffersFromCatalog))),
        &[pin("1.0.0", &"b".repeat(64))],
        Some(("1.0.0", REASON)),
    );
    let attention = pack_attention(&recalled, Some(&PackReProof::Failed("x".to_owned())))
        .expect("a recalled release always has something to say");
    assert_eq!(attention.phrase, "release revoked");
    assert_eq!(attention.tone, AttentionTone::Error);
    assert!(
        attention.detail.contains(REASON),
        "the row carries the publisher's own reason: {}",
        attention.detail
    );
    assert!(
        attention.detail.contains("keeps solving"),
        "and says that what the project already retained is unaffected: {}",
        attention.detail
    );
    // It also lights for a project pinned to a recalled release that nobody
    // here installed — the reader with the most reason to hear about it.
    let pinned_only = recalled_pack(
        Vec::new(),
        None,
        &[pin("0.9.0", &"a".repeat(64))],
        Some(("0.9.0", REASON)),
    );
    assert_eq!(
        phrase(&pinned_only, None),
        Some("release revoked".to_owned())
    );
    // And the "needs attention" facet collects it, so the chip counts it.
    assert!(ledger_matches(
        &recalled,
        pack_attention(&recalled, None).as_ref(),
        ModelHubFacet::NeedsAttention
    ));
}

/// The Project cell states the commitment, and how it diverged.
#[test]
fn the_project_column_states_the_release_this_design_committed_to() {
    let matching = pinned_pack(
        vec![row("Proving", "1.0.0", HubPackState::Installed)],
        Some(held("1.0.0", Some(ArchiveEvidence::MatchesCatalog))),
        &[pin("1.0.0", &"a".repeat(64))],
    );
    assert_eq!(
        project_cell(&matching.adoption),
        Some(("1 part @ 1.0.0".to_owned(), false)),
        "a matching pin states the commitment and nothing else"
    );

    let stale = pinned_pack(
        vec![row("Proving", "1.1.0", HubPackState::Installed)],
        Some(held("1.1.0", Some(ArchiveEvidence::MatchesCatalog))),
        &[pin("1.0.0", &"a".repeat(64)), pin("1.0.0", &"a".repeat(64))],
    );
    assert_eq!(
        project_cell(&stale.adoption),
        Some(("2 parts @ 1.0.0 · differs from 1.1.0".to_owned(), true))
    );

    let unpinned = pack(vec![row("Proving", "1.0.0", HubPackState::Available)], None);
    assert_eq!(
        project_cell(&unpinned.adoption),
        None,
        "a pack this project never adopted leaves the cell blank"
    );
}

#[test]
fn the_status_line_states_what_is_held_and_offers_the_two_things_to_do() {
    assert_eq!(
        catalog_summary(Some("2026-08-14"), Some(0), Some(41), false, None),
        "Catalog signed 2026-08-14 · generation 41 · verified"
    );
    assert_eq!(
        catalog_summary(Some("2026-08-14"), Some(9), None, false, None),
        "Catalog signed 2026-08-14 · verified · 9 days old",
        "a cached snapshot carries no generation, so none is claimed"
    );
    assert_eq!(
        catalog_summary(None, None, None, false, None),
        "No catalog fetched"
    );
    // An expiry replaces the age clause rather than joining it: a reader owed
    // one verdict should not have to assemble it from two adverbs.
    assert_eq!(
        catalog_summary(
            Some("2026-08-14"),
            Some(9),
            Some(41),
            false,
            Some("2026-09-13T09:30:00Z")
        ),
        "Catalog signed 2026-08-14 · generation 41 · verified · expired \
         2026-09-13T09:30:00Z — refresh"
    );

    let mut state = AppState::default();
    let stale = catalog(
        vec![pack(
            vec![row("Proving", "1.0.0", HubPackState::Available)],
            None,
        )],
        Some(9),
        true,
    );
    let nodes = accessibility_nodes(&mut state, egui::vec2(1100.0, 760.0), move |ui, app| {
        packs_page(ui, app, &stale);
    });
    assert!(button(&nodes, "Refresh catalog").is_some());
    assert!(
        button(&nodes, "Catalog details…").is_some(),
        "the doctrine is one button away rather than printed on the page"
    );

    // A hub that could not open says that instead of a fresh-looking age.
    let mut state = AppState::default();
    let unavailable = HubCatalog {
        unavailable: Some("RSpice could not open its model-pack storage.".to_owned()),
        ..HubCatalog::default()
    };
    let nodes = accessibility_nodes(&mut state, egui::vec2(1100.0, 760.0), move |ui, app| {
        packs_page(ui, app, &unavailable);
    });
    assert!(
        button(&nodes, "Refresh catalog").is_none(),
        "there is nothing to refresh"
    );
    assert!(
        button(&nodes, "Install").is_none(),
        "and nothing to install"
    );
}

/// Both trust refusals reach a reader who cannot see the page.
///
/// Everything on this page is painter text, which publishes no node of its
/// own, so the loudest states on it were also the only silent ones: an expired
/// catalog's whole page, and the one word on a recalled release's row.
#[test]
fn the_expired_page_and_the_revoked_row_are_both_announced() {
    const REASON: &str = "the output stage mismodels saturation above 85 C.";

    // An expired catalog with nothing installed: the page states the instant
    // and what is unaffected, and both sentences are on one node.
    let mut state = AppState::default();
    let mut expired = catalog(Vec::new(), Some(30), true);
    expired.expired = Some("2026-07-01T00:00:00Z".to_owned());
    let announced_expiry = expired.expired.clone().expect("the fixture is expired");
    let nodes = accessibility_nodes(&mut state, egui::vec2(1100.0, 760.0), move |ui, app| {
        packs_page(ui, app, &expired);
    });
    assert!(
        nodes.iter().any(|(_, node)| node
            .label()
            .is_some_and(|label| label.contains("The held catalog expired")
                && label.contains(&announced_expiry)
                && label.contains("Installed packs, retained project sources"))),
        "the expired page names the instant and what still works"
    );
    assert!(
        nodes
            .iter()
            .any(|(_, node)| node.label().is_some_and(|label| {
                label.contains("expired 2026-07-01T00:00:00Z") && label.contains("refresh")
            })),
        "and the status line above it says the same thing in one phrase"
    );

    // A recalled release that is installed: the row's own node carries the
    // publisher's reason, because the cell that paints "release revoked"
    // cannot.
    let mut state = AppState::default();
    let recalled = catalog(
        vec![recalled_pack(
            vec![row("Proving", "1.0.0", HubPackState::Installed)],
            Some(held("1.0.0", Some(ArchiveEvidence::MatchesCatalog))),
            &[],
            Some(("1.0.0", REASON)),
        )],
        Some(1),
        false,
    );
    let nodes = accessibility_nodes(&mut state, egui::vec2(1100.0, 760.0), move |ui, app| {
        packs_page(ui, app, &recalled);
    });
    assert!(
        nodes.iter().any(|(_, node)| node
            .label()
            .is_some_and(|label| label.contains("recalled 1.0.0") && label.contains(REASON))),
        "the recalled row announces which release and why"
    );
}

/// A pack being fetched says so, on its own row, until the receipt lands.
///
/// Both halves matter. The first is the one the browser build had nothing
/// of: a priming fetch ran with no visible state anywhere, so a reader who
/// pressed Install saw a page that had not moved. The second is what stops
/// it becoming a lie — the moment the operation finishes, the row is back
/// to whatever the ledger's ordinary projection says about it.
#[test]
fn a_pack_being_fetched_says_so_until_the_operation_resolves() {
    use crate::workbench::state::ModelsAttemptedOperation;

    let landing = pack(vec![row("Proving", "1.0.0", HubPackState::Available)], None);
    let other = {
        let mut releases = vec![row("Bystander", "2.0.0", HubPackState::Available)];
        releases[0].pack_id = "rspice-bystander".to_owned();
        pinned_pack(releases, None, &[])
    };
    let running = |state: &mut AppState, progress: Option<f32>| {
        state.workbench.models_view.model_import_in_progress = true;
        state.workbench.models_view.model_import_progress = progress;
        state.workbench.models_view.attempted_operation = Some(ModelsAttemptedOperation {
            label: "model-pack install of 'rspice-proving 1.0.0'".to_owned(),
            reissuable: false,
            landing_pack: Some(landing.pack_id.clone()),
        });
    };

    // A length the signed catalog declared reaches the cell as a
    // percentage; a transfer with no declared length still says it is
    // running rather than inventing one.
    let mut state = AppState::default();
    running(&mut state, Some(0.42));
    assert_eq!(
        pack_transfer(&state, &landing).map(|transfer| transfer.phrase),
        Some("installing 42%".to_owned())
    );
    state.workbench.models_view.model_import_progress = None;
    assert_eq!(
        pack_transfer(&state, &landing).map(|transfer| transfer.phrase),
        Some("installing".to_owned())
    );
    assert!(
        pack_transfer(&state, &other).is_none(),
        "one operation runs at a time, and it lights one row"
    );

    // A pack with a standing exception yields the cell while bytes move:
    // the reader's next move is to wait, not to act on the older fact.
    let mut drifted = pack(
        vec![row("Proving", "1.0.0", HubPackState::Installed)],
        Some(held("1.0.0", Some(ArchiveEvidence::DiffersFromCatalog))),
    );
    drifted.pack_id.clone_from(&landing.pack_id);
    assert_eq!(
        pack_attention(&drifted, None).map(|attention| attention.phrase),
        Some("archive differs".to_owned()),
        "the exception is there when nothing is running"
    );

    let mut state = AppState::default();
    running(&mut state, Some(0.42));
    state.workbench.models_view.selected_pack = Some(landing.pack_id.clone());
    let listed = catalog(vec![landing.clone(), other.clone()], Some(1), false);
    let nodes = accessibility_nodes(&mut state, egui::vec2(1100.0, 760.0), |ui, app| {
        packs_page(ui, app, &listed);
    });
    let running_detail = pack_transfer(&state, &landing)
        .expect("the landing pack is transferring")
        .detail;
    assert!(
        nodes.iter().any(|(_, node)| node
            .label()
            .is_some_and(|label| label.contains(&running_detail))),
        "the row a reader is waiting on says what is happening to it"
    );

    // The receipt lands: the operation is over, and so is the cell.
    state.workbench.models_view.model_import_in_progress = false;
    state.workbench.models_view.model_import_progress = None;
    state.workbench.models_view.action_receipt =
        Some(Ok("RSpice proving pack 1.0.0 installed.".to_owned()));
    assert!(pack_transfer(&state, &landing).is_none());
    let listed = catalog(vec![landing, other], Some(1), false);
    let nodes = accessibility_nodes(&mut state, egui::vec2(1100.0, 760.0), |ui, app| {
        packs_page(ui, app, &listed);
    });
    assert!(
        !nodes.iter().any(|(_, node)| node
            .label()
            .is_some_and(|label| label.contains(&running_detail))),
        "nothing is still described as running once it has finished"
    );
}

/// A browser session says how long what it installs lasts; a desktop does not.
///
/// The sentence is asserted whole, on the accessibility tree rather than on
/// the projection, because the point of the line is that a reader — screen
/// reader included — actually receives it. The desktop half of the same
/// test is what keeps this change off the native render: the composition is
/// identical, and the only reason the line appears is the host the projection
/// carries.
#[test]
fn a_browser_session_states_the_lifetime_of_what_it_installs() {
    let listed = || {
        catalog(
            vec![pack(
                vec![row("Proving", "1.0.0", HubPackState::Available)],
                None,
            )],
            Some(1),
            false,
        )
    };

    let mut state = AppState::default();
    let session = HubCatalog {
        host: browser::Host::Browser,
        ..listed()
    };
    let nodes = accessibility_nodes(&mut state, egui::vec2(1100.0, 760.0), move |ui, app| {
        packs_page(ui, app, &session);
    });
    assert!(
        labelled(&nodes, browser::SESSION_SCOPE_NOTE),
        "the browser projection states its pack lifetime on an announced node"
    );

    let mut state = AppState::default();
    let desktop = HubCatalog {
        host: browser::Host::Desktop,
        ..listed()
    };
    let nodes = accessibility_nodes(&mut state, egui::vec2(1100.0, 760.0), move |ui, app| {
        packs_page(ui, app, &desktop);
    });
    assert!(
        !labelled(&nodes, browser::SESSION_SCOPE_NOTE),
        "a desktop session says nothing about session lifetime"
    );

    // A session with no pack store holds no packs, so it does not describe
    // how long the packs it cannot have would last.
    let mut state = AppState::default();
    let unavailable = HubCatalog {
        host: browser::Host::Browser,
        unavailable: Some("this browser denied RSpice storage".to_owned()),
        ..HubCatalog::default()
    };
    let nodes = accessibility_nodes(&mut state, egui::vec2(1100.0, 760.0), move |ui, app| {
        packs_page(ui, app, &unavailable);
    });
    assert!(!labelled(&nodes, browser::SESSION_SCOPE_NOTE));
}

/// Every operational state the workspace can reach paints its own banner.
///
/// The vocabulary existed for a release with no reader: fourteen variants,
/// twenty-five writers, and `label()` behind `#[cfg(test)]`. The check that
/// matters is therefore not "does one failure render" but "does every
/// variant render", because a variant nothing paints is a variant nobody
/// can act on.
#[test]
fn every_operational_state_paints_its_word_and_its_consequence() {
    use crate::workbench::state::{ModelsAttemptedOperation, ModelsOperationalState};

    for state in ModelsOperationalState::ALL {
        let mut app_state = AppState::default();
        app_state.workbench.models_view.operational_state = state;
        app_state.workbench.models_view.action_receipt =
            Some(Err("the pack format refused: truncated archive".to_owned()));
        app_state.workbench.models_view.attempted_operation = Some(ModelsAttemptedOperation {
            label: "model-pack install of 'rspice-proving 1.0.0'".to_owned(),
            reissuable: false,
            landing_pack: None,
        });
        let listed = catalog(
            vec![pack(
                vec![row("Proving", "1.0.0", HubPackState::Available)],
                None,
            )],
            Some(1),
            false,
        );
        let nodes =
            accessibility_nodes(&mut app_state, egui::vec2(1100.0, 760.0), move |ui, app| {
                packs_page(ui, app, &listed);
            });
        assert!(
            labelled(&nodes, state.label()),
            "the {} banner names its state",
            state.label()
        );
        assert!(
            labelled(&nodes, state.consequence()),
            "the {} banner says what the failure left behind",
            state.label()
        );
        assert!(
            labelled(&nodes, "model-pack install of 'rspice-proving 1.0.0'"),
            "the {} banner names the operation it is about",
            state.label()
        );
        assert!(
            button(&nodes, "Retry the catalog refresh").is_none(),
            "an install is retried from its own pack row, never from a \
             button that would have to guess the version"
        );
    }
}

#[test]
fn a_failed_catalog_refresh_offers_the_one_retry_it_can_re_issue() {
    use crate::workbench::state::{ModelsAttemptedOperation, ModelsOperationalState};

    let mut app_state = AppState::default();
    app_state.workbench.models_view.operational_state = ModelsOperationalState::Offline;
    app_state.workbench.models_view.action_receipt =
        Some(Err("the model hub could not be reached".to_owned()));
    app_state.workbench.models_view.attempted_operation = Some(ModelsAttemptedOperation {
        label: "model-catalog refresh".to_owned(),
        reissuable: true,
        landing_pack: None,
    });
    let offline = catalog(
        vec![pack(
            vec![row("Proving", "1.0.0", HubPackState::Available)],
            None,
        )],
        None,
        true,
    );
    let nodes = accessibility_nodes(&mut app_state, egui::vec2(1100.0, 760.0), |ui, app| {
        packs_page(ui, app, &offline);
    });
    assert!(button(&nodes, "Retry the catalog refresh").is_some());

    // And a page whose last operation succeeded says nothing at all.
    let mut healthy = AppState::default();
    healthy.workbench.models_view.action_receipt = Some(Ok("installed".to_owned()));
    let current = catalog(
        vec![pack(
            vec![row("Proving", "1.0.0", HubPackState::Available)],
            None,
        )],
        Some(0),
        false,
    );
    let nodes = accessibility_nodes(&mut healthy, egui::vec2(1100.0, 760.0), |ui, app| {
        packs_page(ui, app, &current);
    });
    assert!(
        !labelled(&nodes, ModelsOperationalState::Ready.consequence()),
        "the healthy state is silent"
    );
}

/// Every proof state an installed release can be in reaches the reader.
///
/// The re-proof itself has existed since packs shipped — `verify_installed`
/// re-hashes the archive under the release key — with no control that
/// called it and no cell that reported it, and the startup sweep computed
/// each archive's digest and compared it to nothing. The phrases below are
/// what those two facts look like once a reader can see them: the loud ones
/// on the row, and every one of them in the inspector.
#[test]
fn an_installed_release_reports_what_re_proved_it_and_what_did_not() {
    let installed = held("1.0.0", Some(ArchiveEvidence::MatchesCatalog));
    assert_eq!(
        evidence(&installed, Some(&PackReProof::Verified)).1,
        "re-proved this session"
    );
    assert_eq!(
        evidence(&installed, None).0,
        "the archive matches the published digest; nothing has re-proved it"
    );
    assert_eq!(
        evidence(
            &installed,
            Some(&PackReProof::Failed("truncated archive".to_owned()))
        ),
        (
            "truncated archive".to_owned(),
            "re-proof under the release key"
        )
    );
    let replaced = held("1.0.0", Some(ArchiveEvidence::DiffersFromCatalog));
    assert_eq!(
        evidence(&replaced, Some(&PackReProof::Verified)).0,
        "the retained archive no longer hashes to the published digest"
    );

    // And the action that produces the verdict is reachable on exactly the
    // packs that can be re-proved.
    let mut state = AppState::default();
    state.workbench.models_view.selected_pack = Some("rspice-proving".to_owned());
    let field = catalog(
        vec![pack(
            vec![row("Proving", "1.0.0", HubPackState::Installed)],
            Some(installed),
        )],
        Some(1),
        false,
    );
    let nodes = accessibility_nodes(&mut state, egui::vec2(1100.0, 760.0), move |ui, app| {
        packs_page(ui, app, &field);
    });
    assert!(button(&nodes, "Verify installed").is_some());

    let mut state = AppState::default();
    state.workbench.models_view.selected_pack = Some("rspice-proving".to_owned());
    let offered = catalog(
        vec![pack(
            vec![row("Proving", "1.1.0", HubPackState::Available)],
            None,
        )],
        Some(1),
        false,
    );
    let nodes = accessibility_nodes(&mut state, egui::vec2(1100.0, 760.0), move |ui, app| {
        packs_page(ui, app, &offered);
    });
    assert!(
        button(&nodes, "Verify installed").is_none(),
        "there is nothing on this machine to re-prove"
    );
}

#[test]
fn a_discarded_catalog_cache_is_not_reported_as_never_having_asked() {
    assert_eq!(
        catalog_summary(None, None, None, true, None),
        "The cached catalog failed verification and was discarded — refresh"
    );

    let mut state = AppState::default();
    let discarded = HubCatalog {
        packs: vec![pack(
            vec![row("Proving", "1.0.0", HubPackState::Available)],
            None,
        )],
        stale: true,
        cache_discarded: true,
        ..HubCatalog::default()
    };
    let nodes = accessibility_nodes(&mut state, egui::vec2(1100.0, 760.0), move |ui, app| {
        packs_page(ui, app, &discarded);
    });
    assert!(
        labelled(
            &nodes,
            "The cached catalog failed verification and was discarded — refresh"
        ),
        "the status line carries the discarded-cache state"
    );
    assert!(button(&nodes, "Refresh catalog").is_some());
}

/// A size is spelled in the workspace's units, by the workspace's formatter.
///
/// The last two assertions are the point of routing this through the shared
/// one: the page's own formatter divided by 1024 and then wrote `kB` and `MB`
/// beside the quotient, which is a different quantity from the one it had
/// computed. Only the em-dash is this page's own, and it is not a size.
#[test]
fn a_byte_count_reads_as_a_size_rather_than_a_number() {
    assert_eq!(byte_size(0), "—");
    assert_eq!(byte_size(512), "512 B");
    assert_eq!(byte_size(2048), "2.00 KiB");
    assert_eq!(byte_size(3 * 1024 * 1024), "3.00 MiB");
    assert_eq!(byte_size(150 * 1024 * 1024), "150.00 MiB");
    assert_eq!(
        byte_size(2 * 1024 * 1024),
        crate::simulation::run_set::format_bytes(2 * 1024 * 1024),
        "every non-zero size is the shared spelling, character for character"
    );
}
