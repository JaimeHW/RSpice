//! What resolves a corner to executable sections, and what must not.
//!
//! A section spelled with a tab still resolves; a name that appears only in a
//! comment does not. A corner binding several domains needs every one of them,
//! a legacy corner with no declared binding falls back to its own name, and
//! the typical corner carries no privilege the others lack. Inspecting is
//! UI-only; activation is an explicit transaction.

use std::path::PathBuf;

use super::super::corner_ops::{
    add_corner, bind_corner_section, delete_corner, set_default_corner, unbind_corner_section,
};
use super::*;
use crate::state::model_library::{
    CornerSectionBinding, CornerSectionDomain, DeviceModel, ModelSourceContent, ModelSourcePin,
    ModelType,
};

/// A library whose parsed models declare `sections`, and whose retained
/// bytes deliberately disagree with them.
fn library_with_sections(sections: &[&str], retained_bytes: &str) -> ModelLibrary {
    let mut library = ModelLibrary::new("pdk");
    library.root_path = Some(PathBuf::from("pdk.lib"));
    for section in sections {
        let mut model = DeviceModel::new(format!("nch_{section}"), ModelType::Nmos);
        model.section = Some((*section).to_owned());
        // A real load records the section in the complete catalog as well as
        // in the active projection, and the complete catalog is what a run
        // matches a requested section name against.
        library
            .section_models
            .entry((*section).to_owned())
            .or_default()
            .insert(model.name.clone(), model.clone());
        library.add_model(model);
    }
    library.source_closure = vec![ModelSourcePin {
        path: PathBuf::from("pdk.lib"),
        digest: crate::product::ContentDigest::from_bytes([0x11; 32]),
    }];
    library.source_contents = vec![ModelSourceContent {
        path: PathBuf::from("pdk.lib"),
        bytes: retained_bytes.as_bytes().to_vec(),
    }];
    library
}

fn corner_bound_to(name: &str, section: &str) -> ProcessCorner {
    let mut corner = ProcessCorner::new(name);
    corner.file_path = Some(PathBuf::from("pdk.lib"));
    corner.section_bindings = vec![CornerSectionBinding::new(
        CornerSectionDomain::Composite,
        section,
    )];
    corner
}

#[test]
fn a_section_spelled_with_a_tab_still_resolves() {
    // The byte search this replaced looked for the literal `.lib tt`, so a
    // file writing `.LIB\ttt` reported the corner unresolved and told the
    // engineer a run was blocked when it was not.
    let library = library_with_sections(&["tt"], ".LIB\ttt\n.model nch_tt nmos\n.ENDL\n");
    assert_eq!(corner_blocker(&library, &corner_bound_to("tt", "tt")), None);
}

#[test]
fn a_section_name_appearing_only_in_a_comment_does_not_resolve() {
    // And the same search reported a section present whenever its name
    // appeared anywhere in the retained bytes, including a comment.
    let library = library_with_sections(&["tt"], "* see .lib ff for the fast corner\n");
    let blocker = corner_blocker(&library, &corner_bound_to("ff", "ff"))
        .expect("a section nothing defines must block run expansion");
    assert!(
        blocker.contains("'ff'"),
        "the blocker must name the missing section: {blocker}"
    );
}

#[test]
fn a_corner_binding_several_domains_needs_every_section() {
    let library = library_with_sections(&["tt", "res_tt"], "");
    let mut corner = ProcessCorner::new("tt");
    corner.file_path = Some(PathBuf::from("pdk.lib"));
    corner.section_bindings = vec![
        CornerSectionBinding::new(CornerSectionDomain::Mos, "tt"),
        CornerSectionBinding::new(CornerSectionDomain::Passives, "res_tt"),
    ];
    assert_eq!(corner_blocker(&library, &corner), None);

    corner.section_bindings = vec![
        CornerSectionBinding::new(CornerSectionDomain::Mos, "tt"),
        CornerSectionBinding::new(CornerSectionDomain::Passives, "res_ss"),
    ];
    let blocker = corner_blocker(&library, &corner).expect("one missing section blocks");
    assert!(blocker.contains("'res_ss'"), "{blocker}");
    assert!(
        !blocker.contains("'tt'"),
        "a resolved axis must not be reported as missing: {blocker}"
    );
}

