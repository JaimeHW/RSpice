//! The hardcopy worker's operations over sources resolved from the session:
//! resolution, preview and publication, each bound to its request.

use crate::state::{Point, Wire};
use crate::workbench::AppState;
use crate::workbench::hardcopy_adapters::sources::{
    PreparedRetainedHardcopyResolution, SourceReads, prepare_retained_hardcopy_resolution,
    resolve_retained_hardcopy_source,
};
use crate::workbench::state::WorkspaceDocumentId;
use rspice_formats::zip::deterministic_stored_zip;
use rspice_hardcopy::render::{HardcopyPreviewPage, RenderedHardcopyPublication};
use rspice_hardcopy::render::{HardcopyRenderer, HardcopySceneMetadata};
use rspice_hardcopy::sources::ResolvedHardcopyDocument;
use rspice_hardcopy::worker::HARDCOPY_WORKER_PROTOCOL_VERSION;
use rspice_hardcopy::worker::HardcopyWorkerCommand;
use rspice_hardcopy::worker::HardcopyWorkerOperation;
use rspice_hardcopy::worker::HardcopyWorkerRequest;
use rspice_hardcopy::worker::HardcopyWorkerResponse;
use rspice_hardcopy::worker::decode_packaged_publication;
use rspice_hardcopy::worker::execute_request;
use rspice_hardcopy::worker::publication_part_count;
use rspice_hardcopy_contract::{
    BackgroundMode, Bleed, ColorMapping, DecorationSetup, FontPolicy, OutsideSheetContentPolicy,
    PageMargins, PhysicalPageSetup, RenderSetup, RenderTarget, ScaleMode, SchematicHardcopyExtent,
    SchematicHardcopySetup, TilingMode, TilingSetup, Watermark,
};
use rspice_hardcopy_contract::{
    HardcopyPlan, HardcopyPlanId, HardcopyScope, HardcopySetup, OutputFormat,
};
use rspice_vocabulary::product::ContentDigest;
use sha2::{Digest as _, Sha256};

struct WorkerFixture {
    prepared: PreparedRetainedHardcopyResolution,
    source: ResolvedHardcopyDocument,
    source_key: String,
    scope: HardcopyScope,
}

fn worker_fixture() -> WorkerFixture {
    worker_fixture_with_wire_endpoint(80)
}

fn worker_fixture_with_wire_endpoint(endpoint_x: i32) -> WorkerFixture {
    let mut state = AppState::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(
            881,
            Point::new(-20, 5),
            Point::new(endpoint_x, 5),
        ));
    let active_view = state.model.workspace.content.active_view.clone();
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(active_view));
    let source_key = format!(
        "project:{}:cell-view:{}",
        state.model.workspace.content.project.id().as_uuid(),
        state.model.workspace.content.active_key()
    );
    let scope = HardcopyScope::ActiveDocument;
    let source = resolve_retained_hardcopy_source(&state, &source_key, scope.clone())
        .expect("fixture source resolves");
    let prepared =
        prepare_retained_hardcopy_resolution(SourceReads::of(&state), &source_key, scope.clone())
            .expect("fixture source prepares");
    WorkerFixture {
        prepared,
        source,
        source_key,
        scope,
    }
}

fn two_page_setup() -> HardcopySetup {
    let defaults = HardcopySetup::default();
    let physical_page = PhysicalPageSetup::try_new(
        defaults.physical_page().paper().clone(),
        PageMargins::uniform(rspice_hardcopy_contract::Length::ZERO),
        Bleed::None,
        defaults.physical_page().orientation(),
    )
    .expect("zero-margin worker fixture page is valid");
    let tiling = TilingSetup::try_new(
        TilingMode::Manual {
            columns: 2,
            rows: 1,
        },
        defaults.tiling().overlap(),
        false,
    )
    .expect("two-page tiling is valid");
    let decorations = DecorationSetup::try_new(false, false, false, Watermark::None)
        .expect("undecorated worker fixture is valid");
    let schematic = SchematicHardcopySetup::new(
        SchematicHardcopyExtent::CompleteSchematicContent,
        OutsideSheetContentPolicy::ExtendOutput,
        true,
        true,
        true,
        true,
        true,
        false,
    );
    HardcopySetup::try_new_with_schematic(
        physical_page,
        ScaleMode::EngineeringOneToOne,
        tiling,
        defaults.render().clone(),
        decorations,
        schematic,
        defaults.print_mapping().clone(),
    )
    .expect("two-page setup is valid")
}

