//! The model browser against the compiled-in foundation catalog.
//!
//! Which cards an instance is offered is asked of the binding gate itself, so
//! these pins are written over real foundation cards rather than over fixtures
//! a type filter would also pass: the bulk/SOI split and the shadowed copy are
//! the two places the type filter this replaced offered rows the commit then
//! refused.

use super::*;
use crate::state::model_library::ModelLibrary;

/// The compiled-in foundation catalog, which every project starts with.
fn foundation() -> ModelLibraryManager {
    let mut manager = ModelLibraryManager::default();
    manager.load_builtin_models();
    manager
}

fn target(kind: ComponentType, model: Option<&str>, library: Option<&str>) -> ModelTarget {
    ModelTarget {
        reference: "D1".to_owned(),
        kind,
        current: model.map(|model| (model.to_owned(), library.map(str::to_owned))),
        geometry: InstanceGeometry::default(),
    }
}

fn listed(groups: &[Group<'_>]) -> Vec<String> {
    groups
        .iter()
        .flat_map(|group| &group.rows)
        .map(|row| format!("{}/{}", row.library.name, row.entry.name()))
        .collect()
}

/// `L × W` in metres.
fn sized(length: f64, width: f64) -> InstanceGeometry {
    InstanceGeometry {
        length: Some(length),
        width: Some(width),
    }
}

/// A second library that defines the foundation's diode under the same name.
fn vendor_diode() -> ModelLibrary {
    let mut library = ModelLibrary::new("Vendor discretes");
    let mut diode = DeviceModel::new("RSPICE_DIODE", ModelType::Diode);
    diode.spice_type = Some("D".to_owned());
    diode.add_parameter("is", 2.5e-9);
    library.add_model(diode);
    library
}

fn key(library: &str, model: &str) -> ModelKey {
    ModelKey {
        library: library.to_owned(),
        model: model.to_owned(),
    }
}

/// A diode is offered diode cards and nothing else, through the binding
/// gate's own rule rather than a type filter beside it.
#[test]
fn an_instance_is_offered_only_the_cards_it_can_run() {
    let manager = foundation();
    let diode = catalog(
        &manager,
        Some(&target(ComponentType::Diode, None, None)),
        "",
    );
    assert_eq!(
        listed(&diode),
        [
            "RSpice Foundation/RSPICE_DIODE",
            "RSpice Foundation/RSPICE_ZENER"
        ]
    );
}

/// The type filter this replaced offered an SOI device every bulk NMOS card,
/// all of which the commit then refused.
#[test]
fn bulk_and_soi_mosfets_are_offered_disjoint_cards() {
    let manager = foundation();
    let bulk = catalog(&manager, Some(&target(ComponentType::Nmos, None, None)), "");
    let soi = catalog(
        &manager,
        Some(&target(ComponentType::NmosSoi, None, None)),
        "",
    );
    assert_eq!(listed(&bulk), ["RSpice Foundation/RSPICE_NMOS"]);
    assert_eq!(listed(&soi), ["RSpice Foundation/RSPICE_NMOS_SOI"]);
}

#[test]
fn the_browser_without_an_instance_lists_every_loaded_card() {
    let manager = foundation();
    let all = catalog(&manager, None, "");
    assert_eq!(listed(&all).len(), manager.total_model_count());
}

/// Every word narrows, across the name, the family and the library.
#[test]
fn each_filter_word_narrows_the_list() {
    let manager = foundation();
    assert_eq!(
        listed(&catalog(&manager, None, "zener")),
        ["RSpice Foundation/RSPICE_ZENER"]
    );
    assert_eq!(listed(&catalog(&manager, None, "foundation npn")).len(), 2);
    assert_eq!(
        listed(&catalog(&manager, None, "pnp thermal")),
        ["RSpice Foundation/RSPICE_PNP_THERMAL"]
    );
    assert!(catalog(&manager, None, "npn nothing-like-this").is_empty());
}

#[test]
fn names_sort_the_way_a_designer_counts() {
    let mut names = vec!["nch.10", "nch.2", "NCH.1", "Q2N3904", "q2n2222"];
    names.sort_by(|left, right| natural_cmp(left, right));
    assert_eq!(names, ["NCH.1", "nch.2", "nch.10", "q2n2222", "Q2N3904"]);
}

/// A name two libraries define is listed under both, and neither copy can be
/// bound until the project decides which one runs; after it decides, only the
/// other copy is refused.
#[test]
fn a_copy_the_project_does_not_run_is_listed_but_inert() {
    let mut manager = foundation();
    manager.add_library(vendor_diode());
    let diode = target(ComponentType::Diode, None, None);

    let undecided = catalog(&manager, Some(&diode), "rspice_diode");
    let tags: Vec<Option<&str>> = undecided
        .iter()
        .flat_map(|group| &group.rows)
        .map(|row| row.refusal.as_ref().map(|refusal| refusal.tag))
        .collect();
    assert_eq!(tags, [Some("unresolved"), Some("unresolved")]);

    manager
        .resolve_definition_provider(
            ModelConsumerScope::PrimitiveModel,
            "RSPICE_DIODE",
            "RSpice Foundation",
            "test",
        )
        .expect("resolve");
    let decided = catalog(&manager, Some(&diode), "rspice_diode");
    let refusals: Vec<(String, Option<&str>)> = decided
        .iter()
        .flat_map(|group| &group.rows)
        .map(|row| {
            (
                row.library.name.clone(),
                row.refusal.as_ref().map(|refusal| refusal.tag),
            )
        })
        .collect();
    assert_eq!(
        refusals,
        [
            ("RSpice Foundation".to_owned(), None),
            ("Vendor discretes".to_owned(), Some("shadowed")),
        ]
    );
}

/// The arrow keys walk the rows a reader can bind and never land on one the
/// primary would refuse.
#[test]
fn the_arrow_keys_skip_rows_that_cannot_be_bound() {
    let mut manager = foundation();
    manager.add_library(vendor_diode());
    manager
        .resolve_definition_provider(
            ModelConsumerScope::PrimitiveModel,
            "RSPICE_DIODE",
            "RSpice Foundation",
            "test",
        )
        .expect("resolve");
    let groups = catalog(
        &manager,
        Some(&target(ComponentType::Diode, None, None)),
        "",
    );
    let first = stepped(&groups, None, 1).expect("a first row");
    assert_eq!(first, key("RSpice Foundation", "RSPICE_DIODE"));
    let next = stepped(&groups, Some(&first), 1).expect("a second row");
    assert_eq!(next, key("RSpice Foundation", "RSPICE_ZENER"));
    // The vendor's shadowed copy is the last row; the walk stops short of it.
    assert_eq!(stepped(&groups, Some(&next), 1), Some(next.clone()));
    assert_eq!(stepped(&groups, Some(&next), -5), Some(first));
}

#[test]
fn a_rows_figures_lead_with_what_tells_two_cards_of_its_family_apart() {
    let manager = foundation();
    let library = manager
        .get_library("RSpice Foundation")
        .expect("foundation");
    let zener = library.get_model("RSPICE_ZENER").expect("zener");
    let row = Entry::Card(zener);
    assert_eq!(meta_line(&row, false), ["IS 10f", "N 1.2", "BV 5.6"]);
    assert_eq!(meta_line(&row, true)[0], "Diode");

    let names: Vec<String> = parameters(zener)
        .into_iter()
        .map(|parameter| parameter.name)
        .collect();
    assert_eq!(&names[..5], ["IS", "N", "BV", "RS", "CJO"]);
    assert_eq!(names.len(), zener.parameters.len());

    let bjt = library
        .get_model("RSPICE_NPN_THERMAL")
        .expect("thermal npn");
    assert!(
        parameters(bjt)
            .iter()
            .all(|parameter| parameter.name != "LEVEL"),
        "the identity line states the level, so the grid does not"
    );
    assert_eq!(level_label(bjt).as_deref(), Some("VBIC"));
}

/// A value past the engineering ladder is an exponent, not a pile of digits
/// in front of the top prefix.
#[test]
fn a_value_past_the_ladder_is_spelled_as_an_exponent() {
    assert_eq!(spelled(1.7e17), "1.7e17");
    assert_eq!(spelled(2.29e-18), "2.29e-18");
    assert_eq!(spelled(1.12e5), "112k");
    assert_eq!(spelled(-0.0234), "-23.4m");
}

fn kit_manager() -> ModelLibraryManager {
    let mut manager = foundation();
    manager.add_library(foundry_kit());
    manager
}

fn family_row<'a>(groups: &'a [Group<'a>], name: &str) -> &'a Row<'a> {
    groups
        .iter()
        .flat_map(|group| &group.rows)
        .find(|row| row.entry.name() == name)
        .expect("listed row")
}

/// A kit's bins are one row named for their family, beside the kit's cards
/// that are no bin, and a lone numbered card is a family of one. A bin's own
/// name still finds its family.
#[test]
fn a_binned_family_is_one_row_named_for_the_family() {
    let manager = kit_manager();
    let nmos = target(ComponentType::Nmos, None, None);
    let rows = catalog(&manager, Some(&nmos), "");
    assert_eq!(
        listed(&rows),
        [
            "RSpice Foundation/RSPICE_NMOS",
            "gpdk090/nch",
            "gpdk090/nch_25"
        ]
    );
    assert_eq!(
        family_row(&rows, "nch").meta,
        [
            "BSIM4 4.5",
            "12 bins",
            "L 100nm\u{2013}20µm",
            "W 120nm\u{2013}100µm"
        ]
    );
    assert_eq!(
        listed(&catalog(&manager, Some(&nmos), "nch.11")),
        ["gpdk090/nch"]
    );
    let pmos = catalog(
        &manager,
        Some(&target(ComponentType::Pmos, None, None)),
        "gpdk",
    );
    assert_eq!(listed(&pmos), ["gpdk090/pch"]);
    assert_eq!(family_row(&pmos, "pch").meta[1], "1 bin");
}

#[test]
fn a_bin_window_reads_its_unit_once() {
    assert_eq!(window_label(Some(0.1e-6), Some(0.5e-6)), "100\u{2013}500nm");
    assert_eq!(window_label(Some(0.5e-6), Some(2e-6)), "500nm\u{2013}2µm");
    assert_eq!(window_label(Some(1e-6), None), "min 1µm");
    assert_eq!(window_label(None, None), "\u{2014}");
}

/// Opened on an instance pinned to one bin, the browser picks its family and
/// says which bin the instance's size runs; Select binds the family's name.
#[test]
fn a_pinned_bin_is_replaced_by_its_family_and_the_size_names_the_bin() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let manager = kit_manager();
    let mut state = ModelBrowserState::default();
    state.open_for_instance(
        "M3",
        ComponentType::Nmos,
        Some("nch.2".to_owned()),
        Some("gpdk090".to_owned()),
        sized(1e-6, 5e-6),
    );
    let _ = frame(&ctx, &mut state, &manager, Vec::new());
    assert_eq!(state.pick, Some(key("gpdk090", "nch")));

    let target = state.target.clone().expect("target");
    let library = manager.get_library("gpdk090").expect("kit");
    let family = entry(library, "nch").expect("family");
    let status = footer(&manager, Some(&target), Some((library, &family)), &[], "");
    assert!(status.can_select);
    assert!(matches!(status.tone, DialogHintTone::Neutral));
    // L 1µm falls in the second L window and W 5µm in the second W window.
    assert_eq!(
        status.hint,
        "Replaces nch.2 on M3 with nch from gpdk090; its L and W run nch.5"
    );
    assert_eq!(family.card(target.geometry).name, "nch.5");

    let mut result = ModelBrowserResult::None;
    for _ in 0..3 {
        result = frame(&ctx, &mut state, &manager, vec![press(Key::Enter)]);
        if result != ModelBrowserResult::None {
            break;
        }
    }
    assert_eq!(
        result,
        ModelBrowserResult::Selected {
            library: "gpdk090".to_owned(),
            model: "nch".to_owned(),
        }
    );
}