#[test]
fn a_legacy_corner_with_no_declared_binding_resolves_through_its_own_name() {
    // `effective_section_bindings` synthesises a composite binding named
    // for the corner when a source-backed corner declares none. The page
    // must follow that, not special-case `tt`.
    let library = library_with_sections(&["ss"], "");
    let mut corner = ProcessCorner::new("ss");
    corner.file_path = Some(PathBuf::from("pdk.lib"));
    assert_eq!(corner_blocker(&library, &corner), None);

    let mut absent = ProcessCorner::new("ff");
    absent.file_path = Some(PathBuf::from("pdk.lib"));
    assert!(corner_blocker(&library, &absent).is_some());
}

#[test]
fn the_lone_typical_corner_is_no_longer_privileged() {
    // The rule this replaced resolved any single corner named `tt`
    // regardless of whether the closure defined it.
    let library = library_with_sections(&["ss"], "");
    let mut typical = ProcessCorner::new("tt");
    typical.file_path = Some(PathBuf::from("pdk.lib"));
    assert!(
        corner_blocker(&library, &typical).is_some(),
        "a lone 'tt' with no matching section must not resolve"
    );
}

#[test]
fn a_corner_bound_to_nothing_at_all_reports_no_executable_bindings() {
    let library = library_with_sections(&["tt"], "");
    let unbound = ProcessCorner::new("floating");
    let blocker = corner_blocker(&library, &unbound)
        .expect("a retained closure with no binding is not executable");
    assert!(
        blocker.contains("no executable section bindings"),
        "the page must use the run's own wording: {blocker}"
    );

    // With no retained closure and no file there is nothing to resolve, so
    // there is nothing to report either.
    let bare = ModelLibrary::new("in-memory");
    assert_eq!(corner_blocker(&bare, &ProcessCorner::new("floating")), None);
}

/// The run's answer for one library and its execution-active corner.
///
/// It goes through the persisted shape a run actually reads, and through the
/// run's own entry point, so a page assertion beside it is a comparison rather
/// than a restatement.
fn run_required_sections(library: &ModelLibrary) -> Result<Vec<String>, String> {
    crate::io::persisted_active_model_section_names(&crate::io::ProjectModelLibrary::from(library))
}

#[test]
fn an_authored_project_library_is_never_reported_as_run_blocked() {
    // A project-owned library carrying its own definition metadata resolves no
    // sections at all: the run returns an empty list before it looks at a
    // single binding. The page had no counterpart for that escape, so it
    // painted "run expansion blocked" over runs that proceeded.
    let mut library = library_with_sections(&["tt"], "");
    library.source_authority = ModelSourceAuthority::ProjectOwned {
        source_id: crate::product::ModelSourceId::new(),
        revision: crate::product::ObjectRevision::INITIAL,
        digest: crate::product::ContentDigest::from_bytes([0x11; 32]),
    };
    library.model_definition_metadata.insert(
        "nch_tt".to_owned(),
        crate::state::model_library::ModelDefinitionMetadata::default(),
    );
    // A corner bound to a section the closure does not carry — the exact shape
    // the page used to block on.
    let corner = corner_bound_to("ff", "ff");
    library.corners.insert(corner.name.clone(), corner.clone());
    library.selected_corner = Some(corner.name.clone());

    assert_eq!(
        run_required_sections(&library),
        Ok(Vec::new()),
        "the run checks no section binding for an authored project library"
    );
    assert_eq!(
        corner_blocker(&library, &corner),
        None,
        "so the page must not call the same corner blocked"
    );
}

