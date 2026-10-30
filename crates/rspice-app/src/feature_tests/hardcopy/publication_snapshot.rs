//! The publication snapshot built from the session's open project: its
//! sheets, engineering projection, run results and deck.

use crate::feature_tests::hardcopy::publication_reads;
use crate::state::AnalysisResult;
use crate::state::AnalysisType;
use crate::state::RetainedTrace;
use crate::workbench::app_state::AppState;
use crate::workbench::framework::feature::PublicationDraft;
use crate::workbench::hardcopy_adapters::publication_snapshot::*;
use rspice_publication_contract::AnalysisRecord;
use rspice_publication_contract::Dataset;
use rspice_publication_contract::PublicationSection;
use rspice_publication_contract::PublicationSnapshot;
use rspice_publication_contract::ResultsSection;
use rspice_publication_contract::SignalTarget;
use rspice_publication_contract::SweepAxis;
use rspice_publication_contract::Trace;
use rspice_publication_contract::TraceValues;
use std::sync::Arc;

fn waveform(name: &str, x: &[f64], y: &[f64]) -> RetainedTrace {
    RetainedTrace::new(name, Arc::new(x.to_vec()), Arc::new(y.to_vec()), "#000000")
}

fn analysis(label: &str, waveforms: Vec<RetainedTrace>) -> AnalysisResult {
    AnalysisResult {
        data: rspice_results::analysis_result::AnalysisResult {
            convergence: None,
            id: 1,
            analysis_type: AnalysisType::Transient,
            label: label.to_string(),
            timestamp: 0.0,
            waveforms,
            dc_op: None,
            device_op: None,
            noise_summary: None,
            family_metadata: None,
            result_payload: None,
            native_scalar_units: None,
            monte_carlo_checkpoint: None,
            transient_checkpoint: None,
            measurements: Vec::new(),
            checks: Default::default(),
            saved_output_receipts: Vec::new(),
            success: true,
            error_message: None,
            failure_attribution: None,
            failure_capability: None,
            provenance: None,
            import_source: None,
        },
    }
}

#[test]
fn deck_line_endings_normalize_to_canonical_form() {
    let mut state = AppState::default();
    state.model.simulation.source.netlist_content =
        "* RSpice Netlist\r\nR1 a b 1k\r\n.end".to_string();
    let deck = effective_deck(publication_reads(&state))
        .expect("deck")
        .deck;
    assert!(!deck.contains('\r'));
    assert_eq!(deck.lines().count(), 3);
}

#[test]
fn authored_presentation_and_disclosure_choices_are_sealed() {
    let mut state = AppState::default();
    state.model.simulation.source.netlist_content = "* Filter\nR1 in out 1k\n.end".to_string();
    let snapshot = build_publication_snapshot(
        publication_reads(&state),
        &PublicationDraft {
            title: "Filter".to_string(),
            description: "Short page description".to_string(),
            author_display: "Test".to_string(),
            created_utc: "2026-08-06T00:00:00Z".to_string(),
            overview_narrative: "A low-pass filter used at the converter input.".to_string(),
            specification_label: "Cutoff".to_string(),
            specification_value: "1.59".to_string(),
            specification_unit: "kHz".to_string(),
            ..Default::default()
        },
    )
    .expect("netlist publication builds");

    let presentation = snapshot.presentation.expect("v3 presentation");
    assert_eq!(
        presentation.section_order,
        vec![
            PublicationSection::Overview,
            PublicationSection::Files,
            PublicationSection::Details,
        ]
    );
    assert_eq!(presentation.default_section, PublicationSection::Files);
    let overview = presentation.overview.expect("authored overview");
    assert_eq!(
        overview.narrative,
        "A low-pass filter used at the converter input."
    );
    assert_eq!(overview.specifications[0].label, "Cutoff");
    assert_eq!(overview.specifications[0].unit.as_deref(), Some("kHz"));

    assert!(matches!(
        build_publication_snapshot(
            publication_reads(&state),
            &PublicationDraft {
                title: "Withheld".to_string(),
                author_display: "Test".to_string(),
                created_utc: "2026-08-06T00:00:00Z".to_string(),
                include_netlist: false,
                ..Default::default()
            }
        ),
        Err(PublicationBuildError::NothingToPublish)
    ));
}