/// A size no bin holds is what the simulator refuses, at the run: the footer
/// says so, in the warning tone, and Select still binds the family, since the
/// size is what has to change. The sheet falls back to the first bin.
#[test]
fn a_size_no_bin_holds_is_warned_of_before_the_run_refuses_it() {
    let manager = kit_manager();
    let library = manager.get_library("gpdk090").expect("kit");
    let family = entry(library, "nch").expect("family");
    let mut uncovered = target(ComponentType::Nmos, Some("nch"), Some("gpdk090"));
    uncovered.reference = "M3".to_owned();
    uncovered.geometry = sized(25e-6, 5e-6);
    let status = footer(
        &manager,
        Some(&uncovered),
        Some((library, &family)),
        &[],
        "",
    );
    assert!(matches!(status.tone, DialogHintTone::Warn));
    assert_eq!(
        status.hint,
        "M3 already uses nch; no nch bin holds L 25µm, W 5µm, so a run refuses M3"
    );
    assert_eq!(family.card(uncovered.geometry).name, "nch.1");

    uncovered.current = None;
    let status = footer(
        &manager,
        Some(&uncovered),
        Some((library, &family)),
        &[],
        "",
    );
    assert!(status.can_select);
    assert!(
        status
            .hint
            .starts_with("Binds nch from gpdk090 to M3; no nch bin")
    );
}

