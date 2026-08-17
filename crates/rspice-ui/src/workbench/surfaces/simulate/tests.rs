//! Tests for the simulate surface's form and output contracts.
//!
//! Output specifications must name an explicit dataset and retain their
//! measurement-contract failures rather than hiding them, and the form
//! viewport anchor must survive structure changes without following user
//! scroll.

use super::output_evidence::{
    OutputMeasurementEvidence, measurement_in_output_dataset, selected_output_dataset,
};
use super::*;
use crate::product::{ContentDigest, ObjectRevision};
use crate::state::{AnalysisResult, AnalysisType, SimulationRun, SimulationState};

#[test]
fn run_set_workload_counts_analysis_owned_temperature_points_and_assembly() {
    let mut app = RSpiceApp::test_instance();
    let before = page_runset::exact_plan_task_count(&app)
        .unwrap()
        .expect("baseline workload");
    insert_analysis_instance(&mut app, AnalysisKind::Temperature);

    let tasks = page_runset::exact_plan_task_count(&app)
        .expect("workload is valid")
        .expect("reference-only Run Set has an exact workload");

    // Default temperature range: -40 through 110 in 25-degree steps, plus
    // one family-assembly task. The stop at 125 is not a step point.
    assert_eq!(tasks - before, 8);
}

#[test]
fn run_set_workload_counts_generated_pss_spectrum_task() {
    let mut app = RSpiceApp::test_instance();
    let before = page_runset::exact_plan_task_count(&app)
        .unwrap()
        .expect("baseline workload");
    insert_analysis_instance(&mut app, AnalysisKind::Pss);

    let tasks = page_runset::exact_plan_task_count(&app)
        .expect("workload is valid")
        .expect("reference-only Run Set has an exact workload");

    // The default plan already owns the OP prerequisite, so this adds one PSS
    // solve and its generated spectrum task.
    assert_eq!(tasks - before, 2);
}

/// A specification bounding `measurement`, used to order a dataset's points.
fn spec(measurement: &str, min: Option<f64>, max: Option<f64>) -> crate::state::SpecEntry {
    crate::state::SpecEntry {
        measurement: measurement.to_owned(),
        expression: String::new(),
        min,
        max,
        unit: String::new(),
        scope: crate::state::SpecPointScope::AllPoints,
    }
}

fn attributed(analysis: AnalysisResult) -> AnalysisResult {
    analysis.with_provenance(
        crate::state::AnalysisResultProvenance::new(
            AnalysisInstanceId::new(),
            ObjectRevision::INITIAL,
            ContentDigest::from_bytes([0x5a; 32]),
            Vec::new(),
        )
        .expect("valid test provenance"),
    )
}

#[test]
fn lifecycle_receipt_height_is_stable_across_short_and_verbose_receipts() {
    let render_height = |detail: &str, width: f32| {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        app.state
            .workbench
            .analysis_lifecycle_status
            .record_receipt(detail);
        let mut height = 0.0;
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(width, 240.0))),
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        height = lifecycle_receipt_strip(ui, &app).rect.height();
                    });
            },
        );
        height
    };
    let short = "Edit committed.";
    let verbose = "Receipt #42 committed for an immutable analysis instance. Dependency bindings were refreshed from enabled earlier instances. Prior datasets remain immutable and the complete diagnostic remains available for audit.";

    assert_eq!(render_height(short, 960.0), 40.0);
    assert_eq!(render_height(verbose, 960.0), 40.0);
    assert_eq!(render_height(short, 560.0), 64.0);
    assert_eq!(render_height(verbose, 560.0), 64.0);
}

#[test]
fn envelope_source_catalog_rejects_unknown_and_dc_only_selections() {
    let catalog = EnvelopeSourceCatalog {
        source_digest: ContentDigest::from_bytes([0x33; 32]),
        names: vec!["Xdrv.VMOD".to_owned(), "VPULSE".to_owned()],
        netlist_source: None,
        diagnostic: None,
    };
    assert_eq!(catalog.selection_error(&["xDRV.vmod".to_owned()]), None);
    assert_eq!(
        catalog.selection_error(&["VBIAS".to_owned()]).as_deref(),
        Some("Unknown or DC-only circuit modulation source: VBIAS")
    );
}

#[test]
fn dependency_source_catalog_uses_the_configured_design_not_the_open_netlist_document() {
    let mut app = RSpiceApp::test_instance();
    app.state.provision_test_project_technology_contract();
    crate::workbench::examples::load_example("CMOS Inverter", &mut app.state.schematic);
    app.state.simulation.netlist_content =
        "stale document\nVSTALE stale 0 PULSE(0 1 0 1n 1n 1u 2u)\n.end\n".to_owned();

    let catalog = build_envelope_source_catalog(&app);

    assert_eq!(catalog.diagnostic, None);
    assert!(catalog.names.iter().any(|name| name == "VIN"));
    assert!(!catalog.names.iter().any(|name| name == "VSTALE"));
}

#[test]
fn dependency_source_cache_ignores_document_switches_and_tracks_design_commits() {
    let mut app = RSpiceApp::test_instance();
    let original = envelope_source_catalog_input_digest(&app);
    app.state.simulation.netlist_content = "unrelated editor document".to_owned();
    assert_eq!(envelope_source_catalog_input_digest(&app), original);

    app.state.design_execution_epoch = app.state.design_execution_epoch.wrapping_add(1);
    assert_ne!(envelope_source_catalog_input_digest(&app), original);

    let mut app = RSpiceApp::test_instance();
    let original = envelope_source_catalog_input_digest(&app);
    app.state.sim_setup.options.reltol *= 0.5;
    assert_ne!(
        envelope_source_catalog_input_digest(&app),
        original,
        "prepared-source option changes invalidate the catalog"
    );
}

#[test]
fn pss_source_catalog_requires_the_complete_exact_tone_set() {
    let catalog = EnvelopeSourceCatalog {
        source_digest: ContentDigest::from_bytes([0x34; 32]),
        names: vec!["VCLK".to_owned(), "Xdrv.VLO".to_owned()],
        netlist_source: None,
        diagnostic: None,
    };
    assert_eq!(
        catalog.exact_periodic_selection_error(&["xDRV.vlo".to_owned(), "vclk".to_owned()]),
        None
    );
    assert_eq!(
        catalog
            .exact_periodic_selection_error(&["VCLK".to_owned()])
            .as_deref(),
        Some("PSS Tones must include every elaborated periodic source; omitted: Xdrv.VLO")
    );
}

#[test]
fn desktop_analysis_split_matches_mockup_ratio_and_minimums() {
    let (left, right) = analysis_split_widths(1001.0, 1_440.0);
    assert!((left - 340.0).abs() < f32::EPSILON);
    assert!((right - 660.0).abs() < f32::EPSILON);

    let (left, right) = analysis_split_widths(551.0, 1_440.0);
    assert!((left - 190.0).abs() < f32::EPSILON);
    assert!((right - 360.0).abs() < f32::EPSILON);

    let (left, right) = analysis_split_widths(751.0, 1_020.0);
    assert!((left - 217.5).abs() < f32::EPSILON);
    assert!((right - 532.5).abs() < f32::EPSILON);
}

#[test]
fn responsive_breakpoints_match_mockup_contract() {
    assert_eq!(SIMULATION_STACK_BREAKPOINT, 820.0);
    assert_eq!(TITLE_ACTION_STACK_BREAKPOINT, 560.0);
    assert!(!analysis_workspace_is_split(820.0, 700.0));
    assert!(analysis_workspace_is_split(821.0, 506.0));
    assert!(!analysis_workspace_is_split(1_440.0, 505.0));
    assert!(!analysis_workspace_is_split(1_440.0, 550.0));
    assert!(analysis_workspace_is_split(1_440.0, 551.0));
    assert_eq!(analysis_split_min_width(1_020.0), 506.0);
    assert_eq!(analysis_split_min_width(1_021.0), 551.0);
}

#[test]
fn responsive_surface_geometry_matches_mockup_contract() {
    assert_eq!(ANALYSIS_ROW_HEIGHT, 53.0);
    assert_eq!(ANALYSIS_INDEX_DIAMETER, 22.0);
    assert_eq!(ANALYSIS_ROW_LEFT_PADDING, 9.0);
    assert_eq!(ANALYSIS_STACK_TABLET_MIN_WIDTH, 175.0);
    assert_eq!(ANALYSIS_STACK_DESKTOP_MIN_WIDTH, 190.0);
    assert!(ANALYSIS_STACK_TABLET_MIN_WIDTH >= analysis_row_content_min_width());
    assert_eq!(PREFLIGHT_CELL_HEIGHT, 42.0);
    assert_eq!(STACKED_WORKSPACE_GAP, 9.0);
    assert_eq!(analysis_column_min_height(720.0, 148.0), 572.0);
    assert_eq!(analysis_column_min_height(100.0, 120.0), 1.0);
    let row_rect = Rect::from_min_size(egui::pos2(10.0, 20.0), vec2(1_000.0, 572.0));
    let background = analysis_stack_background_rect(row_rect, 340.0);
    assert_eq!(background.min, row_rect.min);
    assert_eq!(background.width(), 340.0);
    assert_eq!(background.height(), row_rect.height());
}

