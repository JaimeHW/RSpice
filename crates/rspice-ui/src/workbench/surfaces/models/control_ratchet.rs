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
        ("manager/adoption.rs", include_str!("manager/adoption.rs")),
        ("manager/bins.rs", include_str!("manager/bins.rs")),
        (
            "manager/corner_ops.rs",
            include_str!("manager/corner_ops.rs"),
        ),
        ("manager/corners.rs", include_str!("manager/corners.rs")),
        ("manager/corpus.rs", include_str!("manager/corpus.rs")),
        ("manager/detail.rs", include_str!("manager/detail.rs")),
        ("manager/bindings.rs", include_str!("manager/bindings.rs")),
        ("manager/dialogs.rs", include_str!("manager/dialogs.rs")),
        ("manager/drift.rs", include_str!("manager/drift.rs")),
        (
            "manager/held_catalog.rs",
            include_str!("manager/held_catalog.rs"),
        ),
        (
            "manager/held_parts.rs",
            include_str!("manager/held_parts.rs"),
        ),
        ("manager/hub.rs", include_str!("manager/hub.rs")),
        ("manager/include.rs", include_str!("manager/include.rs")),
        ("manager/place.rs", include_str!("manager/place.rs")),
        (
            "manager/qualification_page.rs",
            include_str!("manager/qualification_page.rs"),
        ),
        ("manager/shelf.rs", include_str!("manager/shelf.rs")),
        (
            "manager/symbol_contracts.rs",
            include_str!("manager/symbol_contracts.rs"),
        ),
        ("manager/symbols.rs", include_str!("manager/symbols.rs")),
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
    /// The constructors this workspace uses to author a control whose label is
    /// the next literal after the pattern.
    ///
    /// A new one has to be added here, and the count floor in the caller is
    /// what makes forgetting that a red test rather than a silent gap.
    ///
    /// `Button::new("` is written without a path qualifier since the design
    /// system's button replaced the raw egui one; matching the bare form also
    /// catches a qualified `egui::Button::new("`, because the pattern is a
    /// substring.
    ///
    /// `checkbox` and `selectable_label` take the bound value first, so their
    /// pattern stops before the quote and their label is the following literal
    /// — which is what the second field says.
    const CONSTRUCTORS: &[(&str, bool)] = &[
        ("ui.button(\"", true),
        ("Button::new(\"", true),
        ("ui.link(\"", true),
        (".hint_text(\"", true),
        // A sortable table's header cell: clicking one reorders the table.
        ("sort_column(\"", true),
        // Two of the design system's three modal footer actions, which carry
        // their own label. The third — the primary — is positional, and is
        // read by `dialog_primary_labels` below.
        (".secondary(\"", true),
        (".ghost(\"", true),
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
            push_label(&mut labels, &body[..end]);
        }
    }
    labels.extend(dialog_primary_labels(source));
    labels.sort();
    labels.dedup();
    labels
}

/// Keep an authored label; drop a computed one.
///
/// A format string's content belongs to whatever produced it, and an escaped
/// quote means the naive quote walk above cut the literal in the wrong place.
fn push_label(labels: &mut Vec<String>, label: &str) {
    if label.is_empty() || label.contains('{') || label.contains('\\') {
        return;
    }
    labels.push(label.to_owned());
}

/// Every primary-action label a modal's `Dialog::new` call authors.
///
/// That constructor takes a kicker, a title and the primary's label, in that
/// order, and only the third is a control. Being positional rather than named,
/// it cannot be found by a pattern that ends on a quote: the kicker and the
/// title are prose that would be swept up with it, and either may be an
/// expression rather than a literal. This walks the call's own parentheses
/// instead, takes its third argument, and reads every literal inside it — one
/// label for a plain call, and two for the dialogs whose primary reads
/// `if duplicate { … } else { … }`, both of which really are authored controls.
fn dialog_primary_labels(source: &str) -> Vec<String> {
    // Spelled in two pieces so the design system's own source audit — which
    // requires every production `Dialog` construction to publish purpose text —
    // does not read this scanner's search pattern as a dialog being built here.
    const CALL: &str = concat!("Dialog", "::new(");
    let mut labels = Vec::new();
    for (index, _) in source.match_indices(CALL) {
        let open = index + CALL.len() - 1;
        let Some(close) = matching_paren(source, open) else {
            continue;
        };
        let arguments = &source[open + 1..close];
        let mut depth = 0usize;
        let mut in_string = false;
        let mut argument = 0usize;
        let mut literal = String::new();
        let mut characters = arguments.chars();
        while let Some(character) = characters.next() {
            if in_string {
                match character {
                    '\\' => {
                        literal.push(character);
                        if let Some(escaped) = characters.next() {
                            literal.push(escaped);
                        }
                    }
                    '"' => {
                        in_string = false;
                        if argument == 2 {
                            push_label(&mut labels, &literal);
                        }
                        literal.clear();
                    }
                    _ => literal.push(character),
                }
                continue;
            }
            match character {
                '"' => in_string = true,
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => depth = depth.saturating_sub(1),
                ',' if depth == 0 => argument += 1,
                _ => {}
            }
        }
    }
    labels
}

/// The index of the `)` closing the `(` at `open`, ignoring parentheses inside
/// string literals.
fn matching_paren(source: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    for (index, character) in source[open..].char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
            continue;
        }
        match character {
            '"' => in_string = true,
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(open + index);
                }
            }
            _ => {}
        }
    }
    None
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
    // The project catalog's sortable header cells. Each one reorders the rows
    // the page already derived; the order itself is durable session state.
    (
        "MODEL",
        "models_view.catalog_sort = Model, read by sort_catalog_rows",
    ),
    (
        "FAMILY",
        "models_view.catalog_sort = Family, read by sort_catalog_rows",
    ),
    (
        "SOURCE",
        "models_view.catalog_sort = Source, read by sort_catalog_rows",
    ),
    (
        "USED BY",
        "models_view.catalog_sort = UsedBy, read by sort_catalog_rows",
    ),
    (
        "VECTORS",
        "models_view.catalog_sort = Vectors, read by sort_catalog_rows",
    ),
    (
        "STATUS",
        "models_view.catalog_sort = Status, read by sort_catalog_rows",
    ),
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
    (
        "Adopt",
        "ModelHubRequest::AdoptPart, which re-retains one pinned part under the offered release \
         and moves that part's pin",
    ),
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
    (
        "Place",
        "ManagerAction::PlacePart -> schematic.arm_pack_part, or ModelHubRequest::InstallPack \
         whose completion arms the same cursor",
    ),
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
    ("Save corner", "edit_corner publishes a revision"),
    (
        "Duplicate corner",
        "edit_corner publishes a revision under the new name",
    ),
    (
        "Use as the library default corner",
        "EditCorner.make_default",
    ),
    ("Bind section…", "models_view.dialog = BindCornerSection"),
    (
        "Bind section",
        "bind_corner_section publishes an authenticated revision",
    ),
    // The bound-section excerpt pane's one action, and the painted per-row
    // unbind the details pane carries beside each bound section. The scanner
    // cannot harvest a painted hit-region's label, so the unbind entry is a
    // declaration the stale-check still verifies against the source literal.
    ("Open the file", "models_view.dialog = SourcePreview"),
    ("Unbind", "unbind_corner_section publishes a revision"),
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
    ("Trace in schematic…", "models_view.dialog = BindingTrace"),
    // Include graph
    (
        "Bind a model…",
        "schematic.selection + Command::ModelsPage(Models)",
    ),
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