/// A family competes for its name as a card does: a second library's card of
/// that name leaves both unbindable until the project decides which one runs.
#[test]
fn a_family_another_library_names_as_a_card_waits_for_the_projects_decision() {
    let mut manager = kit_manager();
    let mut vendor = ModelLibrary::new("vendor");
    let mut card = DeviceModel::new("nch", ModelType::Nmos).with_level(ModelLevel::Bsim4);
    card.spice_type = Some("NMOS".to_owned());
    vendor.add_model(card);
    manager.add_library(vendor);
    let nmos = target(ComponentType::Nmos, None, None);
    let tags = |manager: &ModelLibraryManager| {
        catalog(manager, Some(&nmos), "nch")
            .iter()
            .flat_map(|group| &group.rows)
            .filter(|row| row.entry.name() == "nch")
            .map(|row| {
                (
                    row.library.name.clone(),
                    row.refusal.as_ref().map(|refusal| refusal.tag),
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(
        tags(&manager),
        [
            ("gpdk090".to_owned(), Some("unresolved")),
            ("vendor".to_owned(), Some("unresolved")),
        ]
    );
    manager
        .resolve_definition_provider(ModelConsumerScope::PrimitiveModel, "nch", "gpdk090", "test")
        .expect("the family is a provider of its name");
    assert_eq!(
        tags(&manager),
        [
            ("gpdk090".to_owned(), None),
            ("vendor".to_owned(), Some("shadowed")),
        ]
    );
}

fn press(key: Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    }
}

fn frame(
    ctx: &Context,
    state: &mut ModelBrowserState,
    manager: &ModelLibraryManager,
    events: Vec<egui::Event>,
) -> ModelBrowserResult {
    let mut result = ModelBrowserResult::None;
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            vec2(1_100.0, 850.0),
        )),
        events,
        ..Default::default()
    };
    let _ = ctx.run_ui(input, |ui| {
        result = render_model_browser(ui.ctx(), state, manager);
    });
    result
}