#[test]
fn dependency_repair_cta_describes_the_first_repairable_issue() {
    let mut plan = crate::simulation::plan::SimulationPlan::empty();
    let (dependent, _) = plan
        .insert(AnalysisKind::Ac)
        .expect("dependent analysis inserts");
    let target = AnalysisInstanceId::new();
    let dependency = AnalysisDependency::new(AnalysisKind::OperatingPoint, target);

    assert_eq!(
        dependency_repair_cta(
            &plan,
            &[
                AnalysisPlanIssue::NoEnabledInstances,
                AnalysisPlanIssue::MissingPrerequisite {
                    dependent,
                    prerequisite: AnalysisKind::OperatingPoint,
                },
            ],
            &[],
        )
        .as_deref(),
        Some("Add Operating point")
    );
    let (op, _) = plan
        .insert_at(AnalysisKind::OperatingPoint, 0)
        .expect("candidate OP inserts");
    plan.set_enabled(op, false).expect("candidate OP disables");
    assert_eq!(
        dependency_repair_cta(
            &plan,
            &[AnalysisPlanIssue::MissingPrerequisite {
                dependent,
                prerequisite: AnalysisKind::OperatingPoint,
            }],
            &[],
        )
        .as_deref(),
        Some("Enable Operating point")
    );
    plan.set_enabled(op, true).expect("candidate OP enables");
    assert_eq!(
        dependency_repair_cta(
            &plan,
            &[AnalysisPlanIssue::MissingPrerequisite {
                dependent,
                prerequisite: AnalysisKind::OperatingPoint,
            }],
            &[],
        )
        .as_deref(),
        Some("Bind Operating point")
    );
    plan.reorder(op, 1).expect("unbound OP can move later");
    assert_eq!(
        dependency_repair_cta(
            &plan,
            &[AnalysisPlanIssue::MissingPrerequisite {
                dependent,
                prerequisite: AnalysisKind::OperatingPoint,
            }],
            &[],
        )
        .as_deref(),
        Some("Move Operating point earlier")
    );
    assert_eq!(
        dependency_repair_cta(
            &plan,
            &[AnalysisPlanIssue::DisabledDependency { dependent, target }],
            &[dependency],
        )
        .as_deref(),
        Some("Enable Operating point")
    );
    assert_eq!(
        dependency_repair_cta(
            &plan,
            &[
                AnalysisPlanIssue::DisabledDependency { dependent, target },
                AnalysisPlanIssue::DependencyNotEarlier { dependent, target },
            ],
            &[dependency],
        )
        .as_deref(),
        Some("Enable and move Operating point earlier")
    );
    assert_eq!(
        dependency_repair_cta(
            &plan,
            &[AnalysisPlanIssue::DependencyNotEarlier { dependent, target }],
            &[dependency],
        )
        .as_deref(),
        Some("Move Operating point earlier")
    );
    assert_eq!(
        dependency_repair_cta(
            &plan,
            &[AnalysisPlanIssue::WrongDependencyKind {
                dependent,
                prerequisite: AnalysisKind::OperatingPoint,
                target,
                actual: AnalysisKind::Transient,
            }],
            &[dependency],
        )
        .as_deref(),
        Some("Move Operating point earlier")
    );
    assert_eq!(
        dependency_repair_cta(
            &plan,
            &[AnalysisPlanIssue::DanglingDependency { dependent, target }],
            &[dependency],
        )
        .as_deref(),
        Some("Move Operating point earlier")
    );
    assert_eq!(
        dependency_repair_cta(
            &plan,
            &[AnalysisPlanIssue::DuplicateDependencyRole {
                dependent,
                prerequisite: AnalysisKind::OperatingPoint,
            }],
            &[dependency],
        )
        .as_deref(),
        Some("Resolve duplicate Operating point binding")
    );
    let self_dependency = AnalysisDependency::new(AnalysisKind::OperatingPoint, dependent);
    assert_eq!(
        dependency_repair_cta(
            &plan,
            &[AnalysisPlanIssue::SelfDependency { dependent }],
            &[self_dependency],
        )
        .as_deref(),
        Some("Move Operating point earlier")
    );

    let mut no_prerequisites = crate::simulation::plan::SimulationPlan::empty();
    let (op, _) = no_prerequisites
        .insert(AnalysisKind::OperatingPoint)
        .expect("OP inserts");
    let unexpected_self = AnalysisDependency::new(AnalysisKind::Ac, op);
    assert_eq!(
        dependency_repair_cta(
            &no_prerequisites,
            &[
                AnalysisPlanIssue::UnexpectedDependencyRole {
                    dependent: op,
                    prerequisite: AnalysisKind::Ac,
                },
                AnalysisPlanIssue::SelfDependency { dependent: op },
            ],
            &[unexpected_self],
        )
        .as_deref(),
        Some("Remove unexpected AC response binding"),
        "an unexpected legacy edge must be removable with one atomic quick repair"
    );

    let mut mixed_corruption = crate::simulation::plan::SimulationPlan::empty();
    let (ac, _) = mixed_corruption
        .insert(AnalysisKind::Ac)
        .expect("AC inserts");
    let unexpected_self = AnalysisDependency::new(AnalysisKind::Ac, ac);
    assert_eq!(
        dependency_repair_cta(
            &mixed_corruption,
            &[
                AnalysisPlanIssue::SelfDependency { dependent: ac },
                AnalysisPlanIssue::MissingPrerequisite {
                    dependent: ac,
                    prerequisite: AnalysisKind::OperatingPoint,
                },
            ],
            &[unexpected_self],
        )
        .as_deref(),
        Some("Add Operating point"),
        "an undeclared corrupt edge must not hide a repairable declared prerequisite"
    );
}

#[test]
fn every_declared_prerequisite_kind_has_a_contextual_add_action() {
    let dependent_kinds = AnalysisKind::ALL
        .into_iter()
        .filter(|kind| !kind.prerequisites().is_empty())
        .collect::<Vec<_>>();
    assert_eq!(dependent_kinds.len(), 24);

    for kind in dependent_kinds {
        let mut plan = crate::simulation::plan::SimulationPlan::empty();
        let (dependent, _) = plan.insert(kind).expect("dependent inserts");
        let issue = AnalysisPlanIssue::MissingPrerequisite {
            dependent,
            prerequisite: kind.prerequisites()[0],
        };
        let expected = if kind == AnalysisKind::Fourier {
            "Add compatible Transient".to_owned()
        } else {
            format!("Add {}", kind.prerequisites()[0].label())
        };
        assert_eq!(
            dependency_repair_cta(&plan, &[issue], &[]).as_deref(),
            Some(expected.as_str()),
            "{kind}"
        );
    }
}

#[test]
fn fourier_dependency_cta_requires_a_valid_compatible_transient() {
    let mut plan = crate::simulation::plan::SimulationPlan::new();
    let coarse = plan.instances()[0].id();
    let (fourier, _) = plan.insert(AnalysisKind::Fourier).expect("Fourier inserts");
    plan.edit(fourier, |draft| {
        let AnalysisDraft::Fourier(draft) = draft else {
            panic!("expected Fourier draft");
        };
        draft.fundamental = "100Meg".to_owned();
        draft.harmonics = "10".to_owned();
        draft.start_time = "0".to_owned();
        draft.stop_time = "100n".to_owned();
    })
    .expect("Fourier edits");
    let missing = AnalysisPlanIssue::MissingPrerequisite {
        dependent: fourier,
        prerequisite: AnalysisKind::Transient,
    };
    assert!(!plan.dependency_candidate_is_compatible(fourier, AnalysisKind::Transient, coarse));
    assert_eq!(
        dependency_repair_cta(&plan, std::slice::from_ref(&missing), &[]).as_deref(),
        Some("Add compatible Transient")
    );
    let incompatible = AnalysisPlanIssue::IncompatibleDependencyConfiguration {
        dependent: fourier,
        prerequisite: AnalysisKind::Transient,
        target: coarse,
        detail: "sample interval is too coarse".to_owned(),
    };
    let dependencies = plan
        .instance(fourier)
        .expect("Fourier remains in the plan")
        .dependencies();
    assert_eq!(
        dependency_repair_cta(&plan, std::slice::from_ref(&incompatible), dependencies).as_deref(),
        Some("Add compatible Transient")
    );

    plan.edit(fourier, |draft| {
        let AnalysisDraft::Fourier(draft) = draft else {
            panic!("expected Fourier draft");
        };
        draft.fundamental = "unfinished(".to_owned();
    })
    .expect("in-progress draft is retained");
    assert_eq!(
        dependency_repair_cta(&plan, std::slice::from_ref(&missing), &[]),
        None,
        "repair must not be offered until the consumer configuration is valid"
    );
    assert_eq!(
        dependency_repair_cta(
            &plan,
            &[AnalysisPlanIssue::DuplicateDependencyRole {
                dependent: fourier,
                prerequisite: AnalysisKind::Transient,
            }],
            &[
                AnalysisDependency::new(AnalysisKind::Transient, coarse),
                AnalysisDependency::new(AnalysisKind::Transient, AnalysisInstanceId::new()),
            ],
        ),
        None,
        "corrupt-edge repair must not be offered for an invalid Fourier consumer"
    );
}

#[test]
fn dependent_surfaces_include_transitive_prerequisite_failures() {
    let mut plan = crate::simulation::plan::SimulationPlan::empty();
    let (pss, _) = plan.insert(AnalysisKind::Pss).expect("PSS inserts");
    let (pac, _) = plan.insert(AnalysisKind::Pac).expect("PAC inserts");
    plan.bind_dependency(pac, AnalysisKind::Pss, pss)
        .expect("PAC binds the exact PSS");

    let closure = dependency_closure_ids(&plan, pac);
    assert!(closure.contains(&pac));
    assert!(closure.contains(&pss));
    let issues = plan
        .validation_issues()
        .into_iter()
        .filter(|issue| {
            closure
                .iter()
                .any(|instance| issue_applies_to(issue, *instance))
        })
        .collect::<Vec<_>>();
    assert!(issues.contains(&AnalysisPlanIssue::MissingPrerequisite {
        dependent: pss,
        prerequisite: AnalysisKind::OperatingPoint,
    }));
    assert_eq!(
        dependency_repair_cta(&plan, &issues, plan.instance(pac).unwrap().dependencies())
            .as_deref(),
        Some("Add Operating point")
    );
}

