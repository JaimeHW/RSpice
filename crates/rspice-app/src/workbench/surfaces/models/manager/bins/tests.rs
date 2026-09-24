//! What the bin audit counts, and what it refuses to count.
//!
//! A family is one library's cards sharing a base name; a finding is the
//! engine's own positive-area overlap; a shared boundary is the ordinary tiled
//! case and is reported separately from both.

use super::*;

/// The receipt's geometry for one card, on all three axes.
fn geometry(
    length: (f64, f64),
    width: (f64, f64),
    nfin: (f64, f64),
) -> rspice_core::engine::ModelBinCardGeometry {
    let axis = |(min, max): (f64, f64)| rspice_core::engine::ModelBinAxisRange {
        min: Some(min),
        max: Some(max),
    };
    rspice_core::engine::ModelBinCardGeometry {
        length: axis(length),
        width: axis(width),
        nfin: axis(nfin),
    }
}

fn bin_card(
    model: &str,
    declaration_order: usize,
    geometry: rspice_core::engine::ModelBinCardGeometry,
) -> BinCard {
    BinCard {
        model: model.to_owned(),
        device: "NMOS".to_owned(),
        geometry,
        declaration_order,
    }
}

fn bin_family(name: &str, cards: Vec<BinCard>) -> BinFamily {
    BinFamily {
        library: "foundry".to_owned(),
        family: name.to_owned(),
        cards,
    }
}

fn placed(
    element: &str,
    requested: &str,
    selected: &str,
    length: f64,
    width: f64,
    selection: rspice_core::engine::ModelBinSelectionKind,
) -> rspice_core::engine::ModelBinInstanceInspection {
    rspice_core::engine::ModelBinInstanceInspection {
        element: element.to_owned(),
        requested_model: requested.to_owned(),
        selected_model: selected.to_owned(),
        selection,
        match_count: 1,
        length: Some(length),
        width: Some(width),
        nfin: None,
        multiplier: Some(1.0),
    }
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
    let geometry = geometry((1.0e-7, 2.0e-7), (1.0e-7, 2.0e-7), (1.0, 2.0));
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
    // The receipt names the device type; the family table reports that rather
    // than inventing one from the model name.
    assert_eq!(families[0].device(), "NMOS");
    assert_eq!(families[1].family, "pch");
    assert_eq!(families[1].cards.len(), 1);
}

#[test]
fn a_family_whose_cards_disagree_on_device_type_says_so() {
    // One base name selecting between an NMOS and a PMOS card is a defect in
    // the source, and a cell that printed the first card's type would hide it.
    let geometry = geometry((1.0e-7, 2.0e-7), (1.0e-7, 2.0e-7), (1.0, 2.0));
    let mut mixed = bin_family("nch", vec![bin_card("nch.1", 0, geometry)]);
    mixed.cards.push(BinCard {
        device: "PMOS".to_owned(),
        ..bin_card("nch.2", 1, geometry)
    });
    assert_eq!(mixed.device(), "2 device types");
}

#[test]
fn geometry_findings_use_the_engine_s_nfin_axis() {
    let inspection = rspice_core::engine::ModelBinInspection::default();
    let disjoint = bin_family(
        "nch",
        vec![
            bin_card(
                "nch.1",
                0,
                geometry((1.0e-7, 5.0e-7), (1.0e-7, 5.0e-7), (1.0, 2.0)),
            ),
            bin_card(
                "nch.2",
                1,
                geometry((1.0e-7, 5.0e-7), (1.0e-7, 5.0e-7), (3.0, 4.0)),
            ),
        ],
    );
    assert!(
        BinAudit::derive(&[disjoint], &inspection)
            .findings
            .is_empty(),
        "cards overlapping in L/W but disjoint in NFIN are not ambiguous"
    );

    let overlapping = bin_family(
        "nch",
        vec![
            bin_card(
                "nch.1",
                0,
                geometry((1.0e-7, 5.0e-7), (1.0e-7, 5.0e-7), (1.0, 3.0)),
            ),
            bin_card(
                "nch.2",
                1,
                geometry((1.0e-7, 5.0e-7), (1.0e-7, 5.0e-7), (2.0, 4.0)),
            ),
        ],
    );
    let audit = BinAudit::derive(&[overlapping], &inspection);
    assert_eq!(audit.findings.len(), 1);
    // The strip and the map read the same finding, and the region it names is
    // the intersection rather than either whole card.
    assert_eq!(audit.findings[0].length.min, Some(1.0e-7));
    assert_eq!(audit.findings[0].length.max, Some(5.0e-7));
    assert_eq!(audit.tally("foundry · nch").findings, 1);
}

