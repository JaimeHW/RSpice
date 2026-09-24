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

/// A project that has attached nothing has no corner bindings to report.
///
/// `ModelLibrary::new` used to seed tt/ff/ss/fs/sf into every library it made,
/// so the compiled-in foundation catalog — which has no source file, and so
/// can bind no section — opened a brand-new project on five red rows, a
/// blocked default corner, and no control that could ever resolve them: Bind
/// section can only fail against a library that declares none. The page's own
/// empty state is the truthful reading of a project that has attached nothing.
#[test]
fn a_fresh_project_reports_no_corner_bindings_at_all() {
    let mut state = AppState::default();
    let mut pending = Vec::new();
    let app = ManagerRenderContext {
        state: &mut state,
        pending_actions: &mut pending,
    };

    assert!(
        corner_rows(&app).is_empty(),
        "the compiled-in catalog declares no section, so it declares no corner"
    );
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

/// Retained bytes with two sections in one file, which is the shape a PDK
/// corner file actually has: sections addressed by name, never a file each.
const RETAINED_CORNER_FILE: &str = "\
* demo180 corner sections
.param vdd_nom=1.8
.lib tt
.param dvthn=0 dvthp=0
*
* technology constants — shared by every section
.param toxe=4.1n
.model nch_tt nmos level=54 version=4.8
.model pch_tt pmos level=54 version=4.8
.endl tt
*
.lib mc_g
.param mc_process=1
.model nch_mc_g nmos level=54
.endl
";

#[test]
fn the_bound_section_slice_is_the_pair_the_corner_names() {
    let excerpt = slice_section(RETAINED_CORNER_FILE.as_bytes(), "tt")
        .expect("the retained file defines `.lib tt`");
    assert_eq!(
        excerpt.first_line, 3,
        "the gutter states where in the file the section is, not where the slice starts"
    );
    assert_eq!(excerpt.total_lines, 8);
    assert_eq!(excerpt.lines.first().map(String::as_str), Some(".lib tt"));
    assert_eq!(excerpt.lines.last().map(String::as_str), Some(".endl tt"));
    assert!(
        !excerpt.lines.iter().any(|line| line.contains("mc_process")),
        "the slice stopped at its own .endl: {:?}",
        excerpt.lines
    );

    // The second section is reachable by name in the same file, and its
    // `.endl` carries no name at all.
    let mc = slice_section(RETAINED_CORNER_FILE.as_bytes(), "mc_g").expect("the second section");
    assert_eq!(mc.total_lines, 4);
    assert_eq!(mc.lines.last().map(String::as_str), Some(".endl"));

    // Case and tabs are the file's business, not the reader's.
    let odd = slice_section(b".LIB\tTT\n.model m nmos\n.ENDL\n", "tt").expect("case and tabs");
    assert_eq!(odd.total_lines, 3);

    // A three-token `.lib <file> <section>` includes a section defined
    // elsewhere. Reading it as a definition would show an empty pane under a
    // header claiming this file defines the corner.
    assert!(
        slice_section(b".lib ../shared/models.lib tt\n", "tt").is_none(),
        "an include is not a definition"
    );
    assert!(slice_section(RETAINED_CORNER_FILE.as_bytes(), "ff").is_none());
}

#[test]
fn a_long_section_is_capped_and_says_how_much_it_left() {
    let mut source = String::from(".lib tt\n");
    for index in 0..200 {
        source.push_str(&format!(".param p{index}=0\n"));
    }
    source.push_str(".endl\n");
    let excerpt = slice_section(source.as_bytes(), "tt").expect("the section");
    assert_eq!(
        excerpt.lines.len(),
        SECTION_EXCERPT_LINES,
        "the pane holds a fixed slice however large the file is"
    );
    assert_eq!(
        excerpt.total_lines, 202,
        "and still counts what it did not keep, so the footer can state it"
    );
}

/// A library whose retained file carries both sections, one corner bound to
/// `tt`, and one draft corner with a required domain left unbound.
fn corner_page_state() -> AppState {
    let mut state = AppState::default();
    state.model_library_manager.clear();
    let mut library = library_with_sections(&["tt", "mc_g"], RETAINED_CORNER_FILE);
    library.corners.clear();
    let mut typical = corner_bound_to("tt", "tt");
    typical.section_bindings.push(CornerSectionBinding::new(
        CornerSectionDomain::StatisticalGlobal,
        "mc_g",
    ));
    library.corners.insert("tt".to_owned(), typical);

    let mut draft = ProcessCorner::new("hot_5v5");
    draft.file_path = Some(PathBuf::from("pdk.lib"));
    draft.required_domains = vec![CornerSectionDomain::Mos, CornerSectionDomain::Passives];
    draft.section_bindings = vec![CornerSectionBinding::new(CornerSectionDomain::Mos, "tt")];
    library.corners.insert("hot_5v5".to_owned(), draft);
    library.selected_corner = Some("tt".to_owned());

    state.model_library_manager.add_library(library);
    state
        .model_library_manager
        .select_library("pdk")
        .expect("the fixture library is loaded");
    state
}

/// Render the page and report every accessibility node it published.
fn rendered_corner_page(state: &mut AppState) -> Vec<(egui::accesskit::Role, String)> {
    let ctx = egui::Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut pending = Vec::new();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1200.0, 900.0),
            )),
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut app = ManagerRenderContext {
                    state,
                    pending_actions: &mut pending,
                };
                corners_page(ui, &mut app);
            });
        },
    );
    output
        .platform_output
        .accesskit_update
        .expect("an access tree")
        .nodes
        .iter()
        .filter_map(|(_, node)| Some((node.role(), node.label()?.to_owned())))
        .collect()
}

