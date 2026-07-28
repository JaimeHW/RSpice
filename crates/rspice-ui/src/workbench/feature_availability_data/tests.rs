//! Tests that the authored tables match their fixture sources.
//!
//! The claim projection must contain only product cases and must agree with
//! the evidence sources it cites - the tables are hand-written, so these tests
//! are what keeps them honest.

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::OnceLock,
};

use serde_json::Value;

use super::*;
use crate::workbench::state::InteroperabilitySection;

const MOCKUP_ROOT_ENV: &str = "RSPICE_MOCKUP_ROOT";

static PRODUCT_MANIFEST_SOURCE: OnceLock<String> = OnceLock::new();
static PLATFORM_TASK_SOURCE: OnceLock<String> = OnceLock::new();
static CAPABILITY_FIXTURE_SOURCE: OnceLock<String> = OnceLock::new();
static SURFACE_REGISTRY_SOURCE: OnceLock<String> = OnceLock::new();
static APP_SOURCE: OnceLock<String> = OnceLock::new();
static STYLES_SOURCE: OnceLock<String> = OnceLock::new();

fn mockup_root() -> PathBuf {
    std::env::var_os(MOCKUP_ROOT_ENV).map_or_else(
            || {
                panic!(
                    "set {MOCKUP_ROOT_ENV} to the rspice-workbench-host root before running ignored mockup parity tests"
                )
            },
            PathBuf::from,
        )
}

fn read_mockup_source(relative_path: &Path) -> String {
    let path = mockup_root().join(relative_path);
    std::fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!("could not read governed source {}: {error}", path.display())
    })
}

fn cached_mockup_source(cache: &'static OnceLock<String>, relative_path: &str) -> &'static str {
    cache
        .get_or_init(|| read_mockup_source(Path::new(relative_path)))
        .as_str()
}

fn product_manifest_source() -> &'static str {
    cached_mockup_source(&PRODUCT_MANIFEST_SOURCE, "rspice-src/product-manifest.js")
}

fn platform_task_source() -> &'static str {
    cached_mockup_source(
        &PLATFORM_TASK_SOURCE,
        "implementation/platform-task-contract.json",
    )
}

fn capability_fixture_source() -> &'static str {
    cached_mockup_source(
        &CAPABILITY_FIXTURE_SOURCE,
        "implementation/capability-readiness-fixture.json",
    )
}

fn surface_registry_source() -> &'static str {
    cached_mockup_source(
        &SURFACE_REGISTRY_SOURCE,
        "implementation/surface-registry.json",
    )
}

fn app_source() -> &'static str {
    cached_mockup_source(&APP_SOURCE, "rspice-src/app.js")
}

fn styles_source() -> &'static str {
    cached_mockup_source(&STYLES_SOURCE, "rspice-src/styles.css")
}

fn parse_json(source: &str) -> Value {
    serde_json::from_str(source).expect("governed mockup source must be valid JSON")
}

fn source_section<'a>(source: &'a str, start: &str, end: &str) -> &'a str {
    let start = source.find(start).expect("source section start must exist");
    let tail = &source[start..];
    let end = tail.find(end).expect("source section end must exist");
    &tail[..end]
}

fn object_line<'a>(section: &'a str, needle: &str) -> &'a str {
    section
        .lines()
        .find(|line| line.contains(needle))
        .unwrap_or_else(|| panic!("missing source record `{needle}`"))
}

fn planned_specification_source(id: &str) -> &str {
    let marker = format!("if (action === \"{id}\") specification = {{");
    let source = app_source();
    let start = source
        .find(&marker)
        .unwrap_or_else(|| panic!("missing planned specification `{id}`"));
    let tail = &source[start..];
    let end = tail
        .find("\n    };")
        .expect("planned specification must have a closed object literal");
    &tail[..end]
}

fn workspace_owner_id(workspace: Workspace) -> &'static str {
    match workspace {
        Workspace::Project => "project",
        Workspace::Design => "design",
        Workspace::Simulate => "simulate",
        Workspace::Results => "results",
        Workspace::Verify => "verify",
        Workspace::Models => "models",
        Workspace::Netlist => "netlist",
    }
}

fn analysis_source_availability(tier: AnalysisIntendedTier) -> &'static str {
    match tier {
        AnalysisIntendedTier::ReleaseTarget => "production",
        AnalysisIntendedTier::PreviewNonSignOff => "preview",
        AnalysisIntendedTier::CompatibilityPath => "compat",
    }
}