#[test]
fn a_section_that_parses_but_defines_nothing_is_accepted_by_page_and_run_alike() {
    // `section_index` deliberately omits a section that declares nothing — a
    // corner cannot bind to empty content. But the run never asks that: it
    // matches a requested name against the sections the authenticated closure
    // *parsed*, and an empty `.lib`/`.endl` pair is one of them. The page asked
    // the stricter question and blocked on a corner the run expands.
    let mut library = library_with_sections(&["tt"], "");
    library
        .section_models
        .entry("empty".to_owned())
        .or_default();
    let corner = corner_bound_to("empty", "empty");
    library.corners.insert(corner.name.clone(), corner.clone());
    library.selected_corner = Some(corner.name.clone());

    assert!(
        !library.defines_section("empty"),
        "the fixture is the divergent case: nothing is defined in it"
    );
    assert_eq!(
        run_required_sections(&library),
        Ok(vec!["empty".to_owned()]),
        "the run demands the section the corner names"
    );
    assert!(
        crate::io::ProjectModelLibrary::from(&library)
            .section_models
            .contains_key("empty"),
        "and the authenticated closure parsed it, so the run resolves it"
    );
    assert_eq!(
        corner_blocker(&library, &corner),
        None,
        "so the page must not call the same corner blocked"
    );
}

#[test]
fn corner_lifecycle_publishes_drafts_bindings_defaults_and_deletion() {
    let mut state = AppState::default();
    state.project_lifecycle.project_open = true;
    let library = state
        .model_library_manager
        .load_library_bytes(
            "corner-lifecycle.lib",
            b".lib TT\n.model nch NMOS (LEVEL=1 KP=1e-3)\n.endl TT\n".to_vec(),
            None,
        )
        .expect("sectioned source imports");
    let initial_revision = state.workspace.project.revision();
    let mut pending = Vec::new();
    let mut app = ManagerRenderContext {
        state: &mut state,
        pending_actions: &mut pending,
    };

    add_corner(&mut app, &library, "hot", "125", "0.9");
    let draft = app
        .state
        .model_library_manager
        .get_library(&library)
        .and_then(|library| library.corners.get("hot"))
        .expect("draft corner publishes");
    draft
        .validate_draft_contract()
        .expect("unbound draft remains persistable");
    assert!(draft.validate_contract().is_err());

    bind_corner_section(
        &mut app,
        &library,
        "hot",
        CornerSectionDomain::Composite,
        "TT",
    );
    app.state
        .model_library_manager
        .get_library(&library)
        .and_then(|library| library.corners.get("hot"))
        .expect("bound corner remains present")
        .validate_contract()
        .expect("exact authenticated section makes the corner executable");

    unbind_corner_section(&mut app, &library, "hot", CornerSectionDomain::Composite);
    let unbound = app
        .state
        .model_library_manager
        .get_library(&library)
        .and_then(|library| library.corners.get("hot"))
        .expect("unbound draft remains present");
    assert!(unbound.validate_draft_contract().is_ok());
    assert!(unbound.validate_contract().is_err());

    set_default_corner(&mut app, &library, "hot");
    delete_corner(&mut app, &library, "hot");
    let retained = app
        .state
        .model_library_manager
        .get_library(&library)
        .expect("library remains attached");
    assert!(!retained.corners.contains_key("hot"));
    assert!(retained.corners.values().any(|corner| corner.is_default));
    assert!(app.state.workspace.project.revision() > initial_revision);
    assert!(app.state.workspace.project_metadata_dirty);
}