#[test]
fn the_bound_section_source_is_shown_where_the_binding_is_stated() {
    let mut state = corner_page_state();
    let nodes = rendered_corner_page(&mut state);
    let labels = nodes
        .iter()
        .map(|(_, label)| label.as_str())
        .collect::<Vec<_>>();

    assert!(
        labels
            .iter()
            .any(|label| label.contains("pdk.lib(tt)") || label.contains("pdk.lib(TT)")),
        "the pane's header names the file and the section it sliced: {labels:?}"
    );
    assert!(
        labels.iter().any(|label| label.contains("bound by TT")),
        "and which corner binds it: {labels:?}"
    );
    assert!(
        labels.iter().any(|label| label.contains("read-only")),
        "and that nothing here is editable: {labels:?}"
    );
    assert!(
        nodes.iter().any(|(role, label)| *role
            == egui::accesskit::Role::Button
            && label == "Open the file"),
        "the whole file stays one control away: {labels:?}"
    );
    // The slice itself reaches a reader who cannot see the painted glyphs.
    assert!(
        labels
            .iter()
            .any(|label| label.contains(".lib tt") && label.contains("level=54")),
        "the excerpt publishes one node carrying its own lines: {labels:?}"
    );
    assert!(
        labels
            .iter()
            .any(|label| label.contains("nothing here is copied into the deck")),
        "and the pane states what a section binding actually does: {labels:?}"
    );
}

#[test]
fn a_section_the_retained_file_does_not_carry_is_stated_rather_than_left_blank() {
    // A corner naming a section the file never defines is exactly the state
    // this pane exists for. Rendering nothing at all would read as a pane that
    // failed rather than as a binding that cannot resolve.
    let mut state = corner_page_state();
    let library = state
        .model_library_manager
        .get_library_mut("pdk")
        .expect("the fixture library");
    library.corners.remove("hot_5v5");
    library
        .corners
        .insert("ff".to_owned(), corner_bound_to("ff", "ff"));
    library.selected_corner = Some("ff".to_owned());
    state.workbench.models_view.selected_corner = Some("pdk\u{1f}ff".to_owned());

    let labels = rendered_corner_page(&mut state)
        .into_iter()
        .map(|(_, label)| label)
        .collect::<Vec<_>>();
    assert!(
        labels
            .iter()
            .any(|label| label.contains("No `.lib ff` … `.endl` pair appears")),
        "{labels:?}"
    );
}

#[test]
fn the_source_excerpt_is_sliced_once_and_read_back_after_that() {
    // The page carries no scroll of its own and repaints at the frame rate, so
    // a slice re-derived per frame would walk every retained byte sixty times a
    // second for a pane that changed nothing.
    let mut state = corner_page_state();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut pending = Vec::new();
    let mut excerpts = Vec::new();
    let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let app = ManagerRenderContext {
                state: &mut state,
                pending_actions: &mut pending,
            };
            let rows = corner_rows(&app);
            let row = rows
                .iter()
                .find(|row| row.corner.name == "tt")
                .expect("the typical corner");
            for _ in 0..3 {
                excerpts.push(bound_section_excerpt(ui, &app, row, "tt"));
            }
        });
    });
    assert!(
        Arc::ptr_eq(&excerpts[0], &excerpts[1]) && Arc::ptr_eq(&excerpts[1], &excerpts[2]),
        "every read after the first is the cached slice, not a fresh walk"
    );
    assert!(excerpts[0].is_ok());

    // A different section is a different slice; the key has to notice.
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut sections = Vec::new();
    let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let app = ManagerRenderContext {
                state: &mut state,
                pending_actions: &mut pending,
            };
            let rows = corner_rows(&app);
            let row = rows
                .iter()
                .find(|row| row.corner.name == "tt")
                .expect("the typical corner");
            sections.push(bound_section_excerpt(ui, &app, row, "tt"));
            sections.push(bound_section_excerpt(ui, &app, row, "mc_g"));
        });
    });
    assert!(!Arc::ptr_eq(&sections[0], &sections[1]));
    assert_eq!(
        sections[1]
            .as_ref()
            .as_ref()
            .expect("the second section slices")
            .first_line,
        12
    );
}

