//! Renames cascade, deletions are transactional, and trust is never implied.
//!
//! A layer or purpose rename has to reach every physical identity that
//! references it, and a referenced deletion either completes or does not
//! happen. The surface cases hold the honest empty state: real actions, a
//! stated trust boundary, and provisioning still reachable from an empty
//! registry rather than a dead end.

use super::*;

fn fixture_package() -> ValidatedPdkTechnologyPackage {
    let (bytes, trust, authority) = crate::state::pdk_config::signed_technology_test_fixture();
    let mut config = crate::state::pdk_config::PdkConfig::default();
    config.publisher_trust_store = trust;
    config
        .technology_registry
        .install_archive_bytes(
            &bytes,
            &config.publisher_trust_store,
            &authority,
            "install authoring fixture",
        )
        .expect("install fixture");
    config.technology_registry.validated_packages()[0].clone()
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn background_package_candidate_is_complete_and_leaves_base_unchanged() {
    let (bytes, trust, authority) = crate::state::pdk_config::signed_technology_test_fixture();
    let mut base = crate::state::pdk_config::PdkConfig::default();
    base.publisher_trust_store = trust;
    let before = base.clone();

    let candidate =
        prepare_native_package_import(&base, &bytes, &authority, "background import test")
            .expect("background candidate validates");

    assert_eq!(base, before);
    assert_eq!(candidate.package_id, "demo180");
    assert_eq!(candidate.revision, "2.3.1");
    assert_eq!(candidate.sequence, 1);
    assert_eq!(
        candidate
            .config
            .technology_registry
            .validated_packages()
            .len(),
        1
    );
}

#[test]
fn layer_rename_cascades_through_every_physical_identity_reference() {
    let package = fixture_package();
    let mut manifest = package.manifest().clone();
    manifest.layer_aliases.push(PdkLayerAlias {
        alias: "m1_draw".to_owned(),
        layer: "metal1".to_owned(),
        purpose: "drawing".to_owned(),
    });
    let edge = manifest.connectivity[0].clone();
    manifest.vias.push(default_via_for_edge(&manifest, &edge));
    let index = manifest
        .layers
        .iter()
        .position(|layer| layer.name == "metal1")
        .unwrap();
    manifest.layers[index].name = "metal_top".to_owned();

    cascade_layer_rename(&mut manifest, "metal1", index);

    assert!(
        manifest
            .stream_map
            .iter()
            .filter(|entry| entry.layer == "metal_top")
            .count()
            >= 2
    );
    assert_eq!(manifest.connectivity[0].to_layer, "metal_top");
    assert_eq!(manifest.vias[0].upper_layer, "metal_top");
    assert_eq!(manifest.layer_aliases[0].layer, "metal_top");
}

#[test]
fn purpose_rename_cascades_and_referenced_deletion_is_transactional() {
    let package = fixture_package();
    let mut manifest = package.manifest().clone();
    manifest.layer_aliases.push(PdkLayerAlias {
        alias: "m1_draw".to_owned(),
        layer: "metal1".to_owned(),
        purpose: "drawing".to_owned(),
    });
    manifest.extraction.push(PdkExtractionContract {
        contract_id: "metal1-purpose-cascade".to_owned(),
        rule_artifact_path: "models/demo.lib".to_owned(),
        quantities: vec![PdkExtractionQuantity::Resistance],
        layer_purposes: vec![PdkLayerPurposeRef {
            layer: "metal1".to_owned(),
            purpose: "drawing".to_owned(),
        }],
        qualification_vectors: Vec::new(),
    });
    let index = manifest
        .layers
        .iter()
        .position(|layer| layer.name == "metal1")
        .expect("metal1 layer");

    apply_layer_purpose_edit(
        &mut manifest,
        index,
        vec!["artwork".to_owned(), "pin".to_owned()],
    )
    .expect("one-for-one purpose rename cascades");
    assert_eq!(manifest.layer_aliases[0].purpose, "artwork");
    assert!(
        manifest
            .stream_map
            .iter()
            .any(|mapping| { mapping.layer == "metal1" && mapping.purpose == "artwork" })
    );
    assert_eq!(manifest.extraction[0].layer_purposes[0].purpose, "artwork");

    let before = manifest.clone();
    let error = apply_layer_purpose_edit(&mut manifest, index, vec!["pin".to_owned()])
        .expect_err("referenced purpose deletion must be blocked");
    assert!(error.contains("still referenced"), "{error}");
    assert_eq!(
        manifest, before,
        "a rejected edit cannot partially mutate refs"
    );
}

#[test]
fn default_via_layers_exclude_marker_and_cut_endpoints() {
    let package = fixture_package();
    let mut manifest = package.manifest().clone();
    manifest.layers[0].kind = PdkLayerKind::Marker;
    assert!(default_via_layers(&manifest).is_none());
    manifest.layers[0].kind = PdkLayerKind::Active;
    let (lower, cut, upper) = default_via_layers(&manifest).expect("legal transition");
    assert_eq!(lower, "active");
    assert_eq!(cut, "cont");
    assert_eq!(upper, "metal1");
}

#[test]
fn layer_and_via_helpers_preserve_stream_and_connectivity_completeness() {
    let package = fixture_package();
    let mut manifest = package.manifest().clone();
    let original_layers = manifest.layers.len();
    let original_maps = manifest.stream_map.len();
    add_draft_layer(&mut manifest);
    assert_eq!(manifest.layers.len(), original_layers + 1);
    assert_eq!(manifest.stream_map.len(), original_maps + 1);

    let edge = manifest.connectivity[0].clone();
    let mut via = default_via_for_edge(&manifest, &edge);
    via.upper_layer = manifest.layers.last().unwrap().name.clone();
    manifest.vias.push(via.clone());
    synchronize_via_connectivity(&mut manifest);
    assert!(manifest.connectivity.iter().any(|candidate| {
        candidate.from_layer == via.lower_layer
            && candidate.through_layer == via.cut_layer
            && candidate.to_layer == via.upper_layer
    }));
}

#[test]
fn installed_revision_exposes_unsigned_authoring_entrypoint() {
    let mut app = RSpiceApp::test_instance();
    let (bytes, trust, authority) = crate::state::pdk_config::signed_technology_test_fixture();
    app.state.pdk_config = crate::state::pdk_config::PdkConfig::default();
    app.state.pdk_config.publisher_trust_store = trust;
    app.state
        .pdk_config
        .technology_registry
        .install_archive_bytes(
            &bytes,
            &app.state.pdk_config.publisher_trust_store,
            &authority,
            "install authoring surface fixture",
        )
        .expect("install package");
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_100.0, 1_200.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
        },
    );
    let accessibility = output
        .platform_output
        .accesskit_update
        .expect("PDK accessibility tree");
    let labels = accessibility
        .nodes
        .iter()
        .filter_map(|(_, node)| node.label())
        .collect::<Vec<_>>();
    assert!(labels.contains(&"Fork selected revision into draft"));
    assert!(labels.contains(&"Vias"));
}