#[test]
fn empty_projects_report_nothing_to_publish() {
    let state = AppState::default();
    assert!(matches!(
        build_publication_snapshot(
            publication_reads(&state),
            &PublicationDraft {
                title: "Empty".to_string(),
                description: String::new(),
                author_display: "Test".to_string(),
                created_utc: "2026-08-06T00:00:00Z".to_string(),
                license: rspice_publication_contract::ContentLicense::AllRightsReserved,
                ..Default::default()
            }
        ),
        Err(PublicationBuildError::NothingToPublish)
    ));
}

#[test]
fn a_multi_sheet_cell_view_publishes_each_sheet_once() {
    use crate::state::{SheetDefinition, SheetPortPolicy, SheetTemplate, Wire};
    use crate::workbench::state::WorkspaceDocumentId;

    let mut state = AppState::default();
    // Both sheets draw their only wire at the same coordinates, so a page
    // that ignored the sheet partition would show them overlaid.
    for wire_id in [11, 22] {
        state
            .model
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::segment(
                wire_id,
                crate::state::Point::new(0, 0),
                crate::state::Point::new(20, 0),
            ));
    }
    let reference = state.model.workspace.content.active_view.clone();
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let active_key = state.model.workspace.content.active_key();
    let design_management = &mut state.model.workspace.content.design_management;
    let first = design_management
        .bootstrap_for_cell_view(&active_key, "First", [11])
        .unwrap();
    let catalog = design_management.sheet_catalog_mut(&active_key).unwrap();
    let second = catalog
        .create_sheet(
            SheetDefinition {
                name: "Second".to_owned(),
                template: SheetTemplate::AnalogSchematic,
                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: None,
            },
            Some(first),
        )
        .unwrap();
    catalog
        .assign_objects(catalog.revision(), second, [22])
        .unwrap();

    let (sheets, plots) = collect_scenes(publication_reads(&state)).unwrap();
    let path = state.model.workspace.content.active_display_path();
    assert_eq!(
        sheets
            .iter()
            .map(|sheet| sheet.name.as_str())
            .collect::<Vec<_>>(),
        [format!("{path} · First"), format!("{path} · Second")]
    );
    assert!(plots.is_empty());
}

#[test]
fn native_engineering_projection_preserves_components_nets_and_signal_identity() {
    let mut state = AppState::default();
    state.model.schematic.add_component(
        crate::state::ComponentType::Resistor,
        crate::state::Point::new(0, 0),
    );

    let projected = engineering_publication(publication_reads(&state), true, None)
        .expect("the fixture design resolves");
    assert_eq!(projected.components.len(), 1);
    let resistor = &projected.components[0];
    assert_eq!(resistor.reference, "R1");
    assert_eq!(resistor.device, "Resistor");
    assert_eq!(resistor.pins.len(), 2);
    assert!(resistor.pins.iter().all(|pin| pin.net.is_some()));
    assert!(projected.nets.iter().all(|net| {
        net.connections.iter().all(|connection| {
            connection.component_reference == "R1"
                && resistor
                    .pins
                    .iter()
                    .any(|pin| pin.name == connection.pin_name)
        })
    }));

    let net_name = resistor.pins[0].net.as_deref().expect("resolved net");
    let results = ResultsSection {
        analyses: vec![AnalysisRecord {
            id: 1,
            label: "Operating point".to_owned(),
            card: ".op".to_owned(),
        }],
        datasets: vec![Dataset {
            id: 7,
            analysis_id: 1,
            name: "Operating point".to_owned(),
            variant: None,
            sweep: SweepAxis {
                label: "point".to_owned(),
                unit: String::new(),
                values_bits: vec![0.0_f64.to_bits()],
            },
            traces: vec![
                Trace {
                    label: format!("V({net_name})"),
                    unit: "V".to_owned(),
                    values: TraceValues::Real {
                        bits: vec![1.0_f64.to_bits()],
                    },
                },
                Trace {
                    label: "i(r1)".to_owned(),
                    unit: "A".to_owned(),
                    values: TraceValues::Real {
                        bits: vec![0.001_f64.to_bits()],
                    },
                },
                Trace {
                    label: "V(out) - V(in)".to_owned(),
                    unit: "V".to_owned(),
                    values: TraceValues::Real {
                        bits: vec![0.5_f64.to_bits()],
                    },
                },
            ],
        }],
        measurements: Vec::new(),
    };

    let projected = engineering_publication(publication_reads(&state), true, Some(&results))
        .expect("the fixture design resolves");
    assert!(matches!(
        &projected.signals[0].target,
        SignalTarget::NetVoltage { net } if net == net_name
    ));
    assert!(matches!(
        &projected.signals[1].target,
        SignalTarget::DeviceCurrent { reference } if reference == "R1"
    ));
    assert!(matches!(
        &projected.signals[2].target,
        SignalTarget::Expression { label } if label == "V(out) - V(in)"
    ));

    let withheld = engineering_publication(publication_reads(&state), false, Some(&results))
        .expect("a withheld schematic needs no design at all");
    assert!(withheld.components.is_empty());
    assert!(withheld.nets.is_empty());
    assert!(
        withheld
            .signals
            .iter()
            .all(|signal| matches!(signal.target, SignalTarget::Expression { .. }))
    );
}