#[test]
fn selected_dependency_closure_ignores_wrong_dangling_self_and_duplicate_edges() {
    let mut plan = crate::simulation::plan::SimulationPlan::empty();
    let (ac, _) = plan.insert(AnalysisKind::Ac).expect("AC inserts");
    let (pss, _) = plan.insert(AnalysisKind::Pss).expect("PSS inserts");
    let (first_op, _) = plan
        .insert(AnalysisKind::OperatingPoint)
        .expect("first OP inserts");
    let (second_op, _) = plan
        .insert(AnalysisKind::OperatingPoint)
        .expect("second OP inserts");

    let with_dependencies = |dependencies: Vec<AnalysisDependency>| {
        let mut encoded = serde_json::to_value(&plan).expect("plan serializes");
        let instances = encoded["instances"]
            .as_array_mut()
            .expect("instances serialize as an array");
        let ac_instance = instances
            .iter_mut()
            .find(|instance| instance["id"] == serde_json::json!(ac))
            .expect("serialized AC remains present");
        ac_instance["dependencies"] =
            serde_json::to_value(dependencies).expect("dependencies serialize");
        serde_json::from_value::<crate::simulation::plan::SimulationPlan>(encoded)
            .expect("raw corrupt fixture deserializes")
    };
    let assert_root_only = |dependencies| {
        let corrupt = with_dependencies(dependencies);
        assert_eq!(dependency_closure_ids(&corrupt, ac), HashSet::from([ac]));
    };

    assert_root_only(vec![AnalysisDependency::new(
        AnalysisKind::OperatingPoint,
        pss,
    )]);
    assert_root_only(vec![AnalysisDependency::new(
        AnalysisKind::OperatingPoint,
        AnalysisInstanceId::new(),
    )]);
    assert_root_only(vec![AnalysisDependency::new(
        AnalysisKind::OperatingPoint,
        ac,
    )]);
    assert_root_only(vec![
        AnalysisDependency::new(AnalysisKind::OperatingPoint, first_op),
        AnalysisDependency::new(AnalysisKind::OperatingPoint, second_op),
    ]);

    let valid = with_dependencies(vec![AnalysisDependency::new(
        AnalysisKind::OperatingPoint,
        first_op,
    )]);
    assert_eq!(
        dependency_closure_ids(&valid, ac),
        HashSet::from([ac, first_op])
    );
}

#[test]
fn validating_a_dependent_checks_every_transitive_prerequisite_draft() {
    let mut plan = crate::simulation::plan::SimulationPlan::empty();
    let (op, _) = plan
        .insert(AnalysisKind::OperatingPoint)
        .expect("OP inserts");
    let (pss, _) = plan.insert(AnalysisKind::Pss).expect("PSS inserts");
    plan.bind_dependency(pss, AnalysisKind::OperatingPoint, op)
        .expect("PSS binds OP");
    let (pac, _) = plan.insert(AnalysisKind::Pac).expect("PAC inserts");
    plan.bind_dependency(pac, AnalysisKind::Pss, pss)
        .expect("PAC binds PSS");
    plan.edit(pss, |draft| {
        let AnalysisDraft::Pss(pss) = draft else {
            panic!("expected PSS draft");
        };
        pss.fund_freq = "unfinished-expression(".to_owned();
    })
    .expect("invalid editable PSS draft is retained");

    let mut app = RSpiceApp::test_instance();
    app.state.sim_setup.analysis_plan = Some(plan);
    validate_analysis_instance(&mut app, pac);

    assert!(
        app.state
            .workbench
            .analysis_lifecycle_status
            .message()
            .contains(&pss.to_string())
    );
    assert!(
        app.state
            .workbench
            .analysis_lifecycle_status
            .message()
            .starts_with("Validate rejected fail-closed")
    );
    assert!(
        app.state.workbench.analysis_lifecycle_status.is_refusal(),
        "a rejected validation is a refusal, not a receipt"
    );
}

#[test]
fn selected_analysis_exposes_every_compatible_earlier_prerequisite() {
    let mut plan = crate::simulation::plan::SimulationPlan::empty();
    let (first_op, _) = plan
        .insert(AnalysisKind::OperatingPoint)
        .expect("first OP inserts");
    let (second_op, _) = plan
        .insert(AnalysisKind::OperatingPoint)
        .expect("second OP inserts");
    let (ac, _) = plan.insert(AnalysisKind::Ac).expect("AC inserts");
    plan.bind_dependency(ac, AnalysisKind::OperatingPoint, first_op)
        .expect("exact first OP binding succeeds");

    let mut app = RSpiceApp::test_instance();
    app.state.sim_setup.analysis_plan = Some(plan);
    app.state.workbench.active_analysis_instance = Some(ac);
    let dependency_sources = EnvelopeSourceCatalog {
        source_digest: ContentDigest::from_bytes([7; 32]),
        names: vec!["VIN_DIFF".to_owned()],
        netlist_source: None,
        diagnostic: None,
    };
    let selected = selected_analysis(&app, &dependency_sources)
        .expect("selection resolves")
        .expect("analysis is selected");
    let candidates = selected
        .prerequisite_candidates
        .iter()
        .find_map(|(kind, candidates)| {
            (*kind == AnalysisKind::OperatingPoint).then_some(candidates)
        })
        .expect("OP candidates are available");
    assert_eq!(candidates.len(), 2);
    assert_eq!(candidates[0].id, first_op);
    assert_eq!(candidates[1].id, second_op);
}

#[test]
fn selected_dependent_reports_contextual_pss_mismatch_and_only_offers_executable_repair() {
    let mut plan = crate::simulation::plan::SimulationPlan::empty();
    let (op, _) = plan
        .insert(AnalysisKind::OperatingPoint)
        .expect("OP inserts");
    let (pss, _) = plan.insert(AnalysisKind::Pss).expect("PSS inserts");
    plan.bind_dependency(pss, AnalysisKind::OperatingPoint, op)
        .expect("PSS binds OP");
    let (pac, _) = plan.insert(AnalysisKind::Pac).expect("PAC inserts");
    plan.bind_dependency(pac, AnalysisKind::Pss, pss)
        .expect("PAC binds PSS");

    let mut app = RSpiceApp::test_instance();
    app.state.sim_setup.analysis_plan = Some(plan);
    app.state.workbench.active_analysis_instance = Some(pac);

    let exact_sources = EnvelopeSourceCatalog {
            source_digest: ContentDigest::from_bytes([8; 32]),
            names: vec!["VLO".to_owned(), "VRF".to_owned()],
            netlist_source: Some(
                "periodic fixture\nVLO lo 0 SIN(0 1 1k)\nVRF rf 0 SIN(0 1 2k)\nR1 lo 0 1k\nR2 rf 0 1k\n.end\n"
                    .to_owned(),
            ),
            diagnostic: None,
        };
    let selected = selected_analysis(&app, &exact_sources)
        .expect("selection resolves")
        .expect("PAC remains selected");
    assert!(
        selected
            .contextual_dependency_error
            .as_deref()
            .is_some_and(|error| error.contains("active circuit"))
    );
    assert_eq!(
        selected.repair_label.as_deref(),
        Some("Repair PSS prerequisite")
    );

    let unavailable_sources = EnvelopeSourceCatalog {
        source_digest: ContentDigest::from_bytes([9; 32]),
        names: Vec::new(),
        netlist_source: None,
        diagnostic: Some("netlist elaboration failed".to_owned()),
    };
    let selected = selected_analysis(&app, &unavailable_sources)
        .expect("selection resolves fail-closed")
        .expect("PAC remains selected");
    assert!(selected.contextual_dependency_error.is_some());
    assert_eq!(
        selected.repair_label, None,
        "a quick action must be hidden when the selected root cannot be repaired"
    );
}

#[test]
fn disabled_dependents_can_prepare_required_prerequisites_in_one_action() {
    let mut plan = crate::simulation::plan::SimulationPlan::empty();
    let (ac, _) = plan.insert(AnalysisKind::Ac).expect("AC inserts");
    plan.set_enabled(ac, false).expect("AC disables");
    let mut app = RSpiceApp::test_instance();
    app.state.sim_setup.analysis_plan = Some(plan);
    app.state.workbench.active_analysis_instance = Some(ac);
    let sources = build_envelope_source_catalog(&app);

    let selected = selected_analysis(&app, &sources)
        .expect("selection resolves")
        .expect("AC remains selected");
    assert!(!selected.enabled);
    assert_eq!(
        selected.repair_label.as_deref(),
        Some("Add Operating point")
    );

    apply_analysis_action(&mut app, ac, AnalysisAction::RepairDependencies);
    let plan = app.state.sim_setup.stable_analysis_plan().unwrap();
    let dependency = plan.instance(ac).unwrap().dependencies()[0];
    let op = plan.instance(dependency.target()).unwrap();
    assert_eq!(op.kind(), AnalysisKind::OperatingPoint);
    assert!(op.enabled());
    assert!(
        app.state
            .workbench
            .analysis_lifecycle_status
            .message()
            .contains("Prerequisite repair completed atomically")
    );
}