/// Opening picks the model the instance runs, which Select would not change;
/// moving the pick says what Select will do and binds it.
#[test]
fn the_pick_opens_on_the_model_in_use_and_select_binds_a_different_one() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let manager = foundation();
    let mut state = ModelBrowserState::default();
    state.open_for_instance(
        "D1",
        ComponentType::Diode,
        Some("RSPICE_DIODE".to_owned()),
        Some("RSpice Foundation".to_owned()),
        InstanceGeometry::default(),
    );

    assert_eq!(
        frame(&ctx, &mut state, &manager, Vec::new()),
        ModelBrowserResult::None
    );
    assert_eq!(state.pick, Some(key("RSpice Foundation", "RSPICE_DIODE")));
    let target = state.target.clone().expect("target");
    let library = manager
        .get_library("RSpice Foundation")
        .expect("foundation");
    let card = |name: &str| Entry::Card(library.get_model(name).expect("card"));
    let unchanged = footer(
        &manager,
        Some(&target),
        Some((library, &card("RSPICE_DIODE"))),
        &[],
        "",
    );
    assert!(!unchanged.can_select);
    assert_eq!(unchanged.hint, "D1 already uses RSPICE_DIODE");

    let _ = frame(&ctx, &mut state, &manager, vec![press(Key::ArrowDown)]);
    assert_eq!(state.pick, Some(key("RSpice Foundation", "RSPICE_ZENER")));
    let changed = footer(
        &manager,
        Some(&target),
        Some((library, &card("RSPICE_ZENER"))),
        &[],
        "",
    );
    assert!(changed.can_select);
    assert_eq!(
        changed.hint,
        "Replaces RSPICE_DIODE on D1 with RSPICE_ZENER from RSpice Foundation"
    );

    let mut result = ModelBrowserResult::None;
    for _ in 0..3 {
        result = frame(&ctx, &mut state, &manager, vec![press(Key::Enter)]);
        if result != ModelBrowserResult::None {
            break;
        }
    }
    assert_eq!(
        result,
        ModelBrowserResult::Selected {
            library: "RSpice Foundation".to_owned(),
            model: "RSPICE_ZENER".to_owned(),
        }
    );
    assert!(!state.open);
}