#[test]
fn empty_surface_exposes_real_actions_and_honest_trust_boundary() {
    let mut app = RSpiceApp::test_instance();
    app.state.pdk_config = crate::state::pdk_config::PdkConfig::default();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_100.0, 760.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
        },
    );
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("PDK accessibility tree")
        .nodes;
    let labels = nodes
        .iter()
        .filter_map(|(_, node)| node.label())
        .collect::<Vec<_>>();
    assert!(labels.contains(&"PDK technology administration"));
    assert!(labels.iter().any(|label| {
        label.contains("Import signed package") && label.contains("no publisher trust keys")
    }));
    assert!(labels.contains(&"Revalidate installed packages"));
    assert!(labels.iter().any(|label| {
        label.contains("Compare selected signed PDK revision")
            && label.contains("install another trusted revision")
    }));
}

#[test]
fn phone_header_stacks_runtime_status_below_description_without_overlap() {
    let mut app = RSpiceApp::test_instance();
    app.state.pdk_config = crate::state::pdk_config::PdkConfig::default();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(390.0, 844.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
        },
    );
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("PDK phone accessibility tree")
        .nodes;
    let description = nodes
        .iter()
        .find(|(_, node)| {
            node.label().is_some_and(|label| {
                label.contains(
                    "Validate signed technology packages, inspect exact physical resources",
                )
            })
        })
        .and_then(|(_, node)| node.bounds())
        .unwrap_or_else(|| {
            panic!(
                "PDK header description bounds; labels={:?}",
                nodes
                    .iter()
                    .filter_map(|(_, node)| node.label())
                    .collect::<Vec<_>>()
            )
        });
    let status = nodes
        .iter()
        .find(|(_, node)| node.label() == Some("NO ACTIVE BINDING"))
        .and_then(|(_, node)| node.bounds())
        .expect("PDK runtime status bounds");

    assert!(
        status.y0 >= description.y1 - 1.0,
        "compact runtime status overlaps the header description: description={description:?}, status={status:?}"
    );
}

