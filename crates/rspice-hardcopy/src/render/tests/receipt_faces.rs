//! A receipt names the faces its hardcopy's text was lettered in, as the
//! render that lettered them reports them: the same faces whatever the format,
//! carried unchanged through the browser worker and its package, and read back
//! from the receipt the project file persists.

use rspice_fonts::Face;
use rspice_hardcopy_contract::{
    HardcopyFace, HardcopyFaceSource, HardcopyOutcome, HardcopyReceipt,
};
use sha2::{Digest as _, Sha256};

use super::glyph_fallback::{text, text_scene, undecorated_plan};
use super::*;
use crate::worker::{
    HardcopyWorkerCommand, HardcopyWorkerRequest, decode_packaged_publication, execute_request,
    publication_part_count,
};

/// `recorded` names exactly `expected`, in order: each by its family and
/// weight, sealed to the bytes the binary embeds and marked bundled.
fn assert_bundled(recorded: &[HardcopyFace], expected: &[Face]) {
    assert_eq!(recorded.len(), expected.len(), "{recorded:?}");
    for (entry, face) in recorded.iter().zip(expected) {
        assert_eq!(
            (entry.family(), entry.weight()),
            (face.family(), face.weight())
        );
        assert_eq!(
            entry.sha256(),
            ContentDigest::from_bytes(Sha256::digest(face.bytes()).into()),
            "{entry:?}"
        );
        assert_eq!(entry.source(), &HardcopyFaceSource::Bundled);
        assert!(entry.version().starts_with("Version "), "{entry:?}");
    }
}

#[test]
fn a_render_reports_each_face_its_text_is_lettered_in_once_whatever_the_format() {
    let content = extent(150_000, 80_000);
    let scene = text_scene(
        content,
        vec![
            text(SceneFont::Sans, "\u{2300} 6 mm", 20_000),
            text(SceneFont::MonospaceMedium, "R1", 40_000),
            text(SceneFont::Sans, "Plain text", 60_000),
        ],
    );
    // Canonical order is by family, then weight; the diameter sign is
    // borrowed from RSpice Symbols.
    let expected = [Face::MonoMedium, Face::SansRegular, Face::Symbols];
    for format in [
        OutputFormat::SvgVector,
        OutputFormat::PdfVector,
        OutputFormat::Png { dpi: 72 },
    ] {
        let publication =
            HardcopyRenderer::render(&undecorated_plan(format, content), &scene).unwrap();
        assert_bundled(publication.faces_used(), &expected);
    }
    let printer = HardcopyRenderer::render_printer_pages(
        &undecorated_plan(OutputFormat::NativePrinter, content),
        &scene,
        72,
    )
    .unwrap();
    assert_bundled(printer.faces_used(), &expected);

    // A page with no text is lettered in no face.
    let blank = HardcopyRenderer::render(
        &undecorated_plan(OutputFormat::SvgVector, content),
        &text_scene(content, Vec::new()),
    )
    .unwrap();
    assert!(blank.faces_used().is_empty());
}

#[test]
fn the_receipt_reads_back_the_faces_the_worker_lettered() {
    let source = resolved_wide_symbol();
    let settings = setup(OutputFormat::SvgVector, true);
    let plan = HardcopyPlan::compile_with_id(
        HardcopyPlanId::try_from_uuid(Uuid::from_u128(82)).unwrap(),
        source.authority().clone(),
        settings.clone(),
        source.content_extent(),
    )
    .unwrap();
    let metadata = resolved_metadata();
    let rendered = HardcopyRenderer::render_resolved(&plan, &source, metadata.clone()).unwrap();
    let faces = rendered.faces_used();
    assert!(
        faces.iter().any(|face| face.family() == "IBM Plex Sans"),
        "the title strip is lettered in IBM Plex Sans: {faces:?}"
    );

    let publish = |package_multi_part| {
        let request = HardcopyWorkerRequest::try_new(
            7,
            3,
            5,
            HardcopyWorkerCommand::Publication {
                plan_id: plan.id(),
                expected_plan_digest: plan.content_digest(),
                expected_part_count: publication_part_count(&plan),
                package_multi_part,
                setup: settings.clone(),
                metadata: metadata.clone(),
            },
        )
        .unwrap();
        execute_request(request, vec![source.worker_snapshot_json().unwrap()])
            .unwrap()
            .into_buffers()
    };

    let mut buffers = publish(false);
    let manifest = buffers.remove(0);
    let transferred =
        RenderedHardcopyPublication::from_worker_transfer(&plan, &source, &manifest, buffers)
            .unwrap();
    assert_eq!(transferred.faces_used(), faces);

    let packaged = decode_packaged_publication(&plan, &source, publish(true)).unwrap();
    assert_eq!(packaged.faces_used(), faces);
    let faces_used = packaged.faces_used().to_vec();
    let (_, artifact, _) = packaged.into_parts();
    let receipt = HardcopyReceipt::record(
        &plan,
        HardcopyOutcome::ArtifactExported { artifact },
        faces_used,
    )
    .unwrap();
    let persisted = serde_json::to_string(&receipt).unwrap();
    let read: HardcopyReceipt = serde_json::from_str(&persisted).unwrap();
    assert_eq!(read.faces_used(), faces);
    assert_eq!(read, receipt);
}