#[test]
fn catalog_counts_and_identities_are_exhaustive_and_unique() {
    assert_eq!(PLATFORM_AVAILABILITY_ROWS.len(), 4);
    assert_eq!(PLANNED_WORKFLOW_ROWS.len(), 11);
    assert_eq!(PLANNED_WORKFLOW_SPECIFICATIONS.len(), 11);
    assert_eq!(ANALYSIS_AVAILABILITY_ROWS.len(), 34);
    assert_eq!(SPECIALIST_WORKSPACE_ROWS.len(), 41);
    assert_eq!(CAPABILITY_CLAIM_PROJECTIONS.len(), 12);

    let platform_ids = PLATFORM_AVAILABILITY_ROWS
        .iter()
        .map(|row| row.id)
        .collect::<HashSet<_>>();
    assert_eq!(platform_ids.len(), PLATFORM_AVAILABILITY_ROWS.len());

    let planned_ids = PLANNED_WORKFLOW_ROWS
        .iter()
        .map(|row| row.id)
        .collect::<HashSet<_>>();
    assert_eq!(planned_ids.len(), PLANNED_WORKFLOW_ROWS.len());
    for row in PLANNED_WORKFLOW_ROWS {
        assert_eq!(row.workflow.as_str(), row.id);
        assert_eq!(row.workflow.label(), row.label);
    }
    let planned_specification_ids = PLANNED_WORKFLOW_SPECIFICATIONS
        .iter()
        .map(|specification| specification.id)
        .collect::<HashSet<_>>();
    assert_eq!(
        planned_specification_ids.len(),
        PLANNED_WORKFLOW_SPECIFICATIONS.len()
    );
    assert_eq!(planned_specification_ids, planned_ids);

    let analysis_ids = ANALYSIS_AVAILABILITY_ROWS
        .iter()
        .map(|row| row.id)
        .collect::<HashSet<_>>();
    let analysis_codes = ANALYSIS_AVAILABILITY_ROWS
        .iter()
        .map(|row| row.code)
        .collect::<HashSet<_>>();
    assert_eq!(analysis_ids.len(), ANALYSIS_AVAILABILITY_ROWS.len());
    assert_eq!(analysis_codes.len(), ANALYSIS_AVAILABILITY_ROWS.len());

    let surfaces = SPECIALIST_WORKSPACE_ROWS
        .iter()
        .map(|row| row.surface_id)
        .collect::<HashSet<_>>();
    assert_eq!(surfaces.len(), SPECIALIST_WORKSPACE_ROWS.len());

    let case_ids = CAPABILITY_CLAIM_PROJECTIONS
        .iter()
        .map(|row| row.case_id)
        .collect::<HashSet<_>>();
    assert_eq!(case_ids.len(), CAPABILITY_CLAIM_PROJECTIONS.len());
}

#[test]
fn interoperability_filters_are_explicit_complete_and_non_mutating() {
    assert_eq!(INTEROPERABILITY_FORMAT_ROWS.len(), 12);
    let identities = INTEROPERABILITY_FORMAT_ROWS
        .iter()
        .map(|row| row.domain_format)
        .collect::<HashSet<_>>();
    assert_eq!(identities.len(), INTEROPERABILITY_FORMAT_ROWS.len());

    let domain_counts = InteroperabilityDomain::ALL.map(|domain| {
        INTEROPERABILITY_FORMAT_ROWS
            .iter()
            .filter(|row| row.matches(domain, InteroperabilitySupportLevel::All))
            .count()
    });
    assert_eq!(domain_counts, [12, 4, 2, 2, 1, 3]);

    let support_counts = InteroperabilitySupportLevel::ALL.map(|support| {
        INTEROPERABILITY_FORMAT_ROWS
            .iter()
            .filter(|row| row.matches(InteroperabilityDomain::All, support))
            .count()
    });
    assert_eq!(support_counts, [12, 2, 8, 4, 0]);

    assert!(INTEROPERABILITY_FORMAT_ROWS.iter().all(|row| {
        row.domain != InteroperabilityDomain::All
            && !row.support_levels.is_empty()
            && !row
                .support_levels
                .contains(&InteroperabilitySupportLevel::All)
    }));
}