#[test]
fn phase_noise_guides_autonomous_pss_authoring_when_it_cannot_be_inferred() {
    let mut plan = crate::simulation::plan::SimulationPlan::empty();
    let (pnoise, _) = plan.insert(AnalysisKind::Pnoise).expect("PNOISE inserts");
    plan.edit(pnoise, |draft| {
        let AnalysisDraft::Pnoise(pnoise) = draft else {
            panic!("expected PNOISE draft");
        };
        pnoise.noise_ref_idx = 2;
    })
    .expect("phase-noise selection commits");
    let mut app = RSpiceApp::test_instance();
    app.state.sim_setup.analysis_plan = Some(plan);
    app.state.workbench.active_analysis_instance = Some(pnoise);
    let sources = build_envelope_source_catalog(&app);

    let selected = selected_analysis(&app, &sources)
        .expect("selection resolves")
        .expect("PNOISE remains selected");
    assert_eq!(selected.repair_label, None);
    assert_eq!(
        selected.configure_pss_label.as_deref(),
        Some("Add and configure autonomous PSS")
    );

    apply_analysis_action(&mut app, pnoise, AnalysisAction::PrepareAutonomousPss);
    let prepared = app
        .state
        .workbench
        .active_analysis_instance
        .expect("prepared PSS becomes selected");
    let plan = app.state.sim_setup.stable_analysis_plan().unwrap();
    let prerequisite = plan.instance(prepared).expect("prepared PSS is retained");
    assert_eq!(prerequisite.kind(), AnalysisKind::Pss);
    assert!(!prerequisite.enabled());
    let AnalysisDraft::Pss(pss) = prerequisite.draft() else {
        panic!("prepared prerequisite is PSS");
    };
    assert!(pss.osc_mode);
    assert!(pss.osc_node.is_empty());
    assert!(plan.instance(pnoise).unwrap().dependencies().is_empty());
    assert!(
        app.state
            .workbench
            .analysis_lifecycle_status
            .message()
            .contains("Enter the exact oscillator node")
    );

    app.state.workbench.active_analysis_instance = Some(pnoise);
    let selected = selected_analysis(&app, &sources)
        .expect("selection resolves after prerequisite preparation")
        .expect("PNOISE becomes selected again");
    assert_eq!(
        selected.configure_pss_label.as_deref(),
        Some("Configure existing autonomous PSS")
    );
    let before_count = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .unwrap()
        .instances()
        .len();
    apply_analysis_action(&mut app, pnoise, AnalysisAction::PrepareAutonomousPss);
    let plan = app.state.sim_setup.stable_analysis_plan().unwrap();
    assert_eq!(plan.instances().len(), before_count);
    assert_eq!(app.state.workbench.active_analysis_instance, Some(prepared));
    assert!(
        app.state
            .workbench
            .analysis_lifecycle_status
            .message()
            .contains("without creating a duplicate instance")
    );
}

#[test]
fn analysis_catalog_search_text_is_centered_in_its_48_point_row() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut query = "noise".to_owned();
    let mut geometry = None;
    let _output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1_280.0, 720.0))),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let output = ui
                    .allocate_ui_with_layout(
                        vec2(480.0, 48.0),
                        Layout::centered_and_justified(egui::Direction::LeftToRight),
                        |ui| analysis_catalog_search_field(&mut query).show(ui),
                    )
                    .inner;
                geometry = Some((
                    output.response.rect,
                    output.galley_pos,
                    output.galley.size(),
                ));
            });
        },
    );
    let (response, galley_pos, galley_size) = geometry.expect("search field rendered");

    assert!((response.height() - 48.0).abs() <= 0.5);
    assert!((galley_pos.y + galley_size.y * 0.5 - response.center().y).abs() <= 0.5);
}

#[test]
fn saved_output_storage_preview_uses_exact_binary_units_and_analysis_count() {
    assert_eq!(format_storage_bytes(960), "960 B");
    assert_eq!(format_storage_bytes(1_536), "1.50 KiB");
    assert_eq!(format_storage_bytes(2 * 1024 * 1024), "2.00 MiB");
    assert_eq!(
        saved_output_storage_summary(&SavedOutputStorageEstimate::ExactBytes(960), 1, 0),
        "960 B · 1 compatible analysis"
    );
    assert_eq!(
        saved_output_storage_summary(&SavedOutputStorageEstimate::ExactBytes(1_536), 2, 0),
        "1.50 KiB · 2 compatible analyses"
    );
    assert_eq!(
        saved_output_storage_summary(
            &SavedOutputStorageEstimate::Indeterminate {
                reason: "adaptive transient grid".to_owned(),
            },
            1,
            0,
        ),
        "indeterminate · adaptive transient grid"
    );
    assert_eq!(
        saved_output_storage_summary(&SavedOutputStorageEstimate::ExactBytes(0), 2, 2),
        "deferred · shared source ceiling counted at plan level · 2 compatible analyses"
    );
}

#[test]
fn analysis_catalog_uses_the_mockup_dialog_and_row_contracts() {
    assert_eq!(ANALYSIS_CATALOG_GROUP_HEIGHT, 29.0);
    assert_eq!(ANALYSIS_CATALOG_ROW_HEIGHT, 57.0);
    assert_eq!(ANALYSIS_CATALOG_READINESS_WIDTH, 142.0);
    assert_eq!(analysis_catalog_column_count(1_199.99), 1);
    assert_eq!(analysis_catalog_column_count(1_200.0), 2);
    assert_eq!(analysis_catalog_readiness(AnalysisKind::Pac), None);
    assert_eq!(analysis_catalog_readiness(AnalysisKind::Transient), None);
    assert_eq!(
        analysis_catalog_readiness(AnalysisKind::Psp),
        Some("Preview engine · non-sign-off")
    );
    assert_eq!(
        analysis_catalog_disposition(&[], AnalysisKind::Psp),
        "Add instance"
    );
    assert_eq!(
        analysis_catalog_readiness(AnalysisKind::Hbsp),
        Some("Preview engine · non-sign-off")
    );
    assert_eq!(
        analysis_catalog_disposition(&[], AnalysisKind::Hbsp),
        "Add instance"
    );
    assert_eq!(
        analysis_catalog_readiness(AnalysisKind::Hbnoise),
        Some("Preview engine · non-sign-off")
    );
    assert_eq!(
        analysis_catalog_disposition(&[], AnalysisKind::Hbnoise),
        "Add instance"
    );
    assert_eq!(
        analysis_catalog_readiness(AnalysisKind::Qpss),
        Some("the QPSS spectral-lattice solver is not available in this engine build")
    );
    assert_eq!(
        analysis_catalog_disposition(&[], AnalysisKind::Qpss),
        "Unavailable"
    );
}

#[test]
fn analysis_catalog_search_preserves_canonical_group_order() {
    let all = filtered_catalog_kinds("");
    let unavailable = [
        AnalysisKind::Qpss,
        AnalysisKind::Qpac,
        AnalysisKind::Qpnoise,
        AnalysisKind::Qpxf,
        AnalysisKind::TransientNoise,
        AnalysisKind::DcMismatch,
    ];
    assert_eq!(all.len(), AnalysisKind::ALL.len());
    assert!(unavailable.iter().all(|kind| all.contains(kind)));
    assert_eq!(all.first(), Some(&AnalysisKind::OperatingPoint));
    assert_eq!(all, AnalysisKind::MANIFEST_ORDER.to_vec());
    assert!(
        all.iter()
            .position(|kind| *kind == AnalysisKind::MonteCarlo)
            < all
                .iter()
                .position(|kind| *kind == AnalysisKind::Reliability)
    );
    assert_eq!(
        filtered_catalog_kinds("periodic noise"),
        vec![AnalysisKind::Pnoise, AnalysisKind::Qpnoise]
    );
    assert_eq!(
        filtered_catalog_kinds("spectral lattice"),
        vec![AnalysisKind::Qpss, AnalysisKind::Qpnoise]
    );
}

#[test]
fn unavailable_analysis_cannot_be_inserted_through_the_surface_action() {
    let mut app = RSpiceApp::test_instance();
    let before = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .unwrap()
        .instances()
        .len();

    insert_analysis_instance(&mut app, AnalysisKind::Qpss);

    let after = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .unwrap()
        .instances()
        .len();
    assert_eq!(after, before);
    assert!(
        app.state
            .workbench
            .analysis_lifecycle_status
            .message()
            .contains("not available")
    );
}

#[test]
fn design_variable_workflow_commits_to_the_active_plan_atomically() {
    let mut app = RSpiceApp::test_instance();
    let plan_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .id();
    let revision_before = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .revision();
    let count_before = app
        .state
        .workspace
        .plan_data(plan_id)
        .map_or(0, |payload| payload.design_variables.len());

    commit_design_variable(&mut app, &DesignVariableDraft::default())
        .expect("valid variable commits");

    let payload = app
        .state
        .workspace
        .plan_data(plan_id)
        .expect("plan payload");
    assert_eq!(payload.design_variables.len(), count_before + 1);
    assert_eq!(
        payload
            .design_variables
            .last()
            .map(|variable| variable.name.as_str()),
        Some("RLOAD_TEST")
    );
    assert!(
        app.state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .revision()
            .get()
            > revision_before.get()
    );
}

#[test]
fn invalid_design_variable_workflow_leaves_authoritative_state_unchanged() {
    let mut app = RSpiceApp::test_instance();
    let plan_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .id();
    let revision_before = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .revision();
    let count_before = app
        .state
        .workspace
        .plan_data(plan_id)
        .map_or(0, |payload| payload.design_variables.len());
    let mut draft = DesignVariableDraft::default();
    draft.allowed_range = "20 kohm … 30 kohm".to_owned();

    assert!(commit_design_variable(&mut app, &draft).is_err());
    assert_eq!(
        app.state
            .workspace
            .plan_data(plan_id)
            .map_or(0, |payload| payload.design_variables.len()),
        count_before
    );
    assert_eq!(
        app.state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .revision(),
        revision_before
    );
}

#[test]
fn saved_output_workflow_commits_a_typed_plan_contract() {
    let mut app = RSpiceApp::test_instance();
    app.state.provision_test_project_technology_contract();
    let plan_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .id();
    let count_before = app
        .state
        .workspace
        .plan_data(plan_id)
        .map_or(0, |payload| payload.saved_outputs.len());

    commit_saved_output(&mut app, &SavedOutputDraft::default()).expect("valid output commits");

    let output = app
        .state
        .workspace
        .plan_data(plan_id)
        .expect("plan payload")
        .saved_outputs
        .last()
        .expect("saved output");
    assert_eq!(
        count_before + 1,
        app.state
            .workspace
            .plan_data(plan_id)
            .unwrap()
            .saved_outputs
            .len()
    );
    assert_eq!(output.name, "V(afe_out)");
    assert_eq!(output.source_expression, "V(afe_out)");
}