#[test]
fn adjacent_cards_are_a_shared_boundary_and_not_a_finding() {
    // The ordinary tiled family: one card's `lmax` is the next card's `lmin`.
    // Counting that as a fault would report every correct PDK as broken; not
    // counting it at all would leave the "resolving to one card" ratio with no
    // population to be measured against.
    let inspection = rspice_core::engine::ModelBinInspection::default();
    let tiled = bin_family(
        "nch",
        vec![
            bin_card(
                "nch.1",
                0,
                geometry((1.0e-7, 5.0e-7), (1.0e-7, 1.0e-4), (1.0, 8.0)),
            ),
            bin_card(
                "nch.2",
                1,
                geometry((5.0e-7, 1.0e-6), (1.0e-7, 1.0e-4), (1.0, 8.0)),
            ),
        ],
    );
    let audit = BinAudit::derive(&[tiled], &inspection);
    assert!(
        audit.findings.is_empty(),
        "a touching edge encloses no area"
    );
    assert_eq!(audit.boundaries, 1);

    let separated = bin_family(
        "nch",
        vec![
            bin_card(
                "nch.1",
                0,
                geometry((1.0e-7, 5.0e-7), (1.0e-7, 1.0e-4), (1.0, 8.0)),
            ),
            bin_card(
                "nch.2",
                1,
                geometry((9.0e-7, 1.0e-6), (1.0e-7, 1.0e-4), (1.0, 8.0)),
            ),
        ],
    );
    assert_eq!(
        BinAudit::derive(&[separated], &inspection).boundaries,
        0,
        "cards with a gap between them do not touch"
    );
}

#[test]
fn an_instance_settled_by_declaration_order_is_not_resolving_to_one_card() {
    // The ratio the strip reports is what the geometry decided on its own. An
    // instance sitting exactly on a shared edge matched two cards and was
    // settled by source order — deterministic, and not the same claim.
    use rspice_core::engine::ModelBinSelectionKind;

    let geometry = geometry((1.0e-7, 5.0e-7), (1.0e-7, 1.0e-4), (1.0, 8.0));
    let families = [bin_family(
        "nch",
        vec![
            bin_card("nch.1", 0, geometry),
            bin_card("nch.2", 1, geometry),
        ],
    )];
    let inspection = rspice_core::engine::ModelBinInspection {
        cards: Vec::new(),
        instances: vec![
            placed(
                "m1",
                "nch",
                "nch.1",
                2.0e-7,
                1.0e-6,
                ModelBinSelectionKind::FamilyMatch,
            ),
            placed(
                "m2",
                "nch",
                "nch.1",
                5.0e-7,
                1.0e-6,
                ModelBinSelectionKind::SharedBoundary,
            ),
            placed(
                "m3",
                "nch.2",
                "nch.2",
                3.0e-7,
                1.0e-6,
                ModelBinSelectionKind::ExactCard,
            ),
        ],
    };
    let audit = BinAudit::derive(&families, &inspection);
    assert_eq!(audit.instances, 3);
    assert_eq!(audit.shared, 1);
    assert_eq!(audit.one_card, 2);
    assert_eq!(audit.tally("foundry · nch").one_card(), 2);
    // Every instance is indexed back to its family, in receipt order, so the
    // map and the table below it read exactly the same three rows.
    assert_eq!(audit.instances_of("foundry · nch"), [0, 1, 2]);
}

#[test]
fn an_instance_of_an_unrelated_family_is_not_counted_here() {
    use rspice_core::engine::ModelBinSelectionKind;

    let geometry = geometry((1.0e-7, 5.0e-7), (1.0e-7, 1.0e-4), (1.0, 8.0));
    let families = [bin_family("nch", vec![bin_card("nch.1", 0, geometry)])];
    let inspection = rspice_core::engine::ModelBinInspection {
        cards: Vec::new(),
        instances: vec![placed(
            "m9",
            "pch",
            "pch.1",
            2.0e-7,
            1.0e-6,
            ModelBinSelectionKind::FamilyMatch,
        )],
    };
    let audit = BinAudit::derive(&families, &inspection);
    assert_eq!(audit.instances, 0);
    assert!(audit.instances_of("foundry · nch").is_empty());
}