#[test]
#[ignore = "requires RSPICE_MOCKUP_ROOT and the separately governed workbench sources"]
fn interoperability_document_copy_matches_the_mockup_exactly() {
    let source = source_section(
        app_source(),
        "if (action === \"interoperability-matrix\")",
        "if (action === \"legal-privacy-center\")",
    );
    assert!(source.contains(&format!("title: \"{INTEROPERABILITY_DIALOG_TITLE}\"")));
    assert!(source.contains(&format!("eyebrow: \"{INTEROPERABILITY_DIALOG_EYEBROW}\"")));
    assert!(source.contains(&format!(
        "localWorkflowTabsMarkup(\"{INTEROPERABILITY_TABLIST_LABEL}\""
    )));
    for section in InteroperabilitySection::ALL {
        assert!(source.contains(&format!("label: \"{}\"", section.label())));
    }
    for domain in InteroperabilityDomain::ALL {
        assert!(source.contains(&format!(">{}</option>", domain.label())));
    }
    for support in InteroperabilitySupportLevel::ALL {
        assert!(source.contains(&format!(">{}</option>", support.label())));
    }

    for row in INTEROPERABILITY_FORMAT_ROWS {
        for text in [
            row.domain_format,
            row.version_dialect,
            row.direction,
            row.round_trip_loss_policy,
        ] {
            assert!(source.contains(&format!("<td>{text}</td>")));
        }
        let release = match row.release_tone.source_class() {
            Some(class) => format!("<td class=\"{class}\">{}</td>", row.release_contract),
            None => format!("<td>{}</td>", row.release_contract),
        };
        assert!(source.contains(&release), "missing `{release}`");
    }
    for note in INTEROPERABILITY_ROUND_TRIP_NOTES {
        assert!(source.contains(&format!("<strong>{}</strong>", note.title)));
        assert!(source.contains(&format!("<p>{}</p>", note.body)));
    }
    for row in INTEROPERABILITY_ROUND_TRIP_GATES {
        for text in [row.gate, row.evidence, row.failure_behavior] {
            assert!(source.contains(&format!("<td>{text}</td>")));
        }
    }
    for row in INTEROPERABILITY_QUALIFICATION_ROWS {
        for text in [
            row.profile,
            row.golden_corpus,
            row.required_comparison,
            row.platform_gate,
        ] {
            assert!(source.contains(&format!("<td>{text}</td>")));
        }
        let release = match row.release_tone.source_class() {
            Some(class) => format!("<td class=\"{class}\">{}</td>", row.release_state),
            None => format!("<td>{}</td>", row.release_state),
        };
        assert!(source.contains(&release), "missing `{release}`");
    }
    assert!(source.contains(INTEROPERABILITY_QUALIFICATION_BOUNDARY));
}

#[test]
#[ignore = "requires RSPICE_MOCKUP_ROOT and the separately governed workbench sources"]
fn touch_edit_guide_document_and_responsive_contract_match_the_mockup_exactly() {
    let source = source_section(
        app_source(),
        "if (action === \"touch-edit-guide\")",
        "if (action === \"platform-lifecycle\")",
    );
    assert!(source.contains(&format!("title: \"{TOUCH_EDIT_GUIDE_DIALOG_TITLE}\"")));
    assert!(source.contains(&format!("eyebrow: \"{TOUCH_EDIT_GUIDE_DIALOG_EYEBROW}\"")));
    assert!(source.contains("primary: \"Close\""));
    for step in TOUCH_EDIT_GUIDE_STEPS {
        assert!(source.contains(&format!(
            "<span class=\"touch-gesture-mark\">{}</span>",
            step.number
        )));
        assert!(source.contains(&format!("<strong>{}</strong>", step.title)));
        assert!(source.contains(&format!("<p>{}</p>", step.body)));
    }
    assert!(source.contains(TOUCH_EDIT_GUIDE_CONCEPT));

    let styles = source_section(
        styles_source(),
        ".touch-edit-guide {",
        ".canvas-breadcrumb {",
    );
    for declaration in [
        "grid-template-columns: repeat(2, minmax(0, 1fr));",
        "grid-template-columns: 32px minmax(0, 1fr);",
        "gap: 10px;",
        "min-height: 96px;",
        "padding: 13px;",
        "width: 30px;",
        "height: 30px;",
        "border-radius: 50%;",
        "@media (max-width: 620px)",
        "grid-template-columns: 1fr;",
    ] {
        assert!(styles.contains(declaration), "missing `{declaration}`");
    }

    let note_body_styles = source_section(
        styles_source(),
        ".dialog-note-grid p {",
        ".detail-record-header {",
    );
    assert!(note_body_styles.contains("font-size: 11px;"));
    assert!(note_body_styles.contains("line-height: 1.45;"));
}