#[test]
fn empty_registry_still_exposes_accessible_trust_root_provisioning() {
    let mut app = RSpiceApp::test_instance();
    app.state.pdk_config = crate::state::pdk_config::PdkConfig::default();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    ctx.data_mut(|data| {
        data.insert_temp(
            egui::Id::new(VIEW_STATE_ID),
            AdminViewState {
                section: AdminSection::TrustAudit,
                ..AdminViewState::default()
            },
        );
    });
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_100.0, 760.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
        },
    );
    let accessibility = output
        .platform_output
        .accesskit_update
        .expect("PDK accessibility tree");
    let labels = accessibility
        .nodes
        .iter()
        .filter_map(|(_, node)| node.label())
        .collect::<Vec<_>>();
    assert!(labels.iter().any(|label| {
        label.contains("Provision publisher key") && label.contains("actor, authority, and reason")
    }));
    assert!(labels.contains(&"Copy complete audit JSON"));
}

#[test]
fn display_profile_editor_is_accessible_at_phone_width_and_package_bound() {
    let mut app = RSpiceApp::test_instance();
    let (bytes, trust, authority) = crate::state::pdk_config::signed_technology_test_fixture();
    app.state.pdk_config = crate::state::pdk_config::PdkConfig::default();
    app.state.pdk_config.publisher_trust_store = trust;
    app.state
        .pdk_config
        .technology_registry
        .install_archive_bytes(
            &bytes,
            &app.state.pdk_config.publisher_trust_store,
            &authority,
            "install display editor fixture",
        )
        .expect("install package");
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    ctx.data_mut(|data| {
        data.insert_temp(
            egui::Id::new(VIEW_STATE_ID),
            AdminViewState {
                section: AdminSection::Display,
                actor_id: authority.actor_id,
                authority_id: authority.authority_id,
                reason: "publish reviewed display profile".to_owned(),
                ..AdminViewState::default()
            },
        );
    });

    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(430.0, 900.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
        },
    );
    let accessibility = output
        .platform_output
        .accesskit_update
        .expect("PDK display accessibility tree");
    let labels = accessibility
        .nodes
        .iter()
        .filter_map(|(_, node)| node.label())
        .collect::<Vec<_>>();

    assert!(labels.contains(&"PDK technology administration"));
    assert!(labels.contains(&"New from signed defaults"));
    assert!(labels.contains(&"Reset draft to signed colors"));
    assert!(labels.contains(&"Save and activate immutable display-profile revision"));
    assert!(labels.iter().any(|label| label.starts_with("Visible ")));
    assert!(labels.iter().any(|label| label.starts_with("Selectable ")));
    assert!(
        labels
            .iter()
            .any(|label| label.starts_with("Screen color for "))
    );
    assert!(
        labels
            .iter()
            .any(|label| label.starts_with("Print fill for "))
    );
}

#[test]
fn revision_comparison_fails_closed_at_phone_width_without_a_second_trusted_revision() {
    let mut app = RSpiceApp::test_instance();
    let (bytes, trust, authority) = crate::state::pdk_config::signed_technology_test_fixture();
    app.state.pdk_config = crate::state::pdk_config::PdkConfig::default();
    app.state.pdk_config.publisher_trust_store = trust;
    app.state
        .pdk_config
        .technology_registry
        .install_archive_bytes(
            &bytes,
            &app.state.pdk_config.publisher_trust_store,
            &authority,
            "install comparison fixture",
        )
        .expect("install package");
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    ctx.data_mut(|data| {
        data.insert_temp(
            egui::Id::new(VIEW_STATE_ID),
            AdminViewState {
                section: AdminSection::Compare,
                ..AdminViewState::default()
            },
        );
    });

    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(430.0, 900.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
        },
    );
    let accessibility = output
        .platform_output
        .accesskit_update
        .expect("PDK comparison accessibility tree");
    let labels = accessibility
        .nodes
        .iter()
        .filter_map(|(_, node)| node.label())
        .collect::<Vec<_>>();

    assert!(labels.iter().any(|label| {
        label.contains("Compare selected signed PDK revision")
            && label.contains("install another trusted revision")
    }));
}

