//! The shortcut reference PDF over the product's own commands.

use egui::os::OperatingSystem;
use lopdf::{Document as ParsedPdf, Object};

use crate::state::command_catalog::CommandPlatform;
use crate::state::shortcuts::ShortcutPreferences;
use crate::workbench::shortcuts::artifacts::pdf::*;
use crate::workbench::shortcuts::artifacts::projection::{
    ShortcutExportRequest, ShortcutExportScope, ShortcutReferenceModel,
    build_shortcut_reference_model,
};

fn complete_model() -> ShortcutReferenceModel {
    build_shortcut_reference_model(
        &ShortcutPreferences::default(),
        &ShortcutExportRequest {
            scope: ShortcutExportScope::CompleteResolved,
            include_platform_mappings: false,
            runtime_platform: CommandPlatform::Desktop,
            operating_system: OperatingSystem::Windows,
            current_contexts: Vec::new(),
        },
        crate::workbench::commands::vocabulary::command_registry(),
    )
    .unwrap()
}

#[test]
fn pdf_is_byte_deterministic_and_has_no_time_metadata() {
    let model = complete_model();
    let first = serialize_shortcut_reference_pdf(&model).unwrap();
    let second = serialize_shortcut_reference_pdf(&model).unwrap();
    assert_eq!(first, second);

    let parsed = ParsedPdf::load_mem(&first).unwrap();
    let info = parsed
        .trailer
        .get(b"Info")
        .ok()
        .and_then(|object| object.as_reference().ok())
        .and_then(|id| parsed.get_object(id).ok())
        .and_then(|object| object.as_dict().ok());
    if let Some(info) = info {
        assert!(!info.has(b"CreationDate"));
        assert!(!info.has(b"ModDate"));
    }
}

#[test]
fn pdf_parser_recovers_structure_text_and_embedded_fonts() {
    let pdf = serialize_shortcut_reference_pdf(&complete_model()).unwrap();
    let parsed = ParsedPdf::load_mem(&pdf).unwrap();
    let pages = parsed.get_pages();
    assert!(
        pages.len() > 1,
        "default resolved map must exercise pagination"
    );

    let text = parsed
        .extract_text(&pages.keys().copied().collect::<Vec<_>>())
        .unwrap();
    assert!(text.contains("RSpice keyboard shortcuts"));
    assert!(text.contains("save-project"));
    assert!(text.contains("Ctrl+S"));
    assert!(text.contains("Page 1 of"));

    let raw = String::from_utf8_lossy(&pdf);
    assert!(raw.contains("/FontFile2"));
    assert!(raw.contains("IBMPlexSans"));
    assert!(raw.contains("IBMPlexMono"));
}

#[test]
fn every_multipage_table_page_repeats_column_headers() {
    let pdf = serialize_shortcut_reference_pdf(&complete_model()).unwrap();
    let parsed = ParsedPdf::load_mem(&pdf).unwrap();
    let pages = parsed.get_pages();
    assert!(pages.len() > 2);
    for page_number in pages.keys().copied() {
        let text = parsed.extract_text(&[page_number]).unwrap();
        if text.contains("save-project")
            || text.contains("SHORTCUT REFERENCE")
            || text.contains("CONTEXT")
        {
            for header in [
                "CONTEXT", "GROUP", "COMMAND", "BINDING", "PLATFORM", "STATUS",
            ] {
                assert!(
                    text.contains(header),
                    "page {page_number} is missing repeated table header {header}"
                );
            }
        }
    }
}

#[test]
fn page_tree_contains_only_page_objects_and_numbered_footers() {
    let pdf = serialize_shortcut_reference_pdf(&complete_model()).unwrap();
    let parsed = ParsedPdf::load_mem(&pdf).unwrap();
    let pages = parsed.get_pages();
    let page_count = pages.len();
    for (page_number, object_id) in pages {
        let page = parsed.get_object(object_id).unwrap().as_dict().unwrap();
        assert_eq!(page.get(b"Type").unwrap(), &Object::Name(b"Page".to_vec()));
        let text = parsed.extract_text(&[page_number]).unwrap();
        assert!(text.contains(&format!("Page {page_number} of {page_count}")));
    }
}

#[test]
fn wrapping_normalizes_controls_and_prevents_overwide_tokens() {
    let lines = wrap_text("alpha\nvery-long-token-with-many-segments", 42.0, 8.0, true);
    assert!(lines.len() > 2);
    assert!(lines.iter().all(|line| !line.contains('\n')));
    assert!(lines.iter().all(|line| text_width(line, 8.0, true) <= 42.0));
}