fn two_page_svg_setup() -> HardcopySetup {
    let defaults = two_page_setup();
    HardcopySetup::try_new_with_schematic(
        defaults.physical_page().clone(),
        defaults.scale(),
        defaults.tiling(),
        RenderSetup::try_new(
            RenderTarget::ExportArtifact,
            OutputFormat::SvgVector,
            ColorMapping::PrintSafeEngineeringPalette,
            BackgroundMode::White,
            FontPolicy::new(true, true),
            true,
        )
        .expect("SVG render setup is valid"),
        defaults.decorations().clone(),
        defaults.schematic(),
        defaults.print_mapping().clone(),
    )
    .expect("two-page SVG setup is valid")
}

fn fixture_plan(source: &ResolvedHardcopyDocument, setup: HardcopySetup) -> HardcopyPlan {
    let plan_id = HardcopyPlanId::new();
    let sections = source
        .hardcopy_sections_for_setup(rspice_hardcopy_contract::SchematicHardcopySetup::default())
        .expect("fixture sections resolve");
    if sections.is_empty() {
        HardcopyPlan::compile_with_id(
            plan_id,
            source.authority().clone(),
            setup,
            source.content_extent(),
        )
    } else {
        HardcopyPlan::compile_with_id_and_sections(
            plan_id,
            source.authority().clone(),
            setup,
            source.content_extent(),
            sections,
        )
    }
    .expect("fixture plan compiles")
}

fn fixture_metadata(source: &ResolvedHardcopyDocument) -> HardcopySceneMetadata {
    crate::workbench::hardcopy_adapters::tests::source_metadata(source, "RSpice worker tests")
        .expect("fixture metadata is valid")
}

fn request(
    id: u32,
    epoch: u64,
    generation: u64,
    command: HardcopyWorkerCommand,
) -> HardcopyWorkerRequest {
    HardcopyWorkerRequest::try_new(id, epoch, generation, command)
        .expect("fixture request is valid")
}

fn execution_error(result: Result<HardcopyWorkerResponse, String>, context: &str) -> String {
    match result {
        Ok(_) => panic!("{context}"),
        Err(error) => error,
    }
}

#[test]
fn resolve_source_preserves_exact_identity_and_response_ticket() {
    let fixture = worker_fixture();
    let expected = fixture.source.clone();
    let response = execute_request(
        request(
            31,
            44,
            55,
            HardcopyWorkerCommand::ResolveSource {
                source_key: fixture.source_key,
                scope: fixture.scope,
            },
        ),
        vec![
            fixture
                .prepared
                .into_worker_snapshot_json()
                .expect("prepared snapshot encodes"),
        ],
    )
    .expect("source worker request succeeds");

    assert_eq!(
        response.protocol_version(),
        HARDCOPY_WORKER_PROTOCOL_VERSION
    );
    assert_eq!(response.id(), 31);
    assert_eq!(response.epoch(), "44");
    assert_eq!(response.generation(), "55");
    assert_eq!(response.operation(), HardcopyWorkerOperation::ResolveSource);
    let buffers = response.into_buffers();
    assert_eq!(buffers.len(), 1);
    let restored = ResolvedHardcopyDocument::from_worker_snapshot_json(&buffers[0])
        .expect("resolved worker snapshot decodes");
    assert_eq!(restored, expected);
}

#[test]
fn resolve_source_rejects_requested_key_and_scope_mismatch() {
    let wrong_key = worker_fixture();
    let error = execution_error(
        execute_request(
            request(
                1,
                1,
                1,
                HardcopyWorkerCommand::ResolveSource {
                    source_key: format!("{}:other", wrong_key.source_key),
                    scope: wrong_key.scope,
                },
            ),
            vec![
                wrong_key
                    .prepared
                    .into_worker_snapshot_json()
                    .expect("prepared snapshot encodes"),
            ],
        ),
        "a different requested source key must fail",
    );
    assert!(error.contains("other than the requested retained identity"));

    let wrong_scope = worker_fixture();
    let error = execution_error(
        execute_request(
            request(
                2,
                1,
                1,
                HardcopyWorkerCommand::ResolveSource {
                    source_key: wrong_scope.source_key,
                    scope: HardcopyScope::CurrentSheet,
                },
            ),
            vec![
                wrong_scope
                    .prepared
                    .into_worker_snapshot_json()
                    .expect("prepared snapshot encodes"),
            ],
        ),
        "a different requested source scope must fail",
    );
    assert!(error.contains("other than the requested retained identity"));
}