#[test]
#[ignore = "requires RSPICE_MOCKUP_ROOT and the separately governed workbench sources"]
fn platform_lifecycle_document_copy_matches_the_mockup_exactly() {
    let source = source_section(
        app_source(),
        "if (action === \"platform-lifecycle\")",
        "if (action === \"design-bulk-tools\")",
    );
    assert!(source.contains(&format!("title: \"{PLATFORM_LIFECYCLE_DIALOG_TITLE}\"")));
    assert!(source.contains(&format!("eyebrow: \"{PLATFORM_LIFECYCLE_DIALOG_EYEBROW}\"")));
    assert!(source.contains(PLATFORM_LIFECYCLE_WARNING));
    for row in PLATFORM_LIFECYCLE_ROWS {
        for text in [
            row.platform_event,
            row.protected_state,
            row.user_visible_response,
            row.recovery,
        ] {
            assert!(source.contains(&format!("<td>{text}</td>")));
        }
        let eligibility = match row.eligibility_tone.source_class() {
            Some(class) => {
                format!("<td class=\"{class}\">{}</td>", row.release_eligibility)
            }
            None => format!("<td>{}</td>", row.release_eligibility),
        };
        assert!(source.contains(&eligibility), "missing `{eligibility}`");
    }
    for note in PLATFORM_LIFECYCLE_NOTES {
        assert!(source.contains(&format!("<strong>{}</strong>", note.title)));
        assert!(source.contains(&format!("<p>{}</p>", note.body)));
    }
    assert!(source.contains("data-ui-action-contract-id=\"touch-edit-guide\""));
    assert!(source.contains("data-surface-action=\"touch-edit-guide\""));
    assert!(source.contains(&format!(">{TOUCH_EDIT_GUIDE_ACTION_LABEL}</button>")));

    let capability_toolbar = source_section(
        app_source(),
        "function productCapabilityWorkflowMarkup()",
        "function specialistToolBrowserWorkflowMarkup()",
    );
    assert!(capability_toolbar.contains(">Format interoperability…</button>"));
    assert!(capability_toolbar.contains(">Lifecycle behavior…</button>"));
    let help_menu = object_line(app_source(), "help: [[\"help\"");
    assert!(help_menu.contains(
        "[\"compare\", \"Interoperability and format matrix…\", \"\", \"interoperability-matrix\"]"
    ));
}

#[test]
fn analysis_kind_mapping_is_exact_and_covers_the_full_manifest() {
    let mapped = ANALYSIS_AVAILABILITY_ROWS
        .iter()
        .filter_map(|row| row.analysis_kind)
        .collect::<Vec<_>>();
    assert_eq!(mapped.len(), AnalysisKind::ALL.len());
    assert_eq!(
        mapped.iter().copied().collect::<HashSet<_>>().len(),
        mapped.len()
    );

    for kind in AnalysisKind::ALL {
        let row = ANALYSIS_AVAILABILITY_ROWS
            .iter()
            .find(|row| row.analysis_kind == Some(kind))
            .expect("each current analysis kind has exactly one manifest row");
        assert_eq!(row.id, kind.stable_id());
    }

    let absent = ANALYSIS_AVAILABILITY_ROWS
        .iter()
        .filter(|row| row.analysis_kind.is_none())
        .map(|row| row.id)
        .collect::<Vec<_>>();
    assert!(absent.is_empty());
}

#[test]
fn lookup_and_query_helpers_do_not_invent_fallback_rows() {
    assert!(platform_availability_row("not-a-platform").is_none());
    assert!(planned_workflow_row("not-a-workflow").is_none());
    assert!(planned_workflow_specification("not-a-workflow").is_none());
    assert!(analysis_availability_row("not-an-analysis").is_none());
    assert!(specialist_workspace_row(SurfaceId::FeatureAvailability).is_none());
    assert!(capability_claim_projection("not-a-case").is_none());

    assert_eq!(platform_rows_matching("").count(), 4);
    assert_eq!(planned_workflow_rows_matching("").count(), 11);
    assert_eq!(analysis_rows_matching("").count(), 34);
    assert_eq!(specialist_workspace_rows_matching("").count(), 41);
    assert_eq!(capability_claim_rows_matching("").count(), 12);

    assert_eq!(platform_rows_matching("NATIVE DESKTOP").count(), 1);
    assert_eq!(
        planned_workflow_rows_matching("pelgrom coefficient").count(),
        1
    );
    assert_eq!(
        planned_workflow_rows_matching("round-trip comparison").count(),
        1
    );
    assert_eq!(analysis_rows_matching("qpnoise").count(), 1);
    assert_eq!(specialist_workspace_rows_matching("models-pdk").count(), 8);
    assert_eq!(capability_claim_rows_matching("unverified").count(), 1);
}

#[test]
#[ignore = "requires RSPICE_MOCKUP_ROOT and the separately governed workbench sources"]
fn platform_rows_match_both_governed_sources_exactly() {
    const DIMENSIONS: [&str; 12] = [
        "viewport",
        "pointer",
        "keyboard",
        "stylus",
        "renderer",
        "storage",
        "filesystem",
        "network",
        "lifecycle",
        "windowing",
        "printing",
        "assistive-technology",
    ];

    let contract = parse_json(platform_task_source());
    let fixture = parse_json(capability_fixture_source());
    for row in PLATFORM_AVAILABILITY_ROWS {
        let platform = &contract["platforms"][row.id];
        let target = &fixture["platformTargets"][row.id];
        assert_eq!(platform["id"].as_str(), Some(row.id));
        assert_eq!(platform["label"].as_str(), Some(row.label));
        assert_eq!(
            platform["qualificationBoundary"].as_str(),
            Some(row.design_qualification_boundary)
        );
        assert_eq!(target["state"].as_str(), Some(row.fixture_state.as_str()));
        assert_eq!(
            target["qualification"].as_str(),
            Some(row.fixture_qualification)
        );
        assert_eq!(
            target["boundary"].as_str(),
            Some(row.qualification_boundary)
        );
        assert_eq!(row.sign_off_rule, PLATFORM_SIGN_OFF_RULE);

        let summary = DIMENSIONS
            .iter()
            .map(|dimension| {
                format!(
                    "{dimension}: {}",
                    platform["capabilities"][dimension]["mode"]
                        .as_str()
                        .expect("every governed capability has a mode")
                )
            })
            .collect::<Vec<_>>()
            .join(" \u{00b7} ");
        assert_eq!(summary, row.capability_mode_summary);
    }
}