#[test]
fn an_unresolved_corner_carries_the_bind_control_on_its_own_row() {
    // The action row below acts on whichever corner is selected, so resolving
    // a draft used to mean selecting it first and then finding the control
    // somewhere else. The mockup puts it on the row, and a painted row can
    // still publish a real button.
    let mut state = corner_page_state();
    let nodes = rendered_corner_page(&mut state);
    assert!(
        nodes
            .iter()
            .any(|(role, label)| *role == egui::accesskit::Role::Button
                && label == "Bind section for corner hot_5v5"),
        "the draft corner's own row offers the bind: {nodes:?}"
    );
    assert!(
        !nodes
            .iter()
            .any(|(_, label)| label == "Bind section for corner tt"),
        "and a corner that already resolves is offered nothing to fix"
    );
}

#[test]
fn a_corner_the_bind_dialog_cannot_help_is_offered_no_row_control() {
    // Two blockers the dialog cannot lift: a corner bound to no retained
    // source at all, and one whose own contract is malformed. Offering a
    // control that cannot change the verdict is worse than offering none.
    let mut state = AppState::default();
    state.model_library_manager.clear();
    let mut library = library_with_sections(&["tt"], RETAINED_CORNER_FILE);
    library.corners.clear();
    library.root_path = None;
    library.source_contents.clear();
    let mut sourceless = ProcessCorner::new("nowhere");
    sourceless.file_path = None;
    library.corners.insert("nowhere".to_owned(), sourceless);

    let mut malformed = corner_bound_to("bad", "tt");
    malformed.vdd_factor = 0.0;
    library.corners.insert("bad".to_owned(), malformed);
    state.model_library_manager.add_library(library);
    state
        .model_library_manager
        .select_library("pdk")
        .expect("the fixture library is loaded");

    let mut pending = Vec::new();
    let app = ManagerRenderContext {
        state: &mut state,
        pending_actions: &mut pending,
    };
    for row in corner_rows(&app) {
        assert!(
            !row.binding_blocked,
            "'{}' cannot be fixed by naming a section: {:?}",
            row.corner.name, row.blocker
        );
    }
}

#[test]
fn each_unbind_sits_on_the_section_it_removes() {
    // The action row used to grow one "Unbind <domain>" per binding, so a PDK
    // binding several domains pushed the corner's own lifecycle actions along
    // a row of destructive controls. Each now names the section it acts on,
    // which the domain label alone never did.
    let mut state = corner_page_state();
    let nodes = rendered_corner_page(&mut state);
    let mut unbinds = nodes
        .iter()
        .filter(|(role, label)| {
            *role == egui::accesskit::Role::Button && label.starts_with("Unbind")
        })
        .map(|(_, label)| label.as_str())
        .collect::<Vec<_>>();
    unbinds.sort_unstable();
    assert_eq!(
        unbinds,
        vec![
            "Unbind Composite section tt",
            "Unbind Statistical (global) section mc_g",
        ],
        "one per binding, each naming its section: {nodes:?}"
    );

    // And the action row is the fixed set of corner actions, with no control
    // whose presence depends on how many domains the PDK splits into.
    let actions = nodes
        .iter()
        .filter(|(role, _)| *role == egui::accesskit::Role::Button)
        .map(|(_, label)| label.as_str())
        .collect::<Vec<_>>();
    for action in [
        "Use for execution",
        "Edit corner…",
        "Duplicate…",
        "Set default",
        "Delete corner…",
        "Bind section…",
        "Open source",
        "View include graph",
        "Model editor…",
    ] {
        assert!(actions.contains(&action), "missing {action}: {actions:?}");
    }
}