#[test]
fn run_results_and_deck_build_a_valid_snapshot() {
    let mut state = AppState::default();
    state.model.simulation.source.netlist_content =
        "* RSpice Netlist\nR1 in out 1k\n.end".to_string();
    let mut run = crate::state::SimulationRun::new(1);
    run.analyses.push({
        let mut result = analysis(
            "Transient",
            vec![waveform("V(out)", &[0.0, 1.0, 2.0], &[0.0, 0.5, 0.8])],
        );
        result.measurements.push(rspice_core::MeasureResult {
            name: "final".to_string(),
            value: Some(2.0),
            raw_value: Some(-0.8),
            error: None,
            passed: true,
            expected: None,
            tolerance: None,
            failure_limit: Some(1.0),
            failure_limit_exceeded: false,
            event_axis: Some(2.0),
            units: None,
        });
        result
    });
    run.success = true;
    state.model.simulation.retained.runs.push(run);
    state.model.simulation.view.active_run_idx = Some(0);

    let snapshot = build_publication_snapshot(
        publication_reads(&state),
        &PublicationDraft {
            title: "RC deck".to_string(),
            description: String::new(),
            author_display: "Test".to_string(),
            created_utc: "2026-08-06T00:00:00Z".to_string(),
            license: rspice_publication_contract::ContentLicense::CcBy40,
            ..Default::default()
        },
    )
    .expect("snapshot builds");
    assert!(snapshot.netlist.is_some());
    let results = snapshot.results.as_ref().expect("results");
    assert_eq!(results.analyses.len(), 1);
    assert_eq!(results.datasets.len(), 1);
    assert_eq!(results.measurements.len(), 1);
    assert_eq!(results.measurements[0].value_bits, Some(2.0_f64.to_bits()));
    let fail_value = results.measurements[0]
        .fail_value
        .as_ref()
        .expect("projected measurement retains FAILVALUE evidence");
    assert_eq!(fail_value.raw_value_bits, Some((-0.8_f64).to_bits()));
    assert_eq!(fail_value.failure_limit_bits, 1.0_f64.to_bits());
    assert!(!fail_value.exceeded);
    assert!(snapshot.disclosure.netlist);
    assert!(snapshot.disclosure.results);
    assert!(!snapshot.disclosure.schematic);

    let canonical = snapshot.canonical_bytes().expect("canonical snapshot");
    let reparsed =
        PublicationSnapshot::from_canonical_bytes(&canonical).expect("published evidence reparses");
    assert_eq!(snapshot, reparsed);
}