#[test]
#[ignore = "requires RSPICE_MOCKUP_ROOT and the separately governed workbench sources"]
fn planned_and_analysis_rows_match_the_product_manifest_source() {
    let analyses = source_section(
        product_manifest_source(),
        "const analyses = [",
        "].map((analysis)",
    );
    for row in ANALYSIS_AVAILABILITY_ROWS {
        let line = object_line(analyses, &format!("{{ id: {:?}", row.id));
        assert!(line.contains(&format!("code: {:?}", row.code)));
        assert!(line.contains(&format!("title: {:?}", row.title)));
        assert!(line.contains(&format!(
            "availability: {:?}",
            analysis_source_availability(row.intended_tier)
        )));
    }

    let planned = source_section(
        product_manifest_source(),
        "const plannedCapabilityDesigns = [",
        "];",
    );
    for row in PLANNED_WORKFLOW_ROWS {
        let line = object_line(planned, &format!("{{ id: {:?}", row.id));
        assert!(line.contains(&format!("label: {:?}", row.label)));
        assert!(line.contains(&format!("group: {:?}", row.group)));
        assert!(line.contains(&format!("owner: {:?}", row.owner)));
        assert!(line.contains(&format!("entry: {:?}", row.entry)));
        assert!(line.contains(&format!("status: {:?}", row.status.as_str())));
    }
}

#[test]
#[ignore = "requires RSPICE_MOCKUP_ROOT and the separately governed workbench sources"]
fn planned_workflow_shared_frame_matches_the_mockup_exactly() {
    let frame = source_section(
        app_source(),
        "function plannedCapabilityFrame",
        "function plannedCapabilityWorkflowMarkup",
    );
    assert!(frame.contains(&format!(
        "data-capability-design-status=\"{}\"",
        PLANNED_WORKFLOW_FRAME.design_status
    )));
    assert!(frame.contains(&format!(
        "data-capability-executable=\"{}\"",
        PLANNED_WORKFLOW_FRAME.executable
    )));
    assert!(frame.contains(&format!(
        "{}${{escapeHtml(meta.status)}}{}",
        PLANNED_WORKFLOW_FRAME.unavailable_heading_prefix,
        PLANNED_WORKFLOW_FRAME.unavailable_heading_suffix
    )));
    assert!(frame.contains(PLANNED_WORKFLOW_FRAME.unavailable_explanation));
    assert!(frame.contains(PLANNED_WORKFLOW_FRAME.ownership_section_title));
    assert!(frame.contains(PLANNED_WORKFLOW_FRAME.ownership_state));
    for field_label in PLANNED_WORKFLOW_FRAME.ownership_field_labels {
        assert!(frame.contains(field_label));
    }
    assert!(frame.contains(PLANNED_WORKFLOW_FRAME.task_sequence_section_title));
    assert!(frame.contains(PLANNED_WORKFLOW_FRAME.task_sequence_route_state));
    for step in PLANNED_WORKFLOW_FRAME.task_steps {
        assert!(frame.contains(step));
    }
    assert!(frame.contains(PLANNED_WORKFLOW_FRAME.validation_section_title));
    assert!(frame.contains(PLANNED_WORKFLOW_FRAME.outputs_section_title));
    assert!(frame.contains(PLANNED_WORKFLOW_FRAME.implementation_boundary));

    assert!(app_source().contains(&format!(
        "${{meta.label}}{}",
        PLANNED_WORKFLOW_FRAME.dialog_title_suffix
    )));
    assert!(app_source().contains(&format!(
        "${{meta.group.toUpperCase()}} \u{00b7} {} \u{00b7} ${{meta.status.toUpperCase()}}",
        PLANNED_WORKFLOW_FRAME.dialog_eyebrow_status
    )));
    assert!(app_source().contains(&format!(
        "primary: {:?}",
        PLANNED_WORKFLOW_FRAME.primary_action
    )));
}