/// Typing goes to the filter wherever focus is, and Escape cancels the browser
/// as it cancels every dialog, filter or no filter.
#[test]
fn typing_narrows_and_escape_cancels() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let manager = foundation();
    let mut state = ModelBrowserState::default();
    state.open_for_instance(
        "D1",
        ComponentType::Diode,
        None,
        None,
        InstanceGeometry::default(),
    );
    let _ = frame(&ctx, &mut state, &manager, Vec::new());
    let _ = frame(
        &ctx,
        &mut state,
        &manager,
        vec![egui::Event::Text("zen".to_owned())],
    );
    assert_eq!(state.filter, "zen");

    let mut result = ModelBrowserResult::None;
    for events in [vec![press(Key::Escape)], Vec::new(), Vec::new()] {
        result = frame(&ctx, &mut state, &manager, events);
        if result != ModelBrowserResult::None {
            break;
        }
    }
    assert_eq!(result, ModelBrowserResult::Cancelled);
    assert!(!state.open);
}

/// The L and W windows of the foundry kit's twelve `nch` bins, in metres: a
/// four-by-three grid, numbered across W within each L.
const KIT_LENGTHS: [(f64, f64); 4] = [
    (0.1e-6, 0.5e-6),
    (0.5e-6, 2e-6),
    (2e-6, 10e-6),
    (10e-6, 20e-6),
];
const KIT_WIDTHS: [(f64, f64); 3] = [(0.12e-6, 1e-6), (1e-6, 10e-6), (10e-6, 100e-6)];