#[test]
fn preview_operation_returns_decodable_transfer_and_rejects_digest_mismatch() {
    let fixture = worker_fixture();
    let setup = HardcopySetup::default();
    let plan = fixture_plan(&fixture.source, setup.clone());
    let dpi = 72;
    let response = execute_request(
        request(
            90,
            12,
            34,
            HardcopyWorkerCommand::Preview {
                plan_id: plan.id(),
                expected_plan_digest: plan.content_digest(),
                setup: setup.clone(),
                metadata: fixture_metadata(&fixture.source),
                page_indices: vec![0],
                dpi,
            },
        ),
        vec![
            fixture
                .source
                .worker_snapshot_json()
                .expect("source snapshot encodes"),
        ],
    )
    .expect("preview worker request succeeds");

    assert_eq!(response.operation(), HardcopyWorkerOperation::Preview);
    assert_eq!(response.id(), 90);
    assert_eq!(response.epoch(), "12");
    assert_eq!(response.generation(), "34");
    let buffers = response.into_buffers();
    assert_eq!(buffers.len(), 2);
    let preview = HardcopyPreviewPage::from_worker_transfer(
        &plan,
        &fixture.source,
        0,
        dpi,
        &buffers[0],
        buffers[1].clone(),
    )
    .expect("preview transfer authenticates and decodes");
    assert_eq!(preview.page_number(), 1);
    let manifest: serde_json::Value = serde_json::from_slice(&buffers[0]).unwrap();
    assert_eq!(manifest["dpi"].as_u64(), Some(u64::from(dpi)));
    assert!(!preview.rgba().is_empty());

    let mismatch = execution_error(
        execute_request(
            request(
                91,
                12,
                35,
                HardcopyWorkerCommand::Preview {
                    plan_id: plan.id(),
                    expected_plan_digest: ContentDigest::from_bytes([0xa5; 32]),
                    setup,
                    metadata: fixture_metadata(&fixture.source),
                    page_indices: vec![0],
                    dpi,
                },
            ),
            vec![
                fixture
                    .source
                    .worker_snapshot_json()
                    .expect("source snapshot encodes"),
            ],
        ),
        "a mismatched trusted plan digest must fail",
    );
    assert!(mismatch.contains("trusted plan digest"));
}