#[test]
fn clone_workflow_creates_fresh_plan_and_payload_identities_without_results() {
    let mut app = RSpiceApp::test_instance();
    app.state.provision_test_project_technology_contract();
    commit_design_variable(&mut app, &DesignVariableDraft::default()).expect("source variable");
    commit_saved_output(&mut app, &SavedOutputDraft::default()).expect("source output");
    let source_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable source plan")
        .id();
    let source_payload = app
        .state
        .workspace
        .plan_data(source_id)
        .expect("source payload")
        .clone();
    let retained_runs_before = app
        .state
        .simulation
        .runs
        .iter()
        .map(|run| (run.run_id, run.dataset_id, run.id))
        .collect::<Vec<_>>();
    let mut draft = ClonePlanDraft::for_source(
        app.state.sim_setup.stable_analysis_plan().unwrap().id(),
        app.state.sim_setup.active_plan_name().as_str(),
    );
    draft.name = "Independent characterization".to_owned();

    commit_clone_plan(&mut app, &draft).expect("valid clone");

    let clone_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("active clone")
        .id();
    assert_ne!(clone_id, source_id);
    assert_eq!(app.state.sim_setup.active_plan_name().as_str(), draft.name);
    let clone_payload = app
        .state
        .workspace
        .plan_data(clone_id)
        .expect("cloned payload");
    assert_eq!(clone_payload.design_variables.len(), 1);
    assert_eq!(clone_payload.saved_outputs.len(), 1);
    assert_ne!(
        clone_payload.design_variables[0].id,
        source_payload.design_variables[0].id
    );
    assert_ne!(
        clone_payload.saved_outputs[0].id,
        source_payload.saved_outputs[0].id
    );
    assert_eq!(
        app.state
            .simulation
            .runs
            .iter()
            .map(|run| (run.run_id, run.dataset_id, run.id))
            .collect::<Vec<_>>(),
        retained_runs_before
    );
}

#[test]
fn plan_manager_export_import_remaps_all_local_identities() {
    let mut app = RSpiceApp::test_instance();
    app.state.provision_test_project_technology_contract();
    commit_design_variable(&mut app, &DesignVariableDraft::default()).expect("source variable");
    commit_saved_output(&mut app, &SavedOutputDraft::default()).expect("source output");
    let source_plan = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable source plan");
    let source_id = source_plan.id();
    let source_analysis_ids = source_plan
        .instances()
        .iter()
        .map(|instance| instance.id())
        .collect::<Vec<_>>();
    let source_payload = app
        .state
        .workspace
        .plan_data(source_id)
        .expect("source payload")
        .clone();
    let json = export_simulation_plan_package(&app, source_id).expect("portable export");

    let (imported_id, _) =
        commit_import_simulation_plan(&mut app, &json, "Imported independent characterization")
            .expect("portable import validates and commits");

    assert_ne!(imported_id, source_id);
    assert_eq!(
        app.state.sim_setup.active_plan_name().as_str(),
        "Imported independent characterization"
    );
    let imported_plan = app.state.sim_setup.stable_analysis_plan().unwrap();
    assert!(
        imported_plan
            .instances()
            .iter()
            .all(|instance| !source_analysis_ids.contains(&instance.id()))
    );
    let imported_payload = app
        .state
        .workspace
        .plan_data(imported_id)
        .expect("imported payload");
    assert_eq!(imported_payload.design_variables.len(), 1);
    assert_eq!(imported_payload.saved_outputs.len(), 1);
    assert_ne!(
        imported_payload.design_variables[0].id,
        source_payload.design_variables[0].id
    );
    assert_ne!(
        imported_payload.saved_outputs[0].id,
        source_payload.saved_outputs[0].id
    );
    assert_eq!(imported_payload.regression_baseline_run, None);
}

#[test]
fn cancelling_a_simulation_workflow_never_invokes_its_commit() {
    let mut app = RSpiceApp::test_instance();
    let source_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable source plan")
        .id();
    let inactive_before = app.state.sim_setup.inactive_plans().len();
    let mut draft = ClonePlanDraft::for_source(
        app.state.sim_setup.stable_analysis_plan().unwrap().id(),
        app.state.sim_setup.active_plan_name().as_str(),
    );
    draft.name = "Cancelled clone".to_owned();
    app.state.workbench.simulation_workflow =
        Some(SimulationWorkflowDialog::ClonePlan(draft.clone()));

    finish_workflow_choice(
        &egui::Context::default(),
        &mut app,
        DialogChoice::Cancelled,
        draft,
        commit_clone_plan,
    );

    assert!(app.state.workbench.simulation_workflow.is_none());
    assert_eq!(
        app.state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable source plan")
            .id(),
        source_id
    );
    assert_eq!(app.state.sim_setup.inactive_plans().len(), inactive_before);
}

#[test]
fn output_specifications_never_mix_measurements_across_retained_datasets() {
    let mut older = SimulationRun::new(1);
    older.add_analysis(attributed(
        AnalysisResult::new(1, AnalysisType::Ac, "older")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 12.0)]),
    ));
    let mut selected = SimulationRun::new(2);
    selected.add_analysis(attributed(
        AnalysisResult::new(1, AnalysisType::Ac, "selected")
            .with_measurements(vec![rspice_core::MeasureResult::success("bandwidth", 42.0)]),
    ));

    let selected_dataset = selected.dataset_id;
    let mut simulation = SimulationState::default();
    simulation.runs = vec![older, selected];
    simulation.active_run_idx = Some(1);

    let run = selected_output_dataset(&simulation).expect("selected dataset");
    assert_eq!(run.dataset_id, selected_dataset);
    assert_eq!(
        measurement_in_output_dataset(run, &spec("bandwidth", Some(0.0), None)),
        Some(OutputMeasurementEvidence {
            value: 42.0,
            measurement_passed: true,
            retained_measurements: 1,
        })
    );
    assert_eq!(
        measurement_in_output_dataset(run, &spec("gain", Some(0.0), None)),
        None
    );
}

#[test]
fn swept_evidence_reports_how_many_points_the_value_was_chosen_from() {
    let mut run = SimulationRun::new(1);
    for (id, value) in [(1u64, 12.0), (2, 9.0), (3, 15.0)] {
        run.add_analysis(attributed(
            AnalysisResult::new(id, AnalysisType::Ac, "corner")
                .with_measurements(vec![rspice_core::MeasureResult::success("gain", value)]),
        ));
    }

    let evidence = measurement_in_output_dataset(&run, &spec("gain", Some(10.0), None))
        .expect("evidence resolves");

    assert_eq!(
        evidence.retained_measurements, 3,
        "every retained point of the sweep counts toward coverage"
    );
    assert!(
        !evidence.is_complete_coverage(),
        "one point of a three-point sweep cannot stand for the sweep"
    );
    assert_eq!(
        evidence.value, 9.0,
        "the reported value is the worst point against the bound, not the last one taken"
    );
}

/// The failure mode this ordering exists to prevent.
#[test]
fn a_specification_cannot_pass_while_a_retained_point_fails_it() {
    let mut run = SimulationRun::new(1);
    // The failing point is taken first, so "most recent" would have hidden it.
    for (id, value) in [(1u64, 4.0), (2, 11.0), (3, 12.0)] {
        run.add_analysis(attributed(
            AnalysisResult::new(id, AnalysisType::Ac, "corner")
                .with_measurements(vec![rspice_core::MeasureResult::success("gain", value)]),
        ));
    }
    let bound = spec("gain", Some(10.0), None);

    let evidence = measurement_in_output_dataset(&run, &bound).expect("evidence resolves");

    assert_eq!(evidence.value, 4.0);
    assert!(!bound.passes(evidence.value));
}

#[test]
fn the_worst_point_of_a_two_sided_bound_is_the_one_nearest_a_violation() {
    let mut run = SimulationRun::new(1);
    for (id, value) in [(1u64, 5.0), (2, 9.6), (3, 5.5)] {
        run.add_analysis(attributed(
            AnalysisResult::new(id, AnalysisType::Ac, "corner")
                .with_measurements(vec![rspice_core::MeasureResult::success("gain", value)]),
        ));
    }

    let evidence = measurement_in_output_dataset(&run, &spec("gain", Some(0.0), Some(10.0)))
        .expect("evidence resolves");

    assert_eq!(
        evidence.value, 9.6,
        "0.4 from the ceiling beats 5.0 from the floor"
    );
}

#[test]
fn a_measurement_the_engine_could_not_complete_outranks_every_margin() {
    let mut run = SimulationRun::new(1);
    run.add_analysis(attributed(
        AnalysisResult::new(1, AnalysisType::Ac, "incomplete").with_measurements(vec![
            rspice_core::MeasureResult {
                name: "gain".to_owned(),
                value: Some(50.0),
                error: Some("no crossing in the swept band".to_owned()),
                passed: false,
                expected: None,
                tolerance: None,
                event_axis: None,
            },
        ]),
    ));
    run.add_analysis(attributed(
        AnalysisResult::new(2, AnalysisType::Ac, "tight")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 10.001)]),
    ));

    let evidence = measurement_in_output_dataset(&run, &spec("gain", Some(10.0), None))
        .expect("evidence resolves");

    assert!(!evidence.measurement_passed);
    assert_eq!(evidence.value, 50.0);
}

/// A specification with no scalar bound has no ordering to offer.
#[test]
fn an_unbounded_specification_reports_its_latest_point() {
    let mut run = SimulationRun::new(1);
    for (id, value) in [(1u64, 3.0), (2, 7.0)] {
        run.add_analysis(attributed(
            AnalysisResult::new(id, AnalysisType::Ac, "corner")
                .with_measurements(vec![rspice_core::MeasureResult::success("gain", value)]),
        ));
    }

    let evidence =
        measurement_in_output_dataset(&run, &spec("gain", None, None)).expect("evidence resolves");

    assert_eq!(evidence.value, 7.0);
    assert_eq!(evidence.retained_measurements, 2);
}

#[test]
fn a_lone_measurement_is_reported_as_complete_coverage() {
    let mut run = SimulationRun::new(1);
    run.add_analysis(attributed(
        AnalysisResult::new(1, AnalysisType::Ac, "single")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 12.0)]),
    ));

    let evidence = measurement_in_output_dataset(&run, &spec("gain", Some(0.0), None))
        .expect("evidence resolves");

    assert!(evidence.is_complete_coverage());
}