/// A library shaped like a foundry kit: a twelve-bin BSIM4 family with a full
/// parameter set, an unbinned thick-oxide card, and a PMOS family of one.
fn foundry_kit() -> ModelLibrary {
    let mut library = ModelLibrary::new("gpdk090");
    let windows = KIT_LENGTHS
        .iter()
        .flat_map(|length| KIT_WIDTHS.iter().map(move |width| (*length, *width)));
    for (index, ((l_min, l_max), (w_min, w_max))) in windows.enumerate() {
        let bin = index + 1;
        let mut card = DeviceModel::new(format!("nch.{bin}"), ModelType::Nmos)
            .with_level(ModelLevel::Bsim4)
            .with_geometry(l_min, l_max, w_min, w_max);
        card.spice_type = Some("NMOS".to_owned());
        card.spice_level = Some(54);
        card.model_version = Some(4.5);
        card.description = "1.2 V core NMOS, standard threshold".to_owned();
        card.file_path = Some(std::path::PathBuf::from(
            "C:/pdk/gpdk090/models/spectre/gpdk090.scs",
        ));
        card.source_line = Some(118 + 96 * bin);
        card.add_parameter("vth0", 0.3923 + 0.004 * bin as f64);
        for (name, value) in [
            ("toxe", 2.05e-9),
            ("u0", 0.0352),
            ("k1", 0.4421),
            ("k2", -0.0234),
            ("dvt0", 2.21),
            ("dvt1", 0.53),
            ("vsat", 1.12e5),
            ("ua", -1.31e-9),
            ("ub", 2.29e-18),
            ("uc", 7.4e-11),
            ("rdsw", 150.0),
            ("pclm", 1.23),
            ("pdiblc1", 0.39),
            ("pdiblc2", 8.6e-3),
            ("eta0", 0.08),
            ("nfactor", 1.35),
            ("cgso", 2.1e-10),
            ("cgdo", 2.1e-10),
            ("cj", 1.03e-3),
            ("cjsw", 1.3e-10),
            ("mj", 0.38),
            ("xj", 1.5e-7),
            ("ndep", 1.7e17),
            ("lint", 6.5e-9),
            ("wint", 5e-9),
            ("level", 54.0),
            ("version", 4.5),
        ] {
            card.add_parameter(name, value);
        }
        library.add_model(card);
    }
    let mut thick = DeviceModel::new("nch_25", ModelType::Nmos)
        .with_level(ModelLevel::Bsim4)
        .with_geometry(0.28e-6, 20e-6, 0.4e-6, 100e-6);
    thick.spice_type = Some("NMOS".to_owned());
    thick.description = "2.5 V thick-oxide I/O NMOS".to_owned();
    thick.add_parameter("vth0", 0.52);
    thick.add_parameter("toxe", 5.4e-9);
    library.add_model(thick);
    let mut pmos = DeviceModel::new("pch.1", ModelType::Pmos).with_level(ModelLevel::Bsim4);
    pmos.spice_type = Some("PMOS".to_owned());
    pmos.add_parameter("vth0", -0.37);
    library.add_model(pmos);
    library
}

/// Run `pass` until the modal's fade has finished, then rasterize it.
#[cfg(not(target_arch = "wasm32"))]
fn settled(
    mode: crate::ui::tokens::Mode,
    size: egui::Vec2,
    mut pass: impl FnMut(&Context),
) -> crate::ui::raster::Canvas {
    let ctx = Context::default();
    ctx.set_os(egui::os::OperatingSystem::Nix);
    crate::ui::Theme {
        mode,
        ..crate::ui::Theme::default()
    }
    .apply(&ctx);
    let input = || crate::ui::raster::display_input(size, 2.0);
    let _ = ctx.run_ui(input(), |ui| pass(ui.ctx()));
    let visible_at = ctx.input(|input| input.time) + f64::from(ctx.global_style().animation_time);
    for _ in 0..240 {
        let _ = ctx.run_ui(input(), |ui| pass(ui.ctx()));
        if ctx.input(|input| input.time) > visible_at {
            break;
        }
    }
    let output = ctx.run_ui(input(), |ui| pass(ui.ctx()));
    crate::ui::raster::rasterize(&ctx, size, &output)
}