#[test]
fn inspecting_a_corner_is_ui_only_and_activation_is_an_explicit_transaction() {
    let mut state = AppState::default();
    state.project_lifecycle.project_open = true;
    let library = state
        .model_library_manager
        .load_library_bytes(
            "corner-authority.lib",
            b".lib TT\n.model nch NMOS (LEVEL=1 KP=1e-3)\n.endl TT\n".to_vec(),
            None,
        )
        .expect("executable section imports");
    state
        .model_library_manager
        .select_library(&library)
        .expect("the fixture library is loaded");
    let root_path = state
        .model_library_manager
        .get_library(&library)
        .and_then(|library| library.root_path.clone())
        .expect("imported library has a retained root");
    let mut inspection_target = ProcessCorner::new("inspection-target");
    inspection_target.file_path = Some(root_path);
    inspection_target.required_domains = vec![CornerSectionDomain::Composite];
    inspection_target.section_bindings = vec![CornerSectionBinding::new(
        CornerSectionDomain::Composite,
        "TT",
    )];
    state
        .model_library_manager
        .get_library_mut(&library)
        .expect("imported library remains present")
        .corners
        .insert(inspection_target.name.clone(), inspection_target);
    let initial_revision = state.workspace.project.revision();
    let initial_epoch = state.design_execution_epoch;
    let initial_active = state
        .model_library_manager
        .get_library(&library)
        .and_then(|library| library.selected_corner.clone());
    let mut pending = Vec::new();
    let mut app = ManagerRenderContext {
        state: &mut state,
        pending_actions: &mut pending,
    };
    let rows = corner_rows(&app);
    let inspected = rows
        .iter()
        .find(|row| {
            row.resolved()
                && initial_active
                    .as_deref()
                    .is_none_or(|active| !active.eq_ignore_ascii_case(&row.corner.name))
        })
        .cloned()
        .expect("fixture supplies a resolved inactive corner");

    inspect_corner(&mut app, &inspected);

    assert_eq!(
        app.state.workbench.models_view.selected_corner.as_deref(),
        Some(inspected.key.as_str())
    );
    assert_eq!(
        app.state
            .model_library_manager
            .get_library(&library)
            .and_then(|library| library.selected_corner.as_deref()),
        initial_active.as_deref(),
        "inspection must not change the executable section"
    );
    assert_eq!(app.state.workspace.project.revision(), initial_revision);
    assert_eq!(app.state.design_execution_epoch, initial_epoch);

    activate_corner(&mut app, &library, &inspected.corner.name);

    assert_eq!(
        app.state
            .model_library_manager
            .get_library(&library)
            .and_then(|library| library.selected_corner.as_deref()),
        Some(inspected.corner.name.as_str())
    );
    assert!(app.state.workspace.project.revision() > initial_revision);
    assert_eq!(
        app.state.design_execution_epoch,
        initial_epoch.wrapping_add(1)
    );

    let active_after_activation = inspected.corner.name.clone();
    let default_target = rows
        .iter()
        .find(|row| {
            !row.corner
                .name
                .eq_ignore_ascii_case(&active_after_activation)
        })
        .expect("fixture supplies another corner")
        .corner
        .name
        .clone();
    set_default_corner(&mut app, &library, &default_target);
    assert_eq!(
        app.state
            .model_library_manager
            .get_library(&library)
            .and_then(|library| library.selected_corner.as_deref()),
        Some(active_after_activation.as_str()),
        "changing the new-plan default must not silently activate it"
    );
}

/// Every matrix cell is a fact about *this* corner, or it is blank.
///
/// The cells this replaces were neither: BJT and passives painted the literal
/// word "section" for any corner whose composite binding resolved, and the
/// statistical and aging columns painted per-library booleans, so every corner
/// in a library showed identical cells whatever it bound. A PDK with
/// independently selectable device sections could not see which corner bound
/// which — which is the only question the matrix exists to answer.
#[test]
fn a_matrix_cell_names_the_section_this_corner_binds_or_stays_blank() {
    // A conventional `.lib TT` corner: one composite section owning every
    // device class, and nothing at all for statistics or aging.
    let composite = ProcessCorner::from_composite_section("tt", PathBuf::from("pdk.lib"), true);
    assert_eq!(domain_cell(&composite, CornerSectionDomain::Bjt), "tt");
    assert_eq!(domain_cell(&composite, CornerSectionDomain::Passives), "tt");
    assert_eq!(
        statistical_cell(&composite),
        "",
        "a composite section says nothing about statistics, and inventing a \
         binding for it would turn 'this PDK ships none' into 'bound'"
    );
    assert_eq!(
        domain_cell(&composite, CornerSectionDomain::Aging),
        "",
        "nor about aging"
    );

    // A PDK publishing independently selectable sections: each cell names the
    // one this corner actually binds, and an unbound domain stays empty.
    let mut split = ProcessCorner::new("ff");
    split.file_path = Some(PathBuf::from("pdk.lib"));
    split.section_bindings = vec![
        CornerSectionBinding::new(CornerSectionDomain::Mos, "mos_ff"),
        CornerSectionBinding::new(CornerSectionDomain::Bjt, "bjt_ff"),
        CornerSectionBinding::new(CornerSectionDomain::StatisticalGlobal, "stat_global"),
        CornerSectionBinding::new(CornerSectionDomain::StatisticalLocal, "stat_local"),
    ];
    assert_eq!(domain_cell(&split, CornerSectionDomain::Bjt), "bjt_ff");
    assert_eq!(
        domain_cell(&split, CornerSectionDomain::Passives),
        "",
        "a domain this corner binds nothing to is blank, not 'section'"
    );
    assert_eq!(statistical_cell(&split), "stat_global · stat_local");
    assert_eq!(domain_cell(&split, CornerSectionDomain::Aging), "");

    split
        .section_bindings
        .push(CornerSectionBinding::new(CornerSectionDomain::Aging, "hci"));
    assert_eq!(domain_cell(&split, CornerSectionDomain::Aging), "hci");

    // And two corners of one library no longer show the same cells: the old
    // derivations were per-library, so they could not differ.
    assert_ne!(
        domain_cell(&composite, CornerSectionDomain::Bjt),
        domain_cell(&split, CornerSectionDomain::Bjt)
    );
}