#[test]
fn coverage_counts_only_attributed_finite_measurements() {
    let mut run = SimulationRun::new(1);
    run.add_analysis(attributed(
        AnalysisResult::new(1, AnalysisType::Ac, "kept")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 12.0)]),
    ));
    // Unattributed: cannot be traced to a configuration, so it is not a point.
    run.add_analysis(
        AnalysisResult::new(2, AnalysisType::Ac, "legacy")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 13.0)]),
    );
    // Attributed but non-finite: not a value, so not a point either.
    run.add_analysis(attributed(
        AnalysisResult::new(3, AnalysisType::Ac, "non-finite").with_measurements(vec![
            rspice_core::MeasureResult {
                name: "gain".to_owned(),
                value: Some(f64::INFINITY),
                error: None,
                passed: true,
                expected: None,
                tolerance: None,
                event_axis: None,
            },
        ]),
    ));

    let evidence = measurement_in_output_dataset(&run, &spec("gain", Some(0.0), None))
        .expect("evidence resolves");

    assert_eq!(evidence.retained_measurements, 1);
    assert!(evidence.is_complete_coverage());
}

#[test]
fn output_specifications_reject_unattributed_and_non_finite_but_retain_failed_analysis_evidence() {
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "legacy")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.0)]),
    );
    run.add_analysis(attributed(
        AnalysisResult::failed(2, AnalysisType::Ac, "failed", "solver failed")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 2.0)]),
    ));
    run.add_analysis(attributed(
        AnalysisResult::new(3, AnalysisType::Ac, "non-finite").with_measurements(vec![
            rspice_core::MeasureResult {
                name: "gain".to_owned(),
                value: Some(f64::NAN),
                error: Some("non-finite".to_owned()),
                passed: false,
                expected: None,
                tolerance: None,
                event_axis: None,
            },
        ]),
    ));

    assert_eq!(
        measurement_in_output_dataset(&run, &spec("gain", Some(0.0), None)),
        Some(OutputMeasurementEvidence {
            value: 2.0,
            measurement_passed: false,
            retained_measurements: 1,
        })
    );
}

#[test]
fn output_specifications_retain_finite_measurement_contract_failures() {
    let mut run = SimulationRun::new(1);
    run.add_analysis(attributed(
        AnalysisResult::new(1, AnalysisType::Ac, "goal miss").with_measurements(vec![
            rspice_core::MeasureResult {
                name: "gain".to_owned(),
                value: Some(9.0),
                error: Some("goal miss".to_owned()),
                passed: false,
                expected: Some(10.0),
                tolerance: Some(0.1),
                event_axis: None,
            },
        ]),
    ));

    assert_eq!(
        measurement_in_output_dataset(&run, &spec("gain", Some(0.0), None)),
        Some(OutputMeasurementEvidence {
            value: 9.0,
            measurement_passed: false,
            retained_measurements: 1,
        })
    );
}

#[test]
fn output_specifications_require_an_explicit_active_dataset() {
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "retained")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 12.0)]),
    );
    let mut simulation = SimulationState::default();
    simulation.runs.push(run);

    assert!(selected_output_dataset(&simulation).is_none());
}

#[test]
fn editor_structure_changes_preserve_the_selected_form_viewport_anchor() {
    let anchor_y = 135.0;
    let displaced_y = anchor_y + 2.0 * ANALYSIS_ROW_HEIGHT + ANALYSIS_GROUP_HEADER_HEIGHT;
    let measured_delta = editor_anchor_scroll_delta(anchor_y, displaced_y);

    assert_eq!(
        adjusted_scroll_for_stack_delta(420.0, 0.0, measured_delta),
        420.0 + 2.0 * ANALYSIS_ROW_HEIGHT + ANALYSIS_GROUP_HEADER_HEIGHT
    );
    assert_eq!(editor_anchor_scroll_delta(280.0, 180.0), -100.0);
    assert_eq!(adjusted_scroll_for_stack_delta(20.0, 0.0, -100.0), 0.0);
}

#[test]
fn editor_anchor_ignores_user_scroll_but_tracks_layout_displacement() {
    let before = content_space_anchor(320.0, 80.0);
    let after_user_scroll = content_space_anchor(270.0, 30.0);
    let after_layout_change = content_space_anchor(296.0, 30.0);

    assert_eq!(before, after_user_scroll);
    assert_eq!(editor_anchor_scroll_delta(before, after_user_scroll), 0.0);
    assert_eq!(
        editor_anchor_scroll_delta(before, after_layout_change),
        26.0
    );
}

/// An analysis attributed to one exact PVT point.
fn at_point(
    analysis: AnalysisResult,
    process: &str,
    temperature_celsius: f64,
    nominal: bool,
) -> AnalysisResult {
    analysis.with_provenance(
        crate::state::AnalysisResultProvenance::new(
            AnalysisInstanceId::new(),
            ObjectRevision::INITIAL,
            ContentDigest::from_bytes([0x5a; 32]),
            Vec::new(),
        )
        .expect("valid test provenance")
        .with_pvt_point(Some(
            crate::state::AnalysisResultPvtPoint::new(
                process,
                Some(1.8),
                temperature_celsius,
                None,
                nominal,
            )
            .expect("valid attributed point"),
        )),
    )
}

fn scoped(
    measurement: &str,
    min: Option<f64>,
    scope: crate::state::SpecPointScope,
) -> crate::state::SpecEntry {
    crate::state::SpecEntry {
        scope,
        ..spec(measurement, min, None)
    }
}

/// The whole point of the scope: a limit that claims to hold at nominal is
/// answered by the nominal point, and the same limit taken across the run set
/// is answered by its worst point. If these two agreed, the scope would be
/// decoration.
#[test]
fn a_specification_scoped_to_nominal_ignores_a_failing_off_nominal_point() {
    let mut run = SimulationRun::new(1);
    run.add_analysis(at_point(
        AnalysisResult::new(1, AnalysisType::DcOp, "TT 27C")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 12.0)]),
        "TT",
        27.0,
        true,
    ));
    run.add_analysis(at_point(
        AnalysisResult::new(2, AnalysisType::DcOp, "SS 125C")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 4.0)]),
        "SS",
        125.0,
        false,
    ));

    let nominal = scoped("gain", Some(10.0), crate::state::SpecPointScope::Nominal);
    let evidence = measurement_in_output_dataset(&run, &nominal).expect("nominal evidence");
    assert_eq!(evidence.value, 12.0);
    assert!(nominal.passes(evidence.value));
    assert!(
        evidence.is_complete_coverage(),
        "one point in scope is the scope's whole answer"
    );

    let everywhere = scoped("gain", Some(10.0), crate::state::SpecPointScope::AllPoints);
    let evidence = measurement_in_output_dataset(&run, &everywhere).expect("run-set evidence");
    assert_eq!(evidence.value, 4.0);
    assert!(!everywhere.passes(evidence.value));

    let slow = scoped(
        "gain",
        Some(10.0),
        crate::state::SpecPointScope::SelectedCorners {
            corners: vec!["ss".to_owned()],
        },
    );
    let evidence = measurement_in_output_dataset(&run, &slow).expect("corner evidence");
    assert_eq!(
        evidence.value, 4.0,
        "the corner scope is matched case-insensitively against the attributed process"
    );
}

/// Coverage is what the page prints beside the value, so counting points the
/// specification was never judged against would overstate the verdict.
#[test]
fn coverage_counts_only_the_points_inside_the_specification_scope() {
    let mut run = SimulationRun::new(1);
    for (id, process, value) in [
        (1u64, "TT", 12.0),
        (2, "SS", 11.0),
        (3, "SS", 10.5),
        (4, "FF", 13.0),
    ] {
        run.add_analysis(at_point(
            AnalysisResult::new(id, AnalysisType::DcOp, process)
                .with_measurements(vec![rspice_core::MeasureResult::success("gain", value)]),
            process,
            27.0,
            id == 1,
        ));
    }

    let all = measurement_in_output_dataset(
        &run,
        &scoped("gain", Some(10.0), crate::state::SpecPointScope::AllPoints),
    )
    .expect("evidence");
    assert_eq!(all.retained_measurements, 4);

    let slow = measurement_in_output_dataset(
        &run,
        &scoped(
            "gain",
            Some(10.0),
            crate::state::SpecPointScope::SelectedCorners {
                corners: vec!["SS".to_owned()],
            },
        ),
    )
    .expect("evidence");
    assert_eq!(
        slow.retained_measurements, 2,
        "only the two SS points were judged"
    );
    assert_eq!(slow.value, 10.5, "the worst of the two SS points");
}

/// A result the executor could not attribute is not proof about a corner. It
/// still answers an unscoped limit, because that limit asked about every point
/// the dataset holds.
#[test]
fn an_unattributed_measurement_never_answers_a_narrowed_specification() {
    let mut run = SimulationRun::new(1);
    run.add_analysis(attributed(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 4.0)]),
    ));

    assert_eq!(
        measurement_in_output_dataset(
            &run,
            &scoped("gain", Some(10.0), crate::state::SpecPointScope::Nominal),
        ),
        None,
        "an unattributed result must not be read as the nominal point"
    );
    assert_eq!(
        measurement_in_output_dataset(
            &run,
            &scoped(
                "gain",
                Some(10.0),
                crate::state::SpecPointScope::SelectedCorners {
                    corners: vec!["TT".to_owned()],
                },
            ),
        ),
        None
    );
    assert!(
        measurement_in_output_dataset(
            &run,
            &scoped("gain", Some(10.0), crate::state::SpecPointScope::AllPoints),
        )
        .is_some(),
        "an unscoped limit still reads every retained point"
    );
}

// ------------------------------------------------- refused-command reporting

/// Run one Simulation Studio frame on the given setup route.
///
/// The drain lives in the surface entry point rather than in any page, so a
/// test of it has to go through `show` and not through `pages::show`.
fn simulate_frame(app: &mut RSpiceApp, page: crate::workbench::state::SimulationPage) {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    app.state.workbench.simulation_page = page;
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1_280.0, 900.0))),
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| super::show(ui, app));
        },
    );
}