#[test]
#[ignore = "requires RSPICE_MOCKUP_ROOT and the separately governed workbench sources"]
fn all_eleven_planned_workflow_specifications_match_the_mockup_exactly() {
    let metadata_ids = PLANNED_WORKFLOW_ROWS
        .iter()
        .map(|row| row.id)
        .collect::<Vec<_>>();
    let specification_ids = PLANNED_WORKFLOW_SPECIFICATIONS
        .iter()
        .map(|specification| specification.id)
        .collect::<Vec<_>>();
    assert_eq!(specification_ids, metadata_ids);

    for specification in PLANNED_WORKFLOW_SPECIFICATIONS {
        assert_eq!(
            planned_workflow_specification(specification.id),
            Some(&specification)
        );
        let source = planned_specification_source(specification.id);
        assert_eq!(
            app_source()
                .matches(&format!(
                    "if (action === \"{}\") specification = {{",
                    specification.id
                ))
                .count(),
            1,
            "each canonical specification has exactly one mockup definition"
        );
        assert!(source.contains(&format!("purpose: {:?}", specification.purpose)));
        assert!(source.contains(&format!(
            "<div class=\"settings-section-label\">{}</div>",
            specification.content_section_title
        )));
        assert!(source.contains(&format!(
            "validation: {:?}",
            specification.validation_recovery
        )));
        assert!(source.contains(&format!("outputs: {:?}", specification.outputs_provenance)));

        assert_eq!(
            source.matches("<table class=\"data-table\">").count(),
            specification.tables.len(),
            "table count differs for {}",
            specification.id
        );
        let expected_header_count = specification
            .tables
            .iter()
            .map(|table| table.headers.len())
            .sum::<usize>();
        let expected_cell_count = specification
            .tables
            .iter()
            .flat_map(|table| table.rows.iter())
            .map(|row| row.len())
            .sum::<usize>();
        assert_eq!(source.matches("<th>").count(), expected_header_count);
        assert_eq!(source.matches("<td").count(), expected_cell_count);

        for table in specification.tables {
            assert!(!table.headers.is_empty());
            assert!(!table.rows.is_empty());
            for header in table.headers {
                assert!(source.contains(&format!("<th>{header}</th>")));
            }
            for row in table.rows {
                assert_eq!(row.len(), table.headers.len());
                for cell in *row {
                    let fragment = match cell.style.source_class() {
                        Some(class) => {
                            format!("<td class=\"{class}\">{}</td>", cell.text)
                        }
                        None => format!("<td>{}</td>", cell.text),
                    };
                    assert!(
                        source.contains(&fragment),
                        "missing exact table cell `{fragment}` for {}",
                        specification.id
                    );
                }
            }
        }

        if let Some(title) = specification.property_section_title {
            assert!(source.contains(&format!("<strong>{title}</strong>")));
        }
        if specification.properties.is_empty() {
            assert!(!source.contains("plannedCapabilityProperties(["));
        } else {
            let property_start = source
                .find("plannedCapabilityProperties([")
                .expect("property-backed design must have the property helper");
            let property_tail = &source[property_start..];
            let property_end = property_tail
                .find("]) }")
                .or_else(|| property_tail.find("])}</section>"))
                .expect("property helper must close in the specification");
            let property_source = &property_tail[..property_end];
            assert_eq!(
                property_source.matches("[\"").count(),
                specification.properties.len(),
                "property count differs for {}",
                specification.id
            );
            for property in specification.properties {
                let fragment = match property.style.source_class() {
                    Some(class) => format!(
                        "[\"{}\", \"{}\", \"{class}\"]",
                        property.label, property.value
                    ),
                    None => format!("[\"{}\", \"{}\"]", property.label, property.value),
                };
                assert!(
                    property_source.contains(&fragment),
                    "missing exact property `{fragment}` for {}",
                    specification.id
                );
            }
        }

        match specification.chip_section {
            Some(section) => {
                assert!(source.contains(&format!(
                    "<div class=\"settings-section-label\">{}</div>",
                    section.title
                )));
                assert_eq!(
                    source.matches("<span class=\"mini-badge\">").count(),
                    section.chips.len()
                );
                for chip in section.chips {
                    assert!(source.contains(&format!("<span class=\"mini-badge\">{chip}</span>")));
                }
            }
            None => assert_eq!(source.matches("<span class=\"mini-badge\">").count(), 0),
        }

        match specification.content_layout {
            PlannedWorkflowContentLayout::TableStackAndProperties => {
                assert!(!specification.properties.is_empty());
            }
            PlannedWorkflowContentLayout::SplitTables => {
                assert_eq!(specification.tables.len(), 2);
                assert!(specification.properties.is_empty());
            }
        }
    }
}