/// The Corners page states which corners the run set runs outside.
///
/// The corner already carried a qualified range and the run set already
/// carried its temperatures, and nothing compared them — so a project sweeping
/// to 150 °C against a corner qualified to 125 °C was told nothing anywhere.
#[test]
fn a_corner_qualified_narrower_than_the_run_set_says_so_on_its_own_page() {
    let mut state = AppState::default();
    state.model_library_manager.clear();
    let mut library = ModelLibrary::new("pdk");
    library.root_path = Some(PathBuf::from("pdk.lib"));
    library.corners.clear();
    let mut hot = crate::state::model_library::ProcessCorner::from_composite_section(
        "hot",
        PathBuf::from("pdk.lib"),
        true,
    );
    hot.minimum_temperature_c = Some(-40.0);
    hot.maximum_temperature_c = Some(125.0);
    library.corners.insert("hot".to_owned(), hot);
    library.selected_corner = Some("hot".to_owned());
    state.model_library_manager.add_library(library);
    state
        .model_library_manager
        .select_library("pdk")
        .expect("the fixture library is loaded");
    state.sim_setup.reference_pvt.temperature_celsius = 150.0;

    let rendered = |state: &mut AppState| {
        let ctx = egui::Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        let mut pending = Vec::new();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1200.0, 800.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let mut app = ManagerRenderContext {
                        state,
                        pending_actions: &mut pending,
                    };
                    let rows = corner_rows(&app);
                    temperature_validity_findings(ui, &rows);
                    let _ = &mut app;
                });
            },
        );
        output
            .platform_output
            .accesskit_update
            .expect("an access tree")
            .nodes
            .iter()
            .filter_map(|(_, node)| node.label().map(str::to_owned))
            .collect::<Vec<_>>()
    };

    let labels = rendered(&mut state);
    assert!(
        labels
            .iter()
            .any(|label| label
                == "HOT is qualified -40.000 to 125.000 °C; this run set requests 150 °C"),
        "{labels:?}"
    );

    // Inside the range — including exactly at the endpoint — is silent.
    state.sim_setup.reference_pvt.temperature_celsius = 125.0;
    assert!(
        rendered(&mut state)
            .iter()
            .all(|label| !label.contains("is qualified")),
        "a corner that covers the run set says nothing"
    );
}

#[test]
fn the_family_list_declares_the_height_a_row_really_takes() {
    // `show_rows` places rows from the height it is given. If that height
    // is short, every row after the first drifts up under the one above
    // and the last families fall off the end of the list.
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut declared = 0.0;
    let mut measured = 0.0;
    for _ in 0..2 {
        let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                declared = selectable_label_height(ui);
                measured = ui
                    .selectable_label(false, "pdk7 · nch7  ·  25 cards")
                    .rect
                    .height();
            });
        });
    }
    assert!(
        (declared - measured).abs() < 0.01,
        "the family list declares {declared} per row but a row takes {measured}"
    );
}