/// Seven of the eight routes never draw the lifecycle strip, so a refusal on
/// one of them has to leave the surface to be seen at all. A receipt must not:
/// the registry pages commit routinely, and toasting every one of them would
/// bury the refusals among them.
#[test]
fn a_refusal_away_from_the_analyses_page_is_reported_and_a_receipt_is_not() {
    use crate::ui::widgets::ToastKind;
    use crate::workbench::state::SimulationPage;

    let mut app = RSpiceApp::test_instance();
    app.state
        .workbench
        .analysis_lifecycle_status
        .record_receipt("Receipt #3 · Edit committed for instance 1.");
    simulate_frame(&mut app, SimulationPage::Variables);
    assert!(
        app.state.ui.toasts.activity().is_empty(),
        "a committed receipt is not an error the reader has to be chased with"
    );

    app.state
        .workbench
        .analysis_lifecycle_status
        .record_refusal("Remove rejected fail-closed: another analysis is bound to it.");
    simulate_frame(&mut app, SimulationPage::Variables);
    let reported = app.state.ui.toasts.activity();
    assert_eq!(
        reported.len(),
        1,
        "the refusal reaches the reader exactly once"
    );
    assert_eq!(reported[0].kind(), ToastKind::Error);
    assert!(
        reported[0].message().contains("another analysis is bound"),
        "{}",
        reported[0].message()
    );
}

/// The guard is a sequence, not a snapshot of the message, and it has to hold
/// for as long as the outcome stands. Three of the announcing sites sit on the
/// render path, so an unguarded drain would repeat the same refusal every
/// frame for as long as the plan stayed broken.
#[test]
fn a_standing_refusal_is_reported_once_and_not_again_on_the_next_frame() {
    use crate::workbench::state::SimulationPage;

    let mut app = RSpiceApp::test_instance();
    app.state
        .workbench
        .analysis_lifecycle_status
        .record_refusal("Reorder rejected fail-closed: the position is out of range.");
    simulate_frame(&mut app, SimulationPage::Outputs);
    assert_eq!(app.state.ui.toasts.activity().len(), 1);

    simulate_frame(&mut app, SimulationPage::Outputs);
    simulate_frame(&mut app, SimulationPage::Outputs);
    assert_eq!(
        app.state.ui.toasts.activity().len(),
        1,
        "a refusal that is merely still true is not a new refusal"
    );

    // Restating it verbatim is the render path doing its job, not a new event.
    app.state
        .workbench
        .analysis_lifecycle_status
        .record_refusal("Reorder rejected fail-closed: the position is out of range.");
    simulate_frame(&mut app, SimulationPage::Outputs);
    assert_eq!(app.state.ui.toasts.activity().len(), 1);
}

/// Two refusals in a row are two events, and the second must not be swallowed
/// by the guard that suppresses the first one's restatement.
#[test]
fn two_different_refusals_in_a_row_are_both_reported() {
    use crate::workbench::state::SimulationPage;

    let mut app = RSpiceApp::test_instance();
    app.state
        .workbench
        .analysis_lifecycle_status
        .record_refusal("Design-variable import refused · line 3 · out of range");
    simulate_frame(&mut app, SimulationPage::RunSet);
    app.state
        .workbench
        .analysis_lifecycle_status
        .record_refusal("Run set · blocked · the declared space exceeds the task budget");
    simulate_frame(&mut app, SimulationPage::RunSet);

    let reported = app.state.ui.toasts.activity();
    assert_eq!(reported.len(), 2);
    // The activity centre lists the newest record first.
    assert!(reported[0].message().contains("task budget"));
    assert!(reported[1].message().contains("line 3"));
}

/// A receipt between two refusals must not let the earlier one through again
/// silently: the guard advances on every outcome, refusal or not.
#[test]
fn a_receipt_advances_the_guard_without_reporting_anything() {
    use crate::workbench::state::SimulationPage;

    let mut app = RSpiceApp::test_instance();
    app.state
        .workbench
        .analysis_lifecycle_status
        .record_refusal("Clone rejected fail-closed: the instance no longer exists.");
    simulate_frame(&mut app, SimulationPage::Specifications);
    app.state
        .workbench
        .analysis_lifecycle_status
        .record_receipt("Receipt #4 · Edit committed for instance 2.");
    simulate_frame(&mut app, SimulationPage::Specifications);
    app.state
        .workbench
        .analysis_lifecycle_status
        .record_refusal("Clone rejected fail-closed: the instance no longer exists.");
    simulate_frame(&mut app, SimulationPage::Specifications);

    assert_eq!(
        app.state.ui.toasts.activity().len(),
        2,
        "the refusal is a fresh event once a receipt has replaced it"
    );
}

/// The field is `#[serde(skip)]`, so serde fills it from `Default` and never
/// from the struct literal. A `Default` that was empty would leave the strip
/// blank after every session restore.
#[test]
fn a_restored_session_shows_a_lifecycle_line_rather_than_a_blank_strip() {
    use crate::workbench::state::{AnalysisLifecycleOutcome, WorkbenchState};

    assert!(!AnalysisLifecycleOutcome::default().message().is_empty());

    let mut app = RSpiceApp::test_instance();
    app.state
        .workbench
        .analysis_lifecycle_status
        .record_receipt("Receipt #9 · Insert committed for instance 5.");
    let saved = serde_json::to_value(&app.state.workbench).expect("the workbench serializes");
    let restored: WorkbenchState = serde_json::from_value(saved).expect("the workbench restores");

    assert!(
        !restored.analysis_lifecycle_status.message().is_empty(),
        "a restored session must not open on an empty lifecycle strip"
    );
    assert_eq!(
        restored.analysis_lifecycle_status,
        AnalysisLifecycleOutcome::default(),
        "the outcome is runtime-only, so a restore starts from the default line"
    );
    assert_eq!(restored.analysis_lifecycle_toasted_sequence, 0);
}

/// Identical wording, opposite severity: the strip has to paint these
/// differently, which it can only do by reading the severity.
#[test]
fn the_lifecycle_strip_separates_a_refusal_from_a_receipt_by_severity() {
    fn collect(shape: &egui::epaint::Shape, found: &mut Vec<egui::Color32>) {
        match shape {
            egui::epaint::Shape::Text(text) if text.galley.job.text.starts_with("Instance 4") => {
                found.extend(text.galley.job.sections.iter().map(|s| s.format.color));
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, found);
                }
            }
            _ => {}
        }
    }

    let detail_colors = |refusal: bool| {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        let wording = "Instance 4 · dependency binding · revision 7 to 8.";
        if refusal {
            app.state
                .workbench
                .analysis_lifecycle_status
                .record_refusal(wording);
        } else {
            app.state
                .workbench
                .analysis_lifecycle_status
                .record_receipt(wording);
        }
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(960.0, 240.0))),
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        let _ = lifecycle_receipt_strip(ui, &app);
                    });
            },
        );
        let mut found = Vec::new();
        for clipped in &output.shapes {
            collect(&clipped.shape, &mut found);
        }
        assert!(!found.is_empty(), "the strip paints its detail line");
        (found, Tokens::get(&ctx).color)
    };

    let (receipt, palette) = detail_colors(false);
    let (refusal, _) = detail_colors(true);
    assert_ne!(
        receipt, refusal,
        "the same wording must not paint the same way for a receipt and a refusal"
    );
    assert!(refusal.iter().all(|color| *color == palette.err));
    assert!(receipt.iter().all(|color| *color == palette.text_dim));
}

/// Three announcing sites sit on the render path rather than in a click
/// handler: the surface resolves the active instance, the editor resolves the
/// selection, and the editor re-serializes the draft, each of them every frame.
/// A plan that stays unavailable therefore restates the same refusal forever,
/// and the sequence must not move with it.
#[test]
fn a_render_path_refusal_restated_every_frame_does_not_spin_the_sequence() {
    use crate::workbench::state::SimulationPage;

    let mut app = RSpiceApp::test_instance();
    app.state.sim_setup.analysis_plan = None;
    for _ in 0..4 {
        simulate_frame(&mut app, SimulationPage::Analyses);
    }

    let outcome = &app.state.workbench.analysis_lifecycle_status;
    assert!(outcome.is_refusal(), "{}", outcome.message());
    assert_eq!(
        outcome.sequence(),
        1,
        "four frames of the same standing refusal are one event"
    );
    assert_eq!(app.state.ui.toasts.activity().len(), 1);
}

/// A supply-and-process corner declaration whose transient base analysis
/// measures the divider output. The supply axis is what makes the corners
/// disagree: `V(out)` is half the supply the point was solved at.
fn corner_evidence_run() -> SimulationRun {
    use crate::services::simulation_runner::{
        CornerBaseMode, CornerModelBinding, CornerProcess, CornerRunConfig,
    };

    let deck = "corner evidence\n\
         VDD vdd 0 DC 1.8\n\
         R1 vdd out 1k\n\
         R2 out 0 1k\n\
         C1 out 0 1p\n\
         .tran 1n 100n\n\
         .meas tran vout FIND V(out) AT=100n\n\
         .end\n";
    let binding =
        |process: CornerProcess, label: &str, saturation_current: &str| CornerModelBinding {
            process,
            source_label: label.to_owned(),
            section: Some(process.as_keyword().to_owned()),
            materialized_model_cards: format!(".model DPROCESS D (IS={saturation_current})"),
        };
    let contract = CornerRunConfig {
        process_corners: vec![CornerProcess::TT, CornerProcess::SS],
        voltages: vec![1.8, 1.62],
        supply_source_names: vec!["VDD".to_owned()],
        temperatures_c: vec![27.0, 125.0],
        full_matrix: false,
        nominal_voltage: Some(1.8),
        base_mode: CornerBaseMode::Transient {
            stop_time: 100.0e-9,
            step_time: 1.0e-9,
        },
        model_bindings: vec![
            binding(CornerProcess::TT, "tt.lib", "1e-12"),
            binding(CornerProcess::SS, "ss.lib", "1e-13"),
        ],
        points: Vec::new(),
    };

    crate::simulation::runner::pvt_point_evidence::run_corner_declaration(deck, contract, 27.0)
        .expect("the corner declaration prepares, authorizes and runs")
}

