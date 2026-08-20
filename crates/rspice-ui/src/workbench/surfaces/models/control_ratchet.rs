//! Every control this workspace authors must declare where its effect lands.
//!
//! This workspace shipped a "Direct dependencies only" checkbox that was
//! written to session state and read by nothing, a source preview that took
//! keystrokes into a per-frame clone and dropped every one, and a card of fixed
//! prose that could never say anything else. Each was found by reading, one at
//! a time, long after it landed.
//!
//! The check here is mechanical instead. It reads the workspace's own shipped
//! source, takes every control label authored there, and requires each to
//! appear in [`CONTROL_EFFECTS`] naming the state or command it moves. That
//! does not prove the wiring works — only a test per control can do that — but
//! it makes a control with nowhere to land impossible to add silently, which is
//! how all three arrived.
//!
//! Scanning the source rather than the rendered accessibility tree is
//! deliberate. egui reports a painted table row and a real button with the same
//! role and no selection state, so the tree cannot tell a control from a row of
//! data — an earlier attempt at this drowned in model names. The source can: a
//! control's label is a literal at its call site.
//!
//! ## Adding a line to [`CONTROL_EFFECTS`]
//!
//! An entry is a claim that the control has somewhere for its effect to land,
//! named so a reader can check it against the code. It is not a place to park a
//! control that does nothing. Prefer wiring the control, or deleting it.

use crate::source_guard::production_source;

#[test]
fn every_authored_control_declares_its_effect() {
    const SOURCES: &[(&str, &str)] = &[
        ("manager.rs", include_str!("manager.rs")),
        (
            "manager/corner_ops.rs",
            include_str!("manager/corner_ops.rs"),
        ),
        ("manager/corpus.rs", include_str!("manager/corpus.rs")),
        ("manager/detail.rs", include_str!("manager/detail.rs")),
        ("manager/bindings.rs", include_str!("manager/bindings.rs")),
        ("manager/dialogs.rs", include_str!("manager/dialogs.rs")),
        ("manager/drift.rs", include_str!("manager/drift.rs")),
        (
            "manager/held_catalog.rs",
            include_str!("manager/held_catalog.rs"),
        ),
        ("manager/hub.rs", include_str!("manager/hub.rs")),
        ("manager/shelf.rs", include_str!("manager/shelf.rs")),
        (
            "manager/specialist_pages.rs",
            include_str!("manager/specialist_pages.rs"),
        ),
        (
            "manager/symbol_contracts.rs",
            include_str!("manager/symbol_contracts.rs"),
        ),
    ];

    let mut authored = std::collections::BTreeMap::<String, &str>::new();
    for (file, source) in SOURCES {
        for label in authored_control_labels(production_source(source)) {
            authored.entry(label).or_insert(file);
        }
    }

    let undeclared = authored
        .iter()
        .filter(|(label, _)| {
            !CONTROL_EFFECTS
                .iter()
                .any(|(control, _)| *control == label.as_str())
        })
        .map(|(label, file)| format!("{file}: \"{label}\""))
        .collect::<Vec<_>>();
    assert!(
        undeclared.is_empty(),
        "controls with no declared effect — wire them, delete them, or add a \
         line to CONTROL_EFFECTS saying what they move:\n  {}",
        undeclared.join("\n  ")
    );

    // The other direction, which is what stops the table rotting: a declared
    // effect whose control no longer exists is a claim about nothing, and a
    // table full of those would let the scan quietly stop matching anything.
    //
    // This half looks for the label anywhere in the source rather than at a
    // recognised constructor, because a label can legitimately sit inside the
    // constructor's argument — `compact_button(if pinned { "Refresh pin" }
    // else { "Pin source" })`. Catching those in the first direction would
    // mean parsing Rust; what matters there is the plain form, which is how a
    // new control is almost always written.
    let stale = CONTROL_EFFECTS
        .iter()
        .filter(|(control, _)| {
            let literal = format!("\"{control}\"");
            !SOURCES
                .iter()
                .any(|(_, source)| production_source(source).contains(&literal))
        })
        .map(|(control, effect)| format!("\"{control}\" ({effect})"))
        .collect::<Vec<_>>();
    assert!(
        stale.is_empty(),
        "declared effects for controls this workspace no longer authors — \
         delete the lines:\n  {}",
        stale.join("\n  ")
    );

    // A guard that finds nothing passes forever.
    assert!(
        authored.len() >= 40,
        "the scan found only {} authored controls; the label patterns have \
         stopped matching the source",
        authored.len()
    );
}