#[test]
#[ignore = "requires RSPICE_MOCKUP_ROOT and the separately governed workbench sources"]
fn specialist_rows_match_manifest_and_registry_without_runtime_inference() {
    let manifest = source_section(
        product_manifest_source(),
        "const commercialWorkspaces = {",
        "const plannedCapabilityDesigns = [",
    );
    let registry = parse_json(surface_registry_source());
    let surfaces = registry["surfaces"]
        .as_array()
        .expect("surface registry has a surface array");

    for row in SPECIALIST_WORKSPACE_ROWS {
        let line = object_line(manifest, &format!("{:?}: {{", row.surface_id.as_str()));
        assert!(line.contains(&format!("owner: {:?}", workspace_owner_id(row.owner))));
        assert!(line.contains(&format!("tier: {:?}", row.tier.as_str())));
        assert!(line.contains(&format!("purpose: {:?}", row.purpose)));

        let surface = surfaces
            .iter()
            .find(|surface| surface["id"].as_str() == Some(row.surface_id.as_str()))
            .expect("every workspace row has one registry surface");
        let identity = &surface["identityAndCommercialModule"];
        assert_eq!(
            identity["archetypeId"].as_str(),
            Some("specialist-workspace")
        );
        assert_eq!(identity["label"].as_str(), Some(row.label()));
        assert_eq!(identity["canonicalPurpose"].as_str(), Some(row.purpose));
        assert_eq!(identity["canonicalTier"].as_str(), Some(row.tier.as_str()));
        assert_eq!(
            identity["primaryModuleId"].as_str(),
            Some(row.primary_module_id)
        );
        assert_eq!(
            identity["moduleAvailability"].as_str(),
            Some(row.module_availability.as_str())
        );

        assert_eq!(
            row.runtime_availability(),
            surface_availability(row.surface_id),
            "workspace data may not infer a route executor"
        );
    }
}

#[test]
#[ignore = "requires RSPICE_MOCKUP_ROOT and the separately governed workbench sources"]
fn engineering_profile_identity_copy_and_visibility_match_the_mockup_exactly() {
    let governed = [
        (
            EngineeringProfile::AnalogIc,
            "\"analog-ic\": {",
            "\"rf-microwave\": {",
        ),
        (
            EngineeringProfile::RfMicrowave,
            "\"rf-microwave\": {",
            "\"si-pi\": {",
        ),
        (EngineeringProfile::SiPi, "\"si-pi\": {", "\"power\": {"),
        (EngineeringProfile::Power, "\"power\": {", "\"emerging\": {"),
        (
            EngineeringProfile::Emerging,
            "\"emerging\": {",
            "\"all\": {",
        ),
    ];

    for (profile, start, end) in governed {
        let source = source_section(app_source(), start, end);
        assert!(source.contains(&format!("label: {:?}", profile.label())));
        assert!(source.contains(&format!("detail: {:?}", profile.detail())));
        for row in SPECIALIST_WORKSPACE_ROWS {
            let present = source.contains(&format!("\"{}\"", row.surface_id.as_str()));
            assert_eq!(
                row.shown_in_profile(profile),
                present,
                "{} profile mismatch for {}",
                profile.id(),
                row.surface_id
            );
        }
    }

    let all_source = source_section(
        app_source(),
        "\"all\": {",
        "function activeProductProfile()",
    );
    assert!(all_source.contains("actions: Object.keys(COMMERCIAL_WORKSPACE_META)"));
    assert!(
        SPECIALIST_WORKSPACE_ROWS
            .into_iter()
            .all(|row| row.shown_in_profile(EngineeringProfile::All))
    );
}

fn entitlement_state(value: &str) -> EntitlementState {
    match value {
        "granted" => EntitlementState::Granted,
        "denied" => EntitlementState::Denied,
        "expired" => EntitlementState::Expired,
        "revoked" => EntitlementState::Revoked,
        "unknown" => EntitlementState::Unknown,
        other => panic!("unknown entitlement state `{other}`"),
    }
}

fn expected_evidence_classification(evidence: &[&Value], as_of: &str) -> EvidenceClassification {
    if evidence.is_empty() {
        return EvidenceClassification::Missing;
    }
    if evidence
        .iter()
        .any(|record| record["verification"].as_str() != Some("verified"))
    {
        return EvidenceClassification::Unverified;
    }
    if evidence.iter().any(|record| {
        record["currentness"].as_str() == Some("stale")
            || record["expiresAt"]
                .as_str()
                .is_some_and(|expires_at| expires_at <= as_of)
    }) {
        return EvidenceClassification::Stale;
    }
    EvidenceClassification::Bound
}

fn evidence_source_summary(evidence: &[&Value]) -> String {
    if evidence.is_empty() {
        return "No applicable evidence binding".to_owned();
    }
    evidence
        .iter()
        .map(|record| {
            format!(
                "{} \u{00b7} {} \u{00b7} {}@{} \u{00b7} {}/{}",
                record["id"].as_str().expect("evidence ID"),
                record["source"]["authorityClass"]
                    .as_str()
                    .expect("source authority"),
                record["source"]["locator"]
                    .as_str()
                    .expect("source locator"),
                record["source"]["revision"]
                    .as_str()
                    .expect("source revision"),
                record["currentness"].as_str().expect("currentness"),
                record["verification"].as_str().expect("verification")
            )
        })
        .collect::<Vec<_>>()
        .join(" | ")
}