#[test]
fn the_fail_closed_contract_is_stated_under_the_matrix() {
    let mut state = corner_page_state();
    let labels = rendered_corner_page(&mut state)
        .into_iter()
        .map(|(_, label)| label)
        .collect::<Vec<_>>();
    assert!(
        labels.iter().any(|label| label
            .contains("no implicit typical fallback and no silent alias resolution")
            && label.contains("held out of every run")),
        "the contract, and that this project is currently held by it: {labels:?}"
    );
}

#[test]
fn the_statistical_card_counts_only_what_the_bound_section_declares() {
    use crate::state::model_library::{
        FiniteF64, ModelDefinitionMetadata, StatisticalDefinition, StatisticalDistribution,
        StatisticalHierarchyScope, StatisticalVariableDefinition,
    };

    let mut library = library_with_sections(&["mc_g", "mc_l"], RETAINED_CORNER_FILE);
    let variable = |name: &str| StatisticalVariableDefinition {
        name: name.to_owned(),
        parameter: "vth0".to_owned(),
        distribution: StatisticalDistribution::Normal {
            sigma: FiniteF64::new(1e-3).expect("finite"),
        },
        correlation_group: None,
        hierarchy: StatisticalHierarchyScope::Global,
        description: String::new(),
    };
    library.model_definition_metadata.insert(
        "nch_mc_g".to_owned(),
        ModelDefinitionMetadata {
            statistics: StatisticalDefinition {
                variables: vec![variable("dvth_g"), variable("du0_g")],
                correlation_matrices: Vec::new(),
            },
            ..ModelDefinitionMetadata::default()
        },
    );

    assert_eq!(
        declared_statistics(&library, "mc_g").0,
        "2 declared variables"
    );
    assert_eq!(
        declared_statistics(&library, "mc_l").0,
        "no declared statistical variable",
        "a section whose models declare none must not borrow the other section's count"
    );
    assert_eq!(
        declared_statistics(&library, "aging_10y").0,
        "no retained model carries this section",
        "and a section no retained model belongs to says exactly that"
    );
}

#[test]
fn a_corner_binding_no_statistics_says_so_instead_of_inventing_a_sampling_plan() {
    let mut state = corner_page_state();
    let library = state
        .model_library_manager
        .get_library_mut("pdk")
        .expect("the fixture library");
    let typical = library.corners.get_mut("tt").expect("the typical corner");
    typical
        .section_bindings
        .retain(|binding| binding.domain == CornerSectionDomain::Composite);

    let labels = rendered_corner_page(&mut state)
        .into_iter()
        .map(|(_, label)| label)
        .collect::<Vec<_>>();
    assert!(
        labels
            .iter()
            .any(|label| label.contains("TT binds no statistical or aging section")),
        "{labels:?}"
    );
    assert!(
        labels
            .iter()
            .all(|label| !label.contains("seed") && !label.contains("sample")),
        "sampling and seeds belong to the run set, not to a corner: {labels:?}"
    );
}

/// Renders of the populated page, so its density can be looked at rather than
/// only asserted about.
///
/// Run with `--ignored`; the PNGs go to `RSPICE_RASTER_DIR`.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_corner_page_states() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();

    let raster = |state: &mut AppState| {
        let mut pending = Vec::new();
        crate::ui::raster::render(egui::vec2(1180.0, 900.0), |ui, background| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(background))
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                    let mut app = ManagerRenderContext {
                        state,
                        pending_actions: &mut pending,
                    };
                    corners_page(ui, &mut app);
                });
        })
    };

    let mut bound = corner_page_state();
    let mut draft = corner_page_state();
    draft.workbench.models_view.selected_corner = Some("pdk\u{1f}hot_5v5".to_owned());
    let mut narrow = corner_page_state();
    {
        let library = narrow
            .model_library_manager
            .get_library_mut("pdk")
            .expect("the fixture library");
        let typical = library.corners.get_mut("tt").expect("the typical corner");
        typical.minimum_temperature_c = Some(-40.0);
        typical.maximum_temperature_c = Some(125.0);
    }
    narrow.sim_setup.reference_pvt.temperature_celsius = 150.0;

    for (name, state) in [
        ("corners-bound", &mut bound),
        ("corners-draft-selected", &mut draft),
        ("corners-temperature-finding", &mut narrow),
    ] {
        let canvas = raster(state);
        let height = canvas.content_height().max(200);
        let path = directory.join(format!("{name}.png"));
        std::fs::write(&path, canvas.png(height)).expect("write png");
        writeln!(
            report,
            "wrote {} ({}x{height})",
            path.display(),
            canvas.width()
        )
        .ok();
    }
}