/// Control labels authored in a source file, as literals at their call site.
///
/// Only the constructors this workspace uses are recognised. A new one has to
/// be added here, and the count floor in the caller is what makes forgetting
/// that a red test rather than a silent gap.
fn authored_control_labels(source: &str) -> Vec<String> {
    /// `(pattern, whether the label is the pattern's own next literal)`.
    ///
    /// `checkbox` and `selectable_label` take the bound value first, so their
    /// label is the *following* string literal rather than the first argument.
    const CONSTRUCTORS: &[(&str, bool)] = &[
        ("ui.button(\"", true),
        ("compact_button(\"", true),
        ("egui::Button::new(\"", true),
        ("ui.link(\"", true),
        (".hint_text(\"", true),
        ("ui.checkbox(", false),
        ("ui.selectable_label(", false),
    ];
    let mut labels = Vec::new();
    for (constructor, immediate) in CONSTRUCTORS {
        for (index, _) in source.match_indices(constructor) {
            let rest = &source[index + constructor.len()..];
            let start = if *immediate {
                0
            } else {
                match rest.find('"') {
                    Some(at) => at + 1,
                    None => continue,
                }
            };
            let body = &rest[start..];
            let Some(end) = body.find('"') else { continue };
            let label = &body[..end];
            // A format string is a computed label, not an authored one: its
            // content belongs to whatever produced it.
            if label.is_empty() || label.contains('{') || label.contains('\\') {
                continue;
            }
            labels.push(label.to_owned());
        }
    }
    labels.sort();
    labels.dedup();
    labels
}