#[test]
#[ignore = "requires RSPICE_MOCKUP_ROOT and the separately governed workbench sources"]
fn claim_projection_contains_only_product_cases_and_matches_fixture_sources() {
    let fixture = parse_json(capability_fixture_source());
    assert_eq!(fixture["status"].as_str(), Some(CAPABILITY_FIXTURE_STATUS));
    assert_eq!(
        fixture["fixtureRevision"].as_str(),
        Some(CAPABILITY_FIXTURE_REVISION)
    );
    assert_eq!(fixture["asOf"].as_str(), Some(CAPABILITY_FIXTURE_AS_OF));
    assert_eq!(
        fixture["boundary"]["doesNotClaim"].as_str(),
        Some(CAPABILITY_FIXTURE_BOUNDARY)
    );

    let cases = fixture["resolutionCases"]
        .as_array()
        .expect("fixture has resolution cases")
        .iter()
        .filter(|case| case["claimMode"].as_str() == Some("product"))
        .collect::<Vec<_>>();
    assert_eq!(cases.len(), CAPABILITY_CLAIM_PROJECTIONS.len());
    let evidence_records = fixture["evidence"]
        .as_array()
        .expect("fixture has evidence records");
    let entitlements = fixture["entitlements"]
        .as_array()
        .expect("fixture has entitlement records");

    for (case, row) in cases.into_iter().zip(CAPABILITY_CLAIM_PROJECTIONS) {
        assert_eq!(case["id"].as_str(), Some(row.case_id));
        assert_eq!(case["label"].as_str(), Some(row.original_label));
        assert_eq!(case["subjectKind"].as_str(), Some(row.subject_kind));
        assert_eq!(case["assertedStage"].as_str(), Some(row.asserted_stage));
        assert_eq!(case["expected"]["state"].as_str(), Some(row.state.as_str()));
        assert_eq!(
            case["expected"]["labelAllowed"].as_bool(),
            Some(row.label_allowed)
        );

        let expected_label = if row.label_allowed {
            row.original_label.to_owned()
        } else {
            format!("Claim blocked \u{00b7} {}", row.original_label)
        };
        assert_eq!(row.resolved_label, expected_label);

        let entitlement_id = case["targets"]["entitlementPolicyId"]
            .as_str()
            .expect("product case has an entitlement target");
        let entitlement = entitlements
            .iter()
            .find(|record| record["id"].as_str() == Some(entitlement_id))
            .expect("entitlement target resolves exactly");
        assert_eq!(
            row.entitlement_state,
            entitlement_state(
                entitlement["grantState"]
                    .as_str()
                    .expect("entitlement has grant state")
            )
        );

        let bound_evidence = case["evidenceBindingIds"]
            .as_array()
            .expect("case has an evidence binding array")
            .iter()
            .map(|id| {
                let id = id.as_str().expect("evidence binding ID is a string");
                evidence_records
                    .iter()
                    .find(|record| record["id"].as_str() == Some(id))
                    .expect("every fixture binding resolves exactly")
            })
            .collect::<Vec<_>>();
        assert_eq!(
            row.evidence_classification,
            expected_evidence_classification(&bound_evidence, CAPABILITY_FIXTURE_AS_OF)
        );
        assert_eq!(
            row.applicable_source_summary,
            evidence_source_summary(&bound_evidence)
        );

        for required_reason in case["expected"]["requiredReasonCodes"]
            .as_array()
            .expect("expected required reason code array")
        {
            let required_reason = required_reason
                .as_str()
                .expect("required reason code is a string");
            assert!(row.reason_codes.contains(&required_reason));
        }
        assert_eq!(row.fixture_revision, CAPABILITY_FIXTURE_REVISION);
        assert_eq!(row.as_of, CAPABILITY_FIXTURE_AS_OF);
        assert_eq!(row.boundary, CAPABILITY_FIXTURE_BOUNDARY);
        assert_eq!(row.label_allowed, row.state.is_current());
    }

    let contract_test_vector_ids = fixture["resolutionCases"]
        .as_array()
        .expect("fixture has resolution cases")
        .iter()
        .filter(|case| case["claimMode"].as_str() == Some("contract-test-vector"))
        .filter_map(|case| case["id"].as_str())
        .collect::<HashSet<_>>();
    assert_eq!(contract_test_vector_ids.len(), 2);
    assert!(
        CAPABILITY_CLAIM_PROJECTIONS
            .iter()
            .all(|row| !contract_test_vector_ids.contains(row.case_id))
    );
}