/// A binned family with a real finding and real instances, rendered.
///
/// Run with `--ignored`; the PNG goes to `RSPICE_RASTER_DIR` (default: the
/// system temp directory). The composition is a function of exactly the values
/// handed to [`bins_body`], so the receipt is built here rather than prepared
/// from a design — which is also the only way to raster a family that carries
/// a finding, since a design that reaches one has no receipt to render from.
#[test]
#[ignore = "writes a PNG for a human to look at; run with --ignored"]
fn render_a_populated_bins_page() {
    use rspice_core::engine::ModelBinSelectionKind;

    let tile = |bin: usize| {
        geometry(
            (2.8e-8 * (bin + 1) as f64, 2.8e-8 * (bin + 2) as f64),
            (1.0e-7, 1.0e-4),
            (1.0, 8.0),
        )
    };
    // Six tiled cards, then two narrow fix-up cards dropped inside them — the
    // shape a family acquires when somebody patches one corner of the plane
    // without re-binning the card underneath. Two findings over five clean
    // shared boundaries.
    let mut cards = (0..6)
        .map(|bin| bin_card(&format!("nch.{}", bin + 1), bin, tile(bin)))
        .collect::<Vec<_>>();
    cards.push(bin_card(
        "nch.7",
        6,
        geometry((1.75e-7, 1.9e-7), (1.0e-6, 1.0e-5), (1.0, 8.0)),
    ));
    cards.push(bin_card(
        "nch.8",
        7,
        geometry((5.6e-8, 7.0e-8), (2.0e-5, 8.0e-5), (1.0, 8.0)),
    ));
    let families = vec![
        bin_family("nch", cards),
        bin_family(
            "pch",
            (0..4)
                .map(|bin| bin_card(&format!("pch.{}", bin + 1), bin, tile(bin)))
                .collect(),
        ),
    ];
    let inspection = rspice_core::engine::ModelBinInspection {
        cards: Vec::new(),
        instances: vec![
            placed(
                "m1",
                "nch",
                "nch.1",
                3.0e-8,
                1.2e-6,
                ModelBinSelectionKind::FamilyMatch,
            ),
            placed(
                "m2",
                "nch",
                "nch.2",
                6.5e-8,
                3.2e-6,
                ModelBinSelectionKind::FamilyMatch,
            ),
            placed(
                "m3",
                "nch",
                "nch.3",
                8.4e-8,
                8.0e-6,
                ModelBinSelectionKind::SharedBoundary,
            ),
            placed(
                "m4",
                "nch",
                "nch.4",
                1.3e-7,
                2.0e-6,
                ModelBinSelectionKind::FamilyMatch,
            ),
            placed(
                "m5",
                "nch.5",
                "nch.5",
                1.5e-7,
                6.0e-7,
                ModelBinSelectionKind::ExactCard,
            ),
            placed(
                "m6",
                "nch",
                "nch.6",
                1.9e-7,
                4.0e-5,
                ModelBinSelectionKind::SharedBoundary,
            ),
            placed(
                "m7",
                "nch",
                "nch.2",
                5.6e-8,
                1.0e-5,
                ModelBinSelectionKind::FamilyMatch,
            ),
        ],
    };

    let mut state = AppState::default();
    state.model_library_manager.clear();
    let mut pending = Vec::new();
    let canvas = crate::ui::raster::render(egui::vec2(1180.0, 900.0), |ui, background| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(background))
            .show(ui, |ui| {
                let mut app = ManagerRenderContext {
                    state: &mut state,
                    pending_actions: &mut pending,
                };
                bins_body(ui, &mut app, &inspection, &families);
            });
    });
    write_render(&canvas, "laneD-03-bins-geometry.png");
}

/// The page when the simulator refused this design's inspection outright.
#[test]
#[ignore = "writes a PNG for a human to look at; run with --ignored"]
fn render_the_blocked_bins_page() {
    let mut state = AppState::default();
    state.model_library_manager.clear();
    let mut pending = Vec::new();
    let canvas = crate::ui::raster::render(egui::vec2(1180.0, 620.0), |ui, background| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(background))
            .show(ui, |ui| {
                let mut app = ManagerRenderContext {
                    state: &mut state,
                    pending_actions: &mut pending,
                };
                bins_blocked(
                    ui,
                    &mut app,
                    "MOSFET 'm3' model family 'nch' is ambiguous for its declared geometry: \
                     matching cards 'nch.3' and 'nch.7' overlap across a positive-area bin \
                     region; correct the model-bin bounds before simulation",
                );
            });
    });
    write_render(&canvas, "laneD-03b-bins-blocked.png");
}

/// Write one render where the harness was told to put it, and say where.
fn write_render(canvas: &crate::ui::raster::Canvas, name: &str) {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let path = directory.join(name);
    let height = canvas.content_height().max(200);
    std::fs::write(&path, canvas.png(height)).expect("write png");
    writeln!(
        std::io::stderr(),
        "wrote {} ({}x{height})",
        path.display(),
        canvas.width()
    )
    .ok();
}