/// The whole claim of the per-point expansion: a specification is answerable
/// corner by corner, against measurements the executor really produced rather
/// than fixtures a test wrote. Before the expansion the corner run retained
/// one scalar per node and no `.MEAS` result at all, so every scope here
/// would have reported that the evidence was missing.
#[test]
fn a_corner_run_answers_a_specification_at_each_of_its_own_points() {
    let run = corner_evidence_run();

    let attributed_points = run
        .analyses
        .iter()
        .filter_map(|analysis| {
            analysis
                .provenance
                .as_ref()
                .and_then(crate::state::AnalysisResultProvenance::pvt_point)
        })
        .count();
    assert_eq!(
        attributed_points, 2,
        "each declared point produced its own attributed result"
    );

    let nominal = measurement_in_output_dataset(
        &run,
        &scoped("vout", Some(0.85), crate::state::SpecPointScope::Nominal),
    )
    .expect("the nominal point answers a nominal limit");
    assert!(
        (nominal.value - 0.9).abs() < 1.0e-3,
        "the nominal point solved at the full supply, got {}",
        nominal.value
    );
    assert!(nominal.is_complete_coverage());

    let everywhere = measurement_in_output_dataset(
        &run,
        &scoped("vout", Some(0.85), crate::state::SpecPointScope::AllPoints),
    )
    .expect("the run set answers an unscoped limit");
    assert!(
        (everywhere.value - 0.81).abs() < 1.0e-3,
        "the worst point is the derated supply, got {}",
        everywhere.value
    );
    assert_eq!(everywhere.retained_measurements, 2);

    let slow = measurement_in_output_dataset(
        &run,
        &scoped(
            "vout",
            Some(0.85),
            crate::state::SpecPointScope::SelectedCorners {
                corners: vec!["SS".to_owned()],
            },
        ),
    )
    .expect("the SS corner answers a limit scoped to it");
    assert_eq!(slow.value, everywhere.value);
    assert_eq!(slow.retained_measurements, 1);

    let bound = scoped("vout", Some(0.85), crate::state::SpecPointScope::Nominal);
    assert!(bound.passes(nominal.value), "nominal holds the limit");
    assert!(
        !bound.passes(everywhere.value),
        "the derated corner does not, which is the verdict the run set had no way to report"
    );

    // The per-point results are additional evidence, not a replacement for the
    // corner family: the same run still carries the axis a corner plot draws.
    let Some(crate::state::AnalysisResultFamilyMetadata::Corner { corner_labels, .. }) = run
        .analyses
        .iter()
        .find(|analysis| analysis.analysis_type == AnalysisType::Corner)
        .and_then(|analysis| analysis.family_metadata.as_ref())
    else {
        panic!("the corner declaration still produces its plotting family");
    };
    assert_eq!(corner_labels.len(), 2);
}

/// A corner that will not solve is a result about that corner, not an absence.
/// Dropping it would let a specification scoped to every point report a pass
/// it was never given evidence for.
#[test]
fn a_corner_point_that_cannot_be_solved_is_retained_as_a_failure() {
    use crate::services::simulation_runner::{
        CornerBaseMode, CornerModelBinding, CornerProcess, CornerRunConfig,
    };

    // The base analysis names a sweep source the deck does not define, so
    // every point fails in the engine rather than in preparation.
    let deck = "corner failure\n\
         VDD vdd 0 DC 1.8\n\
         R1 vdd out 1k\n\
         R2 out 0 1k\n\
         .op\n\
         .end\n";
    let contract = CornerRunConfig {
        process_corners: vec![CornerProcess::TT],
        voltages: vec![1.8, 1.62],
        supply_source_names: vec!["VDD".to_owned()],
        temperatures_c: vec![27.0],
        full_matrix: true,
        nominal_voltage: Some(1.8),
        base_mode: CornerBaseMode::DcSweep {
            source_name: "VMISSING".to_owned(),
            start: 0.0,
            stop: 1.0,
            step: 0.5,
        },
        model_bindings: vec![CornerModelBinding {
            process: CornerProcess::TT,
            source_label: "tt.lib".to_owned(),
            section: Some("TT".to_owned()),
            materialized_model_cards: ".model DPROCESS D (IS=1e-12)".to_owned(),
        }],
        points: Vec::new(),
    };

    let run =
        crate::simulation::runner::pvt_point_evidence::run_corner_declaration(deck, contract, 27.0)
            .expect("a run whose points fail still completes preparation");

    let failed: Vec<_> = run
        .analyses
        .iter()
        .filter(|analysis| !analysis.success)
        .filter(|analysis| {
            analysis
                .provenance
                .as_ref()
                .and_then(crate::state::AnalysisResultProvenance::pvt_point)
                .is_some()
        })
        .collect();
    assert_eq!(failed.len(), 2, "both points are reported, not dropped");
    assert!(
        failed
            .iter()
            .all(|analysis| analysis.error_message.is_some()),
        "a failed point says why it failed"
    );
    assert_eq!(
        measurement_in_output_dataset(
            &run,
            &scoped("vout", Some(0.85), crate::state::SpecPointScope::AllPoints),
        ),
        None,
        "a point that did not solve is not evidence that the limit holds"
    );
}

/// A temperature step whose transient base analysis measures the divider
/// output. The upper leg carries a linear temperature coefficient, so the
/// temperatures disagree by construction: `V(out)` falls as the deck heats up.
fn temperature_evidence_run() -> SimulationRun {
    use crate::services::simulation_runner::{CornerBaseMode, TempRunConfig};

    let deck = "temperature evidence\n\
         VDD vdd 0 DC 1.8\n\
         R1 vdd out 1k TC1=0.01\n\
         R2 out 0 1k\n\
         C1 out 0 1p\n\
         .tran 1n 100n\n\
         .meas tran vout FIND V(out) AT=100n\n\
         .end\n";
    let contract = TempRunConfig {
        temperatures_c: vec![27.0, 125.0],
        base_mode: CornerBaseMode::Transient {
            stop_time: 100.0e-9,
            step_time: 1.0e-9,
        },
    };

    crate::simulation::runner::pvt_point_evidence::run_temperature_declaration(deck, contract, 27.0)
        .expect("the temperature declaration prepares, authorizes and runs")
}

/// The whole claim of the per-temperature expansion: a specification is
/// answerable temperature by temperature, against measurements the executor
/// really produced rather than fixtures a test wrote. Before the expansion a
/// temperature step retained one scalar per node per temperature and no `.MEAS`
/// result at all, so every scope here would have reported that the evidence was
/// missing.
#[test]
fn a_temperature_step_answers_a_specification_at_each_of_its_own_temperatures() {
    let run = temperature_evidence_run();

    // Each declared temperature has its own retained measurement, and the
    // temperature it was solved at is on the result rather than inferred from
    // its position.
    let mut measured: Vec<(f64, f64)> = run
        .analyses
        .iter()
        .filter_map(|analysis| {
            let point = analysis
                .provenance
                .as_ref()
                .and_then(crate::state::AnalysisResultProvenance::pvt_point)?;
            let value = analysis
                .measurements
                .iter()
                .find(|measurement| measurement.name.eq_ignore_ascii_case("vout"))?
                .value?;
            Some((point.temperature_celsius(), value))
        })
        .collect();
    measured.sort_by(|left, right| left.0.total_cmp(&right.0));
    assert_eq!(
        measured.len(),
        2,
        "one measurement per declared temperature"
    );
    assert_eq!(measured[0].0, 27.0);
    assert_eq!(measured[1].0, 125.0);
    assert!(
        (measured[0].1 - 0.9).abs() < 1.0e-3,
        "the 27 C leg is 1 k against 1 k, got {}",
        measured[0].1
    );
    assert!(
        (measured[1].1 - 1.8 / 2.98).abs() < 1.0e-3,
        "the 125 C leg has derated to 1.98 k, got {}",
        measured[1].1
    );

    let nominal = measurement_in_output_dataset(
        &run,
        &scoped("vout", Some(0.85), crate::state::SpecPointScope::Nominal),
    )
    .expect("the reference temperature answers a nominal limit");
    assert!((nominal.value - measured[0].1).abs() < 1.0e-12);
    assert!(nominal.is_complete_coverage());

    let everywhere = measurement_in_output_dataset(
        &run,
        &scoped("vout", Some(0.85), crate::state::SpecPointScope::AllPoints),
    )
    .expect("the run set answers an unscoped limit");
    assert!((everywhere.value - measured[1].1).abs() < 1.0e-12);
    assert_eq!(everywhere.retained_measurements, 2);

    let bound = scoped("vout", Some(0.85), crate::state::SpecPointScope::Nominal);
    assert!(bound.passes(nominal.value), "nominal holds the limit");
    assert!(
        !bound.passes(everywhere.value),
        "the hot temperature does not, which is the verdict the run set had no way to report"
    );

    // A temperature step declares no process axis, so every one of its points
    // solved the run's reference models and a corner-scoped limit is answered
    // by all of them rather than by a subset.
    let reference_process = measurement_in_output_dataset(
        &run,
        &scoped(
            "vout",
            Some(0.85),
            crate::state::SpecPointScope::SelectedCorners {
                corners: vec!["TT".to_owned()],
            },
        ),
    )
    .expect("the reference process answers a limit scoped to it");
    assert_eq!(reference_process.retained_measurements, 2);

    // The per-point results are additional evidence, not a replacement for the
    // parametric family: the same run still carries the axis a plot draws.
    let Some(crate::state::AnalysisResultFamilyMetadata::Parametric { sweep_values, .. }) = run
        .analyses
        .iter()
        .find(|analysis| analysis.analysis_type == AnalysisType::Parametric)
        .and_then(|analysis| analysis.family_metadata.as_ref())
    else {
        panic!("the temperature declaration still produces its plotting family");
    };
    assert_eq!(sweep_values, &vec![27.0, 125.0]);
}
