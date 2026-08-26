//! What the bin family list declares, and what its audit compares.
//!
//! A family is one library's cards sharing a base name, the geometry audit
//! reads the engine's NFIN axis, and the list declares the height a row
//! really takes.

use super::*;

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