/// Write the browser's shapes to PNGs for a human to look at.
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn print_model_browser_renders_for_review() {
    use crate::ui::tokens::Mode;
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();

    let no_size = InstanceGeometry::default();
    let scenes: [(&str, Mode, egui::Vec2, ModelBrowserState); 8] = [
        ("diode-dark", Mode::Dark, vec2(1100.0, 760.0), {
            let mut state = ModelBrowserState::default();
            state.open_for_instance(
                "D1",
                ComponentType::Diode,
                Some("RSPICE_DIODE".to_owned()),
                Some("RSpice Foundation".to_owned()),
                no_size,
            );
            state.pick = Some(key("RSpice Foundation", "RSPICE_ZENER"));
            state.seeded = true;
            state
        }),
        ("diode-light", Mode::Light, vec2(1100.0, 760.0), {
            let mut state = ModelBrowserState::default();
            state.open_for_instance("D1", ComponentType::Diode, None, None, no_size);
            state.pick = Some(key("RSpice Foundation", "RSPICE_DIODE"));
            state.seeded = true;
            state
        }),
        // Pinned to one bin: the browser opens on the family, which Select
        // binds in its place, with the bin the instance's size selects marked.
        ("nmos-pdk", Mode::Dark, vec2(1100.0, 760.0), {
            let mut state = ModelBrowserState::default();
            state.open_for_instance(
                "M3",
                ComponentType::Nmos,
                Some("nch.2".to_owned()),
                Some("gpdk090".to_owned()),
                sized(1e-6, 5e-6),
            );
            state
        }),
        ("nch-family-light", Mode::Light, vec2(1100.0, 760.0), {
            let mut state = ModelBrowserState::default();
            state.open_for_instance(
                "M3",
                ComponentType::Nmos,
                Some("nch".to_owned()),
                Some("gpdk090".to_owned()),
                sized(4e-6, 20e-6),
            );
            state
        }),
        ("nch-family-uncovered", Mode::Dark, vec2(1100.0, 760.0), {
            let mut state = ModelBrowserState::default();
            state.open_for_instance("M3", ComponentType::Nmos, None, None, sized(25e-6, 5e-6));
            state.pick = Some(key("gpdk090", "nch"));
            state.seeded = true;
            state
        }),
        ("browse-all", Mode::Dark, vec2(1100.0, 760.0), {
            let mut state = ModelBrowserState {
                open: true,
                ..ModelBrowserState::default()
            };
            state.pick = Some(key("RSpice Foundation", "RSPICE_NPN_THERMAL"));
            state.seeded = true;
            state
        }),
        ("filter-empty", Mode::Dark, vec2(1100.0, 760.0), {
            let mut state = ModelBrowserState::default();
            state.open_for_instance("Q1", ComponentType::NpnBjt, None, None, no_size);
            state.filter = "bsim".to_owned();
            state
        }),
        ("tablet", Mode::Light, vec2(744.0, 900.0), {
            let mut state = ModelBrowserState::default();
            state.open_for_instance(
                "M3",
                ComponentType::Nmos,
                Some("nch.2".to_owned()),
                Some("gpdk090".to_owned()),
                sized(1e-6, 5e-6),
            );
            state
        }),
    ];
    for (stem, mode, size, mut state) in scenes {
        let mut manager = foundation();
        if !stem.starts_with("diode") && stem != "browse-all" {
            manager.add_library(foundry_kit());
        }
        let canvas = settled(mode, size, |ctx| {
            let _ = render_model_browser(ctx, &mut state, &manager);
        });
        let path = directory.join(format!("model-browser-{stem}.png"));
        std::fs::write(&path, canvas.png(canvas.height())).expect("write render");
        writeln!(
            report,
            "{} {}x{}",
            path.display(),
            canvas.width(),
            canvas.height()
        )
        .expect("write render report");
    }
}