#[test]
fn revision_comparison_table_is_complete_and_accessible_at_phone_width() {
    let mut app = RSpiceApp::test_instance();
    let (baseline, candidate, trust, authority) =
        crate::state::pdk_config::signed_technology_diff_test_fixture();
    app.state.pdk_config = crate::state::pdk_config::PdkConfig::default();
    app.state.pdk_config.publisher_trust_store = trust;
    app.state
        .pdk_config
        .technology_registry
        .install_archive_bytes(
            &baseline,
            &app.state.pdk_config.publisher_trust_store,
            &authority,
            "install comparison baseline",
        )
        .expect("install baseline");
    app.state
        .pdk_config
        .technology_registry
        .install_archive_bytes(
            &candidate,
            &app.state.pdk_config.publisher_trust_store,
            &authority,
            "install comparison candidate",
        )
        .expect("install candidate");
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    ctx.data_mut(|data| {
        data.insert_temp(
            egui::Id::new(VIEW_STATE_ID),
            AdminViewState {
                section: AdminSection::Compare,
                selected: Some(("demo180".to_owned(), "2.4.0".to_owned())),
                compare_against: Some(("demo180".to_owned(), "2.3.1".to_owned())),
                ..AdminViewState::default()
            },
        );
    });

    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(430.0, 900.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
        },
    );
    let accessibility = output
        .platform_output
        .accesskit_update
        .expect("PDK comparison accessibility tree");
    let nodes = &accessibility.nodes;
    let labels = nodes
        .iter()
        .filter_map(|(_, node)| node.label())
        .collect::<Vec<_>>();

    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Table
            && node.label() == Some("Signed PDK revision differences")
    }));
    assert_eq!(
        nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::ColumnHeader)
            .count(),
        6
    );
    assert!(
        nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::Row)
            .count()
            >= 4
    );
    assert!(
        nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::Cell)
            .count()
            >= 18
    );
    assert!(labels.contains(&"Copy exact signed PDK revision comparison as JSON"));
    assert!(labels.contains(&"active"));
    assert!(labels.contains(&"signed archive digest"));
}

#[test]
fn project_callback_workflow_executes_exact_pin_and_exposes_durable_receipt() {
    let mut app = RSpiceApp::test_instance();
    app.state.provision_test_project_technology_contract();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut view = AdminViewState {
        section: AdminSection::Resources,
        actor_id: "callback-operator@rspice.invalid".to_owned(),
        authority_id: "test:project-callback-ui".to_owned(),
        reason: "Execute and retain exact project callback evidence".to_owned(),
        ..AdminViewState::default()
    };

    apply_action(
        &ctx,
        &mut app,
        &mut view,
        AdminAction::RunProjectCallback {
            callback_id: "derive-device".to_owned(),
        },
    );
    assert_eq!(app.state.workspace.pdk_callback_receipts().len(), 1);
    app.state
        .workspace
        .validate_pdk_callback_receipts()
        .expect("committed callback ledger validates");
    ctx.data_mut(|data| {
        data.insert_temp(egui::Id::new(VIEW_STATE_ID), view);
    });

    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(430.0, 3_000.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
        },
    );
    let accessibility = output
        .platform_output
        .accesskit_update
        .expect("PDK callback accessibility tree");
    let labels = accessibility
        .nodes
        .iter()
        .filter_map(|(_, node)| node.label())
        .collect::<Vec<_>>();

    assert!(labels.iter().any(|label| {
        label.contains("Run callback derive-device")
            && label.contains("exact attached project revision")
    }));
    assert!(
        labels
            .iter()
            .any(|label| label.contains("receipt verified")),
        "callback receipt status is absent from accessibility labels: {labels:?}"
    );
    assert!(
        labels
            .iter()
            .any(|label| { label.contains("Copy exact project callback receipt 1 as JSON") })
    );
}