/// `(control label, the state or command it moves)`.
const CONTROL_EFFECTS: &[(&str, &str)] = &[
    // Catalog
    ("Clear filter", "models_view.catalog_query + project_facet"),
    ("Clear search", "models_view.catalog_query + part_facet"),
    (
        "Search models, parameters or consumers…",
        "models_view.catalog_query",
    ),
    ("Search installed packs…", "models_view.catalog_query"),
    // Selected model detail
    ("Pin source", "refresh_library publishes a revision"),
    ("Refresh pin", "refresh_library publishes a revision"),
    ("Open source", "models_view.dialog = SourcePreview"),
    ("Compare…", "models_view.dialog = CompareModels"),
    ("Model editor…", "Command::ModelEditor"),
    ("Author project copy…", "Command::ModelCreateProjectCopy"),
    ("Qualification", "workbench.models_page"),
    ("Bind to selection…", "bind_component_model_from_catalog"),
    (
        "Show instance",
        "schematic.selection.select_only_component + SurfaceId::Design",
    ),
    // Distributed model packs
    (
        "Refresh catalog",
        "ModelHubRequest::FetchSnapshot on the model-catalog operation machine",
    ),
    (
        "Retry the catalog refresh",
        "ModelHubRequest::FetchSnapshot, re-issued from the failed-operation banner",
    ),
    (
        "Clear release filter",
        "models_view.hub_facet + catalog_query",
    ),
    (
        "Catalog details…",
        "models_view.dialog = HeldCatalog, which reads the held catalog's identity",
    ),
    ("Close", "clears models_view.dialog"),
    ("Install", "models_view.dialog = ConfirmPack with a release"),
    ("Update", "models_view.dialog = ConfirmPack with a release"),
    ("Remove", "ModelHubRequest::RemovePack"),
    (
        "Verify installed",
        "ModelHubRequest::VerifyInstalled, whose verdict lands in models_view.pack_verification",
    ),
    (
        "Install pack",
        "ModelHubRequest::InstallPack or UpdatePack, from the confirmed release",
    ),
    // Packs and shipped parts
    ("Browse parts", "models_view.catalog_scope + selected_pack"),
    (
        "Previous",
        "models_view.part_catalog_offset + selected_part",
    ),
    ("Next", "models_view.part_catalog_offset + selected_part"),
    ("Refresh snapshot", "refresh_library publishes a revision"),
    ("Detach…", "models_view.dialog = ConfirmPack"),
    ("Attach…", "models_view.dialog = ConfirmPack"),
    ("Show pack", "models_view.catalog_scope + selected_pack"),
    ("Add to project…", "models_view.dialog = ConfirmPart"),
    ("Open qualification", "workbench.models_page"),
    ("Open card", "models_view.dialog = SourcePreview"),
    // Dialogs
    ("Cancel", "clears models_view.dialog"),
    (
        "Import",
        "select_browser_import_root publishes the chosen authenticated source root",
    ),
    ("Attach pack", "attach_pack publishes a revision"),
    ("Detach pack", "detach_pack publishes a revision"),
    ("Add to project", "add_part publishes a revision"),
    ("Add corner", "add_corner publishes a revision"),
    ("Open Model Editor", "Command::ModelEditor"),
    (
        "Clear provider decision",
        "clear_definition_provider publishes a guarded revision",
    ),
    (
        "Publish provider decision",
        "publish_definition_provider publishes a source-qualified record",
    ),
    ("Edit in Model Editor…", "Command::ModelEditor"),
    // Symbols & CDF
    ("Library manager", "SurfaceId::LibraryCellviewManager"),
    ("Import symbol", "symbol import dialog"),
    ("Form designer", "symbol parameter form dialog"),
    ("Create symbol", "model-bound symbol dialog"),
    ("Author a variant…", "model-bound symbol dialog"),
    (
        "Author project variant",
        "author_technology_symbol_variant publishes a project symbol revision",
    ),
    ("Open symbol editor", "opens the symbol view in Design"),
    ("Edit form…", "symbol parameter form dialog"),
    // Corners & sections
    ("Import section map", "Command::PdkSettings"),
    ("Add corner…", "models_view.dialog = AddCorner"),
    ("Edit corner…", "models_view.dialog = EditCorner"),
    (
        "Duplicate…",
        "models_view.dialog = EditCorner duplicate mode",
    ),
    ("Set default", "set_default_corner publishes a revision"),
    (
        "Use for execution",
        "activate_corner publishes the executable library projection",
    ),
    ("Delete corner…", "models_view.dialog = ConfirmDeleteCorner"),
    ("Delete corner", "delete_corner publishes a revision"),
    (
        "Use as the library default corner",
        "EditCorner.make_default",
    ),
    ("Bind section…", "models_view.dialog = BindCornerSection"),
    (
        "Bind section",
        "bind_corner_section publishes an authenticated revision",
    ),
    (
        "Validate bindings",
        "seals and validates the exact model execution plan",
    ),
    ("View include graph", "workbench.models_page"),
    // Bins & geometry
    ("Import bin map", "Command::PdkSettings"),
    ("Edit cards…", "Command::ModelEditor"),
    (
        "Audit all families",
        "console receipt from geometry_findings",
    ),
    ("Trace schematic", "models_view.dialog = BindingTrace"),
    // Include graph
    (
        "Resolve drift…",
        "models_view.dialog = ResolveDrift, opened on the last scan's findings",
    ),
    (
        "Re-pin this library",
        "refresh_library publishes a revision that accepts the present bytes",
    ),
    (
        "Export manifest",
        "export_workflow writes the closure manifest",
    ),
    (
        "Direct dependencies only",
        "models_view.include_direct_only, read by is_direct_closure_member",
    ),
    (
        "Filter definitions or providers…",
        "models_view.include_definition_query",
    ),
    // Qualification
    (
        "Compare approved",
        "QualificationPageAction::CompareRelease",
    ),
    (
        "Release closure",
        "QualificationPageAction::ReviewReleaseBinding",
    ),
    ("Run suite", "QualificationPageAction::RunSuite"),
    (
        "Review qualification",
        "QualificationPageAction::ReviewVectors",
    ),
    (
        "Measurement correlation",
        "QualificationPageAction::OpenCorrelation",
    ),
    (
        "Review dispositions",
        "QualificationPageAction::ReviewVectors",
    ),
];