#[test]
fn a_family_is_one_library_s_cards_sharing_a_base_name() {
    let mut state = AppState::default();
    state.model_library_manager.clear();
    let mut library = ModelLibrary::new("foundry");
    for name in ["nch.1", "nch.2", "pch.1"] {
        library.add_model(DeviceModel::new(
            name,
            crate::state::model_library::ModelType::Nmos,
        ));
    }
    state.model_library_manager.add_library(library);
    let geometry = rspice_core::engine::ModelBinCardGeometry {
        length: rspice_core::engine::ModelBinAxisRange {
            min: Some(1.0e-7),
            max: Some(2.0e-7),
        },
        width: rspice_core::engine::ModelBinAxisRange {
            min: Some(1.0e-7),
            max: Some(2.0e-7),
        },
        nfin: rspice_core::engine::ModelBinAxisRange {
            min: Some(1.0),
            max: Some(2.0),
        },
    };
    let inspection = rspice_core::engine::ModelBinInspection {
        cards: [("nch.1", "nch"), ("nch.2", "nch"), ("pch.1", "pch")]
            .into_iter()
            .enumerate()
            .map(|(declaration_order, (model, family))| {
                rspice_core::engine::ModelBinCardInspection {
                    model: model.to_owned(),
                    family: family.to_owned(),
                    model_type: "NMOS".to_owned(),
                    declaration_order,
                    geometry,
                }
            })
            .collect(),
        instances: Vec::new(),
    };
    let mut pending = Vec::new();
    let render = ManagerRenderContext {
        state: &mut state,
        pending_actions: &mut pending,
    };
    let families = bin_families(&render, &inspection).expect("providers resolve");
    assert_eq!(families.len(), 2);
    assert_eq!(families[0].library, "foundry");
    assert_eq!(families[0].family, "nch");
    assert_eq!(families[0].cards.len(), 2);
    assert_eq!(families[1].family, "pch");
    assert_eq!(families[1].cards.len(), 1);
}

#[test]
fn geometry_findings_use_the_engine_s_nfin_axis() {
    let axis = |min, max| rspice_core::engine::ModelBinAxisRange {
        min: Some(min),
        max: Some(max),
    };
    let card = |model: &str, nfin_min: f64, nfin_max: f64| BinCard {
        model: model.to_owned(),
        geometry: rspice_core::engine::ModelBinCardGeometry {
            length: axis(1.0e-7, 5.0e-7),
            width: axis(1.0e-7, 5.0e-7),
            nfin: axis(nfin_min, nfin_max),
        },
        declaration_order: 0,
    };
    let disjoint = BinFamily {
        library: "foundry".to_owned(),
        family: "nch".to_owned(),
        cards: vec![card("nch.1", 1.0, 2.0), card("nch.2", 3.0, 4.0)],
    };
    assert!(
        geometry_findings(&[disjoint]).is_empty(),
        "cards overlapping in L/W but disjoint in NFIN are not ambiguous"
    );

    let overlapping = BinFamily {
        library: "foundry".to_owned(),
        family: "nch".to_owned(),
        cards: vec![card("nch.1", 1.0, 3.0), card("nch.2", 2.0, 4.0)],
    };
    assert_eq!(geometry_findings(&[overlapping]).len(), 1);
}

/// A duplicate the manager settled is not contested.
///
/// `contested()` was `providers.len() > 1`, which marked every multi-provider
/// name — including the ones whose RESOLUTION column, three cells to the right,
/// already said "resolved · <library>". That inflated the card's own count of
/// what has to be repaired and offered a conflict dialog for a name that binds.
#[test]
fn a_definition_with_a_recorded_provider_is_not_contested() {
    use crate::state::model_library::ModelConsumerScope;

    let settled = super::DefinitionRow {
        definition: "nch".to_owned(),
        scope: ModelConsumerScope::PrimitiveModel,
        providers: vec!["foundry".to_owned(), "vendor".to_owned()],
        provider_list: "foundry, vendor".to_owned(),
        resolution: "resolved · foundry".to_owned(),
        resolved_provider: Some("foundry".to_owned()),
    };
    assert!(
        !settled.contested(),
        "a name with a recorded provider binds, so it is not what has to be repaired"
    );

    let unsettled = super::DefinitionRow {
        resolved_provider: None,
        resolution: "contested · fails closed".to_owned(),
        ..settled
    };
    assert!(
        unsettled.contested(),
        "two providers and no decision is the case that fails closed"
    );

    let unique = super::DefinitionRow {
        providers: vec!["foundry".to_owned()],
        provider_list: "foundry".to_owned(),
        resolution: "unique".to_owned(),
        resolved_provider: None,
        ..unsettled
    };
    assert!(!unique.contested(), "one provider is never contested");
}