#[test]
fn preview_operation_preserves_two_page_pair_order_and_count() {
    let fixture = worker_fixture_with_wire_endpoint(1_600);
    let setup = two_page_setup();
    let plan = fixture_plan(&fixture.source, setup.clone());
    assert_eq!(
        plan.pagination().pages().len(),
        2,
        "fixture must remain an exact two-page plan"
    );
    let dpi = 72;
    let response = execute_request(
        request(
            92,
            13,
            36,
            HardcopyWorkerCommand::Preview {
                plan_id: plan.id(),
                expected_plan_digest: plan.content_digest(),
                setup,
                metadata: fixture_metadata(&fixture.source),
                page_indices: vec![0, 1],
                dpi,
            },
        ),
        vec![
            fixture
                .source
                .worker_snapshot_json()
                .expect("source snapshot encodes"),
        ],
    )
    .expect("two-page preview worker request succeeds");

    let buffers = response.into_buffers();
    assert_eq!(buffers.len(), 4);
    let previews = [0usize, 1]
        .into_iter()
        .map(|page_index| {
            let buffer_index = page_index * 2;
            HardcopyPreviewPage::from_worker_transfer(
                &plan,
                &fixture.source,
                page_index,
                dpi,
                &buffers[buffer_index],
                buffers[buffer_index + 1].clone(),
            )
            .expect("ordered preview pair authenticates")
        })
        .collect::<Vec<_>>();
    assert_eq!(
        previews
            .iter()
            .map(HardcopyPreviewPage::page_number)
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_ne!(previews[0].coordinate(), previews[1].coordinate());
}

#[test]
fn publication_operation_returns_exact_decodable_parts_and_rejects_count_mismatch() {
    let fixture = worker_fixture();
    let setup = HardcopySetup::default();
    let plan = fixture_plan(&fixture.source, setup.clone());
    let expected_part_count = publication_part_count(&plan);
    assert_eq!(expected_part_count, 1, "PDF publication is one artifact");

    let response = execute_request(
        request(
            101,
            22,
            33,
            HardcopyWorkerCommand::Publication {
                plan_id: plan.id(),
                expected_plan_digest: plan.content_digest(),
                expected_part_count,
                package_multi_part: false,
                setup: setup.clone(),
                metadata: fixture_metadata(&fixture.source),
            },
        ),
        vec![
            fixture
                .source
                .worker_snapshot_json()
                .expect("source snapshot encodes"),
        ],
    )
    .expect("publication worker request succeeds");

    assert_eq!(response.operation(), HardcopyWorkerOperation::Publication);
    assert_eq!(response.id(), 101);
    assert_eq!(response.epoch(), "22");
    assert_eq!(response.generation(), "33");
    let buffers = response.into_buffers();
    assert_eq!(buffers.len(), expected_part_count + 1);
    let publication = RenderedHardcopyPublication::from_worker_transfer(
        &plan,
        &fixture.source,
        &buffers[0],
        buffers[1..].to_vec(),
    )
    .expect("publication transfer authenticates and decodes");
    assert_eq!(publication.format(), OutputFormat::PdfVector);
    assert_eq!(
        publication.page_count(),
        plan.pagination().pages().len() as u32
    );

    let mismatch = execution_error(
        execute_request(
            request(
                102,
                22,
                34,
                HardcopyWorkerCommand::Publication {
                    plan_id: plan.id(),
                    expected_plan_digest: plan.content_digest(),
                    expected_part_count: expected_part_count + 1,
                    package_multi_part: false,
                    setup,
                    metadata: fixture_metadata(&fixture.source),
                },
            ),
            vec![
                fixture
                    .source
                    .worker_snapshot_json()
                    .expect("source snapshot encodes"),
            ],
        ),
        "an inexact publication part count must fail",
    );
    assert!(mismatch.contains("part count does not match the exact plan"));
}

#[test]
fn packaged_publication_is_deterministic_authority_bound_and_tamper_evident() {
    // Schematic coordinates are 254 um per unit. Extend the authenticated
    // source beyond one Letter-landscape printable viewport so this is a
    // real two-part SVG publication rather than a mislabeled fixture.
    let fixture = worker_fixture_with_wire_endpoint(1_420);
    let setup = two_page_svg_setup();
    let plan = fixture_plan(&fixture.source, setup.clone());
    let expected_part_count = publication_part_count(&plan);
    assert!(
        expected_part_count >= 2,
        "fixture must produce a multi-page SVG publication"
    );
    let metadata = fixture_metadata(&fixture.source);
    let response = execute_request(
        request(
            111,
            24,
            35,
            HardcopyWorkerCommand::Publication {
                plan_id: plan.id(),
                expected_plan_digest: plan.content_digest(),
                expected_part_count,
                package_multi_part: true,
                setup,
                metadata: metadata.clone(),
            },
        ),
        vec![
            fixture
                .source
                .worker_snapshot_json()
                .expect("source snapshot encodes"),
        ],
    )
    .expect("packaged publication worker request succeeds");
    assert_eq!(
        response.operation(),
        HardcopyWorkerOperation::PackagedPublication
    );
    let buffers = response.into_buffers();
    assert_eq!(buffers.len(), 2);

    let packaged = decode_packaged_publication(&plan, &fixture.source, buffers.clone())
        .expect("packaged transfer authenticates");
    let (bytes, artifact, page_count) = packaged.into_parts();
    let independently_rendered =
        HardcopyRenderer::render_resolved(&plan, &fixture.source, metadata)
            .expect("independent publication renders");
    let expected_entries = independently_rendered
        .parts()
        .iter()
        .map(|part| (part.suggested_filename(), part.bytes()))
        .collect::<Vec<_>>();
    let expected_zip = deterministic_stored_zip(&expected_entries).expect("expected ZIP assembles");
    assert_eq!(
        bytes, expected_zip,
        "ZIP entry order, names, and bytes must match the ordered rendered publication"
    );
    assert_eq!(
        artifact.content_digest(),
        ContentDigest::from_bytes(Sha256::digest(&bytes).into())
    );
    assert_eq!(page_count, plan.pagination().pages().len() as u32);

    let mut tampered_bytes = buffers.clone();
    tampered_bytes[1][0] ^= 0x01;
    assert!(
        decode_packaged_publication(&plan, &fixture.source, tampered_bytes).is_err(),
        "tampered ZIP bytes must fail artifact authentication"
    );

    let mut manifest: serde_json::Value =
        serde_json::from_slice(&buffers[0]).expect("manifest decodes");
    manifest["planContentDigest"] = serde_json::to_value(ContentDigest::from_bytes(
        Sha256::digest(b"wrong hardcopy plan").into(),
    ))
    .expect("tampered digest serializes");
    let mut tampered_manifest = buffers;
    tampered_manifest[0] = serde_json::to_vec(&manifest).expect("tampered manifest encodes");
    assert!(
        decode_packaged_publication(&plan, &fixture.source, tampered_manifest).is_err(),
        "tampered authority metadata must fail closed"
    );
}
